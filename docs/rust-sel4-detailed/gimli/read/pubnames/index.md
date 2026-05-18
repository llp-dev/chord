*[gimli](../../index.md) / [read](../index.md) / [pubnames](index.md)*

---

# Module `pubnames`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PubNamesEntry`](#pubnamesentry) | struct | A single parsed pubname. |
| [`DebugPubNames`](#debugpubnames) | struct | The `DebugPubNames` struct represents the DWARF public names information found in the `.debug_pubnames` section. |
| [`PubNamesEntryIter`](#pubnamesentryiter) | struct | An iterator over the pubnames from a `.debug_pubnames` section. |

## Structs

### `PubNamesEntry<R: Reader>`

```rust
struct PubNamesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubname.

#### Implementations

- <span id="pubnamesentry-name"></span>`fn name(&self) -> &R`

  Returns the name this entry refers to.

- <span id="pubnamesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains this name.

- <span id="pubnamesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  has this name.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntry<R>`

- <span id="pubnamesentry-clone"></span>`fn clone(&self) -> PubNamesEntry<R>` — [`PubNamesEntry`](../index.md#pubnamesentry)

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntry<R>`

- <span id="pubnamesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubNamesEntry<R>`

- <span id="pubnamesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`DebugInfoOffset`](../../index.md#debuginfooffset)

### `DebugPubNames<R: Reader>`

```rust
struct DebugPubNames<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

The `DebugPubNames` struct represents the DWARF public names information
found in the `.debug_pubnames` section.

#### Implementations

- <span id="debugpubnames-new"></span>`fn new(debug_pubnames_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubNames` instance from the data in the `.debug_pubnames`

  section.

  

  It is the caller's responsibility to read the `.debug_pubnames` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubNames, LittleEndian};

  

  let buf = [];

  let read_debug_pubnames_section_somehow = || &buf;

  let debug_pubnames =

      DebugPubNames::new(read_debug_pubnames_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugPubNames<R>`

- <span id="debugpubnames-clone"></span>`fn clone(&self) -> DebugPubNames<R>` — [`DebugPubNames`](../index.md#debugpubnames)

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubNames<R>`

- <span id="debugpubnames-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Section for DebugPubNames<R>`

- <span id="debugpubnames-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugpubnames-section-reader"></span>`fn reader(&self) -> &R`

### `PubNamesEntryIter<R: Reader>`

```rust
struct PubNamesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

An iterator over the pubnames from a `.debug_pubnames` section.

#### Implementations

- <span id="pubnamesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubNamesEntry<R>>>` — [`Result`](../../index.md#result), [`PubNamesEntry`](../index.md#pubnamesentry)

  Advance the iterator and return the next pubname.

  

  Returns the newly parsed pubname as `Ok(Some(pubname))`. Returns

  `Ok(None)` when iteration is complete and all pubnames have already been

  parsed and yielded. If an error occurs while parsing the next pubname,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-clone"></span>`fn clone(&self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](../index.md#pubnamesentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pubnamesentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pubnamesentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-iterator-type-item"></span>`type Item = Result<PubNamesEntry<R>, Error>`

- <span id="pubnamesentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

