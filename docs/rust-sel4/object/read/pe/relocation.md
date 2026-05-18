**object > read > pe > relocation**

# Module: read::pe::relocation

## Contents

**Structs**

- [`Relocation`](#relocation) - A relocation in the `.reloc` section of a PE file.
- [`RelocationBlockIterator`](#relocationblockiterator) - An iterator over the relocation blocks in the `.reloc` section of a PE file.
- [`RelocationIterator`](#relocationiterator) - An iterator of the relocations in a block in the `.reloc` section of a PE file.

---

## object::read::pe::relocation::Relocation

*Struct*

A relocation in the `.reloc` section of a PE file.

**Fields:**
- `virtual_address: u32` - The virtual address of the relocation.
- `typ: u16` - One of the `pe::IMAGE_REL_BASED_*` constants.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Relocation`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Relocation`



## object::read::pe::relocation::RelocationBlockIterator

*Struct*

An iterator over the relocation blocks in the `.reloc` section of a PE file.

Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks).

**Generic Parameters:**
- 'data

**Methods:**

- `fn new(data: &'data [u8]) -> Self` - Construct a new iterator from the data of the `.reloc` section.
- `fn next(self: & mut Self) -> Result<Option<RelocationIterator<'data>>>` - Read the next relocation page.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> RelocationBlockIterator<'data>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> RelocationBlockIterator<'data>`



## object::read::pe::relocation::RelocationIterator

*Struct*

An iterator of the relocations in a block in the `.reloc` section of a PE file.

**Generic Parameters:**
- 'data

**Methods:**

- `fn virtual_address(self: &Self) -> u32` - Return the virtual address of the page that this block of relocations applies to.
- `fn size(self: &Self) -> u32` - Return the size in bytes of this block of relocations.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<Relocation>`
- **Clone**
  - `fn clone(self: &Self) -> RelocationIterator<'data>`



