**object > read > elf > relocation**

# Module: read::elf::relocation

## Contents

**Structs**

- [`Crel`](#crel) - Compact relocation
- [`CrelIterator`](#creliterator) - Compact relocation iterator.
- [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator) - An iterator for the dynamic relocations in an [`ElfFile`].
- [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator) - An iterator for the relocations for an [`ElfSection`](super::ElfSection).
- [`RelocationSections`](#relocationsections) - A mapping from section index to associated relocation sections.
- [`RelrIterator`](#relriterator) - An iterator over the relative relocations in an ELF `SHT_RELR` section.

**Traits**

- [`Rel`](#rel) - A trait for generic access to [`elf::Rel32`] and [`elf::Rel64`].
- [`Rela`](#rela) - A trait for generic access to [`elf::Rela32`] and [`elf::Rela64`].
- [`Relr`](#relr) - A trait for generic access to [`elf::Relr32`] and [`elf::Relr64`].

**Type Aliases**

- [`ElfDynamicRelocationIterator32`](#elfdynamicrelocationiterator32) - An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32).
- [`ElfDynamicRelocationIterator64`](#elfdynamicrelocationiterator64) - An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64).
- [`ElfSectionRelocationIterator32`](#elfsectionrelocationiterator32) - An iterator for the relocations for an [`ElfSection32`](super::ElfSection32).
- [`ElfSectionRelocationIterator64`](#elfsectionrelocationiterator64) - An iterator for the relocations for an [`ElfSection64`](super::ElfSection64).

---

## object::read::elf::relocation::Crel

*Struct*

Compact relocation

The specification has been submited here: <https://groups.google.com/g/generic-abi/c/ppkaxtLb0P0/m/awgqZ_1CBAAJ>.

**Fields:**
- `r_offset: u64` - Relocation offset.
- `r_sym: u32` - Relocation symbol index.
- `r_type: u32` - Relocation type.
- `r_addend: i64` - Relocation addend.

**Methods:**

- `fn symbol(self: &Self) -> Option<SymbolIndex>` - Get the symbol index referenced by the relocation.
- `fn from_rel<R>(r: &R, endian: <R as >::Endian) -> Crel` - Build Crel type from Rel.
- `fn from_rela<R>(r: &R, endian: <R as >::Endian, is_mips64el: bool) -> Crel` - Build Crel type from Rela.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Crel`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::elf::relocation::CrelIterator

*Struct*

Compact relocation iterator.

**Generic Parameters:**
- 'data

**Methods:**

- `fn new(data: &'data [u8]) -> Result<Self, Error>` - Create a new CREL relocation iterator.
- `fn is_rela(self: &Self) -> bool` - True if the encoded relocations have addend.
- `fn len(self: &Self) -> usize` - Return the number of encoded relocations.
- `fn is_empty(self: &Self) -> bool` - Return true if there are no more relocations to parse.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> CrelIterator<'data>`



## object::read::elf::relocation::ElfDynamicRelocationIterator

*Struct*

An iterator for the dynamic relocations in an [`ElfFile`].

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::relocation::ElfDynamicRelocationIterator32

*Type Alias*: `ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>`

An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::relocation::ElfDynamicRelocationIterator64

*Type Alias*: `ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>`

An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::relocation::ElfSectionRelocationIterator

*Struct*

An iterator for the relocations for an [`ElfSection`](super::ElfSection).

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::relocation::ElfSectionRelocationIterator32

*Type Alias*: `ElfSectionRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>`

An iterator for the relocations for an [`ElfSection32`](super::ElfSection32).



## object::read::elf::relocation::ElfSectionRelocationIterator64

*Type Alias*: `ElfSectionRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>`

An iterator for the relocations for an [`ElfSection64`](super::ElfSection64).



## object::read::elf::relocation::Rel

*Trait*

A trait for generic access to [`elf::Rel32`] and [`elf::Rel64`].

**Methods:**

- `Word`
- `Sword`
- `Endian`
- `r_offset`
- `r_info`
- `r_sym`
- `r_type`
- `symbol`: Get the symbol index referenced by the relocation.



## object::read::elf::relocation::Rela

*Trait*

A trait for generic access to [`elf::Rela32`] and [`elf::Rela64`].

**Methods:**

- `Word`
- `Sword`
- `Endian`
- `r_offset`
- `r_info`
- `r_addend`
- `r_sym`
- `r_type`
- `symbol`: Get the symbol index referenced by the relocation.



## object::read::elf::relocation::RelocationSections

*Struct*

A mapping from section index to associated relocation sections.

**Methods:**

- `fn parse<'data, Elf, R>(endian: <Elf as >::Endian, sections: &SectionTable<'data, Elf, R>, symbol_section: SectionIndex) -> read::Result<Self>` - Create a new mapping using the section table.
- `fn get(self: &Self, index: SectionIndex) -> Option<SectionIndex>` - Given a section index, return the section index of the associated relocation section.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> RelocationSections`



## object::read::elf::relocation::Relr

*Trait*

A trait for generic access to [`elf::Relr32`] and [`elf::Relr64`].

**Methods:**

- `Word`
- `Endian`
- `COUNT`: The number of bits in the bit mask, excluding the lowest bit.
- `get`: Get the relocation entry.
- `next`: Return the offset corresponding to the next bit in the bit mask.



## object::read::elf::relocation::RelrIterator

*Struct*

An iterator over the relative relocations in an ELF `SHT_RELR` section.

Returned by [`SectionHeader::relr`](super::SectionHeader::relr).

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn new(endian: <Elf as >::Endian, data: &'data [<Elf as >::Relr]) -> Self` - Create a new iterator given the `SHT_RELR` section data.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



