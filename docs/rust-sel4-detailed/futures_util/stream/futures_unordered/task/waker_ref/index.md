*[futures_util](../../../../index.md) / [stream](../../../index.md) / [futures_unordered](../../index.md) / [task](../index.md) / [waker_ref](index.md)*

---

# Module `waker_ref`

## Contents

- [Structs](#structs)
  - [`WakerRef`](#wakerref)
- [Functions](#functions)
  - [`waker_ref`](#waker-ref)
  - [`waker_vtable`](#waker-vtable)
  - [`increase_refcount`](#increase-refcount)
  - [`clone_arc_raw`](#clone-arc-raw)
  - [`wake_arc_raw`](#wake-arc-raw)
  - [`wake_by_ref_arc_raw`](#wake-by-ref-arc-raw)
  - [`drop_arc_raw`](#drop-arc-raw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WakerRef`](#wakerref) | struct |  |
| [`waker_ref`](#waker-ref) | fn | Copy of `future_task::waker_ref` without `W: 'static` bound. |
| [`waker_vtable`](#waker-vtable) | fn |  |
| [`increase_refcount`](#increase-refcount) | fn |  |
| [`clone_arc_raw`](#clone-arc-raw) | fn |  |
| [`wake_arc_raw`](#wake-arc-raw) | fn |  |
| [`wake_by_ref_arc_raw`](#wake-by-ref-arc-raw) | fn |  |
| [`drop_arc_raw`](#drop-arc-raw) | fn |  |

## Structs

### `WakerRef<'a>`

```rust
struct WakerRef<'a> {
    waker: core::mem::ManuallyDrop<core::task::Waker>,
    _marker: core::marker::PhantomData<&'a ()>,
}
```

#### Implementations

- <span id="wakerref-new-unowned"></span>`fn new_unowned(waker: ManuallyDrop<Waker>) -> Self` — [`Waker`](../../../../task/index.md#waker)

#### Trait Implementations

##### `impl Deref for WakerRef<'_>`

- <span id="wakerref-deref-type-target"></span>`type Target = Waker`

- <span id="wakerref-deref"></span>`fn deref(&self) -> &Waker` — [`Waker`](../../../../task/index.md#waker)

##### `impl Receiver for WakerRef<'a>`

- <span id="wakerref-receiver-type-target"></span>`type Target = T`

## Functions

### `waker_ref`

```rust
unsafe fn waker_ref<W>(wake: &alloc::sync::Arc<W>) -> WakerRef<'_>
where
    W: ArcWake
```

Copy of `future_task::waker_ref` without `W: 'static` bound.

# Safety

The caller must guarantee that use-after-free will not occur.

### `waker_vtable`

```rust
fn waker_vtable<W: ArcWake>() -> &'static core::task::RawWakerVTable
```

### `increase_refcount`

```rust
unsafe fn increase_refcount<T: ArcWake>(data: *const ())
```

### `clone_arc_raw`

```rust
unsafe fn clone_arc_raw<T: ArcWake>(data: *const ()) -> core::task::RawWaker
```

### `wake_arc_raw`

```rust
unsafe fn wake_arc_raw<T: ArcWake>(data: *const ())
```

### `wake_by_ref_arc_raw`

```rust
unsafe fn wake_by_ref_arc_raw<T: ArcWake>(data: *const ())
```

### `drop_arc_raw`

```rust
unsafe fn drop_arc_raw<T: ArcWake>(data: *const ())
```

