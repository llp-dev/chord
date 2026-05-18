*[sel4](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [object](index.md)*

---

# Module `object`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ObjectTypeX86`](#objecttypex86) | enum |  |
| [`ObjectBlueprintX86`](#objectblueprintx86) | enum |  |
| [`ObjectTypeArch`](#objecttypearch) | type |  |
| [`ObjectBlueprintArch`](#objectblueprintarch) | type |  |

## Enums

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

- <span id="objecttypex86-clone"></span>`fn clone(&self) -> ObjectTypeX86` — [`ObjectTypeX86`](#objecttypex86)

##### `impl Debug for ObjectTypeX86`

- <span id="objecttypex86-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectTypeX86`

##### `impl PartialEq for ObjectTypeX86`

- <span id="objecttypex86-partialeq-eq"></span>`fn eq(&self, other: &ObjectTypeX86) -> bool` — [`ObjectTypeX86`](#objecttypex86)

##### `impl StructuralPartialEq for ObjectTypeX86`

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

- <span id="objectblueprintx86-ty"></span>`const fn ty(self) -> ObjectTypeX86` — [`ObjectTypeX86`](#objecttypex86)

- <span id="objectblueprintx86-physical-size-bits"></span>`const fn physical_size_bits(self) -> usize`

#### Trait Implementations

##### `impl Clone for ObjectBlueprintX86`

- <span id="objectblueprintx86-clone"></span>`fn clone(&self) -> ObjectBlueprintX86` — [`ObjectBlueprintX86`](#objectblueprintx86)

##### `impl Copy for ObjectBlueprintX86`

##### `impl Debug for ObjectBlueprintX86`

- <span id="objectblueprintx86-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectBlueprintX86`

##### `impl Ord for ObjectBlueprintX86`

- <span id="objectblueprintx86-ord-cmp"></span>`fn cmp(&self, other: &ObjectBlueprintX86) -> cmp::Ordering` — [`ObjectBlueprintX86`](#objectblueprintx86)

##### `impl PartialEq for ObjectBlueprintX86`

- <span id="objectblueprintx86-partialeq-eq"></span>`fn eq(&self, other: &ObjectBlueprintX86) -> bool` — [`ObjectBlueprintX86`](#objectblueprintx86)

##### `impl PartialOrd for ObjectBlueprintX86`

- <span id="objectblueprintx86-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ObjectBlueprintX86) -> option::Option<cmp::Ordering>` — [`ObjectBlueprintX86`](#objectblueprintx86)

##### `impl StructuralPartialEq for ObjectBlueprintX86`

## Type Aliases

### `ObjectTypeArch`

```rust
type ObjectTypeArch = ObjectTypeX86;
```

### `ObjectBlueprintArch`

```rust
type ObjectBlueprintArch = ObjectBlueprintX86;
```

