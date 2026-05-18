**gimli > common**

# Module: common

## Contents

**Structs**

- [`DebugAbbrevOffset`](#debugabbrevoffset) - An offset into the `.debug_abbrev` section.
- [`DebugAddrBase`](#debugaddrbase) - An offset to a set of entries in the `.debug_addr` section.
- [`DebugAddrIndex`](#debugaddrindex) - An index into a set of addresses in the `.debug_addr` section.
- [`DebugAddrOffset`](#debugaddroffset) - An offset into the `.debug_addr` section.
- [`DebugArangesOffset`](#debugarangesoffset) - An offset into the `.debug_aranges` section.
- [`DebugFrameOffset`](#debugframeoffset) - An offset into the `.debug_frame` section.
- [`DebugInfoOffset`](#debuginfooffset) - An offset into the `.debug_info` section.
- [`DebugLineOffset`](#debuglineoffset) - An offset into the `.debug_line` section.
- [`DebugLineStrOffset`](#debuglinestroffset) - An offset into the `.debug_line_str` section.
- [`DebugLocListsBase`](#debugloclistsbase) - An offset to a set of location list offsets in the `.debug_loclists` section.
- [`DebugLocListsIndex`](#debugloclistsindex) - An index into a set of location list offsets in the `.debug_loclists` section.
- [`DebugMacinfoOffset`](#debugmacinfooffset) - An offset into the `.debug_macinfo` section.
- [`DebugMacroOffset`](#debugmacrooffset) - An offset into the `.debug_macro` section.
- [`DebugRngListsBase`](#debugrnglistsbase) - An offset to a set of range list offsets in the `.debug_rnglists` section.
- [`DebugRngListsIndex`](#debugrnglistsindex) - An index into a set of range list offsets in the `.debug_rnglists` section.
- [`DebugStrOffset`](#debugstroffset) - An offset into the `.debug_str` section.
- [`DebugStrOffsetsBase`](#debugstroffsetsbase) - An offset to a set of entries in the `.debug_str_offsets` section.
- [`DebugStrOffsetsIndex`](#debugstroffsetsindex) - An index into a set of entries in the `.debug_str_offsets` section.
- [`DebugTypeSignature`](#debugtypesignature) - A type signature as used in the `.debug_types` section.
- [`DebugTypesOffset`](#debugtypesoffset) - An offset into the `.debug_types` section.
- [`DwoId`](#dwoid) - An optionally-provided implementation-defined compilation unit ID to enable
- [`EhFrameOffset`](#ehframeoffset) - An offset into the `.eh_frame` section.
- [`Encoding`](#encoding) - Encoding parameters that are commonly used for multiple DWARF sections.
- [`LineEncoding`](#lineencoding) - Encoding parameters for a line number program.
- [`LocationListsOffset`](#locationlistsoffset) - An offset into either the `.debug_loc` section or the `.debug_loclists` section,
- [`RangeListsOffset`](#rangelistsoffset) - An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
- [`RawRangeListsOffset`](#rawrangelistsoffset) - An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
- [`Register`](#register) - A DWARF register number.

**Enums**

- [`DwarfFileType`](#dwarffiletype) - The "type" of file with DWARF debugging information. This determines, among other things,
- [`Format`](#format) - Whether the format of a compilation unit is 32- or 64-bit.
- [`SectionId`](#sectionid) - An identifier for a DWARF section.
- [`UnitSectionOffset`](#unitsectionoffset) - An offset into the `.debug_info` or `.debug_types` sections.
- [`Vendor`](#vendor) - Which vendor extensions to support.

---

## gimli::common::DebugAbbrevOffset

*Struct*

An offset into the `.debug_abbrev` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugAbbrevOffset<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugAbbrevOffset<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::common::DebugAddrBase

*Struct*

An offset to a set of entries in the `.debug_addr` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugAddrBase<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugAddrBase<T>) -> bool`



## gimli::common::DebugAddrIndex

*Struct*

An index into a set of addresses in the `.debug_addr` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugAddrIndex<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugAddrIndex<T>`



## gimli::common::DebugAddrOffset

*Struct*

An offset into the `.debug_addr` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugAddrOffset<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugAddrOffset<T>) -> bool`



## gimli::common::DebugArangesOffset

*Struct*

An offset into the `.debug_aranges` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugArangesOffset<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugArangesOffset<T>`



## gimli::common::DebugFrameOffset

*Struct*

An offset into the `.debug_frame` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(o: T) -> Self`
- **UnwindOffset**
  - `fn into(self: Self) -> T`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugFrameOffset<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DebugFrameOffset<T>`



## gimli::common::DebugInfoOffset

*Struct*

An offset into the `.debug_info` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugInfoOffset<T>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &DebugInfoOffset<T>) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> DebugInfoOffset<T>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DebugInfoOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::common::DebugLineOffset

*Struct*

An offset into the `.debug_line` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugLineOffset<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugLineOffset<T>`



## gimli::common::DebugLineStrOffset

*Struct*

An offset into the `.debug_line_str` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugLineStrOffset<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugLineStrOffset<T>) -> bool`



## gimli::common::DebugLocListsBase

*Struct*

An offset to a set of location list offsets in the `.debug_loclists` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Methods:**

- `fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugLocListsBase<Offset>` - Returns a `DebugLocListsBase` with the default value of DW_AT_loclists_base

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugLocListsBase<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugLocListsBase<T>`



## gimli::common::DebugLocListsIndex

*Struct*

An index into a set of location list offsets in the `.debug_loclists` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugLocListsIndex<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugLocListsIndex<T>`



## gimli::common::DebugMacinfoOffset

*Struct*

An offset into the `.debug_macinfo` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugMacinfoOffset<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DebugMacinfoOffset<T>`



## gimli::common::DebugMacroOffset

*Struct*

An offset into the `.debug_macro` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugMacroOffset<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DebugMacroOffset<T>`



## gimli::common::DebugRngListsBase

*Struct*

An offset to a set of range list offsets in the `.debug_rnglists` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Methods:**

- `fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugRngListsBase<Offset>` - Returns a `DebugRngListsBase` with the default value of DW_AT_rnglists_base

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugRngListsBase<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugRngListsBase<T>`



## gimli::common::DebugRngListsIndex

*Struct*

An index into a set of range list offsets in the `.debug_rnglists` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugRngListsIndex<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugRngListsIndex<T>`



## gimli::common::DebugStrOffset

*Struct*

An offset into the `.debug_str` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DebugStrOffset<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugStrOffset<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::common::DebugStrOffsetsBase

*Struct*

An offset to a set of entries in the `.debug_str_offsets` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Methods:**

- `fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugStrOffsetsBase<Offset>` - Returns a `DebugStrOffsetsBase` with the default value of DW_AT_str_offsets_base

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugStrOffsetsBase<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugStrOffsetsBase<T>`



## gimli::common::DebugStrOffsetsIndex

*Struct*

An index into a set of entries in the `.debug_str_offsets` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &DebugStrOffsetsIndex<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DebugStrOffsetsIndex<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::common::DebugTypeSignature

*Struct*

A type signature as used in the `.debug_types` section.

**Tuple Struct**: `(u64)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugTypeSignature) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DebugTypeSignature`



## gimli::common::DebugTypesOffset

*Struct*

An offset into the `.debug_types` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DebugTypesOffset<T>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DebugTypesOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DebugTypesOffset<T>) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &DebugTypesOffset<T>) -> $crate::cmp::Ordering`



## gimli::common::DwarfFileType

*Enum*

The "type" of file with DWARF debugging information. This determines, among other things,
which files DWARF sections should be loaded from.

**Variants:**
- `Main` - A normal executable or object file.
- `Dwo` - A .dwo split DWARF file.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwarfFileType) -> bool`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> DwarfFileType`



## gimli::common::DwoId

*Struct*

An optionally-provided implementation-defined compilation unit ID to enable
split DWARF and linking a split compilation unit back together.

**Tuple Struct**: `(u64)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DwoId`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DwoId) -> bool`



## gimli::common::EhFrameOffset

*Struct*

An offset into the `.eh_frame` section.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> EhFrameOffset<T>`
- **From**
  - `fn from(o: T) -> Self`
- **UnwindOffset**
  - `fn into(self: Self) -> T`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &EhFrameOffset<T>) -> bool`



## gimli::common::Encoding

*Struct*

Encoding parameters that are commonly used for multiple DWARF sections.

This is intended to be small enough to pass by value.

**Fields:**
- `address_size: u8` - The size of an address.
- `format: Format` - Whether the DWARF format is 32- or 64-bit.
- `version: u16` - The DWARF version of the header.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Encoding`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Encoding) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::common::Format

*Enum*

Whether the format of a compilation unit is 32- or 64-bit.

**Variants:**
- `Dwarf64` - 64-bit DWARF
- `Dwarf32` - 32-bit DWARF

**Methods:**

- `fn initial_length_size(self: Self) -> u8` - Return the serialized size of an initial length field for the format.
- `fn word_size(self: Self) -> u8` - Return the natural word size for the format

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Format`
- **PartialEq**
  - `fn eq(self: &Self, other: &Format) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::common::LineEncoding

*Struct*

Encoding parameters for a line number program.

**Fields:**
- `minimum_instruction_length: u8` - The size in bytes of the smallest target machine instruction.
- `maximum_operations_per_instruction: u8` - The maximum number of individual operations that may be encoded in an
- `default_is_stmt: bool` - The initial value of the `is_stmt` register.
- `line_base: i8` - The minimum value which a special opcode can add to the line register.
- `line_range: u8` - The range of values which a special opcode can add to the line register.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LineEncoding`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &LineEncoding) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::common::LocationListsOffset

*Struct*

An offset into either the `.debug_loc` section or the `.debug_loclists` section,
depending on the version of the unit the offset was contained in.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LocationListsOffset<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &LocationListsOffset<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::common::RangeListsOffset

*Struct*

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RangeListsOffset<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RangeListsOffset<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gimli::common::RawRangeListsOffset

*Struct*

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

If this is from a DWARF 4 DWO file, then it must additionally be offset by the
value of `DW_AT_GNU_ranges_base`. You can use `Dwarf::ranges_offset_from_raw` to do this.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RawRangeListsOffset<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> RawRangeListsOffset<T>`



## gimli::common::Register

*Struct*

A DWARF register number.

The meaning of this value is ABI dependent. This is generally encoded as
a ULEB128, but supported architectures need 16 bits at most.

**Tuple Struct**: `(u16)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Register) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Register`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Register) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Register) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::common::SectionId

*Enum*

An identifier for a DWARF section.

**Variants:**
- `DebugAbbrev` - The `.debug_abbrev` section.
- `DebugAddr` - The `.debug_addr` section.
- `DebugAranges` - The `.debug_aranges` section.
- `DebugCuIndex` - The `.debug_cu_index` section.
- `DebugFrame` - The `.debug_frame` section.
- `EhFrame` - The `.eh_frame` section.
- `EhFrameHdr` - The `.eh_frame_hdr` section.
- `DebugInfo` - The `.debug_info` section.
- `DebugLine` - The `.debug_line` section.
- `DebugLineStr` - The `.debug_line_str` section.
- `DebugLoc` - The `.debug_loc` section.
- `DebugLocLists` - The `.debug_loclists` section.
- `DebugMacinfo` - The `.debug_macinfo` section.
- `DebugMacro` - The `.debug_macro` section.
- `DebugPubNames` - The `.debug_pubnames` section.
- `DebugPubTypes` - The `.debug_pubtypes` section.
- `DebugRanges` - The `.debug_ranges` section.
- `DebugRngLists` - The `.debug_rnglists` section.
- `DebugStr` - The `.debug_str` section.
- `DebugStrOffsets` - The `.debug_str_offsets` section.
- `DebugTuIndex` - The `.debug_tu_index` section.
- `DebugTypes` - The `.debug_types` section.

**Methods:**

- `fn name(self: Self) -> &'static str` - Returns the ELF section name for this kind.
- `fn dwo_name(self: Self) -> Option<&'static str>` - Returns the ELF section name for this kind, when found in a .dwo or .dwp file.
- `fn xcoff_name(self: Self) -> Option<&'static str>` - Returns the XCOFF section name for this kind.
- `fn is_string(self: Self) -> bool` - Returns true if this is a mergeable string section.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SectionId) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &SectionId) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> SectionId`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SectionId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::common::UnitSectionOffset

*Enum*

An offset into the `.debug_info` or `.debug_types` sections.

**Generic Parameters:**
- T

**Variants:**
- `DebugInfoOffset(DebugInfoOffset<T>)` - An offset into the `.debug_info` section.
- `DebugTypesOffset(DebugTypesOffset<T>)` - An offset into the `.debug_types` section.

**Methods:**

- `fn as_debug_info_offset(self: &Self) -> Option<DebugInfoOffset<T>>` - Returns the `DebugInfoOffset` inside, or `None` otherwise.
- `fn as_debug_types_offset(self: &Self) -> Option<DebugTypesOffset<T>>` - Returns the `DebugTypesOffset` inside, or `None` otherwise.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnitSectionOffset<T>) -> bool`
- **From**
  - `fn from(offset: DebugInfoOffset<T>) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &UnitSectionOffset<T>) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> UnitSectionOffset<T>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &UnitSectionOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(offset: DebugTypesOffset<T>) -> Self`



## gimli::common::Vendor

*Enum*

Which vendor extensions to support.

**Variants:**
- `Default` - A default set of extensions, including some common GNU extensions.
- `AArch64` - AAarch64 extensions.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Vendor`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Vendor) -> bool`



