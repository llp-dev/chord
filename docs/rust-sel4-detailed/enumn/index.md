# Crate `enumn`

[![github]](https://github.com/dtolnay/enumn)&ensp;[![crates-io]](https://crates.io/crates/enumn)&ensp;[![docs-rs]](https://docs.rs/enumn)



<br>

Convert number to enum.

This crate provides a derive macro to generate a function for converting a
primitive integer into the corresponding variant of an enum.

The generated function is named `n` and has the following signature:

```rust
const IGNORE: &str = stringify! {
impl YourEnum {
    pub fn n(value: Repr) -> Option<Self>;
}
};
```

where `Repr` is an integer type of the right size as described in more
detail below.

# Example

```rust
use enumn::N;

#[derive(PartialEq, Debug, N)]
enum Status {
    LegendaryTriumph,
    QualifiedSuccess,
    FortuitousRevival,
    IndeterminateStalemate,
    RecoverableSetback,
    DireMisadventure,
    AbjectFailure,
}

fn main() {
    let s = Status::n(1);
    assert_eq!(s, Some(Status::QualifiedSuccess));

    let s = Status::n(9);
    assert_eq!(s, None);
}
```

# Signature

The generated signature depends on whether the enum has a `#[repr(..)]`
attribute. If a `repr` is specified, the input to `n` will be required to be
of that type.

```rust
#[derive(enumn::N)]
enum E0 {
    IGNORE
}

#[repr(u8)]
enum E {
    /* ... */
    IGNORE
}

// expands to:
impl E {
    pub fn n(value: u8) -> Option<Self> {
        /* ... */
        unimplemented!()
    }
}
```

On the other hand if no `repr` is specified then we get a signature that is
generic over a variety of possible types.

```rust
enum E {}

impl E {
    pub fn n<REPR: Into<i64>>(value: REPR) -> Option<Self> {
        /* ... */
        unimplemented!()
    }
}
```

# Discriminants

The conversion respects explicitly specified enum discriminants. Consider
this enum:

```rust
#[derive(enumn::N)]
enum Letter {
    A = 65,
    B = 66,
}
```

Here `Letter::n(65)` would return `Some(Letter::A)`.

*This crate has no public items to document.*
