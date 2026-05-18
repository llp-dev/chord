**rustls > error > other_error**

# Module: error::other_error

## Contents

**Structs**

- [`OtherError`](#othererror) - Any other error that cannot be expressed by a more specific [`Error`] variant.

---

## rustls::error::other_error::OtherError

*Struct*

Any other error that cannot be expressed by a more specific [`Error`] variant.

For example, an `OtherError` could be produced by a custom crypto provider
exposing a provider specific error.

Enums holding this type will never compare equal to each other.

**Tuple Struct**: `()`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> OtherError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, _other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



