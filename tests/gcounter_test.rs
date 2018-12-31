extern crate concur;

use concur::gcounter::*;

#[test]
fn it_increments() {
    let mut crdt = GCounter::new();
    assert_eq!(0, crdt.value());

    crdt.inc();
    assert_eq!(1, crdt.value());

    crdt.increment(10);
    assert_eq!(11, crdt.value());
}

#[test]
fn it_merges() {
    let mut first = GCounter::new();
    let mut second = GCounter::new();
    let mut third = GCounter::new();

    first.increment(20);
    second.increment(30);
    third.increment(40);

    second.merge(third);
    assert_eq!(70, second.value());

    first.merge(second);
    assert_eq!(90, first.value());
}
