*[gimli](../../index.md) / [read](../index.md) / [str](index.md)*

---

# Module `str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugStr`](#debugstr) | struct | The `DebugStr` struct represents the DWARF strings found in the `.debug_str` section. |
| [`DebugStrOffsets`](#debugstroffsets) | struct | The raw contents of the `.debug_str_offsets` section. |
| [`DebugLineStr`](#debuglinestr) | struct | The `DebugLineStr` struct represents the DWARF strings found in the `.debug_line_str` section. |

## Structs

### `DebugStr<R>`

```rust
struct DebugStr<R> {
    debug_str_section: R,
}
```

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

#### Implementations

- <span id="debugstr-new"></span>`fn new(debug_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugStr` instance from the data in the `.debug_str`

  section.

  

  It is the caller's responsibility to read the `.debug_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_str_section_somehow = || &buf;

  let debug_str = DebugStr::new(read_debug_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStr<R>`

- <span id="debugstr-clone"></span>`fn clone(&self) -> DebugStr<R>` — [`DebugStr`](../index.md#debugstr)

##### `impl<R: marker::Copy> Copy for DebugStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugStr<R>`

- <span id="debugstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStr<R>`

- <span id="debugstr-default"></span>`fn default() -> DebugStr<R>` — [`DebugStr`](../index.md#debugstr)

##### `impl<R> Section for DebugStr<R>`

- <span id="debugstr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugstr-section-reader"></span>`fn reader(&self) -> &R`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- <span id="debugstroffsets-get-str-offset"></span>`fn get_str_offset(&self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../../index.md#format), [`DebugStrOffsetsBase`](../../index.md#debugstroffsetsbase), [`Reader`](../index.md#reader), [`DebugStrOffsetsIndex`](../../index.md#debugstroffsetsindex), [`Result`](../../index.md#result), [`DebugStrOffset`](../../index.md#debugstroffset)

  Returns the `.debug_str` offset at the given `base` and `index`.

  

  A set of entries in the `.debug_str_offsets` section consists of a header

  followed by a series of string table offsets.

  

  The `base` must be the `DW_AT_str_offsets_base` value from the compilation unit DIE.

  This is an offset that points to the first entry following the header.

  

  The `index` is the value of a `DW_FORM_strx` attribute.

  

  The `format` must be the DWARF format of the compilation unit. This format must

  match the header. However, note that we do not parse the header to validate this,

  since locating the header is unreliable, and the GNU extensions do not emit it.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStrOffsets<R>`

- <span id="debugstroffsets-clone"></span>`fn clone(&self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md#debugstroffsets)

##### `impl<R: marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: fmt::Debug> Debug for DebugStrOffsets<R>`

- <span id="debugstroffsets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStrOffsets<R>`

- <span id="debugstroffsets-default"></span>`fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md#debugstroffsets)

##### `impl<R> Section for DebugStrOffsets<R>`

- <span id="debugstroffsets-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugstroffsets-section-reader"></span>`fn reader(&self) -> &R`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- <span id="debuglinestr-new"></span>`fn new(debug_line_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLineStr` instance from the data in the `.debug_line_str`

  section.

  

  It is the caller's responsibility to read the `.debug_line_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLineStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_line_str_section_somehow = || &buf;

  let debug_str = DebugLineStr::new(read_debug_line_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLineStr<R>`

- <span id="debuglinestr-clone"></span>`fn clone(&self) -> DebugLineStr<R>` — [`DebugLineStr`](../index.md#debuglinestr)

##### `impl<R: marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugLineStr<R>`

- <span id="debuglinestr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLineStr<R>`

- <span id="debuglinestr-default"></span>`fn default() -> DebugLineStr<R>` — [`DebugLineStr`](../index.md#debuglinestr)

##### `impl<R> Section for DebugLineStr<R>`

- <span id="debuglinestr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debuglinestr-section-reader"></span>`fn reader(&self) -> &R`

