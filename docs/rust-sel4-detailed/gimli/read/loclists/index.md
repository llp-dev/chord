*[gimli](../../index.md) / [read](../index.md) / [loclists](index.md)*

---

# Module `loclists`

## Contents

- [Structs](#structs)
  - [`DebugLoc`](#debugloc)
  - [`DebugLocLists`](#debugloclists)
  - [`LocationLists`](#locationlists)
  - [`RawLocListIter`](#rawloclistiter)
  - [`LocListIter`](#loclistiter)
  - [`LocationListEntry`](#locationlistentry)
- [Enums](#enums)
  - [`LocListsFormat`](#loclistsformat)
  - [`RawLocListEntry`](#rawloclistentry)
- [Functions](#functions)
  - [`parse_data`](#parse-data)
- [Type Aliases](#type-aliases)
  - [`LocListsHeader`](#loclistsheader)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugLoc`](#debugloc) | struct | The raw contents of the `.debug_loc` section. |
| [`DebugLocLists`](#debugloclists) | struct | The `DebugLocLists` struct represents the DWARF data found in the `.debug_loclists` section. |
| [`LocationLists`](#locationlists) | struct | The DWARF data found in `.debug_loc` and `.debug_loclists` sections. |
| [`RawLocListIter`](#rawloclistiter) | struct | A raw iterator over a location list. |
| [`LocListIter`](#loclistiter) | struct | An iterator over a location list. |
| [`LocationListEntry`](#locationlistentry) | struct | A location list entry from the `.debug_loc` or `.debug_loclists` sections. |
| [`LocListsFormat`](#loclistsformat) | enum |  |
| [`RawLocListEntry`](#rawloclistentry) | enum | A raw entry in .debug_loclists. |
| [`parse_data`](#parse-data) | fn |  |
| [`LocListsHeader`](#loclistsheader) | type |  |

## Structs

### `DebugLoc<R>`

```rust
struct DebugLoc<R> {
    section: R,
}
```

The raw contents of the `.debug_loc` section.

#### Implementations

- <span id="debugloc-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLoc` instance from the data in the `.debug_loc`

  section.

  

  It is the caller's responsibility to read the `.debug_loc` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLoc, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loc_section_somehow = || &buf;

  let debug_loc = DebugLoc::new(read_debug_loc_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLoc<R>`

- <span id="debugloc-clone"></span>`fn clone(&self) -> DebugLoc<R>` — [`DebugLoc`](../index.md#debugloc)

##### `impl<R: marker::Copy> Copy for DebugLoc<R>`

##### `impl<R: fmt::Debug> Debug for DebugLoc<R>`

- <span id="debugloc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLoc<R>`

- <span id="debugloc-default"></span>`fn default() -> DebugLoc<R>` — [`DebugLoc`](../index.md#debugloc)

##### `impl<R> Section for DebugLoc<R>`

- <span id="debugloc-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugloc-section-reader"></span>`fn reader(&self) -> &R`

### `DebugLocLists<R>`

```rust
struct DebugLocLists<R> {
    section: R,
}
```

The `DebugLocLists` struct represents the DWARF data
found in the `.debug_loclists` section.

#### Implementations

- <span id="debugloclists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLocLists` instance from the data in the `.debug_loclists`

  section.

  

  It is the caller's responsibility to read the `.debug_loclists` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLocLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loclists_section_somehow = || &buf;

  let debug_loclists = DebugLocLists::new(read_debug_loclists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLocLists<R>`

- <span id="debugloclists-clone"></span>`fn clone(&self) -> DebugLocLists<R>` — [`DebugLocLists`](../index.md#debugloclists)

##### `impl<R: marker::Copy> Copy for DebugLocLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugLocLists<R>`

- <span id="debugloclists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLocLists<R>`

- <span id="debugloclists-default"></span>`fn default() -> DebugLocLists<R>` — [`DebugLocLists`](../index.md#debugloclists)

##### `impl<R> Section for DebugLocLists<R>`

- <span id="debugloclists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugloclists-section-reader"></span>`fn reader(&self) -> &R`

### `LocationLists<R>`

```rust
struct LocationLists<R> {
    debug_loc: DebugLoc<R>,
    debug_loclists: DebugLocLists<R>,
}
```

The DWARF data found in `.debug_loc` and `.debug_loclists` sections.

#### Implementations

- <span id="locationlists-new"></span>`fn new(debug_loc: DebugLoc<R>, debug_loclists: DebugLocLists<R>) -> LocationLists<R>` — [`DebugLoc`](../index.md#debugloc), [`DebugLocLists`](../index.md#debugloclists), [`LocationLists`](../index.md#locationlists)

  Construct a new `LocationLists` instance from the data in the `.debug_loc` and

  `.debug_loclists` sections.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for LocationLists<R>`

- <span id="locationlists-clone"></span>`fn clone(&self) -> LocationLists<R>` — [`LocationLists`](../index.md#locationlists)

##### `impl<R: marker::Copy> Copy for LocationLists<R>`

##### `impl<R: fmt::Debug> Debug for LocationLists<R>`

- <span id="locationlists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for LocationLists<R>`

- <span id="locationlists-default"></span>`fn default() -> LocationLists<R>` — [`LocationLists`](../index.md#locationlists)

### `RawLocListIter<R: Reader>`

```rust
struct RawLocListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: LocListsFormat,
}
```

A raw iterator over a location list.

This iterator does not perform any processing of the location entries,
such as handling base addresses.

#### Implementations

- <span id="rawloclistiter-new"></span>`fn new(input: R, encoding: Encoding, format: LocListsFormat) -> RawLocListIter<R>` — [`Encoding`](../../index.md#encoding), [`LocListsFormat`](#loclistsformat), [`RawLocListIter`](../index.md#rawloclistiter)

  Construct a `RawLocListIter`.

- <span id="rawloclistiter-next"></span>`fn next(&mut self) -> Result<Option<RawLocListEntry<R>>>` — [`Result`](../../index.md#result), [`RawLocListEntry`](../index.md#rawloclistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListIter<R>`

- <span id="rawloclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RawLocListIter<R>`

- <span id="rawloclistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawloclistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawloclistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RawLocListIter<R>`

- <span id="rawloclistiter-iterator-type-item"></span>`type Item = Result<RawLocListEntry<R>, Error>`

- <span id="rawloclistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LocListIter<R: Reader>`

```rust
struct LocListIter<R: Reader> {
    raw: RawLocListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over a location list.

This iterator internally handles processing of base address selection entries
and list end entries.  Thus, it only returns location entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="loclistiter-new"></span>`fn new(raw: RawLocListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> LocListIter<R>` — [`RawLocListIter`](../index.md#rawloclistiter), [`DebugAddr`](../index.md#debugaddr), [`DebugAddrBase`](../../index.md#debugaddrbase), [`Reader`](../index.md#reader), [`LocListIter`](../index.md#loclistiter)

  Construct a `LocListIter`.

- <span id="loclistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="loclistiter-next"></span>`fn next(&mut self) -> Result<Option<LocationListEntry<R>>>` — [`Result`](../../index.md#result), [`LocationListEntry`](../index.md#locationlistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for LocListIter<R>`

- <span id="loclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for LocListIter<R>`

- <span id="loclistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="loclistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="loclistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for LocListIter<R>`

- <span id="loclistiter-iterator-type-item"></span>`type Item = Result<LocationListEntry<R>, Error>`

- <span id="loclistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LocationListEntry<R: Reader>`

```rust
struct LocationListEntry<R: Reader> {
    pub range: crate::read::Range,
    pub data: crate::read::Expression<R>,
}
```

A location list entry from the `.debug_loc` or `.debug_loclists` sections.

#### Fields

- **`range`**: `crate::read::Range`

  The address range that this location is valid for.

- **`data`**: `crate::read::Expression<R>`

  The data containing a single location description.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for LocationListEntry<R>`

- <span id="locationlistentry-clone"></span>`fn clone(&self) -> LocationListEntry<R>` — [`LocationListEntry`](../index.md#locationlistentry)

##### `impl<R: marker::Copy + Reader> Copy for LocationListEntry<R>`

##### `impl<R: fmt::Debug + Reader> Debug for LocationListEntry<R>`

- <span id="locationlistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for LocationListEntry<R>`

##### `impl<R: hash::Hash + Reader> Hash for LocationListEntry<R>`

- <span id="locationlistentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for LocationListEntry<R>`

- <span id="locationlistentry-partialeq-eq"></span>`fn eq(&self, other: &LocationListEntry<R>) -> bool` — [`LocationListEntry`](../index.md#locationlistentry)

##### `impl<R: Reader> StructuralPartialEq for LocationListEntry<R>`

## Enums

### `LocListsFormat`

```rust
enum LocListsFormat {
    Bare,
    Lle,
}
```

#### Variants

- **`Bare`**

  The bare location list format used before DWARF 5.

- **`Lle`**

  The DW_LLE encoded range list format used in DWARF 5 and the non-standard GNU
  split dwarf extension.

#### Trait Implementations

##### `impl Clone for LocListsFormat`

- <span id="loclistsformat-clone"></span>`fn clone(&self) -> LocListsFormat` — [`LocListsFormat`](#loclistsformat)

##### `impl Copy for LocListsFormat`

##### `impl Debug for LocListsFormat`

- <span id="loclistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocListsFormat`

##### `impl PartialEq for LocListsFormat`

- <span id="loclistsformat-partialeq-eq"></span>`fn eq(&self, other: &LocListsFormat) -> bool` — [`LocListsFormat`](#loclistsformat)

##### `impl StructuralPartialEq for LocListsFormat`

### `RawLocListEntry<R: Reader>`

```rust
enum RawLocListEntry<R: Reader> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<<R as >::Offset>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        end: crate::common::DebugAddrIndex<<R as >::Offset>,
        data: crate::read::Expression<R>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        length: u64,
        data: crate::read::Expression<R>,
    },
    OffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    DefaultLocation {
        data: crate::read::Expression<R>,
    },
    StartEnd {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    StartLength {
        begin: u64,
        length: u64,
        data: crate::read::Expression<R>,
    },
}
```

A raw entry in .debug_loclists.

#### Variants

- **`AddressOrOffsetPair`**

  A location from DWARF version <= 4.

- **`BaseAddress`**

  DW_LLE_base_address

- **`BaseAddressx`**

  DW_LLE_base_addressx

- **`StartxEndx`**

  DW_LLE_startx_endx

- **`StartxLength`**

  DW_LLE_startx_length

- **`OffsetPair`**

  DW_LLE_offset_pair

- **`DefaultLocation`**

  DW_LLE_default_location

- **`StartEnd`**

  DW_LLE_start_end

- **`StartLength`**

  DW_LLE_start_length

#### Implementations

- <span id="rawloclistentry-parse"></span>`fn parse(input: &mut R, encoding: Encoding, format: LocListsFormat) -> Result<Option<Self>>` — [`Encoding`](../../index.md#encoding), [`LocListsFormat`](#loclistsformat), [`Result`](../../index.md#result)

  Parse a location list entry from `.debug_loclists`

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for RawLocListEntry<R>`

- <span id="rawloclistentry-clone"></span>`fn clone(&self) -> RawLocListEntry<R>` — [`RawLocListEntry`](../index.md#rawloclistentry)

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListEntry<R>`

- <span id="rawloclistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `parse_data`

```rust
fn parse_data<R: Reader>(input: &mut R, encoding: crate::common::Encoding) -> crate::read::Result<crate::read::Expression<R>>
```

## Type Aliases

### `LocListsHeader`

```rust
type LocListsHeader = crate::read::lists::ListsHeader;
```

