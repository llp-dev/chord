*[sel4](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [vspace](index.md)*

---

# Module `vspace`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`vspace_levels`](#vspace-levels) | mod |  |
| [`FrameObjectType`](#frameobjecttype) | enum | Frame object types for this kernel configuration. |
| [`TranslationTableObjectType`](#translationtableobjecttype) | enum | Translation table object types for this kernel configuration. |

## Modules

- [`vspace_levels`](vspace_levels/index.md)

## Enums

### `FrameObjectType`

```rust
enum FrameObjectType {
    _4k,
    LargePage,
    HugePage,
}
```

Frame object types for this kernel configuration.

#### Implementations

- <span id="frameobjecttype-const-granule"></span>`const GRANULE: Self`

- <span id="frameobjecttype-blueprint"></span>`const fn blueprint(self) -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

- <span id="frameobjecttype-from-bits"></span>`const fn from_bits(bits: usize) -> Option<Self>`

- <span id="frameobjecttype-const-4k-bits"></span>`const _4K_BITS: usize`

- <span id="frameobjecttype-const-large-page-bits"></span>`const LARGE_PAGE_BITS: usize`

- <span id="frameobjecttype-const-huge-page-bits"></span>`const HUGE_PAGE_BITS: usize`

#### Trait Implementations

##### `impl Clone for FrameObjectType`

- <span id="frameobjecttype-clone"></span>`fn clone(&self) -> FrameObjectType` — [`FrameObjectType`](#frameobjecttype)

##### `impl Copy for FrameObjectType`

##### `impl Debug for FrameObjectType`

- <span id="frameobjecttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FrameObjectType`

##### `impl PartialEq for FrameObjectType`

- <span id="frameobjecttype-partialeq-eq"></span>`fn eq(&self, other: &FrameObjectType) -> bool` — [`FrameObjectType`](#frameobjecttype)

##### `impl StructuralPartialEq for FrameObjectType`

### `TranslationTableObjectType`

```rust
enum TranslationTableObjectType {
    PML4,
    PDPT,
    PageDirectory,
    PageTable,
}
```

Translation table object types for this kernel configuration.

#### Implementations

- <span id="translationtableobjecttype-blueprint"></span>`const fn blueprint(&self) -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

- <span id="translationtableobjecttype-index-bits"></span>`const fn index_bits(&self) -> usize`

- <span id="translationtableobjecttype-from-level"></span>`const fn from_level(level: usize) -> Option<Self>`

#### Trait Implementations

##### `impl Clone for TranslationTableObjectType`

- <span id="translationtableobjecttype-clone"></span>`fn clone(&self) -> TranslationTableObjectType` — [`TranslationTableObjectType`](#translationtableobjecttype)

##### `impl Copy for TranslationTableObjectType`

##### `impl Debug for TranslationTableObjectType`

- <span id="translationtableobjecttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TranslationTableObjectType`

##### `impl PartialEq for TranslationTableObjectType`

- <span id="translationtableobjecttype-partialeq-eq"></span>`fn eq(&self, other: &TranslationTableObjectType) -> bool` — [`TranslationTableObjectType`](#translationtableobjecttype)

##### `impl StructuralPartialEq for TranslationTableObjectType`

