**object > read > coff > symbol**

# Module: read::coff::symbol

## Contents

**Structs**

- [`CoffSymbol`](#coffsymbol) - A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile).
- [`CoffSymbolIterator`](#coffsymboliterator) - An iterator for the symbols in a [`CoffFile`](super::CoffFile)
- [`CoffSymbolTable`](#coffsymboltable) - A symbol table in a [`CoffFile`](super::CoffFile)
- [`SymbolIterator`](#symboliterator) - An iterator for symbol entries in a COFF or PE file.
- [`SymbolTable`](#symboltable) - A table of symbol entries in a COFF or PE file.

**Traits**

- [`ImageSymbol`](#imagesymbol) - A trait for generic access to [`pe::ImageSymbol`] and [`pe::ImageSymbolEx`].

**Type Aliases**

- [`CoffBigSymbol`](#coffbigsymbol) - A symbol in a [`CoffBigFile`](super::CoffBigFile).
- [`CoffBigSymbolIterator`](#coffbigsymboliterator) - An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile).
- [`CoffBigSymbolTable`](#coffbigsymboltable) - A symbol table in a [`CoffBigFile`](super::CoffBigFile).

---

## object::read::coff::symbol::CoffBigSymbol

*Type Alias*: `CoffSymbol<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

A symbol in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSymbol`] trait implementation.



## object::read::coff::symbol::CoffBigSymbolIterator

*Type Alias*: `CoffSymbolIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile).



## object::read::coff::symbol::CoffBigSymbolTable

*Type Alias*: `CoffSymbolTable<'data, 'file, R, pe::AnonObjectHeaderBigobj>`

A symbol table in a [`CoffBigFile`](super::CoffBigFile).



## object::read::coff::symbol::CoffSymbol

*Struct*

A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile).

Most functionality is provided by the [`ObjectSymbol`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Methods:**

- `fn raw_symbol(self: &Self) -> &'data <Coff as >::ImageSymbol` - Get the raw `ImageSymbol` struct.
- `fn coff_symbol(self: &Self) -> &'data <Coff as >::ImageSymbol` - Get the raw `ImageSymbol` struct.

**Traits:** Copy

**Trait Implementations:**

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
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CoffSymbol<'data, 'file, R, Coff>`



## object::read::coff::symbol::CoffSymbolIterator

*Struct*

An iterator for the symbols in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::coff::symbol::CoffSymbolTable

*Struct*

A symbol table in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

**Generic Parameters:**
- 'data
- 'file
- R
- Coff

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CoffSymbolTable<'data, 'file, R, Coff>`
- **ObjectSymbolTable**
  - `fn symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`



## object::read::coff::symbol::ImageSymbol

*Trait*

A trait for generic access to [`pe::ImageSymbol`] and [`pe::ImageSymbolEx`].

**Methods:**

- `raw_name`
- `value`
- `section_number`
- `typ`
- `storage_class`
- `number_of_aux_symbols`
- `name`: Parse a COFF symbol name.
- `address`: Return the symbol address.
- `section`: Return the section index for the symbol.
- `is_definition`: Return true if the symbol is a definition of a function or data object.
- `has_aux_file_name`: Return true if the symbol has an auxiliary file name.
- `has_aux_function`: Return true if the symbol has an auxiliary function symbol.
- `has_aux_section`: Return true if the symbol has an auxiliary section symbol.
- `has_aux_weak_external`: Return true if the symbol has an auxiliary weak external symbol.
- `base_type`
- `derived_type`



## object::read::coff::symbol::SymbolIterator

*Struct*

An iterator for symbol entries in a COFF or PE file.

Yields the index and symbol structure for each symbol.

**Generic Parameters:**
- 'data
- 'table
- R
- Coff

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::coff::symbol::SymbolTable

*Struct*

A table of symbol entries in a COFF or PE file.

Also includes the string table used for the symbol names.

Returned by [`CoffHeader::symbols`] and
[`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols).

**Generic Parameters:**
- 'data
- R
- Coff

**Methods:**

- `fn parse(header: &Coff, data: R) -> Result<Self>` - Read the symbol table.
- `fn strings(self: &Self) -> StringTable<'data, R>` - Return the string table used for the symbol names.
- `fn is_empty(self: &Self) -> bool` - Return true if the symbol table is empty.
- `fn len(self: &Self) -> usize` - The number of symbol table entries.
- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` - Iterate over the symbols.
- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` - Return the symbol table entry at the given index.
- `fn aux_function(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` - Return the auxiliary function symbol for the symbol table entry at the given index.
- `fn aux_section(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` - Return the auxiliary section symbol for the symbol table entry at the given index.
- `fn aux_weak_external(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` - Return the auxiliary weak external symbol for the symbol table entry at the given index.
- `fn aux_file_name(self: &Self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` - Return the auxiliary file name for the symbol table entry at the given index.
- `fn get<T>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>` - Return the symbol table entry or auxiliary record at the given index and offset.
- `fn map<Entry, F>(self: &Self, f: F) -> SymbolMap<Entry>` - Construct a map from addresses to a user-defined map entry.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



