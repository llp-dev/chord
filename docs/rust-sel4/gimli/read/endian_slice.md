**gimli > read > endian_slice**

# Module: read::endian_slice

## Contents

**Structs**

- [`EndianSlice`](#endianslice) - A `&[u8]` slice with endianity metadata.

---

## gimli::read::endian_slice::EndianSlice

*Struct*

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

**Generic Parameters:**
- 'input
- Endian

**Methods:**

- `fn range(self: &Self, idx: Range<usize>) -> EndianSlice<'input, Endian>` - Take the given `start..end` range of the underlying slice and return a
- `fn range_from(self: &Self, idx: RangeFrom<usize>) -> EndianSlice<'input, Endian>` - Take the given `start..` range of the underlying slice and return a new
- `fn range_to(self: &Self, idx: RangeTo<usize>) -> EndianSlice<'input, Endian>` - Take the given `..end` range of the underlying slice and return a new
- `fn new(slice: &'input [u8], endian: Endian) -> EndianSlice<'input, Endian>` - Construct a new `EndianSlice` with the given slice and endianity.
- `fn slice(self: &Self) -> &'input [u8]` - Return a reference to the raw slice.
- `fn split_at(self: &Self, idx: usize) -> (EndianSlice<'input, Endian>, EndianSlice<'input, Endian>)` - Split the slice in two at the given index, resulting in the tuple where
- `fn find(self: &Self, byte: u8) -> Option<usize>` - Find the first occurrence of a byte in the slice, and return its index.
- `fn offset_from(self: &Self, base: EndianSlice<'input, Endian>) -> usize` - Return the offset of the start of the slice relative to the start
- `fn to_string(self: &Self) -> Result<&'input str>` - Converts the slice to a string using `str::from_utf8`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Reader**
  - `fn endian(self: &Self) -> Endian`
  - `fn len(self: &Self) -> usize`
  - `fn is_empty(self: &Self) -> bool`
  - `fn empty(self: & mut Self)`
  - `fn truncate(self: & mut Self, len: usize) -> Result<()>`
  - `fn offset_from(self: &Self, base: &Self) -> usize`
  - `fn offset_id(self: &Self) -> ReaderOffsetId`
  - `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`
  - `fn find(self: &Self, byte: u8) -> Result<usize>`
  - `fn skip(self: & mut Self, len: usize) -> Result<()>`
  - `fn split(self: & mut Self, len: usize) -> Result<Self>`
  - `fn cannot_implement() -> super::reader::seal_if_no_alloc::Sealed`
  - `fn read_slice(self: & mut Self, buf: & mut [u8]) -> Result<()>`
- **Default**
  - `fn default() -> EndianSlice<'input, Endian>`
- **PartialEq**
  - `fn eq(self: &Self, other: &EndianSlice<'input, Endian>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Clone**
  - `fn clone(self: &Self) -> EndianSlice<'input, Endian>`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> core::result::Result<(), fmt::Error>`



