**object > read > xcoff > relocation**

# Module: read::xcoff::relocation

## Contents

**Structs**

- [`XcoffRelocationIterator`](#xcoffrelocationiterator) - An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

**Traits**

- [`Rel`](#rel) - A trait for generic access to [`xcoff::Rel32`] and [`xcoff::Rel64`].

**Type Aliases**

- [`XcoffRelocationIterator32`](#xcoffrelocationiterator32) - An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).
- [`XcoffRelocationIterator64`](#xcoffrelocationiterator64) - An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).

---

## object::read::xcoff::relocation::Rel

*Trait*

A trait for generic access to [`xcoff::Rel32`] and [`xcoff::Rel64`].

**Methods:**

- `Word`
- `r_vaddr`
- `r_symndx`
- `r_rsize`
- `r_rtype`
- `symbol`



## object::read::xcoff::relocation::XcoffRelocationIterator

*Struct*

An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::xcoff::relocation::XcoffRelocationIterator32

*Type Alias*: `XcoffRelocationIterator<'data, 'file, xcoff::FileHeader32, R>`

An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).



## object::read::xcoff::relocation::XcoffRelocationIterator64

*Type Alias*: `XcoffRelocationIterator<'data, 'file, xcoff::FileHeader64, R>`

An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).



