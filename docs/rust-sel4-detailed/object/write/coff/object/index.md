*[object](../../../index.md) / [write](../../index.md) / [coff](../index.md) / [object](index.md)*

---

# Module `object`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SectionOffsets`](#sectionoffsets) | struct |  |
| [`SymbolOffsets`](#symboloffsets) | struct |  |
| [`CoffExportStyle`](#coffexportstyle) | enum | Internal format to use for the `.drectve` section containing linker directives for symbol exports. |
| [`checksum`](#checksum) | fn |  |

## Structs

### `SectionOffsets`

```rust
struct SectionOffsets {
    name: writer::Name,
    offset: u32,
    reloc_offset: u32,
    selection: u8,
    associative_section: u32,
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
    name: writer::Name,
    index: u32,
    aux_count: u8,
}
```

#### Trait Implementations

##### `impl Clone for SymbolOffsets`

- <span id="symboloffsets-clone"></span>`fn clone(&self) -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

##### `impl Copy for SymbolOffsets`

##### `impl Default for SymbolOffsets`

- <span id="symboloffsets-default"></span>`fn default() -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

## Enums

### `CoffExportStyle`

```rust
enum CoffExportStyle {
    Msvc,
    Gnu,
}
```

Internal format to use for the `.drectve` section containing linker
directives for symbol exports.

#### Variants

- **`Msvc`**

  MSVC format supported by link.exe and LLD.

- **`Gnu`**

  Gnu format supported by GNU LD and LLD.

#### Trait Implementations

##### `impl Clone for CoffExportStyle`

- <span id="coffexportstyle-clone"></span>`fn clone(&self) -> CoffExportStyle` — [`CoffExportStyle`](../index.md#coffexportstyle)

##### `impl Copy for CoffExportStyle`

##### `impl Debug for CoffExportStyle`

- <span id="coffexportstyle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CoffExportStyle`

##### `impl<K> Equivalent for CoffExportStyle`

- <span id="coffexportstyle-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for CoffExportStyle`

- <span id="coffexportstyle-partialeq-eq"></span>`fn eq(&self, other: &CoffExportStyle) -> bool` — [`CoffExportStyle`](../index.md#coffexportstyle)

##### `impl StructuralPartialEq for CoffExportStyle`

## Functions

### `checksum`

```rust
fn checksum(data: &[u8]) -> u32
```

