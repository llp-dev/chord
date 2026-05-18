*[gimli](../../index.md) / [read](../index.md) / [pubtypes](index.md)*

---

# Module `pubtypes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PubTypesEntry`](#pubtypesentry) | struct | A single parsed pubtype. |
| [`DebugPubTypes`](#debugpubtypes) | struct | The `DebugPubTypes` struct represents the DWARF public types information found in the `.debug_info` section. |
| [`PubTypesEntryIter`](#pubtypesentryiter) | struct | An iterator over the pubtypes from a `.debug_pubtypes` section. |

## Structs

### `PubTypesEntry<R: Reader>`

```rust
struct PubTypesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubtype.

#### Implementations

- <span id="pubtypesentry-name"></span>`fn name(&self) -> &R`

  Returns the name of the type this entry refers to.

- <span id="pubtypesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains the type with this name.

- <span id="pubtypesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  the type with this name.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntry<R>`

- <span id="pubtypesentry-clone"></span>`fn clone(&self) -> PubTypesEntry<R>` — [`PubTypesEntry`](../index.md#pubtypesentry)

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntry<R>`

- <span id="pubtypesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubTypesEntry<R>`

- <span id="pubtypesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`DebugInfoOffset`](../../index.md#debuginfooffset)

### `DebugPubTypes<R: Reader>`

```rust
struct DebugPubTypes<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

The `DebugPubTypes` struct represents the DWARF public types information
found in the `.debug_info` section.

#### Implementations

- <span id="debugpubtypes-new"></span>`fn new(debug_pubtypes_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubTypes` instance from the data in the `.debug_pubtypes`

  section.

  

  It is the caller's responsibility to read the `.debug_pubtypes` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubTypes, LittleEndian};

  

  let buf = [];

  let read_debug_pubtypes_somehow = || &buf;

  let debug_pubtypes =

      DebugPubTypes::new(read_debug_pubtypes_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugPubTypes<R>`

- <span id="debugpubtypes-clone"></span>`fn clone(&self) -> DebugPubTypes<R>` — [`DebugPubTypes`](../index.md#debugpubtypes)

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubTypes<R>`

- <span id="debugpubtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Section for DebugPubTypes<R>`

- <span id="debugpubtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugpubtypes-section-reader"></span>`fn reader(&self) -> &R`

### `PubTypesEntryIter<R: Reader>`

```rust
struct PubTypesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

An iterator over the pubtypes from a `.debug_pubtypes` section.

#### Implementations

- <span id="pubtypesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubTypesEntry<R>>>` — [`Result`](../../index.md#result), [`PubTypesEntry`](../index.md#pubtypesentry)

  Advance the iterator and return the next pubtype.

  

  Returns the newly parsed pubtype as `Ok(Some(pubtype))`. Returns

  `Ok(None)` when iteration is complete and all pubtypes have already been

  parsed and yielded. If an error occurs while parsing the next pubtype,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-clone"></span>`fn clone(&self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](../index.md#pubtypesentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pubtypesentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pubtypesentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-iterator-type-item"></span>`type Item = Result<PubTypesEntry<R>, Error>`

- <span id="pubtypesentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

