**smoltcp > storage > ring_buffer**

# Module: storage::ring_buffer

## Contents

**Structs**

- [`RingBuffer`](#ringbuffer) - A ring buffer.

---

## smoltcp::storage::ring_buffer::RingBuffer

*Struct*

A ring buffer.

This ring buffer implementation provides many ways to interact with it:

  * Enqueueing or dequeueing one element from corresponding side of the buffer;
  * Enqueueing or dequeueing a slice of elements from corresponding side of the buffer;
  * Accessing allocated and unallocated areas directly.

It is also zero-copy; all methods provide references into the buffer's storage.
Note that all references are mutable; it is considered more important to allow
in-place processing than to protect from accidental mutation.

This implementation is suitable for both simple uses such as a FIFO queue
of UDP packets, and advanced ones such as a TCP reassembly buffer.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new<S>(storage: S) -> RingBuffer<'a, T>` - Create a ring buffer with the given storage.
- `fn clear(self: & mut Self)` - Clear the ring buffer.
- `fn capacity(self: &Self) -> usize` - Return the maximum number of elements in the ring buffer.
- `fn reset(self: & mut Self)` - Clear the ring buffer, and reset every element.
- `fn len(self: &Self) -> usize` - Return the current number of elements in the ring buffer.
- `fn window(self: &Self) -> usize` - Return the number of elements that can be added to the ring buffer.
- `fn contiguous_window(self: &Self) -> usize` - Return the largest number of elements that can be added to the buffer
- `fn is_empty(self: &Self) -> bool` - Query whether the buffer is empty.
- `fn is_full(self: &Self) -> bool` - Query whether the buffer is full.
- `fn get_unallocated(self: & mut Self, offset: usize, size: usize) -> & mut [T]` - Return the largest contiguous slice of unallocated buffer elements starting
- `fn write_unallocated(self: & mut Self, offset: usize, data: &[T]) -> usize` - Write as many elements from the given slice into unallocated buffer elements
- `fn enqueue_unallocated(self: & mut Self, count: usize)` - Enqueue the given number of unallocated buffer elements.
- `fn get_allocated(self: &Self, offset: usize, size: usize) -> &[T]` - Return the largest contiguous slice of allocated buffer elements starting
- `fn read_allocated(self: & mut Self, offset: usize, data: & mut [T]) -> usize` - Read as many elements from allocated buffer elements into the given slice
- `fn dequeue_allocated(self: & mut Self, count: usize)` - Dequeue the given number of allocated buffer elements.
- `fn enqueue_many_with<'b, R, F>(self: &'b  mut Self, f: F) -> (usize, R)` - Call `f` with the largest contiguous slice of unallocated buffer elements,
- `fn enqueue_many(self: & mut Self, size: usize) -> & mut [T]` - Enqueue a slice of elements up to the given size into the buffer,
- `fn enqueue_slice(self: & mut Self, data: &[T]) -> usize` - Enqueue as many elements from the given slice into the buffer as possible,
- `fn dequeue_many_with<'b, R, F>(self: &'b  mut Self, f: F) -> (usize, R)` - Call `f` with the largest contiguous slice of allocated buffer elements,
- `fn dequeue_many(self: & mut Self, size: usize) -> & mut [T]` - Dequeue a slice of elements up to the given size from the buffer,
- `fn dequeue_slice(self: & mut Self, data: & mut [T]) -> usize` - Dequeue as many elements from the buffer into the given slice as possible,
- `fn enqueue_one_with<'b, R, E, F>(self: &'b  mut Self, f: F) -> Result<Result<R, E>, Full>` - Call `f` with a single buffer element, and enqueue the element if `f`
- `fn enqueue_one(self: & mut Self) -> Result<& mut T, Full>` - Enqueue a single element into the buffer, and return a reference to it,
- `fn dequeue_one_with<'b, R, E, F>(self: &'b  mut Self, f: F) -> Result<Result<R, E>, Empty>` - Call `f` with a single buffer element, and dequeue the element if `f`
- `fn dequeue_one(self: & mut Self) -> Result<& mut T, Empty>` - Dequeue an element from the buffer, and return a reference to it,

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(slice: ManagedSlice<'a, T>) -> RingBuffer<'a, T>`



