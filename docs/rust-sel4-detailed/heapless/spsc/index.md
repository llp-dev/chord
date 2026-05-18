*[heapless](../index.md) / [spsc](index.md)*

---

# Module `spsc`

A fixed capacity single-producer, single-consumer (SPSC) lock-free queue.

*Note:* This module requires atomic load and store instructions. On
targets where they're not natively available, they are emulated by the
[`portable-atomic`](https://crates.io/crates/portable-atomic) crate.

# Examples

[`Queue`](#queue) can be used as a plain queue:

```rust
use heapless::spsc::Queue;

let mut queue: Queue<u8, 4> = Queue::new();

assert_eq!(queue.enqueue(0), Ok(()));
assert_eq!(queue.enqueue(1), Ok(()));
assert_eq!(queue.enqueue(2), Ok(()));
assert_eq!(queue.enqueue(3), Err(3)); // Queue is full.

assert_eq!(queue.dequeue(), Some(0));
```

`Queue::split` can be used to split the queue into a [`Producer`](#producer)/[`Consumer`](#consumer) pair.

After splitting a `&'static mut Queue`, the resulting [`Producer`](#producer) and [`Consumer`](#consumer)
can be moved into different execution contexts, e.g. threads, interrupt handlers, etc.


```rust
use heapless::spsc::{Producer, Queue};

#[derive(Debug)]
enum Event {
    A,
    B,
}

fn main() {
    let queue: &'static mut Queue<Event, 4> = {
        static mut Q: Queue<Event, 4> = Queue::new();
        // SAFETY: `Q` is only accessible in this scope
        // and `main` is only called once.
        unsafe { &mut Q }
    };

    let (producer, mut consumer) = queue.split();

    // `producer` can be moved into `interrupt_handler` using a static mutex or the mechanism
    // provided by the concurrency framework you are using, e.g. a resource in RTIC.
  let mut producer = producer;
  interrupt_handler(&mut producer);

    loop {
        match consumer.dequeue() {
            Some(Event::A) => { /* .. */ }
            Some(Event::B) => { /* .. */ }
            None => { /* Sleep. */ }
        }
      break
    }
}

// This is a different execution context that can preempt `main`.
fn interrupt_handler(producer: &mut Producer<'static, Event>) {
  let condition = true;

    // ..

    if condition {
        producer.enqueue(Event::A).unwrap();
    } else {
        producer.enqueue(Event::B).unwrap();
    }

    // ..
}
```

# Benchmarks

Measured on an ARM Cortex-M3 core running at 8 MHz and with zero flash wait cycles, compiled with `-C opt-level=3`:

| Method                         | Time |
|:-------------------------------|-----:|
| `Producer::<u8, _>::enqueue()` |   16 |
| `Queue::<u8, _>::enqueue()`    |   14 |
| `Consumer::<u8, _>::dequeue()` |   15 |
| `Queue::<u8, _>::dequeue()`    |   12 |

- All execution times are in clock cycles (1 clock cycle = 125 ns).
- Execution time is *dependent* on `mem::size_of::<T>()`, as both operations include
  `ptr::read::<T>()` or `ptr::write::<T>()` in their successful path.
- The numbers reported correspond to the successful path, i.e. `dequeue` returning `Some`
  and `enqueue` returning `Ok`.

# References

This is an implementation based on [https://www.codeproject.com/Articles/43510/Lock-Free-Single-Producer-Single-Consumer-Circular](
  https://web.archive.org/web/20250117082625/https://www.codeproject.com/Articles/43510/Lock-Free-Single-Producer-Single-Consumer-Circular
).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`QueueInner`](#queueinner) | struct | Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`]. |
| [`Iter`](#iter) | struct | An iterator over the items of a queue. |
| [`IterMut`](#itermut) | struct | An iterator over the items of a queue. |
| [`Consumer`](#consumer) | struct | A consumer; it can dequeue items from the queue. |
| [`Producer`](#producer) | struct | A producer; it can enqueue items into the queue. |
| [`Queue`](#queue) | type | A statically allocated single-producer, single-consumer queue with a capacity of `N - 1` elements. |
| [`QueueView`](#queueview) | type | A [`Queue`] with dynamic capacity. |

## Structs

### `QueueInner<T, S: Storage>`

```rust
struct QueueInner<T, S: Storage> {
    head: atomic::AtomicUsize,
    tail: atomic::AtomicUsize,
    buffer: <S as >::Buffer,
}
```

Base struct for [`Queue`](#queue) and [`QueueView`](#queueview), generic over the [`Storage`](../storage/index.md).

In most cases you should use [`Queue`](#queue) or [`QueueView`](#queueview) directly. Only use this
struct if you want to write code that's generic over both.

#### Implementations

- <span id="queueinner-new"></span>`const fn new() -> Self`

  Creates an empty queue.

- <span id="queueinner-as-view-private"></span>`fn as_view_private(&self) -> &QueueView<T>` — [`QueueView`](#queueview)

  Used in `Storage` implementation

- <span id="queueinner-as-mut-view-private"></span>`fn as_mut_view_private(&mut self) -> &mut QueueView<T>` — [`QueueView`](#queueview)

  Used in `Storage` implementation

#### Trait Implementations

##### `impl<T, S> Debug for QueueInner<T, S>`

- <span id="queueinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S: Storage> Drop for QueueInner<T, S>`

- <span id="queueinner-drop"></span>`fn drop(&mut self)`

##### `impl<T, S: Storage> Eq for QueueInner<T, S>`

##### `impl<T, S> Hash for QueueInner<T, S>`

- <span id="queueinner-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<T, S: Storage> IntoIterator for &'a QueueInner<T, S>`

- <span id="a-queueinner-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-queueinner-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-queueinner-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, S, S2> PartialEq for QueueInner<T, S>`

- <span id="queueinner-partialeq-eq"></span>`fn eq(&self, other: &QueueInner<T, S2>) -> bool` — [`QueueInner`](#queueinner)

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    rb: &'a QueueView<T>,
    index: usize,
    len: usize,
}
```

An iterator over the items of a queue.

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> DoubleEndedIterator for Iter<'_, T>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl IntoIterator for Iter<'a, T>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Iter<'a, T>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a T`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    rb: &'a QueueView<T>,
    index: usize,
    len: usize,
}
```

An iterator over the items of a queue.

#### Trait Implementations

##### `impl<T> DoubleEndedIterator for IterMut<'_, T>`

- <span id="itermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl IntoIterator for IterMut<'a, T>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterMut<'a, T>`

- <span id="itermut-iterator-type-item"></span>`type Item = &'a mut T`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Consumer<'a, T>`

```rust
struct Consumer<'a, T> {
    rb: &'a QueueView<T>,
}
```

A consumer; it can dequeue items from the queue.

**Note:** The consumer semantically owns the `head` pointer of the queue.

#### Implementations

- <span id="consumer-dequeue"></span>`fn dequeue(&mut self) -> Option<T>`

  Returns the item in the front of the queue, or `None` if the queue is empty.

- <span id="consumer-dequeue-unchecked"></span>`unsafe fn dequeue_unchecked(&mut self) -> T`

  Returns the item in the front of the queue, without checking if there are elements in the

  queue.

  

  # Safety

  

  See `Queue::dequeue_unchecked`.

- <span id="consumer-ready"></span>`fn ready(&self) -> bool`

  Returns if there are any items to dequeue. When this returns `true`, at least the

  first subsequent dequeue will succeed.

- <span id="consumer-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in the queue.

- <span id="consumer-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns whether the queue is empty.

  

  # Examples

  

  ```rust

  use heapless::spsc::Queue;

  

  let mut queue: Queue<u8, 235> = Queue::new();

  let (mut producer, mut consumer) = queue.split();

  assert!(consumer.is_empty());

  ```

- <span id="consumer-capacity"></span>`fn capacity(&self) -> usize`

  Returns the maximum number of elements the queue can hold.

- <span id="consumer-peek"></span>`fn peek(&self) -> Option<&T>`

  Returns the item in the front of the queue without dequeuing, or `None` if the queue is

  empty.

  

  # Examples

  

  ```rust

  use heapless::spsc::Queue;

  

  let mut queue: Queue<u8, 235> = Queue::new();

  let (mut producer, mut consumer) = queue.split();

  assert_eq!(None, consumer.peek());

  producer.enqueue(1);

  assert_eq!(Some(&1), consumer.peek());

  assert_eq!(Some(1), consumer.dequeue());

  assert_eq!(None, consumer.peek());

  ```

#### Trait Implementations

##### `impl<T> Send for Consumer<'_, T>`

### `Producer<'a, T>`

```rust
struct Producer<'a, T> {
    rb: &'a QueueView<T>,
}
```

A producer; it can enqueue items into the queue.

**Note:** The producer semantically owns the `tail` pointer of the queue.

#### Implementations

- <span id="producer-enqueue"></span>`fn enqueue(&mut self, item: T) -> Result<(), T>`

  Adds an `item` to the end of the queue, returns back the `item` if the queue is full.

- <span id="producer-enqueue-unchecked"></span>`unsafe fn enqueue_unchecked(&mut self, item: T)`

  Adds an `item` to the end of the queue, without checking if the queue is full.

  

  # Safety

  

  See `Queue::enqueue_unchecked`.

- <span id="producer-ready"></span>`fn ready(&self) -> bool`

  Returns if there is any space to enqueue a new item. When this returns true, at

  least the first subsequent enqueue will succeed.

- <span id="producer-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in the queue.

- <span id="producer-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns whether the queue is empty.

  

  # Examples

  

  ```rust

  use heapless::spsc::Queue;

  

  let mut queue: Queue<u8, 235> = Queue::new();

  let (mut producer, mut consumer) = queue.split();

  assert!(producer.is_empty());

  ```

- <span id="producer-capacity"></span>`fn capacity(&self) -> usize`

  Returns the maximum number of elements the queue can hold.

#### Trait Implementations

##### `impl<T> Send for Producer<'_, T>`

## Type Aliases

### `Queue<T, const N: usize>`

```rust
type Queue<T, const N: usize> = QueueInner<T, crate::storage::OwnedStorage<N>>;
```

A statically allocated single-producer, single-consumer queue with a capacity of `N - 1` elements.

<div class="warning">

To get better performance, use a value for `N` that is a power of 2.

</div>

You will likely want to use [`split`](QueueInner::split) to create a producer-consumer pair.

### `QueueView<T>`

```rust
type QueueView<T> = QueueInner<T, crate::storage::ViewStorage>;
```

A [`Queue`](#queue) with dynamic capacity.

[`Queue`](#queue) coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by reference.

