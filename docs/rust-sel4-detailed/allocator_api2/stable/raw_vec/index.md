*[allocator_api2](../../index.md) / [stable](../index.md) / [raw_vec](index.md)*

---

# Module `raw_vec`

## Contents

- [Structs](#structs)
  - [`TryReserveError`](#tryreserveerror)
  - [`RawVec`](#rawvec)
- [Enums](#enums)
  - [`TryReserveErrorKind`](#tryreserveerrorkind)
  - [`AllocInit`](#allocinit)
- [Functions](#functions)
  - [`finish_grow`](#finish-grow)
  - [`handle_reserve`](#handle-reserve)
  - [`alloc_guard`](#alloc-guard)
  - [`capacity_overflow`](#capacity-overflow)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReserveError`](#tryreserveerror) | struct | The error type for `try_reserve` methods. |
| [`RawVec`](#rawvec) | struct | A low-level utility for more ergonomically allocating, reallocating, and deallocating a buffer of memory on the heap without having to worry about all the corner cases involved. |
| [`TryReserveErrorKind`](#tryreserveerrorkind) | enum | Details of the allocation that caused a `TryReserveError` |
| [`AllocInit`](#allocinit) | enum |  |
| [`finish_grow`](#finish-grow) | fn |  |
| [`handle_reserve`](#handle-reserve) | fn |  |
| [`alloc_guard`](#alloc-guard) | fn |  |
| [`capacity_overflow`](#capacity-overflow) | fn |  |

## Structs

### `TryReserveError`

```rust
struct TryReserveError {
    kind: TryReserveErrorKind,
}
```

The error type for `try_reserve` methods.

#### Implementations

- <span id="tryreserveerror-kind"></span>`fn kind(&self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](#tryreserveerrorkind)

  Details about the allocation that caused the error

#### Trait Implementations

##### `impl Clone for TryReserveError`

- <span id="tryreserveerror-clone"></span>`fn clone(&self) -> TryReserveError` — [`TryReserveError`](#tryreserveerror)

##### `impl Debug for TryReserveError`

- <span id="tryreserveerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryReserveError`

- <span id="tryreserveerror-display-fmt"></span>`fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl Eq for TryReserveError`

##### `impl PartialEq for TryReserveError`

- <span id="tryreserveerror-partialeq-eq"></span>`fn eq(&self, other: &TryReserveError) -> bool` — [`TryReserveError`](#tryreserveerror)

##### `impl StructuralPartialEq for TryReserveError`

##### `impl ToString for TryReserveError`

- <span id="tryreserveerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `RawVec<T, A: Allocator>`

```rust
struct RawVec<T, A: Allocator> {
    ptr: core::ptr::NonNull<T>,
    cap: usize,
    alloc: A,
}
```

A low-level utility for more ergonomically allocating, reallocating, and deallocating
a buffer of memory on the heap without having to worry about all the corner cases
involved. This type is excellent for building your own data structures like Vec and VecDeque.
In particular:

* Produces `NonNull::dangling()` on zero-sized types.
* Produces `NonNull::dangling()` on zero-length allocations.
* Avoids freeing `NonNull::dangling()`.
* Catches all overflows in capacity computations (promotes them to "capacity overflow" panics).
* Guards against 32-bit systems allocating more than isize::MAX bytes.
* Guards against overflowing your length.
* Calls `handle_alloc_error` for fallible allocations.
* Contains a `ptr::NonNull` and thus endows the user with all related benefits.
* Uses the excess returned from the allocator to use the largest available capacity.

This type does not in anyway inspect the memory that it manages. When dropped it *will*
free its memory, but it *won't* try to drop its contents. It is up to the user of `RawVec`
to handle the actual things *stored* inside of a `RawVec`.

Note that the excess of a zero-sized types is always infinite, so `capacity()` always returns
`usize::MAX`. This means that you need to be careful when round-tripping this type with a
`Box<[T]>`, since `capacity()` won't yield the length.

#### Implementations

- <span id="rawvec-new"></span>`const fn new() -> Self`

  Creates the biggest possible `RawVec` (on the system heap)

  without allocating. If `T` has positive size, then this makes a

  `RawVec` with capacity `0`. If `T` is zero-sized, then it makes a

  `RawVec` with capacity `usize::MAX`. Useful for implementing

  delayed allocation.

- <span id="rawvec-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Creates a `RawVec` (on the system heap) with exactly the

  capacity and alignment requirements for a `[T; capacity]`. This is

  equivalent to calling `RawVec::new` when `capacity` is `0` or `T` is

  zero-sized. Note that if `T` is zero-sized this means you will

  *not* get a `RawVec` with the requested capacity.

  

  # Panics

  

  Panics if the requested capacity exceeds `isize::MAX` bytes.

  

  # Aborts

  

  Aborts on OOM.

- <span id="rawvec-with-capacity-zeroed"></span>`fn with_capacity_zeroed(capacity: usize) -> Self`

  Like `with_capacity`, but guarantees the buffer is zeroed.

#### Trait Implementations

##### `impl<T, A: Allocator> Drop for RawVec<T, A>`

- <span id="rawvec-drop"></span>`fn drop(&mut self)`

  Frees the memory owned by the `RawVec` *without* trying to drop its contents.

##### `impl<T, A> Send for RawVec<T, A>`

##### `impl<T, A> Sync for RawVec<T, A>`

## Enums

### `TryReserveErrorKind`

```rust
enum TryReserveErrorKind {
    CapacityOverflow,
    AllocError {
        layout: super::alloc::Layout,
    },
}
```

Details of the allocation that caused a `TryReserveError`

#### Variants

- **`CapacityOverflow`**

  Error due to the computed capacity exceeding the collection's maximum
  (usually `isize::MAX` bytes).

- **`AllocError`**

  The memory allocator returned an error

#### Trait Implementations

##### `impl Clone for TryReserveErrorKind`

- <span id="tryreserveerrorkind-clone"></span>`fn clone(&self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](#tryreserveerrorkind)

##### `impl Debug for TryReserveErrorKind`

- <span id="tryreserveerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryReserveErrorKind`

##### `impl PartialEq for TryReserveErrorKind`

- <span id="tryreserveerrorkind-partialeq-eq"></span>`fn eq(&self, other: &TryReserveErrorKind) -> bool` — [`TryReserveErrorKind`](#tryreserveerrorkind)

##### `impl StructuralPartialEq for TryReserveErrorKind`

### `AllocInit`

```rust
enum AllocInit {
    Uninitialized,
    Zeroed,
}
```

#### Variants

- **`Uninitialized`**

  The contents of the new memory are uninitialized.

- **`Zeroed`**

  The new memory is guaranteed to be zeroed.

## Functions

### `finish_grow`

```rust
fn finish_grow<A>(new_layout: Result<super::alloc::Layout, core::alloc::LayoutError>, current_memory: Option<(core::ptr::NonNull<u8>, super::alloc::Layout)>, alloc: &mut A) -> Result<core::ptr::NonNull<[u8]>, TryReserveError>
where
    A: Allocator
```

### `handle_reserve`

```rust
fn handle_reserve(result: Result<(), TryReserveError>)
```

### `alloc_guard`

```rust
fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError>
```

### `capacity_overflow`

```rust
fn capacity_overflow() -> never
```

