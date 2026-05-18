# Crate `sel4_abstract_ptr`

## Contents

- [Modules](#modules)
  - [`abstract_ptr`](#abstract-ptr)
  - [`abstract_ref`](#abstract-ref)
  - [`core_ext`](#core-ext)
  - [`access`](#access)
  - [`memory_type`](#memory-type)
- [Structs](#structs)
  - [`AbstractPtr`](#abstractptr)
  - [`AbstractRef`](#abstractref)
- [Macros](#macros)
  - [`map_field!`](#map-field)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`abstract_ptr`](#abstract-ptr) | mod |  |
| [`abstract_ref`](#abstract-ref) | mod |  |
| [`core_ext`](#core-ext) | mod |  |
| [`access`](#access) | mod | Marker types for limiting access. |
| [`memory_type`](#memory-type) | mod |  |
| [`AbstractPtr`](#abstractptr) | struct |  |
| [`AbstractRef`](#abstractref) | struct |  |
| [`map_field!`](#map-field) | macro | Provides safe field projection for abstract pointers referencing structs. |

## Modules

- [`abstract_ptr`](abstract_ptr/index.md)
- [`abstract_ref`](abstract_ref/index.md)
- [`core_ext`](core_ext/index.md)
- [`access`](access/index.md) — Marker types for limiting access.
- [`memory_type`](memory_type/index.md)

## Structs

### `AbstractPtr<'a, M, T, A>`

```rust
struct AbstractPtr<'a, M, T, A>
where
    T: ?Sized {
    pointer: core::ptr::NonNull<T>,
    memory_type: core::marker::PhantomData<M>,
    reference: core::marker::PhantomData<&'a T>,
    access: core::marker::PhantomData<A>,
}
```

#### Implementations

- <span id="crateabstractptr-atomic-load"></span>`fn atomic_load(self, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-store"></span>`fn atomic_store(self, val: <M as >::Value, order: Ordering)` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-swap"></span>`fn atomic_swap(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-compare-exchange"></span>`fn atomic_compare_exchange(self, current: <M as >::Value, new: <M as >::Value, success: Ordering, failure: Ordering) -> Result<<M as >::Value, <M as >::Value>` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-compare-exchange-weak"></span>`fn atomic_compare_exchange_weak(self, current: <M as >::Value, new: <M as >::Value, success: Ordering, failure: Ordering) -> Result<<M as >::Value, <M as >::Value>` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-add"></span>`fn atomic_fetch_add(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-sub"></span>`fn atomic_fetch_sub(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-and"></span>`fn atomic_fetch_and(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-nand"></span>`fn atomic_fetch_nand(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-or"></span>`fn atomic_fetch_or(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-xor"></span>`fn atomic_fetch_xor(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-update"></span>`fn atomic_fetch_update<F>(self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<<M as >::Value, <M as >::Value>` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-max"></span>`fn atomic_fetch_max(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-min"></span>`fn atomic_fetch_min(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](memory_type/index.md#atomicops)

#### Trait Implementations

##### `impl<M, T, A> Clone for AbstractPtr<'_, M, T, A>`

- <span id="abstractptr-clone"></span>`fn clone(&self) -> Self`

##### `impl<M, T, A> Copy for AbstractPtr<'_, M, T, A>`

##### `impl<M, T, A> Debug for AbstractPtr<'_, M, T, A>`

- <span id="abstractptr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M, T, A> Eq for AbstractPtr<'_, M, T, A>`

##### `impl<M, T, A> Hash for AbstractPtr<'_, M, T, A>`

- <span id="abstractptr-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<M, T, A> Ord for AbstractPtr<'_, M, T, A>`

- <span id="abstractptr-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<M, T, A> PartialEq for AbstractPtr<'_, M, T, A>`

- <span id="abstractptr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<M, T, A> PartialOrd for AbstractPtr<'_, M, T, A>`

- <span id="abstractptr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Pointee for AbstractPtr<'a, M, T, A>`

- <span id="abstractptr-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<M, T, A> Pointer for AbstractPtr<'_, M, T, A>`

- <span id="abstractptr-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AbstractRef<'a, M, T, A>`

```rust
struct AbstractRef<'a, M, T, A>
where
    T: ?Sized {
    pointer: core::ptr::NonNull<T>,
    memory_type: core::marker::PhantomData<M>,
    reference: core::marker::PhantomData<&'a T>,
    access: core::marker::PhantomData<A>,
}
```

#### Implementations

- <span id="abstractref-new"></span>`unsafe fn new(pointer: NonNull<T>) -> Self`

- <span id="abstractref-new-read-only"></span>`const unsafe fn new_read_only(pointer: NonNull<T>) -> AbstractRef<'a, M, T, ReadOnly>` — [`AbstractRef`](abstract_ref/index.md#abstractref), [`ReadOnly`](access/index.md#readonly)

- <span id="abstractref-new-restricted"></span>`const unsafe fn new_restricted<A>(access: A, pointer: NonNull<T>) -> AbstractRef<'a, M, T, A>` — [`AbstractRef`](abstract_ref/index.md#abstractref)

- <span id="abstractref-from-ref"></span>`fn from_ref(reference: &'a T) -> AbstractRef<'a, M, T, ReadOnly>` — [`AbstractRef`](abstract_ref/index.md#abstractref), [`ReadOnly`](access/index.md#readonly)

- <span id="abstractref-from-mut-ref"></span>`fn from_mut_ref(reference: &'a mut T) -> Self`

- <span id="abstractref-new-generic"></span>`const unsafe fn new_generic<A>(pointer: NonNull<T>) -> AbstractRef<'a, M, T, A>` — [`AbstractRef`](abstract_ref/index.md#abstractref)

#### Trait Implementations

##### `impl<M, T, A> Clone for AbstractRef<'_, M, T, A>`

- <span id="abstractref-clone"></span>`fn clone(&self) -> Self`

##### `impl<M, T, A> Copy for AbstractRef<'_, M, T, A>`

##### `impl<M, T, A> Debug for AbstractRef<'_, M, T, A>`

- <span id="abstractref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M, T, A> Eq for AbstractRef<'_, M, T, A>`

##### `impl<M, T, A> Hash for AbstractRef<'_, M, T, A>`

- <span id="abstractref-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<M, T, A> Ord for AbstractRef<'_, M, T, A>`

- <span id="abstractref-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<M, T, A> PartialEq for AbstractRef<'_, M, T, A>`

- <span id="abstractref-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<M, T, A> PartialOrd for AbstractRef<'_, M, T, A>`

- <span id="abstractref-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Pointee for AbstractRef<'a, M, T, A>`

- <span id="abstractref-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<M, T, A> Pointer for AbstractRef<'_, M, T, A>`

- <span id="abstractref-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M, T, A> Send for AbstractRef<'_, M, T, A>`

##### `impl<M, T, A> Sync for AbstractRef<'_, M, T, A>`

## Macros

### `map_field!`

Provides safe field projection for abstract pointers referencing structs.

## Examples

Accessing a struct field:

```ignore
use sel4_abstract_ptr::{AbstractPtr, map_field};

struct Example { field_1: u32, field_2: u8, }
let mut value = Example { field_1: 15, field_2: 255 };
let ptr = unsafe { AbstractPtr::new((&mut value).into()) };

// construct an abstract reference to a field
let field_2 = map_field!(ptr.field_2);
assert_eq!(field_2.read(), 255);
```

Creating `AbstractPtr`s to unaligned field in packed structs is not allowed:
```ignore
use sel4_abstract_ptr::{AbstractPtr, map_field};

#[repr(packed)]
struct Example { field_1: u8, field_2: usize, }
let mut value = Example { field_1: 15, field_2: 255 };
let ptr = unsafe { AbstractPtr::new((&mut value).into()) };

// Constructing an abstract reference to an unaligned field doesn't compile.
let field_2 = map_field!(ptr.field_2);
```

