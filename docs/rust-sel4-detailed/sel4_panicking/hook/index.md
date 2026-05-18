*[sel4_panicking](../index.md) / [hook](index.md)*

---

# Module `hook`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`set_hook`](#set-hook) | fn | Like `std::panic::set_hook`. |
| [`get_hook`](#get-hook) | fn |  |
| [`default_hook`](#default-hook) | fn |  |
| [`PanicHook`](#panichook) | type | Type for panic hooks. |

## Functions

### `set_hook`

```rust
fn set_hook(hook: PanicHook)
```

Like `std::panic::set_hook`.

### `get_hook`

```rust
fn get_hook() -> &'static PanicHook
```

### `default_hook`

```rust
fn default_hook(info: &core::panic::PanicInfo<'_>)
```

## Type Aliases

### `PanicHook`

```rust
type PanicHook = &'static dyn Fn(&core::panic::PanicInfo<'_>) + Send + Sync;
```

Type for panic hooks.

See [`set_hook`](#set-hook).

