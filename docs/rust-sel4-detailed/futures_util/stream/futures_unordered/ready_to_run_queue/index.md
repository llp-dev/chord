*[futures_util](../../../index.md) / [stream](../../index.md) / [futures_unordered](../index.md) / [ready_to_run_queue](index.md)*

---

# Module `ready_to_run_queue`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReadyToRunQueue`](#readytorunqueue) | struct |  |
| [`Dequeue`](#dequeue) | enum |  |

## Structs

### `ReadyToRunQueue<Fut>`

```rust
struct ReadyToRunQueue<Fut> {
    waker: crate::task::AtomicWaker,
    head: core::sync::atomic::AtomicPtr<super::task::Task<Fut>>,
    tail: core::cell::UnsafeCell<*const super::task::Task<Fut>>,
    stub: alloc::sync::Arc<super::task::Task<Fut>>,
}
```

#### Implementations

- <span id="readytorunqueue-enqueue"></span>`fn enqueue(&self, task: *const Task<Fut>)` — [`Task`](../task/index.md#task)

  The enqueue function from the 1024cores intrusive MPSC queue algorithm.

- <span id="readytorunqueue-dequeue"></span>`unsafe fn dequeue(&self) -> Dequeue<Fut>` — [`Dequeue`](#dequeue)

  The dequeue function from the 1024cores intrusive MPSC queue algorithm

  

  Note that this is unsafe as it required mutual exclusion (only one

  thread can call this) to be guaranteed elsewhere.

- <span id="readytorunqueue-stub"></span>`fn stub(&self) -> *const Task<Fut>` — [`Task`](../task/index.md#task)

#### Trait Implementations

##### `impl<Fut> Drop for ReadyToRunQueue<Fut>`

- <span id="readytorunqueue-drop"></span>`fn drop(&mut self)`

## Enums

### `Dequeue<Fut>`

```rust
enum Dequeue<Fut> {
    Data(*const super::task::Task<Fut>),
    Empty,
    Inconsistent,
}
```

