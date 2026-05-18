*[gimli](../../index.md) / [read](../index.md) / [aranges](index.md)*

---

# Module `aranges`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAranges`](#debugaranges) | struct | The `DebugAranges` struct represents the DWARF address range information found in the `.debug_aranges` section. |
| [`ArangeHeaderIter`](#arangeheaderiter) | struct | An iterator over the headers of a `.debug_aranges` section. |
| [`ArangeHeader`](#arangeheader) | struct | A header for a set of entries in the `.debug_arange` section. |
| [`ArangeEntryIter`](#arangeentryiter) | struct | An iterator over the aranges from a `.debug_aranges` section. |
| [`ArangeEntry`](#arangeentry) | struct | A single parsed arange. |

## Structs

### `DebugAranges<R>`

```rust
struct DebugAranges<R> {
    section: R,
}
```

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

#### Implementations

- <span id="debugaranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugAranges` instance from the data in the `.debug_aranges`

  section.

  

  It is the caller's responsibility to read the `.debug_aranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugAranges, LittleEndian};

  

  let buf = [];

  let read_debug_aranges_section = || &buf;

  let debug_aranges =

      DebugAranges::new(read_debug_aranges_section(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAranges<R>`

- <span id="debugaranges-clone"></span>`fn clone(&self) -> DebugAranges<R>` — [`DebugAranges`](../index.md#debugaranges)

##### `impl<R: marker::Copy> Copy for DebugAranges<R>`

##### `impl<R: fmt::Debug> Debug for DebugAranges<R>`

- <span id="debugaranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAranges<R>`

- <span id="debugaranges-default"></span>`fn default() -> DebugAranges<R>` — [`DebugAranges`](../index.md#debugaranges)

##### `impl<R> Section for DebugAranges<R>`

- <span id="debugaranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugaranges-section-reader"></span>`fn reader(&self) -> &R`

### `ArangeHeaderIter<R: Reader>`

```rust
struct ArangeHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugArangesOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_aranges` section.

#### Implementations

- <span id="arangeheaderiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeHeader<R>>>` — [`Result`](../../index.md#result), [`ArangeHeader`](../index.md#arangeheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-clone"></span>`fn clone(&self) -> ArangeHeaderIter<R>` — [`ArangeHeaderIter`](../index.md#arangeheaderiter)

##### `impl<R: fmt::Debug + Reader> Debug for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arangeheaderiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arangeheaderiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-iterator-type-item"></span>`type Item = Result<ArangeHeader<R>, Error>`

- <span id="arangeheaderiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ArangeHeader<R, Offset>`

```rust
struct ArangeHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugArangesOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    debug_info_offset: crate::common::DebugInfoOffset<Offset>,
    entries: R,
}
```

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

#### Implementations

- <span id="arangeheader-parse"></span>`fn parse(input: &mut R, offset: DebugArangesOffset<Offset>) -> Result<Self>` — [`DebugArangesOffset`](../../index.md#debugarangesoffset), [`Result`](../../index.md#result)

- <span id="arangeheader-offset"></span>`fn offset(&self) -> DebugArangesOffset<Offset>` — [`DebugArangesOffset`](../../index.md#debugarangesoffset)

  Return the offset of this header within the `.debug_aranges` section.

- <span id="arangeheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="arangeheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="arangeheader-debug-info-offset"></span>`fn debug_info_offset(&self) -> DebugInfoOffset<Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset)

  Return the offset into the .debug_info section for this set of arange entries.

- <span id="arangeheader-entries"></span>`fn entries(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md#arangeentryiter)

  Return the arange entries in this set.

#### Trait Implementations

##### `impl<R, Offset> Clone for ArangeHeader<R, Offset>`

- <span id="arangeheader-clone"></span>`fn clone(&self) -> ArangeHeader<R, Offset>` — [`ArangeHeader`](../index.md#arangeheader)

##### `impl<R, Offset> Debug for ArangeHeader<R, Offset>`

- <span id="arangeheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for ArangeHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for ArangeHeader<R, Offset>`

- <span id="arangeheader-partialeq-eq"></span>`fn eq(&self, other: &ArangeHeader<R, Offset>) -> bool` — [`ArangeHeader`](../index.md#arangeheader)

##### `impl<R, Offset> StructuralPartialEq for ArangeHeader<R, Offset>`

### `ArangeEntryIter<R: Reader>`

```rust
struct ArangeEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the aranges from a `.debug_aranges` section.

#### Implementations

- <span id="arangeentryiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md#result), [`ArangeEntry`](../index.md#arangeentry)

  Advance the iterator and return the next arange.

  

  Returns the newly parsed arange as `Ok(Some(arange))`. Returns `Ok(None)`

  when iteration is complete and all aranges have already been parsed and

  yielded. If an error occurs while parsing the next arange, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

- <span id="arangeentryiter-next-raw"></span>`fn next_raw(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md#result), [`ArangeEntry`](../index.md#arangeentry)

  Advance the iterator and return the next arange without validating it.

  

  The returned entry will have `range.end` set to 0.

  This will return tombstone entries as well.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ArangeEntryIter<R>`

- <span id="arangeentryiter-clone"></span>`fn clone(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md#arangeentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for ArangeEntryIter<R>`

- <span id="arangeentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ArangeEntryIter<R>`

- <span id="arangeentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arangeentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arangeentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for ArangeEntryIter<R>`

- <span id="arangeentryiter-iterator-type-item"></span>`type Item = Result<ArangeEntry, Error>`

- <span id="arangeentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ArangeEntry`

```rust
struct ArangeEntry {
    range: crate::read::Range,
    length: u64,
}
```

A single parsed arange.

#### Implementations

- <span id="arangeentry-parse"></span>`fn parse<R: Reader>(input: &mut R, encoding: Encoding) -> Result<Option<Self>>` — [`Encoding`](../../index.md#encoding), [`Result`](../../index.md#result)

  Parse a single arange. Return `None` for the null arange, `Some` for an actual arange.

- <span id="arangeentry-address"></span>`fn address(&self) -> u64`

  Return the beginning address of this arange.

- <span id="arangeentry-length"></span>`fn length(&self) -> u64`

  Return the length of this arange.

- <span id="arangeentry-range"></span>`fn range(&self) -> Range` — [`Range`](../index.md#range)

  Return the range.

#### Trait Implementations

##### `impl Clone for ArangeEntry`

- <span id="arangeentry-clone"></span>`fn clone(&self) -> ArangeEntry` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl Debug for ArangeEntry`

- <span id="arangeentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArangeEntry`

##### `impl Ord for ArangeEntry`

- <span id="arangeentry-ord-cmp"></span>`fn cmp(&self, other: &ArangeEntry) -> cmp::Ordering` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl PartialEq for ArangeEntry`

- <span id="arangeentry-partialeq-eq"></span>`fn eq(&self, other: &ArangeEntry) -> bool` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl PartialOrd for ArangeEntry`

- <span id="arangeentry-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArangeEntry) -> option::Option<cmp::Ordering>` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl StructuralPartialEq for ArangeEntry`

