*[sel4_abstract_ptr](../index.md) / [memory_type](index.md)*

---

# Module `memory_type`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MemoryType`](#memorytype) | trait |  |
| [`UnitaryOps`](#unitaryops) | trait |  |
| [`BulkOps`](#bulkops) | trait |  |
| [`AtomicOps`](#atomicops) | trait |  |

## Traits

### `MemoryType`

```rust
trait MemoryType { ... }
```

### `UnitaryOps<T>`

```rust
trait UnitaryOps<T>: MemoryType { ... }
```

#### Required Methods

- `fn read(src: *const T) -> T`

- `fn write(dst: *mut T, src: T)`

### `BulkOps<T>`

```rust
trait BulkOps<T>: MemoryType { ... }
```

#### Required Methods

- `fn memmove(dst: *mut T, src: *const T, count: usize)`

- `fn memcpy_into(dst: *mut T, src: *const T, count: usize)`

- `fn memcpy_from(dst: *mut T, src: *const T, count: usize)`

- `fn memset(dst: *mut T, val: u8, count: usize)`

### `AtomicOps<T>`

```rust
trait AtomicOps<T>: MemoryType { ... }
```

#### Associated Types

- `type Value`

#### Required Methods

- `fn atomic_store(dst: *mut T, val: <Self as >::Value, order: Ordering)`

- `fn atomic_load(dst: *const T, order: Ordering) -> <Self as >::Value`

- `fn atomic_swap(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_add(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_sub(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_compare_exchange(dst: *mut T, old: <Self as >::Value, new: <Self as >::Value, success: Ordering, failure: Ordering) -> Result<<Self as >::Value, <Self as >::Value>`

- `fn atomic_compare_exchange_weak(dst: *mut T, old: <Self as >::Value, new: <Self as >::Value, success: Ordering, failure: Ordering) -> Result<<Self as >::Value, <Self as >::Value>`

- `fn atomic_and(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_nand(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_or(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_xor(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_max(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

- `fn atomic_min(dst: *mut T, val: <Self as >::Value, order: Ordering) -> <Self as >::Value`

