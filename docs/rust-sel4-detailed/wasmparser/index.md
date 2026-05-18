# Crate `wasmparser`

A simple event-driven library for parsing WebAssembly binary files
(or streams).

The parser library reports events as they happen and only stores
parsing information for a brief period of time, making it very fast
and memory-efficient. The event-driven model, however, has some drawbacks.
If you need random access to the entire WebAssembly data-structure,
this is not the right library for you. You could however, build such
a data-structure using this library.

To get started, create a [`Parser`](#parser) using `Parser::new` and then follow
the examples documented for `Parser::parse` or `Parser::parse_all`.

## Contents

- [Modules](#modules)
  - [`prelude`](#prelude)
  - [`arity`](#arity)
  - [`binary_reader`](#binary-reader)
  - [`features`](#features)
  - [`limits`](#limits)
  - [`parser`](#parser)
  - [`readers`](#readers)
  - [`core`](#core)
- [Structs](#structs)
  - [`BinaryReader`](#binaryreader)
  - [`BinaryReaderError`](#binaryreadererror)
  - [`WasmFeatures`](#wasmfeatures)
  - [`ParserCounts`](#parsercounts)
  - [`Parser`](#parser)
  - [`SectionLimited`](#sectionlimited)
  - [`SectionLimitedIntoIter`](#sectionlimitedintoiter)
  - [`SectionLimitedIntoIterWithOffsets`](#sectionlimitedintoiterwithoffsets)
  - [`Subsections`](#subsections)
- [Enums](#enums)
  - [`Encoding`](#encoding)
  - [`Order`](#order)
  - [`State`](#state)
  - [`Chunk`](#chunk)
  - [`Payload`](#payload)
- [Traits](#traits)
  - [`ModuleArity`](#modulearity)
  - [`FromReader`](#fromreader)
  - [`Subsection`](#subsection)
- [Functions](#functions)
  - [`visit_block`](#visit-block)
  - [`visit_loop`](#visit-loop)
  - [`visit_if`](#visit-if)
  - [`visit_else`](#visit-else)
  - [`visit_end`](#visit-end)
  - [`visit_br`](#visit-br)
  - [`visit_br_if`](#visit-br-if)
  - [`visit_br_table`](#visit-br-table)
  - [`visit_return`](#visit-return)
  - [`visit_call`](#visit-call)
  - [`visit_call_indirect`](#visit-call-indirect)
  - [`visit_struct_new`](#visit-struct-new)
  - [`visit_struct_new_desc`](#visit-struct-new-desc)
  - [`visit_array_new_fixed`](#visit-array-new-fixed)
  - [`visit_br_on_cast`](#visit-br-on-cast)
  - [`visit_br_on_cast_fail`](#visit-br-on-cast-fail)
  - [`visit_typed_select_multi`](#visit-typed-select-multi)
  - [`visit_return_call`](#visit-return-call)
  - [`visit_return_call_indirect`](#visit-return-call-indirect)
  - [`visit_try_table`](#visit-try-table)
  - [`visit_throw`](#visit-throw)
  - [`visit_try`](#visit-try)
  - [`visit_catch`](#visit-catch)
  - [`visit_delegate`](#visit-delegate)
  - [`visit_catch_all`](#visit-catch-all)
  - [`visit_call_ref`](#visit-call-ref)
  - [`visit_return_call_ref`](#visit-return-call-ref)
  - [`visit_br_on_null`](#visit-br-on-null)
  - [`visit_br_on_non_null`](#visit-br-on-non-null)
  - [`visit_cont_bind`](#visit-cont-bind)
  - [`visit_suspend`](#visit-suspend)
  - [`visit_resume`](#visit-resume)
  - [`visit_resume_throw`](#visit-resume-throw)
  - [`visit_switch`](#visit-switch)
  - [`visit_br_on_cast_desc`](#visit-br-on-cast-desc)
  - [`visit_br_on_cast_desc_fail`](#visit-br-on-cast-desc-fail)
  - [`usize_to_u64`](#usize-to-u64)
  - [`section`](#section)
  - [`single_item`](#single-item)
  - [`delimited`](#delimited)
  - [`clear_hint`](#clear-hint)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Constants](#constants)
  - [`WASM_MODULE_VERSION`](#wasm-module-version)
  - [`WASM_COMPONENT_VERSION`](#wasm-component-version)
  - [`KIND_MODULE`](#kind-module)
  - [`KIND_COMPONENT`](#kind-component)
  - [`CUSTOM_SECTION`](#custom-section)
  - [`TYPE_SECTION`](#type-section)
  - [`IMPORT_SECTION`](#import-section)
  - [`FUNCTION_SECTION`](#function-section)
  - [`TABLE_SECTION`](#table-section)
  - [`MEMORY_SECTION`](#memory-section)
  - [`GLOBAL_SECTION`](#global-section)
  - [`EXPORT_SECTION`](#export-section)
  - [`START_SECTION`](#start-section)
  - [`ELEMENT_SECTION`](#element-section)
  - [`CODE_SECTION`](#code-section)
  - [`DATA_SECTION`](#data-section)
  - [`DATA_COUNT_SECTION`](#data-count-section)
  - [`TAG_SECTION`](#tag-section)
- [Macros](#macros)
  - [`_for_each_operator_group!`](#for-each-operator-group)
  - [`define_for_each_non_simd_operator!`](#define-for-each-non-simd-operator)
  - [`format_err!`](#format-err)
  - [`bail!`](#bail)
  - [`define_wasm_features!`](#define-wasm-features)
  - [`foreach_wasm_feature!`](#foreach-wasm-feature)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`prelude`](#prelude) | mod | A small "prelude" to use throughout this crate. |
| [`arity`](#arity) | mod |  |
| [`binary_reader`](#binary-reader) | mod |  |
| [`features`](#features) | mod |  |
| [`limits`](#limits) | mod |  |
| [`parser`](#parser) | mod |  |
| [`readers`](#readers) | mod |  |
| [`core`](#core) | mod |  |
| [`BinaryReader`](#binaryreader) | struct |  |
| [`BinaryReaderError`](#binaryreadererror) | struct |  |
| [`WasmFeatures`](#wasmfeatures) | struct | Enabled WebAssembly proposals and features. |
| [`ParserCounts`](#parsercounts) | struct |  |
| [`Parser`](#parser) | struct | An incremental parser of a binary WebAssembly module or component. |
| [`SectionLimited`](#sectionlimited) | struct | A generic structure for reading a section of a WebAssembly binary which has a limited number of items within it. |
| [`SectionLimitedIntoIter`](#sectionlimitedintoiter) | struct | A consuming iterator of a [`SectionLimited`]. |
| [`SectionLimitedIntoIterWithOffsets`](#sectionlimitedintoiterwithoffsets) | struct | An iterator over a limited section iterator. |
| [`Subsections`](#subsections) | struct | Iterator/reader over the contents of a section which is composed of subsections. |
| [`Encoding`](#encoding) | enum | The supported encoding formats for the parser. |
| [`Order`](#order) | enum |  |
| [`State`](#state) | enum |  |
| [`Chunk`](#chunk) | enum | A successful return payload from [`Parser::parse`]. |
| [`Payload`](#payload) | enum | Values that can be parsed from a WebAssembly module or component. |
| [`ModuleArity`](#modulearity) | trait | To compute the arity (param and result counts) of "variable-arity" operators, the operator_arity macro needs information about the module's types and the current control stack. |
| [`FromReader`](#fromreader) | trait | A trait implemented for items that can be decoded directly from a `BinaryReader`, or that which can be parsed from the WebAssembly binary format. |
| [`Subsection`](#subsection) | trait | A trait implemented for subsections of another outer section. |
| [`visit_block`](#visit-block) | fn |  |
| [`visit_loop`](#visit-loop) | fn |  |
| [`visit_if`](#visit-if) | fn |  |
| [`visit_else`](#visit-else) | fn |  |
| [`visit_end`](#visit-end) | fn |  |
| [`visit_br`](#visit-br) | fn |  |
| [`visit_br_if`](#visit-br-if) | fn |  |
| [`visit_br_table`](#visit-br-table) | fn |  |
| [`visit_return`](#visit-return) | fn |  |
| [`visit_call`](#visit-call) | fn |  |
| [`visit_call_indirect`](#visit-call-indirect) | fn |  |
| [`visit_struct_new`](#visit-struct-new) | fn |  |
| [`visit_struct_new_desc`](#visit-struct-new-desc) | fn |  |
| [`visit_array_new_fixed`](#visit-array-new-fixed) | fn |  |
| [`visit_br_on_cast`](#visit-br-on-cast) | fn |  |
| [`visit_br_on_cast_fail`](#visit-br-on-cast-fail) | fn |  |
| [`visit_typed_select_multi`](#visit-typed-select-multi) | fn |  |
| [`visit_return_call`](#visit-return-call) | fn |  |
| [`visit_return_call_indirect`](#visit-return-call-indirect) | fn |  |
| [`visit_try_table`](#visit-try-table) | fn |  |
| [`visit_throw`](#visit-throw) | fn |  |
| [`visit_try`](#visit-try) | fn |  |
| [`visit_catch`](#visit-catch) | fn |  |
| [`visit_delegate`](#visit-delegate) | fn |  |
| [`visit_catch_all`](#visit-catch-all) | fn |  |
| [`visit_call_ref`](#visit-call-ref) | fn |  |
| [`visit_return_call_ref`](#visit-return-call-ref) | fn |  |
| [`visit_br_on_null`](#visit-br-on-null) | fn |  |
| [`visit_br_on_non_null`](#visit-br-on-non-null) | fn |  |
| [`visit_cont_bind`](#visit-cont-bind) | fn |  |
| [`visit_suspend`](#visit-suspend) | fn |  |
| [`visit_resume`](#visit-resume) | fn |  |
| [`visit_resume_throw`](#visit-resume-throw) | fn |  |
| [`visit_switch`](#visit-switch) | fn |  |
| [`visit_br_on_cast_desc`](#visit-br-on-cast-desc) | fn |  |
| [`visit_br_on_cast_desc_fail`](#visit-br-on-cast-desc-fail) | fn |  |
| [`usize_to_u64`](#usize-to-u64) | fn |  |
| [`section`](#section) | fn | Parses an entire section resident in memory into a `Payload`. |
| [`single_item`](#single-item) | fn | Reads a section that is represented by a single uleb-encoded `u32`. |
| [`delimited`](#delimited) | fn | Attempts to parse using `f`. |
| [`clear_hint`](#clear-hint) | fn |  |
| [`Result`](#result) | type |  |
| [`WASM_MODULE_VERSION`](#wasm-module-version) | const |  |
| [`WASM_COMPONENT_VERSION`](#wasm-component-version) | const |  |
| [`KIND_MODULE`](#kind-module) | const |  |
| [`KIND_COMPONENT`](#kind-component) | const |  |
| [`CUSTOM_SECTION`](#custom-section) | const |  |
| [`TYPE_SECTION`](#type-section) | const |  |
| [`IMPORT_SECTION`](#import-section) | const |  |
| [`FUNCTION_SECTION`](#function-section) | const |  |
| [`TABLE_SECTION`](#table-section) | const |  |
| [`MEMORY_SECTION`](#memory-section) | const |  |
| [`GLOBAL_SECTION`](#global-section) | const |  |
| [`EXPORT_SECTION`](#export-section) | const |  |
| [`START_SECTION`](#start-section) | const |  |
| [`ELEMENT_SECTION`](#element-section) | const |  |
| [`CODE_SECTION`](#code-section) | const |  |
| [`DATA_SECTION`](#data-section) | const |  |
| [`DATA_COUNT_SECTION`](#data-count-section) | const |  |
| [`TAG_SECTION`](#tag-section) | const |  |
| [`_for_each_operator_group!`](#for-each-operator-group) | macro | A helper macro which is used to itself define further macros below. |
| [`define_for_each_non_simd_operator!`](#define-for-each-non-simd-operator) | macro | Helper macro to define a `_for_each_non_simd_operator` which receives the syntax of each instruction individually, but only the non-simd operators. |
| [`format_err!`](#format-err) | macro |  |
| [`bail!`](#bail) | macro |  |
| [`define_wasm_features!`](#define-wasm-features) | macro |  |
| [`foreach_wasm_feature!`](#foreach-wasm-feature) | macro |  |

## Modules

- [`prelude`](prelude/index.md) — A small "prelude" to use throughout this crate.
- [`arity`](arity/index.md)
- [`binary_reader`](binary_reader/index.md)
- [`features`](features/index.md)
- [`limits`](limits/index.md)
- [`parser`](parser/index.md)
- [`readers`](readers/index.md)
- [`core`](core/index.md)

## Structs

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

- <span id="binaryreader-new"></span>`fn new(data: &[u8], original_offset: usize) -> BinaryReader<'_>` — [`BinaryReader`](binary_reader/index.md#binaryreader)

  Creates a new binary reader which will parse the `data` provided.

  

  The `original_offset` provided is used for byte offsets in errors that

  are generated. That offset is added to the current position in `data`.

  This can be helpful when `data` is just a window of a view into a larger

  wasm binary perhaps not even entirely stored locally.

  

  The returned binary reader will have all features known to this crate

  enabled. To reject binaries that aren't valid unless a certain feature

  is enabled use the `BinaryReader::new_features` constructor instead.

- <span id="binaryreader-shrink"></span>`fn shrink(&self) -> BinaryReader<'a>` — [`BinaryReader`](binary_reader/index.md#binaryreader)

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

- <span id="binaryreader-ensure-has-byte"></span>`fn ensure_has_byte(&self) -> Result<()>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-ensure-has-bytes"></span>`fn ensure_has_bytes(&self, len: usize) -> Result<()>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-read"></span>`fn read<T>(&mut self) -> Result<T>` — [`Result`](binary_reader/index.md#result)

  Reads a value of type `T` from this binary reader, advancing the

  internal position in this reader forward as data is read.

- <span id="binaryreader-read-u7"></span>`fn read_u7(&mut self) -> Result<u8>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-external-kind-from-byte"></span>`fn external_kind_from_byte(byte: u8, offset: usize) -> Result<ExternalKind>` — [`Result`](binary_reader/index.md#result), [`ExternalKind`](readers/core/index.md#externalkind)

- <span id="binaryreader-read-size"></span>`fn read_size(&mut self, limit: usize, desc: &str) -> Result<usize>` — [`Result`](binary_reader/index.md#result)

  Reads a variable-length 32-bit size from the byte stream while checking

  against a limit.

- <span id="binaryreader-read-iter"></span>`fn read_iter<'me, T>(self: &'me mut Self, limit: usize, desc: &str) -> Result<BinaryReaderIter<'a, 'me, T>>` — [`Result`](binary_reader/index.md#result), [`BinaryReaderIter`](binary_reader/index.md#binaryreaderiter)

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

- <span id="binaryreader-read-bytes"></span>`fn read_bytes(&mut self, size: usize) -> Result<&'a [u8]>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` `size` bytes, and returns a slice from the

  current position of `size` length.

  

  # Errors

  If `size` exceeds the remaining length in `BinaryReader`.

- <span id="binaryreader-read-reader"></span>`fn read_reader(&mut self) -> Result<BinaryReader<'a>>` — [`Result`](binary_reader/index.md#result), [`BinaryReader`](binary_reader/index.md#binaryreader)

  Reads a length-prefixed list of bytes from this reader and returns a

  new `BinaryReader` to read that list of bytes.

- <span id="binaryreader-read-u32"></span>`fn read_u32(&mut self) -> Result<u32>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` four bytes and returns a `u32`.

  # Errors

  If `BinaryReader` has less than four bytes remaining.

- <span id="binaryreader-read-u64"></span>`fn read_u64(&mut self) -> Result<u64>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` eight bytes and returns a `u64`.

  # Errors

  If `BinaryReader` has less than eight bytes remaining.

- <span id="binaryreader-read-u8"></span>`fn read_u8(&mut self) -> Result<u8>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` a single byte.

  

  # Errors

  

  If `BinaryReader` has no bytes remaining.

- <span id="binaryreader-eof-err"></span>`fn eof_err(&self) -> BinaryReaderError` — [`BinaryReaderError`](binary_reader/index.md#binaryreadererror)

- <span id="binaryreader-read-var-u32"></span>`fn read_var_u32(&mut self) -> Result<u32>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a `u32`.

  

  # Errors

  

  If `BinaryReader` has less than one or up to four bytes remaining, or

  the integer is larger than 32 bits.

- <span id="binaryreader-read-var-u32-big"></span>`fn read_var_u32_big(&mut self, byte: u8) -> Result<u32>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-read-var-u64"></span>`fn read_var_u64(&mut self) -> Result<u64>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a `u64`.

  

  # Errors

  

  If `BinaryReader` has less than one or up to eight bytes remaining, or

  the integer is larger than 64 bits.

- <span id="binaryreader-read-var-u64-big"></span>`fn read_var_u64_big(&mut self, byte: u64) -> Result<u64>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-skip"></span>`fn skip(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<Self>` — [`Result`](binary_reader/index.md#result)

  Executes `f` to skip some data in this binary reader and then returns a

  reader which will read the skipped data.

- <span id="binaryreader-skip-string"></span>`fn skip_string(&mut self) -> Result<()>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` past a WebAssembly string. This method does

  not perform any utf-8 validation.

  # Errors

  If `BinaryReader` has less than four bytes, the string's length exceeds

  the remaining bytes, or the string length

  exceeds `limits::MAX_WASM_STRING_SIZE`.

- <span id="binaryreader-read-var-i32"></span>`fn read_var_i32(&mut self) -> Result<i32>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a `i32`.

  # Errors

  If `BinaryReader` has less than one or up to four bytes remaining, or

  the integer is larger than 32 bits.

- <span id="binaryreader-read-var-i32-big"></span>`fn read_var_i32_big(&mut self, byte: u8) -> Result<i32>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-read-var-s33"></span>`fn read_var_s33(&mut self) -> Result<i64>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` up to four bytes to parse a variable

  length integer as a signed 33 bit integer, returned as a `i64`.

  # Errors

  If `BinaryReader` has less than one or up to five bytes remaining, or

  the integer is larger than 33 bits.

- <span id="binaryreader-read-var-i64"></span>`fn read_var_i64(&mut self) -> Result<i64>` — [`Result`](binary_reader/index.md#result)

  Advances the `BinaryReader` up to eight bytes to parse a variable

  length integer as a 64 bit integer, returned as a `i64`.

  # Errors

  If `BinaryReader` has less than one or up to eight bytes remaining, or

  the integer is larger than 64 bits.

- <span id="binaryreader-read-f32"></span>`fn read_f32(&mut self) -> Result<Ieee32>` — [`Result`](binary_reader/index.md#result), [`Ieee32`](readers/core/index.md#ieee32)

  Advances the `BinaryReader` four bytes to parse a 32 bit floating point

  number, returned as `Ieee32`.

  # Errors

  If `BinaryReader` has less than four bytes remaining.

- <span id="binaryreader-read-f64"></span>`fn read_f64(&mut self) -> Result<Ieee64>` — [`Result`](binary_reader/index.md#result), [`Ieee64`](readers/core/index.md#ieee64)

  Advances the `BinaryReader` eight bytes to parse a 64 bit floating point

  number, returned as `Ieee64`.

  # Errors

  If `BinaryReader` has less than eight bytes remaining.

- <span id="binaryreader-internal-read-string"></span>`fn internal_read_string(&mut self, len: usize) -> Result<&'a str>` — [`Result`](binary_reader/index.md#result)

  (internal) Reads a fixed-size WebAssembly string from the module.

- <span id="binaryreader-read-string"></span>`fn read_string(&mut self) -> Result<&'a str>` — [`Result`](binary_reader/index.md#result)

  Reads a WebAssembly string from the module.

  

  # Errors

  

  If `BinaryReader` has less than up to four bytes remaining, the string's

  length exceeds the remaining bytes, the string's length exceeds

  `limits::MAX_WASM_STRING_SIZE`, or the string contains invalid utf-8.

- <span id="binaryreader-read-unlimited-string"></span>`fn read_unlimited_string(&mut self) -> Result<&'a str>` — [`Result`](binary_reader/index.md#result)

  Reads a unlimited WebAssembly string from the module.

  

  Note that this is similar to `BinaryReader::read_string` except that

  it will not limit the size of the returned string by

  `limits::MAX_WASM_STRING_SIZE`.

- <span id="binaryreader-invalid-leading-byte"></span>`fn invalid_leading_byte<T>(&self, byte: u8, desc: &str) -> Result<T>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-invalid-leading-byte-error"></span>`fn invalid_leading_byte_error(byte: u8, desc: &str, offset: usize) -> BinaryReaderError` — [`BinaryReaderError`](binary_reader/index.md#binaryreadererror)

- <span id="binaryreader-peek"></span>`fn peek(&self) -> Result<u8>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-peek-bytes"></span>`fn peek_bytes(&self, len: usize) -> Result<&[u8]>` — [`Result`](binary_reader/index.md#result)

- <span id="binaryreader-read-block-type"></span>`fn read_block_type(&mut self) -> Result<BlockType>` — [`Result`](binary_reader/index.md#result), [`BlockType`](readers/core/index.md#blocktype)

- <span id="binaryreader-is-end-then-eof"></span>`fn is_end_then_eof(&self) -> bool`

  Returns whether there is an `end` opcode followed by eof remaining in

  this reader.

- <span id="binaryreader-read-header-version"></span>`fn read_header_version(&mut self) -> Result<u32>` — [`Result`](binary_reader/index.md#result)

#### Trait Implementations

##### `impl Clone for BinaryReader<'a>`

- <span id="binaryreader-clone"></span>`fn clone(&self) -> BinaryReader<'a>` — [`BinaryReader`](binary_reader/index.md#binaryreader)

##### `impl Debug for BinaryReader<'a>`

- <span id="binaryreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Hash for BinaryReader<'a>`

- <span id="binaryreader-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

### `BinaryReaderError`

```rust
struct BinaryReaderError {
    inner: Box<BinaryReaderErrorInner>,
}
```

A binary reader for WebAssembly modules.

#### Implementations

- <span id="binaryreadererror-new"></span>`fn _new(kind: BinaryReaderErrorKind, message: String, offset: usize) -> Self` — [`BinaryReaderErrorKind`](binary_reader/index.md#binaryreadererrorkind), [`String`](prelude/index.md#string)

- <span id="binaryreadererror-new"></span>`fn new(message: impl Into<String>, offset: usize) -> Self` — [`String`](prelude/index.md#string)

- <span id="binaryreadererror-invalid"></span>`fn invalid(msg: &'static str, offset: usize) -> Self`

- <span id="binaryreadererror-fmt"></span>`fn fmt(args: fmt::Arguments<'_>, offset: usize) -> Self`

- <span id="binaryreadererror-eof"></span>`fn eof(offset: usize, needed_hint: usize) -> Self`

- <span id="binaryreadererror-kind"></span>`fn kind(&mut self) -> BinaryReaderErrorKind` — [`BinaryReaderErrorKind`](binary_reader/index.md#binaryreadererrorkind)

- <span id="binaryreadererror-message"></span>`fn message(&self) -> &str`

  Get this error's message.

- <span id="binaryreadererror-offset"></span>`fn offset(&self) -> usize`

  Get the offset within the Wasm binary where the error occurred.

- <span id="binaryreadererror-set-message"></span>`fn set_message(&mut self, message: &str)`

#### Trait Implementations

##### `impl Clone for BinaryReaderError`

- <span id="binaryreadererror-clone"></span>`fn clone(&self) -> BinaryReaderError` — [`BinaryReaderError`](binary_reader/index.md#binaryreadererror)

##### `impl Debug for BinaryReaderError`

- <span id="binaryreadererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BinaryReaderError`

- <span id="binaryreadererror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BinaryReaderError`

##### `impl ToString for BinaryReaderError`

- <span id="binaryreadererror-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](prelude/index.md#string)

### `WasmFeatures`

```rust
struct WasmFeatures {
    _priv: (),
}
```

Enabled WebAssembly proposals and features.

This is the disabled zero-size version of this structure because the
`features` feature was disabled at compile time of this crate.

#### Implementations

- <span id="wasmfeatures-mutable-global"></span>`fn mutable_global(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-saturating-float-to-int"></span>`fn saturating_float_to_int(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-sign-extension"></span>`fn sign_extension(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-reference-types"></span>`fn reference_types(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-multi-value"></span>`fn multi_value(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-bulk-memory"></span>`fn bulk_memory(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-simd"></span>`fn simd(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-relaxed-simd"></span>`fn relaxed_simd(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-threads"></span>`fn threads(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-shared-everything-threads"></span>`fn shared_everything_threads(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-tail-call"></span>`fn tail_call(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-floats"></span>`fn floats(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-multi-memory"></span>`fn multi_memory(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-exceptions"></span>`fn exceptions(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-memory64"></span>`fn memory64(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-extended-const"></span>`fn extended_const(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-component-model"></span>`fn component_model(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-function-references"></span>`fn function_references(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-memory-control"></span>`fn memory_control(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-gc"></span>`fn gc(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-custom-page-sizes"></span>`fn custom_page_sizes(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-legacy-exceptions"></span>`fn legacy_exceptions(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-gc-types"></span>`fn gc_types(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-stack-switching"></span>`fn stack_switching(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-wide-arithmetic"></span>`fn wide_arithmetic(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-values"></span>`fn cm_values(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-nested-names"></span>`fn cm_nested_names(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-async"></span>`fn cm_async(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-async-stackful"></span>`fn cm_async_stackful(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-async-builtins"></span>`fn cm_async_builtins(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-threading"></span>`fn cm_threading(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-error-context"></span>`fn cm_error_context(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-fixed-size-list"></span>`fn cm_fixed_size_list(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-cm-gc"></span>`fn cm_gc(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-call-indirect-overlong"></span>`fn call_indirect_overlong(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-bulk-memory-opt"></span>`fn bulk_memory_opt(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

- <span id="wasmfeatures-custom-descriptors"></span>`fn custom_descriptors(&self) -> bool`

  Returns whether this feature is enabled in this feature set.

#### Trait Implementations

##### `impl Clone for WasmFeatures`

- <span id="wasmfeatures-clone"></span>`fn clone(&self) -> WasmFeatures` — [`WasmFeatures`](#wasmfeatures)

##### `impl Copy for WasmFeatures`

##### `impl Debug for WasmFeatures`

- <span id="wasmfeatures-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WasmFeatures`

- <span id="wasmfeatures-default"></span>`fn default() -> WasmFeatures` — [`WasmFeatures`](#wasmfeatures)

##### `impl Hash for WasmFeatures`

- <span id="wasmfeatures-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

### `ParserCounts`

```rust
struct ParserCounts {
    function_entries: Option<u32>,
    code_entries: Option<u32>,
    data_entries: Option<u32>,
    data_count: Option<u32>,
}
```

#### Trait Implementations

##### `impl Clone for ParserCounts`

- <span id="parsercounts-clone"></span>`fn clone(&self) -> ParserCounts` — [`ParserCounts`](parser/index.md#parsercounts)

##### `impl Debug for ParserCounts`

- <span id="parsercounts-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ParserCounts`

- <span id="parsercounts-default"></span>`fn default() -> ParserCounts` — [`ParserCounts`](parser/index.md#parsercounts)

### `Parser`

```rust
struct Parser {
    state: State,
    offset: u64,
    max_size: u64,
    encoding: Encoding,
    counts: ParserCounts,
    order: (Order, u64),
}
```

An incremental parser of a binary WebAssembly module or component.

This type is intended to be used to incrementally parse a WebAssembly module
or component as bytes become available for the module. This can also be used
to parse modules or components that are already entirely resident within memory.

This primary function for a parser is the `Parser::parse` function which
will incrementally consume input. You can also use the `Parser::parse_all`
function to parse a module or component that is entirely resident in memory.

#### Implementations

- <span id="parser-new"></span>`fn new(offset: u64) -> Parser` — [`Parser`](#parser)

  Creates a new parser.

  

  Reports errors and ranges relative to `offset` provided, where `offset`

  is some logical offset within the input stream that we're parsing.

- <span id="parser-is-core-wasm"></span>`fn is_core_wasm(bytes: &[u8]) -> bool`

  Tests whether `bytes` looks like a core WebAssembly module.

  

  This will inspect the first 8 bytes of `bytes` and return `true` if it

  starts with the standard core WebAssembly header.

- <span id="parser-is-component"></span>`fn is_component(bytes: &[u8]) -> bool`

  Tests whether `bytes` looks like a WebAssembly component.

  

  This will inspect the first 8 bytes of `bytes` and return `true` if it

  starts with the standard WebAssembly component header.

- <span id="parser-offset"></span>`fn offset(&self) -> u64`

  Returns the original offset that this parser is currently at.

- <span id="parser-parse"></span>`fn parse<'a>(&mut self, data: &'a [u8], eof: bool) -> Result<Chunk<'a>>` — [`Result`](binary_reader/index.md#result), [`Chunk`](#chunk)

  Attempts to parse a chunk of data.

  

  This method will attempt to parse the next incremental portion of a

  WebAssembly binary. Data available for the module or component is

  provided as `data`, and the data can be incomplete if more data has yet

  to arrive. The `eof` flag indicates whether more data will ever be received.

  

  There are two ways parsing can succeed with this method:

  

  * `Chunk::NeedMoreData` - this indicates that there is not enough bytes

    in `data` to parse a payload. The caller needs to wait for more data to

    be available in this situation before calling this method again. It is

    guaranteed that this is only returned if `eof` is `false`.

  

  * `Chunk::Parsed` - this indicates that a chunk of the input was

    successfully parsed. The payload is available in this variant of what

    was parsed, and this also indicates how many bytes of `data` was

    consumed. It's expected that the caller will not provide these bytes

    back to the [`Parser`](#parser) again.

  

  Note that all `Chunk` return values are connected, with a lifetime, to

  the input buffer. Each parsed chunk borrows the input buffer and is a

  view into it for successfully parsed chunks.

  

  It is expected that you'll call this method until `Payload::End` is

  reached, at which point you're guaranteed that the parse has completed.

  Note that complete parsing, for the top-level module or component,

  implies that `data` is empty and `eof` is `true`.

  

  # Errors

  

  Parse errors are returned as an `Err`. Errors can happen when the

  structure of the data is unexpected or if sections are too large for

  example. Note that errors are not returned for malformed *contents* of

  sections here. Sections are generally not individually parsed and each

  returned [`Payload`](#payload) needs to be iterated over further to detect all

  errors.

  

  # Examples

  

  An example of reading a wasm file from a stream (`std::io::Read`) and

  incrementally parsing it.

  

  ```rust

  use std::io::Read;

  use anyhow::Result;

  use wasmparser::{Parser, Chunk, Payload::*};

  

  fn parse(mut reader: impl Read) -> Result<()> {

      let mut buf = Vec::new();

      let mut cur = Parser::new(0);

      let mut eof = false;

      let mut stack = Vec::new();

  

      loop {

          let (payload, consumed) = match cur.parse(&buf, eof)? {

              Chunk::NeedMoreData(hint) => {

                  assert!(!eof); // otherwise an error would be returned

  

                  // Use the hint to preallocate more space, then read

                  // some more data into our buffer.

                  //

                  // Note that the buffer management here is not ideal,

                  // but it's compact enough to fit in an example!

                  let len = buf.len();

                  buf.extend((0..hint).map(|_| 0u8));

                  let n = reader.read(&mut buf[len..])?;

                  buf.truncate(len + n);

                  eof = n == 0;

                  continue;

              }

  

              Chunk::Parsed { consumed, payload } => (payload, consumed),

          };

  

          match payload {

              // Sections for WebAssembly modules

              Version { .. } => { /* ... */ }

              TypeSection(_) => { /* ... */ }

              ImportSection(_) => { /* ... */ }

              FunctionSection(_) => { /* ... */ }

              TableSection(_) => { /* ... */ }

              MemorySection(_) => { /* ... */ }

              TagSection(_) => { /* ... */ }

              GlobalSection(_) => { /* ... */ }

              ExportSection(_) => { /* ... */ }

              StartSection { .. } => { /* ... */ }

              ElementSection(_) => { /* ... */ }

              DataCountSection { .. } => { /* ... */ }

              DataSection(_) => { /* ... */ }

  

              // Here we know how many functions we'll be receiving as

              // `CodeSectionEntry`, so we can prepare for that, and

              // afterwards we can parse and handle each function

              // individually.

              CodeSectionStart { .. } => { /* ... */ }

              CodeSectionEntry(body) => {

                  // here we can iterate over `body` to parse the function

                  // and its locals

              }

  

              // Sections for WebAssembly components

              InstanceSection(_) => { /* ... */ }

              CoreTypeSection(_) => { /* ... */ }

              ComponentInstanceSection(_) => { /* ... */ }

              ComponentAliasSection(_) => { /* ... */ }

              ComponentTypeSection(_) => { /* ... */ }

              ComponentCanonicalSection(_) => { /* ... */ }

              ComponentStartSection { .. } => { /* ... */ }

              ComponentImportSection(_) => { /* ... */ }

              ComponentExportSection(_) => { /* ... */ }

  

              ModuleSection { parser, .. }

              | ComponentSection { parser, .. } => {

                  stack.push(cur.clone());

                  cur = parser.clone();

              }

  

              CustomSection(_) => { /* ... */ }

  

              // Once we've reached the end of a parser we either resume

              // at the parent parser or we break out of the loop because

              // we're done.

              End(_) => {

                  if let Some(parent_parser) = stack.pop() {

                      cur = parent_parser;

                  } else {

                      break;

                  }

              }

  

              // most likely you'd return an error here

              _ => { /* ... */ }

          }

  

          // once we're done processing the payload we can forget the

          // original.

          buf.drain(..consumed);

      }

  

      Ok(())

  }

  

  parse(&b"\0asm\x01\0\0\0"[..]).unwrap();

  ```

- <span id="parser-update-order"></span>`fn update_order(&mut self, order: Order, pos: usize) -> Result<()>` — [`Order`](parser/index.md#order), [`Result`](binary_reader/index.md#result)

- <span id="parser-parse-reader"></span>`fn parse_reader<'a>(&mut self, reader: &mut BinaryReader<'a>, eof: bool) -> Result<Payload<'a>>` — [`BinaryReader`](binary_reader/index.md#binaryreader), [`Result`](binary_reader/index.md#result), [`Payload`](#payload)

- <span id="parser-parse-all"></span>`fn parse_all(self, data: &[u8]) -> impl Iterator<Item = Result<Payload<'_>>>` — [`Result`](binary_reader/index.md#result), [`Payload`](#payload)

  Convenience function that can be used to parse a module or component

  that is entirely resident in memory.

  

  This function will parse the `data` provided as a WebAssembly module

  or component.

  

  Note that when this function yields sections that provide parsers,

  no further action is required for those sections as payloads from

  those parsers will be automatically returned.

  

  # Examples

  

  An example of reading a wasm file from a stream (`std::io::Read`) into

  a buffer and then parsing it.

  

  ```rust

  use std::io::Read;

  use anyhow::Result;

  use wasmparser::{Parser, Chunk, Payload::*};

  

  fn parse(mut reader: impl Read) -> Result<()> {

      let mut buf = Vec::new();

      reader.read_to_end(&mut buf)?;

      let parser = Parser::new(0);

  

      for payload in parser.parse_all(&buf) {

          match payload? {

              // Sections for WebAssembly modules

              Version { .. } => { /* ... */ }

              TypeSection(_) => { /* ... */ }

              ImportSection(_) => { /* ... */ }

              FunctionSection(_) => { /* ... */ }

              TableSection(_) => { /* ... */ }

              MemorySection(_) => { /* ... */ }

              TagSection(_) => { /* ... */ }

              GlobalSection(_) => { /* ... */ }

              ExportSection(_) => { /* ... */ }

              StartSection { .. } => { /* ... */ }

              ElementSection(_) => { /* ... */ }

              DataCountSection { .. } => { /* ... */ }

              DataSection(_) => { /* ... */ }

  

              // Here we know how many functions we'll be receiving as

              // `CodeSectionEntry`, so we can prepare for that, and

              // afterwards we can parse and handle each function

              // individually.

              CodeSectionStart { .. } => { /* ... */ }

              CodeSectionEntry(body) => {

                  // here we can iterate over `body` to parse the function

                  // and its locals

              }

  

              // Sections for WebAssembly components

              ModuleSection { .. } => { /* ... */ }

              InstanceSection(_) => { /* ... */ }

              CoreTypeSection(_) => { /* ... */ }

              ComponentSection { .. } => { /* ... */ }

              ComponentInstanceSection(_) => { /* ... */ }

              ComponentAliasSection(_) => { /* ... */ }

              ComponentTypeSection(_) => { /* ... */ }

              ComponentCanonicalSection(_) => { /* ... */ }

              ComponentStartSection { .. } => { /* ... */ }

              ComponentImportSection(_) => { /* ... */ }

              ComponentExportSection(_) => { /* ... */ }

  

              CustomSection(_) => { /* ... */ }

  

              // Once we've reached the end of a parser we either resume

              // at the parent parser or the payload iterator is at its

              // end and we're done.

              End(_) => {}

  

              // most likely you'd return an error here, but if you want

              // you can also inspect the raw contents of unknown sections

              other => {

                  match other.as_section() {

                      Some((id, range)) => { /* ... */ }

                      None => { /* ... */ }

                  }

              }

          }

      }

  

      Ok(())

  }

  

  parse(&b"\0asm\x01\0\0\0"[..]).unwrap();

  ```

- <span id="parser-skip-section"></span>`fn skip_section(&mut self)`

  Skip parsing the code section entirely.

  

  This function can be used to indicate, after receiving

  `CodeSectionStart`, that the section will not be parsed.

  

  The caller will be responsible for skipping `size` bytes (found in the

  `CodeSectionStart` payload). Bytes should only be fed into `parse`

  after the `size` bytes have been skipped.

  

  # Panics

  

  This function will panic if the parser is not in a state where it's

  parsing the code section.

  

  # Examples

  

  ```rust

  use wasmparser::{Result, Parser, Chunk, Payload::*};

  use core::ops::Range;

  

  fn objdump_headers(mut wasm: &[u8]) -> Result<()> {

      let mut parser = Parser::new(0);

      loop {

          let payload = match parser.parse(wasm, true)? {

              Chunk::Parsed { consumed, payload } => {

                  wasm = &wasm[consumed..];

                  payload

              }

              // this state isn't possible with `eof = true`

              Chunk::NeedMoreData(_) => unreachable!(),

          };

          match payload {

              TypeSection(s) => print_range("type section", &s.range()),

              ImportSection(s) => print_range("import section", &s.range()),

              // .. other sections

  

              // Print the range of the code section we see, but don't

              // actually iterate over each individual function.

              CodeSectionStart { range, size, .. } => {

                  print_range("code section", &range);

                  parser.skip_section();

                  wasm = &wasm[size as usize..];

              }

              End(_) => break,

              _ => {}

          }

      }

      Ok(())

  }

  

  fn print_range(section: &str, range: &Range<usize>) {

      println!("{:>40}: {:#010x} - {:#010x}", section, range.start, range.end);

  }

  ```

- <span id="parser-check-function-code-counts"></span>`fn check_function_code_counts(&self, pos: usize) -> Result<()>` — [`Result`](binary_reader/index.md#result)

- <span id="parser-check-data-count"></span>`fn check_data_count(&self, pos: usize) -> Result<()>` — [`Result`](binary_reader/index.md#result)

#### Trait Implementations

##### `impl Clone for Parser`

- <span id="parser-clone"></span>`fn clone(&self) -> Parser` — [`Parser`](#parser)

##### `impl Debug for Parser`

- <span id="parser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parser`

- <span id="parser-default"></span>`fn default() -> Parser` — [`Parser`](#parser)

### `SectionLimited<'a, T>`

```rust
struct SectionLimited<'a, T> {
    reader: crate::BinaryReader<'a>,
    count: u32,
    _marker: marker::PhantomData<T>,
}
```

A generic structure for reading a section of a WebAssembly binary which has
a limited number of items within it.

Many WebAssembly sections are a count of items followed by that many items.
This helper structure can be used to parse these sections and provides
an iteration-based API for reading the contents.

Note that this always implements the `Clone` trait to represent the
ability to parse the section multiple times.

#### Implementations

- <span id="sectionlimited-into-iter-err-on-gc-types"></span>`fn into_iter_err_on_gc_types(self) -> impl Iterator<Item = Result<FuncType>> + 'a` — [`Result`](binary_reader/index.md#result), [`FuncType`](readers/core/index.md#functype)

  Returns an iterator over this type section which will only yield

  function types and any usage of GC types from the GC proposal will

  be translated into an error.

#### Trait Implementations

##### `impl<T> Clone for SectionLimited<'_, T>`

- <span id="sectionlimited-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for SectionLimited<'_, T>`

- <span id="sectionlimited-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoIterator for SectionLimited<'a, T>`

- <span id="sectionlimited-intoiterator-type-item"></span>`type Item = Result<T, BinaryReaderError>`

- <span id="sectionlimited-intoiterator-type-intoiter"></span>`type IntoIter = SectionLimitedIntoIter<'a, T>`

- <span id="sectionlimited-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `SectionLimitedIntoIter<'a, T>`

```rust
struct SectionLimitedIntoIter<'a, T> {
    section: SectionLimited<'a, T>,
    remaining: u32,
    end: bool,
}
```

A consuming iterator of a [`SectionLimited`](#sectionlimited).

This is created via the `IntoIterator` `impl` for the [`SectionLimited`](#sectionlimited)
type.

#### Implementations

- <span id="sectionlimitedintoiter-original-position"></span>`fn original_position(&self) -> usize`

  Returns the current byte offset of the section within this iterator.

#### Trait Implementations

##### `impl<T> ExactSizeIterator for SectionLimitedIntoIter<'a, T>`

##### `impl IntoIterator for SectionLimitedIntoIter<'a, T>`

- <span id="sectionlimitedintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionlimitedintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionlimitedintoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for SectionLimitedIntoIter<'a, T>`

- <span id="sectionlimitedintoiter-iterator-type-item"></span>`type Item = Result<T, BinaryReaderError>`

- <span id="sectionlimitedintoiter-iterator-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](binary_reader/index.md#result)

- <span id="sectionlimitedintoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `SectionLimitedIntoIterWithOffsets<'a, T>`

```rust
struct SectionLimitedIntoIterWithOffsets<'a, T> {
    iter: SectionLimitedIntoIter<'a, T>,
}
```

An iterator over a limited section iterator.

#### Trait Implementations

##### `impl<T> ExactSizeIterator for SectionLimitedIntoIterWithOffsets<'a, T>`

##### `impl IntoIterator for SectionLimitedIntoIterWithOffsets<'a, T>`

- <span id="sectionlimitedintoiterwithoffsets-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionlimitedintoiterwithoffsets-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionlimitedintoiterwithoffsets-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for SectionLimitedIntoIterWithOffsets<'a, T>`

- <span id="sectionlimitedintoiterwithoffsets-iterator-type-item"></span>`type Item = Result<(usize, T), BinaryReaderError>`

- <span id="sectionlimitedintoiterwithoffsets-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="sectionlimitedintoiterwithoffsets-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Subsections<'a, T>`

```rust
struct Subsections<'a, T> {
    reader: crate::BinaryReader<'a>,
    _marker: marker::PhantomData<T>,
}
```

Iterator/reader over the contents of a section which is composed of
subsections.

This reader is used for the core `name` section, for example. This type
primarily implements [`Iterator`](../fallible_iterator/index.md) for advancing through the sections.

#### Implementations

- <span id="subsections-new"></span>`fn new(reader: BinaryReader<'a>) -> Self` — [`BinaryReader`](binary_reader/index.md#binaryreader)

  Creates a new reader for the specified section contents starting at

  `offset` within the original wasm file.

- <span id="subsections-original-position"></span>`fn original_position(&self) -> usize`

  Returns whether the original byte offset of this section.

- <span id="subsections-range"></span>`fn range(&self) -> Range<usize>`

  Returns the range, as byte offsets, of this section within the original

  wasm binary.

- <span id="subsections-read"></span>`fn read(&mut self) -> Result<T>` — [`Result`](binary_reader/index.md#result)

#### Trait Implementations

##### `impl<T> Clone for Subsections<'_, T>`

- <span id="subsections-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for Subsections<'_, T>`

- <span id="subsections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for Subsections<'a, T>`

- <span id="subsections-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="subsections-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="subsections-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Subsections<'a, T>`

- <span id="subsections-iterator-type-item"></span>`type Item = Result<T, BinaryReaderError>`

- <span id="subsections-iterator-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](binary_reader/index.md#result)

## Enums

### `Encoding`

```rust
enum Encoding {
    Module,
    Component,
}
```

The supported encoding formats for the parser.

#### Variants

- **`Module`**

  The encoding format is a WebAssembly module.

- **`Component`**

  The encoding format is a WebAssembly component.

#### Trait Implementations

##### `impl Clone for Encoding`

- <span id="encoding-clone"></span>`fn clone(&self) -> Encoding` — [`Encoding`](#encoding)

##### `impl Copy for Encoding`

##### `impl Debug for Encoding`

- <span id="encoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Encoding`

##### `impl PartialEq for Encoding`

- <span id="encoding-partialeq-eq"></span>`fn eq(&self, other: &Encoding) -> bool` — [`Encoding`](#encoding)

##### `impl StructuralPartialEq for Encoding`

### `Order`

```rust
enum Order {
    Initial,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Tag,
    Global,
    Export,
    Start,
    Element,
    DataCount,
    Code,
    Data,
}
```

#### Trait Implementations

##### `impl Clone for Order`

- <span id="order-clone"></span>`fn clone(&self) -> Order` — [`Order`](parser/index.md#order)

##### `impl Copy for Order`

##### `impl Debug for Order`

- <span id="order-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Order`

- <span id="order-default"></span>`fn default() -> Order` — [`Order`](parser/index.md#order)

##### `impl Eq for Order`

##### `impl Ord for Order`

- <span id="order-ord-cmp"></span>`fn cmp(&self, other: &Order) -> cmp::Ordering` — [`Order`](parser/index.md#order)

##### `impl PartialEq for Order`

- <span id="order-partialeq-eq"></span>`fn eq(&self, other: &Order) -> bool` — [`Order`](parser/index.md#order)

##### `impl PartialOrd for Order`

- <span id="order-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Order) -> option::Option<cmp::Ordering>` — [`Order`](parser/index.md#order)

##### `impl StructuralPartialEq for Order`

### `State`

```rust
enum State {
    Header,
    SectionStart,
    FunctionBody {
        remaining: u32,
        len: u32,
    },
}
```

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](parser/index.md#state)

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Chunk<'a>`

```rust
enum Chunk<'a> {
    NeedMoreData(u64),
    Parsed {
        consumed: usize,
        payload: Payload<'a>,
    },
}
```

A successful return payload from `Parser::parse`.

On success one of two possible values can be returned, either that more data
is needed to continue parsing or a chunk of the input was parsed, indicating
how much of it was parsed.

#### Variants

- **`NeedMoreData`**

  This can be returned at any time and indicates that more data is needed
  to proceed with parsing. Zero bytes were consumed from the input to
  `Parser::parse`. The `u64` value here is a hint as to how many more
  bytes are needed to continue parsing.

- **`Parsed`**

  A chunk was successfully parsed.

#### Trait Implementations

##### `impl Debug for Chunk<'a>`

- <span id="chunk-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Payload<'a>`

```rust
enum Payload<'a> {
    Version {
        num: u16,
        encoding: Encoding,
        range: core::ops::Range<usize>,
    },
    TypeSection(crate::TypeSectionReader<'a>),
    ImportSection(crate::ImportSectionReader<'a>),
    FunctionSection(crate::FunctionSectionReader<'a>),
    TableSection(crate::TableSectionReader<'a>),
    MemorySection(crate::MemorySectionReader<'a>),
    TagSection(crate::TagSectionReader<'a>),
    GlobalSection(crate::GlobalSectionReader<'a>),
    ExportSection(crate::ExportSectionReader<'a>),
    StartSection {
        func: u32,
        range: core::ops::Range<usize>,
    },
    ElementSection(crate::ElementSectionReader<'a>),
    DataCountSection {
        count: u32,
        range: core::ops::Range<usize>,
    },
    DataSection(crate::DataSectionReader<'a>),
    CodeSectionStart {
        count: u32,
        range: core::ops::Range<usize>,
        size: u32,
    },
    CodeSectionEntry(crate::FunctionBody<'a>),
    CustomSection(crate::CustomSectionReader<'a>),
    UnknownSection {
        id: u8,
        contents: &'a [u8],
        range: core::ops::Range<usize>,
    },
    End(usize),
}
```

Values that can be parsed from a WebAssembly module or component.

This enumeration is all possible chunks of pieces that can be parsed by a
[`Parser`](#parser) from a binary WebAssembly module or component. Note that for many
sections the entire section is parsed all at once, whereas other functions,
like the code section, are parsed incrementally. This is a distinction where some
sections, like the type section, are required to be fully resident in memory
(fully downloaded) before proceeding. Other sections, like the code section,
can be processed in a streaming fashion where each function is extracted
individually so it can possibly be shipped to another thread while you wait
for more functions to get downloaded.

Note that payloads, when returned, do not indicate that the module or component
is valid. For example when you receive a `Payload::TypeSection` the type
section itself has not yet actually been parsed. The reader returned will be
able to parse it, but you'll have to actually iterate the reader to do the
full parse. Each payload returned is intended to be a *window* into the
original `data` passed to `Parser::parse` which can be further processed
if necessary.

#### Variants

- **`Version`**

  Indicates the header of a WebAssembly module or component.

- **`TypeSection`**

  A module type section was received and the provided reader can be
  used to parse the contents of the type section.

- **`ImportSection`**

  A module import section was received and the provided reader can be
  used to parse the contents of the import section.

- **`FunctionSection`**

  A module function section was received and the provided reader can be
  used to parse the contents of the function section.

- **`TableSection`**

  A module table section was received and the provided reader can be
  used to parse the contents of the table section.

- **`MemorySection`**

  A module memory section was received and the provided reader can be
  used to parse the contents of the memory section.

- **`TagSection`**

  A module tag section was received, and the provided reader can be
  used to parse the contents of the tag section.

- **`GlobalSection`**

  A module global section was received and the provided reader can be
  used to parse the contents of the global section.

- **`ExportSection`**

  A module export section was received, and the provided reader can be
  used to parse the contents of the export section.

- **`StartSection`**

  A module start section was received.

- **`ElementSection`**

  A module element section was received and the provided reader can be
  used to parse the contents of the element section.

- **`DataCountSection`**

  A module data count section was received.

- **`DataSection`**

  A module data section was received and the provided reader can be
  used to parse the contents of the data section.

- **`CodeSectionStart`**

  Indicator of the start of the code section of a WebAssembly module.
  
  This entry is returned whenever the code section starts. The `count`
  field indicates how many entries are in this code section. After
  receiving this start marker you're guaranteed that the next `count`
  items will be either `CodeSectionEntry` or an error will be returned.
  
  This, unlike other sections, is intended to be used for streaming the
  contents of the code section. The code section is not required to be
  fully resident in memory when we parse it. Instead a [`Parser`](#parser) is
  capable of parsing piece-by-piece of a code section.

- **`CodeSectionEntry`**

  An entry of the code section, a function, was parsed from a WebAssembly
  module.
  
  This entry indicates that a function was successfully received from the
  code section, and the payload here is the window into the original input
  where the function resides. Note that the function itself has not been
  parsed, it's only been outlined. You'll need to process the
  `FunctionBody` provided to test whether it parses and/or is valid.

- **`CustomSection`**

  A module or component custom section was received.

- **`UnknownSection`**

  An unknown section was found.
  
  This variant is returned for all unknown sections encountered. This
  likely wants to be interpreted as an error by consumers of the parser,
  but this can also be used to parse sections currently unsupported by
  the parser.

- **`End`**

  The end of the WebAssembly module or component was reached.
  
  The value is the offset in the input byte stream where the end
  was reached.

#### Implementations

- <span id="payload-as-section"></span>`fn as_section(&self) -> Option<(u8, Range<usize>)>`

  If this `Payload` represents a section in the original wasm module then

  the section's id and range within the original wasm binary are returned.

  

  Not all payloads refer to entire sections, such as the `Version` and

  `CodeSectionEntry` variants. These variants will return `None` from this

  function.

  

  Otherwise this function will return `Some` where the first element is

  the byte identifier for the section and the second element is the range

  of the contents of the section within the original wasm binary.

  

  The purpose of this method is to enable tools to easily iterate over

  entire sections if necessary and handle sections uniformly, for example

  dropping custom sections while preserving all other sections.

#### Trait Implementations

##### `impl Debug for Payload<'_>`

- <span id="payload-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `ModuleArity`

```rust
trait ModuleArity { ... }
```

To compute the arity (param and result counts) of "variable-arity"
operators, the operator_arity macro needs information about the
module's types and the current control stack. The ModuleArity
trait exposes this information.

#### Required Methods

- `fn sub_type_at(&self, type_idx: u32) -> Option<&SubType>`

  Type with given index

- `fn tag_type_arity(&self, at: u32) -> Option<(u32, u32)>`

  Arity (param and result counts) of tag with given index

- `fn type_index_of_function(&self, function_idx: u32) -> Option<u32>`

  Type index of function with given index

- `fn func_type_of_cont_type(&self, c: &ContType) -> Option<&FuncType>`

  Function type for a given continuation type

- `fn sub_type_of_ref_type(&self, rt: &RefType) -> Option<&SubType>`

  Sub type for a given reference type

- `fn control_stack_height(&self) -> u32`

  Current height of control stack

- `fn label_block(&self, depth: u32) -> Option<(BlockType, FrameKind)>`

  BlockType and FrameKind of label with given index

#### Provided Methods

- `fn sub_type_arity(&self, t: &SubType) -> Option<(u32, u32)>`

  Computes arity of given SubType

- `fn block_type_arity(&self, ty: BlockType) -> Option<(u32, u32)>`

  Computes arity of given BlockType

### `FromReader<'a>`

```rust
trait FromReader<'a>: Sized { ... }
```

A trait implemented for items that can be decoded directly from a
`BinaryReader`, or that which can be parsed from the WebAssembly binary
format.

Note that this is also accessible as a `BinaryReader::read` method.

#### Required Methods

- `fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>`

  Attempts to read `Self` from the provided binary reader, returning an

#### Implementors

- [`AbstractHeapType`](readers/core/index.md#abstractheaptype)
- [`ArrayType`](readers/core/index.md#arraytype)
- [`BranchHintFunction`](readers/core/index.md#branchhintfunction)
- [`BranchHint`](readers/core/index.md#branchhint)
- [`Catch`](readers/core/index.md#catch)
- [`ComdatSymbolKind`](readers/core/index.md#comdatsymbolkind)
- [`ComdatSymbol`](readers/core/index.md#comdatsymbol)
- [`Comdat`](readers/core/index.md#comdat)
- [`CompositeType`](readers/core/index.md#compositetype)
- [`ConstExpr`](readers/core/index.md#constexpr)
- [`ContType`](readers/core/index.md#conttype)
- [`CoreDumpInstance`](readers/core/index.md#coredumpinstance)
- [`CoreDumpStackFrame`](readers/core/index.md#coredumpstackframe)
- [`CoreDumpValue`](readers/core/index.md#coredumpvalue)
- [`Data`](readers/core/index.md#data)
- [`DefinedDataSymbol`](readers/core/index.md#defineddatasymbol)
- [`Element`](readers/core/index.md#element)
- [`Export`](readers/core/index.md#export)
- [`ExternalKind`](readers/core/index.md#externalkind)
- [`FieldType`](readers/core/index.md#fieldtype)
- [`FuncType`](readers/core/index.md#functype)
- [`FunctionBody`](readers/core/index.md#functionbody)
- [`GlobalType`](readers/core/index.md#globaltype)
- [`Global`](readers/core/index.md#global)
- [`Handle`](readers/core/index.md#handle)
- [`HeapType`](readers/core/index.md#heaptype)
- [`Import`](readers/core/index.md#import)
- [`IndirectNaming`](readers/core/index.md#indirectnaming)
- [`InitFunc`](readers/core/index.md#initfunc)
- [`MemoryType`](readers/core/index.md#memorytype)
- [`Naming`](readers/core/index.md#naming)
- [`ProducersFieldValue`](readers/core/index.md#producersfieldvalue)
- [`ProducersField`](readers/core/index.md#producersfield)
- [`RecGroup`](readers/core/index.md#recgroup)
- [`RefType`](readers/core/index.md#reftype)
- [`RelocationEntry`](readers/core/index.md#relocationentry)
- [`RelocationType`](readers/core/index.md#relocationtype)
- [`ResumeTable`](readers/core/index.md#resumetable)
- [`SegmentFlags`](readers/core/index.md#segmentflags)
- [`Segment`](readers/core/index.md#segment)
- [`StorageType`](readers/core/index.md#storagetype)
- [`StructType`](readers/core/index.md#structtype)
- [`SubType`](readers/core/index.md#subtype)
- [`SymbolFlags`](readers/core/index.md#symbolflags)
- [`SymbolInfo`](readers/core/index.md#symbolinfo)
- [`TableType`](readers/core/index.md#tabletype)
- [`Table`](readers/core/index.md#table)
- [`TagType`](readers/core/index.md#tagtype)
- [`TryTable`](readers/core/index.md#trytable)
- [`TypeRef`](readers/core/index.md#typeref)
- [`ValType`](readers/core/index.md#valtype)
- `&'a str`
- `(T, U)`
- `bool`
- `u32`

### `Subsection<'a>`

```rust
trait Subsection<'a>: Sized { ... }
```

A trait implemented for subsections of another outer section.

This is currently only used for subsections within custom sections, such as
the `name` section of core wasm.

This is used in conjunction with [`Subsections`](#subsections).

#### Required Methods

- `fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>`

  Converts the section identifier provided with the section contents into

#### Implementors

- [`Dylink0Subsection`](readers/core/index.md#dylink0subsection)
- [`Linking`](readers/core/index.md#linking)
- [`Name`](readers/core/index.md#name)

## Functions

### `visit_block`

```rust
fn visit_block(module: &dyn ModuleArity, block: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_loop`

```rust
fn visit_loop(module: &dyn ModuleArity, block: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_if`

```rust
fn visit_if(module: &dyn ModuleArity, block: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_else`

```rust
fn visit_else(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_end`

```rust
fn visit_end(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_br`

```rust
fn visit_br(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_br_if`

```rust
fn visit_br_if(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_br_table`

```rust
fn visit_br_table(module: &dyn ModuleArity, table: crate::BrTable<'_>) -> Option<(u32, u32)>
```

### `visit_return`

```rust
fn visit_return(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_call`

```rust
fn visit_call(module: &dyn ModuleArity, func: u32) -> Option<(u32, u32)>
```

### `visit_call_indirect`

```rust
fn visit_call_indirect(module: &dyn ModuleArity, ty: u32, _table: u32) -> Option<(u32, u32)>
```

### `visit_struct_new`

```rust
fn visit_struct_new(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_struct_new_desc`

```rust
fn visit_struct_new_desc(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_array_new_fixed`

```rust
fn visit_array_new_fixed(_module: &dyn ModuleArity, _ty: u32, size: u32) -> Option<(u32, u32)>
```

### `visit_br_on_cast`

```rust
fn visit_br_on_cast(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

### `visit_br_on_cast_fail`

```rust
fn visit_br_on_cast_fail(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

### `visit_typed_select_multi`

```rust
fn visit_typed_select_multi(_module: &dyn ModuleArity, tys: Vec<crate::ValType>) -> Option<(u32, u32)>
```

### `visit_return_call`

```rust
fn visit_return_call(module: &dyn ModuleArity, func: u32) -> Option<(u32, u32)>
```

### `visit_return_call_indirect`

```rust
fn visit_return_call_indirect(module: &dyn ModuleArity, ty: u32, table: u32) -> Option<(u32, u32)>
```

### `visit_try_table`

```rust
fn visit_try_table(module: &dyn ModuleArity, table: crate::TryTable) -> Option<(u32, u32)>
```

### `visit_throw`

```rust
fn visit_throw(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)>
```

### `visit_try`

```rust
fn visit_try(module: &dyn ModuleArity, ty: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_catch`

```rust
fn visit_catch(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)>
```

### `visit_delegate`

```rust
fn visit_delegate(module: &dyn ModuleArity, _depth: u32) -> Option<(u32, u32)>
```

### `visit_catch_all`

```rust
fn visit_catch_all(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_call_ref`

```rust
fn visit_call_ref(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_return_call_ref`

```rust
fn visit_return_call_ref(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_br_on_null`

```rust
fn visit_br_on_null(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_br_on_non_null`

```rust
fn visit_br_on_non_null(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_cont_bind`

```rust
fn visit_cont_bind(module: &dyn ModuleArity, arg: u32, result: u32) -> Option<(u32, u32)>
```

### `visit_suspend`

```rust
fn visit_suspend(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)>
```

### `visit_resume`

```rust
fn visit_resume(module: &dyn ModuleArity, cont: u32, _table: crate::ResumeTable) -> Option<(u32, u32)>
```

### `visit_resume_throw`

```rust
fn visit_resume_throw(module: &dyn ModuleArity, cont: u32, tag: u32, _table: crate::ResumeTable) -> Option<(u32, u32)>
```

### `visit_switch`

```rust
fn visit_switch(module: &dyn ModuleArity, cont: u32, _tag: u32) -> Option<(u32, u32)>
```

### `visit_br_on_cast_desc`

```rust
fn visit_br_on_cast_desc(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

### `visit_br_on_cast_desc_fail`

```rust
fn visit_br_on_cast_desc_fail(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

### `usize_to_u64`

```rust
fn usize_to_u64(a: usize) -> u64
```

### `section`

```rust
fn section<'a, T>(reader: &mut crate::BinaryReader<'a>, len: u32, ctor: fn(crate::BinaryReader<'a>) -> crate::Result<T>, variant: fn(T) -> Payload<'a>) -> crate::Result<Payload<'a>>
```

Parses an entire section resident in memory into a `Payload`.

Requires that `len` bytes are resident in `reader` and uses `ctor`/`variant`
to construct the section to return.

### `single_item`

```rust
fn single_item<'a, T>(reader: &mut crate::BinaryReader<'a>, len: u32, desc: &str) -> crate::Result<(T, core::ops::Range<usize>)>
where
    T: FromReader<'a>
```

Reads a section that is represented by a single uleb-encoded `u32`.

### `delimited`

```rust
fn delimited<'a, T>(reader: &mut crate::BinaryReader<'a>, len: &mut u32, f: impl FnOnce(&mut crate::BinaryReader<'a>) -> crate::Result<T>) -> crate::Result<T>
```

Attempts to parse using `f`.

This will update `*len` with the number of bytes consumed, and it will cause
a failure to be returned instead of the number of bytes consumed exceeds
what `*len` currently is.

### `clear_hint`

```rust
fn clear_hint(err: crate::BinaryReaderError) -> crate::BinaryReaderError
```

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = core::result::Result<T, E>;
```

The result for `BinaryReader` operations.

## Constants

### `WASM_MODULE_VERSION`
```rust
const WASM_MODULE_VERSION: u16 = 1u16;
```

### `WASM_COMPONENT_VERSION`
```rust
const WASM_COMPONENT_VERSION: u16 = 13u16;
```

### `KIND_MODULE`
```rust
const KIND_MODULE: u16 = 0u16;
```

### `KIND_COMPONENT`
```rust
const KIND_COMPONENT: u16 = 1u16;
```

### `CUSTOM_SECTION`
```rust
const CUSTOM_SECTION: u8 = 0u8;
```

### `TYPE_SECTION`
```rust
const TYPE_SECTION: u8 = 1u8;
```

### `IMPORT_SECTION`
```rust
const IMPORT_SECTION: u8 = 2u8;
```

### `FUNCTION_SECTION`
```rust
const FUNCTION_SECTION: u8 = 3u8;
```

### `TABLE_SECTION`
```rust
const TABLE_SECTION: u8 = 4u8;
```

### `MEMORY_SECTION`
```rust
const MEMORY_SECTION: u8 = 5u8;
```

### `GLOBAL_SECTION`
```rust
const GLOBAL_SECTION: u8 = 6u8;
```

### `EXPORT_SECTION`
```rust
const EXPORT_SECTION: u8 = 7u8;
```

### `START_SECTION`
```rust
const START_SECTION: u8 = 8u8;
```

### `ELEMENT_SECTION`
```rust
const ELEMENT_SECTION: u8 = 9u8;
```

### `CODE_SECTION`
```rust
const CODE_SECTION: u8 = 10u8;
```

### `DATA_SECTION`
```rust
const DATA_SECTION: u8 = 11u8;
```

### `DATA_COUNT_SECTION`
```rust
const DATA_COUNT_SECTION: u8 = 12u8;
```

### `TAG_SECTION`
```rust
const TAG_SECTION: u8 = 13u8;
```

## Macros

### `_for_each_operator_group!`

A helper macro which is used to itself define further macros below.

This is a little complicated, so first off sorry about that. The idea here
though is that there's one source of truth for the listing of instructions
in `wasmparser` and this is the one location. All other locations should be
derivative from this. As this one source of truth it has all instructions
from all proposals all grouped together. Down below though, for compile
time, currently the simd instructions are split out into their own macro.
The structure/syntax of this macro is to facilitate easily splitting out
entire groups of instructions.

This is used below to define more macros.

### `define_for_each_non_simd_operator!`

Helper macro to define a `_for_each_non_simd_operator` which receives
the syntax of each instruction individually, but only the non-simd
operators.

### `format_err!`

### `bail!`

### `define_wasm_features!`

### `foreach_wasm_feature!`

