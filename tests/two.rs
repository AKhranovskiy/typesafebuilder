#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use typesafebuilder::TypeSafeBuilder;

#[derive(Debug, PartialEq, Eq, TypeSafeBuilder)]
struct Two {
    first: bool,
    second: u32,
}

#[test]
fn two() {
    assert_eq!(
        Two::build().first(true).second(5),
        Two {
            first: true,
            second: 5
        }
    );
    assert_eq!(
        Two::build().second(5).first(true),
        Two {
            first: true,
            second: 5
        }
    );
}
