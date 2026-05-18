*[object](../../index.md) / [write](../index.md) / [macho](index.md)*

---

# Module `macho`

## Contents

- [Structs](#structs)
  - [`SectionOffsets`](#sectionoffsets)
  - [`SymbolOffsets`](#symboloffsets)
  - [`MachOBuildVersion`](#machobuildversion)
  - [`MachHeader`](#machheader)
  - [`SegmentCommand`](#segmentcommand)
  - [`SectionHeader`](#sectionheader)
  - [`Nlist`](#nlist)
  - [`MachO32`](#macho32)
  - [`MachO64`](#macho64)
- [Traits](#traits)
  - [`MachO`](#macho)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SectionOffsets`](#sectionoffsets) | struct |  |
| [`SymbolOffsets`](#symboloffsets) | struct |  |
| [`MachOBuildVersion`](#machobuildversion) | struct | The customizable portion of a [`macho::BuildVersionCommand`]. |
| [`MachHeader`](#machheader) | struct |  |
| [`SegmentCommand`](#segmentcommand) | struct |  |
| [`SectionHeader`](#sectionheader) | struct |  |
| [`Nlist`](#nlist) | struct |  |
| [`MachO32`](#macho32) | struct |  |
| [`MachO64`](#macho64) | struct |  |
| [`MachO`](#macho) | trait |  |

## Structs

### `SectionOffsets`

```rust
struct SectionOffsets {
    index: usize,
    offset: usize,
    address: u64,
    reloc_offset: usize,
    reloc_count: usize,
}
```

#### Trait Implementations

##### `impl Clone for SectionOffsets`

- <span id="sectionoffsets-clone"></span>`fn clone(&self) -> SectionOffsets` — [`SectionOffsets`](#sectionoffsets)

##### `impl Copy for SectionOffsets`

##### `impl Default for SectionOffsets`

- <span id="sectionoffsets-default"></span>`fn default() -> SectionOffsets` — [`SectionOffsets`](#sectionoffsets)

### `SymbolOffsets`

```rust
struct SymbolOffsets {
    index: usize,
    str_id: Option<StringId>,
}
```

#### Trait Implementations

##### `impl Clone for SymbolOffsets`

- <span id="symboloffsets-clone"></span>`fn clone(&self) -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

##### `impl Copy for SymbolOffsets`

##### `impl Default for SymbolOffsets`

- <span id="symboloffsets-default"></span>`fn default() -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

### `MachOBuildVersion`

```rust
struct MachOBuildVersion {
    pub platform: u32,
    pub minos: u32,
    pub sdk: u32,
}
```

The customizable portion of a [`macho::BuildVersionCommand`](../../macho/index.md).

#### Fields

- **`platform`**: `u32`

  One of the `PLATFORM_` constants (for example,
  [`object::macho::PLATFORM_MACOS`](macho::PLATFORM_MACOS)).

- **`minos`**: `u32`

  The minimum OS version, where `X.Y.Z` is encoded in nibbles as
  `xxxx.yy.zz`.

- **`sdk`**: `u32`

  The SDK version as `X.Y.Z`, where `X.Y.Z` is encoded in nibbles as
  `xxxx.yy.zz`.

#### Implementations

- <span id="machobuildversion-cmdsize"></span>`fn cmdsize(&self) -> u32`

#### Trait Implementations

##### `impl Clone for MachOBuildVersion`

- <span id="machobuildversion-clone"></span>`fn clone(&self) -> MachOBuildVersion` — [`MachOBuildVersion`](#machobuildversion)

##### `impl Copy for MachOBuildVersion`

##### `impl Debug for MachOBuildVersion`

- <span id="machobuildversion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MachOBuildVersion`

- <span id="machobuildversion-default"></span>`fn default() -> MachOBuildVersion` — [`MachOBuildVersion`](#machobuildversion)

### `MachHeader`

```rust
struct MachHeader {
    cputype: u32,
    cpusubtype: u32,
    filetype: u32,
    ncmds: u32,
    sizeofcmds: u32,
    flags: u32,
}
```

### `SegmentCommand`

```rust
struct SegmentCommand {
    cmdsize: u32,
    segname: [u8; 16],
    vmaddr: u64,
    vmsize: u64,
    fileoff: u64,
    filesize: u64,
    maxprot: u32,
    initprot: u32,
    nsects: u32,
    flags: u32,
}
```

### `SectionHeader`

```rust
struct SectionHeader {
    sectname: [u8; 16],
    segname: [u8; 16],
    addr: u64,
    size: u64,
    offset: u32,
    align: u32,
    reloff: u32,
    nreloc: u32,
    flags: u32,
}
```

### `Nlist`

```rust
struct Nlist {
    n_strx: u32,
    n_type: u8,
    n_sect: u8,
    n_desc: u16,
    n_value: u64,
}
```

### `MachO32<E>`

```rust
struct MachO32<E> {
    endian: E,
}
```

#### Trait Implementations

##### `impl<E: Endian> MachO for MachO32<E>`

- <span id="macho32-macho-mach-header-size"></span>`fn mach_header_size(&self) -> usize`

- <span id="macho32-macho-segment-command-size"></span>`fn segment_command_size(&self) -> usize`

- <span id="macho32-macho-section-header-size"></span>`fn section_header_size(&self) -> usize`

- <span id="macho32-macho-nlist-size"></span>`fn nlist_size(&self) -> usize`

- <span id="macho32-macho-write-mach-header"></span>`fn write_mach_header(&self, buffer: &mut dyn WritableBuffer, header: MachHeader)` — [`WritableBuffer`](../index.md#writablebuffer), [`MachHeader`](#machheader)

- <span id="macho32-macho-write-segment-command"></span>`fn write_segment_command(&self, buffer: &mut dyn WritableBuffer, segment: SegmentCommand)` — [`WritableBuffer`](../index.md#writablebuffer), [`SegmentCommand`](#segmentcommand)

- <span id="macho32-macho-write-section"></span>`fn write_section(&self, buffer: &mut dyn WritableBuffer, section: SectionHeader)` — [`WritableBuffer`](../index.md#writablebuffer), [`SectionHeader`](#sectionheader)

- <span id="macho32-macho-write-nlist"></span>`fn write_nlist(&self, buffer: &mut dyn WritableBuffer, nlist: Nlist)` — [`WritableBuffer`](../index.md#writablebuffer), [`Nlist`](#nlist)

### `MachO64<E>`

```rust
struct MachO64<E> {
    endian: E,
}
```

#### Trait Implementations

##### `impl<E: Endian> MachO for MachO64<E>`

- <span id="macho64-macho-mach-header-size"></span>`fn mach_header_size(&self) -> usize`

- <span id="macho64-macho-segment-command-size"></span>`fn segment_command_size(&self) -> usize`

- <span id="macho64-macho-section-header-size"></span>`fn section_header_size(&self) -> usize`

- <span id="macho64-macho-nlist-size"></span>`fn nlist_size(&self) -> usize`

- <span id="macho64-macho-write-mach-header"></span>`fn write_mach_header(&self, buffer: &mut dyn WritableBuffer, header: MachHeader)` — [`WritableBuffer`](../index.md#writablebuffer), [`MachHeader`](#machheader)

- <span id="macho64-macho-write-segment-command"></span>`fn write_segment_command(&self, buffer: &mut dyn WritableBuffer, segment: SegmentCommand)` — [`WritableBuffer`](../index.md#writablebuffer), [`SegmentCommand`](#segmentcommand)

- <span id="macho64-macho-write-section"></span>`fn write_section(&self, buffer: &mut dyn WritableBuffer, section: SectionHeader)` — [`WritableBuffer`](../index.md#writablebuffer), [`SectionHeader`](#sectionheader)

- <span id="macho64-macho-write-nlist"></span>`fn write_nlist(&self, buffer: &mut dyn WritableBuffer, nlist: Nlist)` — [`WritableBuffer`](../index.md#writablebuffer), [`Nlist`](#nlist)

## Traits

### `MachO`

```rust
trait MachO { ... }
```

#### Required Methods

- `fn mach_header_size(&self) -> usize`

- `fn segment_command_size(&self) -> usize`

- `fn section_header_size(&self) -> usize`

- `fn nlist_size(&self) -> usize`

- `fn write_mach_header(&self, buffer: &mut dyn WritableBuffer, section: MachHeader)`

- `fn write_segment_command(&self, buffer: &mut dyn WritableBuffer, segment: SegmentCommand)`

- `fn write_section(&self, buffer: &mut dyn WritableBuffer, section: SectionHeader)`

- `fn write_nlist(&self, buffer: &mut dyn WritableBuffer, nlist: Nlist)`

#### Implementors

- [`MachO32`](#macho32)
- [`MachO64`](#macho64)

