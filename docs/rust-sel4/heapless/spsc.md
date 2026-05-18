**heapless > spsc**

# Module: spsc

## Contents

**Structs**

- [`Consumer`](#consumer) - A consumer; it can dequeue items from the queue.
- [`Iter`](#iter) - An iterator over the items of a queue.
- [`IterMut`](#itermut) - An iterator over the items of a queue.
- [`Producer`](#producer) - A producer; it can enqueue items into the queue.
- [`QueueInner`](#queueinner) - Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`].

**Type Aliases**

- [`Queue`](#queue) - A statically allocated single-producer, single-consumer queue with a capacity of `N - 1` elements.
- [`QueueView`](#queueview) - A [`Queue`] with dynamic capacity.

---

## heapless::spsc::Consumer

*Struct*

A consumer; it can dequeue items from the queue.

**Note:** The consumer semantically owns the `head` pointer of the queue.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn dequeue(self: & mut Self) -> Option<T>` - Returns the item in the front of the queue, or `None` if the queue is empty.
- `fn dequeue_unchecked(self: & mut Self) -> T` - Returns the item in the front of the queue, without checking if there are elements in the
- `fn ready(self: &Self) -> bool` - Returns if there are any items to dequeue. When this returns `true`, at least the
- `fn len(self: &Self) -> usize` - Returns the number of elements in the queue.
- `fn is_empty(self: &Self) -> bool` - Returns whether the queue is empty.
- `fn capacity(self: &Self) -> usize` - Returns the maximum number of elements the queue can hold.
- `fn peek(self: &Self) -> Option<&T>` - Returns the item in the front of the queue without dequeuing, or `None` if the queue is

**Traits:** Send



## heapless::spsc::Iter

*Struct*

An iterator over the items of a queue.

**Generic Parameters:**
- 'a
- T

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::spsc::IterMut

*Struct*

An iterator over the items of a queue.

**Generic Parameters:**
- 'a
- T

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::spsc::Producer

*Struct*

A producer; it can enqueue items into the queue.

**Note:** The producer semantically owns the `tail` pointer of the queue.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn enqueue(self: & mut Self, item: T) -> Result<(), T>` - Adds an `item` to the end of the queue, returns back the `item` if the queue is full.
- `fn enqueue_unchecked(self: & mut Self, item: T)` - Adds an `item` to the end of the queue, without checking if the queue is full.
- `fn ready(self: &Self) -> bool` - Returns if there is any space to enqueue a new item. When this returns true, at
- `fn len(self: &Self) -> usize` - Returns the number of elements in the queue.
- `fn is_empty(self: &Self) -> bool` - Returns whether the queue is empty.
- `fn capacity(self: &Self) -> usize` - Returns the maximum number of elements the queue can hold.

**Traits:** Send



## heapless::spsc::Queue

*Type Alias*: `QueueInner<T, crate::storage::OwnedStorage<N>>`

A statically allocated single-producer, single-consumer queue with a capacity of `N - 1` elements.

<div class="warning">

To get better performance, use a value for `N` that is a power of 2.

</div>

You will likely want to use [`split`](QueueInner::split) to create a producer-consumer pair.



## heapless::spsc::QueueInner

*Struct*

Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`].

In most cases you should use [`Queue`] or [`QueueView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- T
- S

**Methods:**

- `fn split_const(self: & mut Self) -> (Producer<T>, Consumer<T>)` - Splits a queue into producer and consumer endpoints.
- `fn as_view(self: &Self) -> &QueueView<T>` - Get a reference to the `Queue`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut QueueView<T>` - Get a mutable reference to the `Queue`, erasing the `N` const-generic.
- `fn capacity(self: &Self) -> usize` - Returns the maximum number of elements the queue can hold.
- `fn len(self: &Self) -> usize` - Returns the number of elements in the queue.
- `fn is_empty(self: &Self) -> bool` - Returns whether the queue is empty.
- `fn is_full(self: &Self) -> bool` - Returns whether the queue is full.
- `fn iter(self: &Self) -> Iter<T>` - Iterates from the front of the queue to the back.
- `fn iter_mut(self: & mut Self) -> IterMut<T>` - Returns an iterator that allows modifying each value.
- `fn enqueue(self: & mut Self, item: T) -> Result<(), T>` - Adds an `item` to the end of the queue.
- `fn dequeue(self: & mut Self) -> Option<T>` - Returns the item in the front of the queue, or `None` if the queue is empty.
- `fn peek(self: &Self) -> Option<&T>` - Returns a reference to the item in the front of the queue without dequeuing it, or
- `fn enqueue_unchecked(self: & mut Self, item: T)` - Adds an `item` to the end of the queue, without checking if it's full.
- `fn dequeue_unchecked(self: & mut Self) -> T` - Returns the item in the front of the queue, without checking if there is something in the
- `fn split(self: & mut Self) -> (Producer<T>, Consumer<T>)` - Splits a queue into producer and consumer endpoints.
- `fn new() -> Self` - Creates an empty queue.
- `fn split_const(self: & mut Self) -> (Producer<T>, Consumer<T>)` - Splits a queue into producer and consumer endpoints.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &QueueInner<T, S2>) -> bool`



## heapless::spsc::QueueView

*Type Alias*: `QueueInner<T, crate::storage::ViewStorage>`

A [`Queue`] with dynamic capacity.

[`Queue`] coerces to `QueueView`. `QueueView` is `!Sized`, meaning it can only ever be used by reference.



