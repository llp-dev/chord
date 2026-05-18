*[futures_util](../../../index.md) / [stream](../../index.md) / [futures_unordered](../index.md) / [task](index.md)*

---

# Module `task`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`waker_ref`](#waker-ref) | mod |  |
| [`Task`](#task) | struct |  |

## Modules

- [`waker_ref`](waker_ref/index.md)

## Structs

### `Task<Fut>`

```rust
struct Task<Fut> {
    future: core::cell::UnsafeCell<Option<Fut>>,
    next_all: core::sync::atomic::AtomicPtr<Task<Fut>>,
    prev_all: core::cell::UnsafeCell<*const Task<Fut>>,
    len_all: core::cell::UnsafeCell<usize>,
    next_ready_to_run: core::sync::atomic::AtomicPtr<Task<Fut>>,
    ready_to_run_queue: alloc::sync::Weak<self::ready_to_run_queue::ReadyToRunQueue<Fut>>,
    queued: core::sync::atomic::AtomicBool,
    woken: core::sync::atomic::AtomicBool,
}
```

#### Implementations

- <span id="task-waker-ref"></span>`unsafe fn waker_ref(this: &Arc<Self>) -> waker_ref::WakerRef<'_>` — [`WakerRef`](waker_ref/index.md#wakerref)

  Returns a waker reference for this task without cloning the Arc.

- <span id="task-spin-next-all"></span>`fn spin_next_all(&self, pending_next_all: *mut Self, ordering: Ordering) -> *const Self`

  Spins until `next_all` is no longer set to `pending_next_all`.

  

  The temporary `pending_next_all` value is typically overwritten fairly

  quickly after a node is inserted into the list of all futures, so this

  should rarely spin much.

  

  When it returns, the correct `next_all` value is returned.

  

  `Relaxed` or `Acquire` ordering can be used. `Acquire` ordering must be

  used before `len_all` can be safely read.

#### Trait Implementations

##### `impl<Fut> ArcWake for Task<Fut>`

- <span id="task-arcwake-wake-by-ref"></span>`fn wake_by_ref(arc_self: &Arc<Self>)`

##### `impl<Fut> Drop for Task<Fut>`

- <span id="task-drop"></span>`fn drop(&mut self)`

##### `impl<Fut> Send for Task<Fut>`

##### `impl<Fut> Sync for Task<Fut>`

