# Crate `sel4_alloca`

## Contents

- [Enums](#enums)
  - [`ReserveOnStackContArg`](#reserveonstackcontarg)
- [Functions](#functions)
  - [`__sel4_alloca__reserve_on_stack`](#sel4-alloca-reserve-on-stack)
  - [`with_alloca_bytes`](#with-alloca-bytes)
  - [`with_alloca`](#with-alloca)
  - [`with_alloca_slice`](#with-alloca-slice)
  - [`with_alloca_ptr`](#with-alloca-ptr)
  - [`reserve_on_stack`](#reserve-on-stack)
- [Type Aliases](#type-aliases)
  - [`ReserveOnStackContFn`](#reserveonstackcontfn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReserveOnStackContArg`](#reserveonstackcontarg) | enum |  |
| [`__sel4_alloca__reserve_on_stack`](#sel4-alloca-reserve-on-stack) | fn |  |
| [`with_alloca_bytes`](#with-alloca-bytes) | fn |  |
| [`with_alloca`](#with-alloca) | fn |  |
| [`with_alloca_slice`](#with-alloca-slice) | fn |  |
| [`with_alloca_ptr`](#with-alloca-ptr) | fn |  |
| [`reserve_on_stack`](#reserve-on-stack) | fn |  |
| [`ReserveOnStackContFn`](#reserveonstackcontfn) | type |  |

## Enums

### `ReserveOnStackContArg`

```rust
enum ReserveOnStackContArg {
}
```

## Functions

### `__sel4_alloca__reserve_on_stack`

```rust
unsafe fn __sel4_alloca__reserve_on_stack(reservation_size: usize, reservation_align_down_mask: usize, cont_fn: fn(*mut u8, *mut ReserveOnStackContArg), cont_arg: *mut ReserveOnStackContArg)
```

### `with_alloca_bytes`

```rust
fn with_alloca_bytes<R, F: FnOnce(&mut [core::mem::MaybeUninit<u8>]) -> R>(layout: core::alloc::Layout, f: F) -> R
```

### `with_alloca`

```rust
fn with_alloca<R, T, F: FnOnce(&mut core::mem::MaybeUninit<T>) -> R>(f: F) -> R
```

### `with_alloca_slice`

```rust
fn with_alloca_slice<R, T, F: FnOnce(&mut [core::mem::MaybeUninit<T>]) -> R>(n: usize, f: F) -> R
```

### `with_alloca_ptr`

```rust
fn with_alloca_ptr<R, F: FnOnce(*mut u8) -> R>(layout: core::alloc::Layout, f: F) -> R
```

### `reserve_on_stack`

```rust
unsafe fn reserve_on_stack(layout: core::alloc::Layout, cont_fn: fn(*mut u8, *mut ReserveOnStackContArg), cont_arg: *mut ReserveOnStackContArg)
```

## Type Aliases

### `ReserveOnStackContFn`

```rust
type ReserveOnStackContFn = fn(*mut u8, *mut ReserveOnStackContArg);
```

