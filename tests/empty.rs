#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use typesafe::TypedBuilder;

#[derive(Debug, PartialEq, Eq, TypedBuilder)]
struct Empty {}

#[test]
fn empty() {
    assert_eq!(Empty::build(), Empty {});
}
