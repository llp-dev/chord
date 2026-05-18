*[anstream](../index.md) / [adapter](index.md)*

---

# Module `adapter`

Gracefully degrade styled output

## Contents

- [Modules](#modules)
  - [`strip`](#strip)
  - [`wincon`](#wincon)
- [Structs](#structs)
  - [`StripBytes`](#stripbytes)
  - [`StripBytesIter`](#stripbytesiter)
  - [`StripStr`](#stripstr)
  - [`StripStrIter`](#stripstriter)
  - [`StrippedBytes`](#strippedbytes)
  - [`StrippedStr`](#strippedstr)
  - [`WinconBytes`](#winconbytes)
  - [`WinconBytesIter`](#winconbytesiter)
- [Functions](#functions)
  - [`strip_bytes`](#strip-bytes)
  - [`strip_str`](#strip-str)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`strip`](#strip) | mod |  |
| [`wincon`](#wincon) | mod |  |
| [`StripBytes`](#stripbytes) | struct |  |
| [`StripBytesIter`](#stripbytesiter) | struct |  |
| [`StripStr`](#stripstr) | struct |  |
| [`StripStrIter`](#stripstriter) | struct |  |
| [`StrippedBytes`](#strippedbytes) | struct |  |
| [`StrippedStr`](#strippedstr) | struct |  |
| [`WinconBytes`](#winconbytes) | struct |  |
| [`WinconBytesIter`](#winconbytesiter) | struct |  |
| [`strip_bytes`](#strip-bytes) | fn |  |
| [`strip_str`](#strip-str) | fn |  |

## Modules

- [`strip`](strip/index.md)
- [`wincon`](wincon/index.md)

## Structs

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

- <span id="stripbytes-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](strip/index.md#stripbytesiter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Clone for StripBytes`

- <span id="stripbytes-clone"></span>`fn clone(&self) -> StripBytes` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl Debug for StripBytes`

- <span id="stripbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripBytes`

- <span id="stripbytes-default"></span>`fn default() -> StripBytes` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl Eq for StripBytes`

##### `impl PartialEq for StripBytes`

- <span id="stripbytes-partialeq-eq"></span>`fn eq(&self, other: &StripBytes) -> bool` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl StructuralPartialEq for StripBytes`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

See [`StripBytes`](strip/index.md)

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

- <span id="stripbytesiter-partialeq-eq"></span>`fn eq(&self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](strip/index.md#stripbytesiter)

##### `impl StructuralPartialEq for StripBytesIter<'s>`

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

- <span id="stripstr-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](strip/index.md#stripstriter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Clone for StripStr`

- <span id="stripstr-clone"></span>`fn clone(&self) -> StripStr` — [`StripStr`](strip/index.md#stripstr)

##### `impl Debug for StripStr`

- <span id="stripstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripStr`

- <span id="stripstr-default"></span>`fn default() -> StripStr` — [`StripStr`](strip/index.md#stripstr)

##### `impl Eq for StripStr`

##### `impl PartialEq for StripStr`

- <span id="stripstr-partialeq-eq"></span>`fn eq(&self, other: &StripStr) -> bool` — [`StripStr`](strip/index.md#stripstr)

##### `impl StructuralPartialEq for StripStr`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

See [`StripStr`](strip/index.md)

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

- <span id="stripstriter-partialeq-eq"></span>`fn eq(&self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](strip/index.md#stripstriter)

##### `impl StructuralPartialEq for StripStrIter<'s>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

See [`strip_bytes`](strip/index.md)

#### Implementations

- <span id="strippedbytes-new"></span>`fn new(bytes: &'s [u8]) -> Self`

  See [`strip_bytes`](strip/index.md)

- <span id="strippedbytes-extend"></span>`fn extend(&mut self, bytes: &'s [u8])`

  Strip the next slice of bytes

  

  Used when the content is in several non-contiguous slices

  

  # Panic

  

  May panic if it is not exhausted / empty

- <span id="strippedbytes-is-empty"></span>`fn is_empty(&self) -> bool`

  Report the bytes has been exhausted

- <span id="strippedbytes-into-vec"></span>`fn into_vec(self) -> Vec<u8>`

  Create a [`Vec`](../../addr2line/maybe_small/index.md) of the printable content

#### Trait Implementations

##### `impl Clone for StrippedBytes<'s>`

- <span id="strippedbytes-clone"></span>`fn clone(&self) -> StrippedBytes<'s>` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl Debug for StrippedBytes<'s>`

- <span id="strippedbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedBytes<'s>`

- <span id="strippedbytes-default"></span>`fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl Eq for StrippedBytes<'s>`

##### `impl IntoIterator for StrippedBytes<'s>`

- <span id="strippedbytes-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedbytes-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedbytes-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedBytes<'s>`

- <span id="strippedbytes-iterator-type-item"></span>`type Item = &'s [u8]`

- <span id="strippedbytes-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedBytes<'s>`

- <span id="strippedbytes-partialeq-eq"></span>`fn eq(&self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl StructuralPartialEq for StrippedBytes<'s>`

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

See [`strip_str`](strip/index.md)

#### Implementations

- <span id="strippedstr-new"></span>`fn new(data: &'s str) -> Self`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

  Create a [`String`](../../clap_builder/index.md) of the printable content

#### Trait Implementations

##### `impl Clone for StrippedStr<'s>`

- <span id="strippedstr-clone"></span>`fn clone(&self) -> StrippedStr<'s>` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl Debug for StrippedStr<'s>`

- <span id="strippedstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedStr<'s>`

- <span id="strippedstr-default"></span>`fn default() -> StrippedStr<'s>` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl Display for StrippedStr<'_>`

- <span id="strippedstr-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

  **Note:** this does *not* exhaust the [`Iterator`](../../fallible_iterator/index.md)

##### `impl Eq for StrippedStr<'s>`

##### `impl IntoIterator for StrippedStr<'s>`

- <span id="strippedstr-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedstr-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedstr-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedStr<'s>`

- <span id="strippedstr-iterator-type-item"></span>`type Item = &'s str`

- <span id="strippedstr-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedStr<'s>`

- <span id="strippedstr-partialeq-eq"></span>`fn eq(&self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl StructuralPartialEq for StrippedStr<'s>`

##### `impl ToString for StrippedStr<'s>`

- <span id="strippedstr-tostring-to-string"></span>`fn to_string(&self) -> String`

### `WinconBytes`

```rust
struct WinconBytes {
    parser: anstyle_parse::Parser,
    capture: WinconCapture,
}
```

Incrementally convert to wincon calls for non-contiguous data

#### Implementations

- <span id="winconbytes-new"></span>`fn new() -> Self`

  Initial state

- <span id="winconbytes-extract-next"></span>`fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](wincon/index.md#winconbytesiter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Clone for WinconBytes`

- <span id="winconbytes-clone"></span>`fn clone(&self) -> WinconBytes` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl Debug for WinconBytes`

- <span id="winconbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconBytes`

- <span id="winconbytes-default"></span>`fn default() -> WinconBytes` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl Eq for WinconBytes`

##### `impl PartialEq for WinconBytes`

- <span id="winconbytes-partialeq-eq"></span>`fn eq(&self, other: &WinconBytes) -> bool` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl StructuralPartialEq for WinconBytes`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

See [`WinconBytes`](wincon/index.md)

#### Trait Implementations

##### `impl Debug for WinconBytesIter<'s>`

- <span id="winconbytesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WinconBytesIter<'s>`

##### `impl IntoIterator for WinconBytesIter<'s>`

- <span id="winconbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="winconbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="winconbytesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WinconBytesIter<'_>`

- <span id="winconbytesiter-iterator-type-item"></span>`type Item = (Style, String)`

- <span id="winconbytesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for WinconBytesIter<'s>`

- <span id="winconbytesiter-partialeq-eq"></span>`fn eq(&self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](wincon/index.md#winconbytesiter)

##### `impl StructuralPartialEq for WinconBytesIter<'s>`

## Functions

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

### `strip_str`

```rust
fn strip_str(data: &str) -> StrippedStr<'_>
```

Strip ANSI escapes from a `&str`, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

For non-contiguous data, see [`StripStr`](strip/index.md).

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_str(&styled_text).to_string();
assert_eq!(plain_str, "foo bar");
```

