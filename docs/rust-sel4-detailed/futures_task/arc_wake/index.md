*[futures_task](../index.md) / [arc_wake](index.md)*

---

# Module `arc_wake`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArcWake`](#arcwake) | trait | A way of waking up a specific task. |

## Traits

### `ArcWake`

```rust
trait ArcWake: Send + Sync { ... }
```

A way of waking up a specific task.

By implementing this trait, types that are expected to be wrapped in an `Arc`
can be converted into [`Waker`](../../futures_core/task/index.md) objects.
Those Wakers can be used to signal executors that a task it owns
is ready to be `poll`ed again.

Currently, there are two ways to convert `ArcWake` into [`Waker`](../../futures_core/task/index.md):

* [`waker`](super::waker()) converts `Arc<impl ArcWake>` into [`Waker`](../../futures_core/task/index.md).
* [`waker_ref`](super::waker_ref()) converts `&Arc<impl ArcWake>` into [`WakerRef`](../waker_ref/index.md) that
  provides access to a [`&Waker`][`Waker`](../../futures_core/task/index.md).



#### Required Methods

- `fn wake_by_ref(arc_self: &Arc<Self>)`

  Indicates that the associated task is ready to make progress and should

#### Provided Methods

- `fn wake(self: Arc<Self>)`

  Indicates that the associated task is ready to make progress and should

