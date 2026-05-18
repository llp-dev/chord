*[sel4](../../../../../index.md) / [arch](../../../../index.md) / [imp](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [object](index.md)*

---

# Module `object`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ObjectTypeX64`](#objecttypex64) | enum |  |
| [`ObjectBlueprintX64`](#objectblueprintx64) | enum |  |
| [`ObjectTypeSeL4Arch`](#objecttypesel4arch) | type |  |
| [`ObjectBlueprintSeL4Arch`](#objectblueprintsel4arch) | type |  |

## Enums

### `ObjectTypeX64`

```rust
enum ObjectTypeX64 {
    HugePage,
    PDPT,
    PML4,
}
```

#### Implementations

- <span id="objecttypex64-into-sys"></span>`const fn into_sys(self) -> c_uint`

#### Trait Implementations

##### `impl Clone for ObjectTypeX64`

- <span id="objecttypex64-clone"></span>`fn clone(&self) -> ObjectTypeX64` — [`ObjectTypeX64`](#objecttypex64)

##### `impl Copy for ObjectTypeX64`

##### `impl Debug for ObjectTypeX64`

- <span id="objecttypex64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectTypeX64`

##### `impl PartialEq for ObjectTypeX64`

- <span id="objecttypex64-partialeq-eq"></span>`fn eq(&self, other: &ObjectTypeX64) -> bool` — [`ObjectTypeX64`](#objecttypex64)

##### `impl StructuralPartialEq for ObjectTypeX64`

### `ObjectBlueprintX64`

```rust
enum ObjectBlueprintX64 {
    HugePage,
    PDPT,
    PML4,
}
```

#### Implementations

- <span id="objectblueprintx64-ty"></span>`const fn ty(self) -> ObjectTypeX64` — [`ObjectTypeX64`](#objecttypex64)

- <span id="objectblueprintx64-physical-size-bits"></span>`const fn physical_size_bits(self) -> usize`

#### Trait Implementations

##### `impl Clone for ObjectBlueprintX64`

- <span id="objectblueprintx64-clone"></span>`fn clone(&self) -> ObjectBlueprintX64` — [`ObjectBlueprintX64`](#objectblueprintx64)

##### `impl Copy for ObjectBlueprintX64`

##### `impl Debug for ObjectBlueprintX64`

- <span id="objectblueprintx64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectBlueprintX64`

##### `impl Ord for ObjectBlueprintX64`

- <span id="objectblueprintx64-ord-cmp"></span>`fn cmp(&self, other: &ObjectBlueprintX64) -> cmp::Ordering` — [`ObjectBlueprintX64`](#objectblueprintx64)

##### `impl PartialEq for ObjectBlueprintX64`

- <span id="objectblueprintx64-partialeq-eq"></span>`fn eq(&self, other: &ObjectBlueprintX64) -> bool` — [`ObjectBlueprintX64`](#objectblueprintx64)

##### `impl PartialOrd for ObjectBlueprintX64`

- <span id="objectblueprintx64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ObjectBlueprintX64) -> option::Option<cmp::Ordering>` — [`ObjectBlueprintX64`](#objectblueprintx64)

##### `impl StructuralPartialEq for ObjectBlueprintX64`

## Type Aliases

### `ObjectTypeSeL4Arch`

```rust
type ObjectTypeSeL4Arch = ObjectTypeX64;
```

### `ObjectBlueprintSeL4Arch`

```rust
type ObjectBlueprintSeL4Arch = ObjectBlueprintX64;
```

