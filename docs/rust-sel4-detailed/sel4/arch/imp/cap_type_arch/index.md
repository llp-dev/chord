*[sel4](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [cap_type_arch](index.md)*

---

# Module `cap_type_arch`

## Contents

- [Structs](#structs)
  - [`_4k`](#4k)
  - [`LargePage`](#largepage)
  - [`HugePage`](#hugepage)
  - [`PML4`](#pml4)
  - [`PDPT`](#pdpt)
  - [`PageDirectory`](#pagedirectory)
  - [`PageTable`](#pagetable)
  - [`IOPortControl`](#ioportcontrol)
- [Type Aliases](#type-aliases)
  - [`VSpace`](#vspace)
  - [`Granule`](#granule)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`_4k`](#4k) | struct |  |
| [`LargePage`](#largepage) | struct |  |
| [`HugePage`](#hugepage) | struct |  |
| [`PML4`](#pml4) | struct |  |
| [`PDPT`](#pdpt) | struct |  |
| [`PageDirectory`](#pagedirectory) | struct |  |
| [`PageTable`](#pagetable) | struct |  |
| [`IOPortControl`](#ioportcontrol) | struct |  |
| [`VSpace`](#vspace) | type |  |
| [`Granule`](#granule) | type |  |

## Structs

### `_4k`

```rust
struct _4k;
```

#### Trait Implementations

##### `impl CapType for _4k`

- <span id="4k-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForFrameObject for cap_type::_4k`

##### `impl CapTypeForFrameObjectOfFixedSize for cap_type::_4k`

- <span id="cap-type-4k-captypeforframeobjectoffixedsize-const-frame-object-type"></span>`const FRAME_OBJECT_TYPE: FrameObjectType`

##### `impl CapTypeForObject for _4k`

- <span id="4k-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for _4k`

- <span id="4k-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

##### `impl Clone for _4k`

- <span id="4k-clone"></span>`fn clone(&self) -> _4k` — [`_4k`](../../../cptr/cap_type/index.md#4k)

##### `impl Copy for _4k`

##### `impl Debug for _4k`

- <span id="4k-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for _4k`

##### `impl Hash for _4k`

- <span id="4k-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for _4k`

- <span id="4k-ord-cmp"></span>`fn cmp(&self, other: &_4k) -> cmp::Ordering` — [`_4k`](../../../cptr/cap_type/index.md#4k)

##### `impl PartialEq for _4k`

- <span id="4k-partialeq-eq"></span>`fn eq(&self, other: &_4k) -> bool` — [`_4k`](../../../cptr/cap_type/index.md#4k)

##### `impl PartialOrd for _4k`

- <span id="4k-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &_4k) -> option::Option<cmp::Ordering>` — [`_4k`](../../../cptr/cap_type/index.md#4k)

##### `impl StructuralPartialEq for _4k`

### `LargePage`

```rust
struct LargePage;
```

#### Trait Implementations

##### `impl CapType for LargePage`

- <span id="largepage-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForFrameObject for cap_type::LargePage`

##### `impl CapTypeForFrameObjectOfFixedSize for cap_type::LargePage`

- <span id="cap-typelargepage-captypeforframeobjectoffixedsize-const-frame-object-type"></span>`const FRAME_OBJECT_TYPE: FrameObjectType`

##### `impl CapTypeForObject for LargePage`

- <span id="largepage-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for LargePage`

- <span id="largepage-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

##### `impl Clone for LargePage`

- <span id="largepage-clone"></span>`fn clone(&self) -> LargePage` — [`LargePage`](../../../cptr/cap_type/index.md#largepage)

##### `impl Copy for LargePage`

##### `impl Debug for LargePage`

- <span id="largepage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LargePage`

##### `impl Hash for LargePage`

- <span id="largepage-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LargePage`

- <span id="largepage-ord-cmp"></span>`fn cmp(&self, other: &LargePage) -> cmp::Ordering` — [`LargePage`](../../../cptr/cap_type/index.md#largepage)

##### `impl PartialEq for LargePage`

- <span id="largepage-partialeq-eq"></span>`fn eq(&self, other: &LargePage) -> bool` — [`LargePage`](../../../cptr/cap_type/index.md#largepage)

##### `impl PartialOrd for LargePage`

- <span id="largepage-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LargePage) -> option::Option<cmp::Ordering>` — [`LargePage`](../../../cptr/cap_type/index.md#largepage)

##### `impl StructuralPartialEq for LargePage`

### `HugePage`

```rust
struct HugePage;
```

#### Trait Implementations

##### `impl CapType for HugePage`

- <span id="hugepage-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForFrameObject for cap_type::HugePage`

##### `impl CapTypeForFrameObjectOfFixedSize for cap_type::HugePage`

- <span id="cap-typehugepage-captypeforframeobjectoffixedsize-const-frame-object-type"></span>`const FRAME_OBJECT_TYPE: FrameObjectType`

##### `impl CapTypeForObject for HugePage`

- <span id="hugepage-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for HugePage`

- <span id="hugepage-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

##### `impl Clone for HugePage`

- <span id="hugepage-clone"></span>`fn clone(&self) -> HugePage` — [`HugePage`](../../../cptr/cap_type/index.md#hugepage)

##### `impl Copy for HugePage`

##### `impl Debug for HugePage`

- <span id="hugepage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HugePage`

##### `impl Hash for HugePage`

- <span id="hugepage-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for HugePage`

- <span id="hugepage-ord-cmp"></span>`fn cmp(&self, other: &HugePage) -> cmp::Ordering` — [`HugePage`](../../../cptr/cap_type/index.md#hugepage)

##### `impl PartialEq for HugePage`

- <span id="hugepage-partialeq-eq"></span>`fn eq(&self, other: &HugePage) -> bool` — [`HugePage`](../../../cptr/cap_type/index.md#hugepage)

##### `impl PartialOrd for HugePage`

- <span id="hugepage-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &HugePage) -> option::Option<cmp::Ordering>` — [`HugePage`](../../../cptr/cap_type/index.md#hugepage)

##### `impl StructuralPartialEq for HugePage`

### `PML4`

```rust
struct PML4;
```

#### Trait Implementations

##### `impl CapType for PML4`

- <span id="pml4-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PML4`

- <span id="pml4-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PML4`

- <span id="pml4-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PML4`

- <span id="cap-typepml4-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PML4`

- <span id="pml4-clone"></span>`fn clone(&self) -> PML4` — [`PML4`](../../../cptr/cap_type/index.md#pml4)

##### `impl Copy for PML4`

##### `impl Debug for PML4`

- <span id="pml4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PML4`

##### `impl Hash for PML4`

- <span id="pml4-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PML4`

- <span id="pml4-ord-cmp"></span>`fn cmp(&self, other: &PML4) -> cmp::Ordering` — [`PML4`](../../../cptr/cap_type/index.md#pml4)

##### `impl PartialEq for PML4`

- <span id="pml4-partialeq-eq"></span>`fn eq(&self, other: &PML4) -> bool` — [`PML4`](../../../cptr/cap_type/index.md#pml4)

##### `impl PartialOrd for PML4`

- <span id="pml4-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PML4) -> option::Option<cmp::Ordering>` — [`PML4`](../../../cptr/cap_type/index.md#pml4)

##### `impl StructuralPartialEq for PML4`

### `PDPT`

```rust
struct PDPT;
```

#### Trait Implementations

##### `impl CapType for PDPT`

- <span id="pdpt-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PDPT`

- <span id="pdpt-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PDPT`

- <span id="pdpt-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PDPT`

- <span id="cap-typepdpt-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PDPT`

- <span id="pdpt-clone"></span>`fn clone(&self) -> PDPT` — [`PDPT`](../../../cptr/cap_type/index.md#pdpt)

##### `impl Copy for PDPT`

##### `impl Debug for PDPT`

- <span id="pdpt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PDPT`

##### `impl Hash for PDPT`

- <span id="pdpt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PDPT`

- <span id="pdpt-ord-cmp"></span>`fn cmp(&self, other: &PDPT) -> cmp::Ordering` — [`PDPT`](../../../cptr/cap_type/index.md#pdpt)

##### `impl PartialEq for PDPT`

- <span id="pdpt-partialeq-eq"></span>`fn eq(&self, other: &PDPT) -> bool` — [`PDPT`](../../../cptr/cap_type/index.md#pdpt)

##### `impl PartialOrd for PDPT`

- <span id="pdpt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PDPT) -> option::Option<cmp::Ordering>` — [`PDPT`](../../../cptr/cap_type/index.md#pdpt)

##### `impl StructuralPartialEq for PDPT`

### `PageDirectory`

```rust
struct PageDirectory;
```

#### Trait Implementations

##### `impl CapType for PageDirectory`

- <span id="pagedirectory-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PageDirectory`

- <span id="pagedirectory-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PageDirectory`

- <span id="pagedirectory-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PageDirectory`

- <span id="cap-typepagedirectory-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PageDirectory`

- <span id="pagedirectory-clone"></span>`fn clone(&self) -> PageDirectory` — [`PageDirectory`](../../../cptr/cap_type/index.md#pagedirectory)

##### `impl Copy for PageDirectory`

##### `impl Debug for PageDirectory`

- <span id="pagedirectory-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PageDirectory`

##### `impl Hash for PageDirectory`

- <span id="pagedirectory-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PageDirectory`

- <span id="pagedirectory-ord-cmp"></span>`fn cmp(&self, other: &PageDirectory) -> cmp::Ordering` — [`PageDirectory`](../../../cptr/cap_type/index.md#pagedirectory)

##### `impl PartialEq for PageDirectory`

- <span id="pagedirectory-partialeq-eq"></span>`fn eq(&self, other: &PageDirectory) -> bool` — [`PageDirectory`](../../../cptr/cap_type/index.md#pagedirectory)

##### `impl PartialOrd for PageDirectory`

- <span id="pagedirectory-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PageDirectory) -> option::Option<cmp::Ordering>` — [`PageDirectory`](../../../cptr/cap_type/index.md#pagedirectory)

##### `impl StructuralPartialEq for PageDirectory`

### `PageTable`

```rust
struct PageTable;
```

#### Trait Implementations

##### `impl CapType for PageTable`

- <span id="pagetable-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PageTable`

- <span id="pagetable-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PageTable`

- <span id="pagetable-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PageTable`

- <span id="cap-typepagetable-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PageTable`

- <span id="pagetable-clone"></span>`fn clone(&self) -> PageTable` — [`PageTable`](../../../cptr/cap_type/index.md#pagetable)

##### `impl Copy for PageTable`

##### `impl Debug for PageTable`

- <span id="pagetable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PageTable`

##### `impl Hash for PageTable`

- <span id="pagetable-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PageTable`

- <span id="pagetable-ord-cmp"></span>`fn cmp(&self, other: &PageTable) -> cmp::Ordering` — [`PageTable`](../../../cptr/cap_type/index.md#pagetable)

##### `impl PartialEq for PageTable`

- <span id="pagetable-partialeq-eq"></span>`fn eq(&self, other: &PageTable) -> bool` — [`PageTable`](../../../cptr/cap_type/index.md#pagetable)

##### `impl PartialOrd for PageTable`

- <span id="pagetable-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PageTable) -> option::Option<cmp::Ordering>` — [`PageTable`](../../../cptr/cap_type/index.md#pagetable)

##### `impl StructuralPartialEq for PageTable`

### `IOPortControl`

```rust
struct IOPortControl;
```

#### Trait Implementations

##### `impl CapType for IOPortControl`

- <span id="ioportcontrol-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for IOPortControl`

- <span id="ioportcontrol-clone"></span>`fn clone(&self) -> IOPortControl` — [`IOPortControl`](../../../cptr/cap_type/index.md#ioportcontrol)

##### `impl Copy for IOPortControl`

##### `impl Debug for IOPortControl`

- <span id="ioportcontrol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IOPortControl`

##### `impl Hash for IOPortControl`

- <span id="ioportcontrol-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for IOPortControl`

- <span id="ioportcontrol-ord-cmp"></span>`fn cmp(&self, other: &IOPortControl) -> cmp::Ordering` — [`IOPortControl`](../../../cptr/cap_type/index.md#ioportcontrol)

##### `impl PartialEq for IOPortControl`

- <span id="ioportcontrol-partialeq-eq"></span>`fn eq(&self, other: &IOPortControl) -> bool` — [`IOPortControl`](../../../cptr/cap_type/index.md#ioportcontrol)

##### `impl PartialOrd for IOPortControl`

- <span id="ioportcontrol-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &IOPortControl) -> option::Option<cmp::Ordering>` — [`IOPortControl`](../../../cptr/cap_type/index.md#ioportcontrol)

##### `impl StructuralPartialEq for IOPortControl`

## Type Aliases

### `VSpace`

```rust
type VSpace = PML4;
```

### `Granule`

```rust
type Granule = _4k;
```

