*[sel4](../../index.md) / [cptr](../index.md) / [cap_type](index.md)*

---

# Module `cap_type`

Markers corresponding to capability types and classes of capability types.

These types are used for marking [`Cap`](crate::Cap).

## Contents

- [Structs](#structs)
  - [`Untyped`](#untyped)
  - [`Endpoint`](#endpoint)
  - [`Notification`](#notification)
  - [`Tcb`](#tcb)
  - [`CNode`](#cnode)
  - [`IrqControl`](#irqcontrol)
  - [`IrqHandler`](#irqhandler)
  - [`AsidControl`](#asidcontrol)
  - [`AsidPool`](#asidpool)
  - [`DomainSet`](#domainset)
  - [`Null`](#null)
  - [`Unspecified`](#unspecified)
  - [`UnspecifiedPage`](#unspecifiedpage)
  - [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable)
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
| [`Untyped`](#untyped) | struct | Corresponds to `seL4_Untyped`. |
| [`Endpoint`](#endpoint) | struct | Corresponds to the endpoint capability type. |
| [`Notification`](#notification) | struct | Corresponds to the notification capability type. |
| [`Tcb`](#tcb) | struct | Corresponds to `seL4_TCB`. |
| [`CNode`](#cnode) | struct | Corresponds to `seL4_CNode`. |
| [`IrqControl`](#irqcontrol) | struct | Corresponds to `seL4_IRQControl`. |
| [`IrqHandler`](#irqhandler) | struct | Corresponds to `seL4_IRQHandler`. |
| [`AsidControl`](#asidcontrol) | struct | Corresponds to `seL4_ASIDControl`. |
| [`AsidPool`](#asidpool) | struct | Corresponds to `seL4_ASIDPool`. |
| [`DomainSet`](#domainset) | struct | Corresponds to `seL4_DomainSet` |
| [`Null`](#null) | struct | Corresponds to the null capability. |
| [`Unspecified`](#unspecified) | struct | Any capability. |
| [`UnspecifiedPage`](#unspecifiedpage) | struct | Any page capability. |
| [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable) | struct | Any intermediate translation table capability. |
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

### `Untyped`

```rust
struct Untyped;
```

Corresponds to `seL4_Untyped`.

#### Trait Implementations

##### `impl CapType for Untyped`

- <span id="untyped-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for Untyped`

- <span id="untyped-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfVariableSize for Untyped`

- <span id="untyped-captypeforobjectofvariablesize-object-blueprint"></span>`fn object_blueprint(size_bits: usize) -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for Untyped`

- <span id="untyped-clone"></span>`fn clone(&self) -> Untyped` — [`Untyped`](#untyped)

##### `impl Copy for Untyped`

##### `impl Debug for Untyped`

- <span id="untyped-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Untyped`

##### `impl Hash for Untyped`

- <span id="untyped-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Untyped`

- <span id="untyped-ord-cmp"></span>`fn cmp(&self, other: &Untyped) -> cmp::Ordering` — [`Untyped`](#untyped)

##### `impl PartialEq for Untyped`

- <span id="untyped-partialeq-eq"></span>`fn eq(&self, other: &Untyped) -> bool` — [`Untyped`](#untyped)

##### `impl PartialOrd for Untyped`

- <span id="untyped-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Untyped) -> option::Option<cmp::Ordering>` — [`Untyped`](#untyped)

##### `impl StructuralPartialEq for Untyped`

### `Endpoint`

```rust
struct Endpoint;
```

Corresponds to the endpoint capability type.

#### Trait Implementations

##### `impl CapType for Endpoint`

- <span id="endpoint-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for Endpoint`

- <span id="endpoint-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for Endpoint`

- <span id="endpoint-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for Endpoint`

- <span id="endpoint-clone"></span>`fn clone(&self) -> Endpoint` — [`Endpoint`](#endpoint)

##### `impl Copy for Endpoint`

##### `impl Debug for Endpoint`

- <span id="endpoint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Endpoint`

##### `impl Hash for Endpoint`

- <span id="endpoint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IpcCapType for cap_type::Endpoint`

##### `impl Ord for Endpoint`

- <span id="endpoint-ord-cmp"></span>`fn cmp(&self, other: &Endpoint) -> cmp::Ordering` — [`Endpoint`](#endpoint)

##### `impl PartialEq for Endpoint`

- <span id="endpoint-partialeq-eq"></span>`fn eq(&self, other: &Endpoint) -> bool` — [`Endpoint`](#endpoint)

##### `impl PartialOrd for Endpoint`

- <span id="endpoint-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Endpoint) -> option::Option<cmp::Ordering>` — [`Endpoint`](#endpoint)

##### `impl StructuralPartialEq for Endpoint`

### `Notification`

```rust
struct Notification;
```

Corresponds to the notification capability type.

#### Trait Implementations

##### `impl CapType for Notification`

- <span id="notification-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for Notification`

- <span id="notification-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for Notification`

- <span id="notification-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for Notification`

- <span id="notification-clone"></span>`fn clone(&self) -> Notification` — [`Notification`](#notification)

##### `impl Copy for Notification`

##### `impl Debug for Notification`

- <span id="notification-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Notification`

##### `impl Hash for Notification`

- <span id="notification-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IpcCapType for cap_type::Notification`

##### `impl Ord for Notification`

- <span id="notification-ord-cmp"></span>`fn cmp(&self, other: &Notification) -> cmp::Ordering` — [`Notification`](#notification)

##### `impl PartialEq for Notification`

- <span id="notification-partialeq-eq"></span>`fn eq(&self, other: &Notification) -> bool` — [`Notification`](#notification)

##### `impl PartialOrd for Notification`

- <span id="notification-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Notification) -> option::Option<cmp::Ordering>` — [`Notification`](#notification)

##### `impl StructuralPartialEq for Notification`

### `Tcb`

```rust
struct Tcb;
```

Corresponds to `seL4_TCB`.

#### Trait Implementations

##### `impl CapType for Tcb`

- <span id="tcb-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for Tcb`

- <span id="tcb-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for Tcb`

- <span id="tcb-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for Tcb`

- <span id="tcb-clone"></span>`fn clone(&self) -> Tcb` — [`Tcb`](#tcb)

##### `impl Copy for Tcb`

##### `impl Debug for Tcb`

- <span id="tcb-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Tcb`

##### `impl Hash for Tcb`

- <span id="tcb-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Tcb`

- <span id="tcb-ord-cmp"></span>`fn cmp(&self, other: &Tcb) -> cmp::Ordering` — [`Tcb`](#tcb)

##### `impl PartialEq for Tcb`

- <span id="tcb-partialeq-eq"></span>`fn eq(&self, other: &Tcb) -> bool` — [`Tcb`](#tcb)

##### `impl PartialOrd for Tcb`

- <span id="tcb-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Tcb) -> option::Option<cmp::Ordering>` — [`Tcb`](#tcb)

##### `impl StructuralPartialEq for Tcb`

### `CNode`

```rust
struct CNode;
```

Corresponds to `seL4_CNode`.

#### Trait Implementations

##### `impl CapType for CNode`

- <span id="cnode-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for CNode`

- <span id="cnode-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfVariableSize for CNode`

- <span id="cnode-captypeforobjectofvariablesize-object-blueprint"></span>`fn object_blueprint(size_bits: usize) -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for CNode`

- <span id="cnode-clone"></span>`fn clone(&self) -> CNode` — [`CNode`](#cnode)

##### `impl Copy for CNode`

##### `impl Debug for CNode`

- <span id="cnode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CNode`

##### `impl Hash for CNode`

- <span id="cnode-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CNode`

- <span id="cnode-ord-cmp"></span>`fn cmp(&self, other: &CNode) -> cmp::Ordering` — [`CNode`](#cnode)

##### `impl PartialEq for CNode`

- <span id="cnode-partialeq-eq"></span>`fn eq(&self, other: &CNode) -> bool` — [`CNode`](#cnode)

##### `impl PartialOrd for CNode`

- <span id="cnode-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CNode) -> option::Option<cmp::Ordering>` — [`CNode`](#cnode)

##### `impl StructuralPartialEq for CNode`

### `IrqControl`

```rust
struct IrqControl;
```

Corresponds to `seL4_IRQControl`.

#### Trait Implementations

##### `impl CapType for IrqControl`

- <span id="irqcontrol-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for IrqControl`

- <span id="irqcontrol-clone"></span>`fn clone(&self) -> IrqControl` — [`IrqControl`](#irqcontrol)

##### `impl Copy for IrqControl`

##### `impl Debug for IrqControl`

- <span id="irqcontrol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IrqControl`

##### `impl Hash for IrqControl`

- <span id="irqcontrol-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for IrqControl`

- <span id="irqcontrol-ord-cmp"></span>`fn cmp(&self, other: &IrqControl) -> cmp::Ordering` — [`IrqControl`](#irqcontrol)

##### `impl PartialEq for IrqControl`

- <span id="irqcontrol-partialeq-eq"></span>`fn eq(&self, other: &IrqControl) -> bool` — [`IrqControl`](#irqcontrol)

##### `impl PartialOrd for IrqControl`

- <span id="irqcontrol-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &IrqControl) -> option::Option<cmp::Ordering>` — [`IrqControl`](#irqcontrol)

##### `impl StructuralPartialEq for IrqControl`

### `IrqHandler`

```rust
struct IrqHandler;
```

Corresponds to `seL4_IRQHandler`.

#### Trait Implementations

##### `impl CapType for IrqHandler`

- <span id="irqhandler-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for IrqHandler`

- <span id="irqhandler-clone"></span>`fn clone(&self) -> IrqHandler` — [`IrqHandler`](#irqhandler)

##### `impl Copy for IrqHandler`

##### `impl Debug for IrqHandler`

- <span id="irqhandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IrqHandler`

##### `impl Hash for IrqHandler`

- <span id="irqhandler-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for IrqHandler`

- <span id="irqhandler-ord-cmp"></span>`fn cmp(&self, other: &IrqHandler) -> cmp::Ordering` — [`IrqHandler`](#irqhandler)

##### `impl PartialEq for IrqHandler`

- <span id="irqhandler-partialeq-eq"></span>`fn eq(&self, other: &IrqHandler) -> bool` — [`IrqHandler`](#irqhandler)

##### `impl PartialOrd for IrqHandler`

- <span id="irqhandler-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &IrqHandler) -> option::Option<cmp::Ordering>` — [`IrqHandler`](#irqhandler)

##### `impl StructuralPartialEq for IrqHandler`

### `AsidControl`

```rust
struct AsidControl;
```

Corresponds to `seL4_ASIDControl`.

#### Trait Implementations

##### `impl CapType for AsidControl`

- <span id="asidcontrol-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for AsidControl`

- <span id="asidcontrol-clone"></span>`fn clone(&self) -> AsidControl` — [`AsidControl`](#asidcontrol)

##### `impl Copy for AsidControl`

##### `impl Debug for AsidControl`

- <span id="asidcontrol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AsidControl`

##### `impl Hash for AsidControl`

- <span id="asidcontrol-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for AsidControl`

- <span id="asidcontrol-ord-cmp"></span>`fn cmp(&self, other: &AsidControl) -> cmp::Ordering` — [`AsidControl`](#asidcontrol)

##### `impl PartialEq for AsidControl`

- <span id="asidcontrol-partialeq-eq"></span>`fn eq(&self, other: &AsidControl) -> bool` — [`AsidControl`](#asidcontrol)

##### `impl PartialOrd for AsidControl`

- <span id="asidcontrol-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &AsidControl) -> option::Option<cmp::Ordering>` — [`AsidControl`](#asidcontrol)

##### `impl StructuralPartialEq for AsidControl`

### `AsidPool`

```rust
struct AsidPool;
```

Corresponds to `seL4_ASIDPool`.

#### Trait Implementations

##### `impl CapType for AsidPool`

- <span id="asidpool-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for AsidPool`

- <span id="asidpool-clone"></span>`fn clone(&self) -> AsidPool` — [`AsidPool`](#asidpool)

##### `impl Copy for AsidPool`

##### `impl Debug for AsidPool`

- <span id="asidpool-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AsidPool`

##### `impl Hash for AsidPool`

- <span id="asidpool-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for AsidPool`

- <span id="asidpool-ord-cmp"></span>`fn cmp(&self, other: &AsidPool) -> cmp::Ordering` — [`AsidPool`](#asidpool)

##### `impl PartialEq for AsidPool`

- <span id="asidpool-partialeq-eq"></span>`fn eq(&self, other: &AsidPool) -> bool` — [`AsidPool`](#asidpool)

##### `impl PartialOrd for AsidPool`

- <span id="asidpool-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &AsidPool) -> option::Option<cmp::Ordering>` — [`AsidPool`](#asidpool)

##### `impl StructuralPartialEq for AsidPool`

### `DomainSet`

```rust
struct DomainSet;
```

Corresponds to `seL4_DomainSet`

#### Trait Implementations

##### `impl CapType for DomainSet`

- <span id="domainset-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for DomainSet`

- <span id="domainset-clone"></span>`fn clone(&self) -> DomainSet` — [`DomainSet`](#domainset)

##### `impl Copy for DomainSet`

##### `impl Debug for DomainSet`

- <span id="domainset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DomainSet`

##### `impl Hash for DomainSet`

- <span id="domainset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for DomainSet`

- <span id="domainset-ord-cmp"></span>`fn cmp(&self, other: &DomainSet) -> cmp::Ordering` — [`DomainSet`](#domainset)

##### `impl PartialEq for DomainSet`

- <span id="domainset-partialeq-eq"></span>`fn eq(&self, other: &DomainSet) -> bool` — [`DomainSet`](#domainset)

##### `impl PartialOrd for DomainSet`

- <span id="domainset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DomainSet) -> option::Option<cmp::Ordering>` — [`DomainSet`](#domainset)

##### `impl StructuralPartialEq for DomainSet`

### `Null`

```rust
struct Null;
```

Corresponds to the null capability.

#### Trait Implementations

##### `impl CapType for Null`

- <span id="null-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for Null`

- <span id="null-clone"></span>`fn clone(&self) -> Null` — [`Null`](#null)

##### `impl Copy for Null`

##### `impl Debug for Null`

- <span id="null-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Null`

##### `impl Hash for Null`

- <span id="null-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Null`

- <span id="null-ord-cmp"></span>`fn cmp(&self, other: &Null) -> cmp::Ordering` — [`Null`](#null)

##### `impl PartialEq for Null`

- <span id="null-partialeq-eq"></span>`fn eq(&self, other: &Null) -> bool` — [`Null`](#null)

##### `impl PartialOrd for Null`

- <span id="null-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Null) -> option::Option<cmp::Ordering>` — [`Null`](#null)

##### `impl StructuralPartialEq for Null`

### `Unspecified`

```rust
struct Unspecified;
```

Any capability.

#### Trait Implementations

##### `impl CapType for Unspecified`

- <span id="unspecified-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for Unspecified`

- <span id="unspecified-clone"></span>`fn clone(&self) -> Unspecified` — [`Unspecified`](#unspecified)

##### `impl Copy for Unspecified`

##### `impl Debug for Unspecified`

- <span id="unspecified-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Unspecified`

##### `impl Hash for Unspecified`

- <span id="unspecified-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IpcCapType for cap_type::Unspecified`

##### `impl Ord for Unspecified`

- <span id="unspecified-ord-cmp"></span>`fn cmp(&self, other: &Unspecified) -> cmp::Ordering` — [`Unspecified`](#unspecified)

##### `impl PartialEq for Unspecified`

- <span id="unspecified-partialeq-eq"></span>`fn eq(&self, other: &Unspecified) -> bool` — [`Unspecified`](#unspecified)

##### `impl PartialOrd for Unspecified`

- <span id="unspecified-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Unspecified) -> option::Option<cmp::Ordering>` — [`Unspecified`](#unspecified)

##### `impl StructuralPartialEq for Unspecified`

### `UnspecifiedPage`

```rust
struct UnspecifiedPage;
```

Any page capability.

#### Trait Implementations

##### `impl CapType for UnspecifiedPage`

- <span id="unspecifiedpage-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForFrameObject for cap_type::UnspecifiedPage`

##### `impl Clone for UnspecifiedPage`

- <span id="unspecifiedpage-clone"></span>`fn clone(&self) -> UnspecifiedPage` — [`UnspecifiedPage`](#unspecifiedpage)

##### `impl Copy for UnspecifiedPage`

##### `impl Debug for UnspecifiedPage`

- <span id="unspecifiedpage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnspecifiedPage`

##### `impl Hash for UnspecifiedPage`

- <span id="unspecifiedpage-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for UnspecifiedPage`

- <span id="unspecifiedpage-ord-cmp"></span>`fn cmp(&self, other: &UnspecifiedPage) -> cmp::Ordering` — [`UnspecifiedPage`](#unspecifiedpage)

##### `impl PartialEq for UnspecifiedPage`

- <span id="unspecifiedpage-partialeq-eq"></span>`fn eq(&self, other: &UnspecifiedPage) -> bool` — [`UnspecifiedPage`](#unspecifiedpage)

##### `impl PartialOrd for UnspecifiedPage`

- <span id="unspecifiedpage-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnspecifiedPage) -> option::Option<cmp::Ordering>` — [`UnspecifiedPage`](#unspecifiedpage)

##### `impl StructuralPartialEq for UnspecifiedPage`

### `UnspecifiedIntermediateTranslationTable`

```rust
struct UnspecifiedIntermediateTranslationTable;
```

Any intermediate translation table capability.

#### Trait Implementations

##### `impl CapType for UnspecifiedIntermediateTranslationTable`

- <span id="unspecifiedintermediatetranslationtable-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for UnspecifiedIntermediateTranslationTable`

- <span id="unspecifiedintermediatetranslationtable-clone"></span>`fn clone(&self) -> UnspecifiedIntermediateTranslationTable` — [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable)

##### `impl Copy for UnspecifiedIntermediateTranslationTable`

##### `impl Debug for UnspecifiedIntermediateTranslationTable`

- <span id="unspecifiedintermediatetranslationtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnspecifiedIntermediateTranslationTable`

##### `impl Hash for UnspecifiedIntermediateTranslationTable`

- <span id="unspecifiedintermediatetranslationtable-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for UnspecifiedIntermediateTranslationTable`

- <span id="unspecifiedintermediatetranslationtable-ord-cmp"></span>`fn cmp(&self, other: &UnspecifiedIntermediateTranslationTable) -> cmp::Ordering` — [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable)

##### `impl PartialEq for UnspecifiedIntermediateTranslationTable`

- <span id="unspecifiedintermediatetranslationtable-partialeq-eq"></span>`fn eq(&self, other: &UnspecifiedIntermediateTranslationTable) -> bool` — [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable)

##### `impl PartialOrd for UnspecifiedIntermediateTranslationTable`

- <span id="unspecifiedintermediatetranslationtable-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnspecifiedIntermediateTranslationTable) -> option::Option<cmp::Ordering>` — [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable)

##### `impl StructuralPartialEq for UnspecifiedIntermediateTranslationTable`

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

- <span id="4k-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for _4k`

- <span id="4k-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for _4k`

- <span id="4k-clone"></span>`fn clone(&self) -> _4k` — [`_4k`](#4k)

##### `impl Copy for _4k`

##### `impl Debug for _4k`

- <span id="4k-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for _4k`

##### `impl Hash for _4k`

- <span id="4k-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for _4k`

- <span id="4k-ord-cmp"></span>`fn cmp(&self, other: &_4k) -> cmp::Ordering` — [`_4k`](#4k)

##### `impl PartialEq for _4k`

- <span id="4k-partialeq-eq"></span>`fn eq(&self, other: &_4k) -> bool` — [`_4k`](#4k)

##### `impl PartialOrd for _4k`

- <span id="4k-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &_4k) -> option::Option<cmp::Ordering>` — [`_4k`](#4k)

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

- <span id="largepage-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for LargePage`

- <span id="largepage-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for LargePage`

- <span id="largepage-clone"></span>`fn clone(&self) -> LargePage` — [`LargePage`](#largepage)

##### `impl Copy for LargePage`

##### `impl Debug for LargePage`

- <span id="largepage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LargePage`

##### `impl Hash for LargePage`

- <span id="largepage-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LargePage`

- <span id="largepage-ord-cmp"></span>`fn cmp(&self, other: &LargePage) -> cmp::Ordering` — [`LargePage`](#largepage)

##### `impl PartialEq for LargePage`

- <span id="largepage-partialeq-eq"></span>`fn eq(&self, other: &LargePage) -> bool` — [`LargePage`](#largepage)

##### `impl PartialOrd for LargePage`

- <span id="largepage-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LargePage) -> option::Option<cmp::Ordering>` — [`LargePage`](#largepage)

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

- <span id="hugepage-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for HugePage`

- <span id="hugepage-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl Clone for HugePage`

- <span id="hugepage-clone"></span>`fn clone(&self) -> HugePage` — [`HugePage`](#hugepage)

##### `impl Copy for HugePage`

##### `impl Debug for HugePage`

- <span id="hugepage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HugePage`

##### `impl Hash for HugePage`

- <span id="hugepage-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for HugePage`

- <span id="hugepage-ord-cmp"></span>`fn cmp(&self, other: &HugePage) -> cmp::Ordering` — [`HugePage`](#hugepage)

##### `impl PartialEq for HugePage`

- <span id="hugepage-partialeq-eq"></span>`fn eq(&self, other: &HugePage) -> bool` — [`HugePage`](#hugepage)

##### `impl PartialOrd for HugePage`

- <span id="hugepage-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &HugePage) -> option::Option<cmp::Ordering>` — [`HugePage`](#hugepage)

##### `impl StructuralPartialEq for HugePage`

### `PML4`

```rust
struct PML4;
```

#### Trait Implementations

##### `impl CapType for PML4`

- <span id="pml4-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PML4`

- <span id="pml4-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PML4`

- <span id="pml4-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PML4`

- <span id="cap-typepml4-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PML4`

- <span id="pml4-clone"></span>`fn clone(&self) -> PML4` — [`PML4`](#pml4)

##### `impl Copy for PML4`

##### `impl Debug for PML4`

- <span id="pml4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PML4`

##### `impl Hash for PML4`

- <span id="pml4-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PML4`

- <span id="pml4-ord-cmp"></span>`fn cmp(&self, other: &PML4) -> cmp::Ordering` — [`PML4`](#pml4)

##### `impl PartialEq for PML4`

- <span id="pml4-partialeq-eq"></span>`fn eq(&self, other: &PML4) -> bool` — [`PML4`](#pml4)

##### `impl PartialOrd for PML4`

- <span id="pml4-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PML4) -> option::Option<cmp::Ordering>` — [`PML4`](#pml4)

##### `impl StructuralPartialEq for PML4`

### `PDPT`

```rust
struct PDPT;
```

#### Trait Implementations

##### `impl CapType for PDPT`

- <span id="pdpt-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PDPT`

- <span id="pdpt-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PDPT`

- <span id="pdpt-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PDPT`

- <span id="cap-typepdpt-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PDPT`

- <span id="pdpt-clone"></span>`fn clone(&self) -> PDPT` — [`PDPT`](#pdpt)

##### `impl Copy for PDPT`

##### `impl Debug for PDPT`

- <span id="pdpt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PDPT`

##### `impl Hash for PDPT`

- <span id="pdpt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PDPT`

- <span id="pdpt-ord-cmp"></span>`fn cmp(&self, other: &PDPT) -> cmp::Ordering` — [`PDPT`](#pdpt)

##### `impl PartialEq for PDPT`

- <span id="pdpt-partialeq-eq"></span>`fn eq(&self, other: &PDPT) -> bool` — [`PDPT`](#pdpt)

##### `impl PartialOrd for PDPT`

- <span id="pdpt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PDPT) -> option::Option<cmp::Ordering>` — [`PDPT`](#pdpt)

##### `impl StructuralPartialEq for PDPT`

### `PageDirectory`

```rust
struct PageDirectory;
```

#### Trait Implementations

##### `impl CapType for PageDirectory`

- <span id="pagedirectory-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PageDirectory`

- <span id="pagedirectory-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PageDirectory`

- <span id="pagedirectory-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PageDirectory`

- <span id="cap-typepagedirectory-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PageDirectory`

- <span id="pagedirectory-clone"></span>`fn clone(&self) -> PageDirectory` — [`PageDirectory`](#pagedirectory)

##### `impl Copy for PageDirectory`

##### `impl Debug for PageDirectory`

- <span id="pagedirectory-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PageDirectory`

##### `impl Hash for PageDirectory`

- <span id="pagedirectory-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PageDirectory`

- <span id="pagedirectory-ord-cmp"></span>`fn cmp(&self, other: &PageDirectory) -> cmp::Ordering` — [`PageDirectory`](#pagedirectory)

##### `impl PartialEq for PageDirectory`

- <span id="pagedirectory-partialeq-eq"></span>`fn eq(&self, other: &PageDirectory) -> bool` — [`PageDirectory`](#pagedirectory)

##### `impl PartialOrd for PageDirectory`

- <span id="pagedirectory-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PageDirectory) -> option::Option<cmp::Ordering>` — [`PageDirectory`](#pagedirectory)

##### `impl StructuralPartialEq for PageDirectory`

### `PageTable`

```rust
struct PageTable;
```

#### Trait Implementations

##### `impl CapType for PageTable`

- <span id="pagetable-captype-const-name"></span>`const NAME: &'static str`

##### `impl CapTypeForObject for PageTable`

- <span id="pagetable-captypeforobject-object-type"></span>`fn object_type() -> ObjectType` — [`ObjectType`](../../object/index.md#objecttype)

##### `impl CapTypeForObjectOfFixedSize for PageTable`

- <span id="pagetable-captypeforobjectoffixedsize-object-blueprint"></span>`fn object_blueprint() -> ObjectBlueprint` — [`ObjectBlueprint`](../../object/index.md#objectblueprint)

##### `impl CapTypeForTranslationTableObject for cap_type::PageTable`

- <span id="cap-typepagetable-captypefortranslationtableobject-const-translation-table-object-type"></span>`const TRANSLATION_TABLE_OBJECT_TYPE: TranslationTableObjectType`

##### `impl Clone for PageTable`

- <span id="pagetable-clone"></span>`fn clone(&self) -> PageTable` — [`PageTable`](#pagetable)

##### `impl Copy for PageTable`

##### `impl Debug for PageTable`

- <span id="pagetable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PageTable`

##### `impl Hash for PageTable`

- <span id="pagetable-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PageTable`

- <span id="pagetable-ord-cmp"></span>`fn cmp(&self, other: &PageTable) -> cmp::Ordering` — [`PageTable`](#pagetable)

##### `impl PartialEq for PageTable`

- <span id="pagetable-partialeq-eq"></span>`fn eq(&self, other: &PageTable) -> bool` — [`PageTable`](#pagetable)

##### `impl PartialOrd for PageTable`

- <span id="pagetable-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PageTable) -> option::Option<cmp::Ordering>` — [`PageTable`](#pagetable)

##### `impl StructuralPartialEq for PageTable`

### `IOPortControl`

```rust
struct IOPortControl;
```

#### Trait Implementations

##### `impl CapType for IOPortControl`

- <span id="ioportcontrol-captype-const-name"></span>`const NAME: &'static str`

##### `impl Clone for IOPortControl`

- <span id="ioportcontrol-clone"></span>`fn clone(&self) -> IOPortControl` — [`IOPortControl`](#ioportcontrol)

##### `impl Copy for IOPortControl`

##### `impl Debug for IOPortControl`

- <span id="ioportcontrol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IOPortControl`

##### `impl Hash for IOPortControl`

- <span id="ioportcontrol-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for IOPortControl`

- <span id="ioportcontrol-ord-cmp"></span>`fn cmp(&self, other: &IOPortControl) -> cmp::Ordering` — [`IOPortControl`](#ioportcontrol)

##### `impl PartialEq for IOPortControl`

- <span id="ioportcontrol-partialeq-eq"></span>`fn eq(&self, other: &IOPortControl) -> bool` — [`IOPortControl`](#ioportcontrol)

##### `impl PartialOrd for IOPortControl`

- <span id="ioportcontrol-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &IOPortControl) -> option::Option<cmp::Ordering>` — [`IOPortControl`](#ioportcontrol)

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

