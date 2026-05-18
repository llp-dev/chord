*[gimli](../index.md) / [common](index.md)*

---

# Module `common`

## Contents

- [Structs](#structs)
  - [`Encoding`](#encoding)
  - [`LineEncoding`](#lineencoding)
  - [`Register`](#register)
  - [`DebugAbbrevOffset`](#debugabbrevoffset)
  - [`DebugAddrOffset`](#debugaddroffset)
  - [`DebugAddrBase`](#debugaddrbase)
  - [`DebugAddrIndex`](#debugaddrindex)
  - [`DebugArangesOffset`](#debugarangesoffset)
  - [`DebugInfoOffset`](#debuginfooffset)
  - [`DebugLineOffset`](#debuglineoffset)
  - [`DebugLineStrOffset`](#debuglinestroffset)
  - [`LocationListsOffset`](#locationlistsoffset)
  - [`DebugLocListsBase`](#debugloclistsbase)
  - [`DebugLocListsIndex`](#debugloclistsindex)
  - [`DebugMacinfoOffset`](#debugmacinfooffset)
  - [`DebugMacroOffset`](#debugmacrooffset)
  - [`DebugNamesOffset`](#debugnamesoffset)
  - [`RawRangeListsOffset`](#rawrangelistsoffset)
  - [`RangeListsOffset`](#rangelistsoffset)
  - [`DebugRngListsBase`](#debugrnglistsbase)
  - [`DebugRngListsIndex`](#debugrnglistsindex)
  - [`DebugStrOffset`](#debugstroffset)
  - [`DebugStrOffsetsBase`](#debugstroffsetsbase)
  - [`DebugStrOffsetsIndex`](#debugstroffsetsindex)
  - [`DebugTypesOffset`](#debugtypesoffset)
  - [`DebugTypeSignature`](#debugtypesignature)
  - [`DebugFrameOffset`](#debugframeoffset)
  - [`EhFrameOffset`](#ehframeoffset)
  - [`UnitSectionOffset`](#unitsectionoffset)
  - [`DwoId`](#dwoid)
- [Enums](#enums)
  - [`Format`](#format)
  - [`Vendor`](#vendor)
  - [`SectionId`](#sectionid)
  - [`DwarfFileType`](#dwarffiletype)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Encoding`](#encoding) | struct | Encoding parameters that are commonly used for multiple DWARF sections. |
| [`LineEncoding`](#lineencoding) | struct | Encoding parameters for a line number program. |
| [`Register`](#register) | struct | A DWARF register number. |
| [`DebugAbbrevOffset`](#debugabbrevoffset) | struct | An offset into the `.debug_abbrev` section. |
| [`DebugAddrOffset`](#debugaddroffset) | struct | An offset into the `.debug_addr` section. |
| [`DebugAddrBase`](#debugaddrbase) | struct | An offset to a set of entries in the `.debug_addr` section. |
| [`DebugAddrIndex`](#debugaddrindex) | struct | An index into a set of addresses in the `.debug_addr` section. |
| [`DebugArangesOffset`](#debugarangesoffset) | struct | An offset into the `.debug_aranges` section. |
| [`DebugInfoOffset`](#debuginfooffset) | struct | An offset into the `.debug_info` section. |
| [`DebugLineOffset`](#debuglineoffset) | struct | An offset into the `.debug_line` section. |
| [`DebugLineStrOffset`](#debuglinestroffset) | struct | An offset into the `.debug_line_str` section. |
| [`LocationListsOffset`](#locationlistsoffset) | struct | An offset into either the `.debug_loc` section or the `.debug_loclists` section, depending on the version of the unit the offset was contained in. |
| [`DebugLocListsBase`](#debugloclistsbase) | struct | An offset to a set of location list offsets in the `.debug_loclists` section. |
| [`DebugLocListsIndex`](#debugloclistsindex) | struct | An index into a set of location list offsets in the `.debug_loclists` section. |
| [`DebugMacinfoOffset`](#debugmacinfooffset) | struct | An offset into the `.debug_macinfo` section. |
| [`DebugMacroOffset`](#debugmacrooffset) | struct | An offset into the `.debug_macro` section. |
| [`DebugNamesOffset`](#debugnamesoffset) | struct | An offset into the `.debug_names` section. |
| [`RawRangeListsOffset`](#rawrangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section, depending on the version of the unit the offset was contained in. |
| [`RangeListsOffset`](#rangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section, depending on the version of the unit the offset was contained in. |
| [`DebugRngListsBase`](#debugrnglistsbase) | struct | An offset to a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugRngListsIndex`](#debugrnglistsindex) | struct | An index into a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugStrOffset`](#debugstroffset) | struct | An offset into the `.debug_str` section. |
| [`DebugStrOffsetsBase`](#debugstroffsetsbase) | struct | An offset to a set of entries in the `.debug_str_offsets` section. |
| [`DebugStrOffsetsIndex`](#debugstroffsetsindex) | struct | An index into a set of entries in the `.debug_str_offsets` section. |
| [`DebugTypesOffset`](#debugtypesoffset) | struct | An offset into the `.debug_types` section. |
| [`DebugTypeSignature`](#debugtypesignature) | struct | A type signature as used in the `.debug_types` section. |
| [`DebugFrameOffset`](#debugframeoffset) | struct | An offset into the `.debug_frame` section. |
| [`EhFrameOffset`](#ehframeoffset) | struct | An offset into the `.eh_frame` section. |
| [`UnitSectionOffset`](#unitsectionoffset) | struct | An offset into the `.debug_info` or `.debug_types` sections. |
| [`DwoId`](#dwoid) | struct | An optionally-provided implementation-defined compilation unit ID to enable split DWARF and linking a split compilation unit back together. |
| [`Format`](#format) | enum | Whether the format of a compilation unit is 32- or 64-bit. |
| [`Vendor`](#vendor) | enum | Which vendor extensions to support. |
| [`SectionId`](#sectionid) | enum | An identifier for a DWARF section. |
| [`DwarfFileType`](#dwarffiletype) | enum | The "type" of file with DWARF debugging information. |

## Structs

### `Encoding`

```rust
struct Encoding {
    pub address_size: u8,
    pub format: Format,
    pub version: u16,
}
```

Encoding parameters that are commonly used for multiple DWARF sections.

This is intended to be small enough to pass by value.

#### Fields

- **`address_size`**: `u8`

  The size of an address.

- **`format`**: `Format`

  Whether the DWARF format is 32- or 64-bit.

- **`version`**: `u16`

  The DWARF version of the header.

#### Trait Implementations

##### `impl Clone for Encoding`

- <span id="encoding-clone"></span>`fn clone(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

##### `impl Copy for Encoding`

##### `impl Debug for Encoding`

- <span id="encoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Encoding`

##### `impl Hash for Encoding`

- <span id="encoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Encoding`

- <span id="encoding-partialeq-eq"></span>`fn eq(&self, other: &Encoding) -> bool` — [`Encoding`](../index.md#encoding)

##### `impl StructuralPartialEq for Encoding`

### `LineEncoding`

```rust
struct LineEncoding {
    pub minimum_instruction_length: u8,
    pub maximum_operations_per_instruction: u8,
    pub default_is_stmt: bool,
    pub line_base: i8,
    pub line_range: u8,
}
```

Encoding parameters for a line number program.

#### Fields

- **`minimum_instruction_length`**: `u8`

  The size in bytes of the smallest target machine instruction.

- **`maximum_operations_per_instruction`**: `u8`

  The maximum number of individual operations that may be encoded in an
  instruction.

- **`default_is_stmt`**: `bool`

  The initial value of the `is_stmt` register.

- **`line_base`**: `i8`

  The minimum value which a special opcode can add to the line register.

- **`line_range`**: `u8`

  The range of values which a special opcode can add to the line register.

#### Trait Implementations

##### `impl Clone for LineEncoding`

- <span id="lineencoding-clone"></span>`fn clone(&self) -> LineEncoding` — [`LineEncoding`](../index.md#lineencoding)

##### `impl Copy for LineEncoding`

##### `impl Debug for LineEncoding`

- <span id="lineencoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LineEncoding`

- <span id="lineencoding-default"></span>`fn default() -> Self`

##### `impl Eq for LineEncoding`

##### `impl Hash for LineEncoding`

- <span id="lineencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LineEncoding`

- <span id="lineencoding-partialeq-eq"></span>`fn eq(&self, other: &LineEncoding) -> bool` — [`LineEncoding`](../index.md#lineencoding)

##### `impl StructuralPartialEq for LineEncoding`

### `Register`

```rust
struct Register(u16);
```

A DWARF register number.

The meaning of this value is ABI dependent. This is generally encoded as
a ULEB128, but supported architectures need 16 bits at most.

#### Implementations

- <span id="cratecommonregister-from-u64"></span>`fn from_u64(x: u64) -> Result<Register>` — [`Result`](../index.md#result), [`Register`](../index.md#register)

#### Trait Implementations

##### `impl Clone for Register`

- <span id="register-clone"></span>`fn clone(&self) -> Register` — [`Register`](../index.md#register)

##### `impl Copy for Register`

##### `impl Debug for Register`

- <span id="register-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Register`

##### `impl Hash for Register`

- <span id="register-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Register`

- <span id="register-ord-cmp"></span>`fn cmp(&self, other: &Register) -> cmp::Ordering` — [`Register`](../index.md#register)

##### `impl PartialEq for Register`

- <span id="register-partialeq-eq"></span>`fn eq(&self, other: &Register) -> bool` — [`Register`](../index.md#register)

##### `impl PartialOrd for Register`

- <span id="register-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Register) -> option::Option<cmp::Ordering>` — [`Register`](../index.md#register)

##### `impl StructuralPartialEq for Register`

### `DebugAbbrevOffset<T>`

```rust
struct DebugAbbrevOffset<T>(T);
```

An offset into the `.debug_abbrev` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-clone"></span>`fn clone(&self) -> DebugAbbrevOffset<T>` — [`DebugAbbrevOffset`](../index.md#debugabbrevoffset)

##### `impl<T: marker::Copy> Copy for DebugAbbrevOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAbbrevOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugAbbrevOffset<T>) -> bool` — [`DebugAbbrevOffset`](../index.md#debugabbrevoffset)

##### `impl<T> StructuralPartialEq for DebugAbbrevOffset<T>`

### `DebugAddrOffset<T>`

```rust
struct DebugAddrOffset<T>(T);
```

An offset into the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrOffset<T>`

- <span id="debugaddroffset-clone"></span>`fn clone(&self) -> DebugAddrOffset<T>` — [`DebugAddrOffset`](../index.md#debugaddroffset)

##### `impl<T: marker::Copy> Copy for DebugAddrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrOffset<T>`

- <span id="debugaddroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrOffset<T>`

- <span id="debugaddroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrOffset<T>) -> bool` — [`DebugAddrOffset`](../index.md#debugaddroffset)

##### `impl<T> StructuralPartialEq for DebugAddrOffset<T>`

### `DebugAddrBase<T>`

```rust
struct DebugAddrBase<T>(T);
```

An offset to a set of entries in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrBase<T>`

- <span id="debugaddrbase-clone"></span>`fn clone(&self) -> DebugAddrBase<T>` — [`DebugAddrBase`](../index.md#debugaddrbase)

##### `impl<T: marker::Copy> Copy for DebugAddrBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrBase<T>`

- <span id="debugaddrbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrBase<T>`

- <span id="debugaddrbase-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrBase<T>) -> bool` — [`DebugAddrBase`](../index.md#debugaddrbase)

##### `impl<T> StructuralPartialEq for DebugAddrBase<T>`

### `DebugAddrIndex<T>`

```rust
struct DebugAddrIndex<T>(T);
```

An index into a set of addresses in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugAddrIndex<T>`

- <span id="debugaddrindex-clone"></span>`fn clone(&self) -> DebugAddrIndex<T>` — [`DebugAddrIndex`](../index.md#debugaddrindex)

##### `impl<T: marker::Copy> Copy for DebugAddrIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrIndex<T>`

- <span id="debugaddrindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrIndex<T>`

- <span id="debugaddrindex-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrIndex<T>) -> bool` — [`DebugAddrIndex`](../index.md#debugaddrindex)

##### `impl<T> StructuralPartialEq for DebugAddrIndex<T>`

### `DebugArangesOffset<T>`

```rust
struct DebugArangesOffset<T>(T);
```

An offset into the `.debug_aranges` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugArangesOffset<T>`

- <span id="debugarangesoffset-clone"></span>`fn clone(&self) -> DebugArangesOffset<T>` — [`DebugArangesOffset`](../index.md#debugarangesoffset)

##### `impl<T: marker::Copy> Copy for DebugArangesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugArangesOffset<T>`

- <span id="debugarangesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugArangesOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugArangesOffset<T>`

- <span id="debugarangesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugArangesOffset<T>) -> bool` — [`DebugArangesOffset`](../index.md#debugarangesoffset)

##### `impl<T> StructuralPartialEq for DebugArangesOffset<T>`

### `DebugInfoOffset<T>`

```rust
struct DebugInfoOffset<T>(T);
```

An offset into the `.debug_info` section.

#### Implementations

- <span id="cratecommondebuginfooffset-to-unit-section-offset"></span>`fn to_unit_section_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitSectionOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`UnitSectionOffset`](../index.md#unitsectionoffset)

  Convert a `.debug_info` offset to be an offset within the section containing the

  given unit.

  

  Returns `None` if the unit is not within the `.debug_info` section.

- <span id="cratecommondebuginfooffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`UnitOffset`](../index.md#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the `.debug_info` section.

  

  Returns `None` if the offset is not in bounds for the unit's entries.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugInfoOffset<T>`

- <span id="debuginfooffset-clone"></span>`fn clone(&self) -> DebugInfoOffset<T>` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T: marker::Copy> Copy for DebugInfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugInfoOffset<T>`

- <span id="debuginfooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugInfoOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugInfoOffset<T>`

- <span id="debuginfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for DebugInfoOffset<T>`

- <span id="debuginfooffset-ord-cmp"></span>`fn cmp(&self, other: &DebugInfoOffset<T>) -> cmp::Ordering` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugInfoOffset<T>`

- <span id="debuginfooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugInfoOffset<T>) -> bool` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugInfoOffset<T>`

- <span id="debuginfooffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugInfoOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T> StructuralPartialEq for DebugInfoOffset<T>`

### `DebugLineOffset<T>`

```rust
struct DebugLineOffset<T>(T);
```

An offset into the `.debug_line` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLineOffset<T>`

- <span id="debuglineoffset-clone"></span>`fn clone(&self) -> DebugLineOffset<T>` — [`DebugLineOffset`](../index.md#debuglineoffset)

##### `impl<T: marker::Copy> Copy for DebugLineOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineOffset<T>`

- <span id="debuglineoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineOffset<T>`

- <span id="debuglineoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugLineOffset<T>) -> bool` — [`DebugLineOffset`](../index.md#debuglineoffset)

##### `impl<T> StructuralPartialEq for DebugLineOffset<T>`

### `DebugLineStrOffset<T>`

```rust
struct DebugLineStrOffset<T>(T);
```

An offset into the `.debug_line_str` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-clone"></span>`fn clone(&self) -> DebugLineStrOffset<T>` — [`DebugLineStrOffset`](../index.md#debuglinestroffset)

##### `impl<T: marker::Copy> Copy for DebugLineStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineStrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugLineStrOffset<T>) -> bool` — [`DebugLineStrOffset`](../index.md#debuglinestroffset)

##### `impl<T> StructuralPartialEq for DebugLineStrOffset<T>`

### `LocationListsOffset<T>`

```rust
struct LocationListsOffset<T>(T);
```

An offset into either the `.debug_loc` section or the `.debug_loclists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for LocationListsOffset<T>`

- <span id="locationlistsoffset-clone"></span>`fn clone(&self) -> LocationListsOffset<T>` — [`LocationListsOffset`](../index.md#locationlistsoffset)

##### `impl<T: marker::Copy> Copy for LocationListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for LocationListsOffset<T>`

- <span id="locationlistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for LocationListsOffset<T>`

##### `impl<T: hash::Hash> Hash for LocationListsOffset<T>`

- <span id="locationlistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for LocationListsOffset<T>`

- <span id="locationlistsoffset-partialeq-eq"></span>`fn eq(&self, other: &LocationListsOffset<T>) -> bool` — [`LocationListsOffset`](../index.md#locationlistsoffset)

##### `impl<T> StructuralPartialEq for LocationListsOffset<T>`

### `DebugLocListsBase<T>`

```rust
struct DebugLocListsBase<T>(T);
```

An offset to a set of location list offsets in the `.debug_loclists` section.

#### Implementations

- <span id="cratecommondebugloclistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugLocListsBase<Offset>` — [`Encoding`](../index.md#encoding), [`DwarfFileType`](../index.md#dwarffiletype), [`DebugLocListsBase`](../index.md#debugloclistsbase)

  Returns a `DebugLocListsBase` with the default value of DW_AT_loclists_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLocListsBase<T>`

- <span id="debugloclistsbase-clone"></span>`fn clone(&self) -> DebugLocListsBase<T>` — [`DebugLocListsBase`](../index.md#debugloclistsbase)

##### `impl<T: marker::Copy> Copy for DebugLocListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsBase<T>`

- <span id="debugloclistsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsBase<T>`

- <span id="debugloclistsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugLocListsBase<T>) -> bool` — [`DebugLocListsBase`](../index.md#debugloclistsbase)

##### `impl<T> StructuralPartialEq for DebugLocListsBase<T>`

### `DebugLocListsIndex<T>`

```rust
struct DebugLocListsIndex<T>(T);
```

An index into a set of location list offsets in the `.debug_loclists` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-clone"></span>`fn clone(&self) -> DebugLocListsIndex<T>` — [`DebugLocListsIndex`](../index.md#debugloclistsindex)

##### `impl<T: marker::Copy> Copy for DebugLocListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugLocListsIndex<T>) -> bool` — [`DebugLocListsIndex`](../index.md#debugloclistsindex)

##### `impl<T> StructuralPartialEq for DebugLocListsIndex<T>`

### `DebugMacinfoOffset<T>`

```rust
struct DebugMacinfoOffset<T>(T);
```

An offset into the `.debug_macinfo` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-clone"></span>`fn clone(&self) -> DebugMacinfoOffset<T>` — [`DebugMacinfoOffset`](../index.md#debugmacinfooffset)

##### `impl<T: marker::Copy> Copy for DebugMacinfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacinfoOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugMacinfoOffset<T>) -> bool` — [`DebugMacinfoOffset`](../index.md#debugmacinfooffset)

##### `impl<T> StructuralPartialEq for DebugMacinfoOffset<T>`

### `DebugMacroOffset<T>`

```rust
struct DebugMacroOffset<T>(T);
```

An offset into the `.debug_macro` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugMacroOffset<T>`

- <span id="debugmacrooffset-clone"></span>`fn clone(&self) -> DebugMacroOffset<T>` — [`DebugMacroOffset`](../index.md#debugmacrooffset)

##### `impl<T: marker::Copy> Copy for DebugMacroOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacroOffset<T>`

- <span id="debugmacrooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacroOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugMacroOffset<T>`

- <span id="debugmacrooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacroOffset<T>`

- <span id="debugmacrooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugMacroOffset<T>) -> bool` — [`DebugMacroOffset`](../index.md#debugmacrooffset)

##### `impl<T> StructuralPartialEq for DebugMacroOffset<T>`

### `DebugNamesOffset<T>`

```rust
struct DebugNamesOffset<T>(T);
```

An offset into the `.debug_names` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugNamesOffset<T>`

- <span id="debugnamesoffset-clone"></span>`fn clone(&self) -> DebugNamesOffset<T>` — [`DebugNamesOffset`](../index.md#debugnamesoffset)

##### `impl<T: marker::Copy> Copy for DebugNamesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugNamesOffset<T>`

- <span id="debugnamesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugNamesOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugNamesOffset<T>`

- <span id="debugnamesoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugNamesOffset<T>`

- <span id="debugnamesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugNamesOffset<T>) -> bool` — [`DebugNamesOffset`](../index.md#debugnamesoffset)

##### `impl<T> StructuralPartialEq for DebugNamesOffset<T>`

### `RawRangeListsOffset<T>`

```rust
struct RawRangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

If this is from a DWARF 4 DWO file, then it must additionally be offset by the
value of `DW_AT_GNU_ranges_base`. You can use `Dwarf::ranges_offset_from_raw` to do this.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-clone"></span>`fn clone(&self) -> RawRangeListsOffset<T>` — [`RawRangeListsOffset`](../index.md#rawrangelistsoffset)

##### `impl<T: marker::Copy> Copy for RawRangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RawRangeListsOffset<T>`

##### `impl<T: hash::Hash> Hash for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-partialeq-eq"></span>`fn eq(&self, other: &RawRangeListsOffset<T>) -> bool` — [`RawRangeListsOffset`](../index.md#rawrangelistsoffset)

##### `impl<T> StructuralPartialEq for RawRangeListsOffset<T>`

### `RangeListsOffset<T>`

```rust
struct RangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RangeListsOffset<T>`

- <span id="rangelistsoffset-clone"></span>`fn clone(&self) -> RangeListsOffset<T>` — [`RangeListsOffset`](../index.md#rangelistsoffset)

##### `impl<T: marker::Copy> Copy for RangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RangeListsOffset<T>`

- <span id="rangelistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RangeListsOffset<T>`

##### `impl<T: hash::Hash> Hash for RangeListsOffset<T>`

- <span id="rangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for RangeListsOffset<T>`

- <span id="rangelistsoffset-partialeq-eq"></span>`fn eq(&self, other: &RangeListsOffset<T>) -> bool` — [`RangeListsOffset`](../index.md#rangelistsoffset)

##### `impl<T> StructuralPartialEq for RangeListsOffset<T>`

### `DebugRngListsBase<T>`

```rust
struct DebugRngListsBase<T>(T);
```

An offset to a set of range list offsets in the `.debug_rnglists` section.

#### Implementations

- <span id="cratecommondebugrnglistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugRngListsBase<Offset>` — [`Encoding`](../index.md#encoding), [`DwarfFileType`](../index.md#dwarffiletype), [`DebugRngListsBase`](../index.md#debugrnglistsbase)

  Returns a `DebugRngListsBase` with the default value of DW_AT_rnglists_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-clone"></span>`fn clone(&self) -> DebugRngListsBase<T>` — [`DebugRngListsBase`](../index.md#debugrnglistsbase)

##### `impl<T: marker::Copy> Copy for DebugRngListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugRngListsBase<T>) -> bool` — [`DebugRngListsBase`](../index.md#debugrnglistsbase)

##### `impl<T> StructuralPartialEq for DebugRngListsBase<T>`

### `DebugRngListsIndex<T>`

```rust
struct DebugRngListsIndex<T>(T);
```

An index into a set of range list offsets in the `.debug_rnglists` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-clone"></span>`fn clone(&self) -> DebugRngListsIndex<T>` — [`DebugRngListsIndex`](../index.md#debugrnglistsindex)

##### `impl<T: marker::Copy> Copy for DebugRngListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugRngListsIndex<T>) -> bool` — [`DebugRngListsIndex`](../index.md#debugrnglistsindex)

##### `impl<T> StructuralPartialEq for DebugRngListsIndex<T>`

### `DebugStrOffset<T>`

```rust
struct DebugStrOffset<T>(T);
```

An offset into the `.debug_str` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffset<T>`

- <span id="debugstroffset-clone"></span>`fn clone(&self) -> DebugStrOffset<T>` — [`DebugStrOffset`](../index.md#debugstroffset)

##### `impl<T: marker::Copy> Copy for DebugStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffset<T>`

- <span id="debugstroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffset<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffset<T>`

- <span id="debugstroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffset<T>) -> bool` — [`DebugStrOffset`](../index.md#debugstroffset)

##### `impl<T> StructuralPartialEq for DebugStrOffset<T>`

### `DebugStrOffsetsBase<T>`

```rust
struct DebugStrOffsetsBase<T>(T);
```

An offset to a set of entries in the `.debug_str_offsets` section.

#### Implementations

- <span id="cratecommondebugstroffsetsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugStrOffsetsBase<Offset>` — [`Encoding`](../index.md#encoding), [`DwarfFileType`](../index.md#dwarffiletype), [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase)

  Returns a `DebugStrOffsetsBase` with the default value of DW_AT_str_offsets_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-clone"></span>`fn clone(&self) -> DebugStrOffsetsBase<T>` — [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase)

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsBase<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffsetsBase<T>) -> bool` — [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsBase<T>`

### `DebugStrOffsetsIndex<T>`

```rust
struct DebugStrOffsetsIndex<T>(T);
```

An index into a set of entries in the `.debug_str_offsets` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-clone"></span>`fn clone(&self) -> DebugStrOffsetsIndex<T>` — [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex)

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsIndex<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffsetsIndex<T>) -> bool` — [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsIndex<T>`

### `DebugTypesOffset<T>`

```rust
struct DebugTypesOffset<T>(T);
```

An offset into the `.debug_types` section.

#### Implementations

- <span id="cratecommondebugtypesoffset-to-unit-section-offset"></span>`fn to_unit_section_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitSectionOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`UnitSectionOffset`](../index.md#unitsectionoffset)

  Convert a `.debug_types` offset to be an offset within the section containing the

  given unit.

  

  Returns `None` if the unit is not within the `.debug_types` section.

- <span id="cratecommondebugtypesoffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`UnitOffset`](../index.md#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the `.debug_types` section.

  

  Returns `None` if the offset is not in bounds for the unit's entries.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugTypesOffset<T>`

- <span id="debugtypesoffset-clone"></span>`fn clone(&self) -> DebugTypesOffset<T>` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T: marker::Copy> Copy for DebugTypesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugTypesOffset<T>`

- <span id="debugtypesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugTypesOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugTypesOffset<T>`

- <span id="debugtypesoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for DebugTypesOffset<T>`

- <span id="debugtypesoffset-ord-cmp"></span>`fn cmp(&self, other: &DebugTypesOffset<T>) -> cmp::Ordering` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugTypesOffset<T>`

- <span id="debugtypesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugTypesOffset<T>) -> bool` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugTypesOffset<T>`

- <span id="debugtypesoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugTypesOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T> StructuralPartialEq for DebugTypesOffset<T>`

### `DebugTypeSignature`

```rust
struct DebugTypeSignature(u64);
```

A type signature as used in the `.debug_types` section.

#### Trait Implementations

##### `impl Clone for DebugTypeSignature`

- <span id="debugtypesignature-clone"></span>`fn clone(&self) -> DebugTypeSignature` — [`DebugTypeSignature`](../index.md#debugtypesignature)

##### `impl Copy for DebugTypeSignature`

##### `impl Debug for DebugTypeSignature`

- <span id="debugtypesignature-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DebugTypeSignature`

##### `impl Hash for DebugTypeSignature`

- <span id="debugtypesignature-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DebugTypeSignature`

- <span id="debugtypesignature-partialeq-eq"></span>`fn eq(&self, other: &DebugTypeSignature) -> bool` — [`DebugTypeSignature`](../index.md#debugtypesignature)

##### `impl StructuralPartialEq for DebugTypeSignature`

### `DebugFrameOffset<T>`

```rust
struct DebugFrameOffset<T>(T);
```

An offset into the `.debug_frame` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DebugFrameOffset<T>`

- <span id="debugframeoffset-clone"></span>`fn clone(&self) -> DebugFrameOffset<T>` — [`DebugFrameOffset`](../index.md#debugframeoffset)

##### `impl<T: marker::Copy> Copy for DebugFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugFrameOffset<T>`

- <span id="debugframeoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugFrameOffset<T>`

##### `impl<T: hash::Hash> Hash for DebugFrameOffset<T>`

- <span id="debugframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for DebugFrameOffset<T>`

- <span id="debugframeoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugFrameOffset<T>) -> bool` — [`DebugFrameOffset`](../index.md#debugframeoffset)

##### `impl<T> StructuralPartialEq for DebugFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::DebugFrameOffset<T>`

- <span id="cratecommondebugframeoffset-unwindoffset-into"></span>`fn into(self) -> T`

### `EhFrameOffset<T>`

```rust
struct EhFrameOffset<T>(T);
```

An offset into the `.eh_frame` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for EhFrameOffset<T>`

- <span id="ehframeoffset-clone"></span>`fn clone(&self) -> EhFrameOffset<T>` — [`EhFrameOffset`](../index.md#ehframeoffset)

##### `impl<T: marker::Copy> Copy for EhFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for EhFrameOffset<T>`

- <span id="ehframeoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for EhFrameOffset<T>`

##### `impl<T: hash::Hash> Hash for EhFrameOffset<T>`

- <span id="ehframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for EhFrameOffset<T>`

- <span id="ehframeoffset-partialeq-eq"></span>`fn eq(&self, other: &EhFrameOffset<T>) -> bool` — [`EhFrameOffset`](../index.md#ehframeoffset)

##### `impl<T> StructuralPartialEq for EhFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::EhFrameOffset<T>`

- <span id="cratecommonehframeoffset-unwindoffset-into"></span>`fn into(self) -> T`

### `UnitSectionOffset<T>`

```rust
struct UnitSectionOffset<T>(T);
```

An offset into the `.debug_info` or `.debug_types` sections.

This type does not store which section the offset applies to. You will need to either
determine that from the context of its use, or store a [`SectionId`](../index.md) along with it.

#### Implementations

- <span id="cratecommonunitsectionoffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`UnitOffset`](../index.md#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the section.

  

  Returns `None` if the offset is not in bounds for the unit's entries.

- <span id="cratecommonunitsectionoffset-to-debug-info-offset"></span>`fn to_debug_info_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugInfoOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`DebugInfoOffset`](../index.md#debuginfooffset)

  Convert an offset to be relative to the start of the `.debug_info` section,

  instead of relative to the start of the section for the given unit.

  

  Returns `None` if the unit is not within the `.debug_info` section.

- <span id="cratecommonunitsectionoffset-to-debug-types-offset"></span>`fn to_debug_types_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugTypesOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`DebugTypesOffset`](../index.md#debugtypesoffset)

  Convert an offset to be relative to the start of the `.debug_types` section,

  instead of relative to the start of the section for the given unit.

  

  Returns `None` if the unit is not within the `.debug_types` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for UnitSectionOffset<T>`

- <span id="unitsectionoffset-clone"></span>`fn clone(&self) -> UnitSectionOffset<T>` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T: marker::Copy> Copy for UnitSectionOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitSectionOffset<T>`

- <span id="unitsectionoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitSectionOffset<T>`

##### `impl<T: hash::Hash> Hash for UnitSectionOffset<T>`

- <span id="unitsectionoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for UnitSectionOffset<T>`

- <span id="unitsectionoffset-ord-cmp"></span>`fn cmp(&self, other: &UnitSectionOffset<T>) -> cmp::Ordering` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitSectionOffset<T>`

- <span id="unitsectionoffset-partialeq-eq"></span>`fn eq(&self, other: &UnitSectionOffset<T>) -> bool` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitSectionOffset<T>`

- <span id="unitsectionoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitSectionOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T> StructuralPartialEq for UnitSectionOffset<T>`

### `DwoId`

```rust
struct DwoId(u64);
```

An optionally-provided implementation-defined compilation unit ID to enable
split DWARF and linking a split compilation unit back together.

#### Trait Implementations

##### `impl Clone for DwoId`

- <span id="dwoid-clone"></span>`fn clone(&self) -> DwoId` — [`DwoId`](../index.md#dwoid)

##### `impl Copy for DwoId`

##### `impl Debug for DwoId`

- <span id="dwoid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DwoId`

##### `impl Hash for DwoId`

- <span id="dwoid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for DwoId`

- <span id="dwoid-partialeq-eq"></span>`fn eq(&self, other: &DwoId) -> bool` — [`DwoId`](../index.md#dwoid)

##### `impl StructuralPartialEq for DwoId`

## Enums

### `Format`

```rust
enum Format {
    Dwarf64,
    Dwarf32,
}
```

Whether the format of a compilation unit is 32- or 64-bit.

#### Variants

- **`Dwarf64`**

  64-bit DWARF

- **`Dwarf32`**

  32-bit DWARF

#### Implementations

- <span id="format-initial-length-size"></span>`fn initial_length_size(self) -> u8`

  Return the serialized size of an initial length field for the format.

- <span id="format-word-size"></span>`fn word_size(self) -> u8`

  Return the natural word size for the format

#### Trait Implementations

##### `impl Clone for Format`

- <span id="format-clone"></span>`fn clone(&self) -> Format` — [`Format`](../index.md#format)

##### `impl Copy for Format`

##### `impl Debug for Format`

- <span id="format-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Format`

##### `impl Hash for Format`

- <span id="format-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Format`

- <span id="format-partialeq-eq"></span>`fn eq(&self, other: &Format) -> bool` — [`Format`](../index.md#format)

##### `impl StructuralPartialEq for Format`

### `Vendor`

```rust
enum Vendor {
    Default,
    AArch64,
}
```

Which vendor extensions to support.

#### Variants

- **`Default`**

  A default set of extensions, including some common GNU extensions.

- **`AArch64`**

  AAarch64 extensions.

#### Trait Implementations

##### `impl Clone for Vendor`

- <span id="vendor-clone"></span>`fn clone(&self) -> Vendor` — [`Vendor`](../index.md#vendor)

##### `impl Copy for Vendor`

##### `impl Debug for Vendor`

- <span id="vendor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Vendor`

##### `impl PartialEq for Vendor`

- <span id="vendor-partialeq-eq"></span>`fn eq(&self, other: &Vendor) -> bool` — [`Vendor`](../index.md#vendor)

##### `impl StructuralPartialEq for Vendor`

### `SectionId`

```rust
enum SectionId {
    DebugAbbrev,
    DebugAddr,
    DebugAranges,
    DebugCuIndex,
    DebugFrame,
    EhFrame,
    EhFrameHdr,
    DebugInfo,
    DebugLine,
    DebugLineStr,
    DebugLoc,
    DebugLocLists,
    DebugMacinfo,
    DebugMacro,
    DebugNames,
    DebugPubNames,
    DebugPubTypes,
    DebugRanges,
    DebugRngLists,
    DebugStr,
    DebugStrOffsets,
    DebugTuIndex,
    DebugTypes,
}
```

An identifier for a DWARF section.

#### Variants

- **`DebugAbbrev`**

  The `.debug_abbrev` section.

- **`DebugAddr`**

  The `.debug_addr` section.

- **`DebugAranges`**

  The `.debug_aranges` section.

- **`DebugCuIndex`**

  The `.debug_cu_index` section.

- **`DebugFrame`**

  The `.debug_frame` section.

- **`EhFrame`**

  The `.eh_frame` section.

- **`EhFrameHdr`**

  The `.eh_frame_hdr` section.

- **`DebugInfo`**

  The `.debug_info` section.

- **`DebugLine`**

  The `.debug_line` section.

- **`DebugLineStr`**

  The `.debug_line_str` section.

- **`DebugLoc`**

  The `.debug_loc` section.

- **`DebugLocLists`**

  The `.debug_loclists` section.

- **`DebugMacinfo`**

  The `.debug_macinfo` section.

- **`DebugMacro`**

  The `.debug_macro` section.

- **`DebugNames`**

  The `.debug_names` section.

- **`DebugPubNames`**

  The `.debug_pubnames` section.

- **`DebugPubTypes`**

  The `.debug_pubtypes` section.

- **`DebugRanges`**

  The `.debug_ranges` section.

- **`DebugRngLists`**

  The `.debug_rnglists` section.

- **`DebugStr`**

  The `.debug_str` section.

- **`DebugStrOffsets`**

  The `.debug_str_offsets` section.

- **`DebugTuIndex`**

  The `.debug_tu_index` section.

- **`DebugTypes`**

  The `.debug_types` section.

#### Implementations

- <span id="sectionid-name"></span>`fn name(self) -> &'static str`

  Returns the ELF section name for this kind.

- <span id="sectionid-dwo-name"></span>`fn dwo_name(self) -> Option<&'static str>`

  Returns the ELF section name for this kind, when found in a .dwo or .dwp file.

- <span id="sectionid-xcoff-name"></span>`fn xcoff_name(self) -> Option<&'static str>`

  Returns the XCOFF section name for this kind.

- <span id="sectionid-is-string"></span>`fn is_string(self) -> bool`

  Returns true if this is a mergeable string section.

  

  This is useful for determining the correct section flags.

#### Trait Implementations

##### `impl Clone for SectionId`

- <span id="sectionid-clone"></span>`fn clone(&self) -> SectionId` — [`SectionId`](../index.md#sectionid)

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- <span id="sectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionId`

##### `impl Hash for SectionId`

- <span id="sectionid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SectionId`

- <span id="sectionid-ord-cmp"></span>`fn cmp(&self, other: &SectionId) -> cmp::Ordering` — [`SectionId`](../index.md#sectionid)

##### `impl PartialEq for SectionId`

- <span id="sectionid-partialeq-eq"></span>`fn eq(&self, other: &SectionId) -> bool` — [`SectionId`](../index.md#sectionid)

##### `impl PartialOrd for SectionId`

- <span id="sectionid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SectionId) -> option::Option<cmp::Ordering>` — [`SectionId`](../index.md#sectionid)

##### `impl StructuralPartialEq for SectionId`

### `DwarfFileType`

```rust
enum DwarfFileType {
    Main,
    Dwo,
}
```

The "type" of file with DWARF debugging information. This determines, among other things,
which files DWARF sections should be loaded from.

#### Variants

- **`Main`**

  A normal executable or object file.

- **`Dwo`**

  A .dwo split DWARF file.

#### Trait Implementations

##### `impl Clone for DwarfFileType`

- <span id="dwarffiletype-clone"></span>`fn clone(&self) -> DwarfFileType` — [`DwarfFileType`](../index.md#dwarffiletype)

##### `impl Copy for DwarfFileType`

##### `impl Debug for DwarfFileType`

- <span id="dwarffiletype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DwarfFileType`

- <span id="dwarffiletype-default"></span>`fn default() -> DwarfFileType` — [`DwarfFileType`](../index.md#dwarffiletype)

##### `impl Eq for DwarfFileType`

##### `impl PartialEq for DwarfFileType`

- <span id="dwarffiletype-partialeq-eq"></span>`fn eq(&self, other: &DwarfFileType) -> bool` — [`DwarfFileType`](../index.md#dwarffiletype)

##### `impl StructuralPartialEq for DwarfFileType`

