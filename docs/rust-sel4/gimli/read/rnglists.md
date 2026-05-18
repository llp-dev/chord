**gimli > read > rnglists**

# Module: read::rnglists

## Contents

**Structs**

- [`DebugRanges`](#debugranges) - The raw contents of the `.debug_ranges` section.
- [`DebugRngLists`](#debugrnglists) - The `DebugRngLists` struct represents the contents of the
- [`Range`](#range) - An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.
- [`RangeLists`](#rangelists) - The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.
- [`RawRngListIter`](#rawrnglistiter) - A raw iterator over an address range list.
- [`RngListIter`](#rnglistiter) - An iterator over an address range list.

**Enums**

- [`RawRngListEntry`](#rawrnglistentry) - A raw entry in .debug_rnglists

---

## gimli::read::rnglists::DebugRanges

*Struct*

The raw contents of the `.debug_ranges` section.

**Generic Parameters:**
- R

**Methods:**

- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugRanges` instance from the data in the `.debug_ranges`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugRanges<R>`
- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugRanges<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`



## gimli::read::rnglists::DebugRngLists

*Struct*

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

**Generic Parameters:**
- R

**Methods:**

- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugRngLists` instance from the data in the

**Traits:** Copy

**Trait Implementations:**

- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugRngLists<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`
- **Clone**
  - `fn clone(self: &Self) -> DebugRngLists<R>`



## gimli::read::rnglists::Range

*Struct*

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

**Fields:**
- `begin: u64` - The beginning address of the range.
- `end: u64` - The first address past the end of the range.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Range) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Range) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Range`
- **Ord**
  - `fn cmp(self: &Self, other: &Range) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::read::rnglists::RangeLists

*Struct*

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

**Generic Parameters:**
- R

**Methods:**

- `fn new(debug_ranges: DebugRanges<R>, debug_rnglists: DebugRngLists<R>) -> RangeLists<R>` - Construct a new `RangeLists` instance from the data in the `.debug_ranges` and
- `fn debug_ranges(self: &Self) -> &DebugRanges<R>` - Return the `.debug_ranges` section.
- `fn set_debug_ranges(self: & mut Self, debug_ranges: DebugRanges<R>)` - Replace the `.debug_ranges` section.
- `fn debug_rnglists(self: &Self) -> &DebugRngLists<R>` - Return the `.debug_rnglists` section.
- `fn ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>, unit_encoding: Encoding, base_address: u64, debug_addr: &DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> Result<RngListIter<R>>` - Iterate over the `Range` list entries starting at the given offset.
- `fn raw_ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>, unit_encoding: Encoding) -> Result<RawRngListIter<R>>` - Iterate over the `RawRngListEntry`ies starting at the given offset.
- `fn get_offset(self: &Self, unit_encoding: Encoding, base: DebugRngListsBase<<R as >::Offset>, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` - Returns the `.debug_rnglists` offset at the given `base` and `index`.
- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>` - Call `Reader::lookup_offset_id` for each section, and return the first match.
- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> RangeLists<R>` - Create a `RangeLists` that references the data in `self`.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RangeLists<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> RangeLists<R>`



## gimli::read::rnglists::RawRngListEntry

*Enum*

A raw entry in .debug_rnglists

**Generic Parameters:**
- T

**Variants:**
- `AddressOrOffsetPair{ begin: u64, end: u64 }` - A range from DWARF version <= 4.
- `BaseAddress{ addr: u64 }` - DW_RLE_base_address
- `BaseAddressx{ addr: crate::common::DebugAddrIndex<T> }` - DW_RLE_base_addressx
- `StartxEndx{ begin: crate::common::DebugAddrIndex<T>, end: crate::common::DebugAddrIndex<T> }` - DW_RLE_startx_endx
- `StartxLength{ begin: crate::common::DebugAddrIndex<T>, length: u64 }` - DW_RLE_startx_length
- `OffsetPair{ begin: u64, end: u64 }` - DW_RLE_offset_pair
- `StartEnd{ begin: u64, end: u64 }` - DW_RLE_start_end
- `StartLength{ begin: u64, length: u64 }` - DW_RLE_start_length

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RawRngListEntry<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::rnglists::RawRngListIter

*Struct*

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` - Advance the iterator to the next range.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::rnglists::RngListIter

*Struct*

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<Range>>` - Advance the iterator to the next range.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



