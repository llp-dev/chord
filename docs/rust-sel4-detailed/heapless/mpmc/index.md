*[heapless](../index.md) / [mpmc](index.md)*

---

# Module `mpmc`

A fixed capacity multiple-producer, multiple-consumer (MPMC) lock-free queue.

**Note:** This module requires atomic compare-and-swap (CAS) instructions. On
targets where they're not natively available, they are emulated by the
[`portable-atomic`](https://crates.io/crates/portable-atomic) crate.

# Example

This queue can be constructed in `const` context. Placing it in a `static` variable lets *all*
contexts (interrupts/threads/`main`) safely enqueue and dequeue items.

```rust
use core::sync::atomic::{AtomicU8, Ordering};

use heapless::mpmc::Queue;

static Q: Queue<u8, 2> = Queue::new();

fn main() {
    // Configure systick interrupt.

    loop {
        if let Some(x) = Q.dequeue() {
            println!("{}", x);
        } else {
            // Wait for interrupt.
        }
      break
    }
}

fn systick() {
    static COUNT: AtomicU8 = AtomicU8::new(0);
    let count = COUNT.fetch_add(1, Ordering::SeqCst);

  let _ =
    Q.enqueue(count);
}
```

# Benchmark

Measured on an ARM Cortex-M3 core running at 8 MHz and with zero flash wait cycles, compiled with `-C opt-level=z`:

| Method                      | Time | N  |
|:----------------------------|-----:|---:|
| `Queue::<u8, 8>::enqueue()` |   34 |  0 |
| `Queue::<u8, 8>::enqueue()` |   52 |  1 |
| `Queue::<u8, 8>::enqueue()` |   69 |  2 |
| `Queue::<u8, 8>::dequeue()` |   35 |  0 |
| `Queue::<u8, 8>::dequeue()` |   53 |  1 |
| `Queue::<u8, 8>::dequeue()` |   71 |  2 |

- N denotes the number of interruptions. On Cortex-M, an interruption consists of an
  interrupt handler preempting the would-be atomic section of the `enqueue`/`dequeue`
  operation. Note that it does *not* matter if the higher priority handler uses the queue or
  not.
- All execution times are in clock cycles (1 clock cycle = 125 ns).
- Execution time is *dependent* on `mem::size_of::<T>()`, as both operations include
  `ptr::read::<T>()` or `ptr::write::<T>()` in their successful path.
- The numbers reported correspond to the successful path, i.e. `dequeue` returning `Some`
  and `enqueue` returning `Ok`.

# References

This is an implementation of Dmitry Vyukov's [bounded MPMC queue], minus the
cache padding.


## Contents

- [Structs](#structs)
  - [`QueueInner`](#queueinner)
  - [`Cell`](#cell)
- [Functions](#functions)
  - [`dequeue`](#dequeue)
  - [`enqueue`](#enqueue)
- [Type Aliases](#type-aliases)
  - [`AtomicTargetSize`](#atomictargetsize)
  - [`UintSize`](#uintsize)
  - [`IntSize`](#intsize)
  - [`Queue`](#queue)
  - [`QueueView`](#queueview)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`QueueInner`](#queueinner) | struct | Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`]. |
| [`Cell`](#cell) | struct |  |
| [`dequeue`](#dequeue) | fn |  |
| [`enqueue`](#enqueue) | fn |  |
| [`AtomicTargetSize`](#atomictargetsize) | type |  |
| [`UintSize`](#uintsize) | type |  |
| [`IntSize`](#intsize) | type |  |
| [`Queue`](#queue) | type | A statically allocated multi-producer, multi-consumer queue with a capacity of `N` elements. |
| [`QueueView`](#queueview) | type | A [`Queue`] with dynamic capacity. |

## Structs

### `QueueInner<T, S: Storage>`

```rust
struct QueueInner<T, S: Storage> {
    dequeue_pos: atomic::AtomicU8,
    enqueue_pos: atomic::AtomicU8,
    buffer: core::cell::UnsafeCell<<S as >::Buffer>,
}
```

Base struct for [`Queue`](#queue) and [`QueueView`](#queueview), generic over the [`Storage`](../storage/index.md).

In most cases you should use [`Queue`](#queue) or [`QueueView`](#queueview) directly. Only use this
struct if you want to write code that's generic over both.

#### Implementations

- <span id="queueinner-new"></span>`const fn new() -> Self`

  Creates an empty queue.

- <span id="queueinner-as-view-private"></span>`fn as_view_private(&self) -> &QueueView<T>` — [`QueueView`](#queueview)

  Used in `Storage` implementation.

- <span id="queueinner-as-view-mut-private"></span>`fn as_view_mut_private(&mut self) -> &mut QueueView<T>` — [`QueueView`](#queueview)

  Used in `Storage` implementation.

#### Trait Implementations

##### `impl<T, S: Storage> Drop for QueueInner<T, S>`

- <span id="queueinner-drop"></span>`fn drop(&mut self)`

##### `impl<T, S: Storage> Sync for QueueInner<T, S>`

### `Cell<T>`

```rust
struct Cell<T> {
    data: core::mem::MaybeUninit<T>,
    sequence: atomic::AtomicU8,
}
```

#### Implementations

- <span id="cell-new"></span>`const fn new(seq: usize) -> Self`

## Functions

### `dequeue`

```rust
unsafe fn dequeue<T>(buffer: *mut Cell<T>, dequeue_pos: &atomic::AtomicU8, mask: u8) -> Option<T>
```

### `enqueue`

```rust
unsafe fn enqueue<T>(buffer: *mut Cell<T>, enqueue_pos: &atomic::AtomicU8, mask: u8, item: T) -> Result<(), T>
```

## Type Aliases

### `AtomicTargetSize`

```rust
type AtomicTargetSize = atomic::AtomicU8;
```

### `UintSize`

```rust
type UintSize = u8;
```

### `IntSize`

```rust
type IntSize = i8;
```

### `Queue<T, const N: usize>`

```rust
type Queue<T, const N: usize> = QueueInner<T, crate::storage::OwnedStorage<N>>;
```

A statically allocated multi-producer, multi-consumer queue with a capacity of `N` elements.

<div class="warning">

`N` must be a power of 2.

</div>

The maximum value of `N` is 128 if the `mpmc_large` feature is not enabled.

### `QueueView<T>`

```rust
type QueueView<T> = QueueInner<T, crate::storage::ViewStorage>;
```

A [`Queue`](#queue) with dynamic capacity.

[`Queue`](#queue) coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by reference.

