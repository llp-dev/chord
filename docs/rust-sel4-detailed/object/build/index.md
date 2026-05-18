*[object](../index.md) / [build](index.md)*

---

# Module `build`

Interface for building object files.

This module provides common types and traits used in the builders.

The submodules define the builders for each file format.

## Contents

- [Modules](#modules)
  - [`error`](#error)
  - [`bytes`](#bytes)
  - [`table`](#table)
  - [`elf`](#elf)
- [Structs](#structs)
  - [`Error`](#error)
  - [`ByteString`](#bytestring)
  - [`Bytes`](#bytes)
  - [`Table`](#table)
- [Traits](#traits)
  - [`Id`](#id)
  - [`Item`](#item)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`error`](#error) | mod |  |
| [`bytes`](#bytes) | mod |  |
| [`table`](#table) | mod |  |
| [`elf`](#elf) | mod | This module provides a [`Builder`] for reading, modifying, and then writing ELF files. |
| [`Error`](#error) | struct |  |
| [`ByteString`](#bytestring) | struct |  |
| [`Bytes`](#bytes) | struct |  |
| [`Table`](#table) | struct |  |
| [`Id`](#id) | trait |  |
| [`Item`](#item) | trait |  |
| [`Result`](#result) | type |  |

## Modules

- [`error`](error/index.md)
- [`bytes`](bytes/index.md)
- [`table`](table/index.md)
- [`elf`](elf/index.md) — This module provides a [`Builder`] for reading, modifying, and then writing ELF files.

## Structs

### `Error`

```rust
struct Error(alloc::string::String);
```

The error type used within the build module.

#### Implementations

- <span id="error-new"></span>`fn new(message: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](error/index.md#error)

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl<K> Equivalent for Error`

- <span id="error-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](error/index.md#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ByteString<'a>`

```rust
struct ByteString<'a>(alloc::borrow::Cow<'a, [u8]>);
```

A byte slice that is a string of an unknown encoding.

Uses copy-on-write to avoid unnecessary allocations. The bytes can be
accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
`to_mut` method.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

#### Implementations

- <span id="bytestring-to-mut"></span>`fn to_mut(&mut self) -> &mut Vec<u8>`

  Acquire a mutable reference to the bytes.

  

  Clones the bytes if they are shared.

- <span id="bytestring-as-slice"></span>`fn as_slice(&self) -> &[u8]`

  Get the bytes as a slice.

#### Trait Implementations

##### `impl Clone for ByteString<'a>`

- <span id="bytestring-clone"></span>`fn clone(&self) -> ByteString<'a>` — [`ByteString`](bytes/index.md#bytestring)

##### `impl Debug for ByteString<'a>`

- <span id="bytestring-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteString<'a>`

- <span id="bytestring-default"></span>`fn default() -> ByteString<'a>` — [`ByteString`](bytes/index.md#bytestring)

##### `impl Deref for ByteString<'a>`

- <span id="bytestring-deref-type-target"></span>`type Target = [u8]`

- <span id="bytestring-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl Display for ByteString<'a>`

- <span id="bytestring-display-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ByteString<'a>`

##### `impl<K> Equivalent for ByteString<'a>`

- <span id="bytestring-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ByteString<'a>`

- <span id="bytestring-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ByteString<'a>`

- <span id="bytestring-partialeq-eq"></span>`fn eq(&self, other: &ByteString<'a>) -> bool` — [`ByteString`](bytes/index.md#bytestring)

##### `impl Receiver for ByteString<'a>`

- <span id="bytestring-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for ByteString<'a>`

##### `impl ToString for ByteString<'a>`

- <span id="bytestring-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Bytes<'a>`

```rust
struct Bytes<'a>(alloc::borrow::Cow<'a, [u8]>);
```

A byte slice.

Uses copy-on-write to avoid unnecessary allocations. The bytes can be
accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
`to_mut` method.

Provides a `Debug` implementation that shows the first 8 bytes and the length.

#### Implementations

- <span id="bytes-to-mut"></span>`fn to_mut(&mut self) -> &mut Vec<u8>`

  Acquire a mutable reference to the bytes.

  

  Clones the bytes if they are shared.

- <span id="bytes-as-slice"></span>`fn as_slice(&self) -> &[u8]`

  Get the bytes as a slice.

#### Trait Implementations

##### `impl Clone for Bytes<'a>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'a>` — [`Bytes`](bytes/index.md#bytes)

##### `impl Debug for Bytes<'a>`

- <span id="bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bytes<'a>`

- <span id="bytes-default"></span>`fn default() -> Bytes<'a>` — [`Bytes`](bytes/index.md#bytes)

##### `impl Deref for Bytes<'a>`

- <span id="bytes-deref-type-target"></span>`type Target = [u8]`

- <span id="bytes-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl Eq for Bytes<'a>`

##### `impl<K> Equivalent for Bytes<'a>`

- <span id="bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Bytes<'a>`

- <span id="bytes-partialeq-eq"></span>`fn eq(&self, other: &Bytes<'a>) -> bool` — [`Bytes`](bytes/index.md#bytes)

##### `impl Receiver for Bytes<'a>`

- <span id="bytes-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Bytes<'a>`

### `Table<T>`

```rust
struct Table<T>(alloc::vec::Vec<T>);
```

A table of items.

Each item has a unique identifier.
Items can be deleted without changing the identifiers of other items.

#### Implementations

- <span id="table-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Table<T>`

- <span id="table-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Item> IntoIterator for &'a Table<T>`

- <span id="a-table-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-table-intoiterator-type-intoiter"></span>`type IntoIter = TableIter<'a, T>`

- <span id="a-table-intoiterator-into-iter"></span>`fn into_iter(self) -> TableIter<'a, T>` — [`TableIter`](table/index.md#tableiter)

## Traits

### `Id`

```rust
trait Id: IdPrivate { ... }
```

An identifier for referring to an item in a [`Table`](table/index.md).

#### Required Methods

- `fn index(&self) -> usize`

  Return the index of the item in the table.

#### Implementors

- [`SectionId`](elf/index.md#sectionid)
- [`SegmentId`](elf/index.md#segmentid)
- [`SymbolId`](elf/index.md#symbolid)
- [`VersionFileId`](elf/index.md#versionfileid)
- [`VersionId`](elf/index.md#versionid)

### `Item`

```rust
trait Item { ... }
```

An item in a [`Table`](table/index.md).

#### Associated Types

- `type Id: 1`

#### Required Methods

- `fn is_deleted(&self) -> bool`

  Return `True` if the item is deleted.

#### Implementors

- [`Section`](elf/index.md#section)
- [`Segment`](elf/index.md#segment)
- [`Symbol`](elf/index.md#symbol)
- [`VersionFile`](elf/index.md#versionfile)
- [`Version`](elf/index.md#version)

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result type used within the build module.

