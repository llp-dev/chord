**gimli > read > str**

# Module: read::str

## Contents

**Structs**

- [`DebugLineStr`](#debuglinestr) - The `DebugLineStr` struct represents the DWARF strings
- [`DebugStr`](#debugstr) - The `DebugStr` struct represents the DWARF strings
- [`DebugStrOffsets`](#debugstroffsets) - The raw contents of the `.debug_str_offsets` section.

---

## gimli::read::str::DebugLineStr

*Struct*

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

**Generic Parameters:**
- R

**Methods:**

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugLineStr<R>` - Create a `DebugLineStr` section that references the data in `self`.
- `fn new(debug_line_str_section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugLineStr` instance from the data in the `.debug_line_str`
- `fn get_str(self: &Self, offset: DebugLineStrOffset<<R as >::Offset>) -> Result<R>` - Lookup a string from the `.debug_line_str` section by DebugLineStrOffset.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugLineStr<R>`
- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugLineStr<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`



## gimli::read::str::DebugStr

*Struct*

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

**Generic Parameters:**
- R

**Methods:**

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugStr<R>` - Create a `DebugStr` section that references the data in `self`.
- `fn new(debug_str_section: &'input [u8], endian: Endian) -> Self` - Construct a new `DebugStr` instance from the data in the `.debug_str`
- `fn get_str(self: &Self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` - Lookup a string from the `.debug_str` section by DebugStrOffset.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugStr<R>`
- **From**
  - `fn from(debug_str_section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> DebugStr<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`



## gimli::read::str::DebugStrOffsets

*Struct*

The raw contents of the `.debug_str_offsets` section.

**Generic Parameters:**
- R

**Methods:**

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugStrOffsets<R>` - Create a `DebugStrOffsets` section that references the data in `self`.
- `fn get_str_offset(self: &Self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` - Returns the `.debug_str` offset at the given `base` and `index`.

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> DebugStrOffsets<R>`
- **Section**
  - `fn id() -> SectionId`
  - `fn reader(self: &Self) -> &R`
- **Clone**
  - `fn clone(self: &Self) -> DebugStrOffsets<R>`
- **From**
  - `fn from(section: R) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



