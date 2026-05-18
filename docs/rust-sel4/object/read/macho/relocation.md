**object > read > macho > relocation**

# Module: read::macho::relocation

## Contents

**Structs**

- [`MachORelocationIterator`](#machorelocationiterator) - An iterator for the relocations in a [`MachOSection`](super::MachOSection).

**Type Aliases**

- [`MachORelocationIterator32`](#machorelocationiterator32) - An iterator for the relocations in a [`MachOSection32`](super::MachOSection32).
- [`MachORelocationIterator64`](#machorelocationiterator64) - An iterator for the relocations in a [`MachOSection64`](super::MachOSection64).

---

## object::read::macho::relocation::MachORelocationIterator

*Struct*

An iterator for the relocations in a [`MachOSection`](super::MachOSection).

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::macho::relocation::MachORelocationIterator32

*Type Alias*: `MachORelocationIterator<'data, 'file, macho::MachHeader32<Endian>, R>`

An iterator for the relocations in a [`MachOSection32`](super::MachOSection32).



## object::read::macho::relocation::MachORelocationIterator64

*Type Alias*: `MachORelocationIterator<'data, 'file, macho::MachHeader64<Endian>, R>`

An iterator for the relocations in a [`MachOSection64`](super::MachOSection64).



