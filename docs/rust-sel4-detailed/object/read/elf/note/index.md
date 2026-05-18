*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [note](index.md)*

---

# Module `note`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoteIterator`](#noteiterator) | struct | An iterator over the notes in an ELF section or segment. |
| [`Note`](#note) | struct | A parsed [`NoteHeader`]. |
| [`GnuPropertyIterator`](#gnupropertyiterator) | struct | An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note. |
| [`GnuProperty`](#gnuproperty) | struct | A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note. |
| [`NoteHeader`](#noteheader) | trait | A trait for generic access to [`elf::NoteHeader32`] and [`elf::NoteHeader64`]. |

## Structs

### `NoteIterator<'data, Elf>`

```rust
struct NoteIterator<'data, Elf>
where
    Elf: FileHeader {
    endian: <Elf as >::Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator over the notes in an ELF section or segment.

Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)
and [`SectionHeader::notes`](super::SectionHeader::notes).

#### Implementations

- <span id="noteiterator-new"></span>`fn new(endian: <Elf as >::Endian, align: <Elf as >::Word, data: &'data [u8]) -> read::Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

  An iterator over the notes in an ELF section or segment.

  

  `align` should be from the `p_align` field of the segment,

  or the `sh_addralign` field of the section. Supported values are

  either 4 or 8, but values less than 4 are treated as 4.

  This matches the behaviour of binutils.

  

  Returns `Err` if `align` is invalid.

- <span id="noteiterator-next"></span>`fn next(&mut self) -> read::Result<Option<Note<'data, Elf>>>` — [`Result`](../../../index.md#result), [`Note`](../index.md#note)

  Returns the next note.

- <span id="noteiterator-parse"></span>`fn parse(&mut self) -> read::Result<Note<'data, Elf>>` — [`Result`](../../../index.md#result), [`Note`](../index.md#note)

#### Trait Implementations

##### `impl<Elf> Debug for NoteIterator<'data, Elf>`

- <span id="noteiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="noteiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="noteiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-iterator-type-item"></span>`type Item = Result<Note<'data, Elf>, Error>`

- <span id="noteiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Note<'data, Elf>`

```rust
struct Note<'data, Elf>
where
    Elf: FileHeader {
    header: &'data <Elf as >::NoteHeader,
    name: &'data [u8],
    desc: &'data [u8],
}
```

A parsed [`NoteHeader`](../index.md).

#### Implementations

- <span id="note-n-type"></span>`fn n_type(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md#fileheader)

  Return the `n_type` field of the `NoteHeader`.

  

  The meaning of this field is determined by `name`.

- <span id="note-n-namesz"></span>`fn n_namesz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md#fileheader)

  Return the `n_namesz` field of the `NoteHeader`.

- <span id="note-n-descsz"></span>`fn n_descsz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md#fileheader)

  Return the `n_descsz` field of the `NoteHeader`.

- <span id="note-name-bytes"></span>`fn name_bytes(&self) -> &'data [u8]`

  Return the bytes for the name field following the `NoteHeader`.

  

  This field is usually a string including one or more trailing null bytes

  (but it is not required to be).

  

  The length of this field is given by `n_namesz`.

- <span id="note-name"></span>`fn name(&self) -> &'data [u8]`

  Return the bytes for the name field following the `NoteHeader`,

  excluding all trailing null bytes.

- <span id="note-desc"></span>`fn desc(&self) -> &'data [u8]`

  Return the bytes for the desc field following the `NoteHeader`.

  

  The length of this field is given by `n_descsz`. The meaning

  of this field is determined by `name` and `n_type`.

- <span id="note-gnu-properties"></span>`fn gnu_properties(&self, endian: <Elf as >::Endian) -> Option<GnuPropertyIterator<'data, <Elf as >::Endian>>` — [`FileHeader`](../index.md#fileheader), [`GnuPropertyIterator`](../index.md#gnupropertyiterator)

  Return an iterator for properties if this note's type is [`elf::NT_GNU_PROPERTY_TYPE_0`](../../../elf/index.md).

#### Trait Implementations

##### `impl<Elf> Debug for Note<'data, Elf>`

- <span id="note-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `GnuPropertyIterator<'data, Endian: endian::Endian>`

```rust
struct GnuPropertyIterator<'data, Endian: endian::Endian> {
    endian: Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../../elf/index.md) note.

Returned by `Note::gnu_properties`.

#### Implementations

- <span id="gnupropertyiterator-next"></span>`fn next(&mut self) -> read::Result<Option<GnuProperty<'data>>>` — [`Result`](../../../index.md#result), [`GnuProperty`](../index.md#gnuproperty)

  Returns the next property.

- <span id="gnupropertyiterator-parse"></span>`fn parse(&mut self) -> read::Result<GnuProperty<'data>>` — [`Result`](../../../index.md#result), [`GnuProperty`](../index.md#gnuproperty)

#### Trait Implementations

##### `impl<Endian: fmt::Debug + endian::Endian> Debug for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="gnupropertyiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="gnupropertyiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Endian: endian::Endian> Iterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-iterator-type-item"></span>`type Item = Result<GnuProperty<'data>, Error>`

- <span id="gnupropertyiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `GnuProperty<'data>`

```rust
struct GnuProperty<'data> {
    pr_type: u32,
    pr_data: &'data [u8],
}
```

A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../../elf/index.md) note.

#### Implementations

- <span id="gnuproperty-pr-type"></span>`fn pr_type(&self) -> u32`

  Return the property type.

  

  This is one of the `GNU_PROPERTY_*` constants.

- <span id="gnuproperty-pr-data"></span>`fn pr_data(&self) -> &'data [u8]`

  Return the property data.

- <span id="gnuproperty-data-u32"></span>`fn data_u32<E: endian::Endian>(&self, endian: E) -> read::Result<u32>` — [`Result`](../../../index.md#result)

  Parse the property data as an unsigned 32-bit integer.

#### Trait Implementations

##### `impl Debug for GnuProperty<'data>`

- <span id="gnuproperty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `NoteHeader`

```rust
trait NoteHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::NoteHeader32`](../../../elf/index.md) and [`elf::NoteHeader64`](../../../elf/index.md).

#### Associated Types

- `type Endian: 1`

#### Required Methods

- `fn n_namesz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_type(&self, endian: <Self as >::Endian) -> u32`

#### Implementors

- [`NoteHeader32`](../../../elf/index.md#noteheader32)
- [`NoteHeader64`](../../../elf/index.md#noteheader64)

