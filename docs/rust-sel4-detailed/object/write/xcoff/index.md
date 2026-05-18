*[object](../../index.md) / [write](../index.md) / [xcoff](index.md)*

---

# Module `xcoff`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SectionOffsets`](#sectionoffsets) | struct |  |
| [`SymbolOffsets`](#symboloffsets) | struct |  |

## Structs

### `SectionOffsets`

```rust
struct SectionOffsets {
    address: u64,
    data_offset: usize,
    reloc_offset: usize,
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
    aux_count: u8,
    storage_class: u8,
    x_smtyp: u8,
    x_smclas: u8,
    containing_csect: Option<SymbolId>,
}
```

#### Trait Implementations

##### `impl Clone for SymbolOffsets`

- <span id="symboloffsets-clone"></span>`fn clone(&self) -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

##### `impl Copy for SymbolOffsets`

##### `impl Default for SymbolOffsets`

- <span id="symboloffsets-default"></span>`fn default() -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

