use core::panic;

use owl::{
    reader::{Reader, ReaderError},
    values::Value::{self, Bool, List, Num, Str, Sym},
};

#[test]
fn skipping_whitespace() {
    let code = String::from("  \t \nX");
    let mut reader = Reader::new();
    reader.skip_whitespace(&code);
    assert_eq!(reader.chr(&code).unwrap(), 'X')
}

#[test]
fn reading_numbers() {
    let code = String::from(r"1 123 -54 0.0 .3 -.3 3.1415926 ");
    let mut reader = Reader::new();
    assert_eq!(reader.read(&code).unwrap(), Num(1.0));
    assert_eq!(reader.read(&code).unwrap(), Num(123.0));
    assert_eq!(reader.read(&code).unwrap(), Num(-54.0));
    assert_eq!(reader.read(&code).unwrap(), Num(0.0));
    assert_eq!(reader.read(&code).unwrap(), Num(0.3));
    assert_eq!(reader.read(&code).unwrap(), Num(-0.3));
    assert_eq!(reader.read(&code).unwrap(), Num(3.1415926));

    reader.reset();
    let error_code = String::from("34.41.123");
    assert_eq!(
        reader.read(&error_code),
        Err(ReaderError::InvalidNumber("Too many dots".into()))
    )
}

#[test]
fn reading_booleans() {
    let code = String::from("#t #T #f #F");
    let mut reader = Reader::new();
    assert_eq!(reader.read(&code).unwrap(), Bool(true));
    assert_eq!(reader.read(&code).unwrap(), Bool(true));
    assert_eq!(reader.read(&code).unwrap(), Bool(false));
    assert_eq!(reader.read(&code).unwrap(), Bool(false));
}

#[test]
fn reading_symbols() {
    let code = String::from("hello-world test54 a");
    let mut reader = Reader::new();
    assert_eq!(reader.read(&code).unwrap(), Sym("hello-world".into()));
    assert_eq!(reader.read(&code).unwrap(), Sym("test54".into()));
    assert_eq!(reader.read(&code).unwrap(), Sym("a".into()));
}

#[test]
fn reading_strings() {
    let code = String::from(r#" "Hello, World!" "#);
    let mut reader = Reader::new();
    assert_eq!(reader.read(&code).unwrap(), Str("Hello, World!".into()));
}

#[test]
fn reading_lists() {
    let code = String::from("(a 1 2 3)");
    let mut reader = Reader::new();
    match reader.read(&code).unwrap() {
        List(xs) => {
            let a1 = xs.get(0).unwrap();
            let a2 = xs.get(1).unwrap();
            let a3 = xs.get(2).unwrap();
            let a4 = xs.get(3).unwrap();
            assert_eq!(a1, &Sym("a".into()));
            assert_eq!(a2, &Num(1.0));
            assert_eq!(a3, &Num(2.0));
            assert_eq!(a4, &Num(3.0));
        }
        x => panic!("Expected list but got {:?}", x),
    }

    let code2 = r#"
    (def x (= (+ 1 2) 3))
    x
    "#
    .to_string();

    reader.reset();
    let def = reader.read(&code2).unwrap();
    assert_eq!(
        def,
        Value::List(vec![
            Sym("def".into()),
            Sym("x".into()),
            Value::List(vec![
                Sym("=".into()),
                Value::List(vec![Sym("+".into()), Num(1.0), Num(2.0),]),
                Num(3.0)
            ])
        ])
    );
    reader.skip_whitespace(&code2);
    let sym = reader.read(&code2).unwrap();
    assert_eq!(sym, Sym("x".into()));

    reader.reset();
    let error_code = String::from("(a (b c)");
    assert_eq!(
        reader.read(&error_code),
        Err(ReaderError::UnbalancedParenthesis)
    )
}

#[test]
fn reading_function_calls() {
    let code = String::from("a(1 2 3)");
    let mut reader = Reader::new();
    match reader.read(&code).unwrap() {
        List(xs) => {
            let a1 = xs.get(0).unwrap();
            let a2 = xs.get(1).unwrap();
            let a3 = xs.get(2).unwrap();
            let a4 = xs.get(3).unwrap();
            assert_eq!(a1, &Sym("a".into()));
            assert_eq!(a2, &Num(1.0));
            assert_eq!(a3, &Num(2.0));
            assert_eq!(a4, &Num(3.0));
        }
        x => panic!("Expected list but got {:?}", x),
    }

    let code2 = String::from("a (1 2 3)");
    reader.reset();
    match reader.read(&code2).unwrap() {
        Sym(s) => assert_eq!(s, "a".to_string()),
        x => panic!("Symbol list but got {:?}", x),
    }
}

#[test]
fn reading_do_blocks() {
    let code = String::from("{ a 1 2 3 }");
    let mut reader = Reader::new();
    match reader.read(&code).unwrap() {
        List(xs) => {
            let a0 = xs.get(0).unwrap();
            let a1 = xs.get(1).unwrap();
            let a2 = xs.get(2).unwrap();
            let a3 = xs.get(3).unwrap();
            let a4 = xs.get(4).unwrap();
            assert_eq!(a0, &Sym("do".into()));
            assert_eq!(a1, &Sym("a".into()));
            assert_eq!(a2, &Num(1.0));
            assert_eq!(a3, &Num(2.0));
            assert_eq!(a4, &Num(3.0));
        }
        x => panic!("Expected list but got {:?}", x),
    }

    reader.reset();
    let error_code = String::from("{a {b c}");
    assert_eq!(reader.read(&error_code), Err(ReaderError::UnbalancedBraces))
}
