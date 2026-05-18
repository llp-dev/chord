**anstream > adapter > strip**

# Module: adapter::strip

## Contents

**Structs**

- [`StripBytes`](#stripbytes) - Incrementally strip non-contiguous data
- [`StripBytesIter`](#stripbytesiter) - See [`StripBytes`]
- [`StripStr`](#stripstr) - Incrementally strip non-contiguous data
- [`StripStrIter`](#stripstriter) - See [`StripStr`]
- [`StrippedBytes`](#strippedbytes) - See [`strip_bytes`]
- [`StrippedStr`](#strippedstr) - See [`strip_str`]

**Functions**

- [`strip_bytes`](#strip_bytes) - Strip ANSI escapes from bytes, returning the printable content
- [`strip_str`](#strip_str) - Strip ANSI escapes from a `&str`, returning the printable content

---

## anstream::adapter::strip::StripBytes

*Struct*

Incrementally strip non-contiguous data

**Methods:**

- `fn new() -> Self` - Initial state
- `fn strip_next<'s>(self: &'s  mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` - Strip the next segment of data

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StripBytes) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StripBytes`
- **Default**
  - `fn default() -> StripBytes`



## anstream::adapter::strip::StripBytesIter

*Struct*

See [`StripBytes`]

**Generic Parameters:**
- 's

**Traits:** Eq

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **PartialEq**
  - `fn eq(self: &Self, other: &StripBytesIter<'s>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## anstream::adapter::strip::StripStr

*Struct*

Incrementally strip non-contiguous data

**Methods:**

- `fn new() -> Self` - Initial state
- `fn strip_next<'s>(self: &'s  mut Self, data: &'s str) -> StripStrIter<'s>` - Strip the next segment of data

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &StripStr) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StripStr`
- **Default**
  - `fn default() -> StripStr`



## anstream::adapter::strip::StripStrIter

*Struct*

See [`StripStr`]

**Generic Parameters:**
- 's

**Traits:** Eq

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **PartialEq**
  - `fn eq(self: &Self, other: &StripStrIter<'s>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## anstream::adapter::strip::StrippedBytes

*Struct*

See [`strip_bytes`]

**Generic Parameters:**
- 's

**Methods:**

- `fn new(bytes: &'s [u8]) -> Self` - See [`strip_bytes`]
- `fn extend(self: & mut Self, bytes: &'s [u8])` - Strip the next slice of bytes
- `fn is_empty(self: &Self) -> bool` - Report the bytes has been exhausted
- `fn into_vec(self: Self) -> Vec<u8>` - Create a [`Vec`] of the printable content

**Traits:** Eq

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **PartialEq**
  - `fn eq(self: &Self, other: &StrippedBytes<'s>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StrippedBytes<'s>`
- **Default**
  - `fn default() -> StrippedBytes<'s>`



## anstream::adapter::strip::StrippedStr

*Struct*

See [`strip_str`]

**Generic Parameters:**
- 's

**Methods:**

- `fn to_string(self: &Self) -> String` - Create a [`String`] of the printable content

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> StrippedStr<'s>`
- **Default**
  - `fn default() -> StrippedStr<'s>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result` - **Note:** this does *not* exhaust the [`Iterator`]
- **PartialEq**
  - `fn eq(self: &Self, other: &StrippedStr<'s>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## anstream::adapter::strip::strip_bytes

*Function*

Strip ANSI escapes from bytes, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_bytes(styled_text.as_bytes()).into_vec();
assert_eq!(plain_str.as_slice(), &b"foo bar"[..]);
```

```rust
fn strip_bytes(data: &[u8]) -> StrippedBytes
```



## anstream::adapter::strip::strip_str

*Function*

Strip ANSI escapes from a `&str`, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

For non-contiguous data, see [`StripStr`].

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_str(&styled_text).to_string();
assert_eq!(plain_str, "foo bar");
```

```rust
fn strip_str(data: &str) -> StrippedStr
```



