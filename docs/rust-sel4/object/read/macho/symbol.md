**object > read > macho > symbol**

# Module: read::macho::symbol

## Contents

**Structs**

- [`MachOSymbol`](#machosymbol) - A symbol in a [`MachOFile`].
- [`MachOSymbolIterator`](#machosymboliterator) - An iterator for the symbols in a [`MachOFile`].
- [`MachOSymbolTable`](#machosymboltable) - A symbol table in a [`MachOFile`].
- [`SymbolTable`](#symboltable) - A table of symbol entries in a Mach-O file.

**Traits**

- [`Nlist`](#nlist) - A trait for generic access to [`macho::Nlist32`] and [`macho::Nlist64`].

**Type Aliases**

- [`MachOSymbol32`](#machosymbol32) - A symbol in a [`MachOFile32`](super::MachOFile32).
- [`MachOSymbol64`](#machosymbol64) - A symbol in a [`MachOFile64`](super::MachOFile64).
- [`MachOSymbolIterator32`](#machosymboliterator32) - An iterator for the symbols in a [`MachOFile32`](super::MachOFile32).
- [`MachOSymbolIterator64`](#machosymboliterator64) - An iterator for the symbols in a [`MachOFile64`](super::MachOFile64).
- [`MachOSymbolTable32`](#machosymboltable32) - A symbol table in a [`MachOFile32`](super::MachOFile32).
- [`MachOSymbolTable64`](#machosymboltable64) - A symbol table in a [`MachOFile64`](super::MachOFile64).

---

## object::read::macho::symbol::MachOSymbol

*Struct*

A symbol in a [`MachOFile`].

Most functionality is provided by the [`ObjectSymbol`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Methods:**

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` - Get the Mach-O file containing this symbol.
- `fn macho_symbol(self: &Self) -> &'data <Mach as >::Nlist` - Get the raw Mach-O symbol structure.

**Traits:** Copy

**Trait Implementations:**

- **ObjectSymbol**
  - `fn index(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
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
  - `fn clone(self: &Self) -> MachOSymbol<'data, 'file, Mach, R>`



## object::read::macho::symbol::MachOSymbol32

*Type Alias*: `MachOSymbol<'data, 'file, macho::MachHeader32<Endian>, R>`

A symbol in a [`MachOFile32`](super::MachOFile32).



## object::read::macho::symbol::MachOSymbol64

*Type Alias*: `MachOSymbol<'data, 'file, macho::MachHeader64<Endian>, R>`

A symbol in a [`MachOFile64`](super::MachOFile64).



## object::read::macho::symbol::MachOSymbolIterator

*Struct*

An iterator for the symbols in a [`MachOFile`].

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::macho::symbol::MachOSymbolIterator32

*Type Alias*: `MachOSymbolIterator<'data, 'file, macho::MachHeader32<Endian>, R>`

An iterator for the symbols in a [`MachOFile32`](super::MachOFile32).



## object::read::macho::symbol::MachOSymbolIterator64

*Type Alias*: `MachOSymbolIterator<'data, 'file, macho::MachHeader64<Endian>, R>`

An iterator for the symbols in a [`MachOFile64`](super::MachOFile64).



## object::read::macho::symbol::MachOSymbolTable

*Struct*

A symbol table in a [`MachOFile`].

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MachOSymbolTable<'data, 'file, Mach, R>`
- **ObjectSymbolTable**
  - `fn symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`



## object::read::macho::symbol::MachOSymbolTable32

*Type Alias*: `MachOSymbolTable<'data, 'file, macho::MachHeader32<Endian>, R>`

A symbol table in a [`MachOFile32`](super::MachOFile32).



## object::read::macho::symbol::MachOSymbolTable64

*Type Alias*: `MachOSymbolTable<'data, 'file, macho::MachHeader64<Endian>, R>`

A symbol table in a [`MachOFile64`](super::MachOFile64).



## object::read::macho::symbol::Nlist

*Trait*

A trait for generic access to [`macho::Nlist32`] and [`macho::Nlist64`].

**Methods:**

- `Word`
- `Endian`
- `n_strx`
- `n_type`
- `n_sect`
- `n_desc`
- `n_value`
- `name`
- `is_stab`: Return true if this is a STAB symbol.
- `is_undefined`: Return true if this is an undefined symbol.
- `is_definition`: Return true if the symbol is a definition of a function or data object.
- `library_ordinal`: Return the library ordinal.



## object::read::macho::symbol::SymbolTable

*Struct*

A table of symbol entries in a Mach-O file.

Also includes the string table used for the symbol names.

Returned by [`macho::SymtabCommand::symbols`].

**Generic Parameters:**
- 'data
- Mach
- R

**Methods:**

- `fn strings(self: &Self) -> StringTable<'data, R>` - Return the string table used for the symbol names.
- `fn iter(self: &Self) -> slice::Iter<'data, <Mach as >::Nlist>` - Iterate over the symbols.
- `fn is_empty(self: &Self) -> bool` - Return true if the symbol table is empty.
- `fn len(self: &Self) -> usize` - The number of symbols.
- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Mach as >::Nlist>` - Return the symbol at the given index.
- `fn map<Entry, F>(self: &Self, f: F) -> SymbolMap<Entry>` - Construct a map from addresses to a user-defined map entry.
- `fn object_map(self: &Self, endian: <Mach as >::Endian) -> ObjectMap<'data>` - Construct a map from addresses to symbol names and object file names.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SymbolTable<'data, Mach, R>`
- **Default**
  - `fn default() -> Self`



