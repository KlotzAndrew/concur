extern crate concur;

use concur::twophase_set::*;

#[test]
fn it_inserts_and_removes() {
    let mut crdt = TwoPhaseSet::new();
    assert!(crdt.value().is_empty());

    crdt.insert("a".to_string());
    assert!(crdt.value().contains("a"));

    crdt.remove("a".to_string());
    assert!(!crdt.value().contains("a"));
}

#[test]
fn it_merges() {
    let mut first = TwoPhaseSet::new();
    let mut second = TwoPhaseSet::new();
    let mut third = TwoPhaseSet::new();

    first.insert("a".to_string());
    second.insert("b".to_string());
    second.insert("c".to_string());

    third.remove("a".to_string());
    first.remove("b".to_string());

    second.merge(third);
    assert!(second.value().contains("b"));
    assert!(second.value().contains("c"));

    first.merge(second);
    assert!(!first.value().contains("a"));
    assert!(!first.value().contains("b"));
    assert!(first.value().contains("c"));
}
