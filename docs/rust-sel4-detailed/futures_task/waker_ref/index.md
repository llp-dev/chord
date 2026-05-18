*[futures_task](../index.md) / [waker_ref](index.md)*

---

# Module `waker_ref`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WakerRef`](#wakerref) | struct | A [`Waker`] that is only valid for a given lifetime. |
| [`waker_ref`](#waker-ref) | fn | Creates a reference to a [`Waker`] from a reference to `Arc<impl ArcWake>`. |

## Structs

### `WakerRef<'a>`

```rust
struct WakerRef<'a> {
    waker: core::mem::ManuallyDrop<core::task::Waker>,
    _marker: core::marker::PhantomData<&'a ()>,
}
```

A [`Waker`](../index.md) that is only valid for a given lifetime.

Note: this type implements [`Deref<Target = Waker>`](std::ops::Deref),
so it can be used to get a `&Waker`.

#### Implementations

- <span id="wakerref-new"></span>`fn new(waker: &'a Waker) -> Self` — [`Waker`](../index.md#waker)

  Create a new [`WakerRef`](#wakerref) from a [`Waker`](../index.md) reference.

- <span id="wakerref-new-unowned"></span>`fn new_unowned(waker: ManuallyDrop<Waker>) -> Self` — [`Waker`](../index.md#waker)

  Create a new [`WakerRef`](#wakerref) from a [`Waker`](../index.md) that must not be dropped.

  

  Note: this if for rare cases where the caller created a [`Waker`](../index.md) in

  an unsafe way (that will be valid only for a lifetime to be determined

  by the caller), and the [`Waker`](../index.md) doesn't need to or must not be

  destroyed.

#### Trait Implementations

##### `impl Debug for WakerRef<'a>`

- <span id="wakerref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for WakerRef<'_>`

- <span id="wakerref-deref-type-target"></span>`type Target = Waker`

- <span id="wakerref-deref"></span>`fn deref(&self) -> &Waker` — [`Waker`](../index.md#waker)

##### `impl Receiver for WakerRef<'a>`

- <span id="wakerref-receiver-type-target"></span>`type Target = T`

## Functions

### `waker_ref`

```rust
fn waker_ref<W>(wake: &alloc::sync::Arc<W>) -> WakerRef<'_>
where
    W: ArcWake + 'static
```

Creates a reference to a [`Waker`](../index.md) from a reference to `Arc<impl ArcWake>`.

The resulting [`Waker`](../index.md) will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

