use evaluator::Intrinsic;

use crate::{
    evaluator::Evaluator,
    values::{Env, Value},
};

pub mod evaluator;
pub mod reader;
pub mod values;

fn main() {
    let mut env = Env::new();
    env.set("x", Value::Num(123.0));

    let evaluator = Evaluator::new();
    assert_eq!(
        evaluator.eval(&mut env, r#"(eval "(+ 1 2 3)")"#),
        Value::Num(6.0)
    );
}
