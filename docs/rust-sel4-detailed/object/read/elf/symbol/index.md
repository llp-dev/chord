*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Contents

- [Structs](#structs)
  - [`SymbolTable`](#symboltable)
  - [`ElfSymbolTable`](#elfsymboltable)
  - [`ElfSymbolIterator`](#elfsymboliterator)
  - [`ElfSymbol`](#elfsymbol)
- [Traits](#traits)
  - [`Sym`](#sym)
- [Type Aliases](#type-aliases)
  - [`ElfSymbolTable32`](#elfsymboltable32)
  - [`ElfSymbolTable64`](#elfsymboltable64)
  - [`ElfSymbolIterator32`](#elfsymboliterator32)
  - [`ElfSymbolIterator64`](#elfsymboliterator64)
  - [`ElfSymbol32`](#elfsymbol32)
  - [`ElfSymbol64`](#elfsymbol64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in an ELF file. |
| [`ElfSymbolTable`](#elfsymboltable) | struct | A symbol table in an [`ElfFile`](super::ElfFile). |
| [`ElfSymbolIterator`](#elfsymboliterator) | struct | An iterator for the symbols in an [`ElfFile`](super::ElfFile). |
| [`ElfSymbol`](#elfsymbol) | struct | A symbol in an [`ElfFile`](super::ElfFile). |
| [`Sym`](#sym) | trait | A trait for generic access to [`elf::Sym32`] and [`elf::Sym64`]. |
| [`ElfSymbolTable32`](#elfsymboltable32) | type | A symbol table in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbolTable64`](#elfsymboltable64) | type | A symbol table in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbolIterator32`](#elfsymboliterator32) | type | An iterator for the symbols in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbolIterator64`](#elfsymboliterator64) | type | An iterator for the symbols in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSymbol32`](#elfsymbol32) | type | A symbol in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbol64`](#elfsymbol64) | type | A symbol in an [`ElfFile64`](super::ElfFile64). |

## Structs

### `SymbolTable<'data, Elf: FileHeader, R>`

```rust
struct SymbolTable<'data, Elf: FileHeader, R>
where
    R: ReadRef<'data> {
    section: crate::read::SectionIndex,
    string_section: crate::read::SectionIndex,
    shndx_section: crate::read::SectionIndex,
    symbols: &'data [<Elf as >::Sym],
    strings: crate::read::util::StringTable<'data, R>,
    shndx: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A table of symbol entries in an ELF file.

Also includes the string table used for the symbol names.

Returned by `SectionTable::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: R, sections: &SectionTable<'data, Elf, R>, section_index: SectionIndex, section: &<Elf as >::SectionHeader) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](../index.md#fileheader), [`SectionTable`](../index.md#sectiontable), [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`SymbolTable`](../index.md#symboltable)

  Parse the given symbol table section.

- <span id="symboltable-section"></span>`fn section(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

  Return the section index of this symbol table.

- <span id="symboltable-shndx-section"></span>`fn shndx_section(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

  Return the section index of the shndx table.

- <span id="symboltable-string-section"></span>`fn string_section(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

  Return the section index of the linked string table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-symbols"></span>`fn symbols(&self) -> &'data [<Elf as >::Sym]` — [`FileHeader`](../index.md#fileheader)

  Return the symbol table.

- <span id="symboltable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Elf as >::Sym>` — [`FileHeader`](../index.md#fileheader)

  Iterate over the symbols.

  

  This includes the null symbol at index 0, which you will usually need to skip.

- <span id="symboltable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SymbolIndex, &'data <Elf as >::Sym)>` — [`SymbolIndex`](../../../index.md#symbolindex), [`FileHeader`](../index.md#fileheader)

  Iterate over the symbols and their indices.

  

  This includes the null symbol at index 0, which you will usually need to skip.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbols.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> read::Result<&'data <Elf as >::Sym>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Get the symbol at the given index.

  

  Returns an error for null entry at index 0.

- <span id="symboltable-shndx"></span>`fn shndx(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> Option<u32>` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex)

  Return the extended section index for the given symbol if present.

- <span id="symboltable-symbol-section"></span>`fn symbol_section(&self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym, index: SymbolIndex) -> read::Result<Option<SectionIndex>>` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`SectionIndex`](../../../index.md#sectionindex)

  Return the section index for the given symbol.

  

  This uses the extended section index if present.

- <span id="symboltable-symbol-name"></span>`fn symbol_name(&self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym) -> read::Result<&'data [u8]>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

  Return the symbol name for the given symbol.

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Elf as >::Sym) -> Option<Entry>>(&self, endian: <Elf as >::Endian, f: F) -> SymbolMap<Entry>` — [`FileHeader`](../index.md#fileheader), [`SymbolMap`](../../../index.md#symbolmap)

  Construct a map from addresses to a user-defined map entry.

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader, R> Clone for SymbolTable<'data, Elf, R>`

- <span id="symboltable-clone"></span>`fn clone(&self) -> SymbolTable<'data, Elf, R>` — [`SymbolTable`](../index.md#symboltable)

##### `impl<Elf: marker::Copy + FileHeader, R> Copy for SymbolTable<'data, Elf, R>`

##### `impl<Elf: fmt::Debug + FileHeader, R> Debug for SymbolTable<'data, Elf, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Elf, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `ElfSymbolTable<'data, 'file, Elf, R>`

```rust
struct ElfSymbolTable<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
}
```

A symbol table in an [`ElfFile`](super::ElfFile).

#### Trait Implementations

##### `impl<Elf, R> Clone for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-clone"></span>`fn clone(&self) -> ElfSymbolTable<'data, 'file, Elf, R>` — [`ElfSymbolTable`](../index.md#elfsymboltable)

##### `impl<Elf, R> Copy for ElfSymbolTable<'data, 'file, Elf, R>`

##### `impl<Elf, R> Debug for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md#objectsymboltable)

- <span id="elfsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ObjectSymbolTable`](../../index.md#objectsymboltable)

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbolTable<'data, 'file, Elf, R>`

### `ElfSymbolIterator<'data, 'file, Elf, R>`

```rust
struct ElfSymbolIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in an [`ElfFile`](super::ElfFile).

#### Implementations

- <span id="elfsymboliterator-new"></span>`fn new(endian: <Elf as >::Endian, symbols: &'file SymbolTable<'data, Elf, R>) -> Self` — [`FileHeader`](../index.md#fileheader), [`SymbolTable`](../index.md#symboltable)

#### Trait Implementations

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Debug for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Iterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-iterator-type-item"></span>`type Item = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ElfSymbol<'data, 'file, Elf, R>`

```rust
struct ElfSymbol<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Elf as >::Sym,
}
```

A symbol in an [`ElfFile`](super::ElfFile).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- <span id="elfsymbol-endian"></span>`fn endian(&self) -> <Elf as >::Endian` — [`FileHeader`](../index.md#fileheader)

  Get the endianness of the ELF file.

- <span id="elfsymbol-raw-symbol"></span>`fn raw_symbol(&self) -> &'data <Elf as >::Sym` — [`FileHeader`](../index.md#fileheader)

  Return a reference to the raw symbol structure.

- <span id="elfsymbol-elf-symbol"></span>`fn elf_symbol(&self) -> &'data <Elf as >::Sym` — [`FileHeader`](../index.md#fileheader)

  Get the raw ELF symbol structure.

#### Trait Implementations

##### `impl<Elf, R> Clone for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-clone"></span>`fn clone(&self) -> ElfSymbol<'data, 'file, Elf, R>` — [`ElfSymbol`](../index.md#elfsymbol)

##### `impl<Elf, R> Copy for ElfSymbol<'data, 'file, Elf, R>`

##### `impl<Elf, R> Debug for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> ObjectSymbol for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="elfsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="elfsymbol-objectsymbol-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="elfsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="elfsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="elfsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../../index.md#symbolkind)

- <span id="elfsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../../index.md#symbolsection)

- <span id="elfsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="elfsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../../index.md#symbolscope)

- <span id="elfsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="elfsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md#symbolflags), [`SectionIndex`](../../../index.md#sectionindex), [`SymbolIndex`](../../../index.md#symbolindex)

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbol<'data, 'file, Elf, R>`

## Traits

### `Sym`

```rust
trait Sym: Debug + Pod { ... }
```

A trait for generic access to [`elf::Sym32`](../../../elf/index.md) and [`elf::Sym64`](../../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn st_name(&self, endian: <Self as >::Endian) -> u32`

- `fn st_info(&self) -> u8`

- `fn st_bind(&self) -> u8`

- `fn st_type(&self) -> u8`

- `fn st_other(&self) -> u8`

- `fn st_visibility(&self) -> u8`

- `fn st_shndx(&self, endian: <Self as >::Endian) -> u16`

- `fn st_value(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn st_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> read::Result<&'data [u8]>`

  Parse the symbol name from the string table.

- `fn is_undefined(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_UNDEF`.

- `fn is_definition(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn is_common(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_COMMON`.

- `fn is_absolute(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_ABS`.

- `fn is_local(&self) -> bool`

  Return true if the symbol binding is `STB_LOCAL`.

- `fn is_weak(&self) -> bool`

  Return true if the symbol binding is `STB_WEAK`.

#### Implementors

- [`Sym32`](../../../elf/index.md#sym32)
- [`Sym64`](../../../elf/index.md#sym64)

## Type Aliases

### `ElfSymbolTable32<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable32<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolTable64<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable64<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator32<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator32<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator64<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator64<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile64`](super::ElfFile64).

### `ElfSymbol32<'data, 'file, Endian, R>`

```rust
type ElfSymbol32<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbol64<'data, 'file, Endian, R>`

```rust
type ElfSymbol64<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol in an [`ElfFile64`](super::ElfFile64).

