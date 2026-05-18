**uguid**

# Module: uguid

## Contents

**Modules**

- [`error`](#error)
- [`guid`](#guid)
- [`util`](#util)

**Macros**

- [`guid`](#guid) - Create a [`Guid`] from a string at compile time.
- [`mtry`](#mtry) - Macro replacement for the `?` operator, which cannot be used in

---

## Module: error



## uguid::guid

*Declarative Macro*

Create a [`Guid`] from a string at compile time.

# Examples

```
use uguid::{guid, Guid};
assert_eq!(
    guid!("01234567-89ab-cdef-0123-456789abcdef"),
    Guid::new(
        [0x67, 0x45, 0x23, 0x01],
        [0xab, 0x89],
        [0xef, 0xcd],
        0x01,
        0x23,
        [0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
    )
);
```

```rust
macro_rules! guid {
    ($s:literal) => { ... };
}
```



## Module: guid



## uguid::mtry

*Declarative Macro*

Macro replacement for the `?` operator, which cannot be used in
const functions.

```rust
macro_rules! mtry {
    ($expr:expr $(,)?) => { ... };
}
```



## Module: util



