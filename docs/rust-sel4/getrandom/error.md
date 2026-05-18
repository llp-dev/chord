**getrandom > error**

# Module: error

## Contents

**Structs**

- [`Error`](#error) - A small and `no_std` compatible error type

---

## getrandom::error::Error

*Struct*

A small and `no_std` compatible error type

The [`Error::raw_os_error()`] will indicate if the error is from the OS, and
if so, which error code the OS gave the application. If such an error is
encountered, please consult with your system documentation.

Internally this type is a NonZeroU32, with certain values reserved for
certain purposes, see [`Error::INTERNAL_START`] and [`Error::CUSTOM_START`].

*If this crate's `"std"` Cargo feature is enabled*, then:
- [`getrandom::Error`][Error] implements
  [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
- [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) implements
  [`From<getrandom::Error>`](https://doc.rust-lang.org/std/convert/trait.From.html).

**Tuple Struct**: `()`

**Methods:**

- `fn raw_os_error(self: Self) -> Option<i32>` - Extract the raw OS error code (if this error came from the OS)
- `fn code(self: Self) -> NonZeroU32` - Extract the bare error code.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(code: NonZeroU32) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`



