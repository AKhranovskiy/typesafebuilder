# Type-safe builder for structs

`TypeSafeBuilder` is a derive macro for structs to generate a type-safe builder.

Note: requires `generic_const_exprs` feature on Nightly toolchain.

The builder interface automatically returns the complete struct when all fields are set.

## Usage

```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[derive(Debug, typesafebuilder::TypeSafeBuilder)]
struct Foo {
    first: bool,
    second: u32,
    third: String
}

let foo = Foo::build() // Initiates the builder.
.first(false)
.third("string".into()) // Call methods in any order.
.second(15); // Returns initialized `Foo` when all fields are set.
```
