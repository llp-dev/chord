**object > read > xcoff > comdat**

# Module: read::xcoff::comdat

## Contents

**Structs**

- [`XcoffComdat`](#xcoffcomdat) - A COMDAT section group in a [`XcoffFile`].
- [`XcoffComdatIterator`](#xcoffcomdatiterator) - An iterator for the COMDAT section groups in a [`XcoffFile`].
- [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator) - An iterator for the sections in a COMDAT section group in a [`XcoffFile`].

**Type Aliases**

- [`XcoffComdat32`](#xcoffcomdat32) - A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).
- [`XcoffComdat64`](#xcoffcomdat64) - A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).
- [`XcoffComdatIterator32`](#xcoffcomdatiterator32) - An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).
- [`XcoffComdatIterator64`](#xcoffcomdatiterator64) - An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).
- [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32) - An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).
- [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64) - An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

---

## object::read::xcoff::comdat::XcoffComdat

*Struct*

A COMDAT section group in a [`XcoffFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Trait Implementations:**

- **ObjectComdat**
  - `fn kind(self: &Self) -> ComdatKind`
  - `fn symbol(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn sections(self: &Self) -> <Self as >::SectionIterator`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::xcoff::comdat::XcoffComdat32

*Type Alias*: `XcoffComdat<'data, 'file, xcoff::FileHeader32, R>`

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::comdat::XcoffComdat64

*Type Alias*: `XcoffComdat<'data, 'file, xcoff::FileHeader64, R>`

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).



## object::read::xcoff::comdat::XcoffComdatIterator

*Struct*

An iterator for the COMDAT section groups in a [`XcoffFile`].

This is a stub that doesn't implement any functionality.

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



## object::read::xcoff::comdat::XcoffComdatIterator32

*Type Alias*: `XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>`

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::comdat::XcoffComdatIterator64

*Type Alias*: `XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>`

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).



## object::read::xcoff::comdat::XcoffComdatSectionIterator

*Struct*

An iterator for the sections in a COMDAT section group in a [`XcoffFile`].

This is a stub that doesn't implement any functionality.

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



## object::read::xcoff::comdat::XcoffComdatSectionIterator32

*Type Alias*: `XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>`

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::comdat::XcoffComdatSectionIterator64

*Type Alias*: `XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>`

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).



