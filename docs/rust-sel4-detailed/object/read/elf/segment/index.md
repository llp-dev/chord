*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [segment](index.md)*

---

# Module `segment`

## Contents

- [Structs](#structs)
  - [`ElfSegmentIterator`](#elfsegmentiterator)
  - [`ElfSegment`](#elfsegment)
- [Traits](#traits)
  - [`ProgramHeader`](#programheader)
- [Type Aliases](#type-aliases)
  - [`ElfSegmentIterator32`](#elfsegmentiterator32)
  - [`ElfSegmentIterator64`](#elfsegmentiterator64)
  - [`ElfSegment32`](#elfsegment32)
  - [`ElfSegment64`](#elfsegment64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ElfSegmentIterator`](#elfsegmentiterator) | struct | An iterator for the segments in an [`ElfFile`]. |
| [`ElfSegment`](#elfsegment) | struct | A segment in an [`ElfFile`]. |
| [`ProgramHeader`](#programheader) | trait | A trait for generic access to [`elf::ProgramHeader32`] and [`elf::ProgramHeader64`]. |
| [`ElfSegmentIterator32`](#elfsegmentiterator32) | type | An iterator for the segments in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSegmentIterator64`](#elfsegmentiterator64) | type | An iterator for the segments in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSegment32`](#elfsegment32) | type | A segment in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSegment64`](#elfsegment64) | type | A segment in an [`ElfFile64`](super::ElfFile64). |

## Structs

### `ElfSegmentIterator<'data, 'file, Elf, R>`

```rust
struct ElfSegmentIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: slice::Iter<'data, <Elf as >::ProgramHeader>,
}
```

An iterator for the segments in an [`ElfFile`](../index.md).

#### Trait Implementations

##### `impl<Elf, R> Debug for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-iterator-type-item"></span>`type Item = ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ElfSegment<'data, 'file, Elf, R>`

```rust
struct ElfSegment<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    segment: &'data <Elf as >::ProgramHeader,
}
```

A segment in an [`ElfFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- <span id="elfsegment-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](../index.md#elffile)

  Get the ELF file containing this segment.

- <span id="elfsegment-elf-program-header"></span>`fn elf_program_header(&self) -> &'data <Elf as >::ProgramHeader` — [`FileHeader`](../index.md#fileheader)

  Get the raw ELF program header for the segment.

- <span id="elfsegment-bytes"></span>`fn bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl<Elf, R> Debug for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf, R> ObjectSegment for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="elfsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="elfsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="elfsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="elfsegment-objectsegment-data"></span>`fn data(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="elfsegment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="elfsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> read::Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="elfsegment-objectsegment-name"></span>`fn name(&self) -> read::Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="elfsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md#segmentflags)

##### `impl<Elf, R> Sealed for ElfSegment<'data, 'file, Elf, R>`

## Traits

### `ProgramHeader`

```rust
trait ProgramHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::ProgramHeader32`](../../../elf/index.md) and [`elf::ProgramHeader64`](../../../elf/index.md).

#### Associated Types

- `type Elf: 1`

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn p_type(&self, endian: <Self as >::Endian) -> u32`

- `fn p_flags(&self, endian: <Self as >::Endian) -> u32`

- `fn p_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_vaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_paddr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_filesz(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_memsz(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_align(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn file_range(&self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> Result<&'data [u8], ()>`

  Return the segment data.

- `fn data_as_array<'data, T: Pod, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> Result<&'data [T], ()>`

  Return the segment data as a slice of the given type.

- `fn data_range<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, address: u64, size: u64) -> Result<Option<&'data [u8]>, ()>`

  Return the segment data in the given virtual address range

- `fn dynamic<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [<<Self as >::Elf as FileHeader>::Dyn]>>`

  Return entries in a dynamic segment.

- `fn interpreter<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [u8]>>`

  Return the data in an interpreter segment.

- `fn notes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<NoteIterator<'data, <Self as >::Elf>>>`

  Return a note iterator for the segment data.

#### Implementors

- [`ProgramHeader32`](../../../elf/index.md#programheader32)
- [`ProgramHeader64`](../../../elf/index.md#programheader64)

## Type Aliases

### `ElfSegmentIterator32<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator32<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the segments in an [`ElfFile32`](super::ElfFile32).

### `ElfSegmentIterator64<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator64<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the segments in an [`ElfFile64`](super::ElfFile64).

### `ElfSegment32<'data, 'file, Endian, R>`

```rust
type ElfSegment32<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A segment in an [`ElfFile32`](super::ElfFile32).

### `ElfSegment64<'data, 'file, Endian, R>`

```rust
type ElfSegment64<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A segment in an [`ElfFile64`](super::ElfFile64).

