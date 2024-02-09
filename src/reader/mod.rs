use std::char;

use crate::values::Value;

pub struct Reader {
    pub it: usize,
}

#[derive(Debug, PartialEq)]
pub enum ReaderError {
    NotANumber,
    NotABoolean,
    NotAString,
    NotAList,
    NotAFunctionCall,
    UnterminatedString,
    UnbalancedParenthesis,
    UnbalancedBraces,
    InvalidNumber(String),
    InvalidSymbol(String),
    GenericError(String),
}

type ReaderResult = Result<Value, ReaderError>;

impl Reader {
    pub fn new() -> Self {
        Self { it: 0 }
    }

    pub fn reset(self: &mut Self) {
        self.it = 0;
    }

    pub fn at_eof(self: &Self, code: &String) -> bool {
        self.it >= code.len()
    }

    pub fn chr(self: &Self, code: &String) -> Option<char> {
        code.chars().nth(self.it)
    }

    pub fn is_chr_p(self: &Self, code: &String, f: fn(char) -> bool) -> bool {
        self.chr(code).map_or(false, f)
    }
    pub fn is_chr(self: &Self, code: &String, chr: char) -> bool {
        self.chr(code).map_or(false, |ch| ch == chr)
    }

    pub fn is_whitespace(self: &Self, code: &String) -> bool {
        code.chars()
            .nth(self.it)
            .map_or(false, |ch| ch.is_whitespace())
    }

    pub fn is_delimiter(self: &Self, code: &String) -> bool {
        !self.at_eof(code)
            && self.chr(code).map_or(false, |ch| match ch {
                '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '\'' => true,
                _ => false,
            })
    }

    pub fn skip_whitespace(self: &mut Self, code: &String) {
        while !self.at_eof(code) && self.is_whitespace(code) {
            self.it += 1
        }
    }

    pub fn read_boolean(self: &mut Self, code: &String) -> ReaderResult {
        let start = self.it;
        if !self.is_chr(code, '#') {
            return Err(ReaderError::NotABoolean);
        }
        self.it += 1;
        if self.chr(code).map_or(false, |ch| ch == 't' || ch == 'T') {
            self.it += 1;
            Ok(Value::Bool(true))
        } else if self.chr(code).map_or(false, |ch| ch == 'f' || ch == 'F') {
            self.it += 1;
            Ok(Value::Bool(false))
        } else {
            self.it = start;
            Err(ReaderError::NotABoolean)
        }
    }

    pub fn read_number(self: &mut Self, code: &String) -> ReaderResult {
        let start = self.it;
        let mut is_real = false;

        if self.is_chr(code, '-') {
            self.it += 1;
        }

        if self.is_chr(code, '.') {
            self.it += 1;
            is_real = true;
        }

        if !self.chr(code).map_or(false, |ch| ch.is_digit(10)) {
            self.it = start;
            return Err(ReaderError::NotANumber);
        }

        while !self.at_eof(code) {
            if self.is_chr(code, '.') {
                if is_real {
                    self.it = start;
                    return Err(ReaderError::InvalidNumber("Too many dots".into()));
                } else {
                    is_real = true;
                    self.it += 1;
                    continue;
                }
            }

            if !self.chr(code).map_or(false, |ch| ch.is_digit(10)) {
                if start == self.it {
                    return Err(ReaderError::NotANumber);
                }

                let sc = &code[start..self.it];
                let n = sc
                    .parse::<f64>()
                    .map_err(|e| ReaderError::InvalidNumber(e.to_string()))?;
                return Ok(Value::Num(n));
            }

            self.it += 1;
        }

        Err(ReaderError::NotANumber)
    }

    pub fn read_string(self: &mut Self, code: &String) -> ReaderResult {
        let start = self.it;
        if !self.is_chr(code, '"') {
            return Err(ReaderError::NotAString);
        }
        self.it += 1;
        while !self.at_eof(code) {
            if self.is_chr(code, '"') {
                self.it += 1;
                return Ok(Value::Str(code[start + 1..self.it - 1].into()));
            }
            self.it += 1;
        }
        self.it = start;
        Err(ReaderError::UnterminatedString)
    }

    pub fn read_symbol(self: &mut Self, code: &String) -> ReaderResult {
        let start = self.it;
        while !self.at_eof(code) && !self.is_whitespace(code) && !self.is_delimiter(code) {
            self.it += 1;
        }
        if self.it == start {
            return Err(ReaderError::InvalidSymbol("Empty symbol".into()));
        }
        return Ok(Value::Sym(code[start..self.it].into()));
    }

    pub fn read_list(self: &mut Self, code: &String) -> ReaderResult {
        let mut xs = Vec::new();
        if self.is_chr(code, '(') {
            self.it += 1;
            while !self.at_eof(code) {
                let exp = self.read(code)?;
                xs.push(Box::new(exp));
                self.skip_whitespace(code);
                if self.at_eof(code) {
                    return Err(ReaderError::UnbalancedParenthesis);
                } else if self.is_chr(code, ')') {
                    self.it += 1;
                    return Ok(Value::List(xs));
                }
            }
        }
        Err(ReaderError::NotAList)
    }

    pub fn read_do_block(self: &mut Self, code: &String) -> ReaderResult {
        let mut xs = Vec::new();
        if self.is_chr(code, '{') {
            self.it += 1;
            while !self.at_eof(code) {
                let exp = self.read(code)?;
                xs.push(Box::new(exp));
                self.skip_whitespace(code);
                if self.at_eof(code) {
                    return Err(ReaderError::UnbalancedBraces);
                } else if self.is_chr(code, '}') {
                    self.it += 1;
                    xs.insert(0, Box::new(Value::Sym("do".into())));
                    return Ok(Value::List(xs));
                }
            }
        }
        Err(ReaderError::NotAList)
    }

    pub fn read_function_call(self: &mut Self, code: &String) -> ReaderResult {
        let start = self.it;
        let sym = self.read_symbol(code)?;
        let list = match self.read_list(code) {
            o @ Ok(_) => o,
            e @ Err(ReaderError::UnbalancedParenthesis) => {
                return e;
            }
            e @ Err(_) => {
                self.it = start;
                return e;
            }
        }?;
        match list {
            Value::List(xs) => {
                let mut vs = xs;
                vs.insert(0, Box::new(sym));
                Ok(Value::List(vs))
            }
            _ => Err(ReaderError::NotAFunctionCall),
        }
    }

    pub fn read(self: &mut Self, code: &String) -> ReaderResult {
        self.skip_whitespace(code);

        match self.read_number(code) {
            n @ Ok(_) => return n,
            e @ Err(ReaderError::InvalidNumber(_)) => return e,
            _ => {}
        }

        match self.read_boolean(code) {
            b @ Ok(_) => return b,
            _ => {}
        }

        match self.read_string(code) {
            s @ Ok(_) => return s,
            _ => {}
        }

        match self.read_list(code) {
            s @ Ok(_) => return s,
            e @ Err(ReaderError::UnbalancedParenthesis) => return e,
            _ => {}
        }

        match self.read_do_block(code) {
            s @ Ok(_) => return s,
            e @ Err(ReaderError::UnbalancedBraces) => return e,
            _ => {}
        }

        match self.read_function_call(code) {
            s @ Ok(_) => return s,
            e @ Err(ReaderError::UnbalancedParenthesis) => return e,
            _ => {}
        }

        return match self.read_symbol(code) {
            s @ Ok(_) => s,
            e @ Err(_) => e,
        };
    }
}
