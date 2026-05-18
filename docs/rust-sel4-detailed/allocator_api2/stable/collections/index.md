*[allocator_api2](../../index.md) / [stable](../index.md) / [collections](index.md)*

---

# Module `collections`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReserveError`](#tryreserveerror) | struct |  |
| [`TryReserveErrorKind`](#tryreserveerrorkind) | enum |  |

## Structs

### `TryReserveError`

```rust
struct TryReserveError {
    kind: TryReserveErrorKind,
}
```

The error type for `try_reserve` methods.

#### Implementations

- <span id="tryreserveerror-kind"></span>`fn kind(&self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](../raw_vec/index.md#tryreserveerrorkind)

  Details about the allocation that caused the error

#### Trait Implementations

##### `impl Clone for TryReserveError`

- <span id="tryreserveerror-clone"></span>`fn clone(&self) -> TryReserveError` — [`TryReserveError`](../raw_vec/index.md#tryreserveerror)

##### `impl Debug for TryReserveError`

- <span id="tryreserveerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryReserveError`

- <span id="tryreserveerror-display-fmt"></span>`fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl Eq for TryReserveError`

##### `impl PartialEq for TryReserveError`

- <span id="tryreserveerror-partialeq-eq"></span>`fn eq(&self, other: &TryReserveError) -> bool` — [`TryReserveError`](../raw_vec/index.md#tryreserveerror)

##### `impl StructuralPartialEq for TryReserveError`

##### `impl ToString for TryReserveError`

- <span id="tryreserveerror-tostring-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="tryreserveerrorkind-clone"></span>`fn clone(&self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](../raw_vec/index.md#tryreserveerrorkind)

##### `impl Debug for TryReserveErrorKind`

- <span id="tryreserveerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryReserveErrorKind`

##### `impl PartialEq for TryReserveErrorKind`

- <span id="tryreserveerrorkind-partialeq-eq"></span>`fn eq(&self, other: &TryReserveErrorKind) -> bool` — [`TryReserveErrorKind`](../raw_vec/index.md#tryreserveerrorkind)

##### `impl StructuralPartialEq for TryReserveErrorKind`

