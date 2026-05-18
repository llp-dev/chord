*[gimli](../../index.md) / [read](../index.md) / [rnglists](index.md)*

---

# Module `rnglists`

## Contents

- [Structs](#structs)
  - [`DebugRanges`](#debugranges)
  - [`DebugRngLists`](#debugrnglists)
  - [`RangeLists`](#rangelists)
  - [`RawRngListIter`](#rawrnglistiter)
  - [`RngListIter`](#rnglistiter)
  - [`RawRange`](#rawrange)
  - [`Range`](#range)
- [Enums](#enums)
  - [`RangeListsFormat`](#rangelistsformat)
  - [`RawRngListEntry`](#rawrnglistentry)
- [Type Aliases](#type-aliases)
  - [`RngListsHeader`](#rnglistsheader)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugRanges`](#debugranges) | struct | The raw contents of the `.debug_ranges` section. |
| [`DebugRngLists`](#debugrnglists) | struct | The `DebugRngLists` struct represents the contents of the `.debug_rnglists` section. |
| [`RangeLists`](#rangelists) | struct | The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections. |
| [`RawRngListIter`](#rawrnglistiter) | struct | A raw iterator over an address range list. |
| [`RngListIter`](#rnglistiter) | struct | An iterator over an address range list. |
| [`RawRange`](#rawrange) | struct | A raw address range from the `.debug_ranges` section. |
| [`Range`](#range) | struct | An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections. |
| [`RangeListsFormat`](#rangelistsformat) | enum |  |
| [`RawRngListEntry`](#rawrnglistentry) | enum | A raw entry in .debug_rnglists |
| [`RngListsHeader`](#rnglistsheader) | type |  |

## Structs

### `DebugRanges<R>`

```rust
struct DebugRanges<R> {
    section: R,
}
```

The raw contents of the `.debug_ranges` section.

#### Implementations

- <span id="debugranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRanges` instance from the data in the `.debug_ranges`

  section.

  

  It is the caller's responsibility to read the `.debug_ranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRanges, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_ranges_section_somehow = || &buf;

  let debug_ranges = DebugRanges::new(read_debug_ranges_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugRanges<R>`

- <span id="debugranges-clone"></span>`fn clone(&self) -> DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

##### `impl<R: marker::Copy> Copy for DebugRanges<R>`

##### `impl<R: fmt::Debug> Debug for DebugRanges<R>`

- <span id="debugranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRanges<R>`

- <span id="debugranges-default"></span>`fn default() -> DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

##### `impl<R> Section for DebugRanges<R>`

- <span id="debugranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugranges-section-reader"></span>`fn reader(&self) -> &R`

### `DebugRngLists<R>`

```rust
struct DebugRngLists<R> {
    section: R,
}
```

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

#### Implementations

- <span id="debugrnglists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRngLists` instance from the data in the

  `.debug_rnglists` section.

  

  It is the caller's responsibility to read the `.debug_rnglists`

  section and present it as a `&[u8]` slice. That means using some ELF

  loader on Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRngLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_rnglists_section_somehow = || &buf;

  let debug_rnglists =

      DebugRngLists::new(read_debug_rnglists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugRngLists<R>`

- <span id="debugrnglists-clone"></span>`fn clone(&self) -> DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

##### `impl<R: marker::Copy> Copy for DebugRngLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugRngLists<R>`

- <span id="debugrnglists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRngLists<R>`

- <span id="debugrnglists-default"></span>`fn default() -> DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

##### `impl<R> Section for DebugRngLists<R>`

- <span id="debugrnglists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugrnglists-section-reader"></span>`fn reader(&self) -> &R`

### `RangeLists<R>`

```rust
struct RangeLists<R> {
    debug_ranges: DebugRanges<R>,
    debug_rnglists: DebugRngLists<R>,
}
```

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

#### Implementations

- <span id="rangelists-new"></span>`fn new(debug_ranges: DebugRanges<R>, debug_rnglists: DebugRngLists<R>) -> RangeLists<R>` — [`DebugRanges`](../index.md#debugranges), [`DebugRngLists`](../index.md#debugrnglists), [`RangeLists`](../index.md#rangelists)

  Construct a new `RangeLists` instance from the data in the `.debug_ranges` and

  `.debug_rnglists` sections.

- <span id="rangelists-debug-ranges"></span>`fn debug_ranges(&self) -> &DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

  Return the `.debug_ranges` section.

- <span id="rangelists-set-debug-ranges"></span>`fn set_debug_ranges(&mut self, debug_ranges: DebugRanges<R>)` — [`DebugRanges`](../index.md#debugranges)

  Replace the `.debug_ranges` section.

  

  This is useful for `.dwo` files when using the GNU split-dwarf extension to DWARF 4.

- <span id="rangelists-debug-rnglists"></span>`fn debug_rnglists(&self) -> &DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

  Return the `.debug_rnglists` section.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for RangeLists<R>`

- <span id="rangelists-clone"></span>`fn clone(&self) -> RangeLists<R>` — [`RangeLists`](../index.md#rangelists)

##### `impl<R: marker::Copy> Copy for RangeLists<R>`

##### `impl<R: fmt::Debug> Debug for RangeLists<R>`

- <span id="rangelists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for RangeLists<R>`

- <span id="rangelists-default"></span>`fn default() -> RangeLists<R>` — [`RangeLists`](../index.md#rangelists)

### `RawRngListIter<R: Reader>`

```rust
struct RawRngListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: RangeListsFormat,
}
```

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

#### Implementations

- <span id="rawrnglistiter-new"></span>`fn new(input: R, encoding: Encoding, format: RangeListsFormat) -> RawRngListIter<R>` — [`Encoding`](../../index.md#encoding), [`RangeListsFormat`](#rangelistsformat), [`RawRngListIter`](../index.md#rawrnglistiter)

  Construct a `RawRngListIter`.

- <span id="rawrnglistiter-next"></span>`fn next(&mut self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` — [`Result`](../../index.md#result), [`RawRngListEntry`](../index.md#rawrnglistentry), [`Reader`](../index.md#reader)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RawRngListIter<R>`

- <span id="rawrnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RawRngListIter<R>`

- <span id="rawrnglistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawrnglistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawrnglistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RawRngListIter<R>`

- <span id="rawrnglistiter-iterator-type-item"></span>`type Item = Result<RawRngListEntry<<R as Reader>::Offset>, Error>`

- <span id="rawrnglistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RngListIter<R: Reader>`

```rust
struct RngListIter<R: Reader> {
    raw: RawRngListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="rnglistiter-new"></span>`fn new(raw: RawRngListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> RngListIter<R>` — [`RawRngListIter`](../index.md#rawrnglistiter), [`DebugAddr`](../index.md#debugaddr), [`DebugAddrBase`](../../index.md#debugaddrbase), [`Reader`](../index.md#reader), [`RngListIter`](../index.md#rnglistiter)

  Construct a `RngListIter`.

- <span id="rnglistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="rnglistiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../../index.md#result), [`Range`](../index.md#range)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RngListIter<R>`

- <span id="rnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RngListIter<R>`

- <span id="rnglistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rnglistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rnglistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RngListIter<R>`

- <span id="rnglistiter-iterator-type-item"></span>`type Item = Result<Range, Error>`

- <span id="rnglistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RawRange`

```rust
struct RawRange {
    pub begin: u64,
    pub end: u64,
}
```

A raw address range from the `.debug_ranges` section.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="rawrange-is-end"></span>`fn is_end(&self) -> bool`

  Check if this is a range end entry.

- <span id="rawrange-is-base-address"></span>`fn is_base_address(&self, address_size: u8) -> bool`

  Check if this is a base address selection entry.

  

  A base address selection entry changes the base address that subsequent

  range entries are relative to.

- <span id="rawrange-parse"></span>`fn parse<R: Reader>(input: &mut R, address_size: u8) -> Result<RawRange>` — [`Result`](../../index.md#result), [`RawRange`](#rawrange)

  Parse an address range entry from `.debug_ranges` or `.debug_loc`.

#### Trait Implementations

##### `impl Clone for RawRange`

- <span id="rawrange-clone"></span>`fn clone(&self) -> RawRange` — [`RawRange`](#rawrange)

##### `impl Copy for RawRange`

##### `impl Debug for RawRange`

- <span id="rawrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RawRange`

##### `impl Hash for RawRange`

- <span id="rawrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RawRange`

- <span id="rawrange-partialeq-eq"></span>`fn eq(&self, other: &RawRange) -> bool` — [`RawRange`](#rawrange)

##### `impl StructuralPartialEq for RawRange`

### `Range`

```rust
struct Range {
    pub begin: u64,
    pub end: u64,
}
```

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="range-add-base-address"></span>`fn add_base_address(&mut self, base_address: u64, address_size: u8)`

  Add a base address to this range.

#### Trait Implementations

##### `impl Clone for Range`

- <span id="range-clone"></span>`fn clone(&self) -> Range` — [`Range`](../index.md#range)

##### `impl Copy for Range`

##### `impl Debug for Range`

- <span id="range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Range`

##### `impl Hash for Range`

- <span id="range-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Range`

- <span id="range-ord-cmp"></span>`fn cmp(&self, other: &Range) -> cmp::Ordering` — [`Range`](../index.md#range)

##### `impl PartialEq for Range`

- <span id="range-partialeq-eq"></span>`fn eq(&self, other: &Range) -> bool` — [`Range`](../index.md#range)

##### `impl PartialOrd for Range`

- <span id="range-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Range) -> option::Option<cmp::Ordering>` — [`Range`](../index.md#range)

##### `impl StructuralPartialEq for Range`

## Enums

### `RangeListsFormat`

```rust
enum RangeListsFormat {
    Bare,
    Rle,
}
```

#### Variants

- **`Bare`**

  The bare range list format used before DWARF 5.

- **`Rle`**

  The DW_RLE encoded range list format used in DWARF 5.

#### Trait Implementations

##### `impl Clone for RangeListsFormat`

- <span id="rangelistsformat-clone"></span>`fn clone(&self) -> RangeListsFormat` — [`RangeListsFormat`](#rangelistsformat)

##### `impl Copy for RangeListsFormat`

##### `impl Debug for RangeListsFormat`

- <span id="rangelistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RangeListsFormat`

##### `impl PartialEq for RangeListsFormat`

- <span id="rangelistsformat-partialeq-eq"></span>`fn eq(&self, other: &RangeListsFormat) -> bool` — [`RangeListsFormat`](#rangelistsformat)

##### `impl StructuralPartialEq for RangeListsFormat`

### `RawRngListEntry<T>`

```rust
enum RawRngListEntry<T> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<T>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<T>,
        end: crate::common::DebugAddrIndex<T>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<T>,
        length: u64,
    },
    OffsetPair {
        begin: u64,
        end: u64,
    },
    StartEnd {
        begin: u64,
        end: u64,
    },
    StartLength {
        begin: u64,
        length: u64,
    },
}
```

A raw entry in .debug_rnglists

#### Variants

- **`AddressOrOffsetPair`**

  A range from DWARF version <= 4.

- **`BaseAddress`**

  DW_RLE_base_address

- **`BaseAddressx`**

  DW_RLE_base_addressx

- **`StartxEndx`**

  DW_RLE_startx_endx

- **`StartxLength`**

  DW_RLE_startx_length

- **`OffsetPair`**

  DW_RLE_offset_pair

- **`StartEnd`**

  DW_RLE_start_end

- **`StartLength`**

  DW_RLE_start_length

#### Implementations

- <span id="rawrnglistentry-parse"></span>`fn parse<R: Reader<Offset = T>>(input: &mut R, encoding: Encoding, format: RangeListsFormat) -> Result<Option<Self>>` — [`Encoding`](../../index.md#encoding), [`RangeListsFormat`](#rangelistsformat), [`Result`](../../index.md#result)

  Parse a range entry from `.debug_rnglists`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RawRngListEntry<T>`

- <span id="rawrnglistentry-clone"></span>`fn clone(&self) -> RawRngListEntry<T>` — [`RawRngListEntry`](../index.md#rawrnglistentry)

##### `impl<T: fmt::Debug> Debug for RawRngListEntry<T>`

- <span id="rawrnglistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `RngListsHeader`

```rust
type RngListsHeader = crate::read::lists::ListsHeader;
```

