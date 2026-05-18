**heapless > mpmc**

# Module: mpmc

## Contents

**Structs**

- [`QueueInner`](#queueinner) - Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`].

**Type Aliases**

- [`Queue`](#queue) - A statically allocated multi-producer, multi-consumer queue with a capacity of `N` elements.
- [`QueueView`](#queueview) - A [`Queue`] with dynamic capacity.

---

## heapless::mpmc::Queue

*Type Alias*: `QueueInner<T, crate::storage::OwnedStorage<N>>`

A statically allocated multi-producer, multi-consumer queue with a capacity of `N` elements.

<div class="warning">

`N` must be a power of 2.

</div>

The maximum value of `N` is 128 if the `mpmc_large` feature is not enabled.



## heapless::mpmc::QueueInner

*Struct*

Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`].

In most cases you should use [`Queue`] or [`QueueView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- T
- S

**Methods:**

- `fn new() -> Self` - Creates an empty queue.
- `fn capacity(self: &Self) -> usize` - Returns the maximum number of elements the queue can hold.
- `fn as_view(self: &Self) -> &QueueView<T>` - Get a reference to the `Queue`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut QueueView<T>` - Get a mutable reference to the `Queue`, erasing the `N` const-generic.
- `fn dequeue(self: &Self) -> Option<T>` - Returns the item in the front of the queue, or `None` if the queue is empty.
- `fn enqueue(self: &Self, item: T) -> Result<(), T>` - Adds an `item` to the end of the queue.

**Traits:** Sync

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



## heapless::mpmc::QueueView

*Type Alias*: `QueueInner<T, crate::storage::ViewStorage>`

A [`Queue`] with dynamic capacity.

[`Queue`] coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by reference.



