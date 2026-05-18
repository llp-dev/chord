**futures_task > waker**

# Module: waker

## Contents

**Functions**

- [`waker`](#waker) - Creates a [`Waker`] from an `Arc<impl ArcWake>`.

---

## futures_task::waker::waker

*Function*

Creates a [`Waker`] from an `Arc<impl ArcWake>`.

The returned [`Waker`] will call
[`ArcWake.wake()`](ArcWake::wake) if awoken.

```rust
fn waker<W>(wake: alloc::sync::Arc<W>) -> core::task::Waker
```



