**object > read > macho > section**

# Module: read::macho::section

## Contents

**Structs**

- [`MachOSection`](#machosection) - A section in a [`MachOFile`].
- [`MachOSectionIterator`](#machosectioniterator) - An iterator for the sections in a [`MachOFile`].

**Traits**

- [`Section`](#section) - A trait for generic access to [`macho::Section32`] and [`macho::Section64`].

**Type Aliases**

- [`MachOSection32`](#machosection32) - A section in a [`MachOFile32`](super::MachOFile32).
- [`MachOSection64`](#machosection64) - A section in a [`MachOFile64`](super::MachOFile64).
- [`MachOSectionIterator32`](#machosectioniterator32) - An iterator for the sections in a [`MachOFile32`](super::MachOFile32).
- [`MachOSectionIterator64`](#machosectioniterator64) - An iterator for the sections in a [`MachOFile64`](super::MachOFile64).

---

## object::read::macho::section::MachOSection

*Struct*

A section in a [`MachOFile`].

Most functionality is provided by the [`ObjectSection`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Methods:**

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` - Get the Mach-O file containing this section.
- `fn macho_section(self: &Self) -> &'data <Mach as >::Section` - Get the raw Mach-O section structure.
- `fn macho_relocations(self: &Self) -> Result<&'data [macho::Relocation<<Mach as >::Endian>]>` - Get the raw Mach-O relocation entries.

**Trait Implementations:**

- **ObjectSection**
  - `fn index(self: &Self) -> SectionIndex`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> Option<(u64, u64)>`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`
  - `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>`
  - `fn compressed_data(self: &Self) -> read::Result<CompressedData<'data>>`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn segment_name(self: &Self) -> Result<Option<&str>>`
  - `fn kind(self: &Self) -> SectionKind`
  - `fn relocations(self: &Self) -> MachORelocationIterator<'data, 'file, Mach, R>`
  - `fn relocation_map(self: &Self) -> read::Result<RelocationMap>`
  - `fn flags(self: &Self) -> SectionFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::section::MachOSection32

*Type Alias*: `MachOSection<'data, 'file, macho::MachHeader32<Endian>, R>`

A section in a [`MachOFile32`](super::MachOFile32).



## object::read::macho::section::MachOSection64

*Type Alias*: `MachOSection<'data, 'file, macho::MachHeader64<Endian>, R>`

A section in a [`MachOFile64`](super::MachOFile64).



## object::read::macho::section::MachOSectionIterator

*Struct*

An iterator for the sections in a [`MachOFile`].

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::macho::section::MachOSectionIterator32

*Type Alias*: `MachOSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>`

An iterator for the sections in a [`MachOFile32`](super::MachOFile32).



## object::read::macho::section::MachOSectionIterator64

*Type Alias*: `MachOSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>`

An iterator for the sections in a [`MachOFile64`](super::MachOFile64).



## object::read::macho::section::Section

*Trait*

A trait for generic access to [`macho::Section32`] and [`macho::Section64`].

**Methods:**

- `Word`
- `Endian`
- `sectname`
- `segname`
- `addr`
- `size`
- `offset`
- `align`
- `reloff`
- `nreloc`
- `flags`
- `name`: Return the `sectname` bytes up until the null terminator.
- `segment_name`: Return the `segname` bytes up until the null terminator.
- `file_range`: Return the offset and size of the section in the file.
- `data`: Return the section data.
- `relocations`: Return the relocation array.



