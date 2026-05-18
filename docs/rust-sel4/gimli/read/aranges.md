**gimli > read > aranges**

# Module: read::aranges

## Contents

**Structs**

- [`ArangeEntry`](#arangeentry) - A single parsed arange.
- [`ArangeEntryIter`](#arangeentryiter) - An iterator over the aranges from a `.debug_aranges` section.
- [`ArangeHeader`](#arangeheader) - A header for a set of entries in the `.debug_arange` section.
- [`ArangeHeaderIter`](#arangeheaderiter) - An iterator over the headers of a `.debug_aranges` section.
- [`DebugAranges`](#debugaranges) - The `DebugAranges` struct represents the DWARF address range information

---

## gimli::read::aranges::ArangeEntry

*Struct*

A single parsed arange.

**Methods:**

- `fn address(self: &Self) -> u64` - Return the beginning address of this arange.
- `fn length(self: &Self) -> u64` - Return the length of this arange.
- `fn range(self: &Self) -> Range` - Return the range.

**Traits:** Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArangeEntry) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArangeEntry) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArangeEntry`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArangeEntry) -> bool`



## gimli::read::aranges::ArangeEntryIter

*Struct*

An iterator over the aranges from a `.debug_aranges` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<ArangeEntry>>` - Advance the iterator and return the next arange.
- `fn next_raw(self: & mut Self) -> Result<Option<ArangeEntry>>` - Advance the iterator and return the next arange without validating it.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ArangeEntryIter<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::aranges::ArangeHeader

*Struct*

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

**Generic Parameters:**
- R
- Offset

**Methods:**

- `fn offset(self: &Self) -> DebugArangesOffset<Offset>` - Return the offset of this header within the `.debug_aranges` section.
- `fn length(self: &Self) -> Offset` - Return the length of this set of entries, including the header.
- `fn encoding(self: &Self) -> Encoding` - Return the encoding parameters for this set of entries.
- `fn debug_info_offset(self: &Self) -> DebugInfoOffset<Offset>` - Return the offset into the .debug_info section for this set of arange entries.
- `fn entries(self: &Self) -> ArangeEntryIter<R>` - Return the arange entries in this set.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ArangeHeader<R, Offset>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArangeHeader<R, Offset>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::aranges::ArangeHeaderIter

*Struct*

An iterator over the headers of a `.debug_aranges` section.

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<ArangeHeader<R>>>` - Advance the iterator to the next header.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArangeHeaderIter<R>`



## gimli::read::aranges::DebugAranges

*Struct*

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

**Generic Parameters:**
- R

**Methods:**

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugAranges<R>` - Create a `DebugAranges` section that references the data in `self`.
- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugAranges` instance from the data in the `.debug_aranges`
- `fn headers(self: &Self) -> ArangeHeaderIter<R>` - Iterate the sets of entries in the `.debug_aranges` section.
- `fn header(self: &Self, offset: DebugArangesOffset<<R as >::Offset>) -> Result<ArangeHeader<R>>` - Get the header at the given offset.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugAranges<R>`
- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugAranges<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`



