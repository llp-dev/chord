**object > read > elf > symbol**

# Module: read::elf::symbol

## Contents

**Structs**

- [`ElfSymbol`](#elfsymbol) - A symbol in an [`ElfFile`](super::ElfFile).
- [`ElfSymbolIterator`](#elfsymboliterator) - An iterator for the symbols in an [`ElfFile`](super::ElfFile).
- [`ElfSymbolTable`](#elfsymboltable) - A symbol table in an [`ElfFile`](super::ElfFile).
- [`SymbolTable`](#symboltable) - A table of symbol entries in an ELF file.

**Traits**

- [`Sym`](#sym) - A trait for generic access to [`elf::Sym32`] and [`elf::Sym64`].

**Type Aliases**

- [`ElfSymbol32`](#elfsymbol32) - A symbol in an [`ElfFile32`](super::ElfFile32).
- [`ElfSymbol64`](#elfsymbol64) - A symbol in an [`ElfFile64`](super::ElfFile64).
- [`ElfSymbolIterator32`](#elfsymboliterator32) - An iterator for the symbols in an [`ElfFile32`](super::ElfFile32).
- [`ElfSymbolIterator64`](#elfsymboliterator64) - An iterator for the symbols in an [`ElfFile64`](super::ElfFile64).
- [`ElfSymbolTable32`](#elfsymboltable32) - A symbol table in an [`ElfFile32`](super::ElfFile32).
- [`ElfSymbolTable64`](#elfsymboltable64) - A symbol table in an [`ElfFile32`](super::ElfFile32).

---

## object::read::elf::symbol::ElfSymbol

*Struct*

A symbol in an [`ElfFile`](super::ElfFile).

Most functionality is provided by the [`ObjectSymbol`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Methods:**

- `fn endian(self: &Self) -> <Elf as >::Endian` - Get the endianness of the ELF file.
- `fn raw_symbol(self: &Self) -> &'data <Elf as >::Sym` - Return a reference to the raw symbol structure.
- `fn elf_symbol(self: &Self) -> &'data <Elf as >::Sym` - Get the raw ELF symbol structure.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ElfSymbol<'data, 'file, Elf, R>`
- **ObjectSymbol**
  - `fn index(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>`
  - `fn name(self: &Self) -> read::Result<&'data str>`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn kind(self: &Self) -> SymbolKind`
  - `fn section(self: &Self) -> SymbolSection`
  - `fn is_undefined(self: &Self) -> bool`
  - `fn is_definition(self: &Self) -> bool`
  - `fn is_common(self: &Self) -> bool`
  - `fn is_weak(self: &Self) -> bool`
  - `fn scope(self: &Self) -> SymbolScope`
  - `fn is_global(self: &Self) -> bool`
  - `fn is_local(self: &Self) -> bool`
  - `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>`



## object::read::elf::symbol::ElfSymbol32

*Type Alias*: `ElfSymbol<'data, 'file, elf::FileHeader32<Endian>, R>`

A symbol in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::symbol::ElfSymbol64

*Type Alias*: `ElfSymbol<'data, 'file, elf::FileHeader64<Endian>, R>`

A symbol in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::symbol::ElfSymbolIterator

*Struct*

An iterator for the symbols in an [`ElfFile`](super::ElfFile).

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



## object::read::elf::symbol::ElfSymbolIterator32

*Type Alias*: `ElfSymbolIterator<'data, 'file, elf::FileHeader32<Endian>, R>`

An iterator for the symbols in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::symbol::ElfSymbolIterator64

*Type Alias*: `ElfSymbolIterator<'data, 'file, elf::FileHeader64<Endian>, R>`

An iterator for the symbols in an [`ElfFile64`](super::ElfFile64).



## object::read::elf::symbol::ElfSymbolTable

*Struct*

A symbol table in an [`ElfFile`](super::ElfFile).

**Generic Parameters:**
- 'data
- 'file
- Elf
- R

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ElfSymbolTable<'data, 'file, Elf, R>`
- **ObjectSymbolTable**
  - `fn symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>`



## object::read::elf::symbol::ElfSymbolTable32

*Type Alias*: `ElfSymbolTable<'data, 'file, elf::FileHeader32<Endian>, R>`

A symbol table in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::symbol::ElfSymbolTable64

*Type Alias*: `ElfSymbolTable<'data, 'file, elf::FileHeader64<Endian>, R>`

A symbol table in an [`ElfFile32`](super::ElfFile32).



## object::read::elf::symbol::Sym

*Trait*

A trait for generic access to [`elf::Sym32`] and [`elf::Sym64`].

**Methods:**

- `Word`
- `Endian`
- `st_name`
- `st_info`
- `st_bind`
- `st_type`
- `st_other`
- `st_visibility`
- `st_shndx`
- `st_value`
- `st_size`
- `name`: Parse the symbol name from the string table.
- `is_undefined`: Return true if the symbol section is `SHN_UNDEF`.
- `is_definition`: Return true if the symbol is a definition of a function or data object.
- `is_common`: Return true if the symbol section is `SHN_COMMON`.
- `is_absolute`: Return true if the symbol section is `SHN_ABS`.
- `is_local`: Return true if the symbol binding is `STB_LOCAL`.
- `is_weak`: Return true if the symbol binding is `STB_WEAK`.



## object::read::elf::symbol::SymbolTable

*Struct*

A table of symbol entries in an ELF file.

Also includes the string table used for the symbol names.

Returned by [`SectionTable::symbols`].

**Generic Parameters:**
- 'data
- Elf
- R

**Methods:**

- `fn parse(endian: <Elf as >::Endian, data: R, sections: &SectionTable<'data, Elf, R>, section_index: SectionIndex, section: &<Elf as >::SectionHeader) -> read::Result<SymbolTable<'data, Elf, R>>` - Parse the given symbol table section.
- `fn section(self: &Self) -> SectionIndex` - Return the section index of this symbol table.
- `fn shndx_section(self: &Self) -> SectionIndex` - Return the section index of the shndx table.
- `fn string_section(self: &Self) -> SectionIndex` - Return the section index of the linked string table.
- `fn strings(self: &Self) -> StringTable<'data, R>` - Return the string table used for the symbol names.
- `fn symbols(self: &Self) -> &'data [<Elf as >::Sym]` - Return the symbol table.
- `fn iter(self: &Self) -> slice::Iter<'data, <Elf as >::Sym>` - Iterate over the symbols.
- `fn enumerate(self: &Self) -> impl Trait` - Iterate over the symbols and their indices.
- `fn is_empty(self: &Self) -> bool` - Return true if the symbol table is empty.
- `fn len(self: &Self) -> usize` - The number of symbols.
- `fn symbol(self: &Self, index: SymbolIndex) -> read::Result<&'data <Elf as >::Sym>` - Get the symbol at the given index.
- `fn shndx(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> Option<u32>` - Return the extended section index for the given symbol if present.
- `fn symbol_section(self: &Self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym, index: SymbolIndex) -> read::Result<Option<SectionIndex>>` - Return the section index for the given symbol.
- `fn symbol_name(self: &Self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym) -> read::Result<&'data [u8]>` - Return the symbol name for the given symbol.
- `fn map<Entry, F>(self: &Self, endian: <Elf as >::Endian, f: F) -> SymbolMap<Entry>` - Construct a map from addresses to a user-defined map entry.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SymbolTable<'data, Elf, R>`
- **Default**
  - `fn default() -> Self`



