*[wasmparser](../index.md) / [binary_reader](index.md)*

---

# Module `binary_reader`

## Contents

- [Structs](#structs)
  - [`BinaryReaderError`](#binaryreadererror)
  - [`BinaryReaderErrorInner`](#binaryreadererrorinner)
  - [`BinaryReader`](#binaryreader)
  - [`BinaryReaderIter`](#binaryreaderiter)
- [Enums](#enums)
  - [`BinaryReaderErrorKind`](#binaryreadererrorkind)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Constants](#constants)
  - [`WASM_MAGIC_NUMBER`](#wasm-magic-number)
- [Macros](#macros)
  - [`define_feature_accessor!`](#define-feature-accessor)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BinaryReaderError`](#binaryreadererror) | struct | A binary reader for WebAssembly modules. |
| [`BinaryReaderErrorInner`](#binaryreadererrorinner) | struct |  |
| [`BinaryReader`](#binaryreader) | struct | A binary reader of the WebAssembly structures and types. |
| [`BinaryReaderIter`](#binaryreaderiter) | struct | Iterator returned from [`BinaryReader::read_iter`]. |
| [`BinaryReaderErrorKind`](#binaryreadererrorkind) | enum |  |
| [`Result`](#result) | type | The result for `BinaryReader` operations. |
| [`WASM_MAGIC_NUMBER`](#wasm-magic-number) | const |  |
| [`define_feature_accessor!`](#define-feature-accessor) | macro |  |

## Structs

### `BinaryReaderError`

```rust
struct BinaryReaderError {
    inner: Box<BinaryReaderErrorInner>,
}
```

A binary reader for WebAssembly modules.

#### Implementations

- <span id="binaryreadererror-new"></span>`fn _new(kind: BinaryReaderErrorKind, message: String, offset: usize) -> Self` — [`BinaryReaderErrorKind`](#binaryreadererrorkind), [`String`](../prelude/index.md#string)

- <span id="binaryreadererror-new"></span>`fn new(message: impl Into<String>, offset: usize) -> Self` — [`String`](../prelude/index.md#string)

- <span id="binaryreadererror-invalid"></span>`fn invalid(msg: &'static str, offset: usize) -> Self`

- <span id="binaryreadererror-fmt"></span>`fn fmt(args: fmt::Arguments<'_>, offset: usize) -> Self`

- <span id="binaryreadererror-eof"></span>`fn eof(offset: usize, needed_hint: usize) -> Self`

- <span id="binaryreadererror-kind"></span>`fn kind(&mut self) -> BinaryReaderErrorKind` — [`BinaryReaderErrorKind`](#binaryreadererrorkind)

- <span id="binaryreadererror-message"></span>`fn message(&self) -> &str`

  Get this error's message.

- <span id="binaryreadererror-offset"></span>`fn offset(&self) -> usize`

  Get the offset within the Wasm binary where the error occurred.

- <span id="binaryreadererror-set-message"></span>`fn set_message(&mut self, message: &str)`

#### Trait Implementations

##### `impl Clone for BinaryReaderError`

- <span id="binaryreadererror-clone"></span>`fn clone(&self) -> BinaryReaderError` — [`BinaryReaderError`](#binaryreadererror)

##### `impl Debug for BinaryReaderError`

- <span id="binaryreadererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BinaryReaderError`

- <span id="binaryreadererror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BinaryReaderError`

##### `impl ToString for BinaryReaderError`

- <span id="binaryreadererror-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../prelude/index.md#string)

### `BinaryReaderErrorInner`

```rust
struct BinaryReaderErrorInner {
    message: String,
    kind: BinaryReaderErrorKind,
    offset: usize,
    needed_hint: Option<usize>,
}
```

#### Trait Implementations

##### `impl Clone for BinaryReaderErrorInner`

- <span id="binaryreadererrorinner-clone"></span>`fn clone(&self) -> BinaryReaderErrorInner` — [`BinaryReaderErrorInner`](#binaryreadererrorinner)

##### `impl Debug for BinaryReaderErrorInner`

- <span id="binaryreadererrorinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BinaryReader<'a>`

```rust
struct BinaryReader<'a> {
    buffer: &'a [u8],
    position: usize,
    original_offset: usize,
}
```

A binary reader of the WebAssembly structures and types.

#### Implementations

- <span id="binaryreader-new"></span>`fn new(data: &[u8], original_offset: usize) -> BinaryReader<'_>` — [`BinaryReader`](#binaryreader)

  Creates a new binary reader which will parse the `data` provided.

  

  The `original_offset` provided is used for byte offsets in errors that

  are generated. That offset is added to the current position in `data`.

  This can be helpful when `data` is just a window of a view into a larger

  wasm binary perhaps not even entirely stored locally.

  

  The returned binary reader will have all features known to this crate

  enabled. To reject binaries that aren't valid unless a certain feature

  is enabled use the `BinaryReader::new_features` constructor instead.

- <span id="binaryreader-shrink"></span>`fn shrink(&self) -> BinaryReader<'a>` — [`BinaryReader`](#binaryreader)

  "Shrinks" this binary reader to retain only the buffer left-to-parse.

  

  The primary purpose of this method is to change the return value of the

  `range()` method. That method returns the range of the original buffer

  within the wasm binary so calling `range()` on the returned

  `BinaryReader` will return a smaller range than if `range()` is called

  on `self`.

  

  Otherwise parsing values from either `self` or the return value should

  return the same thing.

- <span id="binaryreader-original-position"></span>`fn original_position(&self) -> usize`

  Gets the original position of the binary reader.

- <span id="binaryreader-range"></span>`fn range(&self) -> Range<usize>`

  Returns a range from the starting offset to the end of the buffer.

- <span id="binaryreader-remaining-buffer"></span>`fn remaining_buffer(&self) -> &'a [u8]`

- <span id="binaryreader-ensure-has-byte"></span>`fn ensure_has_byte(&self) -> Result<()>` — [`Result`](#result)

- <span id="binaryreader-ensure-has-bytes"></span>`fn ensure_has_bytes(&self, len: usize) -> Result<()>` — [`Result`](#result)

- <span id="binaryreader-read"></span>`fn read<T>(&mut self) -> Result<T>` — [`Result`](#result)

  Reads a value of type `T` from this binary reader, advancing the

  internal position in this reader forward as data is read.

- <span id="binaryreader-read-u7"></span>`fn read_u7(&mut self) -> Result<u8>` — [`Result`](#result)

- <span id="binaryreader-external-kind-from-byte"></span>`fn external_kind_from_byte(byte: u8, offset: usize) -> Result<ExternalKind>` — [`Result`](#result), [`ExternalKind`](../readers/core/index.md#externalkind)

- <span id="binaryreader-read-size"></span>`fn read_size(&mut self, limit: usize, desc: &str) -> Result<usize>` — [`Result`](#result)

  Reads a variable-length 32-bit size from the byte stream while checking

  against a limit.

- <span id="binaryreader-read-iter"></span>`fn read_iter<'me, T>(self: &'me mut Self, limit: usize, desc: &str) -> Result<BinaryReaderIter<'a, 'me, T>>` — [`Result`](#result), [`BinaryReaderIter`](#binaryreaderiter)

  Reads a variable-length 32-bit size from the byte stream while checking

  against a limit.

  

  Then reads that many values of type `T` and returns them as an iterator.

  

  Note that regardless of how many items are read from the returned

  iterator the items will still be parsed from this reader.

- <span id="binaryreader-eof"></span>`fn eof(&self) -> bool`

  Returns whether the `BinaryReader` has reached the end of the file.

- <span id="binaryreader-current-position"></span>`fn current_position(&self) -> usize`

  Returns the `BinaryReader`'s current position.

- <span id="binaryreader-bytes-remaining"></span>`fn bytes_remaining(&self) -> usize`

  Returns the number of bytes remaining in the `BinaryReader`.

- <span id="binaryreader-read-bytes"></span>`fn read_bytes(&mut self, size: usize) -> Result<&'a [u8]>` — [`Result`](#result)

  Advances the `BinaryReader` `size` bytes, and returns a slice from the

  current position of `size` length.

  

  # Errors

  If `size` exceeds the remaining length in `BinaryReader`.

- <span id="binaryreader-read-reader"></span>`fn read_reader(&mut self) -> Result<BinaryReader<'a>>` — [`Result`](#result), [`BinaryReader`](#binaryreader)

  Reads a length-prefixed list of bytes from this reader and returns a

  new `BinaryReader` to read that list of bytes.

- <span id="binaryreader-read-u32"></span>`fn read_u32(&mut self) -> Result<u32>` — [`Result`](#result)

  Advances the `BinaryReader` four bytes and returns a `u32`.

  # Errors

  If `BinaryReader` has less than four bytes remaining.

- <span id="binaryreader-read-u64"></span>`fn read_u64(&mut self) -> Result<u64>` — [`Result`](#result)

  Advances the `BinaryReader` eight bytes and returns a `u64`.

  # Errors

  If `BinaryReader` has less than eight bytes remaining.

- <span id="binaryreader-read-u8"></span>`fn read_u8(&mut self) -> Result<u8>` — [`Result`](#result)

  Advances the `BinaryReader` a single byte.

  

  # Errors

  

  If `BinaryReader` has no bytes remaining.

- <span id="binaryreader-eof-err"></span>`fn eof_err(&self) -> BinaryReaderError` — [`BinaryReaderError`](#binaryreadererror)

- <span id="binaryreader-read-var-u32"></span>`fn read_var_u32(&mut self) -> Result<u32>` — [`Result`](#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a `u32`.

  

  # Errors

  

  If `BinaryReader` has less than one or up to four bytes remaining, or

  the integer is larger than 32 bits.

- <span id="binaryreader-read-var-u32-big"></span>`fn read_var_u32_big(&mut self, byte: u8) -> Result<u32>` — [`Result`](#result)

- <span id="binaryreader-read-var-u64"></span>`fn read_var_u64(&mut self) -> Result<u64>` — [`Result`](#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a `u64`.

  

  # Errors

  

  If `BinaryReader` has less than one or up to eight bytes remaining, or

  the integer is larger than 64 bits.

- <span id="binaryreader-read-var-u64-big"></span>`fn read_var_u64_big(&mut self, byte: u64) -> Result<u64>` — [`Result`](#result)

- <span id="binaryreader-skip"></span>`fn skip(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<Self>` — [`Result`](#result)

  Executes `f` to skip some data in this binary reader and then returns a

  reader which will read the skipped data.

- <span id="binaryreader-skip-string"></span>`fn skip_string(&mut self) -> Result<()>` — [`Result`](#result)

  Advances the `BinaryReader` past a WebAssembly string. This method does

  not perform any utf-8 validation.

  # Errors

  If `BinaryReader` has less than four bytes, the string's length exceeds

  the remaining bytes, or the string length

  exceeds `limits::MAX_WASM_STRING_SIZE`.

- <span id="binaryreader-read-var-i32"></span>`fn read_var_i32(&mut self) -> Result<i32>` — [`Result`](#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a `i32`.

  # Errors

  If `BinaryReader` has less than one or up to four bytes remaining, or

  the integer is larger than 32 bits.

- <span id="binaryreader-read-var-i32-big"></span>`fn read_var_i32_big(&mut self, byte: u8) -> Result<i32>` — [`Result`](#result)

- <span id="binaryreader-read-var-s33"></span>`fn read_var_s33(&mut self) -> Result<i64>` — [`Result`](#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a signed 33 bit integer, returned as a `i64`.

  # Errors

  If `BinaryReader` has less than one or up to five bytes remaining, or

  the integer is larger than 33 bits.

- <span id="binaryreader-read-var-i64"></span>`fn read_var_i64(&mut self) -> Result<i64>` — [`Result`](#result)

  Advances the `BinaryReader` up to eight bytes to parse a variable

  length integer as a 64 bit integer, returned as a `i64`.

  # Errors

  If `BinaryReader` has less than one or up to eight bytes remaining, or

  the integer is larger than 64 bits.

- <span id="binaryreader-read-f32"></span>`fn read_f32(&mut self) -> Result<Ieee32>` — [`Result`](#result), [`Ieee32`](../readers/core/index.md#ieee32)

  Advances the `BinaryReader` four bytes to parse a 32 bit floating point

  number, returned as `Ieee32`.

  # Errors

  If `BinaryReader` has less than four bytes remaining.

- <span id="binaryreader-read-f64"></span>`fn read_f64(&mut self) -> Result<Ieee64>` — [`Result`](#result), [`Ieee64`](../readers/core/index.md#ieee64)

  Advances the `BinaryReader` eight bytes to parse a 64 bit floating point

  number, returned as `Ieee64`.

  # Errors

  If `BinaryReader` has less than eight bytes remaining.

- <span id="binaryreader-internal-read-string"></span>`fn internal_read_string(&mut self, len: usize) -> Result<&'a str>` — [`Result`](#result)

  (internal) Reads a fixed-size WebAssembly string from the module.

- <span id="binaryreader-read-string"></span>`fn read_string(&mut self) -> Result<&'a str>` — [`Result`](#result)

  Reads a WebAssembly string from the module.

  

  # Errors

  

  If `BinaryReader` has less than up to four bytes remaining, the string's

  length exceeds the remaining bytes, the string's length exceeds

  `limits::MAX_WASM_STRING_SIZE`, or the string contains invalid utf-8.

- <span id="binaryreader-read-unlimited-string"></span>`fn read_unlimited_string(&mut self) -> Result<&'a str>` — [`Result`](#result)

  Reads a unlimited WebAssembly string from the module.

  

  Note that this is similar to `BinaryReader::read_string` except that

  it will not limit the size of the returned string by

  `limits::MAX_WASM_STRING_SIZE`.

- <span id="binaryreader-invalid-leading-byte"></span>`fn invalid_leading_byte<T>(&self, byte: u8, desc: &str) -> Result<T>` — [`Result`](#result)

- <span id="binaryreader-invalid-leading-byte-error"></span>`fn invalid_leading_byte_error(byte: u8, desc: &str, offset: usize) -> BinaryReaderError` — [`BinaryReaderError`](#binaryreadererror)

- <span id="binaryreader-peek"></span>`fn peek(&self) -> Result<u8>` — [`Result`](#result)

- <span id="binaryreader-peek-bytes"></span>`fn peek_bytes(&self, len: usize) -> Result<&[u8]>` — [`Result`](#result)

- <span id="binaryreader-read-block-type"></span>`fn read_block_type(&mut self) -> Result<BlockType>` — [`Result`](#result), [`BlockType`](../readers/core/index.md#blocktype)

- <span id="binaryreader-is-end-then-eof"></span>`fn is_end_then_eof(&self) -> bool`

  Returns whether there is an `end` opcode followed by eof remaining in

  this reader.

- <span id="binaryreader-read-header-version"></span>`fn read_header_version(&mut self) -> Result<u32>` — [`Result`](#result)

#### Trait Implementations

##### `impl Clone for BinaryReader<'a>`

- <span id="binaryreader-clone"></span>`fn clone(&self) -> BinaryReader<'a>` — [`BinaryReader`](#binaryreader)

##### `impl Debug for BinaryReader<'a>`

- <span id="binaryreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Hash for BinaryReader<'a>`

- <span id="binaryreader-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

### `BinaryReaderIter<'a, 'me, T: FromReader<'a>>`

```rust
struct BinaryReaderIter<'a, 'me, T: FromReader<'a>> {
    remaining: usize,
    reader: &'me mut BinaryReader<'a>,
    _marker: marker::PhantomData<T>,
}
```

Iterator returned from `BinaryReader::read_iter`.

#### Trait Implementations

##### `impl<T> Drop for BinaryReaderIter<'a, '_, T>`

- <span id="binaryreaderiter-drop"></span>`fn drop(&mut self)`

##### `impl IntoIterator for BinaryReaderIter<'a, 'me, T>`

- <span id="binaryreaderiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="binaryreaderiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="binaryreaderiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for BinaryReaderIter<'a, '_, T>`

- <span id="binaryreaderiter-iterator-type-item"></span>`type Item = Result<T, BinaryReaderError>`

- <span id="binaryreaderiter-iterator-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](#result)

- <span id="binaryreaderiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `BinaryReaderErrorKind`

```rust
enum BinaryReaderErrorKind {
    Custom,
    Invalid,
}
```

#### Trait Implementations

##### `impl Clone for BinaryReaderErrorKind`

- <span id="binaryreadererrorkind-clone"></span>`fn clone(&self) -> BinaryReaderErrorKind` — [`BinaryReaderErrorKind`](#binaryreadererrorkind)

##### `impl Copy for BinaryReaderErrorKind`

##### `impl Debug for BinaryReaderErrorKind`

- <span id="binaryreadererrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = core::result::Result<T, E>;
```

The result for `BinaryReader` operations.

## Constants

### `WASM_MAGIC_NUMBER`
```rust
const WASM_MAGIC_NUMBER: &[u8; 4];
```

## Macros

### `define_feature_accessor!`

