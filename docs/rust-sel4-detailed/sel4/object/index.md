*[sel4](../index.md) / [object](index.md)*

---

# Module `object`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ObjectType`](#objecttype) | enum | Corresponds to `seL4_ObjectType`. |
| [`ObjectBlueprint`](#objectblueprint) | enum | An object description for [`Untyped::untyped_retype`](crate::cap::Untyped::untyped_retype). |
| [`CapTypeForObject`](#captypeforobject) | trait | Trait for [`CapType`]s which correspond to kernel objects. |
| [`CapTypeForObjectOfFixedSize`](#captypeforobjectoffixedsize) | trait | Trait for [`CapType`]s which correspond to kernel objects of fixed size. |
| [`CapTypeForObjectOfVariableSize`](#captypeforobjectofvariablesize) | trait | Trait for [`CapType`]s which correspond to kernel objects of variable size. |

## Enums

### `ObjectType`

```rust
enum ObjectType {
    Untyped,
    Endpoint,
    Notification,
    CNode,
    Tcb,
    Arch(crate::ObjectTypeArch),
}
```

Corresponds to `seL4_ObjectType`.

#### Implementations

- <span id="objecttype-into-sys"></span>`const fn into_sys(self) -> c_uint`

#### Trait Implementations

##### `impl Clone for ObjectType`

- <span id="objecttype-clone"></span>`fn clone(&self) -> ObjectType` — [`ObjectType`](#objecttype)

##### `impl Debug for ObjectType`

- <span id="objecttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectType`

##### `impl PartialEq for ObjectType`

- <span id="objecttype-partialeq-eq"></span>`fn eq(&self, other: &ObjectType) -> bool` — [`ObjectType`](#objecttype)

##### `impl StructuralPartialEq for ObjectType`

### `ObjectBlueprint`

```rust
enum ObjectBlueprint {
    Untyped {
        size_bits: usize,
    },
    Endpoint,
    Notification,
    CNode {
        size_bits: usize,
    },
    Tcb,
    Arch(crate::ObjectBlueprintArch),
}
```

An object description for [`Untyped::untyped_retype`](crate::cap::Untyped::untyped_retype).

#### Implementations

- <span id="objectblueprint-ty"></span>`const fn ty(self) -> ObjectType` — [`ObjectType`](#objecttype)

- <span id="objectblueprint-api-size-bits"></span>`const fn api_size_bits(self) -> Option<usize>`

- <span id="objectblueprint-physical-size-bits"></span>`const fn physical_size_bits(self) -> usize`

- <span id="objectblueprint-asid-pool"></span>`const fn asid_pool() -> Self`

#### Trait Implementations

##### `impl Clone for ObjectBlueprint`

- <span id="objectblueprint-clone"></span>`fn clone(&self) -> ObjectBlueprint` — [`ObjectBlueprint`](#objectblueprint)

##### `impl Copy for ObjectBlueprint`

##### `impl Debug for ObjectBlueprint`

- <span id="objectblueprint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectBlueprint`

##### `impl Ord for ObjectBlueprint`

- <span id="objectblueprint-ord-cmp"></span>`fn cmp(&self, other: &ObjectBlueprint) -> cmp::Ordering` — [`ObjectBlueprint`](#objectblueprint)

##### `impl PartialEq for ObjectBlueprint`

- <span id="objectblueprint-partialeq-eq"></span>`fn eq(&self, other: &ObjectBlueprint) -> bool` — [`ObjectBlueprint`](#objectblueprint)

##### `impl PartialOrd for ObjectBlueprint`

- <span id="objectblueprint-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ObjectBlueprint) -> option::Option<cmp::Ordering>` — [`ObjectBlueprint`](#objectblueprint)

##### `impl StructuralPartialEq for ObjectBlueprint`

## Traits

### `CapTypeForObject`

```rust
trait CapTypeForObject: CapType { ... }
```

Trait for [`CapType`](../cptr/index.md)s which correspond to kernel objects.

#### Required Methods

- `fn object_type() -> ObjectType`

#### Implementors

- [`CNode`](../cptr/cap_type/index.md#cnode)
- [`Endpoint`](../cptr/cap_type/index.md#endpoint)
- [`HugePage`](../cptr/cap_type/index.md#hugepage)
- [`LargePage`](../cptr/cap_type/index.md#largepage)
- [`Notification`](../cptr/cap_type/index.md#notification)
- [`PDPT`](../cptr/cap_type/index.md#pdpt)
- [`PML4`](../cptr/cap_type/index.md#pml4)
- [`PageDirectory`](../cptr/cap_type/index.md#pagedirectory)
- [`PageTable`](../cptr/cap_type/index.md#pagetable)
- [`Tcb`](../cptr/cap_type/index.md#tcb)
- [`Untyped`](../cptr/cap_type/index.md#untyped)
- [`_4k`](../cptr/cap_type/index.md#4k)

### `CapTypeForObjectOfFixedSize`

```rust
trait CapTypeForObjectOfFixedSize: CapTypeForObject { ... }
```

Trait for [`CapType`](../cptr/index.md)s which correspond to kernel objects of fixed size.

#### Required Methods

- `fn object_blueprint() -> ObjectBlueprint`

#### Implementors

- [`Endpoint`](../cptr/cap_type/index.md#endpoint)
- [`HugePage`](../cptr/cap_type/index.md#hugepage)
- [`LargePage`](../cptr/cap_type/index.md#largepage)
- [`Notification`](../cptr/cap_type/index.md#notification)
- [`PDPT`](../cptr/cap_type/index.md#pdpt)
- [`PML4`](../cptr/cap_type/index.md#pml4)
- [`PageDirectory`](../cptr/cap_type/index.md#pagedirectory)
- [`PageTable`](../cptr/cap_type/index.md#pagetable)
- [`Tcb`](../cptr/cap_type/index.md#tcb)
- [`_4k`](../cptr/cap_type/index.md#4k)

### `CapTypeForObjectOfVariableSize`

```rust
trait CapTypeForObjectOfVariableSize: CapTypeForObject { ... }
```

Trait for [`CapType`](../cptr/index.md)s which correspond to kernel objects of variable size.

#### Required Methods

- `fn object_blueprint(size_bits: usize) -> ObjectBlueprint`

#### Implementors

- [`CNode`](../cptr/cap_type/index.md#cnode)
- [`Untyped`](../cptr/cap_type/index.md#untyped)

