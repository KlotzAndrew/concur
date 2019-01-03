extern crate concur;

use concur::lww_register::*;

#[test]
fn it_assigns() {
    let mut crdt = LWWRegister::new();
    assert_eq!(0, crdt.data);

    assert!(crdt.assign(1, 1).is_ok());
    assert_eq!(1, crdt.data);

    assert!(!crdt.assign(2, 1).is_ok());
    assert_eq!(1, crdt.data);

    assert!(crdt.assign(1, 1).is_ok());
    assert_eq!(1, crdt.data);

    assert!(crdt.assign(2, 2).is_ok());
    assert_eq!(2, crdt.data);
}
