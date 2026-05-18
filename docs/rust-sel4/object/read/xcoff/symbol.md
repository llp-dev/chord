**object > read > xcoff > symbol**

# Module: read::xcoff::symbol

## Contents

**Structs**

- [`SymbolIterator`](#symboliterator) - An iterator for symbol entries in an XCOFF file.
- [`SymbolTable`](#symboltable) - A table of symbol entries in an XCOFF file.
- [`XcoffSymbol`](#xcoffsymbol) - A symbol in an [`XcoffFile`].
- [`XcoffSymbolIterator`](#xcoffsymboliterator) - An iterator for the symbols in an [`XcoffFile`].
- [`XcoffSymbolTable`](#xcoffsymboltable) - A symbol table in an [`XcoffFile`].

**Traits**

- [`CsectAux`](#csectaux) - A trait for generic access to [`xcoff::CsectAux32`] and [`xcoff::CsectAux64`].
- [`FileAux`](#fileaux) - A trait for generic access to [`xcoff::FileAux32`] and [`xcoff::FileAux64`].
- [`Symbol`](#symbol) - A trait for generic access to [`xcoff::Symbol32`] and [`xcoff::Symbol64`].

**Type Aliases**

- [`XcoffSymbol32`](#xcoffsymbol32) - A symbol in an [`XcoffFile32`](super::XcoffFile32).
- [`XcoffSymbol64`](#xcoffsymbol64) - A symbol in an [`XcoffFile64`](super::XcoffFile64).
- [`XcoffSymbolIterator32`](#xcoffsymboliterator32) - An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).
- [`XcoffSymbolIterator64`](#xcoffsymboliterator64) - An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).
- [`XcoffSymbolTable32`](#xcoffsymboltable32) - A symbol table in an [`XcoffFile32`](super::XcoffFile32).
- [`XcoffSymbolTable64`](#xcoffsymboltable64) - A symbol table in an [`XcoffFile64`](super::XcoffFile64).

---

## object::read::xcoff::symbol::CsectAux

*Trait*

A trait for generic access to [`xcoff::CsectAux32`] and [`xcoff::CsectAux64`].

**Methods:**

- `x_scnlen`
- `x_parmhash`
- `x_snhash`
- `x_smtyp`
- `x_smclas`
- `x_stab`
- `x_snstab`
- `x_auxtype`
- `alignment`
- `sym_type`



## object::read::xcoff::symbol::FileAux

*Trait*

A trait for generic access to [`xcoff::FileAux32`] and [`xcoff::FileAux64`].

**Methods:**

- `x_fname`
- `x_ftype`
- `x_auxtype`
- `name_offset`
- `fname`: Parse the x_fname field, which may be an inline string or a string table offset.



## object::read::xcoff::symbol::Symbol

*Trait*

A trait for generic access to [`xcoff::Symbol32`] and [`xcoff::Symbol64`].

**Methods:**

- `Word`
- `n_value`
- `n_scnum`
- `n_type`
- `n_sclass`
- `n_numaux`
- `name_offset`
- `name`
- `section`: Return the section index for the symbol.
- `is_null`: Return true if the symbol is a null placeholder.
- `is_undefined`: Return true if the symbol is undefined.
- `has_aux_file`: Return true if the symbol has file auxiliary entry.
- `has_aux_csect`: Return true if the symbol has csect auxiliary entry.



## object::read::xcoff::symbol::SymbolIterator

*Struct*

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

**Generic Parameters:**
- 'data
- 'table
- Xcoff
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::xcoff::symbol::SymbolTable

*Struct*

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by [`FileHeader::symbols`].

**Generic Parameters:**
- 'data
- Xcoff
- R

**Methods:**

- `fn parse(header: Xcoff, data: R) -> Result<Self>` - Parse the symbol table.
- `fn strings(self: &Self) -> StringTable<'data, R>` - Return the string table used for the symbol names.
- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` - Iterate over the symbols.
- `fn get<T>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>` - Return the symbol entry at the given index and offset.
- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` - Get the symbol at the given index.
- `fn aux_file(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` - Return a file auxiliary symbol.
- `fn aux_csect(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` - Return the csect auxiliary symbol.
- `fn is_empty(self: &Self) -> bool` - Return true if the symbol table is empty.
- `fn len(self: &Self) -> usize` - The number of symbol table entries.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## object::read::xcoff::symbol::XcoffSymbol

*Struct*

A symbol in an [`XcoffFile`].

Most functionality is provided by the [`ObjectSymbol`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Methods:**

- `fn xcoff_file(self: &Self) -> &'file XcoffFile<'data, Xcoff, R>` - Get the XCOFF file containing this symbol.
- `fn xcoff_symbol(self: &Self) -> &'data <Xcoff as >::Symbol` - Get the raw XCOFF symbol structure.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> XcoffSymbol<'data, 'file, Xcoff, R>`
- **ObjectSymbol**
  - `fn index(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn kind(self: &Self) -> SymbolKind`
  - `fn section(self: &Self) -> SymbolSection`
  - `fn is_undefined(self: &Self) -> bool`
  - `fn is_definition(self: &Self) -> bool` - Return true if the symbol is a definition of a function or data object.
  - `fn is_common(self: &Self) -> bool`
  - `fn is_weak(self: &Self) -> bool`
  - `fn scope(self: &Self) -> SymbolScope`
  - `fn is_global(self: &Self) -> bool`
  - `fn is_local(self: &Self) -> bool`
  - `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::xcoff::symbol::XcoffSymbol32

*Type Alias*: `XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>`

A symbol in an [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::symbol::XcoffSymbol64

*Type Alias*: `XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>`

A symbol in an [`XcoffFile64`](super::XcoffFile64).



## object::read::xcoff::symbol::XcoffSymbolIterator

*Struct*

An iterator for the symbols in an [`XcoffFile`].

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::xcoff::symbol::XcoffSymbolIterator32

*Type Alias*: `XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>`

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::symbol::XcoffSymbolIterator64

*Type Alias*: `XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>`

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).



## object::read::xcoff::symbol::XcoffSymbolTable

*Struct*

A symbol table in an [`XcoffFile`].

**Generic Parameters:**
- 'data
- 'file
- Xcoff
- R

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>`
- **ObjectSymbolTable**
  - `fn symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>`



## object::read::xcoff::symbol::XcoffSymbolTable32

*Type Alias*: `XcoffSymbolTable<'data, 'file, xcoff::FileHeader32, R>`

A symbol table in an [`XcoffFile32`](super::XcoffFile32).



## object::read::xcoff::symbol::XcoffSymbolTable64

*Type Alias*: `XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>`

A symbol table in an [`XcoffFile64`](super::XcoffFile64).



