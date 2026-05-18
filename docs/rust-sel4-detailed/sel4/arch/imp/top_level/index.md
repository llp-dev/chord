*[sel4](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [top_level](index.md)*

---

# Module `top_level`

## Contents

- [Structs](#structs)
  - [`VmAttributes`](#vmattributes)
- [Enums](#enums)
  - [`ObjectBlueprintX86`](#objectblueprintx86)
  - [`ObjectTypeX86`](#objecttypex86)
  - [`FrameObjectType`](#frameobjecttype)
  - [`TranslationTableObjectType`](#translationtableobjecttype)
- [Type Aliases](#type-aliases)
  - [`ObjectBlueprintArch`](#objectblueprintarch)
  - [`ObjectTypeArch`](#objecttypearch)
- [Constants](#constants)
  - [`NUM_FAST_MESSAGE_REGISTERS`](#num-fast-message-registers)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`VmAttributes`](#vmattributes) | struct |  |
| [`ObjectBlueprintX86`](#objectblueprintx86) | enum |  |
| [`ObjectTypeX86`](#objecttypex86) | enum |  |
| [`FrameObjectType`](#frameobjecttype) | enum |  |
| [`TranslationTableObjectType`](#translationtableobjecttype) | enum |  |
| [`ObjectBlueprintArch`](#objectblueprintarch) | type |  |
| [`ObjectTypeArch`](#objecttypearch) | type |  |
| [`NUM_FAST_MESSAGE_REGISTERS`](#num-fast-message-registers) | const |  |

## Structs

### `VmAttributes`

```rust
struct VmAttributes(sys::seL4_X86_VMAttributes::Type);
```

Corresponds to `seL4_X86_VMAttributes`.

#### Implementations

- <span id="vmattributes-const-none"></span>`const NONE: Self`

- <span id="vmattributes-const-default"></span>`const DEFAULT: Self`

- <span id="vmattributes-const-cache-disabled"></span>`const CACHE_DISABLED: Self`

- <span id="vmattributes-from-inner"></span>`const fn from_inner(inner: sys::seL4_X86_VMAttributes::Type) -> Self`

- <span id="vmattributes-into-inner"></span>`const fn into_inner(self) -> sys::seL4_X86_VMAttributes::Type`

- <span id="vmattributes-inner"></span>`const fn inner(&self) -> &sys::seL4_X86_VMAttributes::Type`

- <span id="vmattributes-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_X86_VMAttributes::Type`

- <span id="vmattributes-has"></span>`const fn has(self, rhs: Self) -> bool`

#### Trait Implementations

##### `impl BitAnd for VmAttributes`

- <span id="vmattributes-bitand-type-output"></span>`type Output = VmAttributes`

- <span id="vmattributes-bitand"></span>`fn bitand(self, rhs: Self) -> Self`

##### `impl BitAndAssign for VmAttributes`

- <span id="vmattributes-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: Self)`

##### `impl BitOr for VmAttributes`

- <span id="vmattributes-bitor-type-output"></span>`type Output = VmAttributes`

- <span id="vmattributes-bitor"></span>`fn bitor(self, rhs: Self) -> Self`

##### `impl BitOrAssign for VmAttributes`

- <span id="vmattributes-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: Self)`

##### `impl Clone for VmAttributes`

- <span id="vmattributes-clone"></span>`fn clone(&self) -> VmAttributes` — [`VmAttributes`](../vm_attributes/index.md#vmattributes)

##### `impl Copy for VmAttributes`

##### `impl Debug for VmAttributes`

- <span id="vmattributes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VmAttributes`

- <span id="vmattributes-default"></span>`fn default() -> Self`

##### `impl Eq for VmAttributes`

##### `impl Not for VmAttributes`

- <span id="vmattributes-not-type-output"></span>`type Output = VmAttributes`

- <span id="vmattributes-not"></span>`fn not(self) -> Self`

##### `impl PartialEq for VmAttributes`

- <span id="vmattributes-partialeq-eq"></span>`fn eq(&self, other: &VmAttributes) -> bool` — [`VmAttributes`](../vm_attributes/index.md#vmattributes)

##### `impl StructuralPartialEq for VmAttributes`

## Enums

### `ObjectBlueprintX86`

```rust
enum ObjectBlueprintX86 {
    _4k,
    LargePage,
    PageTable,
    PageDirectory,
    SeL4Arch(crate::ObjectBlueprintSeL4Arch),
}
```

#### Implementations

- <span id="objectblueprintx86-ty"></span>`const fn ty(self) -> ObjectTypeX86` — [`ObjectTypeX86`](../object/index.md#objecttypex86)

- <span id="objectblueprintx86-physical-size-bits"></span>`const fn physical_size_bits(self) -> usize`

#### Trait Implementations

##### `impl Clone for ObjectBlueprintX86`

- <span id="objectblueprintx86-clone"></span>`fn clone(&self) -> ObjectBlueprintX86` — [`ObjectBlueprintX86`](../object/index.md#objectblueprintx86)

##### `impl Copy for ObjectBlueprintX86`

##### `impl Debug for ObjectBlueprintX86`

- <span id="objectblueprintx86-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectBlueprintX86`

##### `impl Ord for ObjectBlueprintX86`

- <span id="objectblueprintx86-ord-cmp"></span>`fn cmp(&self, other: &ObjectBlueprintX86) -> cmp::Ordering` — [`ObjectBlueprintX86`](../object/index.md#objectblueprintx86)

##### `impl PartialEq for ObjectBlueprintX86`

- <span id="objectblueprintx86-partialeq-eq"></span>`fn eq(&self, other: &ObjectBlueprintX86) -> bool` — [`ObjectBlueprintX86`](../object/index.md#objectblueprintx86)

##### `impl PartialOrd for ObjectBlueprintX86`

- <span id="objectblueprintx86-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ObjectBlueprintX86) -> option::Option<cmp::Ordering>` — [`ObjectBlueprintX86`](../object/index.md#objectblueprintx86)

##### `impl StructuralPartialEq for ObjectBlueprintX86`

### `ObjectTypeX86`

```rust
enum ObjectTypeX86 {
    _4k,
    LargePage,
    PageTable,
    PageDirectory,
    SeL4Arch(crate::ObjectTypeSeL4Arch),
}
```

#### Implementations

- <span id="objecttypex86-into-sys"></span>`const fn into_sys(self) -> c_uint`

#### Trait Implementations

##### `impl Clone for ObjectTypeX86`

- <span id="objecttypex86-clone"></span>`fn clone(&self) -> ObjectTypeX86` — [`ObjectTypeX86`](../object/index.md#objecttypex86)

##### `impl Debug for ObjectTypeX86`

- <span id="objecttypex86-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectTypeX86`

##### `impl PartialEq for ObjectTypeX86`

- <span id="objecttypex86-partialeq-eq"></span>`fn eq(&self, other: &ObjectTypeX86) -> bool` — [`ObjectTypeX86`](../object/index.md#objecttypex86)

##### `impl StructuralPartialEq for ObjectTypeX86`

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

- <span id="frameobjecttype-clone"></span>`fn clone(&self) -> FrameObjectType` — [`FrameObjectType`](../vspace/index.md#frameobjecttype)

##### `impl Copy for FrameObjectType`

##### `impl Debug for FrameObjectType`

- <span id="frameobjecttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FrameObjectType`

##### `impl PartialEq for FrameObjectType`

- <span id="frameobjecttype-partialeq-eq"></span>`fn eq(&self, other: &FrameObjectType) -> bool` — [`FrameObjectType`](../vspace/index.md#frameobjecttype)

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

- <span id="translationtableobjecttype-clone"></span>`fn clone(&self) -> TranslationTableObjectType` — [`TranslationTableObjectType`](../vspace/index.md#translationtableobjecttype)

##### `impl Copy for TranslationTableObjectType`

##### `impl Debug for TranslationTableObjectType`

- <span id="translationtableobjecttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TranslationTableObjectType`

##### `impl PartialEq for TranslationTableObjectType`

- <span id="translationtableobjecttype-partialeq-eq"></span>`fn eq(&self, other: &TranslationTableObjectType) -> bool` — [`TranslationTableObjectType`](../vspace/index.md#translationtableobjecttype)

##### `impl StructuralPartialEq for TranslationTableObjectType`

## Type Aliases

### `ObjectBlueprintArch`

```rust
type ObjectBlueprintArch = ObjectBlueprintX86;
```

### `ObjectTypeArch`

```rust
type ObjectTypeArch = ObjectTypeX86;
```

## Constants

### `NUM_FAST_MESSAGE_REGISTERS`
```rust
const NUM_FAST_MESSAGE_REGISTERS: usize = 4usize;
```

