*[sel4](../../index.md) / [cptr](../index.md) / [cap](index.md)*

---

# Module `cap`

Marked aliases of [`Cap`](crate::Cap).

Each type `$t<C = NoExplicitInvocationContext>` in this module is an alias for `Cap<$t, C>`.

## Contents

- [Type Aliases](#type-aliases)
  - [`Untyped`](#untyped)
  - [`Endpoint`](#endpoint)
  - [`Notification`](#notification)
  - [`Tcb`](#tcb)
  - [`CNode`](#cnode)
  - [`IrqControl`](#irqcontrol)
  - [`IrqHandler`](#irqhandler)
  - [`DomainSet`](#domainset)
  - [`AsidControl`](#asidcontrol)
  - [`AsidPool`](#asidpool)
  - [`Null`](#null)
  - [`Unspecified`](#unspecified)
  - [`UnspecifiedPage`](#unspecifiedpage)
  - [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable)
  - [`VSpace`](#vspace)
  - [`Granule`](#granule)
  - [`_4k`](#4k)
  - [`LargePage`](#largepage)
  - [`HugePage`](#hugepage)
  - [`PML4`](#pml4)
  - [`PDPT`](#pdpt)
  - [`PageDirectory`](#pagedirectory)
  - [`PageTable`](#pagetable)
  - [`IOPortControl`](#ioportcontrol)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Untyped`](#untyped) | type |  |
| [`Endpoint`](#endpoint) | type |  |
| [`Notification`](#notification) | type |  |
| [`Tcb`](#tcb) | type |  |
| [`CNode`](#cnode) | type |  |
| [`IrqControl`](#irqcontrol) | type |  |
| [`IrqHandler`](#irqhandler) | type |  |
| [`DomainSet`](#domainset) | type |  |
| [`AsidControl`](#asidcontrol) | type |  |
| [`AsidPool`](#asidpool) | type |  |
| [`Null`](#null) | type |  |
| [`Unspecified`](#unspecified) | type |  |
| [`UnspecifiedPage`](#unspecifiedpage) | type |  |
| [`UnspecifiedIntermediateTranslationTable`](#unspecifiedintermediatetranslationtable) | type |  |
| [`VSpace`](#vspace) | type |  |
| [`Granule`](#granule) | type |  |
| [`_4k`](#4k) | type |  |
| [`LargePage`](#largepage) | type |  |
| [`HugePage`](#hugepage) | type |  |
| [`PML4`](#pml4) | type |  |
| [`PDPT`](#pdpt) | type |  |
| [`PageDirectory`](#pagedirectory) | type |  |
| [`PageTable`](#pagetable) | type |  |
| [`IOPortControl`](#ioportcontrol) | type |  |

## Type Aliases

### `Untyped<C>`

```rust
type Untyped<C> = Cap<cap_type::Untyped, C>;
```

### `Endpoint<C>`

```rust
type Endpoint<C> = Cap<cap_type::Endpoint, C>;
```

### `Notification<C>`

```rust
type Notification<C> = Cap<cap_type::Notification, C>;
```

### `Tcb<C>`

```rust
type Tcb<C> = Cap<cap_type::Tcb, C>;
```

### `CNode<C>`

```rust
type CNode<C> = Cap<cap_type::CNode, C>;
```

### `IrqControl<C>`

```rust
type IrqControl<C> = Cap<cap_type::IrqControl, C>;
```

### `IrqHandler<C>`

```rust
type IrqHandler<C> = Cap<cap_type::IrqHandler, C>;
```

### `DomainSet<C>`

```rust
type DomainSet<C> = Cap<cap_type::DomainSet, C>;
```

### `AsidControl<C>`

```rust
type AsidControl<C> = Cap<cap_type::AsidControl, C>;
```

### `AsidPool<C>`

```rust
type AsidPool<C> = Cap<cap_type::AsidPool, C>;
```

### `Null<C>`

```rust
type Null<C> = Cap<cap_type::Null, C>;
```

### `Unspecified<C>`

```rust
type Unspecified<C> = Cap<cap_type::Unspecified, C>;
```

### `UnspecifiedPage<C>`

```rust
type UnspecifiedPage<C> = Cap<cap_type::UnspecifiedPage, C>;
```

### `UnspecifiedIntermediateTranslationTable<C>`

```rust
type UnspecifiedIntermediateTranslationTable<C> = Cap<cap_type::UnspecifiedIntermediateTranslationTable, C>;
```

### `VSpace<C>`

```rust
type VSpace<C> = Cap<cap_type::VSpace, C>;
```

### `Granule<C>`

```rust
type Granule<C> = Cap<cap_type::Granule, C>;
```

### `_4k<C>`

```rust
type _4k<C> = Cap<cap_type::_4k, C>;
```

### `LargePage<C>`

```rust
type LargePage<C> = Cap<cap_type::LargePage, C>;
```

### `HugePage<C>`

```rust
type HugePage<C> = Cap<cap_type::HugePage, C>;
```

### `PML4<C>`

```rust
type PML4<C> = Cap<cap_type::PML4, C>;
```

### `PDPT<C>`

```rust
type PDPT<C> = Cap<cap_type::PDPT, C>;
```

### `PageDirectory<C>`

```rust
type PageDirectory<C> = Cap<cap_type::PageDirectory, C>;
```

### `PageTable<C>`

```rust
type PageTable<C> = Cap<cap_type::PageTable, C>;
```

### `IOPortControl<C>`

```rust
type IOPortControl<C> = Cap<cap_type::IOPortControl, C>;
```

