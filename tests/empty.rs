#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use typesafe::TypedBuilder;

#[derive(Debug, PartialEq, Eq, TypedBuilder)]
struct Empty {}

#[derive(Debug, PartialEq, Eq, TypedBuilder)]
struct Unit;

#[test]
fn empty() {
    assert_eq!(Empty::build(), Empty {});
}

#[test]
fn unit() {
    assert_eq!(Unit::build(), Unit);
}
