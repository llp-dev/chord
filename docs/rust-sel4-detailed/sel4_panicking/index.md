# Crate `sel4_panicking`

## Contents

- [Modules](#modules)
  - [`count`](#count)
  - [`hook`](#hook)
  - [`strategy`](#strategy)
- [Functions](#functions)
  - [`set_hook`](#set-hook)
  - [`panic`](#panic)
  - [`catch_unwind`](#catch-unwind)
  - [`abort_unwind`](#abort-unwind)
- [Type Aliases](#type-aliases)
  - [`PanicHook`](#panichook)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`count`](#count) | mod |  |
| [`hook`](#hook) | mod |  |
| [`strategy`](#strategy) | mod |  |
| [`set_hook`](#set-hook) | fn |  |
| [`panic`](#panic) | fn |  |
| [`catch_unwind`](#catch-unwind) | fn | Like `std::panic::catch_unwind`. |
| [`abort_unwind`](#abort-unwind) | fn | Like the unstable `core::panic::abort_unwind` |
| [`PanicHook`](#panichook) | type |  |

## Modules

- [`count`](count/index.md)
- [`hook`](hook/index.md)
- [`strategy`](strategy/index.md)

## Functions

### `set_hook`

```rust
fn set_hook(hook: PanicHook)
```

Like `std::panic::set_hook`.

### `panic`

```rust
fn panic(info: &core::panic::PanicInfo<'_>) -> never
```

### `catch_unwind`

```rust
fn catch_unwind<R, F: FnOnce() -> R + UnwindSafe>(f: F) -> Result<R, ()>
```

Like `std::panic::catch_unwind`.

### `abort_unwind`

```rust
fn abort_unwind<F, R>(f: F) -> R
where
    F: FnOnce() -> R
```

Like the unstable `core::panic::abort_unwind`

## Type Aliases

### `PanicHook`

```rust
type PanicHook = &'static dyn Fn(&core::panic::PanicInfo<'_>) + Send + Sync;
```

Type for panic hooks.

See [`set_hook`](hook/index.md).

