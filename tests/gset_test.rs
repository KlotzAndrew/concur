extern crate concur;

use concur::gset::*;

#[test]
fn it_inserts() {
    let mut crdt = GSet::new();
    assert!(crdt.value().is_empty());

    crdt.insert("a".to_string());
    assert!(crdt.value().contains("a"));
}

#[test]
fn it_merges() {
    let mut first = GSet::new();
    let mut second = GSet::new();
    let mut third = GSet::new();

    first.insert("a".to_string());
    second.insert("b".to_string());
    third.insert("c".to_string());

    second.merge(&third);
    assert!(second.value().contains("b"));
    assert!(second.value().contains("c"));

    first.merge(&second);
    assert!(first.value().contains("a"));
    assert!(first.value().contains("b"));
    assert!(first.value().contains("c"));
}
