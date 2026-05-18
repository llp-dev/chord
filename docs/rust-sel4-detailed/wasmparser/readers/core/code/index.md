*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [code](index.md)*

---

# Module `code`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FunctionBody`](#functionbody) | struct | Represents a WebAssembly function body. |
| [`LocalsReader`](#localsreader) | struct | A reader for a function body's locals. |
| [`LocalsIterator`](#localsiterator) | struct | An iterator over locals in a function body. |
| [`CodeSectionReader`](#codesectionreader) | type | A reader for the code section of a WebAssembly module. |

## Structs

### `FunctionBody<'a>`

```rust
struct FunctionBody<'a> {
    reader: crate::BinaryReader<'a>,
}
```

Represents a WebAssembly function body.

#### Implementations

- <span id="functionbody-new"></span>`fn new(reader: BinaryReader<'a>) -> Self` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  Constructs a new `FunctionBody` for the given data and offset.

- <span id="functionbody-get-binary-reader"></span>`fn get_binary_reader(&self) -> BinaryReader<'a>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  Gets a binary reader for this function body.

- <span id="functionbody-skip-locals"></span>`fn skip_locals(reader: &mut BinaryReader<'_>) -> Result<()>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

- <span id="functionbody-get-locals-reader"></span>`fn get_locals_reader(&self) -> Result<LocalsReader<'a>>` — [`Result`](../../../binary_reader/index.md#result), [`LocalsReader`](../index.md#localsreader)

  Gets the locals reader for this function body.

- <span id="functionbody-get-binary-reader-for-operators"></span>`fn get_binary_reader_for_operators(&self) -> Result<BinaryReader<'a>>` — [`Result`](../../../binary_reader/index.md#result), [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  Gets a binary reader for this function body, after skipping locals.

- <span id="functionbody-get-operators-reader"></span>`fn get_operators_reader(&self) -> Result<OperatorsReader<'a>>` — [`Result`](../../../binary_reader/index.md#result), [`OperatorsReader`](../index.md#operatorsreader)

  Uses `FunctionBody::get_binary_reader_for_operators` and then converts

  that to an [`OperatorsReader`](../index.md).

- <span id="functionbody-range"></span>`fn range(&self) -> Range<usize>`

  Gets the range of the function body.

- <span id="functionbody-as-bytes"></span>`fn as_bytes(&self) -> &'a [u8]`

  Returns the body of this function as a list of bytes.

  

  Note that the returned bytes start with the function locals declaration.

#### Trait Implementations

##### `impl Clone for FunctionBody<'a>`

- <span id="functionbody-clone"></span>`fn clone(&self) -> FunctionBody<'a>` — [`FunctionBody`](../index.md#functionbody)

##### `impl Debug for FunctionBody<'a>`

- <span id="functionbody-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for FunctionBody<'a>`

- <span id="functionbody-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `LocalsReader<'a>`

```rust
struct LocalsReader<'a> {
    reader: crate::BinaryReader<'a>,
    declaration_count: u32,
    total_count: u32,
}
```

A reader for a function body's locals.

#### Implementations

- <span id="localsreader-get-count"></span>`fn get_count(&self) -> u32`

  Gets the count of locals declarations in the function body.

- <span id="localsreader-original-position"></span>`fn original_position(&self) -> usize`

  Gets the original position of the reader.

- <span id="localsreader-read"></span>`fn read(&mut self) -> Result<(u32, ValType)>` — [`Result`](../../../binary_reader/index.md#result), [`ValType`](../index.md#valtype)

  Reads an item from the reader.

- <span id="localsreader-get-binary-reader"></span>`fn get_binary_reader(self) -> BinaryReader<'a>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  Gets the binary reader from this LocalsReader

#### Trait Implementations

##### `impl IntoIterator for LocalsReader<'a>`

- <span id="localsreader-intoiterator-type-item"></span>`type Item = Result<(u32, ValType), BinaryReaderError>`

- <span id="localsreader-intoiterator-type-intoiter"></span>`type IntoIter = LocalsIterator<'a>`

- <span id="localsreader-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `LocalsIterator<'a>`

```rust
struct LocalsIterator<'a> {
    reader: LocalsReader<'a>,
    left: u32,
    err: bool,
}
```

An iterator over locals in a function body.

#### Implementations

- <span id="localsiterator-into-binary-reader-for-operators"></span>`fn into_binary_reader_for_operators(self) -> BinaryReader<'a>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  After reading the locals, the BinaryReader is ready to read the operators.

- <span id="localsiterator-into-operators-reader"></span>`fn into_operators_reader(self) -> OperatorsReader<'a>` — [`OperatorsReader`](../index.md#operatorsreader)

  After reading the locals, the BinaryReader is ready to read the operators.

#### Trait Implementations

##### `impl IntoIterator for LocalsIterator<'a>`

- <span id="localsiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="localsiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="localsiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LocalsIterator<'a>`

- <span id="localsiterator-iterator-type-item"></span>`type Item = Result<(u32, ValType), BinaryReaderError>`

- <span id="localsiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="localsiterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Type Aliases

### `CodeSectionReader<'a>`

```rust
type CodeSectionReader<'a> = crate::SectionLimited<'a, FunctionBody<'a>>;
```

A reader for the code section of a WebAssembly module.

