extern crate concur;

use concur::pncounter::*;

#[test]
fn it_increments_and_decrements() {
    let mut crdt = PNCounter::new();
    assert_eq!(0, crdt.value());

    crdt.inc();
    assert_eq!(1, crdt.value());

    crdt.increment(10);
    assert_eq!(11, crdt.value());

    crdt.dec();
    assert_eq!(10, crdt.value());

    crdt.decrement(5);
    assert_eq!(5, crdt.value());
}

#[test]
fn it_merges() {
    let mut first = PNCounter::new();
    let mut second = PNCounter::new();
    let mut third = PNCounter::new();

    first.increment(20);

    second.increment(30);
    second.decrement(3);

    third.increment(40);
    third.decrement(3);

    second.merge(third);
    assert_eq!(64, second.value());

    first.merge(second);
    assert_eq!(84, first.value());
}
