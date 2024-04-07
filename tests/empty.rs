#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use typesafebuilder::TypeSafeBuilder;

#[derive(Debug, PartialEq, Eq, TypeSafeBuilder)]
struct Empty {}

#[derive(Debug, PartialEq, Eq, TypeSafeBuilder)]
struct Unit;

#[test]
fn empty() {
    assert_eq!(Empty::build(), Empty {});
}

#[test]
fn unit() {
    assert_eq!(Unit::build(), Unit);
}
