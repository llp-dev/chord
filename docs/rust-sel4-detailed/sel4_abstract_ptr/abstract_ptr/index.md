*[sel4_abstract_ptr](../index.md) / [abstract_ptr](index.md)*

---

# Module `abstract_ptr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`atomic_operations`](#atomic-operations) | mod |  |
| [`macros`](#macros) | mod |  |
| [`operations`](#operations) | mod |  |
| [`slice_operations`](#slice-operations) | mod |  |
| [`AbstractPtr`](#abstractptr) | struct |  |

## Modules

- [`atomic_operations`](atomic_operations/index.md)
- [`macros`](macros/index.md)
- [`operations`](operations/index.md)
- [`slice_operations`](slice_operations/index.md)

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

- <span id="crateabstractptr-atomic-load"></span>`fn atomic_load(self, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-store"></span>`fn atomic_store(self, val: <M as >::Value, order: Ordering)` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-swap"></span>`fn atomic_swap(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-compare-exchange"></span>`fn atomic_compare_exchange(self, current: <M as >::Value, new: <M as >::Value, success: Ordering, failure: Ordering) -> Result<<M as >::Value, <M as >::Value>` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-compare-exchange-weak"></span>`fn atomic_compare_exchange_weak(self, current: <M as >::Value, new: <M as >::Value, success: Ordering, failure: Ordering) -> Result<<M as >::Value, <M as >::Value>` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-add"></span>`fn atomic_fetch_add(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-sub"></span>`fn atomic_fetch_sub(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-and"></span>`fn atomic_fetch_and(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-nand"></span>`fn atomic_fetch_nand(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-or"></span>`fn atomic_fetch_or(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-xor"></span>`fn atomic_fetch_xor(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-update"></span>`fn atomic_fetch_update<F>(self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<<M as >::Value, <M as >::Value>` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-max"></span>`fn atomic_fetch_max(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

- <span id="crateabstractptr-atomic-fetch-min"></span>`fn atomic_fetch_min(self, val: <M as >::Value, order: Ordering) -> <M as >::Value` — [`AtomicOps`](../memory_type/index.md#atomicops)

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

