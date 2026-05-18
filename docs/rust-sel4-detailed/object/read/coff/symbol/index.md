*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Contents

- [Structs](#structs)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`CoffSymbolTable`](#coffsymboltable)
  - [`CoffSymbolIterator`](#coffsymboliterator)
  - [`CoffSymbol`](#coffsymbol)
- [Traits](#traits)
  - [`ImageSymbol`](#imagesymbol)
- [Type Aliases](#type-aliases)
  - [`CoffBigSymbolTable`](#coffbigsymboltable)
  - [`CoffBigSymbolIterator`](#coffbigsymboliterator)
  - [`CoffBigSymbol`](#coffbigsymbol)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in a COFF or PE file. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for symbol entries in a COFF or PE file. |
| [`CoffSymbolTable`](#coffsymboltable) | struct | A symbol table in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile). |
| [`CoffSymbolIterator`](#coffsymboliterator) | struct | An iterator for the symbols in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile). |
| [`CoffSymbol`](#coffsymbol) | struct | A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile). |
| [`ImageSymbol`](#imagesymbol) | trait | A trait for generic access to [`pe::ImageSymbol`] and [`pe::ImageSymbolEx`]. |
| [`CoffBigSymbolTable`](#coffbigsymboltable) | type | A symbol table in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSymbolIterator`](#coffbigsymboliterator) | type | An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSymbol`](#coffbigsymbol) | type | A symbol in a [`CoffBigFile`](super::CoffBigFile). |

## Structs

### `SymbolTable<'data, R, Coff>`

```rust
struct SymbolTable<'data, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    symbols: &'data [<Coff as >::ImageSymbolBytes],
    strings: crate::read::util::StringTable<'data, R>,
}
```

A table of symbol entries in a COFF or PE file.

Also includes the string table used for the symbol names.

Returned by `CoffHeader::symbols` and
[`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols).

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Read the symbol table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbol table entries.

  

  This includes auxiliary symbol table entries.

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](../index.md#symboliterator)

  Iterate over the symbols.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`CoffHeader`](../index.md#coffheader)

  Return the symbol table entry at the given index.

- <span id="symboltable-aux-function"></span>`fn aux_function(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ImageAuxSymbolFunction`](../../../pe/index.md#imageauxsymbolfunction)

  Return the auxiliary function symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-section"></span>`fn aux_section(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ImageAuxSymbolSection`](../../../pe/index.md#imageauxsymbolsection)

  Return the auxiliary section symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-weak-external"></span>`fn aux_weak_external(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ImageAuxSymbolWeak`](../../../pe/index.md#imageauxsymbolweak)

  Return the auxiliary weak external symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-file-name"></span>`fn aux_file_name(&self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result)

  Return the auxiliary file name for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result)

  Return the symbol table entry or auxiliary record at the given index and offset.

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../../index.md#symbolmap)

  Construct a map from addresses to a user-defined map entry.

#### Trait Implementations

##### `impl<R, Coff> Debug for SymbolTable<'data, R, Coff>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Default for SymbolTable<'data, R, Coff>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `SymbolIterator<'data, 'table, R, Coff>`

```rust
struct SymbolIterator<'data, 'table, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    symbols: &'table SymbolTable<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for symbol entries in a COFF or PE file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<R, Coff> Debug for SymbolIterator<'data, 'table, R, Coff>`

- <span id="symboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SymbolIterator<'data, 'table, R, Coff>`

- <span id="symboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for SymbolIterator<'data, 'table, R, Coff>`

- <span id="symboliterator-iterator-type-item"></span>`type Item = (SymbolIndex, &'data <Coff as CoffHeader>::ImageSymbol)`

- <span id="symboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffSymbolTable<'data, 'file, R, Coff>`

```rust
struct CoffSymbolTable<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
}
```

A symbol table in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

#### Trait Implementations

##### `impl<R, Coff> Clone for CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="coffsymboltable-clone"></span>`fn clone(&self) -> CoffSymbolTable<'data, 'file, R, Coff>` — [`CoffSymbolTable`](../index.md#coffsymboltable)

##### `impl<R, Coff> Copy for CoffSymbolTable<'data, 'file, R, Coff>`

##### `impl<R, Coff> Debug for CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="coffsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbolTable for CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="coffsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md#objectsymboltable)

- <span id="coffsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ObjectSymbolTable`](../../index.md#objectsymboltable)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbolTable<'data, 'file, R, Coff>`

### `CoffSymbolIterator<'data, 'file, R, Coff>`

```rust
struct CoffSymbolIterator<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

#### Implementations

- <span id="coffsymboliterator-new"></span>`fn new(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](../file/index.md#coffcommon)

- <span id="coffsymboliterator-empty"></span>`fn empty(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](../file/index.md#coffcommon)

#### Trait Implementations

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffsymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-iterator-type-item"></span>`type Item = CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffSymbol<'data, 'file, R, Coff>`

```rust
struct CoffSymbol<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Coff as >::ImageSymbol,
}
```

A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- <span id="coffsymbol-raw-symbol"></span>`fn raw_symbol(&self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](../index.md#coffheader)

  Get the raw `ImageSymbol` struct.

- <span id="coffsymbol-coff-symbol"></span>`fn coff_symbol(&self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](../index.md#coffheader)

  Get the raw `ImageSymbol` struct.

#### Trait Implementations

##### `impl<R, Coff> Clone for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-clone"></span>`fn clone(&self) -> CoffSymbol<'data, 'file, R, Coff>` — [`CoffSymbol`](../index.md#coffsymbol)

##### `impl<R, Coff> Copy for CoffSymbol<'data, 'file, R, Coff>`

##### `impl<R, Coff> Debug for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbol for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="coffsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="coffsymbol-objectsymbol-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="coffsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="coffsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="coffsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../../index.md#symbolkind)

- <span id="coffsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../../index.md#symbolsection)

- <span id="coffsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="coffsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../../index.md#symbolscope)

- <span id="coffsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="coffsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md#symbolflags), [`SectionIndex`](../../../index.md#sectionindex), [`SymbolIndex`](../../../index.md#symbolindex)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbol<'data, 'file, R, Coff>`

## Traits

### `ImageSymbol`

```rust
trait ImageSymbol: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageSymbol`](../../../pe/index.md) and [`pe::ImageSymbolEx`](../../../pe/index.md).

#### Required Methods

- `fn raw_name(&self) -> &[u8; 8]`

- `fn value(&self) -> u32`

- `fn section_number(&self) -> i32`

- `fn typ(&self) -> u16`

- `fn storage_class(&self) -> u8`

- `fn number_of_aux_symbols(&self) -> u8`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse a COFF symbol name.

- `fn address(&self, image_base: u64, sections: &SectionTable<'_>) -> Result<Option<u64>>`

  Return the symbol address.

- `fn section(&self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn has_aux_file_name(&self) -> bool`

  Return true if the symbol has an auxiliary file name.

- `fn has_aux_function(&self) -> bool`

  Return true if the symbol has an auxiliary function symbol.

- `fn has_aux_section(&self) -> bool`

  Return true if the symbol has an auxiliary section symbol.

- `fn has_aux_weak_external(&self) -> bool`

  Return true if the symbol has an auxiliary weak external symbol.

- `fn base_type(&self) -> u16`

- `fn derived_type(&self) -> u16`

#### Implementors

- [`ImageSymbolEx`](../../../pe/index.md#imagesymbolex)
- [`ImageSymbol`](../../../pe/index.md#imagesymbol)

## Type Aliases

### `CoffBigSymbolTable<'data, 'file, R>`

```rust
type CoffBigSymbolTable<'data, 'file, R> = CoffSymbolTable<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A symbol table in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSymbolIterator<'data, 'file, R>`

```rust
type CoffBigSymbolIterator<'data, 'file, R> = CoffSymbolIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSymbol<'data, 'file, R>`

```rust
type CoffBigSymbol<'data, 'file, R> = CoffSymbol<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A symbol in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

