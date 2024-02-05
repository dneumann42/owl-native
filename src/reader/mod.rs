use std::char;

use crate::values::Value;

pub struct Reader {
    pub it: usize,
}

#[derive(Debug, PartialEq)]
pub enum ReaderError {
    NotANumber,
    InvalidNumber(String),
    GenericError(String),
}

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

    pub fn is_whitespace(self: &Self, code: &String) -> bool {
        code.chars()
            .nth(self.it)
            .map_or(false, |ch| ch.is_whitespace())
    }

    pub fn skip_whitespace(self: &mut Self, code: &String) {
        while !self.at_eof(code) && self.is_whitespace(code) {
            self.it += 1
        }
    }

    pub fn read_number(self: &mut Self, code: &String) -> Result<Value, ReaderError> {
        let start = self.it;
        let mut is_real = false;

        if self.chr(code).map_or(false, |ch| ch == '-') {
            self.it += 1;
        }

        if self.chr(code).map_or(false, |ch| ch == '.') {
            self.it += 1;
            is_real = true;
        }

        if !self.chr(code).map_or(false, |ch| ch.is_digit(10)) {
            self.it = start;
            return Err(ReaderError::NotANumber);
        }

        while !self.at_eof(code) {
            if self.chr(code).map_or(false, |ch| ch == '.') {
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

    pub fn read(self: &mut Self, code: &String) -> Result<Value, ReaderError> {
        self.skip_whitespace(code);

        match self.read_number(code) {
            Ok(number) => return Ok(number),
            e @ Err(ReaderError::InvalidNumber(_)) => return e,
            _ => {}
        }

        Err(ReaderError::GenericError("".into()))
    }
}
