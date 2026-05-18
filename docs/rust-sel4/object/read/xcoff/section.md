**object > read > xcoff > section**

# Module: read::xcoff::section

## Contents

**Structs**

- [`SectionTable`](#sectiontable) - The table of section headers in an XCOFF file.
- [`XcoffSection`](#xcoffsection) - A section in an [`XcoffFile`].
- [`XcoffSectionIterator`](#xcoffsectioniterator) - An iterator for the sections in an [`XcoffFile`].

**Traits**

- [`SectionHeader`](#sectionheader) - A trait for generic access to [`xcoff::SectionHeader32`] and [`xcoff::SectionHeader64`].

**Type Aliases**

- [`XcoffSection32`](#xcoffsection32) - A section in an [`XcoffFile32`](super::XcoffFile32).
- [`XcoffSection64`](#xcoffsection64) - A section in an [`XcoffFile64`](super::XcoffFile64).
- [`XcoffSectionIterator32`](#xcoffsectioniterator32) - An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).
- [`XcoffSectionIterator64`](#xcoffsectioniterator64) - An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).

---

## object::read::xcoff::section::SectionHeader

*Trait*

A trait for generic access to [`xcoff::SectionHeader32`] and [`xcoff::SectionHeader64`].

**Methods:**

- `Word`
- `HalfWord`
- `Xcoff`
- `Rel`
- `s_name`
- `s_paddr`
- `s_vaddr`
- `s_size`
- `s_scnptr`
- `s_relptr`
- `s_lnnoptr`
- `s_nreloc`
- `s_nlnno`
- `s_flags`
- `name`: Return the section name.
- `file_range`: Return the offset and size of the section in the file.
- `data`: Return the section data.
- `relocations`: Read the relocations.



## object::read::xcoff::section::SectionTable

*Struct*

The table of section headers in an XCOFF file.

Returned by [`FileHeader::sections`].

**Generic Parameters:**
- 'data
- Xcoff

**Methods:**

- `fn parse<R>(header: &Xcoff, data: R, offset: & mut u64) -> Result<Self>` - Parse the section table.
- `fn iter(self: &Self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` - Iterate over the section headers.
- `fn is_empty(self: &Self) -> bool` - Return true if the section table is empty.
- `fn len(self: &Self) -> usize` - The number of section headers.
- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` - Return the section header at the given index.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SectionTable<'data, Xcoff>`
- **Default**
  - `fn default() -> Self`



## object::read::xcoff::section::XcoffSection

*Struct*

A section in an [`XcoffFile`].

Most functionality is provided by the [`ObjectSection`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Methods:**

- `fn xcoff_file(self: &Self) -> &'file XcoffFile<'data, Xcoff, R>` - Get the XCOFF file containing this section.
- `fn xcoff_section(self: &Self) -> &'data <Xcoff as >::SectionHeader` - Get the raw XCOFF section header.
- `fn xcoff_relocations(self: &Self) -> Result<&'data [<Xcoff as >::Rel]>` - Get the raw XCOFF relocation entries for this section.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ObjectSection**
  - `fn index(self: &Self) -> SectionIndex`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> Option<(u64, u64)>`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`
  - `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>`
  - `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>`
  - `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>`
  - `fn name(self: &Self) -> read::Result<&'data str>`
  - `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn segment_name(self: &Self) -> Result<Option<&str>>`
  - `fn kind(self: &Self) -> SectionKind`
  - `fn relocations(self: &Self) -> <Self as >::RelocationIterator`
  - `fn relocation_map(self: &Self) -> read::Result<RelocationMap>`
  - `fn flags(self: &Self) -> SectionFlags`
  - `fn uncompressed_data(self: &Self) -> Result<alloc::borrow::Cow<'data, [u8]>>`



## object::read::xcoff::section::XcoffSection32

*Type Alias*: `XcoffSection<'data, 'file, xcoff::FileHeader32, R>`

A section in an [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::section::XcoffSection64

*Type Alias*: `XcoffSection<'data, 'file, xcoff::FileHeader64, R>`

A section in an [`XcoffFile64`](super::XcoffFile64).



## object::read::xcoff::section::XcoffSectionIterator

*Struct*

An iterator for the sections in an [`XcoffFile`].

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::xcoff::section::XcoffSectionIterator32

*Type Alias*: `XcoffSectionIterator<'data, 'file, xcoff::FileHeader32, R>`

An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::section::XcoffSectionIterator64

*Type Alias*: `XcoffSectionIterator<'data, 'file, xcoff::FileHeader64, R>`

An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).



