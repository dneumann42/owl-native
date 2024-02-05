use owl::{
    reader::{Reader, ReaderError},
    values::Value,
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
    assert_eq!(reader.read(&code).unwrap(), Value::Num(123.0));
    assert_eq!(reader.read(&code).unwrap(), Value::Num(-54.0));
    assert_eq!(reader.read(&code).unwrap(), Value::Num(0.0));
    assert_eq!(reader.read(&code).unwrap(), Value::Num(0.3));
    assert_eq!(reader.read(&code).unwrap(), Value::Num(-0.3));
    assert_eq!(reader.read(&code).unwrap(), Value::Num(3.1415926));

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
    assert_eq!(reader.read(&code).unwrap(), Value::Bool(true));
    assert_eq!(reader.read(&code).unwrap(), Value::Bool(true));
    assert_eq!(reader.read(&code).unwrap(), Value::Bool(false));
    assert_eq!(reader.read(&code).unwrap(), Value::Bool(false));
}
