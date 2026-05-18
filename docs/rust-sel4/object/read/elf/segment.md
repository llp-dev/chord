**object > read > elf > segment**

# Module: read::elf::segment

## Contents

**Structs**

- [`ElfSegment`](#elfsegment) - A segment in an [`ElfFile`].
- [`ElfSegmentIterator`](#elfsegmentiterator) - An iterator for the segments in an [`ElfFile`].

**Traits**

- [`ProgramHeader`](#programheader) - A trait for generic access to [`elf::ProgramHeader32`] and [`elf::ProgramHeader64`].

**Type Aliases**

- [`ElfSegment32`](#elfsegment32) - A segment in an [`ElfFile32`](super::ElfFile32).
- [`ElfSegment64`](#elfsegment64) - A segment in an [`ElfFile64`](super::ElfFile64).
- [`ElfSegmentIterator32`](#elfsegmentiterator32) - An iterator for the segments in an [`ElfFile32`](super::ElfFile32).
- [`ElfSegmentIterator64`](#elfsegmentiterator64) - An iterator for the segments in an [`ElfFile64`](super::ElfFile64).

---

## object::read::elf::segment::ElfSegment

*Struct*

A segment in an [`ElfFile`].

Most functionality is provided by the [`ObjectSegment`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Methods:**

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` - Get the ELF file containing this segment.
- `fn elf_program_header(self: &Self) -> &'data <Elf as >::ProgramHeader` - Get the raw ELF program header for the segment.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ObjectSegment**
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> (u64, u64)`
  - `fn data(self: &Self) -> read::Result<&'data [u8]>`
  - `fn data_range(self: &Self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>`
  - `fn name_bytes(self: &Self) -> read::Result<Option<&[u8]>>`
  - `fn name(self: &Self) -> read::Result<Option<&str>>`
  - `fn flags(self: &Self) -> SegmentFlags`



## object::read::elf::segment::ElfSegment32

*Type Alias*: `ElfSegment<'data, 'file, elf::FileHeader32<Endian>, R>`

A segment in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::segment::ElfSegment64

*Type Alias*: `ElfSegment<'data, 'file, elf::FileHeader64<Endian>, R>`

A segment in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::segment::ElfSegmentIterator

*Struct*

An iterator for the segments in an [`ElfFile`].

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::segment::ElfSegmentIterator32

*Type Alias*: `ElfSegmentIterator<'data, 'file, elf::FileHeader32<Endian>, R>`

An iterator for the segments in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::segment::ElfSegmentIterator64

*Type Alias*: `ElfSegmentIterator<'data, 'file, elf::FileHeader64<Endian>, R>`

An iterator for the segments in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::segment::ProgramHeader

*Trait*

A trait for generic access to [`elf::ProgramHeader32`] and [`elf::ProgramHeader64`].

**Methods:**

- `Elf`
- `Word`
- `Endian`
- `p_type`
- `p_flags`
- `p_offset`
- `p_vaddr`
- `p_paddr`
- `p_filesz`
- `p_memsz`
- `p_align`
- `file_range`: Return the offset and size of the segment in the file.
- `data`: Return the segment data.
- `data_as_array`: Return the segment data as a slice of the given type.
- `data_range`: Return the segment data in the given virtual address range
- `dynamic`: Return entries in a dynamic segment.
- `interpreter`: Return the data in an interpreter segment.
- `notes`: Return a note iterator for the segment data.



