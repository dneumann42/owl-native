use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Index,
    rc::Rc,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    None,
    Num(f64),
    Str(String),
    Sym(String),
    Atom(String),
    Bool(bool),
    List(Vec<Value>),
    Func(OwlFunc),
}

impl Value {
    pub fn is_true(self) -> bool {
        match self {
            Value::None => false,
            Value::Num(n) if n != 0.0 => true,
            Value::Bool(b) => b,
            _ => true,
        }
    }

    pub fn as_num(self) -> f64 {
        match self {
            Value::Num(f) => f,
            _ => 0.0,
        }
    }

    pub fn as_vec(self) -> Vec<Value> {
        match self {
            Value::List(xs) => xs,
            _ => vec![],
        }
    }
}

pub fn car(v: &Value) -> &Value {
    match v {
        Value::List(xs) => &xs[0],
        _ => &Value::None,
    }
}

pub fn cdr(v: &Value) -> Value {
    match v {
        Value::List(xs) => Value::List(xs[1..].to_vec()),
        _ => Value::None,
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::None => todo!(),
            Value::Num(n) => n.to_string(),
            Value::Str(s) => s.to_string(),
            Value::Sym(s) => s.to_string(),
            Value::Atom(a) => a.to_string(),
            Value::Bool(t) => {
                if *t {
                    "#t".to_string()
                } else {
                    "#f".to_string()
                }
            }
            Value::List(_) => todo!(),
            Value::Func(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Env {
    data: HashMap<String, Value>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            parent: None,
        }
    }

    pub fn has<T: ToString>(self: &mut Self, ident: T) -> bool {
        self.data.contains_key(&ident.to_string())
    }

    pub fn set<T: ToString>(self: &mut Self, ident: T, value: Value) {
        self.data.insert(ident.to_string(), value);
    }

    pub fn get(self: &Self, ident: String) -> &Value {
        if self.data.contains_key(&ident) {
            return self.data.get(&ident).unwrap_or(&Value::None);
        }
        match &self.parent {
            Some(env) => env.get(ident),
            None => &Value::None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OwlFunc {
    params: Rc<Value>,
    body: Rc<Value>,
    env: Rc<RefCell<Env>>,
}
