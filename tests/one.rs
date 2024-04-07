#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use typesafe::TypedBuilder;

#[derive(Debug, PartialEq, Eq, TypedBuilder)]
struct One {
    field: bool,
}

#[test]
fn one() {
    assert_eq!(One::build().field(true), One { field: true });
    assert_eq!(One::build().field(false), One { field: false });
}
