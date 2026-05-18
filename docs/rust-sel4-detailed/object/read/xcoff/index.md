*[object](../../index.md) / [read](../index.md) / [xcoff](index.md)*

---

# Module `xcoff`

Support for reading AIX XCOFF files.

Traits are used to abstract over the difference between 32-bit and 64-bit XCOFF.
The primary trait for this is [`FileHeader`](#fileheader).

## High level API

[`XcoffFile`](#xcofffile) implements the [`Object`](crate::read::Object) trait for XCOFF files.
[`XcoffFile`](#xcofffile) is parameterised by [`FileHeader`](#fileheader) to allow reading both 32-bit and
64-bit XCOFF. There are type aliases for these parameters ([`XcoffFile32`](#xcofffile32) and
[`XcoffFile64`](#xcofffile64)).

## Low level API

The [`FileHeader`](#fileheader) trait can be directly used to parse both [`xcoff::FileHeader32`](../../xcoff/index.md)
and [`xcoff::FileHeader64`](../../xcoff/index.md).

### Example for low level API
 ```no_run
use object::xcoff;
use object::read::xcoff::{FileHeader, SectionHeader, Symbol};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = xcoff::FileHeader64::parse(&*data, &mut offset)?;
    let aux_header = header.aux_header(&*data, &mut offset)?;
    let sections = header.sections(&*data, &mut offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name()));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
  }
    Ok(())
}
```

## Contents

- [Modules](#modules)
  - [`file`](#file)
  - [`section`](#section)
  - [`symbol`](#symbol)
  - [`relocation`](#relocation)
  - [`comdat`](#comdat)
  - [`segment`](#segment)
- [Structs](#structs)
  - [`XcoffFile`](#xcofffile)
  - [`XcoffSectionIterator`](#xcoffsectioniterator)
  - [`XcoffSection`](#xcoffsection)
  - [`SectionTable`](#sectiontable)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`XcoffSymbolTable`](#xcoffsymboltable)
  - [`XcoffSymbolIterator`](#xcoffsymboliterator)
  - [`XcoffSymbol`](#xcoffsymbol)
  - [`XcoffRelocationIterator`](#xcoffrelocationiterator)
  - [`XcoffComdatIterator`](#xcoffcomdatiterator)
  - [`XcoffComdat`](#xcoffcomdat)
  - [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator)
  - [`XcoffSegmentIterator`](#xcoffsegmentiterator)
  - [`XcoffSegment`](#xcoffsegment)
- [Traits](#traits)
  - [`FileHeader`](#fileheader)
  - [`AuxHeader`](#auxheader)
  - [`SectionHeader`](#sectionheader)
  - [`Symbol`](#symbol)
  - [`FileAux`](#fileaux)
  - [`CsectAux`](#csectaux)
  - [`Rel`](#rel)
- [Type Aliases](#type-aliases)
  - [`XcoffFile32`](#xcofffile32)
  - [`XcoffFile64`](#xcofffile64)
  - [`XcoffSectionIterator32`](#xcoffsectioniterator32)
  - [`XcoffSectionIterator64`](#xcoffsectioniterator64)
  - [`XcoffSection32`](#xcoffsection32)
  - [`XcoffSection64`](#xcoffsection64)
  - [`XcoffSymbolTable32`](#xcoffsymboltable32)
  - [`XcoffSymbolTable64`](#xcoffsymboltable64)
  - [`XcoffSymbolIterator32`](#xcoffsymboliterator32)
  - [`XcoffSymbolIterator64`](#xcoffsymboliterator64)
  - [`XcoffSymbol32`](#xcoffsymbol32)
  - [`XcoffSymbol64`](#xcoffsymbol64)
  - [`XcoffRelocationIterator32`](#xcoffrelocationiterator32)
  - [`XcoffRelocationIterator64`](#xcoffrelocationiterator64)
  - [`XcoffComdatIterator32`](#xcoffcomdatiterator32)
  - [`XcoffComdatIterator64`](#xcoffcomdatiterator64)
  - [`XcoffComdat32`](#xcoffcomdat32)
  - [`XcoffComdat64`](#xcoffcomdat64)
  - [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32)
  - [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64)
  - [`XcoffSegmentIterator32`](#xcoffsegmentiterator32)
  - [`XcoffSegmentIterator64`](#xcoffsegmentiterator64)
  - [`XcoffSegment32`](#xcoffsegment32)
  - [`XcoffSegment64`](#xcoffsegment64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`file`](#file) | mod |  |
| [`section`](#section) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`relocation`](#relocation) | mod |  |
| [`comdat`](#comdat) | mod | XCOFF doesn't support the COMDAT section. |
| [`segment`](#segment) | mod | TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready. |
| [`XcoffFile`](#xcofffile) | struct | A partially parsed XCOFF file. |
| [`XcoffSectionIterator`](#xcoffsectioniterator) | struct | An iterator for the sections in an [`XcoffFile`]. |
| [`XcoffSection`](#xcoffsection) | struct | A section in an [`XcoffFile`]. |
| [`SectionTable`](#sectiontable) | struct | The table of section headers in an XCOFF file. |
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in an XCOFF file. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for symbol entries in an XCOFF file. |
| [`XcoffSymbolTable`](#xcoffsymboltable) | struct | A symbol table in an [`XcoffFile`]. |
| [`XcoffSymbolIterator`](#xcoffsymboliterator) | struct | An iterator for the symbols in an [`XcoffFile`]. |
| [`XcoffSymbol`](#xcoffsymbol) | struct | A symbol in an [`XcoffFile`]. |
| [`XcoffRelocationIterator`](#xcoffrelocationiterator) | struct | An iterator for the relocations in an [`XcoffSection`](super::XcoffSection). |
| [`XcoffComdatIterator`](#xcoffcomdatiterator) | struct | An iterator for the COMDAT section groups in a [`XcoffFile`]. |
| [`XcoffComdat`](#xcoffcomdat) | struct | A COMDAT section group in a [`XcoffFile`]. |
| [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`XcoffFile`]. |
| [`XcoffSegmentIterator`](#xcoffsegmentiterator) | struct | An iterator for the segments in an [`XcoffFile`]. |
| [`XcoffSegment`](#xcoffsegment) | struct | A loadable section in an [`XcoffFile`]. |
| [`FileHeader`](#fileheader) | trait | A trait for generic access to [`xcoff::FileHeader32`] and [`xcoff::FileHeader64`]. |
| [`AuxHeader`](#auxheader) | trait | A trait for generic access to [`xcoff::AuxHeader32`] and [`xcoff::AuxHeader64`]. |
| [`SectionHeader`](#sectionheader) | trait | A trait for generic access to [`xcoff::SectionHeader32`] and [`xcoff::SectionHeader64`]. |
| [`Symbol`](#symbol) | trait | A trait for generic access to [`xcoff::Symbol32`] and [`xcoff::Symbol64`]. |
| [`FileAux`](#fileaux) | trait | A trait for generic access to [`xcoff::FileAux32`] and [`xcoff::FileAux64`]. |
| [`CsectAux`](#csectaux) | trait | A trait for generic access to [`xcoff::CsectAux32`] and [`xcoff::CsectAux64`]. |
| [`Rel`](#rel) | trait | A trait for generic access to [`xcoff::Rel32`] and [`xcoff::Rel64`]. |
| [`XcoffFile32`](#xcofffile32) | type | A 32-bit XCOFF object file. |
| [`XcoffFile64`](#xcofffile64) | type | A 64-bit XCOFF object file. |
| [`XcoffSectionIterator32`](#xcoffsectioniterator32) | type | An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSectionIterator64`](#xcoffsectioniterator64) | type | An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSection32`](#xcoffsection32) | type | A section in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSection64`](#xcoffsection64) | type | A section in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbolTable32`](#xcoffsymboltable32) | type | A symbol table in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolTable64`](#xcoffsymboltable64) | type | A symbol table in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbolIterator32`](#xcoffsymboliterator32) | type | An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolIterator64`](#xcoffsymboliterator64) | type | An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbol32`](#xcoffsymbol32) | type | A symbol in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbol64`](#xcoffsymbol64) | type | A symbol in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffRelocationIterator32`](#xcoffrelocationiterator32) | type | An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32). |
| [`XcoffRelocationIterator64`](#xcoffrelocationiterator64) | type | An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64). |
| [`XcoffComdatIterator32`](#xcoffcomdatiterator32) | type | An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatIterator64`](#xcoffcomdatiterator64) | type | An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdat32`](#xcoffcomdat32) | type | A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdat64`](#xcoffcomdat64) | type | A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegmentIterator32`](#xcoffsegmentiterator32) | type | An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegmentIterator64`](#xcoffsegmentiterator64) | type | An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegment32`](#xcoffsegment32) | type | A segment in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegment64`](#xcoffsegment64) | type | A segment in an [`XcoffFile64`](super::XcoffFile64). |

## Modules

- [`file`](file/index.md)
- [`section`](section/index.md)
- [`symbol`](symbol/index.md)
- [`relocation`](relocation/index.md)
- [`comdat`](comdat/index.md) — XCOFF doesn't support the COMDAT section.
- [`segment`](segment/index.md) — TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready.

## Structs

### `XcoffFile<'data, Xcoff, R>`

```rust
struct XcoffFile<'data, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    data: R,
    header: &'data Xcoff,
    aux_header: Option<&'data <Xcoff as >::AuxHeader>,
    sections: super::SectionTable<'data, Xcoff>,
    symbols: super::SymbolTable<'data, Xcoff, R>,
}
```

A partially parsed XCOFF file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="xcofffile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw XCOFF file data.

- <span id="xcofffile-data"></span>`fn data(&self) -> R`

  Returns the raw data.

- <span id="xcofffile-raw-header"></span>`fn raw_header(&self) -> &'data Xcoff`

  Returns the raw XCOFF file header.

- <span id="xcofffile-xcoff-header"></span>`fn xcoff_header(&self) -> &'data Xcoff`

  Get the raw XCOFF file header.

- <span id="xcofffile-xcoff-aux-header"></span>`fn xcoff_aux_header(&self) -> Option<&'data <Xcoff as >::AuxHeader>` — [`FileHeader`](#fileheader)

  Get the raw XCOFF auxiliary header.

- <span id="xcofffile-xcoff-section-table"></span>`fn xcoff_section_table(&self) -> &SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

  Get the XCOFF section table.

- <span id="xcofffile-xcoff-symbol-table"></span>`fn xcoff_symbol_table(&self) -> &SymbolTable<'data, Xcoff, R>` — [`SymbolTable`](#symboltable)

  Get the XCOFF symbol table.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> Object for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-object-type-segment"></span>`type Segment = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-segmentiterator"></span>`type SegmentIterator = XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-section"></span>`type Section = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-sectioniterator"></span>`type SectionIterator = XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-comdat"></span>`type Comdat = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-comdatiterator"></span>`type ComdatIterator = XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-symboltable"></span>`type SymbolTable = XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="xcofffile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="xcofffile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="xcofffile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="xcofffile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="xcofffile-object-segments"></span>`fn segments(&self) -> XcoffSegmentIterator<'data, '_, Xcoff, R>` — [`XcoffSegmentIterator`](#xcoffsegmentiterator)

- <span id="xcofffile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>` — [`XcoffSection`](#xcoffsection)

- <span id="xcofffile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<XcoffSection<'data, '_, Xcoff, R>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`XcoffSection`](#xcoffsection)

- <span id="xcofffile-object-sections"></span>`fn sections(&self) -> XcoffSectionIterator<'data, '_, Xcoff, R>` — [`XcoffSectionIterator`](#xcoffsectioniterator)

- <span id="xcofffile-object-comdats"></span>`fn comdats(&self) -> XcoffComdatIterator<'data, '_, Xcoff, R>` — [`XcoffComdatIterator`](#xcoffcomdatiterator)

- <span id="xcofffile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<XcoffSymbolTable<'data, '_, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- <span id="xcofffile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<XcoffSymbol<'data, '_, Xcoff, R>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`XcoffSymbol`](#xcoffsymbol)

- <span id="xcofffile-object-symbols"></span>`fn symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- <span id="xcofffile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- <span id="xcofffile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- <span id="xcofffile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>` — [`Object`](../index.md#object)

- <span id="xcofffile-object-imports"></span>`fn imports(&self) -> Result<alloc::vec::Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="xcofffile-object-exports"></span>`fn exports(&self) -> Result<alloc::vec::Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="xcofffile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="xcofffile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="xcofffile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="xcofffile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<Xcoff, R> Sealed for XcoffFile<'data, Xcoff, R>`

### `XcoffSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Xcoff as >::SectionHeader>>,
}
```

An iterator for the sections in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-iterator-type-item"></span>`type Item = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSection<'data, 'file, Xcoff, R>`

```rust
struct XcoffSection<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    section: &'data <Xcoff as >::SectionHeader,
    index: crate::read::SectionIndex,
}
```

A section in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="xcoffsection-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

  Get the XCOFF file containing this section.

- <span id="xcoffsection-xcoff-section"></span>`fn xcoff_section(&self) -> &'data <Xcoff as >::SectionHeader` — [`FileHeader`](#fileheader)

  Get the raw XCOFF section header.

- <span id="xcoffsection-xcoff-relocations"></span>`fn xcoff_relocations(&self) -> Result<&'data [<Xcoff as >::Rel]>` — [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the raw XCOFF relocation entries for this section.

- <span id="xcoffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> ObjectSection for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="xcoffsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="xcoffsection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="xcoffsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="xcoffsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="xcoffsection-objectsection-relocations"></span>`fn relocations(&self) -> <Self as >::RelocationIterator` — [`ObjectSection`](../index.md#objectsection)

- <span id="xcoffsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="xcoffsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

- <span id="xcoffsection-objectsection-uncompressed-data"></span>`fn uncompressed_data(&self) -> Result<alloc::borrow::Cow<'data, [u8]>>` — [`Result`](../../index.md#result)

##### `impl<Xcoff, R> Sealed for XcoffSection<'data, 'file, Xcoff, R>`

### `SectionTable<'data, Xcoff: FileHeader>`

```rust
struct SectionTable<'data, Xcoff: FileHeader> {
    sections: &'data [<Xcoff as >::SectionHeader],
}
```

The table of section headers in an XCOFF file.

Returned by `FileHeader::sections`.

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<R: ReadRef<'data>>(header: &Xcoff, data: R, offset: &mut u64) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the section table.

  

  `data` must be the entire file data.

  `offset` must be after the optional file header.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` — [`FileHeader`](#fileheader)

  Iterate over the section headers.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Return the section header at the given index.

  

  The index is 1-based.

#### Trait Implementations

##### `impl<Xcoff: clone::Clone + FileHeader> Clone for SectionTable<'data, Xcoff>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

##### `impl<Xcoff: marker::Copy + FileHeader> Copy for SectionTable<'data, Xcoff>`

##### `impl<Xcoff: fmt::Debug + FileHeader> Debug for SectionTable<'data, Xcoff>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff> Default for SectionTable<'data, Xcoff>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

### `SymbolTable<'data, Xcoff, R>`

```rust
struct SymbolTable<'data, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'data [xcoff::SymbolBytes],
    strings: crate::read::StringTable<'data, R>,
    header: core::marker::PhantomData<Xcoff>,
}
```

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by `FileHeader::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: Xcoff, data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the symbol table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

  Iterate over the symbols.

  

  This does not return null symbols.

- <span id="symboltable-iter-none"></span>`fn iter_none<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

  Empty symbol iterator.

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result)

  Return the symbol entry at the given index and offset.

- <span id="symboltable-symbol-unchecked"></span>`fn symbol_unchecked(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the symbol at the given index.

  

  This does not check if the symbol is null, but does check if the index is in bounds.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the symbol at the given index.

  

  Returns an error for null symbols and out of bounds indices.

  Note that this is unable to check whether the index is an auxiliary symbol.

- <span id="symboltable-aux-file"></span>`fn aux_file(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Return a file auxiliary symbol.

- <span id="symboltable-aux-csect"></span>`fn aux_csect(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Return the csect auxiliary symbol.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbol table entries.

  

  This includes auxiliary symbol table entries.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> Default for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `SymbolIterator<'data, 'table, Xcoff, R>`

```rust
struct SymbolIterator<'data, 'table, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'table SymbolTable<'data, Xcoff, R>,
    index: usize,
}
```

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-iterator-type-item"></span>`type Item = (SymbolIndex, &'data <Xcoff as FileHeader>::Symbol)`

- <span id="symboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSymbolTable<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolTable<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
}
```

A symbol table in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<Xcoff, R> Clone for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-clone"></span>`fn clone(&self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>` — [`XcoffSymbolTable`](#xcoffsymboltable)

##### `impl<Xcoff, R> Copy for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl<Xcoff, R> Debug for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md#objectsymboltable)

- <span id="xcoffsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ObjectSymbolTable`](../index.md#objectsymboltable)

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbolTable<'data, 'file, Xcoff, R>`

### `XcoffSymbolIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: SymbolIterator<'data, 'file, Xcoff, R>,
}
```

An iterator for the symbols in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Debug for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Iterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-iterator-type-item"></span>`type Item = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSymbol<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbol<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Xcoff as >::Symbol,
}
```

A symbol in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- <span id="xcoffsymbol-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

  Get the XCOFF file containing this symbol.

- <span id="xcoffsymbol-xcoff-symbol"></span>`fn xcoff_symbol(&self) -> &'data <Xcoff as >::Symbol` — [`FileHeader`](#fileheader)

  Get the raw XCOFF symbol structure.

#### Trait Implementations

##### `impl<Xcoff, R> Clone for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-clone"></span>`fn clone(&self) -> XcoffSymbol<'data, 'file, Xcoff, R>` — [`XcoffSymbol`](#xcoffsymbol)

##### `impl<Xcoff, R> Copy for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl<Xcoff, R> Debug for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbol for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="xcoffsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsymbol-objectsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="xcoffsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md#symbolkind)

- <span id="xcoffsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md#symbolsection)

- <span id="xcoffsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- <span id="xcoffsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md#symbolscope)

- <span id="xcoffsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md#symbolflags), [`SectionIndex`](../../index.md#sectionindex), [`SymbolIndex`](../../index.md#symbolindex)

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbol<'data, 'file, Xcoff, R>`

### `XcoffRelocationIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffRelocationIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    relocations: slice::Iter<'data, <<Xcoff as FileHeader>::SectionHeader as SectionHeader>::Rel>,
}
```

An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="xcoffrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffComdatIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the COMDAT section groups in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-iterator-type-item"></span>`type Item = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffComdat<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdat<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

A COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> ObjectComdat for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="xcoffcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="xcoffcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffcomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="xcoffcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md#objectcomdat)

##### `impl<Xcoff, R> Sealed for XcoffComdat<'data, 'file, Xcoff, R>`

### `XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="xcoffcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSegmentIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegmentIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the segments in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-type-item"></span>`type Item = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSegment<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegment<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

A loadable section in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> ObjectSegment for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="xcoffsegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-data-range"></span>`fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<Xcoff, R> Sealed for XcoffSegment<'data, 'file, Xcoff, R>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::FileHeader32`](../../xcoff/index.md) and [`xcoff::FileHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

- `type AuxHeader: 1`

- `type SectionHeader: 1`

- `type Symbol: 1`

- `type FileAux: 1`

- `type CsectAux: 1`

- `type Rel: 1`

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn f_magic(&self) -> u16`

- `fn f_nscns(&self) -> u16`

- `fn f_timdat(&self) -> u32`

- `fn f_symptr(&self) -> <Self as >::Word`

- `fn f_nsyms(&self) -> u32`

- `fn f_opthdr(&self) -> u16`

- `fn f_flags(&self) -> u16`

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

- `fn aux_header<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<Option<&'data <Self as >::AuxHeader>>`

  Read the auxiliary file header.

- `fn sections<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<SectionTable<'data, Self>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(&self, data: R) -> Result<SymbolTable<'data, Self, R>>`

  Return the symbol table.

#### Implementors

- [`FileHeader32`](../../xcoff/index.md#fileheader32)
- [`FileHeader64`](../../xcoff/index.md#fileheader64)

### `AuxHeader`

```rust
trait AuxHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::AuxHeader32`](../../xcoff/index.md) and [`xcoff::AuxHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn o_mflag(&self) -> u16`

- `fn o_vstamp(&self) -> u16`

- `fn o_tsize(&self) -> <Self as >::Word`

- `fn o_dsize(&self) -> <Self as >::Word`

- `fn o_bsize(&self) -> <Self as >::Word`

- `fn o_entry(&self) -> <Self as >::Word`

- `fn o_text_start(&self) -> <Self as >::Word`

- `fn o_data_start(&self) -> <Self as >::Word`

- `fn o_toc(&self) -> <Self as >::Word`

- `fn o_snentry(&self) -> u16`

- `fn o_sntext(&self) -> u16`

- `fn o_sndata(&self) -> u16`

- `fn o_sntoc(&self) -> u16`

- `fn o_snloader(&self) -> u16`

- `fn o_snbss(&self) -> u16`

- `fn o_algntext(&self) -> u16`

- `fn o_algndata(&self) -> u16`

- `fn o_modtype(&self) -> u16`

- `fn o_cpuflag(&self) -> u8`

- `fn o_cputype(&self) -> u8`

- `fn o_maxstack(&self) -> <Self as >::Word`

- `fn o_maxdata(&self) -> <Self as >::Word`

- `fn o_debugger(&self) -> u32`

- `fn o_textpsize(&self) -> u8`

- `fn o_datapsize(&self) -> u8`

- `fn o_stackpsize(&self) -> u8`

- `fn o_flags(&self) -> u8`

- `fn o_sntdata(&self) -> u16`

- `fn o_sntbss(&self) -> u16`

- `fn o_x64flags(&self) -> Option<u16>`

#### Implementors

- [`AuxHeader32`](../../xcoff/index.md#auxheader32)
- [`AuxHeader64`](../../xcoff/index.md#auxheader64)

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::SectionHeader32`](../../xcoff/index.md) and [`xcoff::SectionHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

- `type HalfWord: 1`

- `type Xcoff: 1`

- `type Rel: 1`

#### Required Methods

- `fn s_name(&self) -> &[u8; 8]`

- `fn s_paddr(&self) -> <Self as >::Word`

- `fn s_vaddr(&self) -> <Self as >::Word`

- `fn s_size(&self) -> <Self as >::Word`

- `fn s_scnptr(&self) -> <Self as >::Word`

- `fn s_relptr(&self) -> <Self as >::Word`

- `fn s_lnnoptr(&self) -> <Self as >::Word`

- `fn s_nreloc(&self) -> <Self as >::HalfWord`

- `fn s_nlnno(&self) -> <Self as >::HalfWord`

- `fn s_flags(&self) -> u32`

- `fn relocations<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [<Self as >::Rel]>`

  Read the relocations.

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the section name.

- `fn file_range(&self) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

#### Implementors

- [`SectionHeader32`](../../xcoff/index.md#sectionheader32)
- [`SectionHeader64`](../../xcoff/index.md#sectionheader64)

### `Symbol`

```rust
trait Symbol: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::Symbol32`](../../xcoff/index.md) and [`xcoff::Symbol64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn n_value(&self) -> <Self as >::Word`

- `fn n_scnum(&self) -> i16`

- `fn n_type(&self) -> u16`

- `fn n_sclass(&self) -> u8`

- `fn n_numaux(&self) -> u8`

- `fn name_offset(&self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

#### Provided Methods

- `fn section(&self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_null(&self) -> bool`

  Return true if the symbol is a null placeholder.

- `fn is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- `fn has_aux_file(&self) -> bool`

  Return true if the symbol has file auxiliary entry.

- `fn has_aux_csect(&self) -> bool`

  Return true if the symbol has csect auxiliary entry.

#### Implementors

- [`Symbol32`](../../xcoff/index.md#symbol32)
- [`Symbol64`](../../xcoff/index.md#symbol64)

### `FileAux`

```rust
trait FileAux: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::FileAux32`](../../xcoff/index.md) and [`xcoff::FileAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_fname(&self) -> &[u8; 8]`

- `fn x_ftype(&self) -> u8`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn name_offset(&self) -> Option<u32>`

- `fn fname<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse the x_fname field, which may be an inline string or a string table offset.

#### Implementors

- [`FileAux32`](../../xcoff/index.md#fileaux32)
- [`FileAux64`](../../xcoff/index.md#fileaux64)

### `CsectAux`

```rust
trait CsectAux: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::CsectAux32`](../../xcoff/index.md) and [`xcoff::CsectAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_scnlen(&self) -> u64`

- `fn x_parmhash(&self) -> u32`

- `fn x_snhash(&self) -> u16`

- `fn x_smtyp(&self) -> u8`

- `fn x_smclas(&self) -> u8`

- `fn x_stab(&self) -> Option<u32>`

- `fn x_snstab(&self) -> Option<u16>`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn alignment(&self) -> u8`

- `fn sym_type(&self) -> u8`

#### Implementors

- [`CsectAux32`](../../xcoff/index.md#csectaux32)
- [`CsectAux64`](../../xcoff/index.md#csectaux64)

### `Rel`

```rust
trait Rel: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::Rel32`](../../xcoff/index.md) and [`xcoff::Rel64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn r_vaddr(&self) -> <Self as >::Word`

- `fn r_symndx(&self) -> u32`

- `fn r_rsize(&self) -> u8`

- `fn r_rtype(&self) -> u8`

#### Provided Methods

- `fn symbol(&self) -> SymbolIndex`

#### Implementors

- [`Rel32`](../../xcoff/index.md#rel32)
- [`Rel64`](../../xcoff/index.md#rel64)

## Type Aliases

### `XcoffFile32<'data, R>`

```rust
type XcoffFile32<'data, R> = XcoffFile<'data, xcoff::FileHeader32, R>;
```

A 32-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader32`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff32`](../../index.md).

### `XcoffFile64<'data, R>`

```rust
type XcoffFile64<'data, R> = XcoffFile<'data, xcoff::FileHeader64, R>;
```

A 64-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader64`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff64`](../../index.md).

### `XcoffSectionIterator32<'data, 'file, R>`

```rust
type XcoffSectionIterator32<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSectionIterator64<'data, 'file, R>`

```rust
type XcoffSectionIterator64<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSection32<'data, 'file, R>`

```rust
type XcoffSection32<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader32, R>;
```

A section in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSection64<'data, 'file, R>`

```rust
type XcoffSection64<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader64, R>;
```

A section in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolTable32<'data, 'file, R>`

```rust
type XcoffSymbolTable32<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader32, R>;
```

A symbol table in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolTable64<'data, 'file, R>`

```rust
type XcoffSymbolTable64<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>;
```

A symbol table in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolIterator32<'data, 'file, R>`

```rust
type XcoffSymbolIterator32<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolIterator64<'data, 'file, R>`

```rust
type XcoffSymbolIterator64<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbol32<'data, 'file, R>`

```rust
type XcoffSymbol32<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>;
```

A symbol in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbol64<'data, 'file, R>`

```rust
type XcoffSymbol64<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>;
```

A symbol in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffRelocationIterator32<'data, 'file, R>`

```rust
type XcoffRelocationIterator32<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).

### `XcoffRelocationIterator64<'data, 'file, R>`

```rust
type XcoffRelocationIterator64<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).

### `XcoffComdatIterator32<'data, 'file, R>`

```rust
type XcoffComdatIterator32<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatIterator64<'data, 'file, R>`

```rust
type XcoffComdatIterator64<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdat32<'data, 'file, R>`

```rust
type XcoffComdat32<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader32, R>;
```

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdat64<'data, 'file, R>`

```rust
type XcoffComdat64<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader64, R>;
```

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdatSectionIterator32<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator32<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatSectionIterator64<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator64<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegmentIterator32<'data, 'file, R>`

```rust
type XcoffSegmentIterator32<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegmentIterator64<'data, 'file, R>`

```rust
type XcoffSegmentIterator64<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegment32<'data, 'file, R>`

```rust
type XcoffSegment32<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader32, R>;
```

A segment in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegment64<'data, 'file, R>`

```rust
type XcoffSegment64<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader64, R>;
```

A segment in an [`XcoffFile64`](super::XcoffFile64).

