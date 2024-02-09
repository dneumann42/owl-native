use owl::{
    reader::{Reader, ReaderError},
    values::Value::{Bool, List, Num, Str, Sym},
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
    let code = String::from(r" 123 -54 0.0 .3 -.3 3.1415926 ");
    let mut reader = Reader::new();
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
    let code = String::from("hello-world test54");
    let mut reader = Reader::new();
    assert_eq!(reader.read(&code).unwrap(), Sym("hello-world".into()));
    assert_eq!(reader.read(&code).unwrap(), Sym("test54".into()));
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
            let a1 = xs.get(0).unwrap().as_ref();
            let a2 = xs.get(1).unwrap().as_ref();
            let a3 = xs.get(2).unwrap().as_ref();
            let a4 = xs.get(3).unwrap().as_ref();
            assert_eq!(a1, &Sym("a".into()));
            assert_eq!(a2, &Num(1.0));
            assert_eq!(a3, &Num(2.0));
            assert_eq!(a4, &Num(3.0));
        }
        _ => {
            assert_eq!(false, true);
        }
    }

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
            let a1 = xs.get(0).unwrap().as_ref();
            let a2 = xs.get(1).unwrap().as_ref();
            let a3 = xs.get(2).unwrap().as_ref();
            let a4 = xs.get(3).unwrap().as_ref();
            assert_eq!(a1, &Sym("a".into()));
            assert_eq!(a2, &Num(1.0));
            assert_eq!(a3, &Num(2.0));
            assert_eq!(a4, &Num(3.0));
        }
        _ => {
            assert_eq!(false, true);
        }
    }
}

#[test]
fn reading_do_blocks() {
    let code = String::from("{ a 1 2 3 }");
    let mut reader = Reader::new();
    match reader.read(&code).unwrap() {
        List(xs) => {
            let a0 = xs.get(0).unwrap().as_ref();
            let a1 = xs.get(1).unwrap().as_ref();
            let a2 = xs.get(2).unwrap().as_ref();
            let a3 = xs.get(3).unwrap().as_ref();
            let a4 = xs.get(4).unwrap().as_ref();
            assert_eq!(a0, &Sym("do".into()));
            assert_eq!(a1, &Sym("a".into()));
            assert_eq!(a2, &Num(1.0));
            assert_eq!(a3, &Num(2.0));
            assert_eq!(a4, &Num(3.0));
        }
        _ => {
            assert_eq!(false, true);
        }
    }

    reader.reset();
    let error_code = String::from("{a {b c}");
    assert_eq!(reader.read(&error_code), Err(ReaderError::UnbalancedBraces))
}
