*[object](../../index.md) / [read](../index.md) / [wasm](index.md)*

---

# Module `wasm`

Support for reading Wasm files.

[`WasmFile`](#wasmfile) implements the [`Object`](../index.md) trait for Wasm files.

## Contents

- [Structs](#structs)
  - [`WasmFile`](#wasmfile)
  - [`SectionHeader`](#sectionheader)
  - [`WasmSegmentIterator`](#wasmsegmentiterator)
  - [`WasmSegment`](#wasmsegment)
  - [`WasmSectionIterator`](#wasmsectioniterator)
  - [`WasmSection`](#wasmsection)
  - [`WasmComdatIterator`](#wasmcomdatiterator)
  - [`WasmComdat`](#wasmcomdat)
  - [`WasmComdatSectionIterator`](#wasmcomdatsectioniterator)
  - [`WasmSymbolTable`](#wasmsymboltable)
  - [`WasmSymbolIterator`](#wasmsymboliterator)
  - [`WasmSymbol`](#wasmsymbol)
  - [`WasmSymbolInternal`](#wasmsymbolinternal)
  - [`WasmRelocationIterator`](#wasmrelocationiterator)
- [Enums](#enums)
  - [`SectionId`](#sectionid)
  - [`LocalFunctionKind`](#localfunctionkind)
- [Constants](#constants)
  - [`MAX_SECTION_ID`](#max-section-id)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WasmFile`](#wasmfile) | struct | A WebAssembly object file. |
| [`SectionHeader`](#sectionheader) | struct |  |
| [`WasmSegmentIterator`](#wasmsegmentiterator) | struct | An iterator for the segments in a [`WasmFile`]. |
| [`WasmSegment`](#wasmsegment) | struct | A segment in a [`WasmFile`]. |
| [`WasmSectionIterator`](#wasmsectioniterator) | struct | An iterator for the sections in a [`WasmFile`]. |
| [`WasmSection`](#wasmsection) | struct | A section in a [`WasmFile`]. |
| [`WasmComdatIterator`](#wasmcomdatiterator) | struct | An iterator for the COMDAT section groups in a [`WasmFile`]. |
| [`WasmComdat`](#wasmcomdat) | struct | A COMDAT section group in a [`WasmFile`]. |
| [`WasmComdatSectionIterator`](#wasmcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`WasmFile`]. |
| [`WasmSymbolTable`](#wasmsymboltable) | struct | A symbol table in a [`WasmFile`]. |
| [`WasmSymbolIterator`](#wasmsymboliterator) | struct | An iterator for the symbols in a [`WasmFile`]. |
| [`WasmSymbol`](#wasmsymbol) | struct | A symbol in a [`WasmFile`]. |
| [`WasmSymbolInternal`](#wasmsymbolinternal) | struct |  |
| [`WasmRelocationIterator`](#wasmrelocationiterator) | struct | An iterator for the relocations for a [`WasmSection`]. |
| [`SectionId`](#sectionid) | enum |  |
| [`LocalFunctionKind`](#localfunctionkind) | enum |  |
| [`MAX_SECTION_ID`](#max-section-id) | const |  |

## Structs

### `WasmFile<'data, R>`

```rust
struct WasmFile<'data, R> {
    data: &'data [u8],
    has_memory64: bool,
    sections: alloc::vec::Vec<SectionHeader<'data>>,
    id_sections: alloc::boxed::Box<[Option<usize>; 14]>,
    has_debug_symbols: bool,
    symbols: alloc::vec::Vec<WasmSymbolInternal<'data>>,
    entry: u64,
    marker: core::marker::PhantomData<R>,
}
```

A WebAssembly object file.

#### Implementations

- <span id="wasmfile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw wasm data.

- <span id="wasmfile-add-section"></span>`fn add_section(&mut self, id: SectionId, range: Range<usize>, name: &'data str)` — [`SectionId`](#sectionid)

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmFile<'data, R>`

- <span id="wasmfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> Object for WasmFile<'data, R>`

- <span id="wasmfile-object-type-segment"></span>`type Segment = WasmSegment<'data, 'file, R>`

- <span id="wasmfile-object-type-segmentiterator"></span>`type SegmentIterator = WasmSegmentIterator<'data, 'file, R>`

- <span id="wasmfile-object-type-section"></span>`type Section = WasmSection<'data, 'file, R>`

- <span id="wasmfile-object-type-sectioniterator"></span>`type SectionIterator = WasmSectionIterator<'data, 'file, R>`

- <span id="wasmfile-object-type-comdat"></span>`type Comdat = WasmComdat<'data, 'file, R>`

- <span id="wasmfile-object-type-comdatiterator"></span>`type ComdatIterator = WasmComdatIterator<'data, 'file, R>`

- <span id="wasmfile-object-type-symbol"></span>`type Symbol = WasmSymbol<'data, 'file>`

- <span id="wasmfile-object-type-symboliterator"></span>`type SymbolIterator = WasmSymbolIterator<'data, 'file>`

- <span id="wasmfile-object-type-symboltable"></span>`type SymbolTable = WasmSymbolTable<'data, 'file>`

- <span id="wasmfile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="wasmfile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="wasmfile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="wasmfile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="wasmfile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="wasmfile-object-segments"></span>`fn segments(&self) -> <Self as >::SegmentIterator` — [`Object`](../index.md#object)

- <span id="wasmfile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<WasmSection<'data, 'file, R>>` — [`WasmSection`](#wasmsection)

- <span id="wasmfile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<WasmSection<'data, '_, R>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`WasmSection`](#wasmsection)

- <span id="wasmfile-object-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`Object`](../index.md#object)

- <span id="wasmfile-object-comdats"></span>`fn comdats(&self) -> <Self as >::ComdatIterator` — [`Object`](../index.md#object)

- <span id="wasmfile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<WasmSymbol<'data, '_>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`WasmSymbol`](#wasmsymbol)

- <span id="wasmfile-object-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`Object`](../index.md#object)

- <span id="wasmfile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<WasmSymbolTable<'data, '_>>` — [`WasmSymbolTable`](#wasmsymboltable)

- <span id="wasmfile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> <Self as >::SymbolIterator` — [`Object`](../index.md#object)

- <span id="wasmfile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<WasmSymbolTable<'data, '_>>` — [`WasmSymbolTable`](#wasmsymboltable)

- <span id="wasmfile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md#nodynamicrelocationiterator)

- <span id="wasmfile-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="wasmfile-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="wasmfile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="wasmfile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="wasmfile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="wasmfile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<R> Sealed for WasmFile<'data, R>`

### `SectionHeader<'data>`

```rust
struct SectionHeader<'data> {
    id: SectionId,
    range: core::ops::Range<usize>,
    name: &'data str,
}
```

#### Trait Implementations

##### `impl Debug for SectionHeader<'data>`

- <span id="sectionheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WasmSegmentIterator<'data, 'file, R>`

```rust
struct WasmSegmentIterator<'data, 'file, R> {
    file: &'file WasmFile<'data, R>,
}
```

An iterator for the segments in a [`WasmFile`](#wasmfile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmSegmentIterator<'data, 'file, R>`

- <span id="wasmsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for WasmSegmentIterator<'data, 'file, R>`

- <span id="wasmsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="wasmsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="wasmsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for WasmSegmentIterator<'data, 'file, R>`

- <span id="wasmsegmentiterator-iterator-type-item"></span>`type Item = WasmSegment<'data, 'file, R>`

- <span id="wasmsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `WasmSegment<'data, 'file, R>`

```rust
struct WasmSegment<'data, 'file, R> {
    file: &'file WasmFile<'data, R>,
}
```

A segment in a [`WasmFile`](#wasmfile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmSegment<'data, 'file, R>`

- <span id="wasmsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R> ObjectSegment for WasmSegment<'data, 'file, R>`

- <span id="wasmsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="wasmsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="wasmsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="wasmsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="wasmsegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="wasmsegment-objectsegment-data-range"></span>`fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="wasmsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="wasmsegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="wasmsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<R> Sealed for WasmSegment<'data, 'file, R>`

### `WasmSectionIterator<'data, 'file, R>`

```rust
struct WasmSectionIterator<'data, 'file, R> {
    file: &'file WasmFile<'data, R>,
    sections: slice::Iter<'file, SectionHeader<'data>>,
}
```

An iterator for the sections in a [`WasmFile`](#wasmfile).

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmSectionIterator<'data, 'file, R>`

- <span id="wasmsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for WasmSectionIterator<'data, 'file, R>`

- <span id="wasmsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="wasmsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="wasmsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for WasmSectionIterator<'data, 'file, R>`

- <span id="wasmsectioniterator-iterator-type-item"></span>`type Item = WasmSection<'data, 'file, R>`

- <span id="wasmsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `WasmSection<'data, 'file, R>`

```rust
struct WasmSection<'data, 'file, R> {
    file: &'file WasmFile<'data, R>,
    section: &'file SectionHeader<'data>,
}
```

A section in a [`WasmFile`](#wasmfile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmSection<'data, 'file, R>`

- <span id="wasmsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> ObjectSection for WasmSection<'data, 'file, R>`

- <span id="wasmsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = WasmRelocationIterator<'data, 'file, R>`

- <span id="wasmsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="wasmsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="wasmsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="wasmsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="wasmsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="wasmsection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="wasmsection-objectsection-data-range"></span>`fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="wasmsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="wasmsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="wasmsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="wasmsection-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="wasmsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="wasmsection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="wasmsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="wasmsection-objectsection-relocations"></span>`fn relocations(&self) -> WasmRelocationIterator<'data, 'file, R>` — [`WasmRelocationIterator`](#wasmrelocationiterator)

- <span id="wasmsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="wasmsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

##### `impl<R> Sealed for WasmSection<'data, 'file, R>`

### `WasmComdatIterator<'data, 'file, R>`

```rust
struct WasmComdatIterator<'data, 'file, R> {
    file: &'file WasmFile<'data, R>,
}
```

An iterator for the COMDAT section groups in a [`WasmFile`](#wasmfile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmComdatIterator<'data, 'file, R>`

- <span id="wasmcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for WasmComdatIterator<'data, 'file, R>`

- <span id="wasmcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="wasmcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="wasmcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for WasmComdatIterator<'data, 'file, R>`

- <span id="wasmcomdatiterator-iterator-type-item"></span>`type Item = WasmComdat<'data, 'file, R>`

- <span id="wasmcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `WasmComdat<'data, 'file, R>`

```rust
struct WasmComdat<'data, 'file, R> {
    file: &'file WasmFile<'data, R>,
}
```

A COMDAT section group in a [`WasmFile`](#wasmfile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmComdat<'data, 'file, R>`

- <span id="wasmcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R> ObjectComdat for WasmComdat<'data, 'file, R>`

- <span id="wasmcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = WasmComdatSectionIterator<'data, 'file, R>`

- <span id="wasmcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="wasmcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="wasmcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="wasmcomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="wasmcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md#objectcomdat)

##### `impl<R> Sealed for WasmComdat<'data, 'file, R>`

### `WasmComdatSectionIterator<'data, 'file, R>`

```rust
struct WasmComdatSectionIterator<'data, 'file, R> {
    file: &'file WasmFile<'data, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`WasmFile`](#wasmfile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmComdatSectionIterator<'data, 'file, R>`

- <span id="wasmcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for WasmComdatSectionIterator<'data, 'file, R>`

- <span id="wasmcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="wasmcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="wasmcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for WasmComdatSectionIterator<'data, 'file, R>`

- <span id="wasmcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="wasmcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `WasmSymbolTable<'data, 'file>`

```rust
struct WasmSymbolTable<'data, 'file> {
    symbols: &'file [WasmSymbolInternal<'data>],
}
```

A symbol table in a [`WasmFile`](#wasmfile).

#### Trait Implementations

##### `impl Debug for WasmSymbolTable<'data, 'file>`

- <span id="wasmsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ObjectSymbolTable for WasmSymbolTable<'data, 'file>`

- <span id="wasmsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = WasmSymbol<'data, 'file>`

- <span id="wasmsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = WasmSymbolIterator<'data, 'file>`

- <span id="wasmsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md#objectsymboltable)

- <span id="wasmsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ObjectSymbolTable`](../index.md#objectsymboltable)

##### `impl Sealed for WasmSymbolTable<'data, 'file>`

### `WasmSymbolIterator<'data, 'file>`

```rust
struct WasmSymbolIterator<'data, 'file> {
    symbols: core::iter::Enumerate<slice::Iter<'file, WasmSymbolInternal<'data>>>,
}
```

An iterator for the symbols in a [`WasmFile`](#wasmfile).

#### Trait Implementations

##### `impl Debug for WasmSymbolIterator<'data, 'file>`

- <span id="wasmsymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for WasmSymbolIterator<'data, 'file>`

- <span id="wasmsymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="wasmsymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="wasmsymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WasmSymbolIterator<'data, 'file>`

- <span id="wasmsymboliterator-iterator-type-item"></span>`type Item = WasmSymbol<'data, 'file>`

- <span id="wasmsymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `WasmSymbol<'data, 'file>`

```rust
struct WasmSymbol<'data, 'file> {
    index: crate::read::SymbolIndex,
    symbol: &'file WasmSymbolInternal<'data>,
}
```

A symbol in a [`WasmFile`](#wasmfile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Trait Implementations

##### `impl Clone for WasmSymbol<'data, 'file>`

- <span id="wasmsymbol-clone"></span>`fn clone(&self) -> WasmSymbol<'data, 'file>` — [`WasmSymbol`](#wasmsymbol)

##### `impl Copy for WasmSymbol<'data, 'file>`

##### `impl Debug for WasmSymbol<'data, 'file>`

- <span id="wasmsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ObjectSymbol for WasmSymbol<'data, 'file>`

- <span id="wasmsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="wasmsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="wasmsymbol-objectsymbol-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="wasmsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="wasmsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="wasmsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md#symbolkind)

- <span id="wasmsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md#symbolsection)

- <span id="wasmsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="wasmsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="wasmsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="wasmsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="wasmsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md#symbolscope)

- <span id="wasmsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="wasmsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="wasmsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md#symbolflags), [`SectionIndex`](../../index.md#sectionindex), [`SymbolIndex`](../../index.md#symbolindex)

##### `impl Sealed for WasmSymbol<'data, 'file>`

### `WasmSymbolInternal<'data>`

```rust
struct WasmSymbolInternal<'data> {
    name: &'data str,
    address: u64,
    size: u64,
    kind: crate::read::SymbolKind,
    section: crate::read::SymbolSection,
    scope: crate::read::SymbolScope,
    weak: bool,
}
```

#### Trait Implementations

##### `impl Clone for WasmSymbolInternal<'data>`

- <span id="wasmsymbolinternal-clone"></span>`fn clone(&self) -> WasmSymbolInternal<'data>` — [`WasmSymbolInternal`](#wasmsymbolinternal)

##### `impl Debug for WasmSymbolInternal<'data>`

- <span id="wasmsymbolinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WasmRelocationIterator<'data, 'file, R>`

```rust
struct WasmRelocationIterator<'data, 'file, R>(core::marker::PhantomData<(&'data (), &'file (), R)>);
```

An iterator for the relocations for a [`WasmSection`](#wasmsection).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for WasmRelocationIterator<'data, 'file, R>`

- <span id="wasmrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for WasmRelocationIterator<'data, 'file, R>`

- <span id="wasmrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="wasmrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="wasmrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for WasmRelocationIterator<'data, 'file, R>`

- <span id="wasmrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="wasmrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `SectionId`

```rust
enum SectionId {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    DataCount,
    Tag,
}
```

#### Trait Implementations

##### `impl Clone for SectionId`

- <span id="sectionid-clone"></span>`fn clone(&self) -> SectionId` — [`SectionId`](#sectionid)

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- <span id="sectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionId`

##### `impl<K> Equivalent for SectionId`

- <span id="sectionid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for SectionId`

- <span id="sectionid-partialeq-eq"></span>`fn eq(&self, other: &SectionId) -> bool` — [`SectionId`](#sectionid)

##### `impl StructuralPartialEq for SectionId`

### `LocalFunctionKind`

```rust
enum LocalFunctionKind {
    Unknown,
    Exported,
}
```

#### Trait Implementations

##### `impl Clone for LocalFunctionKind`

- <span id="localfunctionkind-clone"></span>`fn clone(&self) -> LocalFunctionKind` — [`LocalFunctionKind`](#localfunctionkind)

## Constants

### `MAX_SECTION_ID`
```rust
const MAX_SECTION_ID: usize = 13usize;
```

