**object > read > elf > note**

# Module: read::elf::note

## Contents

**Structs**

- [`GnuProperty`](#gnuproperty) - A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note.
- [`GnuPropertyIterator`](#gnupropertyiterator) - An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note.
- [`Note`](#note) - A parsed [`NoteHeader`].
- [`NoteIterator`](#noteiterator) - An iterator over the notes in an ELF section or segment.

**Traits**

- [`NoteHeader`](#noteheader) - A trait for generic access to [`elf::NoteHeader32`] and [`elf::NoteHeader64`].

---

## object::read::elf::note::GnuProperty

*Struct*

A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note.

**Generic Parameters:**
- 'data

**Methods:**

- `fn pr_type(self: &Self) -> u32` - Return the property type.
- `fn pr_data(self: &Self) -> &'data [u8]` - Return the property data.
- `fn data_u32<E>(self: &Self, endian: E) -> read::Result<u32>` - Parse the property data as an unsigned 32-bit integer.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::elf::note::GnuPropertyIterator

*Struct*

An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note.

Returned by [`Note::gnu_properties`].

**Generic Parameters:**
- 'data
- Endian

**Methods:**

- `fn next(self: & mut Self) -> read::Result<Option<GnuProperty<'data>>>` - Returns the next property.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::note::Note

*Struct*

A parsed [`NoteHeader`].

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn n_type(self: &Self, endian: <Elf as >::Endian) -> u32` - Return the `n_type` field of the `NoteHeader`.
- `fn n_namesz(self: &Self, endian: <Elf as >::Endian) -> u32` - Return the `n_namesz` field of the `NoteHeader`.
- `fn n_descsz(self: &Self, endian: <Elf as >::Endian) -> u32` - Return the `n_descsz` field of the `NoteHeader`.
- `fn name_bytes(self: &Self) -> &'data [u8]` - Return the bytes for the name field following the `NoteHeader`.
- `fn name(self: &Self) -> &'data [u8]` - Return the bytes for the name field following the `NoteHeader`,
- `fn desc(self: &Self) -> &'data [u8]` - Return the bytes for the desc field following the `NoteHeader`.
- `fn gnu_properties(self: &Self, endian: <Elf as >::Endian) -> Option<GnuPropertyIterator<'data, <Elf as >::Endian>>` - Return an iterator for properties if this note's type is [`elf::NT_GNU_PROPERTY_TYPE_0`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::elf::note::NoteHeader

*Trait*

A trait for generic access to [`elf::NoteHeader32`] and [`elf::NoteHeader64`].

**Methods:**

- `Endian`
- `n_namesz`
- `n_descsz`
- `n_type`



## object::read::elf::note::NoteIterator

*Struct*

An iterator over the notes in an ELF section or segment.

Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)
and [`SectionHeader::notes`](super::SectionHeader::notes).

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn new(endian: <Elf as >::Endian, align: <Elf as >::Word, data: &'data [u8]) -> read::Result<Self>` - An iterator over the notes in an ELF section or segment.
- `fn next(self: & mut Self) -> read::Result<Option<Note<'data, Elf>>>` - Returns the next note.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



