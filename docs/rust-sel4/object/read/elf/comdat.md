**object > read > elf > comdat**

# Module: read::elf::comdat

## Contents

**Structs**

- [`ElfComdat`](#elfcomdat) - A COMDAT section group in an [`ElfFile`].
- [`ElfComdatIterator`](#elfcomdatiterator) - An iterator for the COMDAT section groups in an [`ElfFile`].
- [`ElfComdatSectionIterator`](#elfcomdatsectioniterator) - An iterator for the sections in a COMDAT section group in an [`ElfFile`].

**Type Aliases**

- [`ElfComdat32`](#elfcomdat32) - A COMDAT section group in an [`ElfFile32`](super::ElfFile32).
- [`ElfComdat64`](#elfcomdat64) - A COMDAT section group in an [`ElfFile64`](super::ElfFile64).
- [`ElfComdatIterator32`](#elfcomdatiterator32) - An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32).
- [`ElfComdatIterator64`](#elfcomdatiterator64) - An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64).
- [`ElfComdatSectionIterator32`](#elfcomdatsectioniterator32) - An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32).
- [`ElfComdatSectionIterator64`](#elfcomdatsectioniterator64) - An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64).

---

## object::read::elf::comdat::ElfComdat

*Struct*

A COMDAT section group in an [`ElfFile`].

Most functionality is provided by the [`ObjectComdat`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Methods:**

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` - Get the ELF file containing this COMDAT section group.
- `fn elf_section_header(self: &Self) -> &'data <Elf as >::SectionHeader` - Get the raw ELF section header for the COMDAT section group.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ObjectComdat**
  - `fn kind(self: &Self) -> ComdatKind`
  - `fn symbol(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>`
  - `fn name(self: &Self) -> read::Result<&'data str>`
  - `fn sections(self: &Self) -> <Self as >::SectionIterator`



## object::read::elf::comdat::ElfComdat32

*Type Alias*: `ElfComdat<'data, 'file, elf::FileHeader32<Endian>, R>`

A COMDAT section group in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::comdat::ElfComdat64

*Type Alias*: `ElfComdat<'data, 'file, elf::FileHeader64<Endian>, R>`

A COMDAT section group in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::comdat::ElfComdatIterator

*Struct*

An iterator for the COMDAT section groups in an [`ElfFile`].

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



## object::read::elf::comdat::ElfComdatIterator32

*Type Alias*: `ElfComdatIterator<'data, 'file, elf::FileHeader32<Endian>, R>`

An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::comdat::ElfComdatIterator64

*Type Alias*: `ElfComdatIterator<'data, 'file, elf::FileHeader64<Endian>, R>`

An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::comdat::ElfComdatSectionIterator

*Struct*

An iterator for the sections in a COMDAT section group in an [`ElfFile`].

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



## object::read::elf::comdat::ElfComdatSectionIterator32

*Type Alias*: `ElfComdatSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>`

An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::comdat::ElfComdatSectionIterator64

*Type Alias*: `ElfComdatSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>`

An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64).



