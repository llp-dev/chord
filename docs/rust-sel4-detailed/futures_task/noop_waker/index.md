*[futures_task](../index.md) / [noop_waker](index.md)*

---

# Module `noop_waker`

Utilities for creating zero-cost wakers that don't do anything.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`noop_clone`](#noop-clone) | fn |  |
| [`noop`](#noop) | fn |  |
| [`noop_raw_waker`](#noop-raw-waker) | fn |  |
| [`noop_waker`](#noop-waker) | fn | Create a new [`Waker`] which does nothing when `wake()` is called on it. |
| [`noop_waker_ref`](#noop-waker-ref) | fn | Get a static reference to a [`Waker`] which does nothing when `wake()` is called on it. |
| [`NOOP_WAKER_VTABLE`](#noop-waker-vtable) | const |  |

## Functions

### `noop_clone`

```rust
unsafe fn noop_clone(_data: *const ()) -> core::task::RawWaker
```

### `noop`

```rust
unsafe fn noop(_data: *const ())
```

### `noop_raw_waker`

```rust
const fn noop_raw_waker() -> core::task::RawWaker
```

### `noop_waker`

```rust
fn noop_waker() -> core::task::Waker
```

Create a new [`Waker`](../index.md) which does
nothing when `wake()` is called on it.

# Examples

```rust
use futures::task::noop_waker;
let waker = noop_waker();
waker.wake();
```

### `noop_waker_ref`

```rust
fn noop_waker_ref() -> &'static core::task::Waker
```

Get a static reference to a [`Waker`](../index.md) which
does nothing when `wake()` is called on it.

# Examples

```rust
use futures::task::noop_waker_ref;
let waker = noop_waker_ref();
waker.wake_by_ref();
```

## Constants

### `NOOP_WAKER_VTABLE`
```rust
const NOOP_WAKER_VTABLE: core::task::RawWakerVTable;
```

