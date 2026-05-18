**nb**

# Module: nb

## Contents

**Macros**

- [`block`](#block) - Turns the non-blocking expression `$e` into a blocking operation.

**Enums**

- [`Error`](#error) - A non-blocking error

**Type Aliases**

- [`Result`](#result) - A non-blocking result

---

## nb::Error

*Enum*

A non-blocking error

The main use of this enum is to add a `WouldBlock` variant to an existing
error enum.

**Generic Parameters:**
- E

**Variants:**
- `Other(E)` - A different kind of error
- `WouldBlock` - This operation requires blocking behavior to complete

**Methods:**

- `fn map<T, F>(self: Self, op: F) -> Error<T>` - Maps an `Error<E>` to `Error<T>` by applying a function to a contained

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Error<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Error<E>`
- **Ord**
  - `fn cmp(self: &Self, other: &Error<E>) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error<E>) -> bool`
- **From**
  - `fn from(error: E) -> Error<E>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## nb::Result

*Type Alias*: `::core::result::Result<T, Error<E>>`

A non-blocking result



## nb::block

*Declarative Macro*

Turns the non-blocking expression `$e` into a blocking operation.

This is accomplished by continuously calling the expression `$e` until it no
longer returns `Error::WouldBlock`

# Input

An expression `$e` that evaluates to `nb::Result<T, E>`

# Output

- `Ok(t)` if `$e` evaluates to `Ok(t)`
- `Err(e)` if `$e` evaluates to `Err(nb::Error::Other(e))`

```rust
macro_rules! block {
    ($e:expr) => { ... };
}
```



