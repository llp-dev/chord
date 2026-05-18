**heapless > binary_heap**

# Module: binary_heap

## Contents

**Structs**

- [`BinaryHeapInner`](#binaryheapinner) - Base struct for [`BinaryHeap`] and [`BinaryHeapView`], generic over the [`VecStorage`].
- [`PeekMutInner`](#peekmutinner) - Structure wrapping a mutable reference to the greatest item on a

**Enums**

- [`Max`](#max) - Max-heap
- [`Min`](#min) - Min-heap

**Traits**

- [`Kind`](#kind) - The binary heap kind: min-heap or max-heap

**Type Aliases**

- [`BinaryHeap`](#binaryheap) - A priority queue implemented with a binary heap.
- [`BinaryHeapView`](#binaryheapview) - A priority queue implemented with a binary heap.
- [`PeekMut`](#peekmut) - Structure wrapping a mutable reference to the greatest item on a
- [`PeekMutView`](#peekmutview) - Structure wrapping a mutable reference to the greatest item on a

---

## heapless::binary_heap::BinaryHeap

*Type Alias*: `BinaryHeapInner<T, K, crate::vec::OwnedVecStorage<T, N>>`

A priority queue implemented with a binary heap.

This can be either a min-heap or a max-heap.

It is a logic error for an item to be modified in such a way that the item's ordering relative
to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.

```
use heapless::binary_heap::{BinaryHeap, Max};

let mut heap: BinaryHeap<_, Max, 8> = BinaryHeap::new();

// We can use peek to look at the next item in the heap. In this case,
// there's no items in there yet so we get None.
assert_eq!(heap.peek(), None);

// Let's add some scores...
heap.push(1).unwrap();
heap.push(5).unwrap();
heap.push(2).unwrap();

// Now peek shows the most important item in the heap.
assert_eq!(heap.peek(), Some(&5));

// We can check the length of a heap.
assert_eq!(heap.len(), 3);

// We can iterate over the items in the heap, although they are returned in
// a random order.
for x in &heap {
    println!("{}", x);
}

// If we instead pop these scores, they should come back in order.
assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), None);

// We can clear the heap of any remaining items.
heap.clear();

// The heap should now be empty.
assert!(heap.is_empty())
```



## heapless::binary_heap::BinaryHeapInner

*Struct*

Base struct for [`BinaryHeap`] and [`BinaryHeapView`], generic over the [`VecStorage`].

In most cases you should use [`BinaryHeap`] or [`BinaryHeapView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- T
- K
- S

**Methods:**

- `fn into_vec(self: Self) -> Vec<T, N, usize>` - Returns the underlying `Vec<T,N>`. Order is arbitrary and time is *O*(1).
- `fn as_view(self: &Self) -> &BinaryHeapView<T, K>` - Get a reference to the `BinaryHeap`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut BinaryHeapView<T, K>` - Get a mutable reference to the `BinaryHeap`, erasing the `N` const-generic.
- `fn capacity(self: &Self) -> usize` - Returns the capacity of the binary heap.
- `fn clear(self: & mut Self)` - Drops all items from the binary heap.
- `fn len(self: &Self) -> usize` - Returns the length of the binary heap.
- `fn is_empty(self: &Self) -> bool` - Checks if the binary heap is empty.
- `fn is_full(self: &Self) -> bool` - Checks if the binary heap is full.
- `fn iter(self: &Self) -> slice::Iter<T>` - Returns an iterator visiting all values in the underlying vector, in arbitrary order.
- `fn iter_mut(self: & mut Self) -> slice::IterMut<T>` - Returns a mutable iterator visiting all values in the underlying vector, in arbitrary order.
- `fn peek(self: &Self) -> Option<&T>` - Returns the *top* (greatest if max-heap, smallest if min-heap) item in the binary heap, or
- `fn peek_mut(self: & mut Self) -> Option<PeekMutInner<T, K, S>>` - Returns a mutable reference to the greatest item in the binary heap, or
- `fn pop(self: & mut Self) -> Option<T>` - Removes the *top* (greatest if max-heap, smallest if min-heap) item from the binary heap and
- `fn pop_unchecked(self: & mut Self) -> T` - Removes the *top* (greatest if max-heap, smallest if min-heap) item from the binary heap and
- `fn push(self: & mut Self, item: T) -> Result<(), T>` - Pushes an item onto the binary heap.
- `fn push_unchecked(self: & mut Self, item: T)` - Pushes an item onto the binary heap without first checking if it's full.
- `fn new() -> Self` - Creates an empty `BinaryHeap` as a $K-heap.

**Trait Implementations:**

- **Serialize**
  - `fn serialize<SER>(self: &Self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## heapless::binary_heap::BinaryHeapView

*Type Alias*: `BinaryHeapInner<T, K, crate::vec::ViewVecStorage<T>>`

A priority queue implemented with a binary heap.

This can be either a min-heap or a max-heap.

It is a logic error for an item to be modified in such a way that the item's ordering relative
to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.

```
use heapless::binary_heap::{BinaryHeap, BinaryHeapView, Max};

let mut heap_buffer: BinaryHeap<_, Max, 8> = BinaryHeap::new();
let heap: &mut BinaryHeapView<_, Max> = &mut heap_buffer;

// We can use peek to look at the next item in the heap. In this case,
// there's no items in there yet so we get None.
assert_eq!(heap.peek(), None);

// Let's add some scores...
heap.push(1).unwrap();
heap.push(5).unwrap();
heap.push(2).unwrap();

// Now peek shows the most important item in the heap.
assert_eq!(heap.peek(), Some(&5));

// We can check the length of a heap.
assert_eq!(heap.len(), 3);

// We can iterate over the items in the heap, although they are returned in
// a random order.
for x in &*heap {
    println!("{}", x);
}

// If we instead pop these scores, they should come back in order.
assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), None);

// We can clear the heap of any remaining items.
heap.clear();

// The heap should now be empty.
assert!(heap.is_empty())
```



## heapless::binary_heap::Kind

*Trait*

The binary heap kind: min-heap or max-heap



## heapless::binary_heap::Max

*Enum*

Max-heap

**Traits:** Kind



## heapless::binary_heap::Min

*Enum*

Min-heap

**Traits:** Kind



## heapless::binary_heap::PeekMut

*Type Alias*: `PeekMutInner<'a, T, K, crate::vec::OwnedVecStorage<T, N>>`

Structure wrapping a mutable reference to the greatest item on a
`BinaryHeap`.

This `struct` is created by [`BinaryHeap::peek_mut`].
See its documentation for more.



## heapless::binary_heap::PeekMutInner

*Struct*

Structure wrapping a mutable reference to the greatest item on a
`BinaryHeap`.

This `struct` is created by [`BinaryHeapInner::peek_mut`].
See its documentation for more.

**Generic Parameters:**
- 'a
- T
- K
- S

**Methods:**

- `fn pop(this: Self) -> T` - Removes the peeked value from the heap and returns it.

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &T`



## heapless::binary_heap::PeekMutView

*Type Alias*: `PeekMutInner<'a, T, K, crate::vec::ViewVecStorage<T>>`

Structure wrapping a mutable reference to the greatest item on a
`BinaryHeap`.

This `struct` is created by [`BinaryHeapView::peek_mut`].
See its documentation for more.



