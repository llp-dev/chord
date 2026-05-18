*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`XcoffSectionIterator`](#xcoffsectioniterator)
  - [`XcoffSection`](#xcoffsection)
  - [`SectionTable`](#sectiontable)
- [Traits](#traits)
  - [`SectionHeader`](#sectionheader)
- [Type Aliases](#type-aliases)
  - [`XcoffSectionIterator32`](#xcoffsectioniterator32)
  - [`XcoffSectionIterator64`](#xcoffsectioniterator64)
  - [`XcoffSection32`](#xcoffsection32)
  - [`XcoffSection64`](#xcoffsection64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XcoffSectionIterator`](#xcoffsectioniterator) | struct | An iterator for the sections in an [`XcoffFile`]. |
| [`XcoffSection`](#xcoffsection) | struct | A section in an [`XcoffFile`]. |
| [`SectionTable`](#sectiontable) | struct | The table of section headers in an XCOFF file. |
| [`SectionHeader`](#sectionheader) | trait | A trait for generic access to [`xcoff::SectionHeader32`] and [`xcoff::SectionHeader64`]. |
| [`XcoffSectionIterator32`](#xcoffsectioniterator32) | type | An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSectionIterator64`](#xcoffsectioniterator64) | type | An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSection32`](#xcoffsection32) | type | A section in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSection64`](#xcoffsection64) | type | A section in an [`XcoffFile64`](super::XcoffFile64). |

## Structs

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

An iterator for the sections in an [`XcoffFile`](../index.md).

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

A section in an [`XcoffFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="xcoffsection-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](../index.md#xcofffile)

  Get the XCOFF file containing this section.

- <span id="xcoffsection-xcoff-section"></span>`fn xcoff_section(&self) -> &'data <Xcoff as >::SectionHeader` — [`FileHeader`](../index.md#fileheader)

  Get the raw XCOFF section header.

- <span id="xcoffsection-xcoff-relocations"></span>`fn xcoff_relocations(&self) -> Result<&'data [<Xcoff as >::Rel]>` — [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Get the raw XCOFF relocation entries for this section.

- <span id="xcoffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> ObjectSection for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

- <span id="xcoffsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="xcoffsection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

- <span id="xcoffsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md#result), [`CompressedData`](../../../index.md#compresseddata)

- <span id="xcoffsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md#sectionkind)

- <span id="xcoffsection-objectsection-relocations"></span>`fn relocations(&self) -> <Self as >::RelocationIterator` — [`ObjectSection`](../../index.md#objectsection)

- <span id="xcoffsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md#result), [`RelocationMap`](../../../index.md#relocationmap)

- <span id="xcoffsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md#sectionflags)

- <span id="xcoffsection-objectsection-uncompressed-data"></span>`fn uncompressed_data(&self) -> Result<alloc::borrow::Cow<'data, [u8]>>` — [`Result`](../../../index.md#result)

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

- <span id="sectiontable-parse"></span>`fn parse<R: ReadRef<'data>>(header: &Xcoff, data: R, offset: &mut u64) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the section table.

  

  `data` must be the entire file data.

  `offset` must be after the optional file header.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` — [`FileHeader`](../index.md#fileheader)

  Iterate over the section headers.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Return the section header at the given index.

  

  The index is 1-based.

#### Trait Implementations

##### `impl<Xcoff: clone::Clone + FileHeader> Clone for SectionTable<'data, Xcoff>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Xcoff>` — [`SectionTable`](../index.md#sectiontable)

##### `impl<Xcoff: marker::Copy + FileHeader> Copy for SectionTable<'data, Xcoff>`

##### `impl<Xcoff: fmt::Debug + FileHeader> Debug for SectionTable<'data, Xcoff>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff> Default for SectionTable<'data, Xcoff>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

## Traits

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::SectionHeader32`](../../../xcoff/index.md) and [`xcoff::SectionHeader64`](../../../xcoff/index.md).

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

- [`SectionHeader32`](../../../xcoff/index.md#sectionheader32)
- [`SectionHeader64`](../../../xcoff/index.md#sectionheader64)

## Type Aliases

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

