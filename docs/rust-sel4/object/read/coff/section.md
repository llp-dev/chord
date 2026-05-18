**object > read > coff > section**

# Module: read::coff::section

## Contents

**Structs**

- [`CoffSection`](#coffsection) - A section in a [`CoffFile`].
- [`CoffSectionIterator`](#coffsectioniterator) - An iterator for the sections in a [`CoffFile`].
- [`CoffSegment`](#coffsegment) - A loadable section in a [`CoffFile`].
- [`CoffSegmentIterator`](#coffsegmentiterator) - An iterator for the loadable sections in a [`CoffFile`].
- [`SectionTable`](#sectiontable) - The table of section headers in a COFF or PE file.

**Type Aliases**

- [`CoffBigSection`](#coffbigsection) - A section in a [`CoffBigFile`](super::CoffBigFile).
- [`CoffBigSectionIterator`](#coffbigsectioniterator) - An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile).
- [`CoffBigSegment`](#coffbigsegment) - A loadable section in a [`CoffBigFile`](super::CoffBigFile).
- [`CoffBigSegmentIterator`](#coffbigsegmentiterator) - An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile).

---

## object::read::coff::section::CoffBigSection

*Type Alias*: `CoffSection<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

A section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSection`] trait implementation.



## object::read::coff::section::CoffBigSectionIterator

*Type Alias*: `CoffSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile).



## object::read::coff::section::CoffBigSegment

*Type Alias*: `CoffSegment<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

A loadable section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSegment`] trait implementation.



## object::read::coff::section::CoffBigSegmentIterator

*Type Alias*: `CoffSegmentIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile).



## object::read::coff::section::CoffSection

*Struct*

A section in a [`CoffFile`].

Most functionality is provided by the [`ObjectSection`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Methods:**

- `fn coff_file(self: &Self) -> &'file CoffFile<'data, R, Coff>` - Get the COFF file containing this section.
- `fn coff_section(self: &Self) -> &'data pe::ImageSectionHeader` - Get the raw COFF section header.
- `fn coff_relocations(self: &Self) -> Result<&'data [pe::ImageRelocation]>` - Get the raw COFF relocations for this section.

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
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn segment_name(self: &Self) -> Result<Option<&str>>`
  - `fn kind(self: &Self) -> SectionKind`
  - `fn relocations(self: &Self) -> CoffRelocationIterator<'data, 'file, R, Coff>`
  - `fn relocation_map(self: &Self) -> read::Result<RelocationMap>`
  - `fn flags(self: &Self) -> SectionFlags`



## object::read::coff::section::CoffSectionIterator

*Struct*

An iterator for the sections in a [`CoffFile`].

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::coff::section::CoffSegment

*Struct*

A loadable section in a [`CoffFile`].

Most functionality is provided by the [`ObjectSegment`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Methods:**

- `fn coff_file(self: &Self) -> &'file CoffFile<'data, R, Coff>` - Get the COFF file containing this segment.
- `fn coff_section(self: &Self) -> &'data pe::ImageSectionHeader` - Get the raw COFF section header.

**Trait Implementations:**

- **ObjectSegment**
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> (u64, u64)`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`
  - `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn name(self: &Self) -> Result<Option<&str>>`
  - `fn flags(self: &Self) -> SegmentFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::coff::section::CoffSegmentIterator

*Struct*

An iterator for the loadable sections in a [`CoffFile`].

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::coff::section::SectionTable

*Struct*

The table of section headers in a COFF or PE file.

Returned by [`CoffHeader::sections`] and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

**Generic Parameters:**
- 'data

**Methods:**

- `fn pe_file_range_at(self: &Self, va: u32) -> Option<(u32, u32)>` - Return the file offset of the given virtual address, and the size up
- `fn pe_data_at<R>(self: &Self, data: R, va: u32) -> Option<&'data [u8]>` - Return the data starting at the given virtual address, up to the end of the
- `fn pe_data_containing<R>(self: &Self, data: R, va: u32) -> Option<(&'data [u8], u32)>` - Return the data of the section that contains the given virtual address in a PE file.
- `fn section_containing(self: &Self, va: u32) -> Option<&'data ImageSectionHeader>` - Return the section that contains a given virtual address.
- `fn parse<Coff, R>(header: &Coff, data: R, offset: u64) -> Result<Self>` - Parse the section table.
- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageSectionHeader>` - Iterate over the section headers.
- `fn enumerate(self: &Self) -> impl Trait` - Iterate over the section headers and their indices.
- `fn is_empty(self: &Self) -> bool` - Return true if the section table is empty.
- `fn len(self: &Self) -> usize` - The number of section headers.
- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` - Return the section header at the given index.
- `fn section_by_name<R>(self: &Self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` - Return the section header with the given name.
- `fn max_section_file_offset(self: &Self) -> u64` - Compute the maximum file offset used by sections.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SectionTable<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> SectionTable<'data>`



