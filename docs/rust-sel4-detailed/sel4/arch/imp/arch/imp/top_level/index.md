*[sel4](../../../../../index.md) / [arch](../../../../index.md) / [imp](../../../index.md) / [arch](../../index.md) / [imp](../index.md) / [top_level](index.md)*

---

# Module `top_level`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UserContext`](#usercontext) | struct |  |
| [`ObjectBlueprintX64`](#objectblueprintx64) | enum |  |
| [`ObjectTypeX64`](#objecttypex64) | enum |  |
| [`ObjectBlueprintSeL4Arch`](#objectblueprintsel4arch) | type |  |
| [`ObjectTypeSeL4Arch`](#objecttypesel4arch) | type |  |

## Structs

### `UserContext`

```rust
struct UserContext(sys::seL4_UserContext);
```

#### Implementations

- <span id="usercontext-from-inner"></span>`const fn from_inner(inner: sys::seL4_UserContext) -> Self`

- <span id="usercontext-into-inner"></span>`const fn into_inner(self) -> sys::seL4_UserContext`

- <span id="usercontext-inner"></span>`const fn inner(&self) -> &sys::seL4_UserContext`

- <span id="usercontext-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_UserContext`

- <span id="usercontext-pc"></span>`fn pc(&self) -> &Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-pc-mut"></span>`fn pc_mut(&mut self) -> &mut Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-sp"></span>`fn sp(&self) -> &Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-sp-mut"></span>`fn sp_mut(&mut self) -> &mut Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-c-param"></span>`fn c_param(&self, ix: usize) -> &Word` — [`Word`](../../../../../index.md#word)

- <span id="usercontext-c-param-mut"></span>`fn c_param_mut(&mut self, ix: usize) -> &mut Word` — [`Word`](../../../../../index.md#word)

#### Trait Implementations

##### `impl Clone for UserContext`

- <span id="usercontext-clone"></span>`fn clone(&self) -> UserContext` — [`UserContext`](../user_context/index.md#usercontext)

##### `impl Debug for UserContext`

- <span id="usercontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for UserContext`

- <span id="usercontext-default"></span>`fn default() -> UserContext` — [`UserContext`](../user_context/index.md#usercontext)

##### `impl Eq for UserContext`

##### `impl PartialEq for UserContext`

- <span id="usercontext-partialeq-eq"></span>`fn eq(&self, other: &UserContext) -> bool` — [`UserContext`](../user_context/index.md#usercontext)

##### `impl StructuralPartialEq for UserContext`

## Enums

### `ObjectBlueprintX64`

```rust
enum ObjectBlueprintX64 {
    HugePage,
    PDPT,
    PML4,
}
```

#### Implementations

- <span id="objectblueprintx64-ty"></span>`const fn ty(self) -> ObjectTypeX64` — [`ObjectTypeX64`](../object/index.md#objecttypex64)

- <span id="objectblueprintx64-physical-size-bits"></span>`const fn physical_size_bits(self) -> usize`

#### Trait Implementations

##### `impl Clone for ObjectBlueprintX64`

- <span id="objectblueprintx64-clone"></span>`fn clone(&self) -> ObjectBlueprintX64` — [`ObjectBlueprintX64`](../object/index.md#objectblueprintx64)

##### `impl Copy for ObjectBlueprintX64`

##### `impl Debug for ObjectBlueprintX64`

- <span id="objectblueprintx64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectBlueprintX64`

##### `impl Ord for ObjectBlueprintX64`

- <span id="objectblueprintx64-ord-cmp"></span>`fn cmp(&self, other: &ObjectBlueprintX64) -> cmp::Ordering` — [`ObjectBlueprintX64`](../object/index.md#objectblueprintx64)

##### `impl PartialEq for ObjectBlueprintX64`

- <span id="objectblueprintx64-partialeq-eq"></span>`fn eq(&self, other: &ObjectBlueprintX64) -> bool` — [`ObjectBlueprintX64`](../object/index.md#objectblueprintx64)

##### `impl PartialOrd for ObjectBlueprintX64`

- <span id="objectblueprintx64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ObjectBlueprintX64) -> option::Option<cmp::Ordering>` — [`ObjectBlueprintX64`](../object/index.md#objectblueprintx64)

##### `impl StructuralPartialEq for ObjectBlueprintX64`

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

- <span id="objecttypex64-clone"></span>`fn clone(&self) -> ObjectTypeX64` — [`ObjectTypeX64`](../object/index.md#objecttypex64)

##### `impl Copy for ObjectTypeX64`

##### `impl Debug for ObjectTypeX64`

- <span id="objecttypex64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectTypeX64`

##### `impl PartialEq for ObjectTypeX64`

- <span id="objecttypex64-partialeq-eq"></span>`fn eq(&self, other: &ObjectTypeX64) -> bool` — [`ObjectTypeX64`](../object/index.md#objecttypex64)

##### `impl StructuralPartialEq for ObjectTypeX64`

## Type Aliases

### `ObjectBlueprintSeL4Arch`

```rust
type ObjectBlueprintSeL4Arch = ObjectBlueprintX64;
```

### `ObjectTypeSeL4Arch`

```rust
type ObjectTypeSeL4Arch = ObjectTypeX64;
```

