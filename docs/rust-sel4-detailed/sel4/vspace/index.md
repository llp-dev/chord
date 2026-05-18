*[sel4](../index.md) / [vspace](index.md)*

---

# Module `vspace`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`vspace_levels`](#vspace-levels) | mod | Items describing the layout of address translation structures for this kernel configuration. |
| [`CapTypeForFrameObject`](#captypeforframeobject) | trait | Trait for [`CapType`]s which correspond to frame objects. |
| [`CapTypeForFrameObjectOfFixedSize`](#captypeforframeobjectoffixedsize) | trait | Trait for [`CapType`]s which correspond to frame objects of fixed size. |
| [`CapTypeForTranslationTableObject`](#captypefortranslationtableobject) | trait | Trait for [`CapType`]s which correspond to translation table objects. |

## Modules

- [`vspace_levels`](vspace_levels/index.md) — Items describing the layout of address translation structures for this kernel configuration.

## Traits

### `CapTypeForFrameObject`

```rust
trait CapTypeForFrameObject: CapType { ... }
```

Trait for [`CapType`](../cptr/index.md)s which correspond to frame objects.

#### Implementors

- [`HugePage`](../cptr/cap_type/index.md#hugepage)
- [`LargePage`](../cptr/cap_type/index.md#largepage)
- [`UnspecifiedPage`](../cptr/cap_type/index.md#unspecifiedpage)
- [`_4k`](../cptr/cap_type/index.md#4k)

### `CapTypeForFrameObjectOfFixedSize`

```rust
trait CapTypeForFrameObjectOfFixedSize: CapTypeForObjectOfFixedSize + CapTypeForFrameObject { ... }
```

Trait for [`CapType`](../cptr/index.md)s which correspond to frame objects of fixed size.

#### Associated Constants

- `const FRAME_OBJECT_TYPE: FrameObjectType`

#### Implementors

- [`HugePage`](../cptr/cap_type/index.md#hugepage)
- [`LargePage`](../cptr/cap_type/index.md#largepage)
- [`_4k`](../cptr/cap_type/index.md#4k)

### `CapTypeForTranslationTableObject`

```rust
trait CapTypeForTranslationTableObject: CapTypeForObjectOfFixedSize { ... }
```

Trait for [`CapType`](../cptr/index.md)s which correspond to translation table objects.

#### Associated Constants

- `const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

#### Implementors

- [`PDPT`](../cptr/cap_type/index.md#pdpt)
- [`PML4`](../cptr/cap_type/index.md#pml4)
- [`PageDirectory`](../cptr/cap_type/index.md#pagedirectory)
- [`PageTable`](../cptr/cap_type/index.md#pagetable)

