*[object](../../../index.md) / [write](../../index.md) / [elf](../index.md) / [object](index.md)*

---

# Module `object`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ComdatOffsets`](#comdatoffsets) | struct |  |
| [`SectionOffsets`](#sectionoffsets) | struct |  |
| [`SymbolOffsets`](#symboloffsets) | struct |  |

## Structs

### `ComdatOffsets`

```rust
struct ComdatOffsets {
    offset: usize,
    str_id: crate::write::string::StringId,
}
```

#### Trait Implementations

##### `impl Clone for ComdatOffsets`

- <span id="comdatoffsets-clone"></span>`fn clone(&self) -> ComdatOffsets` — [`ComdatOffsets`](#comdatoffsets)

##### `impl Copy for ComdatOffsets`

### `SectionOffsets`

```rust
struct SectionOffsets {
    index: SectionIndex,
    offset: usize,
    str_id: crate::write::string::StringId,
    reloc_offset: usize,
    reloc_str_id: Option<crate::write::string::StringId>,
}
```

#### Trait Implementations

##### `impl Clone for SectionOffsets`

- <span id="sectionoffsets-clone"></span>`fn clone(&self) -> SectionOffsets` — [`SectionOffsets`](#sectionoffsets)

##### `impl Copy for SectionOffsets`

### `SymbolOffsets`

```rust
struct SymbolOffsets {
    index: SymbolIndex,
    str_id: Option<crate::write::string::StringId>,
}
```

#### Trait Implementations

##### `impl Clone for SymbolOffsets`

- <span id="symboloffsets-clone"></span>`fn clone(&self) -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

##### `impl Copy for SymbolOffsets`

##### `impl Default for SymbolOffsets`

- <span id="symboloffsets-default"></span>`fn default() -> SymbolOffsets` — [`SymbolOffsets`](#symboloffsets)

