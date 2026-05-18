*[object](../../index.md) / [read](../index.md) / [any](index.md)*

---

# Module `any`

## Contents

- [Structs](#structs)
  - [`SegmentIterator`](#segmentiterator)
  - [`Segment`](#segment)
  - [`SectionIterator`](#sectioniterator)
  - [`Section`](#section)
  - [`ComdatIterator`](#comdatiterator)
  - [`Comdat`](#comdat)
  - [`ComdatSectionIterator`](#comdatsectioniterator)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`Symbol`](#symbol)
  - [`DynamicRelocationIterator`](#dynamicrelocationiterator)
  - [`SectionRelocationIterator`](#sectionrelocationiterator)
- [Enums](#enums)
  - [`File`](#file)
  - [`SegmentIteratorInternal`](#segmentiteratorinternal)
  - [`SegmentInternal`](#segmentinternal)
  - [`SectionIteratorInternal`](#sectioniteratorinternal)
  - [`SectionInternal`](#sectioninternal)
  - [`ComdatIteratorInternal`](#comdatiteratorinternal)
  - [`ComdatInternal`](#comdatinternal)
  - [`ComdatSectionIteratorInternal`](#comdatsectioniteratorinternal)
  - [`SymbolTableInternal`](#symboltableinternal)
  - [`SymbolIteratorInternal`](#symboliteratorinternal)
  - [`SymbolInternal`](#symbolinternal)
  - [`DynamicRelocationIteratorInternal`](#dynamicrelocationiteratorinternal)
  - [`SectionRelocationIteratorInternal`](#sectionrelocationiteratorinternal)
- [Macros](#macros)
  - [`with_inner!`](#with-inner)
  - [`with_inner_mut!`](#with-inner-mut)
  - [`map_inner!`](#map-inner)
  - [`map_inner_option!`](#map-inner-option)
  - [`map_inner_option_mut!`](#map-inner-option-mut)
  - [`next_inner!`](#next-inner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SegmentIterator`](#segmentiterator) | struct | An iterator for the loadable segments in a [`File`]. |
| [`Segment`](#segment) | struct | A loadable segment in a [`File`]. |
| [`SectionIterator`](#sectioniterator) | struct | An iterator for the sections in a [`File`]. |
| [`Section`](#section) | struct | A section in a [`File`]. |
| [`ComdatIterator`](#comdatiterator) | struct | An iterator for the COMDAT section groups in a [`File`]. |
| [`Comdat`](#comdat) | struct | A COMDAT section group in a [`File`]. |
| [`ComdatSectionIterator`](#comdatsectioniterator) | struct | An iterator for the sections in a [`Comdat`]. |
| [`SymbolTable`](#symboltable) | struct | A symbol table in a [`File`]. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for the symbols in a [`SymbolTable`]. |
| [`Symbol`](#symbol) | struct | An symbol in a [`SymbolTable`]. |
| [`DynamicRelocationIterator`](#dynamicrelocationiterator) | struct | An iterator for the dynamic relocation entries in a [`File`]. |
| [`SectionRelocationIterator`](#sectionrelocationiterator) | struct | An iterator for the relocation entries in a [`Section`]. |
| [`File`](#file) | enum | An object file that can be any supported file format. |
| [`SegmentIteratorInternal`](#segmentiteratorinternal) | enum |  |
| [`SegmentInternal`](#segmentinternal) | enum |  |
| [`SectionIteratorInternal`](#sectioniteratorinternal) | enum |  |
| [`SectionInternal`](#sectioninternal) | enum |  |
| [`ComdatIteratorInternal`](#comdatiteratorinternal) | enum |  |
| [`ComdatInternal`](#comdatinternal) | enum |  |
| [`ComdatSectionIteratorInternal`](#comdatsectioniteratorinternal) | enum |  |
| [`SymbolTableInternal`](#symboltableinternal) | enum |  |
| [`SymbolIteratorInternal`](#symboliteratorinternal) | enum |  |
| [`SymbolInternal`](#symbolinternal) | enum |  |
| [`DynamicRelocationIteratorInternal`](#dynamicrelocationiteratorinternal) | enum |  |
| [`SectionRelocationIteratorInternal`](#sectionrelocationiteratorinternal) | enum |  |
| [`with_inner!`](#with-inner) | macro | Evaluate an expression on the contents of a file format enum. |
| [`with_inner_mut!`](#with-inner-mut) | macro |  |
| [`map_inner!`](#map-inner) | macro | Like `with_inner!`, but wraps the result in another enum. |
| [`map_inner_option!`](#map-inner-option) | macro | Like `map_inner!`, but the result is a Result or Option. |
| [`map_inner_option_mut!`](#map-inner-option-mut) | macro |  |
| [`next_inner!`](#next-inner) | macro | Call `next` for a file format iterator. |

## Structs

### `SegmentIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SegmentIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentIteratorInternal<'data, 'file, R>,
}
```

An iterator for the loadable segments in a [`File`](../index.md).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="segmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="segmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-iterator-type-item"></span>`type Item = Segment<'data, 'file, R>`

- <span id="segmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Segment<'data, 'file, R: ReadRef<'data>>`

```rust
struct Segment<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentInternal<'data, 'file, R>,
}
```

A loadable segment in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<R: ReadRef<'data>> Debug for Segment<'data, 'file, R>`

- <span id="segment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> ObjectSegment for Segment<'data, 'file, R>`

- <span id="segment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="segment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="segment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="segment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="segment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<R: ReadRef<'data>> Sealed for Segment<'data, 'file, R>`

### `SectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionIteratorInternal<'data, 'file, R>,
}
```

An iterator for the sections in a [`File`](../index.md).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-iterator-type-item"></span>`type Item = Section<'data, 'file, R>`

- <span id="sectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Section<'data, 'file, R: ReadRef<'data>>`

```rust
struct Section<'data, 'file, R: ReadRef<'data>> {
    inner: SectionInternal<'data, 'file, R>,
}
```

A section in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<R: ReadRef<'data>> Debug for Section<'data, 'file, R>`

- <span id="section-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> ObjectSection for Section<'data, 'file, R>`

- <span id="section-objectsection-type-relocationiterator"></span>`type RelocationIterator = SectionRelocationIterator<'data, 'file, R>`

- <span id="section-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="section-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="section-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="section-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="section-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="section-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="section-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="section-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="section-objectsection-relocations"></span>`fn relocations(&self) -> SectionRelocationIterator<'data, 'file, R>` — [`SectionRelocationIterator`](../index.md#sectionrelocationiterator)

- <span id="section-objectsection-relocation-map"></span>`fn relocation_map(&self) -> Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="section-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

##### `impl<R: ReadRef<'data>> Sealed for Section<'data, 'file, R>`

### `ComdatIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatIteratorInternal<'data, 'file, R>,
}
```

An iterator for the COMDAT section groups in a [`File`](../index.md).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-iterator-type-item"></span>`type Item = Comdat<'data, 'file, R>`

- <span id="comdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Comdat<'data, 'file, R: ReadRef<'data>>`

```rust
struct Comdat<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatInternal<'data, 'file, R>,
}
```

A COMDAT section group in a [`File`](../index.md).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<R: ReadRef<'data>> Debug for Comdat<'data, 'file, R>`

- <span id="comdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> ObjectComdat for Comdat<'data, 'file, R>`

- <span id="comdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = ComdatSectionIterator<'data, 'file, R>`

- <span id="comdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="comdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="comdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="comdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="comdat-objectcomdat-sections"></span>`fn sections(&self) -> ComdatSectionIterator<'data, 'file, R>` — [`ComdatSectionIterator`](../index.md#comdatsectioniterator)

##### `impl<R: ReadRef<'data>> Sealed for Comdat<'data, 'file, R>`

### `ComdatSectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatSectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatSectionIteratorInternal<'data, 'file, R>,
}
```

An iterator for the sections in a [`Comdat`](../index.md).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="comdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `SymbolTable<'data, 'file, R>`

```rust
struct SymbolTable<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolTableInternal<'data, 'file, R>,
}
```

A symbol table in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSymbolTable`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<R> Debug for SymbolTable<'data, 'file, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> ObjectSymbolTable for SymbolTable<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md#objectsymboltable)

- <span id="symboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ObjectSymbolTable`](../index.md#objectsymboltable)

##### `impl<R: ReadRef<'data>> Sealed for SymbolTable<'data, 'file, R>`

### `SymbolIterator<'data, 'file, R>`

```rust
struct SymbolIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolIteratorInternal<'data, 'file, R>,
}
```

An iterator for the symbols in a [`SymbolTable`](../index.md).

#### Trait Implementations

##### `impl<R> Debug for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-iterator-type-item"></span>`type Item = Symbol<'data, 'file, R>`

- <span id="symboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Symbol<'data, 'file, R>`

```rust
struct Symbol<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolInternal<'data, 'file, R>,
}
```

An symbol in a [`SymbolTable`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<R: ReadRef<'data>> Debug for Symbol<'data, 'file, R>`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> ObjectSymbol for Symbol<'data, 'file, R>`

- <span id="symbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="symbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="symbol-objectsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="symbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="symbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="symbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md#symbolkind)

- <span id="symbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md#symbolsection)

- <span id="symbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="symbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="symbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="symbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="symbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md#symbolscope)

- <span id="symbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="symbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="symbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md#symbolflags), [`SectionIndex`](../../index.md#sectionindex), [`SymbolIndex`](../../index.md#symbolindex)

##### `impl<R: ReadRef<'data>> Sealed for Symbol<'data, 'file, R>`

### `DynamicRelocationIterator<'data, 'file, R>`

```rust
struct DynamicRelocationIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: DynamicRelocationIteratorInternal<'data, 'file, R>,
}
```

An iterator for the dynamic relocation entries in a [`File`](../index.md).

#### Trait Implementations

##### `impl<R> Debug for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dynamicrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="dynamicrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `SectionRelocationIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionRelocationIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionRelocationIteratorInternal<'data, 'file, R>,
}
```

An iterator for the relocation entries in a [`Section`](../index.md).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="sectionrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `File<'data, R: ReadRef<'data>>`

```rust
enum File<'data, R: ReadRef<'data>> {
    Coff(coff::CoffFile<'data, R>),
    CoffBig(coff::CoffBigFile<'data, R>),
    Elf32(elf::ElfFile32<'data, crate::endian::Endianness, R>),
    Elf64(elf::ElfFile64<'data, crate::endian::Endianness, R>),
    MachO32(macho::MachOFile32<'data, crate::endian::Endianness, R>),
    MachO64(macho::MachOFile64<'data, crate::endian::Endianness, R>),
    Pe32(pe::PeFile32<'data, R>),
    Pe64(pe::PeFile64<'data, R>),
    Wasm(wasm::WasmFile<'data, R>),
    Xcoff32(xcoff::XcoffFile32<'data, R>),
    Xcoff64(xcoff::XcoffFile64<'data, R>),
}
```

An object file that can be any supported file format.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="file-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw file data.

- <span id="file-parse-dyld-cache-image"></span>`fn parse_dyld_cache_image<'cache, E: crate::Endian>(image: &macho::DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](../macho/index.md#dyldcacheimage), [`Result`](../../index.md#result)

  Parse a Mach-O image from the dyld shared cache.

- <span id="file-format"></span>`fn format(&self) -> BinaryFormat` — [`BinaryFormat`](../../index.md#binaryformat)

  Return the file format.

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for File<'data, R>`

- <span id="file-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R> Object for File<'data, R>`

- <span id="file-object-type-segment"></span>`type Segment = Segment<'data, 'file, R>`

- <span id="file-object-type-segmentiterator"></span>`type SegmentIterator = SegmentIterator<'data, 'file, R>`

- <span id="file-object-type-section"></span>`type Section = Section<'data, 'file, R>`

- <span id="file-object-type-sectioniterator"></span>`type SectionIterator = SectionIterator<'data, 'file, R>`

- <span id="file-object-type-comdat"></span>`type Comdat = Comdat<'data, 'file, R>`

- <span id="file-object-type-comdatiterator"></span>`type ComdatIterator = ComdatIterator<'data, 'file, R>`

- <span id="file-object-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="file-object-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="file-object-type-symboltable"></span>`type SymbolTable = SymbolTable<'data, 'file, R>`

- <span id="file-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = DynamicRelocationIterator<'data, 'file, R>`

- <span id="file-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="file-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md#subarchitecture)

- <span id="file-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="file-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="file-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="file-object-segments"></span>`fn segments(&self) -> SegmentIterator<'data, '_, R>` — [`SegmentIterator`](../index.md#segmentiterator)

- <span id="file-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<Section<'data, 'file, R>>` — [`Section`](../index.md#section)

- <span id="file-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<Section<'data, '_, R>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`Section`](../index.md#section)

- <span id="file-object-sections"></span>`fn sections(&self) -> SectionIterator<'data, '_, R>` — [`SectionIterator`](../index.md#sectioniterator)

- <span id="file-object-comdats"></span>`fn comdats(&self) -> ComdatIterator<'data, '_, R>` — [`ComdatIterator`](../index.md#comdatiterator)

- <span id="file-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<Symbol<'data, '_, R>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`Symbol`](../index.md#symbol)

- <span id="file-object-symbols"></span>`fn symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](../index.md#symboliterator)

- <span id="file-object-symbol-table"></span>`fn symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](../index.md#symboltable)

- <span id="file-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](../index.md#symboliterator)

- <span id="file-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](../index.md#symboltable)

- <span id="file-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<DynamicRelocationIterator<'data, '_, R>>` — [`DynamicRelocationIterator`](../index.md#dynamicrelocationiterator)

- <span id="file-object-symbol-map"></span>`fn symbol_map(&self) -> SymbolMap<SymbolMapName<'data>>` — [`SymbolMap`](../../index.md#symbolmap), [`SymbolMapName`](../../index.md#symbolmapname)

- <span id="file-object-object-map"></span>`fn object_map(&self) -> ObjectMap<'data>` — [`ObjectMap`](../../index.md#objectmap)

- <span id="file-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="file-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="file-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="file-object-mach-uuid"></span>`fn mach_uuid(&self) -> Result<Option<[u8; 16]>>` — [`Result`](../../index.md#result)

- <span id="file-object-build-id"></span>`fn build_id(&self) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="file-object-gnu-debuglink"></span>`fn gnu_debuglink(&self) -> Result<Option<(&'data [u8], u32)>>` — [`Result`](../../index.md#result)

- <span id="file-object-gnu-debugaltlink"></span>`fn gnu_debugaltlink(&self) -> Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../index.md#result)

- <span id="file-object-pdb-info"></span>`fn pdb_info(&self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../index.md#result), [`CodeView`](../../index.md#codeview)

- <span id="file-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="file-object-entry"></span>`fn entry(&self) -> u64`

- <span id="file-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<R: ReadRef<'data>> Sealed for File<'data, R>`

### `SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegmentIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSegmentIterator<'data, 'file, R>),
    Elf32(elf::ElfSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegmentIterator32<'data, 'file, R>),
    Pe64(pe::PeSegmentIterator64<'data, 'file, R>),
    Wasm(wasm::WasmSegmentIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegmentIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegmentIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SegmentInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegment<'data, 'file, R>),
    CoffBig(coff::CoffBigSegment<'data, 'file, R>),
    Elf32(elf::ElfSegment32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegment64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegment32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegment64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegment32<'data, 'file, R>),
    Pe64(pe::PeSegment64<'data, 'file, R>),
    Wasm(wasm::WasmSegment<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegment32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegment64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeSectionIterator64<'data, 'file, R>),
    Wasm(wasm::WasmSectionIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSectionIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SectionInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSection<'data, 'file, R>),
    CoffBig(coff::CoffBigSection<'data, 'file, R>),
    Elf32(elf::ElfSection32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSection64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSection32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSection64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSection32<'data, 'file, R>),
    Pe64(pe::PeSection64<'data, 'file, R>),
    Wasm(wasm::WasmSection<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSection32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSection64<'data, 'file, R>),
}
```

### `ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatIterator64<'data, 'file, R>),
    Wasm(wasm::WasmComdatIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ComdatInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdat<'data, 'file, R>),
    CoffBig(coff::CoffBigComdat<'data, 'file, R>),
    Elf32(elf::ElfComdat32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdat64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdat32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdat64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdat32<'data, 'file, R>),
    Pe64(pe::PeComdat64<'data, 'file, R>),
    Wasm(wasm::WasmComdat<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdat32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdat64<'data, 'file, R>),
}
```

### `ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatSectionIterator64<'data, 'file, R>),
    Wasm(wasm::WasmComdatSectionIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatSectionIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SymbolTableInternal<'data, 'file, R>`

```rust
enum SymbolTableInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Wasm((wasm::WasmSymbolTable<'data, 'file>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolTable32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolTable64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

#### Trait Implementations

##### `impl<R> Debug for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SymbolIteratorInternal<'data, 'file, R>`

```rust
enum SymbolIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Wasm((wasm::WasmSymbolIterator<'data, 'file>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolIterator32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolIterator64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

#### Trait Implementations

##### `impl<R> Debug for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SymbolInternal<'data, 'file, R>`

```rust
enum SymbolInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Wasm((wasm::WasmSymbol<'data, 'file>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbol32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbol64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

### `DynamicRelocationIteratorInternal<'data, 'file, R>`

```rust
enum DynamicRelocationIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Elf32(elf::ElfDynamicRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfDynamicRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    None(core::marker::PhantomData<(&'data (), &'file (), R)>),
}
```

#### Trait Implementations

##### `impl<R> Debug for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffRelocationIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigRelocationIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachORelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachORelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeRelocationIterator<'data, 'file, R>),
    Pe64(pe::PeRelocationIterator<'data, 'file, R>),
    Wasm(wasm::WasmRelocationIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffRelocationIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffRelocationIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `with_inner!`

Evaluate an expression on the contents of a file format enum.

This is a hack to avoid virtual calls.

### `with_inner_mut!`

### `map_inner!`

Like `with_inner!`, but wraps the result in another enum.

### `map_inner_option!`

Like `map_inner!`, but the result is a Result or Option.

### `map_inner_option_mut!`

### `next_inner!`

Call `next` for a file format iterator.

