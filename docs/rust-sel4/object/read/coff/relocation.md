**object > read > coff > relocation**

# Module: read::coff::relocation

## Contents

**Structs**

- [`CoffRelocationIterator`](#coffrelocationiterator) - An iterator for the relocations in a [`CoffSection`](super::CoffSection).

**Type Aliases**

- [`CoffBigRelocationIterator`](#coffbigrelocationiterator) - An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection).

---

## object::read::coff::relocation::CoffBigRelocationIterator

*Type Alias*: `CoffRelocationIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection).



## object::read::coff::relocation::CoffRelocationIterator

*Struct*

An iterator for the relocations in a [`CoffSection`](super::CoffSection).

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



