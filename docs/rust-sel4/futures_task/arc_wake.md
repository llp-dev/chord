**futures_task > arc_wake**

# Module: arc_wake

## Contents

**Traits**

- [`ArcWake`](#arcwake) - A way of waking up a specific task.

---

## futures_task::arc_wake::ArcWake

*Trait*

A way of waking up a specific task.

By implementing this trait, types that are expected to be wrapped in an `Arc`
can be converted into [`Waker`] objects.
Those Wakers can be used to signal executors that a task it owns
is ready to be `poll`ed again.

Currently, there are two ways to convert `ArcWake` into [`Waker`]:

* [`waker`](super::waker()) converts `Arc<impl ArcWake>` into [`Waker`].
* [`waker_ref`](super::waker_ref()) converts `&Arc<impl ArcWake>` into [`WakerRef`] that
  provides access to a [`&Waker`][`Waker`].

[`Waker`]: std::task::Waker
[`WakerRef`]: super::WakerRef

**Methods:**

- `wake`: Indicates that the associated task is ready to make progress and should
- `wake_by_ref`: Indicates that the associated task is ready to make progress and should



