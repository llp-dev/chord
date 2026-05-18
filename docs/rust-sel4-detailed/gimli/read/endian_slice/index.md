*[gimli](../../index.md) / [read](../index.md) / [endian_slice](index.md)*

---

# Module `endian_slice`

Working with byte slices that have an associated endianity.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EndianSlice`](#endianslice) | struct | A `&[u8]` slice with endianity metadata. |
| [`DebugBytes`](#debugbytes) | struct |  |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |

## Structs

### `EndianSlice<'input, Endian>`

```rust
struct EndianSlice<'input, Endian>
where
    Endian: Endianity {
    slice: &'input [u8],
    endian: Endian,
}
```

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

#### Implementations

- <span id="endianslice-new"></span>`fn new(slice: &'input [u8], endian: Endian) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md#endianslice)

  Construct a new `EndianSlice` with the given slice and endianity.

- <span id="endianslice-slice"></span>`fn slice(&self) -> &'input [u8]`

  Return a reference to the raw slice.

- <span id="endianslice-split-at"></span>`fn split_at(&self, idx: usize) -> (EndianSlice<'input, Endian>, EndianSlice<'input, Endian>)` — [`EndianSlice`](../index.md#endianslice)

  Split the slice in two at the given index, resulting in the tuple where

  the first item has range [0, idx), and the second has range [idx,

  len). Panics if the index is out of bounds.

- <span id="endianslice-find"></span>`fn find(&self, byte: u8) -> Option<usize>`

  Find the first occurrence of a byte in the slice, and return its index.

- <span id="endianslice-offset-from"></span>`fn offset_from(&self, base: EndianSlice<'input, Endian>) -> usize` — [`EndianSlice`](../index.md#endianslice)

  Return the offset of the start of the slice relative to the start

  of the given slice.

- <span id="endianslice-to-string"></span>`fn to_string(&self) -> Result<&'input str>` — [`Result`](../../index.md#result)

  Converts the slice to a string using `str::from_utf8`.

  

  Returns an error if the slice contains invalid characters.

- <span id="endianslice-to-string-lossy"></span>`fn to_string_lossy(&self) -> Cow<'input, str>`

  Converts the slice to a string, including invalid characters,

  using `String::from_utf8_lossy`.

- <span id="endianslice-read-slice"></span>`fn read_slice(&mut self, len: usize) -> Result<&'input [u8]>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl<Endian> Clone for EndianSlice<'input, Endian>`

- <span id="endianslice-clone"></span>`fn clone(&self) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md#endianslice)

##### `impl<Endian> Copy for EndianSlice<'input, Endian>`

##### `impl<Endian: Endianity> Debug for EndianSlice<'input, Endian>`

- <span id="endianslice-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<Endian> Default for EndianSlice<'input, Endian>`

- <span id="endianslice-default"></span>`fn default() -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md#endianslice)

##### `impl<Endian> Deref for EndianSlice<'input, Endian>`

- <span id="endianslice-deref-type-target"></span>`type Target = [u8]`

- <span id="endianslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<Endian> Eq for EndianSlice<'input, Endian>`

##### `impl<Endian> Hash for EndianSlice<'input, Endian>`

- <span id="endianslice-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<Endian> PartialEq for EndianSlice<'input, Endian>`

- <span id="endianslice-partialeq-eq"></span>`fn eq(&self, other: &EndianSlice<'input, Endian>) -> bool` — [`EndianSlice`](../index.md#endianslice)

##### `impl<Endian> Reader for EndianSlice<'input, Endian>`

- <span id="endianslice-reader-type-endian"></span>`type Endian = Endian`

- <span id="endianslice-reader-type-offset"></span>`type Offset = usize`

- <span id="endianslice-reader-endian"></span>`fn endian(&self) -> Endian`

- <span id="endianslice-reader-len"></span>`fn len(&self) -> usize`

- <span id="endianslice-reader-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="endianslice-reader-empty"></span>`fn empty(&mut self)`

- <span id="endianslice-reader-truncate"></span>`fn truncate(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-offset-from"></span>`fn offset_from(&self, base: &Self) -> usize`

- <span id="endianslice-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md#readeroffsetid)

- <span id="endianslice-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](../index.md#readeroffsetid), [`Reader`](../index.md#reader)

- <span id="endianslice-reader-find"></span>`fn find(&self, byte: u8) -> Result<usize>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-skip"></span>`fn skip(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-split"></span>`fn split(&mut self, len: usize) -> Result<Self>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../../index.md#result)

##### `impl Receiver for EndianSlice<'input, Endian>`

- <span id="endianslice-receiver-type-target"></span>`type Target = T`

##### `impl<Endian> StructuralPartialEq for EndianSlice<'input, Endian>`

### `DebugBytes<'input>`

```rust
struct DebugBytes<'input>(&'input [u8]);
```

#### Trait Implementations

##### `impl Debug for DebugBytes<'input>`

- <span id="debugbytes-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

#### Trait Implementations

##### `impl Debug for DebugByte`

- <span id="debugbyte-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

#### Trait Implementations

##### `impl Debug for DebugLen`

- <span id="debuglen-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

