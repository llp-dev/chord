*[sel4](../index.md) / [cptr](index.md)*

---

# Module `cptr`

## Contents

- [Modules](#modules)
  - [`cap_type`](#cap-type)
  - [`cap`](#cap)
- [Structs](#structs)
  - [`CPtr`](#cptr)
  - [`CPtrWithDepth`](#cptrwithdepth)
  - [`Cap`](#cap)
  - [`AbsoluteCPtr`](#absolutecptr)
- [Traits](#traits)
  - [`CapType`](#captype)
  - [`HasCPtrWithDepth`](#hascptrwithdepth)
- [Type Aliases](#type-aliases)
  - [`CPtrBits`](#cptrbits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cap_type`](#cap-type) | mod | Markers corresponding to capability types and classes of capability types. |
| [`cap`](#cap) | mod | Marked aliases of [`Cap`](crate::Cap). |
| [`CPtr`](#cptr) | struct | A capability pointer. |
| [`CPtrWithDepth`](#cptrwithdepth) | struct | A capability pointer with a number of bits to resolve. |
| [`Cap`](#cap) | struct | A capability pointer to be resolved in the current CSpace. |
| [`AbsoluteCPtr`](#absolutecptr) | struct | A [`CPtrWithDepth`] to be resolved in the context of a particular [`CNode`]. |
| [`CapType`](#captype) | trait | Trait for marker types corresponding to capability types in the seL4 API. |
| [`HasCPtrWithDepth`](#hascptrwithdepth) | trait | Trait for types whose members which logically contain a [`CPtrWithDepth`]. |
| [`CPtrBits`](#cptrbits) | type | The raw bits of a capability pointer. |

## Modules

- [`cap_type`](cap_type/index.md) — Markers corresponding to capability types and classes of capability types.
- [`cap`](cap/index.md) — Marked aliases of [`Cap`](crate::Cap).

## Structs

### `CPtr`

```rust
struct CPtr {
    bits: CPtrBits,
}
```

A capability pointer.

#### Implementations

- <span id="cptr-bits"></span>`const fn bits(self) -> CPtrBits` — [`CPtrBits`](#cptrbits)

- <span id="cptr-from-bits"></span>`const fn from_bits(bits: CPtrBits) -> Self` — [`CPtrBits`](#cptrbits)

- <span id="cptr-cast"></span>`const fn cast<T: CapType>(self) -> Cap<T>` — [`Cap`](#cap)

#### Trait Implementations

##### `impl Clone for CPtr`

- <span id="cptr-clone"></span>`fn clone(&self) -> CPtr` — [`CPtr`](#cptr)

##### `impl Copy for CPtr`

##### `impl Debug for CPtr`

- <span id="cptr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CPtr`

##### `impl HasCPtrWithDepth for CPtr`

- <span id="cptr-hascptrwithdepth-cptr-with-depth"></span>`fn cptr_with_depth(self) -> CPtrWithDepth` — [`CPtrWithDepth`](#cptrwithdepth)

##### `impl Hash for CPtr`

- <span id="cptr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CPtr`

- <span id="cptr-ord-cmp"></span>`fn cmp(&self, other: &CPtr) -> cmp::Ordering` — [`CPtr`](#cptr)

##### `impl PartialEq for CPtr`

- <span id="cptr-partialeq-eq"></span>`fn eq(&self, other: &CPtr) -> bool` — [`CPtr`](#cptr)

##### `impl PartialOrd for CPtr`

- <span id="cptr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CPtr) -> option::Option<cmp::Ordering>` — [`CPtr`](#cptr)

##### `impl StructuralPartialEq for CPtr`

### `CPtrWithDepth`

```rust
struct CPtrWithDepth {
    bits: CPtrBits,
    depth: usize,
}
```

A capability pointer with a number of bits to resolve.

#### Implementations

- <span id="cptrwithdepth-from-bits-with-depth"></span>`const fn from_bits_with_depth(bits: CPtrBits, depth: usize) -> Self` — [`CPtrBits`](#cptrbits)

- <span id="cptrwithdepth-bits"></span>`const fn bits(&self) -> CPtrBits` — [`CPtrBits`](#cptrbits)

- <span id="cptrwithdepth-depth"></span>`const fn depth(&self) -> usize`

- <span id="cptrwithdepth-empty"></span>`const fn empty() -> Self`

  The [`CPtrWithDepth`](#cptrwithdepth) with a depth of 0.

- <span id="cptrwithdepth-depth-for-kernel"></span>`fn depth_for_kernel(&self) -> u8`

#### Trait Implementations

##### `impl Clone for CPtrWithDepth`

- <span id="cptrwithdepth-clone"></span>`fn clone(&self) -> CPtrWithDepth` — [`CPtrWithDepth`](#cptrwithdepth)

##### `impl Copy for CPtrWithDepth`

##### `impl Debug for CPtrWithDepth`

- <span id="cptrwithdepth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CPtrWithDepth`

##### `impl HasCPtrWithDepth for CPtrWithDepth`

- <span id="cptrwithdepth-hascptrwithdepth-cptr-with-depth"></span>`fn cptr_with_depth(self) -> CPtrWithDepth` — [`CPtrWithDepth`](#cptrwithdepth)

##### `impl Hash for CPtrWithDepth`

- <span id="cptrwithdepth-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CPtrWithDepth`

- <span id="cptrwithdepth-ord-cmp"></span>`fn cmp(&self, other: &CPtrWithDepth) -> cmp::Ordering` — [`CPtrWithDepth`](#cptrwithdepth)

##### `impl PartialEq for CPtrWithDepth`

- <span id="cptrwithdepth-partialeq-eq"></span>`fn eq(&self, other: &CPtrWithDepth) -> bool` — [`CPtrWithDepth`](#cptrwithdepth)

##### `impl PartialOrd for CPtrWithDepth`

- <span id="cptrwithdepth-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CPtrWithDepth) -> option::Option<cmp::Ordering>` — [`CPtrWithDepth`](#cptrwithdepth)

##### `impl StructuralPartialEq for CPtrWithDepth`

### `Cap<T: CapType, C>`

```rust
struct Cap<T: CapType, C> {
    cptr: CPtr,
    invocation_context: C,
    _phantom: core::marker::PhantomData<T>,
}
```

A capability pointer to be resolved in the current CSpace.

- The `T` parameter is a [`CapType`](#captype) marking the type of the pointed-to capability.
- The `C` parameter is a strategy for discovering the current thread's IPC buffer. When the
  `"state"` feature is enabled, [`NoExplicitInvocationContext`](../invocation_context/index.md) is an alias for
  [`ImplicitInvocationContext`](crate::ImplicitInvocationContext), which uses the [`IpcBuffer`](../ipc_buffer/index.md)
  set by [`set_ipc_buffer`](crate::set_ipc_buffer). Otherwise, it is an alias for
  [`NoInvocationContext`](crate::NoInvocationContext), which does not implement
  [`InvocationContext`](../invocation_context/index.md). In such cases, the [`with`](Cap::with) method is used to specify an
  invocation context before the capability is invoked.

The most general way to construct a [`Cap`](#cap) is with `CPtr::cast`.

Note that `seL4_CNode_*` capability invocations are methods of [`AbsoluteCPtr`](#absolutecptr) rather than
[`Cap`](#cap).

#### Implementations

- <span id="cratecap-frame-map"></span>`fn frame_map(self, vspace: VSpace, vaddr: usize, rights: CapRights, attrs: VmAttributes) -> Result<()>` — [`VSpace`](cap/index.md#vspace), [`CapRights`](../cap_rights/index.md#caprights), [`VmAttributes`](../arch/imp/vm_attributes/index.md#vmattributes), [`Result`](../error/index.md#result)

  Corresponds to `seL4_X86_Page_Map`.

- <span id="cratecap-frame-unmap"></span>`fn frame_unmap(self) -> Result<()>` — [`Result`](../error/index.md#result)

  Corresponds to `seL4_X86_Page_Unmap`.

- <span id="cratecap-frame-get-address"></span>`fn frame_get_address(self) -> Result<usize>` — [`Result`](../error/index.md#result)

  Corresponds to `seL4_X86_Page_GetAddress`.

#### Trait Implementations

##### `impl<T: clone::Clone + CapType, C: clone::Clone> Clone for Cap<T, C>`

- <span id="cap-clone"></span>`fn clone(&self) -> Cap<T, C>` — [`Cap`](#cap)

##### `impl<T: marker::Copy + CapType, C: marker::Copy> Copy for Cap<T, C>`

##### `impl<T: CapType, C> Debug for Cap<T, C>`

- <span id="cap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + CapType, C: cmp::Eq> Eq for Cap<T, C>`

##### `impl<T: CapType, C> HasCPtrWithDepth for Cap<T, C>`

- <span id="cap-hascptrwithdepth-cptr-with-depth"></span>`fn cptr_with_depth(self) -> CPtrWithDepth` — [`CPtrWithDepth`](#cptrwithdepth)

##### `impl<T: hash::Hash + CapType, C: hash::Hash> Hash for Cap<T, C>`

- <span id="cap-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord + CapType, C: cmp::Ord> Ord for Cap<T, C>`

- <span id="cap-ord-cmp"></span>`fn cmp(&self, other: &Cap<T, C>) -> cmp::Ordering` — [`Cap`](#cap)

##### `impl<T: cmp::PartialEq + CapType, C: cmp::PartialEq> PartialEq for Cap<T, C>`

- <span id="cap-partialeq-eq"></span>`fn eq(&self, other: &Cap<T, C>) -> bool` — [`Cap`](#cap)

##### `impl<T: cmp::PartialOrd + CapType, C: cmp::PartialOrd> PartialOrd for Cap<T, C>`

- <span id="cap-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Cap<T, C>) -> option::Option<cmp::Ordering>` — [`Cap`](#cap)

##### `impl<T: CapType, C> StructuralPartialEq for Cap<T, C>`

### `AbsoluteCPtr<C>`

```rust
struct AbsoluteCPtr<C> {
    root: CNode<C>,
    path: CPtrWithDepth,
}
```

A [`CPtrWithDepth`](#cptrwithdepth) to be resolved in the context of a particular [`CNode`](cap/index.md).

[`AbsoluteCPtr`](#absolutecptr) addresses capability slots in a more general way than [`Cap`](#cap). It allows one to
address any capability slot that is directly addressable from any CNode that is directly
addressible in the current thread's CSpace. Furthermore, it allows one to address capability
slots that contain CNodes by limiting the lookup depth to prevent the kernel's lookup procedure
from descending into the CNode contained in that slot.

`seL4_CNode_*` capability invocations are methods of [`AbsoluteCPtr`](#absolutecptr) rather than [`Cap`](#cap).

In addition to `AbsoluteCPtr::new`, the following methods can be used to construct an
[`AbsoluteCPtr`](#absolutecptr):
- `CNode::absolute_cptr`
- `CNode::absolute_cptr_from_bits_with_depth`
- `CNode::absolute_cptr_for_self`

#### Implementations

- <span id="absolutecptr-new"></span>`const fn new(root: CNode<C>, path: CPtrWithDepth) -> Self` — [`CNode`](cap/index.md#cnode), [`CPtrWithDepth`](#cptrwithdepth)

- <span id="absolutecptr-root"></span>`const fn root(&self) -> &CNode<C>` — [`CNode`](cap/index.md#cnode)

- <span id="absolutecptr-into-root"></span>`fn into_root(self) -> CNode<C>` — [`CNode`](cap/index.md#cnode)

- <span id="absolutecptr-path"></span>`const fn path(&self) -> &CPtrWithDepth` — [`CPtrWithDepth`](#cptrwithdepth)

- <span id="absolutecptr-with"></span>`fn with<C1>(self, context: C1) -> AbsoluteCPtr<C1>` — [`AbsoluteCPtr`](#absolutecptr)

- <span id="absolutecptr-without-context"></span>`fn without_context(self) -> AbsoluteCPtr` — [`AbsoluteCPtr`](#absolutecptr)

#### Trait Implementations

##### `impl<C: clone::Clone> Clone for AbsoluteCPtr<C>`

- <span id="absolutecptr-clone"></span>`fn clone(&self) -> AbsoluteCPtr<C>` — [`AbsoluteCPtr`](#absolutecptr)

##### `impl<C: marker::Copy> Copy for AbsoluteCPtr<C>`

##### `impl<C: fmt::Debug> Debug for AbsoluteCPtr<C>`

- <span id="absolutecptr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<C: cmp::Eq> Eq for AbsoluteCPtr<C>`

##### `impl<C: cmp::PartialEq> PartialEq for AbsoluteCPtr<C>`

- <span id="absolutecptr-partialeq-eq"></span>`fn eq(&self, other: &AbsoluteCPtr<C>) -> bool` — [`AbsoluteCPtr`](#absolutecptr)

##### `impl<C> StructuralPartialEq for AbsoluteCPtr<C>`

## Traits

### `CapType`

```rust
trait CapType: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash { ... }
```

Trait for marker types corresponding to capability types in the seL4 API.

Implementors are used to mark instantiations of [`Cap`](#cap).

#### Associated Constants

- `const NAME: &'static str`

#### Implementors

- [`AsidControl`](cap_type/index.md#asidcontrol)
- [`AsidPool`](cap_type/index.md#asidpool)
- [`CNode`](cap_type/index.md#cnode)
- [`DomainSet`](cap_type/index.md#domainset)
- [`Endpoint`](cap_type/index.md#endpoint)
- [`HugePage`](cap_type/index.md#hugepage)
- [`IOPortControl`](cap_type/index.md#ioportcontrol)
- [`IrqControl`](cap_type/index.md#irqcontrol)
- [`IrqHandler`](cap_type/index.md#irqhandler)
- [`LargePage`](cap_type/index.md#largepage)
- [`Notification`](cap_type/index.md#notification)
- [`Null`](cap_type/index.md#null)
- [`PDPT`](cap_type/index.md#pdpt)
- [`PML4`](cap_type/index.md#pml4)
- [`PageDirectory`](cap_type/index.md#pagedirectory)
- [`PageTable`](cap_type/index.md#pagetable)
- [`Tcb`](cap_type/index.md#tcb)
- [`UnspecifiedIntermediateTranslationTable`](cap_type/index.md#unspecifiedintermediatetranslationtable)
- [`UnspecifiedPage`](cap_type/index.md#unspecifiedpage)
- [`Unspecified`](cap_type/index.md#unspecified)
- [`Untyped`](cap_type/index.md#untyped)
- [`_4k`](cap_type/index.md#4k)

### `HasCPtrWithDepth`

```rust
trait HasCPtrWithDepth { ... }
```

Trait for types whose members which logically contain a [`CPtrWithDepth`](#cptrwithdepth).

[`CPtr`](#cptr) and [`Cap`](#cap) each logically contain a [`CPtrWithDepth`](#cptrwithdepth) with a depth of [`WORD_SIZE`](../index.md).

#### Required Methods

- `fn cptr_with_depth(self) -> CPtrWithDepth`

  Returns the logical [`CPtrWithDepth`](#cptrwithdepth) entailed by `self`.

#### Implementors

- [`CPtrWithDepth`](#cptrwithdepth)
- [`CPtr`](#cptr)
- [`Cap`](#cap)

## Type Aliases

### `CPtrBits`

```rust
type CPtrBits = sys::seL4_CPtr;
```

The raw bits of a capability pointer.

