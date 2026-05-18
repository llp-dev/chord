**object > read > wasm**

# Module: read::wasm

## Contents

**Structs**

- [`WasmComdat`](#wasmcomdat) - A COMDAT section group in a [`WasmFile`].
- [`WasmComdatIterator`](#wasmcomdatiterator) - An iterator for the COMDAT section groups in a [`WasmFile`].
- [`WasmComdatSectionIterator`](#wasmcomdatsectioniterator) - An iterator for the sections in a COMDAT section group in a [`WasmFile`].
- [`WasmFile`](#wasmfile) - A WebAssembly object file.
- [`WasmRelocationIterator`](#wasmrelocationiterator) - An iterator for the relocations for a [`WasmSection`].
- [`WasmSection`](#wasmsection) - A section in a [`WasmFile`].
- [`WasmSectionIterator`](#wasmsectioniterator) - An iterator for the sections in a [`WasmFile`].
- [`WasmSegment`](#wasmsegment) - A segment in a [`WasmFile`].
- [`WasmSegmentIterator`](#wasmsegmentiterator) - An iterator for the segments in a [`WasmFile`].
- [`WasmSymbol`](#wasmsymbol) - A symbol in a [`WasmFile`].
- [`WasmSymbolIterator`](#wasmsymboliterator) - An iterator for the symbols in a [`WasmFile`].
- [`WasmSymbolTable`](#wasmsymboltable) - A symbol table in a [`WasmFile`].

---

## object::read::wasm::WasmComdat

*Struct*

A COMDAT section group in a [`WasmFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **ObjectComdat**
  - `fn kind(self: &Self) -> ComdatKind`
  - `fn symbol(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn sections(self: &Self) -> <Self as >::SectionIterator`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::wasm::WasmComdatIterator

*Struct*

An iterator for the COMDAT section groups in a [`WasmFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::wasm::WasmComdatSectionIterator

*Struct*

An iterator for the sections in a COMDAT section group in a [`WasmFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::wasm::WasmFile

*Struct*

A WebAssembly object file.

**Generic Parameters:**
- 'data
- R

**Methods:**

- `fn parse(data: R) -> Result<Self>` - Parse the raw wasm data.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Object**
  - `fn architecture(self: &Self) -> Architecture`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn is_64(self: &Self) -> bool`
  - `fn kind(self: &Self) -> ObjectKind`
  - `fn segments(self: &Self) -> <Self as >::SegmentIterator`
  - `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<WasmSection<'data, 'file, R>>`
  - `fn section_by_index(self: &Self, index: SectionIndex) -> Result<WasmSection<'data, R>>`
  - `fn sections(self: &Self) -> <Self as >::SectionIterator`
  - `fn comdats(self: &Self) -> <Self as >::ComdatIterator`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<WasmSymbol<'data>>`
  - `fn symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn symbol_table(self: &Self) -> Option<WasmSymbolTable<'data>>`
  - `fn dynamic_symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn dynamic_symbol_table(self: &Self) -> Option<WasmSymbolTable<'data>>`
  - `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>`
  - `fn imports(self: &Self) -> Result<Vec<Import<'data>>>`
  - `fn exports(self: &Self) -> Result<Vec<Export<'data>>>`
  - `fn has_debug_symbols(self: &Self) -> bool`
  - `fn relative_address_base(self: &Self) -> u64`
  - `fn entry(self: &Self) -> u64`
  - `fn flags(self: &Self) -> FileFlags`



## object::read::wasm::WasmRelocationIterator

*Struct*

An iterator for the relocations for a [`WasmSection`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- R

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::wasm::WasmSection

*Struct*

A section in a [`WasmFile`].

Most functionality is provided by the [`ObjectSection`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **ObjectSection**
  - `fn index(self: &Self) -> SectionIndex`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> Option<(u64, u64)>`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>`
  - `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>`
  - `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn segment_name(self: &Self) -> Result<Option<&str>>`
  - `fn kind(self: &Self) -> SectionKind`
  - `fn relocations(self: &Self) -> WasmRelocationIterator<'data, 'file, R>`
  - `fn relocation_map(self: &Self) -> read::Result<RelocationMap>`
  - `fn flags(self: &Self) -> SectionFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::wasm::WasmSectionIterator

*Struct*

An iterator for the sections in a [`WasmFile`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::wasm::WasmSegment

*Struct*

A segment in a [`WasmFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **ObjectSegment**
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> (u64, u64)`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>`
  - `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn name(self: &Self) -> Result<Option<&str>>`
  - `fn flags(self: &Self) -> SegmentFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::wasm::WasmSegmentIterator

*Struct*

An iterator for the segments in a [`WasmFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::wasm::WasmSymbol

*Struct*

A symbol in a [`WasmFile`].

Most functionality is provided by the [`ObjectSymbol`] trait implementation.

**Generic Parameters:**
- 'data
- 'file

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
- **Clone**
  - `fn clone(self: &Self) -> WasmSymbol<'data, 'file>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::wasm::WasmSymbolIterator

*Struct*

An iterator for the symbols in a [`WasmFile`].

**Generic Parameters:**
- 'data
- 'file

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::wasm::WasmSymbolTable

*Struct*

A symbol table in a [`WasmFile`].

**Generic Parameters:**
- 'data
- 'file

**Trait Implementations:**

- **ObjectSymbolTable**
  - `fn symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



