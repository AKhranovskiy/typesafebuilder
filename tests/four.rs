#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use typesafebuilder::TypeSafeBuilder;

#[derive(Debug, PartialEq, Eq, TypeSafeBuilder)]
struct Four {
    first: bool,
    second: u32,
    third: usize,
    forth: String,
}

#[test]
fn four() {
    assert_eq!(
        Four::build()
            .first(true)
            .second(5)
            .third(42)
            .forth("four".into()),
        Four {
            first: true,
            second: 5,
            third: 42,
            forth: "four".into()
        }
    );

    assert_eq!(
        Four::build()
            .second(5)
            .first(true)
            .third(42)
            .forth("four".into()),
        Four {
            first: true,
            second: 5,
            third: 42,
            forth: "four".into()
        }
    );

    assert_eq!(
        Four::build()
            .second(5)
            .first(true)
            .forth("four".into())
            .third(42),
        Four {
            first: true,
            second: 5,
            third: 42,
            forth: "four".into()
        }
    );
}
