**gimli > read > relocate**

# Module: read::relocate

## Contents

**Structs**

- [`RelocateReader`](#relocatereader) - A `Reader` which applies relocations to addresses and offsets.

**Traits**

- [`Relocate`](#relocate) - Trait for relocating addresses and offsets while reading a section.

---

## gimli::read::relocate::Relocate

*Trait*

Trait for relocating addresses and offsets while reading a section.

**Methods:**

- `relocate_address`: Relocate an address which was read from the given section offset.
- `relocate_offset`: Relocate a value which was read from the given section offset.



## gimli::read::relocate::RelocateReader

*Struct*

A `Reader` which applies relocations to addresses and offsets.

This is useful for reading sections which contain relocations,
such as those in a relocatable object file.
It is generally not used for reading sections in an executable file.

**Generic Parameters:**
- R
- T

**Methods:**

- `fn new(section: R, relocate: T) -> Self` - Create a new `RelocateReader` which applies relocations to the given section reader.

**Trait Implementations:**

- **Reader**
  - `fn read_address(self: & mut Self, address_size: u8) -> Result<u64>`
  - `fn read_offset(self: & mut Self, format: Format) -> Result<<R as >::Offset>`
  - `fn read_sized_offset(self: & mut Self, size: u8) -> Result<<R as >::Offset>`
  - `fn split(self: & mut Self, len: <Self as >::Offset) -> Result<Self>`
  - `fn endian(self: &Self) -> <Self as >::Endian`
  - `fn len(self: &Self) -> <Self as >::Offset`
  - `fn empty(self: & mut Self)`
  - `fn truncate(self: & mut Self, len: <Self as >::Offset) -> Result<()>`
  - `fn offset_from(self: &Self, base: &Self) -> <Self as >::Offset`
  - `fn offset_id(self: &Self) -> ReaderOffsetId`
  - `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`
  - `fn find(self: &Self, byte: u8) -> Result<<Self as >::Offset>`
  - `fn skip(self: & mut Self, len: <Self as >::Offset) -> Result<()>`
  - `fn cannot_implement() -> super::reader::seal_if_no_alloc::Sealed`
  - `fn read_slice(self: & mut Self, buf: & mut [u8]) -> Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RelocateReader<R, T>`



