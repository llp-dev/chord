# Crate `sel4_shared_memory`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ops`](#ops) | mod |  |
| [`map_field`](#map-field) | mod |  |
| [`SharedMemory`](#sharedmemory) | struct |  |
| [`access`](#access) | type |  |
| [`SharedMemoryRef`](#sharedmemoryref) | type |  |
| [`SharedMemoryPtr`](#sharedmemoryptr) | type |  |

## Modules

- [`ops`](ops/index.md)
- [`map_field`](map_field/index.md)

## Structs

### `SharedMemory`

```rust
struct SharedMemory(());
```

#### Trait Implementations

##### `impl<T: FromBytes + IntoBytes> BulkOps for crate::SharedMemory`

- <span id="cratesharedmemory-bulkops-memmove"></span>`unsafe fn memmove(dst: *mut T, src: *const T, count: usize)`

- <span id="cratesharedmemory-bulkops-memcpy-into"></span>`unsafe fn memcpy_into(dst: *mut T, src: *const T, count: usize)`

- <span id="cratesharedmemory-bulkops-memcpy-from"></span>`unsafe fn memcpy_from(dst: *mut T, src: *const T, count: usize)`

- <span id="cratesharedmemory-bulkops-memset"></span>`unsafe fn memset(dst: *mut T, val: u8, count: usize)`

##### `impl MemoryType for SharedMemory`

##### `impl Pointee for SharedMemory`

- <span id="sharedmemory-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<T: FromBytes + IntoBytes> UnitaryOps for crate::SharedMemory`

- <span id="cratesharedmemory-unitaryops-read"></span>`unsafe fn read(src: *const T) -> T`

- <span id="cratesharedmemory-unitaryops-write"></span>`unsafe fn write(dst: *mut T, src: T)`

## Type Aliases

### `access<T>`

```rust
type access<T> = smallvec::IntoIter<[T; 16]>;
```

### `SharedMemoryRef<'a, T, A>`

```rust
type SharedMemoryRef<'a, T, A> = sel4_abstract_ptr::AbstractRef<'a, SharedMemory, T, A>;
```

### `SharedMemoryPtr<'a, T, A>`

```rust
type SharedMemoryPtr<'a, T, A> = sel4_abstract_ptr::AbstractPtr<'a, SharedMemory, T, A>;
```

