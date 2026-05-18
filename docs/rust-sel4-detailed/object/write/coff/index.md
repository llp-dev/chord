*[object](../../index.md) / [write](../index.md) / [coff](index.md)*

---

# Module `coff`

Support for writing COFF files.

Provides [`Writer`](#writer) for low level writing of COFF files.
This is also used to provide COFF support for [`write::Object`](crate::write::Object).

## Contents

- [Modules](#modules)
  - [`object`](#object)
  - [`writer`](#writer)
- [Structs](#structs)
  - [`SectionOffsets`](#sectionoffsets)
  - [`SymbolOffsets`](#symboloffsets)
  - [`Writer`](#writer)
  - [`FileHeader`](#fileheader)
  - [`SectionHeader`](#sectionheader)
  - [`Symbol`](#symbol)
  - [`AuxSymbolSection`](#auxsymbolsection)
  - [`AuxSymbolWeak`](#auxsymbolweak)
  - [`Relocation`](#relocation)
- [Enums](#enums)
  - [`CoffExportStyle`](#coffexportstyle)
  - [`Name`](#name)
- [Functions](#functions)
  - [`checksum`](#checksum)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`object`](#object) | mod |  |
| [`writer`](#writer) | mod | Helper for writing COFF files. |
| [`SectionOffsets`](#sectionoffsets) | struct |  |
| [`SymbolOffsets`](#symboloffsets) | struct |  |
| [`Writer`](#writer) | struct | A helper for writing COFF files. |
| [`FileHeader`](#fileheader) | struct | Shortened and native endian version of [`pe::ImageFileHeader`]. |
| [`SectionHeader`](#sectionheader) | struct | Native endian version of [`pe::ImageSectionHeader`]. |
| [`Symbol`](#symbol) | struct | Native endian version of [`pe::ImageSymbol`]. |
| [`AuxSymbolSection`](#auxsymbolsection) | struct | Native endian version of [`pe::ImageAuxSymbolSection`]. |
| [`AuxSymbolWeak`](#auxsymbolweak) | struct | Native endian version of [`pe::ImageAuxSymbolWeak`]. |
| [`Relocation`](#relocation) | struct | Native endian version of [`pe::ImageRelocation`]. |
| [`CoffExportStyle`](#coffexportstyle) | enum | Internal format to use for the `.drectve` section containing linker directives for symbol exports. |
| [`Name`](#name) | enum | A section or symbol name. |
| [`checksum`](#checksum) | fn |  |

## Modules

- [`object`](object/index.md)
- [`writer`](writer/index.md) — Helper for writing COFF files.

## Structs

### `SectionOffsets`

```rust
struct SectionOffsets {
    name: writer::Name,
    offset: u32,
    reloc_offset: u32,
    selection: u8,
    associative_section: u32,
}
```

#### Trait Implementations

##### `impl Clone for SectionOffsets`

- <span id="sectionoffsets-clone"></span>`fn clone(&self) -> SectionOffsets` — [`SectionOffsets`](object/index.md#sectionoffsets)

##### `impl Copy for SectionOffsets`

##### `impl Default for SectionOffsets`

- <span id="sectionoffsets-default"></span>`fn default() -> SectionOffsets` — [`SectionOffsets`](object/index.md#sectionoffsets)

### `SymbolOffsets`

```rust
struct SymbolOffsets {
    name: writer::Name,
    index: u32,
    aux_count: u8,
}
```

#### Trait Implementations

##### `impl Clone for SymbolOffsets`

- <span id="symboloffsets-clone"></span>`fn clone(&self) -> SymbolOffsets` — [`SymbolOffsets`](object/index.md#symboloffsets)

##### `impl Copy for SymbolOffsets`

##### `impl Default for SymbolOffsets`

- <span id="symboloffsets-default"></span>`fn default() -> SymbolOffsets` — [`SymbolOffsets`](object/index.md#symboloffsets)

### `Writer<'a>`

```rust
struct Writer<'a> {
    buffer: &'a mut dyn WritableBuffer,
    len: usize,
    section_num: u16,
    symtab_offset: u32,
    symtab_num: u32,
    strtab: crate::write::string::StringTable<'a>,
    strtab_len: usize,
    strtab_offset: u32,
    strtab_data: alloc::vec::Vec<u8>,
}
```

A helper for writing COFF files.

Writing uses a two phase approach. The first phase builds up all of the information
that may need to be known ahead of time:
- build string table
- reserve section indices
- reserve symbol indices
- reserve file ranges for headers and sections

Some of the information has ordering requirements. For example, strings must be added
to the string table before reserving the file range for the string table. There are debug
asserts to check some of these requirements.

The second phase writes everything out in order. Thus the caller must ensure writing
is in the same order that file ranges were reserved. There are debug asserts to assist
with checking this.

#### Implementations

- <span id="writer-new"></span>`fn new(buffer: &'a mut dyn WritableBuffer) -> Self` — [`WritableBuffer`](../index.md#writablebuffer)

  Create a new `Writer`.

- <span id="writer-reserved-len"></span>`fn reserved_len(&self) -> usize`

  Return the current file length that has been reserved.

- <span id="writer-len"></span>`fn len(&self) -> usize`

  Return the current file length that has been written.

- <span id="writer-reserve"></span>`fn reserve(&mut self, len: usize, align_start: usize) -> u32`

  Reserve a file range with the given size and starting alignment.

  

  Returns the aligned offset of the start of the range.

  

  `align_start` must be a power of two.

- <span id="writer-write-align"></span>`fn write_align(&mut self, align_start: usize)`

  Write alignment padding bytes.

- <span id="writer-write"></span>`fn write(&mut self, data: &[u8])`

  Write data.

- <span id="writer-reserve-until"></span>`fn reserve_until(&mut self, offset: usize)`

  Reserve the file range up to the given file offset.

- <span id="writer-pad-until"></span>`fn pad_until(&mut self, offset: usize)`

  Write padding up to the given file offset.

- <span id="writer-reserve-file-header"></span>`fn reserve_file_header(&mut self)`

  Reserve the range for the file header.

  

  This must be at the start of the file.

- <span id="writer-write-file-header"></span>`fn write_file_header(&mut self, header: FileHeader) -> Result<()>` — [`FileHeader`](#fileheader), [`Result`](../index.md#result)

  Write the file header.

  

  This must be at the start of the file.

  

  Fields that can be derived from known information are automatically set by this function.

- <span id="writer-reserve-section-headers"></span>`fn reserve_section_headers(&mut self, section_num: u16)`

  Reserve the range for the section headers.

- <span id="writer-write-section-header"></span>`fn write_section_header(&mut self, section: SectionHeader)` — [`SectionHeader`](#sectionheader)

  Write a section header.

- <span id="writer-reserve-section"></span>`fn reserve_section(&mut self, len: usize) -> u32`

  Reserve the range for the section data.

  

  Returns the aligned offset of the start of the range.

  Does nothing and returns 0 if the length is zero.

- <span id="writer-write-section-align"></span>`fn write_section_align(&mut self)`

  Write the alignment bytes prior to section data.

  

  This is unneeded if you are using `write_section` or `write_section_zeroes`

  for the data.

- <span id="writer-write-section"></span>`fn write_section(&mut self, data: &[u8])`

  Write the section data.

  

  Writes alignment bytes prior to the data.

  Does nothing if the data is empty.

- <span id="writer-write-section-zeroes"></span>`fn write_section_zeroes(&mut self, len: usize)`

  Write the section data using zero bytes.

  

  Writes alignment bytes prior to the data.

  Does nothing if the length is zero.

- <span id="writer-reserve-relocations"></span>`fn reserve_relocations(&mut self, count: usize) -> u32`

  Reserve a file range for the given number of relocations.

  

  This will automatically reserve an extra relocation if there are more than 0xffff.

  

  Returns the offset of the range.

  Does nothing and returns 0 if the count is zero.

- <span id="writer-write-relocations-count"></span>`fn write_relocations_count(&mut self, count: usize)`

  Write a relocation containing the count if required.

  

  This should be called before writing the first relocation for a section.

- <span id="writer-write-relocation"></span>`fn write_relocation(&mut self, reloc: Relocation)` — [`Relocation`](#relocation)

  Write a relocation.

- <span id="writer-reserve-symbol-index"></span>`fn reserve_symbol_index(&mut self) -> u32`

  Reserve a symbol table entry.

  

  This must be called before `Self::reserve_symtab_strtab`.

- <span id="writer-reserve-symbol-indices"></span>`fn reserve_symbol_indices(&mut self, count: u32)`

  Reserve a number of symbol table entries.

- <span id="writer-write-symbol"></span>`fn write_symbol(&mut self, symbol: Symbol)` — [`Symbol`](#symbol)

  Write a symbol table entry.

- <span id="writer-reserve-aux-file-name"></span>`fn reserve_aux_file_name(&mut self, name: &[u8]) -> u8`

  Reserve auxiliary symbols for a file name.

  

  Returns the number of auxiliary symbols required.

  

  This must be called before `Self::reserve_symtab_strtab`.

- <span id="writer-write-aux-file-name"></span>`fn write_aux_file_name(&mut self, name: &[u8], aux_count: u8)`

  Write auxiliary symbols for a file name.

- <span id="writer-reserve-aux-section"></span>`fn reserve_aux_section(&mut self) -> u8`

  Reserve an auxiliary symbol for a section.

  

  Returns the number of auxiliary symbols required.

  

  This must be called before `Self::reserve_symtab_strtab`.

- <span id="writer-write-aux-section"></span>`fn write_aux_section(&mut self, section: AuxSymbolSection)` — [`AuxSymbolSection`](#auxsymbolsection)

  Write an auxiliary symbol for a section.

- <span id="writer-reserve-aux-weak-external"></span>`fn reserve_aux_weak_external(&mut self) -> u8`

  Reserve an auxiliary symbol for a weak external.

  

  Returns the number of auxiliary symbols required.

  

  This must be called before `Self::reserve_symtab_strtab`.

- <span id="writer-write-aux-weak-external"></span>`fn write_aux_weak_external(&mut self, weak: AuxSymbolWeak)` — [`AuxSymbolWeak`](#auxsymbolweak)

  Write an auxiliary symbol for a weak external.

- <span id="writer-symbol-count"></span>`fn symbol_count(&self) -> u32`

  Return the number of reserved symbol table entries.

- <span id="writer-add-string"></span>`fn add_string(&mut self, name: &'a [u8]) -> StringId` — [`StringId`](../string/index.md#stringid)

  Add a string to the string table.

  

  This must be called before `Self::reserve_symtab_strtab`.

- <span id="writer-add-name"></span>`fn add_name(&mut self, name: &'a [u8]) -> Name` — [`Name`](#name)

  Add a section or symbol name to the string table if required.

  

  This must be called before `Self::reserve_symtab_strtab`.

- <span id="writer-reserve-symtab-strtab"></span>`fn reserve_symtab_strtab(&mut self)`

  Reserve the range for the symbol table and string table.

  

  This must be called after functions that reserve symbol

  indices or add strings.

- <span id="writer-write-strtab"></span>`fn write_strtab(&mut self)`

  Write the string table.

### `FileHeader`

```rust
struct FileHeader {
    pub machine: u16,
    pub time_date_stamp: u32,
    pub characteristics: u16,
}
```

Shortened and native endian version of [`pe::ImageFileHeader`](../../pe/index.md).

#### Trait Implementations

##### `impl Clone for FileHeader`

- <span id="fileheader-clone"></span>`fn clone(&self) -> FileHeader` — [`FileHeader`](#fileheader)

##### `impl Debug for FileHeader`

- <span id="fileheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FileHeader`

- <span id="fileheader-default"></span>`fn default() -> FileHeader` — [`FileHeader`](#fileheader)

### `SectionHeader`

```rust
struct SectionHeader {
    pub name: Name,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u32,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}
```

Native endian version of [`pe::ImageSectionHeader`](../../pe/index.md).

#### Fields

- **`number_of_relocations`**: `u32`

  This will automatically be clamped if there are more than 0xffff.

#### Trait Implementations

##### `impl Clone for SectionHeader`

- <span id="sectionheader-clone"></span>`fn clone(&self) -> SectionHeader` — [`SectionHeader`](#sectionheader)

##### `impl Debug for SectionHeader`

- <span id="sectionheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionHeader`

- <span id="sectionheader-default"></span>`fn default() -> SectionHeader` — [`SectionHeader`](#sectionheader)

### `Symbol`

```rust
struct Symbol {
    pub name: Name,
    pub value: u32,
    pub section_number: u16,
    pub typ: u16,
    pub storage_class: u8,
    pub number_of_aux_symbols: u8,
}
```

Native endian version of [`pe::ImageSymbol`](../../pe/index.md).

#### Trait Implementations

##### `impl Clone for Symbol`

- <span id="symbol-clone"></span>`fn clone(&self) -> Symbol` — [`Symbol`](#symbol)

##### `impl Debug for Symbol`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Symbol`

- <span id="symbol-default"></span>`fn default() -> Symbol` — [`Symbol`](#symbol)

### `AuxSymbolSection`

```rust
struct AuxSymbolSection {
    pub length: u32,
    pub number_of_relocations: u32,
    pub number_of_linenumbers: u16,
    pub check_sum: u32,
    pub number: u32,
    pub selection: u8,
}
```

Native endian version of [`pe::ImageAuxSymbolSection`](../../pe/index.md).

#### Fields

- **`number_of_relocations`**: `u32`

  This will automatically be clamped if there are more than 0xffff.

#### Trait Implementations

##### `impl Clone for AuxSymbolSection`

- <span id="auxsymbolsection-clone"></span>`fn clone(&self) -> AuxSymbolSection` — [`AuxSymbolSection`](#auxsymbolsection)

##### `impl Debug for AuxSymbolSection`

- <span id="auxsymbolsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AuxSymbolSection`

- <span id="auxsymbolsection-default"></span>`fn default() -> AuxSymbolSection` — [`AuxSymbolSection`](#auxsymbolsection)

### `AuxSymbolWeak`

```rust
struct AuxSymbolWeak {
    pub weak_default_sym_index: u32,
    pub weak_search_type: u32,
}
```

Native endian version of [`pe::ImageAuxSymbolWeak`](../../pe/index.md).

#### Trait Implementations

##### `impl Clone for AuxSymbolWeak`

- <span id="auxsymbolweak-clone"></span>`fn clone(&self) -> AuxSymbolWeak` — [`AuxSymbolWeak`](#auxsymbolweak)

##### `impl Debug for AuxSymbolWeak`

- <span id="auxsymbolweak-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AuxSymbolWeak`

- <span id="auxsymbolweak-default"></span>`fn default() -> AuxSymbolWeak` — [`AuxSymbolWeak`](#auxsymbolweak)

### `Relocation`

```rust
struct Relocation {
    pub virtual_address: u32,
    pub symbol: u32,
    pub typ: u16,
}
```

Native endian version of [`pe::ImageRelocation`](../../pe/index.md).

#### Trait Implementations

##### `impl Clone for Relocation`

- <span id="relocation-clone"></span>`fn clone(&self) -> Relocation` — [`Relocation`](#relocation)

##### `impl Debug for Relocation`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Relocation`

- <span id="relocation-default"></span>`fn default() -> Relocation` — [`Relocation`](#relocation)

## Enums

### `CoffExportStyle`

```rust
enum CoffExportStyle {
    Msvc,
    Gnu,
}
```

Internal format to use for the `.drectve` section containing linker
directives for symbol exports.

#### Variants

- **`Msvc`**

  MSVC format supported by link.exe and LLD.

- **`Gnu`**

  Gnu format supported by GNU LD and LLD.

#### Trait Implementations

##### `impl Clone for CoffExportStyle`

- <span id="coffexportstyle-clone"></span>`fn clone(&self) -> CoffExportStyle` — [`CoffExportStyle`](#coffexportstyle)

##### `impl Copy for CoffExportStyle`

##### `impl Debug for CoffExportStyle`

- <span id="coffexportstyle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CoffExportStyle`

##### `impl<K> Equivalent for CoffExportStyle`

- <span id="coffexportstyle-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for CoffExportStyle`

- <span id="coffexportstyle-partialeq-eq"></span>`fn eq(&self, other: &CoffExportStyle) -> bool` — [`CoffExportStyle`](#coffexportstyle)

##### `impl StructuralPartialEq for CoffExportStyle`

### `Name`

```rust
enum Name {
    Short([u8; 8]),
    Long(crate::write::string::StringId),
}
```

A section or symbol name.

#### Variants

- **`Short`**

  An inline name.

- **`Long`**

  An id of a string table entry.

#### Trait Implementations

##### `impl Clone for Name`

- <span id="name-clone"></span>`fn clone(&self) -> Name` — [`Name`](#name)

##### `impl Copy for Name`

##### `impl Debug for Name`

- <span id="name-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Name`

- <span id="name-default"></span>`fn default() -> Name` — [`Name`](#name)

## Functions

### `checksum`

```rust
fn checksum(data: &[u8]) -> u32
```

