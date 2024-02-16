use core::panic;
use std::collections::HashMap;
use std::fmt::Pointer;

use crate::reader::Reader;
use crate::values::Value::{Atom, Bool, Func, List, Num, Str, Sym};
use crate::values::{car, cdr, Env, Value};

pub trait Intrinsic {
    fn name(self: &Self) -> &'static str;
    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value;
}

struct Nop {}
impl Intrinsic for Nop {
    fn name(self: &Self) -> &'static str {
        "nop"
    }

    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value {
        args
    }
}

pub struct Evaluator {
    intrinsics: HashMap<String, Box<dyn Intrinsic>>,
}

struct Eval;
impl Intrinsic for Eval {
    fn name(self: &Self) -> &'static str {
        "eval"
    }

    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value {
        match args {
            List(xs) => evaluator.eval(env, xs[0].clone()),
            _ => Value::None,
        }
    }
}

struct Add;
impl Intrinsic for Add {
    fn name(self: &Self) -> &'static str {
        "+"
    }

    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value {
        let mut total = 0.0;
        match args {
            List(args) => {
                for value in args {
                    total += match evaluator.evaluate(env, &value) {
                        Num(n) => n,
                        _ => 0.0,
                    }
                }
            }
            _ => {}
        }
        Num(total)
    }
}

struct Mul;
impl Intrinsic for Mul {
    fn name(self: &Self) -> &'static str {
        "*"
    }

    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value {
        let mut total = 1.0;
        match args {
            List(args) => {
                for value in args {
                    total *= match evaluator.evaluate(env, &value) {
                        Num(n) => n,
                        _ => 0.0,
                    }
                }
            }
            _ => {}
        }
        Num(total)
    }
}

struct Sub;
impl Intrinsic for Sub {
    fn name(self: &Self) -> &'static str {
        "-"
    }

    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value {
        let mut total = car(&args).clone().as_num();
        for value in cdr(&args).as_vec() {
            total -= match evaluator.evaluate(env, &value) {
                Num(n) => n,
                _ => 0.0,
            }
        }
        Num(total)
    }
}

struct Div;
impl Intrinsic for Div {
    fn name(self: &Self) -> &'static str {
        "/"
    }

    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value {
        let mut total = car(&args).clone().as_num();
        for value in cdr(&args).as_vec() {
            total /= match evaluator.evaluate(env, &value) {
                Num(n) => n,
                _ => 0.0,
            }
        }
        Num(total)
    }
}

struct Eqauals;
impl Intrinsic for Eqauals {
    fn name(self: &Self) -> &'static str {
        "="
    }

    fn eval(self: &Self, evaluator: &Evaluator, env: &mut Env, args: Value) -> Value {
        let head = car(&args);
        match cdr(&args) {
            List(xs) => {
                for arg in xs {
                    let value = evaluator.eval(env, arg);
                    if head.clone() != value {
                        return Value::Bool(false);
                    }
                }
                return Value::Bool(true);
            }
            _ => Value::Bool(false),
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        let mut this = Self {
            intrinsics: HashMap::new(),
        };
        this.base_intrinsics();
        this
    }

    pub fn is_intrinsic<T: ToString>(self: &Self, s: T) -> bool {
        self.intrinsics.contains_key(&s.to_string())
    }

    pub fn add_intrinsic<T: Intrinsic + 'static>(self: &mut Self, intr: T) {
        self.intrinsics
            .insert(intr.name().to_string(), Box::new(intr));
    }

    pub fn base_intrinsics(self: &mut Self) {
        self.add_intrinsic(Eval {});
        self.add_intrinsic(Eqauals {});
        self.add_intrinsic(Add {});
        self.add_intrinsic(Mul {});
        self.add_intrinsic(Sub {});
        self.add_intrinsic(Div {});
    }

    pub fn evaluate_if(
        self: &Self,
        env: &mut Env,
        cond: &Value,
        if_true: &Value,
        if_false: &Value,
    ) -> Value {
        if self.evaluate(env, cond).is_true() {
            self.evaluate(env, if_true).clone()
        } else {
            self.evaluate(env, if_false).clone()
        }
    }

    pub fn evaluate_special_form(
        self: &Self,
        env: &mut Env,
        ident: &String,
        args: &Vec<Value>,
    ) -> Option<Value> {
        match ident.as_str() {
            "do" => {
                let mut result = Value::None;
                for arg in args {
                    result = self.evaluate(env, arg);
                }
                Some(result)
            }
            "if" => {
                if args.len() < 2 {
                    panic!("If is missing arguments");
                }
                Some(
                    self.evaluate_if(
                        env,
                        &args[0],
                        &args[1],
                        if args.len() > 2 {
                            &args[2]
                        } else {
                            &Value::None
                        },
                    )
                    .clone(),
                )
            }
            "def" => {
                let sym = &args[0];
                assert!(matches!(sym, Sym(_)));
                let value = self.evaluate(env, &args[1]);
                env.set(sym.to_string(), value.clone());
                Some(value)
            }
            "set" => {
                let sym = &args[0];
                assert!(matches!(sym, Sym(_)));
                assert!(env.has(sym.to_string()));
                let value = self.evaluate(env, &args[1]);
                env.set(sym.to_string(), value.clone());
                Some(value)
            }
            "fun" => {
                let sym = &args[0];
                assert!(matches!(sym, Sym(_)));
                assert!(!env.has(sym.to_string()));
                Some(Value::None)
            }
            _ => None,
        }
    }

    pub fn evaluate(self: &Self, env: &mut Env, value: &Value) -> Value {
        match value {
            Num(_) | Str(_) | Atom(_) | Bool(_) | Func(_) | Value::None => value.clone(),
            Sym(s) => env.get(s.into()).clone(),
            List(xs) if xs.len() == 0 => value.clone(),
            List(xs) => {
                let ident = xs.get(0).unwrap_or(&Value::None).to_string();
                let args = &xs[1..].to_vec();

                match self.evaluate_special_form(env, &ident, args) {
                    Some(v) => return v,
                    None => {} // not a special form
                }

                if self.is_intrinsic(&ident) {
                    let intr = self
                        .intrinsics
                        .get(&ident.clone())
                        .expect("Failed to get intrinsic")
                        .as_ref();
                    return intr.eval(&self, env, Value::List(args.clone()));
                }

                Value::None
            }
        }
    }

    pub fn eval<T: ToString>(self: &Self, env: &mut Env, code: T) -> Value {
        let mut reader = Reader::new();

        match reader.read_script(&code.to_string()) {
            Ok(v) => self.evaluate(env, &v),
            Err(e) => {
                println!("Error {:?}", e);
                Value::None
            }
        }
    }
}

pub fn eval<T: ToString>(code: T) -> Value {
    let mut env = Env::new();
    Evaluator::new().eval(&mut env, code)
}
