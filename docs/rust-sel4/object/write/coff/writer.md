**object > write > coff > writer**

# Module: write::coff::writer

## Contents

**Structs**

- [`AuxSymbolSection`](#auxsymbolsection) - Native endian version of [`pe::ImageAuxSymbolSection`].
- [`AuxSymbolWeak`](#auxsymbolweak) - Native endian version of [`pe::ImageAuxSymbolWeak`].
- [`FileHeader`](#fileheader) - Shortened and native endian version of [`pe::ImageFileHeader`].
- [`Relocation`](#relocation) - Native endian version of [`pe::ImageRelocation`].
- [`SectionHeader`](#sectionheader) - Native endian version of [`pe::ImageSectionHeader`].
- [`Symbol`](#symbol) - Native endian version of [`pe::ImageSymbol`].
- [`Writer`](#writer) - A helper for writing COFF files.

**Enums**

- [`Name`](#name) - A section or symbol name.

---

## object::write::coff::writer::AuxSymbolSection

*Struct*

Native endian version of [`pe::ImageAuxSymbolSection`].

**Fields:**
- `length: u32`
- `number_of_relocations: u32` - This will automatically be clamped if there are more than 0xffff.
- `number_of_linenumbers: u16`
- `check_sum: u32`
- `number: u32`
- `selection: u8`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AuxSymbolSection`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> AuxSymbolSection`



## object::write::coff::writer::AuxSymbolWeak

*Struct*

Native endian version of [`pe::ImageAuxSymbolWeak`].

**Fields:**
- `weak_default_sym_index: u32`
- `weak_search_type: u32`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AuxSymbolWeak`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> AuxSymbolWeak`



## object::write::coff::writer::FileHeader

*Struct*

Shortened and native endian version of [`pe::ImageFileHeader`].

**Fields:**
- `machine: u16`
- `time_date_stamp: u32`
- `characteristics: u16`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> FileHeader`
- **Clone**
  - `fn clone(self: &Self) -> FileHeader`



## object::write::coff::writer::Name

*Enum*

A section or symbol name.

**Variants:**
- `Short([u8; 8])` - An inline name.
- `Long(crate::write::string::StringId)` - An id of a string table entry.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Name`
- **Default**
  - `fn default() -> Name`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::coff::writer::Relocation

*Struct*

Native endian version of [`pe::ImageRelocation`].

**Fields:**
- `virtual_address: u32`
- `symbol: u32`
- `typ: u16`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Relocation`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Relocation`



## object::write::coff::writer::SectionHeader

*Struct*

Native endian version of [`pe::ImageSectionHeader`].

**Fields:**
- `name: Name`
- `size_of_raw_data: u32`
- `pointer_to_raw_data: u32`
- `pointer_to_relocations: u32`
- `pointer_to_linenumbers: u32`
- `number_of_relocations: u32` - This will automatically be clamped if there are more than 0xffff.
- `number_of_linenumbers: u16`
- `characteristics: u32`

**Trait Implementations:**

- **Default**
  - `fn default() -> SectionHeader`
- **Clone**
  - `fn clone(self: &Self) -> SectionHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::coff::writer::Symbol

*Struct*

Native endian version of [`pe::ImageSymbol`].

**Fields:**
- `name: Name`
- `value: u32`
- `section_number: u16`
- `typ: u16`
- `storage_class: u8`
- `number_of_aux_symbols: u8`

**Trait Implementations:**

- **Default**
  - `fn default() -> Symbol`
- **Clone**
  - `fn clone(self: &Self) -> Symbol`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::write::coff::writer::Writer

*Struct*

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

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buffer: &'a  mut dyn WritableBuffer) -> Self` - Create a new `Writer`.
- `fn reserved_len(self: &Self) -> usize` - Return the current file length that has been reserved.
- `fn len(self: &Self) -> usize` - Return the current file length that has been written.
- `fn reserve(self: & mut Self, len: usize, align_start: usize) -> u32` - Reserve a file range with the given size and starting alignment.
- `fn write_align(self: & mut Self, align_start: usize)` - Write alignment padding bytes.
- `fn write(self: & mut Self, data: &[u8])` - Write data.
- `fn reserve_until(self: & mut Self, offset: usize)` - Reserve the file range up to the given file offset.
- `fn pad_until(self: & mut Self, offset: usize)` - Write padding up to the given file offset.
- `fn reserve_file_header(self: & mut Self)` - Reserve the range for the file header.
- `fn write_file_header(self: & mut Self, header: FileHeader) -> Result<()>` - Write the file header.
- `fn reserve_section_headers(self: & mut Self, section_num: u16)` - Reserve the range for the section headers.
- `fn write_section_header(self: & mut Self, section: SectionHeader)` - Write a section header.
- `fn reserve_section(self: & mut Self, len: usize) -> u32` - Reserve the range for the section data.
- `fn write_section_align(self: & mut Self)` - Write the alignment bytes prior to section data.
- `fn write_section(self: & mut Self, data: &[u8])` - Write the section data.
- `fn write_section_zeroes(self: & mut Self, len: usize)` - Write the section data using zero bytes.
- `fn reserve_relocations(self: & mut Self, count: usize) -> u32` - Reserve a file range for the given number of relocations.
- `fn write_relocations_count(self: & mut Self, count: usize)` - Write a relocation containing the count if required.
- `fn write_relocation(self: & mut Self, reloc: Relocation)` - Write a relocation.
- `fn reserve_symbol_index(self: & mut Self) -> u32` - Reserve a symbol table entry.
- `fn reserve_symbol_indices(self: & mut Self, count: u32)` - Reserve a number of symbol table entries.
- `fn write_symbol(self: & mut Self, symbol: Symbol)` - Write a symbol table entry.
- `fn reserve_aux_file_name(self: & mut Self, name: &[u8]) -> u8` - Reserve auxiliary symbols for a file name.
- `fn write_aux_file_name(self: & mut Self, name: &[u8], aux_count: u8)` - Write auxiliary symbols for a file name.
- `fn reserve_aux_section(self: & mut Self) -> u8` - Reserve an auxiliary symbol for a section.
- `fn write_aux_section(self: & mut Self, section: AuxSymbolSection)` - Write an auxiliary symbol for a section.
- `fn reserve_aux_weak_external(self: & mut Self) -> u8` - Reserve an auxiliary symbol for a weak external.
- `fn write_aux_weak_external(self: & mut Self, weak: AuxSymbolWeak)` - Write an auxiliary symbol for a weak external.
- `fn symbol_count(self: &Self) -> u32` - Return the number of reserved symbol table entries.
- `fn add_string(self: & mut Self, name: &'a [u8]) -> StringId` - Add a string to the string table.
- `fn add_name(self: & mut Self, name: &'a [u8]) -> Name` - Add a section or symbol name to the string table if required.
- `fn reserve_symtab_strtab(self: & mut Self)` - Reserve the range for the symbol table and string table.
- `fn write_strtab(self: & mut Self)` - Write the string table.



