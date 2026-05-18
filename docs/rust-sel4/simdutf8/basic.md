**simdutf8 > basic**

# Module: basic

## Contents

**Structs**

- [`Utf8Error`](#utf8error) - Simple zero-sized UTF-8 error.

**Functions**

- [`from_utf8`](#from_utf8) - Analogue to [`std::str::from_utf8()`].
- [`from_utf8_mut`](#from_utf8_mut) - Analogue to [`std::str::from_utf8_mut()`].

---

## simdutf8::basic::Utf8Error

*Struct*

Simple zero-sized UTF-8 error.

No information is provided where the error occurred or how long the invalid byte
byte sequence is.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Utf8Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Utf8Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## simdutf8::basic::from_utf8

*Function*

Analogue to [`std::str::from_utf8()`].

Checks if the passed byte sequence is valid UTF-8 and returns an
[`std::str`] reference to the passed byte slice wrapped in `Ok()` if it is.

# Errors
Will return the zero-sized Err([`Utf8Error`]) on if the input contains invalid UTF-8.

```rust
fn from_utf8(input: &[u8]) -> Result<&str, Utf8Error>
```



## simdutf8::basic::from_utf8_mut

*Function*

Analogue to [`std::str::from_utf8_mut()`].

Checks if the passed mutable byte sequence is valid UTF-8 and returns a mutable
[`std::str`] reference to the passed byte slice wrapped in `Ok()` if it is.

# Errors
Will return the zero-sized Err([`Utf8Error`]) on if the input contains invalid UTF-8.

```rust
fn from_utf8_mut(input: & mut [u8]) -> Result<& mut str, Utf8Error>
```



