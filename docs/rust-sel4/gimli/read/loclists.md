**gimli > read > loclists**

# Module: read::loclists

## Contents

**Structs**

- [`DebugLoc`](#debugloc) - The raw contents of the `.debug_loc` section.
- [`DebugLocLists`](#debugloclists) - The `DebugLocLists` struct represents the DWARF data
- [`LocListIter`](#loclistiter) - An iterator over a location list.
- [`LocationListEntry`](#locationlistentry) - A location list entry from the `.debug_loc` or `.debug_loclists` sections.
- [`LocationLists`](#locationlists) - The DWARF data found in `.debug_loc` and `.debug_loclists` sections.
- [`RawLocListIter`](#rawloclistiter) - A raw iterator over a location list.

**Enums**

- [`RawLocListEntry`](#rawloclistentry) - A raw entry in .debug_loclists.

---

## gimli::read::loclists::DebugLoc

*Struct*

The raw contents of the `.debug_loc` section.

**Generic Parameters:**
- R

**Methods:**

- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugLoc` instance from the data in the `.debug_loc`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugLoc<R>`
- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugLoc<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`



## gimli::read::loclists::DebugLocLists

*Struct*

The `DebugLocLists` struct represents the DWARF data
found in the `.debug_loclists` section.

**Generic Parameters:**
- R

**Methods:**

- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugLocLists` instance from the data in the `.debug_loclists`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugLocLists<R>`
- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugLocLists<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`



## gimli::read::loclists::LocListIter

*Struct*

An iterator over a location list.

This iterator internally handles processing of base address selection entries
and list end entries.  Thus, it only returns location entries that are valid
and already adjusted for the base address.

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<LocationListEntry<R>>>` - Advance the iterator to the next location.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::loclists::LocationListEntry

*Struct*

A location list entry from the `.debug_loc` or `.debug_loclists` sections.

**Generic Parameters:**
- R

**Fields:**
- `range: crate::read::Range` - The address range that this location is valid for.
- `data: crate::read::Expression<R>` - The data containing a single location description.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LocationListEntry<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &LocationListEntry<R>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::read::loclists::LocationLists

*Struct*

The DWARF data found in `.debug_loc` and `.debug_loclists` sections.

**Generic Parameters:**
- R

**Methods:**

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> LocationLists<R>` - Create a `LocationLists` that references the data in `self`.
- `fn locations(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding, base_address: u64, debug_addr: &DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> Result<LocListIter<R>>` - Iterate over the `LocationListEntry`s starting at the given offset.
- `fn locations_dwo(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding, base_address: u64, debug_addr: &DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> Result<LocListIter<R>>` - Similar to `locations`, but with special handling for .dwo files.
- `fn raw_locations(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding) -> Result<RawLocListIter<R>>` - Iterate over the raw `LocationListEntry`s starting at the given offset.
- `fn raw_locations_dwo(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding) -> Result<RawLocListIter<R>>` - Similar to `raw_locations`, but with special handling for .dwo files.
- `fn get_offset(self: &Self, unit_encoding: Encoding, base: DebugLocListsBase<<R as >::Offset>, index: DebugLocListsIndex<<R as >::Offset>) -> Result<LocationListsOffset<<R as >::Offset>>` - Returns the `.debug_loclists` offset at the given `base` and `index`.
- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>` - Call `Reader::lookup_offset_id` for each section, and return the first match.
- `fn new(debug_loc: DebugLoc<R>, debug_loclists: DebugLocLists<R>) -> LocationLists<R>` - Construct a new `LocationLists` instance from the data in the `.debug_loc` and

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> LocationLists<R>`
- **Clone**
  - `fn clone(self: &Self) -> LocationLists<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::loclists::RawLocListEntry

*Enum*

A raw entry in .debug_loclists.

**Generic Parameters:**
- R

**Variants:**
- `AddressOrOffsetPair{ begin: u64, end: u64, data: crate::read::Expression<R> }` - A location from DWARF version <= 4.
- `BaseAddress{ addr: u64 }` - DW_LLE_base_address
- `BaseAddressx{ addr: crate::common::DebugAddrIndex<<R as >::Offset> }` - DW_LLE_base_addressx
- `StartxEndx{ begin: crate::common::DebugAddrIndex<<R as >::Offset>, end: crate::common::DebugAddrIndex<<R as >::Offset>, data: crate::read::Expression<R> }` - DW_LLE_startx_endx
- `StartxLength{ begin: crate::common::DebugAddrIndex<<R as >::Offset>, length: u64, data: crate::read::Expression<R> }` - DW_LLE_startx_length
- `OffsetPair{ begin: u64, end: u64, data: crate::read::Expression<R> }` - DW_LLE_offset_pair
- `DefaultLocation{ data: crate::read::Expression<R> }` - DW_LLE_default_location
- `StartEnd{ begin: u64, end: u64, data: crate::read::Expression<R> }` - DW_LLE_start_end
- `StartLength{ begin: u64, length: u64, data: crate::read::Expression<R> }` - DW_LLE_start_length

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RawLocListEntry<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::loclists::RawLocListIter

*Struct*

A raw iterator over a location list.

This iterator does not perform any processing of the location entries,
such as handling base addresses.

**Generic Parameters:**
- R

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<RawLocListEntry<R>>>` - Advance the iterator to the next location.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



