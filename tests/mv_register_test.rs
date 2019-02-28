extern crate concur;

use concur::mv_register::*;

#[test]
fn it_assigns() {
    let mut first = MVRegister::new();

    first.upsert("a".to_string(), "dog".to_string());
    assert_eq!("dog".to_string(), first.clone().get(&"a"));

    let mut second = first.clone();
    second.upsert("b".to_string(), "cat".to_string());

    first.merge(second);
    assert_eq!("cat".to_string(), first.clone().get(&"b"));

    second = first.clone();
    second.upsert("c".to_string(), "pup1".to_string());
    second.upsert("c".to_string(), "pup2".to_string());
    first.upsert("c".to_string(), "pup3".to_string());

    first.merge(second);
    assert_eq!("pup3".to_string(), first.get(&"c"));
}
