*[sel4_panicking](../../index.md) / [strategy](../index.md) / [unwind](index.md)*

---

# Module `unwind`

## Contents

- [Modules](#modules)
  - [`with_alloc`](#with-alloc)
- [Structs](#structs)
  - [`DropGuard`](#dropguard)
  - [`RustPanic`](#rustpanic)
  - [`ExceptionImpl`](#exceptionimpl)
- [Functions](#functions)
  - [`begin_panic`](#begin-panic)
  - [`catch_unwind`](#catch-unwind)
  - [`drop_panic`](#drop-panic)
  - [`foreign_exception`](#foreign-exception)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`with_alloc`](#with-alloc) | mod |  |
| [`DropGuard`](#dropguard) | struct |  |
| [`RustPanic`](#rustpanic) | struct |  |
| [`ExceptionImpl`](#exceptionimpl) | struct |  |
| [`begin_panic`](#begin-panic) | fn |  |
| [`catch_unwind`](#catch-unwind) | fn |  |
| [`drop_panic`](#drop-panic) | fn |  |
| [`foreign_exception`](#foreign-exception) | fn |  |

## Modules

- [`with_alloc`](with_alloc/index.md)

## Structs

### `DropGuard`

```rust
struct DropGuard;
```

#### Trait Implementations

##### `impl Drop for DropGuard`

- <span id="dropguard-drop"></span>`fn drop(&mut self)`

### `RustPanic`

```rust
struct RustPanic {
    drop_guard: DropGuard,
}
```

#### Implementations

- <span id="rustpanic-const-exception-class"></span>`const EXCEPTION_CLASS: [u8; 8]`

- <span id="rustpanic-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Exception for super::RustPanic`

- <span id="superrustpanic-exception-const-class"></span>`const CLASS: [u8; 8]`

- <span id="superrustpanic-exception-wrap"></span>`fn wrap(this: Self) -> *mut UnwindException`

- <span id="superrustpanic-exception-unwrap"></span>`unsafe fn unwrap(ex: *mut UnwindException) -> Self`

### `ExceptionImpl`

```rust
struct ExceptionImpl {
    exception: core::mem::MaybeUninit<UnwindException>,
    canary: *const u8,
    payload: RustPanic,
}
```

#### Implementations

- <span id="exceptionimpl-new"></span>`fn new(payload: RustPanic) -> Self` — [`RustPanic`](#rustpanic)

- <span id="exceptionimpl-check-cast"></span>`fn check_cast(ex: *mut UnwindException) -> Option<*mut Self>`

## Functions

### `begin_panic`

```rust
fn begin_panic() -> core::ffi::c_int
```

### `catch_unwind`

```rust
fn catch_unwind<R, F: FnOnce() -> R + UnwindSafe>(f: F) -> Result<R, ()>
```

### `drop_panic`

```rust
fn drop_panic() -> never
```

### `foreign_exception`

```rust
fn foreign_exception() -> never
```

