**gimli > read > index**

# Module: read::index

## Contents

**Structs**

- [`DebugCuIndex`](#debugcuindex) - The data in the `.debug_cu_index` section of a `.dwp` file.
- [`DebugTuIndex`](#debugtuindex) - The data in the `.debug_tu_index` section of a `.dwp` file.
- [`UnitIndex`](#unitindex) - The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.
- [`UnitIndexSection`](#unitindexsection) - Information about a unit's contribution to a section in a `.dwp` file.
- [`UnitIndexSectionIterator`](#unitindexsectioniterator) - An iterator over the section offsets and sizes for a row in a `UnitIndex`.

**Enums**

- [`IndexSectionId`](#indexsectionid) - Section kinds which are permitted in a `.dwp` index.

---

## gimli::read::index::DebugCuIndex

*Struct*

The data in the `.debug_cu_index` section of a `.dwp` file.

This section contains the compilation unit index.

**Generic Parameters:**
- R

**Methods:**

- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugCuIndex` instance from the data in the `.debug_cu_index`
- `fn index(self: Self) -> Result<UnitIndex<R>>` - Parse the index header.

**Traits:** Copy

**Trait Implementations:**

- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugCuIndex<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`
- **Clone**
  - `fn clone(self: &Self) -> DebugCuIndex<R>`



## gimli::read::index::DebugTuIndex

*Struct*

The data in the `.debug_tu_index` section of a `.dwp` file.

This section contains the type unit index.

**Generic Parameters:**
- R

**Methods:**

- `fn index(self: Self) -> Result<UnitIndex<R>>` - Parse the index header.
- `fn new(section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugTuIndex` instance from the data in the `.debug_tu_index`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugTuIndex<R>`
- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugTuIndex<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`



## gimli::read::index::IndexSectionId

*Enum*

Section kinds which are permitted in a `.dwp` index.

**Variants:**
- `DebugAbbrev` - The `.debug_abbrev.dwo` section.
- `DebugInfo` - The `.debug_info.dwo` section.
- `DebugLine` - The `.debug_line.dwo` section.
- `DebugLoc` - The `.debug_loc.dwo` section.
- `DebugLocLists` - The `.debug_loclists.dwo` section.
- `DebugMacinfo` - The `.debug_macinfo.dwo` section.
- `DebugMacro` - The `.debug_macro.dwo` section.
- `DebugRngLists` - The `.debug_rnglists.dwo` section.
- `DebugStrOffsets` - The `.debug_str_offsets.dwo` section.
- `DebugTypes` - The `.debug_types.dwo` section.

**Methods:**

- `fn section_id(self: Self) -> SectionId` - Returns the corresponding `SectionId`.
- `fn dwo_name(self: Self) -> &'static str` - Returns the ELF section name for this kind, when found in a .dwo or .dwp file.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &IndexSectionId) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IndexSectionId`



## gimli::read::index::UnitIndex

*Struct*

The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.

**Generic Parameters:**
- R

**Methods:**

- `fn find(self: &Self, id: u64) -> Option<u32>` - Find `id` in the index hash table, and return the row index.
- `fn sections(self: &Self, row: u32) -> Result<UnitIndexSectionIterator<R>>` - Return the section offsets and sizes for the given row index.
- `fn version(self: &Self) -> u16` - Return the version.
- `fn section_count(self: &Self) -> u32` - Return the number of sections.
- `fn unit_count(self: &Self) -> u32` - Return the number of units.
- `fn slot_count(self: &Self) -> u32` - Return the number of slots.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnitIndex<R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::index::UnitIndexSection

*Struct*

Information about a unit's contribution to a section in a `.dwp` file.

**Fields:**
- `section: IndexSectionId` - The section kind.
- `offset: u32` - The base offset of the unit's contribution to the section.
- `size: u32` - The size of the unit's contribution to the section.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &UnitIndexSection) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnitIndexSection`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::read::index::UnitIndexSectionIterator

*Struct*

An iterator over the section offsets and sizes for a row in a `UnitIndex`.

**Generic Parameters:**
- 'index
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<UnitIndexSection>`
- **Clone**
  - `fn clone(self: &Self) -> UnitIndexSectionIterator<'index, R>`



