*[futures_task](../index.md) / [waker](index.md)*

---

# Module `waker`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`waker_vtable`](#waker-vtable) | fn |  |
| [`waker`](#waker) | fn | Creates a [`Waker`] from an `Arc<impl ArcWake>`. |
| [`increase_refcount`](#increase-refcount) | fn |  |
| [`clone_arc_raw`](#clone-arc-raw) | fn |  |
| [`wake_arc_raw`](#wake-arc-raw) | fn |  |
| [`wake_by_ref_arc_raw`](#wake-by-ref-arc-raw) | fn |  |
| [`drop_arc_raw`](#drop-arc-raw) | fn |  |

## Functions

### `waker_vtable`

```rust
fn waker_vtable<W: ArcWake + 'static>() -> &'static core::task::RawWakerVTable
```

### `waker`

```rust
fn waker<W>(wake: alloc::sync::Arc<W>) -> core::task::Waker
where
    W: ArcWake + 'static
```

Creates a [`Waker`](../index.md) from an `Arc<impl ArcWake>`.

The returned [`Waker`](../index.md) will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

### `increase_refcount`

```rust
unsafe fn increase_refcount<T: ArcWake + 'static>(data: *const ())
```

### `clone_arc_raw`

```rust
unsafe fn clone_arc_raw<T: ArcWake + 'static>(data: *const ()) -> core::task::RawWaker
```

### `wake_arc_raw`

```rust
unsafe fn wake_arc_raw<T: ArcWake + 'static>(data: *const ())
```

### `wake_by_ref_arc_raw`

```rust
unsafe fn wake_by_ref_arc_raw<T: ArcWake + 'static>(data: *const ())
```

### `drop_arc_raw`

```rust
unsafe fn drop_arc_raw<T: ArcWake + 'static>(data: *const ())
```

