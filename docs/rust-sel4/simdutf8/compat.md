**simdutf8 > compat**

# Module: compat

## Contents

**Structs**

- [`Utf8Error`](#utf8error) - UTF-8 error information compatible with [`std::str::Utf8Error`].

**Functions**

- [`from_utf8`](#from_utf8) - Analogue to [`std::str::from_utf8()`].
- [`from_utf8_mut`](#from_utf8_mut) - Analogue to [`std::str::from_utf8_mut()`].

---

## simdutf8::compat::Utf8Error

*Struct*

UTF-8 error information compatible with [`std::str::Utf8Error`].

Contains information on the location of the encountered validation error and the length of the
invalid UTF-8 sequence.

**Methods:**

- `fn valid_up_to(self: &Self) -> usize` - Analogue to [`std::str::Utf8Error::valid_up_to()`](std::str::Utf8Error#method.valid_up_to).
- `fn error_len(self: &Self) -> Option<usize>` - Analogue to [`std::str::Utf8Error::error_len()`](std::str::Utf8Error#method.error_len).

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Utf8Error`
- **PartialEq**
  - `fn eq(self: &Self, other: &Utf8Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> core::fmt::Result`



## simdutf8::compat::from_utf8

*Function*

Analogue to [`std::str::from_utf8()`].

Checks if the passed byte sequence is valid UTF-8 and returns an
[`std::str`] reference to the passed byte slice wrapped in `Ok()` if it is.

# Errors
Will return Err([`Utf8Error`]) on if the input contains invalid UTF-8 with
detailed error information.

```rust
fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error>
```



## simdutf8::compat::from_utf8_mut

*Function*

Analogue to [`std::str::from_utf8_mut()`].

Checks if the passed mutable byte sequence is valid UTF-8 and returns a mutable
[`std::str`] reference to the passed byte slice wrapped in `Ok()` if it is.

# Errors
Will return Err([`Utf8Error`]) on if the input contains invalid UTF-8 with
detailed error information.

```rust
fn from_utf8_mut(input: & mut [u8]) -> Result<& mut str, Utf8Error>
```



