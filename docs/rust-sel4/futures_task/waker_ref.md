**futures_task > waker_ref**

# Module: waker_ref

## Contents

**Structs**

- [`WakerRef`](#wakerref) - A [`Waker`] that is only valid for a given lifetime.

**Functions**

- [`waker_ref`](#waker_ref) - Creates a reference to a [`Waker`] from a reference to `Arc<impl ArcWake>`.

---

## futures_task::waker_ref::WakerRef

*Struct*

A [`Waker`] that is only valid for a given lifetime.

Note: this type implements [`Deref<Target = Waker>`](std::ops::Deref),
so it can be used to get a `&Waker`.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(waker: &'a Waker) -> Self` - Create a new [`WakerRef`] from a [`Waker`] reference.
- `fn new_unowned(waker: ManuallyDrop<Waker>) -> Self` - Create a new [`WakerRef`] from a [`Waker`] that must not be dropped.

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &Waker`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_task::waker_ref::waker_ref

*Function*

Creates a reference to a [`Waker`] from a reference to `Arc<impl ArcWake>`.

The resulting [`Waker`] will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

```rust
fn waker_ref<W>(wake: &alloc::sync::Arc<W>) -> WakerRef
```



