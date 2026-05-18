**object > read > macho > segment**

# Module: read::macho::segment

## Contents

**Structs**

- [`MachOSegment`](#machosegment) - A segment in a [`MachOFile`].
- [`MachOSegmentIterator`](#machosegmentiterator) - An iterator for the segments in a [`MachOFile`].

**Traits**

- [`Segment`](#segment) - A trait for generic access to [`macho::SegmentCommand32`] and [`macho::SegmentCommand64`].

**Type Aliases**

- [`MachOSegment32`](#machosegment32) - A segment in a [`MachOFile32`](super::MachOFile32).
- [`MachOSegment64`](#machosegment64) - A segment in a [`MachOFile64`](super::MachOFile64).
- [`MachOSegmentIterator32`](#machosegmentiterator32) - An iterator for the segments in a [`MachOFile32`](super::MachOFile32).
- [`MachOSegmentIterator64`](#machosegmentiterator64) - An iterator for the segments in a [`MachOFile64`](super::MachOFile64).

---

## object::read::macho::segment::MachOSegment

*Struct*

A segment in a [`MachOFile`].

Most functionality is provided by the [`ObjectSegment`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Methods:**

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` - Get the Mach-O file containing this segment.
- `fn macho_segment(self: &Self) -> &'data <Mach as >::Segment` - Get the raw Mach-O segment structure.

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



## object::read::macho::segment::MachOSegment32

*Type Alias*: `MachOSegment<'data, 'file, macho::MachHeader32<Endian>, R>`

A segment in a [`MachOFile32`](super::MachOFile32).



## object::read::macho::segment::MachOSegment64

*Type Alias*: `MachOSegment<'data, 'file, macho::MachHeader64<Endian>, R>`

A segment in a [`MachOFile64`](super::MachOFile64).



## object::read::macho::segment::MachOSegmentIterator

*Struct*

An iterator for the segments in a [`MachOFile`].

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::macho::segment::MachOSegmentIterator32

*Type Alias*: `MachOSegmentIterator<'data, 'file, macho::MachHeader32<Endian>, R>`

An iterator for the segments in a [`MachOFile32`](super::MachOFile32).



## object::read::macho::segment::MachOSegmentIterator64

*Type Alias*: `MachOSegmentIterator<'data, 'file, macho::MachHeader64<Endian>, R>`

An iterator for the segments in a [`MachOFile64`](super::MachOFile64).



## object::read::macho::segment::Segment

*Trait*

A trait for generic access to [`macho::SegmentCommand32`] and [`macho::SegmentCommand64`].

**Methods:**

- `Word`
- `Endian`
- `Section`
- `from_command`
- `cmd`
- `cmdsize`
- `segname`
- `vmaddr`
- `vmsize`
- `fileoff`
- `filesize`
- `maxprot`
- `initprot`
- `nsects`
- `flags`
- `name`: Return the `segname` bytes up until the null terminator.
- `file_range`: Return the offset and size of the segment in the file.
- `data`: Get the segment data from the file data.
- `sections`: Get the array of sections from the data following the segment command.



