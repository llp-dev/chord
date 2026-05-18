**object > read > coff > comdat**

# Module: read::coff::comdat

## Contents

**Structs**

- [`CoffComdat`](#coffcomdat) - A COMDAT section group in a [`CoffFile`].
- [`CoffComdatIterator`](#coffcomdatiterator) - An iterator for the COMDAT section groups in a [`CoffFile`].
- [`CoffComdatSectionIterator`](#coffcomdatsectioniterator) - An iterator for the sections in a COMDAT section group in a [`CoffFile`].

**Type Aliases**

- [`CoffBigComdat`](#coffbigcomdat) - A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).
- [`CoffBigComdatIterator`](#coffbigcomdatiterator) - An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile).
- [`CoffBigComdatSectionIterator`](#coffbigcomdatsectioniterator) - An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

---

## object::read::coff::comdat::CoffBigComdat

*Type Alias*: `CoffComdat<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectComdat`] trait implementation.



## object::read::coff::comdat::CoffBigComdatIterator

*Type Alias*: `CoffComdatIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile).



## object::read::coff::comdat::CoffBigComdatSectionIterator

*Type Alias*: `CoffComdatSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).



## object::read::coff::comdat::CoffComdat

*Struct*

A COMDAT section group in a [`CoffFile`].

Most functionality is provided by the [`ObjectComdat`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Trait Implementations:**

- **ObjectComdat**
  - `fn kind(self: &Self) -> ComdatKind`
  - `fn symbol(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn sections(self: &Self) -> <Self as >::SectionIterator`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::coff::comdat::CoffComdatIterator

*Struct*

An iterator for the COMDAT section groups in a [`CoffFile`].

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



## object::read::coff::comdat::CoffComdatSectionIterator

*Struct*

An iterator for the sections in a COMDAT section group in a [`CoffFile`].

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



