use evaluator::Intrinsic;

use crate::{
    evaluator::{eval, Evaluator},
    values::{Env, Value},
};

pub mod evaluator;
pub mod reader;
pub mod values;

use values::Value::Num;

fn main() {}

#[test]
fn evaluate_math_expressions() {
    assert_eq!(eval("(+ 1 2 3)"), Num(6.0));
    assert_eq!(eval("(+ 1 (+ 1 2) 3)"), Num(7.0));
}
