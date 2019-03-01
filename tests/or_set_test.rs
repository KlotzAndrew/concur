extern crate concur;

use concur::or_set::*;

#[test]
fn it_assigns() {
    let mut first = ORSet::new();

    first.add("a");
    first.remove("a");

    first.add("c");

    let mut second = first.clone();

    second.add("b");

    first.merge(&second);
    let value = first.value();
    assert!(!value.contains("a"));
    assert!(value.contains("b"));
    assert!(value.contains("c"));
}
