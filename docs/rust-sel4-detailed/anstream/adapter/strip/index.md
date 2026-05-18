*[anstream](../../index.md) / [adapter](../index.md) / [strip](index.md)*

---

# Module `strip`

## Contents

- [Structs](#structs)
  - [`StrippedStr`](#strippedstr)
  - [`StripStr`](#stripstr)
  - [`StripStrIter`](#stripstriter)
  - [`StrippedBytes`](#strippedbytes)
  - [`StripBytes`](#stripbytes)
  - [`StripBytesIter`](#stripbytesiter)
  - [`Utf8Parser`](#utf8parser)
  - [`VtUtf8Receiver`](#vtutf8receiver)
- [Functions](#functions)
  - [`strip_str`](#strip-str)
  - [`next_str`](#next-str)
  - [`from_utf8_unchecked`](#from-utf8-unchecked)
  - [`is_utf8_continuation`](#is-utf8-continuation)
  - [`strip_bytes`](#strip-bytes)
  - [`next_bytes`](#next-bytes)
  - [`is_printable_bytes`](#is-printable-bytes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StrippedStr`](#strippedstr) | struct | See [`strip_str`] |
| [`StripStr`](#stripstr) | struct | Incrementally strip non-contiguous data |
| [`StripStrIter`](#stripstriter) | struct | See [`StripStr`] |
| [`StrippedBytes`](#strippedbytes) | struct | See [`strip_bytes`] |
| [`StripBytes`](#stripbytes) | struct | Incrementally strip non-contiguous data |
| [`StripBytesIter`](#stripbytesiter) | struct | See [`StripBytes`] |
| [`Utf8Parser`](#utf8parser) | struct |  |
| [`VtUtf8Receiver`](#vtutf8receiver) | struct |  |
| [`strip_str`](#strip-str) | fn | Strip ANSI escapes from a `&str`, returning the printable content |
| [`next_str`](#next-str) | fn |  |
| [`from_utf8_unchecked`](#from-utf8-unchecked) | fn |  |
| [`is_utf8_continuation`](#is-utf8-continuation) | fn |  |
| [`strip_bytes`](#strip-bytes) | fn | Strip ANSI escapes from bytes, returning the printable content |
| [`next_bytes`](#next-bytes) | fn |  |
| [`is_printable_bytes`](#is-printable-bytes) | fn |  |

## Structs

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

See [`strip_str`](#strip-str)

#### Implementations

- <span id="strippedstr-new"></span>`fn new(data: &'s str) -> Self`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

  Create a [`String`](../../../clap_builder/index.md) of the printable content

#### Trait Implementations

##### `impl Clone for StrippedStr<'s>`

- <span id="strippedstr-clone"></span>`fn clone(&self) -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

##### `impl Debug for StrippedStr<'s>`

- <span id="strippedstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedStr<'s>`

- <span id="strippedstr-default"></span>`fn default() -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

##### `impl Display for StrippedStr<'_>`

- <span id="strippedstr-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

  **Note:** this does *not* exhaust the [`Iterator`](../../../fallible_iterator/index.md)

##### `impl Eq for StrippedStr<'s>`

##### `impl IntoIterator for StrippedStr<'s>`

- <span id="strippedstr-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedstr-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedstr-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedStr<'s>`

- <span id="strippedstr-iterator-type-item"></span>`type Item = &'s str`

- <span id="strippedstr-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedStr<'s>`

- <span id="strippedstr-partialeq-eq"></span>`fn eq(&self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](#strippedstr)

##### `impl StructuralPartialEq for StrippedStr<'s>`

##### `impl ToString for StrippedStr<'s>`

- <span id="strippedstr-tostring-to-string"></span>`fn to_string(&self) -> String`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripstr-new"></span>`fn new() -> Self`

  Initial state

- <span id="stripstr-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](#stripstriter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Clone for StripStr`

- <span id="stripstr-clone"></span>`fn clone(&self) -> StripStr` — [`StripStr`](#stripstr)

##### `impl Debug for StripStr`

- <span id="stripstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripStr`

- <span id="stripstr-default"></span>`fn default() -> StripStr` — [`StripStr`](#stripstr)

##### `impl Eq for StripStr`

##### `impl PartialEq for StripStr`

- <span id="stripstr-partialeq-eq"></span>`fn eq(&self, other: &StripStr) -> bool` — [`StripStr`](#stripstr)

##### `impl StructuralPartialEq for StripStr`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

See [`StripStr`](#stripstr)

#### Trait Implementations

##### `impl Debug for StripStrIter<'s>`

- <span id="stripstriter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripStrIter<'s>`

##### `impl IntoIterator for StripStrIter<'s>`

- <span id="stripstriter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripstriter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stripstriter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripStrIter<'s>`

- <span id="stripstriter-iterator-type-item"></span>`type Item = &'s str`

- <span id="stripstriter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripStrIter<'s>`

- <span id="stripstriter-partialeq-eq"></span>`fn eq(&self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](#stripstriter)

##### `impl StructuralPartialEq for StripStrIter<'s>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

See [`strip_bytes`](#strip-bytes)

#### Implementations

- <span id="strippedbytes-new"></span>`fn new(bytes: &'s [u8]) -> Self`

  See [`strip_bytes`](#strip-bytes)

- <span id="strippedbytes-extend"></span>`fn extend(&mut self, bytes: &'s [u8])`

  Strip the next slice of bytes

  

  Used when the content is in several non-contiguous slices

  

  # Panic

  

  May panic if it is not exhausted / empty

- <span id="strippedbytes-is-empty"></span>`fn is_empty(&self) -> bool`

  Report the bytes has been exhausted

- <span id="strippedbytes-into-vec"></span>`fn into_vec(self) -> Vec<u8>`

  Create a [`Vec`](../../../addr2line/maybe_small/index.md) of the printable content

#### Trait Implementations

##### `impl Clone for StrippedBytes<'s>`

- <span id="strippedbytes-clone"></span>`fn clone(&self) -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl Debug for StrippedBytes<'s>`

- <span id="strippedbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedBytes<'s>`

- <span id="strippedbytes-default"></span>`fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl Eq for StrippedBytes<'s>`

##### `impl IntoIterator for StrippedBytes<'s>`

- <span id="strippedbytes-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedbytes-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedbytes-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedBytes<'s>`

- <span id="strippedbytes-iterator-type-item"></span>`type Item = &'s [u8]`

- <span id="strippedbytes-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedBytes<'s>`

- <span id="strippedbytes-partialeq-eq"></span>`fn eq(&self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](#strippedbytes)

##### `impl StructuralPartialEq for StrippedBytes<'s>`

### `StripBytes`

```rust
struct StripBytes {
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripbytes-new"></span>`fn new() -> Self`

  Initial state

- <span id="stripbytes-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](#stripbytesiter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Clone for StripBytes`

- <span id="stripbytes-clone"></span>`fn clone(&self) -> StripBytes` — [`StripBytes`](#stripbytes)

##### `impl Debug for StripBytes`

- <span id="stripbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripBytes`

- <span id="stripbytes-default"></span>`fn default() -> StripBytes` — [`StripBytes`](#stripbytes)

##### `impl Eq for StripBytes`

##### `impl PartialEq for StripBytes`

- <span id="stripbytes-partialeq-eq"></span>`fn eq(&self, other: &StripBytes) -> bool` — [`StripBytes`](#stripbytes)

##### `impl StructuralPartialEq for StripBytes`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

See [`StripBytes`](#stripbytes)

#### Trait Implementations

##### `impl Debug for StripBytesIter<'s>`

- <span id="stripbytesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripBytesIter<'s>`

##### `impl IntoIterator for StripBytesIter<'s>`

- <span id="stripbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stripbytesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripBytesIter<'s>`

- <span id="stripbytesiter-iterator-type-item"></span>`type Item = &'s [u8]`

- <span id="stripbytesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripBytesIter<'s>`

- <span id="stripbytesiter-partialeq-eq"></span>`fn eq(&self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](#stripbytesiter)

##### `impl StructuralPartialEq for StripBytesIter<'s>`

### `Utf8Parser`

```rust
struct Utf8Parser {
    utf8_parser: utf8parse::Parser,
}
```

#### Implementations

- <span id="utf8parser-add"></span>`fn add(&mut self, byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for Utf8Parser`

- <span id="utf8parser-clone"></span>`fn clone(&self) -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Debug for Utf8Parser`

- <span id="utf8parser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8Parser`

- <span id="utf8parser-default"></span>`fn default() -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Eq for Utf8Parser`

##### `impl PartialEq for Utf8Parser`

- <span id="utf8parser-partialeq-eq"></span>`fn eq(&self, other: &Utf8Parser) -> bool` — [`Utf8Parser`](#utf8parser)

##### `impl StructuralPartialEq for Utf8Parser`

### `VtUtf8Receiver<'a>`

```rust
struct VtUtf8Receiver<'a>(&'a mut bool);
```

#### Trait Implementations

##### `impl Receiver for VtUtf8Receiver<'_>`

- <span id="vtutf8receiver-receiver-codepoint"></span>`fn codepoint(&mut self, _: char)`

- <span id="vtutf8receiver-receiver-invalid-sequence"></span>`fn invalid_sequence(&mut self)`

## Functions

### `strip_str`

```rust
fn strip_str(data: &str) -> StrippedStr<'_>
```

Strip ANSI escapes from a `&str`, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

For non-contiguous data, see [`StripStr`](#stripstr).

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_str(&styled_text).to_string();
assert_eq!(plain_str, "foo bar");
```

### `next_str`

```rust
fn next_str<'s>(bytes: &mut &'s [u8], state: &mut anstyle_parse::state::State) -> Option<&'s str>
```

### `from_utf8_unchecked`

```rust
unsafe fn from_utf8_unchecked<'b>(bytes: &'b [u8], safety_justification: &'static str) -> &'b str
```

### `is_utf8_continuation`

```rust
fn is_utf8_continuation(b: u8) -> bool
```

### `strip_bytes`

```rust
fn strip_bytes(data: &[u8]) -> StrippedBytes<'_>
```

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

### `next_bytes`

```rust
fn next_bytes<'s>(bytes: &mut &'s [u8], state: &mut anstyle_parse::state::State, utf8parser: &mut Utf8Parser) -> Option<&'s [u8]>
```

### `is_printable_bytes`

```rust
fn is_printable_bytes(action: anstyle_parse::state::Action, byte: u8) -> bool
```

