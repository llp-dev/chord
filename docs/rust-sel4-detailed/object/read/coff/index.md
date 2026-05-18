*[object](../../index.md) / [read](../index.md) / [coff](index.md)*

---

# Module `coff`

Support for reading Windows COFF files.

Traits are used to abstract over the difference between COFF object files
and COFF bigobj files. The primary trait for this is [`CoffHeader`](#coffheader).

## High level API

[`CoffFile`](#cofffile) implements the [`Object`](crate::read::Object) trait for
COFF files. [`CoffFile`](#cofffile) is parameterised by [`CoffHeader`](#coffheader).
The default parameter allows reading regular COFF object files,
while the type alias [`CoffBigFile`](#coffbigfile) allows reading COFF bigobj files.

[`ImportFile`](#importfile) allows reading COFF short imports that are used in import
libraries. Currently these are not integrated with the unified read API.

## Low level API

The [`CoffHeader`](#coffheader) trait can be directly used to parse both COFF
object files (which start with [`pe::ImageFileHeader`](../../pe/index.md)) and COFF bigobj
files (which start with [`pe::AnonObjectHeaderBigobj`](../../pe/index.md)).

### Example for low level API
 ```no_run
use object::pe;
use object::read::coff::{CoffHeader, ImageSymbol as _};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = pe::ImageFileHeader::parse(&*data, &mut offset)?;
    let sections = header.sections(&*data, offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
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
  - [`import`](#import)
- [Structs](#structs)
  - [`CoffCommon`](#coffcommon)
  - [`CoffFile`](#cofffile)
  - [`SectionTable`](#sectiontable)
  - [`CoffSegmentIterator`](#coffsegmentiterator)
  - [`CoffSegment`](#coffsegment)
  - [`CoffSectionIterator`](#coffsectioniterator)
  - [`CoffSection`](#coffsection)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`CoffSymbolTable`](#coffsymboltable)
  - [`CoffSymbolIterator`](#coffsymboliterator)
  - [`CoffSymbol`](#coffsymbol)
  - [`CoffRelocationIterator`](#coffrelocationiterator)
  - [`CoffComdatIterator`](#coffcomdatiterator)
  - [`CoffComdat`](#coffcomdat)
  - [`CoffComdatSectionIterator`](#coffcomdatsectioniterator)
  - [`ImportFile`](#importfile)
  - [`ImportObjectData`](#importobjectdata)
- [Enums](#enums)
  - [`ImportName`](#importname)
  - [`ImportType`](#importtype)
- [Traits](#traits)
  - [`CoffHeader`](#coffheader)
  - [`ImageSymbol`](#imagesymbol)
- [Functions](#functions)
  - [`anon_object_class_id`](#anon-object-class-id)
- [Type Aliases](#type-aliases)
  - [`CoffBigFile`](#coffbigfile)
  - [`CoffBigSegmentIterator`](#coffbigsegmentiterator)
  - [`CoffBigSegment`](#coffbigsegment)
  - [`CoffBigSectionIterator`](#coffbigsectioniterator)
  - [`CoffBigSection`](#coffbigsection)
  - [`CoffBigSymbolTable`](#coffbigsymboltable)
  - [`CoffBigSymbolIterator`](#coffbigsymboliterator)
  - [`CoffBigSymbol`](#coffbigsymbol)
  - [`CoffBigRelocationIterator`](#coffbigrelocationiterator)
  - [`CoffBigComdatIterator`](#coffbigcomdatiterator)
  - [`CoffBigComdat`](#coffbigcomdat)
  - [`CoffBigComdatSectionIterator`](#coffbigcomdatsectioniterator)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`file`](#file) | mod |  |
| [`section`](#section) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`relocation`](#relocation) | mod |  |
| [`comdat`](#comdat) | mod |  |
| [`import`](#import) | mod | Support for reading short import files. |
| [`CoffCommon`](#coffcommon) | struct | The common parts of `PeFile` and `CoffFile`. |
| [`CoffFile`](#cofffile) | struct | A COFF object file. |
| [`SectionTable`](#sectiontable) | struct | The table of section headers in a COFF or PE file. |
| [`CoffSegmentIterator`](#coffsegmentiterator) | struct | An iterator for the loadable sections in a [`CoffFile`]. |
| [`CoffSegment`](#coffsegment) | struct | A loadable section in a [`CoffFile`]. |
| [`CoffSectionIterator`](#coffsectioniterator) | struct | An iterator for the sections in a [`CoffFile`]. |
| [`CoffSection`](#coffsection) | struct | A section in a [`CoffFile`]. |
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in a COFF or PE file. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for symbol entries in a COFF or PE file. |
| [`CoffSymbolTable`](#coffsymboltable) | struct | A symbol table in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile). |
| [`CoffSymbolIterator`](#coffsymboliterator) | struct | An iterator for the symbols in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile). |
| [`CoffSymbol`](#coffsymbol) | struct | A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile). |
| [`CoffRelocationIterator`](#coffrelocationiterator) | struct | An iterator for the relocations in a [`CoffSection`](super::CoffSection). |
| [`CoffComdatIterator`](#coffcomdatiterator) | struct | An iterator for the COMDAT section groups in a [`CoffFile`]. |
| [`CoffComdat`](#coffcomdat) | struct | A COMDAT section group in a [`CoffFile`]. |
| [`CoffComdatSectionIterator`](#coffcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`CoffFile`]. |
| [`ImportFile`](#importfile) | struct | A Windows short form description of a symbol to import. |
| [`ImportObjectData`](#importobjectdata) | struct | The data following [`pe::ImportObjectHeader`]. |
| [`ImportName`](#importname) | enum | The name or ordinal to import from a DLL. |
| [`ImportType`](#importtype) | enum | The kind of import symbol. |
| [`CoffHeader`](#coffheader) | trait | A trait for generic access to [`pe::ImageFileHeader`] and [`pe::AnonObjectHeaderBigobj`]. |
| [`ImageSymbol`](#imagesymbol) | trait | A trait for generic access to [`pe::ImageSymbol`] and [`pe::ImageSymbolEx`]. |
| [`anon_object_class_id`](#anon-object-class-id) | fn | Read the `class_id` field from a [`pe::AnonObjectHeader`]. |
| [`CoffBigFile`](#coffbigfile) | type | A COFF bigobj object file with 32-bit section numbers. |
| [`CoffBigSegmentIterator`](#coffbigsegmentiterator) | type | An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSegment`](#coffbigsegment) | type | A loadable section in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSectionIterator`](#coffbigsectioniterator) | type | An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSection`](#coffbigsection) | type | A section in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSymbolTable`](#coffbigsymboltable) | type | A symbol table in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSymbolIterator`](#coffbigsymboliterator) | type | An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSymbol`](#coffbigsymbol) | type | A symbol in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigRelocationIterator`](#coffbigrelocationiterator) | type | An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection). |
| [`CoffBigComdatIterator`](#coffbigcomdatiterator) | type | An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigComdat`](#coffbigcomdat) | type | A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigComdatSectionIterator`](#coffbigcomdatsectioniterator) | type | An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile). |

## Modules

- [`file`](file/index.md)
- [`section`](section/index.md)
- [`symbol`](symbol/index.md)
- [`relocation`](relocation/index.md)
- [`comdat`](comdat/index.md)
- [`import`](import/index.md) — Support for reading short import files.

## Structs

### `CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    sections: super::SectionTable<'data>,
    symbols: super::SymbolTable<'data, R, Coff>,
    image_base: u64,
}
```

The common parts of `PeFile` and `CoffFile`.

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    header: &'data Coff,
    common: CoffCommon<'data, R, Coff>,
    data: R,
}
```

A COFF object file.

This is a file that starts with [`pe::ImageFileHeader`](../../pe/index.md), and corresponds
to [`crate::FileKind::Coff`](../../index.md).

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="cofffile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw COFF file data.

- <span id="cofffile-coff-header"></span>`fn coff_header(&self) -> &'data Coff`

  Get the raw COFF file header.

- <span id="cofffile-coff-section-table"></span>`fn coff_section_table(&self) -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

  Get the COFF section table.

- <span id="cofffile-coff-symbol-table"></span>`fn coff_symbol_table(&self) -> &SymbolTable<'data, R, Coff>` — [`SymbolTable`](#symboltable)

  Get the COFF symbol table.

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffFile<'data, R, Coff>`

- <span id="cofffile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Coff> Object for CoffFile<'data, R, Coff>`

- <span id="cofffile-object-type-segment"></span>`type Segment = CoffSegment<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-segmentiterator"></span>`type SegmentIterator = CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-section"></span>`type Section = CoffSection<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-sectioniterator"></span>`type SectionIterator = CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-comdat"></span>`type Comdat = CoffComdat<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-comdatiterator"></span>`type ComdatIterator = CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-symbol"></span>`type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-symboliterator"></span>`type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-symboltable"></span>`type SymbolTable = CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="cofffile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="cofffile-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md#subarchitecture)

- <span id="cofffile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="cofffile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="cofffile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="cofffile-object-segments"></span>`fn segments(&self) -> CoffSegmentIterator<'data, '_, R, Coff>` — [`CoffSegmentIterator`](#coffsegmentiterator)

- <span id="cofffile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<CoffSection<'data, 'file, R, Coff>>` — [`CoffSection`](#coffsection)

- <span id="cofffile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<CoffSection<'data, '_, R, Coff>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`CoffSection`](#coffsection)

- <span id="cofffile-object-sections"></span>`fn sections(&self) -> CoffSectionIterator<'data, '_, R, Coff>` — [`CoffSectionIterator`](#coffsectioniterator)

- <span id="cofffile-object-comdats"></span>`fn comdats(&self) -> CoffComdatIterator<'data, '_, R, Coff>` — [`CoffComdatIterator`](#coffcomdatiterator)

- <span id="cofffile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R, Coff>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`CoffSymbol`](#coffsymbol)

- <span id="cofffile-object-symbols"></span>`fn symbols(&self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](#coffsymboliterator)

- <span id="cofffile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](#coffsymboltable)

- <span id="cofffile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](#coffsymboliterator)

- <span id="cofffile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](#coffsymboltable)

- <span id="cofffile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md#nodynamicrelocationiterator)

- <span id="cofffile-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="cofffile-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="cofffile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="cofffile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="cofffile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="cofffile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffFile<'data, R, Coff>`

### `SectionTable<'data>`

```rust
struct SectionTable<'data> {
    sections: &'data [pe::ImageSectionHeader],
}
```

The table of section headers in a COFF or PE file.

Returned by `CoffHeader::sections` and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the section table.

  

  `data` must be the entire file data.

  `offset` must be after the optional file header.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Iterate over the section headers.

  

  Warning: section indices start at 1.

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../index.md#sectionindex), [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Iterate over the section headers and their indices.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Return the section header at the given index.

  

  The index is 1-based.

- <span id="sectiontable-section-by-name"></span>`fn section_by_name<R: ReadRef<'data>>(&self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../index.md#stringtable), [`SectionIndex`](../../index.md#sectionindex), [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Return the section header with the given name.

  

  The returned index is 1-based.

  

  Ignores sections with invalid names.

- <span id="sectiontable-max-section-file-offset"></span>`fn max_section_file_offset(&self) -> u64`

  Compute the maximum file offset used by sections.

  

  This will usually match the end of file, unless the PE file has a

  [data overlay](https://security.stackexchange.com/questions/77336/how-is-the-file-overlay-read-by-an-exe-virus)

#### Trait Implementations

##### `impl Clone for SectionTable<'data>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

##### `impl Copy for SectionTable<'data>`

##### `impl Debug for SectionTable<'data>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionTable<'data>`

- <span id="sectiontable-default"></span>`fn default() -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

### `CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

An iterator for the loadable sections in a [`CoffFile`](#cofffile).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-iterator-type-item"></span>`type Item = CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    section: &'data pe::ImageSectionHeader,
}
```

A loadable section in a [`CoffFile`](#cofffile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- <span id="coffsegment-coff-file"></span>`fn coff_file(&self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](#cofffile)

  Get the COFF file containing this segment.

- <span id="coffsegment-coff-section"></span>`fn coff_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Get the raw COFF section header.

- <span id="coffsegment-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSegment for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="coffsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="coffsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="coffsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="coffsegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="coffsegment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="coffsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="coffsegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="coffsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSegment<'data, 'file, R, Coff>`

### `CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

An iterator for the sections in a [`CoffFile`](#cofffile).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-iterator-type-item"></span>`type Item = CoffSection<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    index: crate::read::SectionIndex,
    section: &'data pe::ImageSectionHeader,
}
```

A section in a [`CoffFile`](#cofffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="coffsection-coff-file"></span>`fn coff_file(&self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](#cofffile)

  Get the COFF file containing this section.

- <span id="coffsection-coff-section"></span>`fn coff_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Get the raw COFF section header.

- <span id="coffsection-coff-relocations"></span>`fn coff_relocations(&self) -> Result<&'data [pe::ImageRelocation]>` — [`Result`](../../index.md#result), [`ImageRelocation`](../../pe/index.md#imagerelocation)

  Get the raw COFF relocations for this section.

- <span id="coffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSection for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="coffsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="coffsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="coffsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="coffsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="coffsection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="coffsection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="coffsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="coffsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="coffsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="coffsection-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="coffsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="coffsection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="coffsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="coffsection-objectsection-relocations"></span>`fn relocations(&self) -> CoffRelocationIterator<'data, 'file, R, Coff>` — [`CoffRelocationIterator`](#coffrelocationiterator)

- <span id="coffsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="coffsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSection<'data, 'file, R, Coff>`

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

- <span id="symboltable-parse"></span>`fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Read the symbol table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbol table entries.

  

  This includes auxiliary symbol table entries.

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](#symboliterator)

  Iterate over the symbols.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`CoffHeader`](#coffheader)

  Return the symbol table entry at the given index.

- <span id="symboltable-aux-function"></span>`fn aux_function(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ImageAuxSymbolFunction`](../../pe/index.md#imageauxsymbolfunction)

  Return the auxiliary function symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-section"></span>`fn aux_section(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ImageAuxSymbolSection`](../../pe/index.md#imageauxsymbolsection)

  Return the auxiliary section symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-weak-external"></span>`fn aux_weak_external(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ImageAuxSymbolWeak`](../../pe/index.md#imageauxsymbolweak)

  Return the auxiliary weak external symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-file-name"></span>`fn aux_file_name(&self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result)

  Return the auxiliary file name for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result)

  Return the symbol table entry or auxiliary record at the given index and offset.

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../index.md#symbolmap)

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

- <span id="coffsymboltable-clone"></span>`fn clone(&self) -> CoffSymbolTable<'data, 'file, R, Coff>` — [`CoffSymbolTable`](#coffsymboltable)

##### `impl<R, Coff> Copy for CoffSymbolTable<'data, 'file, R, Coff>`

##### `impl<R, Coff> Debug for CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="coffsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbolTable for CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="coffsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md#objectsymboltable)

- <span id="coffsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ObjectSymbolTable`](../index.md#objectsymboltable)

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

- <span id="coffsymboliterator-new"></span>`fn new(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](file/index.md#coffcommon)

- <span id="coffsymboliterator-empty"></span>`fn empty(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](file/index.md#coffcommon)

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

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- <span id="coffsymbol-raw-symbol"></span>`fn raw_symbol(&self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](#coffheader)

  Get the raw `ImageSymbol` struct.

- <span id="coffsymbol-coff-symbol"></span>`fn coff_symbol(&self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](#coffheader)

  Get the raw `ImageSymbol` struct.

#### Trait Implementations

##### `impl<R, Coff> Clone for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-clone"></span>`fn clone(&self) -> CoffSymbol<'data, 'file, R, Coff>` — [`CoffSymbol`](#coffsymbol)

##### `impl<R, Coff> Copy for CoffSymbol<'data, 'file, R, Coff>`

##### `impl<R, Coff> Debug for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbol for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="coffsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="coffsymbol-objectsymbol-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="coffsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="coffsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="coffsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md#symbolkind)

- <span id="coffsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md#symbolsection)

- <span id="coffsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="coffsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md#symbolscope)

- <span id="coffsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="coffsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="coffsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md#symbolflags), [`SectionIndex`](../../index.md#sectionindex), [`SymbolIndex`](../../index.md#symbolindex)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbol<'data, 'file, R, Coff>`

### `CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageRelocation>,
}
```

An iterator for the relocations in a [`CoffSection`](super::CoffSection).

#### Trait Implementations

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="coffrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffComdatIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdatIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the COMDAT section groups in a [`CoffFile`](#cofffile).

#### Implementations

- <span id="coffcomdatiterator-new"></span>`fn new(file: &'file CoffFile<'data, R, Coff>) -> Self` — [`CoffFile`](#cofffile)

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-iterator-type-item"></span>`type Item = CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffComdat<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdat<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    symbol_index: crate::read::SymbolIndex,
    symbol: &'data <Coff as >::ImageSymbol,
    selection: u8,
}
```

A COMDAT section group in a [`CoffFile`](#cofffile).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Implementations

- <span id="coffcomdat-parse"></span>`fn parse(file: &'file CoffFile<'data, R, Coff>, section_symbol: &'data <Coff as >::ImageSymbol, index: SymbolIndex) -> Option<CoffComdat<'data, 'file, R, Coff>>` — [`CoffFile`](#cofffile), [`CoffHeader`](#coffheader), [`SymbolIndex`](../../index.md#symbolindex), [`CoffComdat`](#coffcomdat)

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectComdat for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="coffcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="coffcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="coffcomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="coffcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md#objectcomdat)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffComdat<'data, 'file, R, Coff>`

### `CoffComdatSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdatSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    section_number: i32,
    index: crate::read::SymbolIndex,
}
```

An iterator for the sections in a COMDAT section group in a [`CoffFile`](#cofffile).

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="coffcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ImportFile<'data>`

```rust
struct ImportFile<'data> {
    header: &'data pe::ImportObjectHeader,
    kind: ImportType,
    dll: crate::read::ByteString<'data>,
    symbol: crate::read::ByteString<'data>,
    import: Option<crate::read::ByteString<'data>>,
}
```

A Windows short form description of a symbol to import.

Used in Windows import libraries to provide a mapping from
a symbol name to a DLL export. This is not an object file.

This is a file that starts with [`pe::ImportObjectHeader`](../../pe/index.md), and corresponds
to [`crate::FileKind::CoffImport`](../../index.md).

#### Implementations

- <span id="importfile-parse"></span>`fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse it.

- <span id="importfile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

  Get the machine type.

- <span id="importfile-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md#subarchitecture)

  Get the sub machine type, if available.

- <span id="importfile-symbol"></span>`fn symbol(&self) -> &'data [u8]`

  The public symbol name.

- <span id="importfile-dll"></span>`fn dll(&self) -> &'data [u8]`

  The name of the DLL to import the symbol from.

- <span id="importfile-import"></span>`fn import(&self) -> ImportName<'data>` — [`ImportName`](#importname)

  The name exported from the DLL.

- <span id="importfile-import-type"></span>`fn import_type(&self) -> ImportType` — [`ImportType`](#importtype)

  The type of import. Usually either a function or data.

#### Trait Implementations

##### `impl Clone for ImportFile<'data>`

- <span id="importfile-clone"></span>`fn clone(&self) -> ImportFile<'data>` — [`ImportFile`](#importfile)

##### `impl Debug for ImportFile<'data>`

- <span id="importfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportObjectData<'data>`

```rust
struct ImportObjectData<'data> {
    symbol: crate::read::ByteString<'data>,
    dll: crate::read::ByteString<'data>,
    export: Option<crate::read::ByteString<'data>>,
}
```

The data following [`pe::ImportObjectHeader`](../../pe/index.md).

#### Implementations

- <span id="importobjectdata-symbol"></span>`fn symbol(&self) -> &'data [u8]`

  The public symbol name.

- <span id="importobjectdata-dll"></span>`fn dll(&self) -> &'data [u8]`

  The name of the DLL to import the symbol from.

- <span id="importobjectdata-export"></span>`fn export(&self) -> Option<&'data [u8]>`

  The name exported from the DLL.

  

  This is only set if the name is not derived from the symbol name.

#### Trait Implementations

##### `impl Clone for ImportObjectData<'data>`

- <span id="importobjectdata-clone"></span>`fn clone(&self) -> ImportObjectData<'data>` — [`ImportObjectData`](#importobjectdata)

##### `impl Debug for ImportObjectData<'data>`

- <span id="importobjectdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ImportName<'data>`

```rust
enum ImportName<'data> {
    Ordinal(u16),
    Name(&'data [u8]),
}
```

The name or ordinal to import from a DLL.

#### Variants

- **`Ordinal`**

  Import by ordinal. Ordinarily this is a 1-based index.

- **`Name`**

  Import by name.

#### Trait Implementations

##### `impl Clone for ImportName<'data>`

- <span id="importname-clone"></span>`fn clone(&self) -> ImportName<'data>` — [`ImportName`](#importname)

##### `impl Copy for ImportName<'data>`

##### `impl Debug for ImportName<'data>`

- <span id="importname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImportName<'data>`

##### `impl<K> Equivalent for ImportName<'data>`

- <span id="importname-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for ImportName<'data>`

- <span id="importname-partialeq-eq"></span>`fn eq(&self, other: &ImportName<'data>) -> bool` — [`ImportName`](#importname)

##### `impl StructuralPartialEq for ImportName<'data>`

### `ImportType`

```rust
enum ImportType {
    Code,
    Data,
    Const,
}
```

The kind of import symbol.

#### Variants

- **`Code`**

  An executable code symbol.

- **`Data`**

  A data symbol.

- **`Const`**

  A constant value.

#### Trait Implementations

##### `impl Clone for ImportType`

- <span id="importtype-clone"></span>`fn clone(&self) -> ImportType` — [`ImportType`](#importtype)

##### `impl Copy for ImportType`

##### `impl Debug for ImportType`

- <span id="importtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImportType`

##### `impl<K> Equivalent for ImportType`

- <span id="importtype-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ImportType`

- <span id="importtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ImportType`

- <span id="importtype-partialeq-eq"></span>`fn eq(&self, other: &ImportType) -> bool` — [`ImportType`](#importtype)

##### `impl StructuralPartialEq for ImportType`

## Traits

### `CoffHeader`

```rust
trait CoffHeader: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageFileHeader`](../../pe/index.md) and [`pe::AnonObjectHeaderBigobj`](../../pe/index.md).

#### Associated Types

- `type ImageSymbol: 1`

- `type ImageSymbolBytes: 2`

#### Required Methods

- `fn is_type_bigobj() -> bool`

  Return true if this type is [`pe::AnonObjectHeaderBigobj`](../../pe/index.md).

- `fn machine(&self) -> u16`

- `fn number_of_sections(&self) -> u32`

- `fn pointer_to_symbol_table(&self) -> u32`

- `fn number_of_symbols(&self) -> u32`

- `fn characteristics(&self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>`

  Read the file header.

#### Provided Methods

- `fn sections<'data, R: ReadRef<'data>>(&self, data: R, offset: u64) -> read::Result<SectionTable<'data>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<SymbolTable<'data, R, Self>>`

  Read the symbol table and string table.

#### Implementors

- [`AnonObjectHeaderBigobj`](../../pe/index.md#anonobjectheaderbigobj)
- [`ImageFileHeader`](../../pe/index.md#imagefileheader)

### `ImageSymbol`

```rust
trait ImageSymbol: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageSymbol`](../../pe/index.md) and [`pe::ImageSymbolEx`](../../pe/index.md).

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

- [`ImageSymbolEx`](../../pe/index.md#imagesymbolex)
- [`ImageSymbol`](../../pe/index.md#imagesymbol)

## Functions

### `anon_object_class_id`

```rust
fn anon_object_class_id<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<pe::ClsId>
```

Read the `class_id` field from a [`pe::AnonObjectHeader`](../../pe/index.md).

This can be used to determine the format of the header.

## Type Aliases

### `CoffBigFile<'data, R>`

```rust
type CoffBigFile<'data, R> = CoffFile<'data, R, pe::AnonObjectHeaderBigobj>;
```

A COFF bigobj object file with 32-bit section numbers.

This is a file that starts with [`pe::AnonObjectHeaderBigobj`](../../pe/index.md), and corresponds
to [`crate::FileKind::CoffBig`](../../index.md).

Most functionality is provided by the [`Object`](../index.md) trait implementation.

### `CoffBigSegmentIterator<'data, 'file, R>`

```rust
type CoffBigSegmentIterator<'data, 'file, R> = CoffSegmentIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSegment<'data, 'file, R>`

```rust
type CoffBigSegment<'data, 'file, R> = CoffSegment<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A loadable section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

### `CoffBigSectionIterator<'data, 'file, R>`

```rust
type CoffBigSectionIterator<'data, 'file, R> = CoffSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSection<'data, 'file, R>`

```rust
type CoffBigSection<'data, 'file, R> = CoffSection<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

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

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

### `CoffBigRelocationIterator<'data, 'file, R>`

```rust
type CoffBigRelocationIterator<'data, 'file, R> = CoffRelocationIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection).

### `CoffBigComdatIterator<'data, 'file, R>`

```rust
type CoffBigComdatIterator<'data, 'file, R> = CoffComdatIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigComdat<'data, 'file, R>`

```rust
type CoffBigComdat<'data, 'file, R> = CoffComdat<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

### `CoffBigComdatSectionIterator<'data, 'file, R>`

```rust
type CoffBigComdatSectionIterator<'data, 'file, R> = CoffComdatSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

