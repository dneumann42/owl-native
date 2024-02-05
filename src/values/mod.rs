#[derive(Debug, PartialEq)]
pub enum Value {
    Num(f64),
    Str(String),
    Sym(String),
    Bool(bool),
    List(Vec<Box<Value>>),
}
