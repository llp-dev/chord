**object > read > pe > section**

# Module: read::pe::section

## Contents

**Structs**

- [`PeRelocationIterator`](#perelocationiterator) - An iterator for the relocations in an [`PeSection`].
- [`PeSection`](#pesection) - A section in a [`PeFile`].
- [`PeSectionIterator`](#pesectioniterator) - An iterator for the sections in a [`PeFile`].
- [`PeSegment`](#pesegment) - A loadable section in a [`PeFile`].
- [`PeSegmentIterator`](#pesegmentiterator) - An iterator for the loadable sections in a [`PeFile`].

**Type Aliases**

- [`PeSection32`](#pesection32) - A section in a [`PeFile32`](super::PeFile32).
- [`PeSection64`](#pesection64) - A section in a [`PeFile64`](super::PeFile64).
- [`PeSectionIterator32`](#pesectioniterator32) - An iterator for the sections in a [`PeFile32`](super::PeFile32).
- [`PeSectionIterator64`](#pesectioniterator64) - An iterator for the sections in a [`PeFile64`](super::PeFile64).
- [`PeSegment32`](#pesegment32) - A loadable section in a [`PeFile32`](super::PeFile32).
- [`PeSegment64`](#pesegment64) - A loadable section in a [`PeFile64`](super::PeFile64).
- [`PeSegmentIterator32`](#pesegmentiterator32) - An iterator for the loadable sections in a [`PeFile32`](super::PeFile32).
- [`PeSegmentIterator64`](#pesegmentiterator64) - An iterator for the loadable sections in a [`PeFile64`](super::PeFile64).

---

## object::read::pe::section::PeRelocationIterator

*Struct*

An iterator for the relocations in an [`PeSection`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- R

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::pe::section::PeSection

*Struct*

A section in a [`PeFile`].

Most functionality is provided by the [`ObjectSection`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Pe
- R

**Methods:**

- `fn pe_file(self: &Self) -> &'file PeFile<'data, Pe, R>` - Get the PE file containing this segment.
- `fn pe_section(self: &Self) -> &'data pe::ImageSectionHeader` - Get the raw PE section header.

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
  - `fn relocations(self: &Self) -> PeRelocationIterator<'data, 'file, R>`
  - `fn relocation_map(self: &Self) -> read::Result<RelocationMap>`
  - `fn flags(self: &Self) -> SectionFlags`



## object::read::pe::section::PeSection32

*Type Alias*: `PeSection<'data, 'file, pe::ImageNtHeaders32, R>`

A section in a [`PeFile32`](super::PeFile32).



## object::read::pe::section::PeSection64

*Type Alias*: `PeSection<'data, 'file, pe::ImageNtHeaders64, R>`

A section in a [`PeFile64`](super::PeFile64).



## object::read::pe::section::PeSectionIterator

*Struct*

An iterator for the sections in a [`PeFile`].

**Generic Parameters:**
- 'data
- 'file
- Pe
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::pe::section::PeSectionIterator32

*Type Alias*: `PeSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>`

An iterator for the sections in a [`PeFile32`](super::PeFile32).



## object::read::pe::section::PeSectionIterator64

*Type Alias*: `PeSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>`

An iterator for the sections in a [`PeFile64`](super::PeFile64).



## object::read::pe::section::PeSegment

*Struct*

A loadable section in a [`PeFile`].

Most functionality is provided by the [`ObjectSegment`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Pe
- R

**Methods:**

- `fn pe_file(self: &Self) -> &'file PeFile<'data, Pe, R>` - Get the PE file containing this segment.
- `fn pe_section(self: &Self) -> &'data pe::ImageSectionHeader` - Get the raw PE section header.

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



## object::read::pe::section::PeSegment32

*Type Alias*: `PeSegment<'data, 'file, pe::ImageNtHeaders32, R>`

A loadable section in a [`PeFile32`](super::PeFile32).



## object::read::pe::section::PeSegment64

*Type Alias*: `PeSegment<'data, 'file, pe::ImageNtHeaders64, R>`

A loadable section in a [`PeFile64`](super::PeFile64).



## object::read::pe::section::PeSegmentIterator

*Struct*

An iterator for the loadable sections in a [`PeFile`].

**Generic Parameters:**
- 'data
- 'file
- Pe
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::pe::section::PeSegmentIterator32

*Type Alias*: `PeSegmentIterator<'data, 'file, pe::ImageNtHeaders32, R>`

An iterator for the loadable sections in a [`PeFile32`](super::PeFile32).



## object::read::pe::section::PeSegmentIterator64

*Type Alias*: `PeSegmentIterator<'data, 'file, pe::ImageNtHeaders64, R>`

An iterator for the loadable sections in a [`PeFile64`](super::PeFile64).



