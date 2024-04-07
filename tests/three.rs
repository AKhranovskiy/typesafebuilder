#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use typesafebuilder::TypeSafeBuilder;

#[derive(Debug, PartialEq, Eq, TypeSafeBuilder)]
struct Three {
    first: bool,
    second: u32,
    third: usize,
}

#[test]
fn three() {
    assert_eq!(
        Three::build().first(true).second(5).third(42),
        Three {
            first: true,
            second: 5,
            third: 42
        }
    );
    assert_eq!(
        Three::build().second(5).first(true).third(42),
        Three {
            first: true,
            second: 5,
            third: 42
        }
    );
    assert_eq!(
        Three::build().second(5).third(42).first(true),
        Three {
            first: true,
            second: 5,
            third: 42
        }
    );
}
