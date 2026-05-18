*[serde_json](../index.md) / [read](index.md)*

---

# Module `read`

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Position`](#position)
  - [`IoRead`](#ioread)
  - [`SliceRead`](#sliceread)
  - [`StrRead`](#strread)
- [Enums](#enums)
  - [`Reference`](#reference)
- [Traits](#traits)
  - [`Read`](#read)
  - [`Fused`](#fused)
- [Functions](#functions)
  - [`is_escape`](#is-escape)
  - [`next_or_eof`](#next-or-eof)
  - [`peek_or_eof`](#peek-or-eof)
  - [`error`](#error)
  - [`as_str`](#as-str)
  - [`parse_escape`](#parse-escape)
  - [`parse_unicode_escape`](#parse-unicode-escape)
  - [`push_wtf8_codepoint`](#push-wtf8-codepoint)
  - [`ignore_escape`](#ignore-escape)
  - [`decode_hex_val_slow`](#decode-hex-val-slow)
  - [`build_hex_table`](#build-hex-table)
  - [`decode_four_hex_digits`](#decode-four-hex-digits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Position`](#position) | struct |  |
| [`IoRead`](#ioread) | struct | JSON input source that reads from a std::io input stream. |
| [`SliceRead`](#sliceread) | struct | JSON input source that reads from a slice of bytes. |
| [`StrRead`](#strread) | struct | JSON input source that reads from a UTF-8 string. |
| [`Reference`](#reference) | enum |  |
| [`Read`](#read) | trait | Trait used by the deserializer for iterating over input. |
| [`Fused`](#fused) | trait | Marker for whether StreamDeserializer can implement FusedIterator. |
| [`is_escape`](#is-escape) | fn |  |
| [`next_or_eof`](#next-or-eof) | fn |  |
| [`peek_or_eof`](#peek-or-eof) | fn |  |
| [`error`](#error) | fn |  |
| [`as_str`](#as-str) | fn |  |
| [`parse_escape`](#parse-escape) | fn | Parses a JSON escape sequence and appends it into the scratch space. |
| [`parse_unicode_escape`](#parse-unicode-escape) | fn | Parses a JSON \u escape and appends it into the scratch space. |
| [`push_wtf8_codepoint`](#push-wtf8-codepoint) | fn | Adds a WTF-8 codepoint to the end of the buffer. |
| [`ignore_escape`](#ignore-escape) | fn | Parses a JSON escape sequence and discards the value. |
| [`decode_hex_val_slow`](#decode-hex-val-slow) | fn |  |
| [`build_hex_table`](#build-hex-table) | fn |  |
| [`decode_four_hex_digits`](#decode-four-hex-digits) | fn |  |

## Modules

- [`private`](private/index.md)

## Structs

### `Position`

```rust
struct Position {
    pub line: usize,
    pub column: usize,
}
```

### `IoRead<R>`

```rust
struct IoRead<R>
where
    R: io::Read {
    iter: crate::iter::LineColIterator<io::Bytes<R>>,
    ch: Option<u8>,
}
```

JSON input source that reads from a std::io input stream.

#### Fields

- **`ch`**: `Option<u8>`

  Temporary storage of peeked byte.

#### Implementations

- <span id="ioread-new"></span>`fn new(reader: R) -> Self`

  Create a JSON input source to read from a std::io input stream.

  

  When reading from a source against which short reads are not efficient, such

  as a `File`, you will want to apply your own buffering because serde_json

  will not buffer the input. See [`std::io::BufReader`](../../flate2/bufreader/index.md).

#### Trait Implementations

##### `impl<R> Read for IoRead<R>`

##### `impl<R> Sealed for IoRead<R>`

### `SliceRead<'a>`

```rust
struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}
```

JSON input source that reads from a slice of bytes.

#### Fields

- **`index`**: `usize`

  Index of the *next* byte that will be returned by next() or peek().

#### Implementations

- <span id="sliceread-new"></span>`fn new(slice: &'a [u8]) -> Self`

  Create a JSON input source to read from a slice of bytes.

- <span id="sliceread-position-of-index"></span>`fn position_of_index(&self, i: usize) -> Position` — [`Position`](#position)

- <span id="sliceread-skip-to-escape"></span>`fn skip_to_escape(&mut self, forbid_control_characters: bool)`

- <span id="sliceread-skip-to-escape-slow"></span>`fn skip_to_escape_slow(&mut self)`

- <span id="sliceread-parse-str-bytes"></span>`fn parse_str_bytes<'s, T, F>(self: &'s mut Self, scratch: &'s mut Vec<u8>, validate: bool, result: F) -> Result<Reference<'a, 's, T>>` — [`Result`](../error/index.md#result), [`Reference`](#reference)

  The big optimization here over IoRead is that if the string contains no

  backslash escape sequences, the returned &str is a slice of the raw JSON

  data so we avoid copying into the scratch space.

#### Trait Implementations

##### `impl Fused for SliceRead<'a>`

##### `impl Read for SliceRead<'a>`

##### `impl Sealed for SliceRead<'a>`

### `StrRead<'a>`

```rust
struct StrRead<'a> {
    delegate: SliceRead<'a>,
}
```

JSON input source that reads from a UTF-8 string.

#### Implementations

- <span id="strread-new"></span>`fn new(s: &'a str) -> Self`

  Create a JSON input source to read from a UTF-8 string.

#### Trait Implementations

##### `impl Fused for StrRead<'a>`

##### `impl Read for StrRead<'a>`

##### `impl Sealed for StrRead<'a>`

## Enums

### `Reference<'b, 'c, T>`

```rust
enum Reference<'b, 'c, T>
where
    T: ?Sized + 'static {
    Borrowed(&'b T),
    Copied(&'c T),
}
```

#### Trait Implementations

##### `impl<T> Deref for Reference<'b, 'c, T>`

- <span id="reference-deref-type-target"></span>`type Target = T`

- <span id="reference-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> Receiver for Reference<'b, 'c, T>`

- <span id="reference-receiver-type-target"></span>`type Target = T`

## Traits

### `Read<'de>`

```rust
trait Read<'de>: private::Sealed { ... }
```

Trait used by the deserializer for iterating over input. This is manually
"specialized" for iterating over `&[u8]`. Once feature(specialization) is
stable we can use actual specialization.

This trait is sealed and cannot be implemented for types outside of
`serde_json`.

#### Implementors

- [`IoRead`](#ioread)
- [`SliceRead`](#sliceread)
- [`StrRead`](#strread)
- `&mut R`

### `Fused`

```rust
trait Fused: private::Sealed { ... }
```

Marker for whether StreamDeserializer can implement FusedIterator.

#### Implementors

- [`SliceRead`](#sliceread)
- [`StrRead`](#strread)

## Functions

### `is_escape`

```rust
fn is_escape(ch: u8, including_control_characters: bool) -> bool
```

### `next_or_eof`

```rust
fn next_or_eof<'de, R>(read: &mut R) -> crate::error::Result<u8>
where
    R: ?Sized + Read<'de>
```

### `peek_or_eof`

```rust
fn peek_or_eof<'de, R>(read: &mut R) -> crate::error::Result<u8>
where
    R: ?Sized + Read<'de>
```

### `error`

```rust
fn error<'de, R, T>(read: &R, reason: crate::error::ErrorCode) -> crate::error::Result<T>
where
    R: ?Sized + Read<'de>
```

### `as_str`

```rust
fn as_str<'de, 's, R: Read<'de>>(read: &R, slice: &'s [u8]) -> crate::error::Result<&'s str>
```

### `parse_escape`

```rust
fn parse_escape<'de, R: Read<'de>>(read: &mut R, validate: bool, scratch: &mut alloc::vec::Vec<u8>) -> crate::error::Result<()>
```

Parses a JSON escape sequence and appends it into the scratch space. Assumes
the previous byte read was a backslash.

### `parse_unicode_escape`

```rust
fn parse_unicode_escape<'de, R: Read<'de>>(read: &mut R, validate: bool, scratch: &mut alloc::vec::Vec<u8>) -> crate::error::Result<()>
```

Parses a JSON \u escape and appends it into the scratch space. Assumes `\u`
has just been read.

### `push_wtf8_codepoint`

```rust
fn push_wtf8_codepoint(n: u32, scratch: &mut alloc::vec::Vec<u8>)
```

Adds a WTF-8 codepoint to the end of the buffer. This is a more efficient
implementation of String::push. The codepoint may be a surrogate.

### `ignore_escape`

```rust
fn ignore_escape<'de, R>(read: &mut R) -> crate::error::Result<()>
where
    R: ?Sized + Read<'de>
```

Parses a JSON escape sequence and discards the value. Assumes the previous
byte read was a backslash.

### `decode_hex_val_slow`

```rust
const fn decode_hex_val_slow(val: u8) -> Option<u8>
```

### `build_hex_table`

```rust
const fn build_hex_table(shift: usize) -> [i16; 256]
```

### `decode_four_hex_digits`

```rust
fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16>
```

