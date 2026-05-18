*[heapless](../index.md) / [binary_heap](index.md)*

---

# Module `binary_heap`

A priority queue implemented with a binary heap.

Insertion and popping the largest element have *O*(log n) time complexity.
Checking the smallest/largest element is *O*(1).

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`BinaryHeapInner`](#binaryheapinner)
  - [`Hole`](#hole)
  - [`PeekMutInner`](#peekmutinner)
- [Enums](#enums)
  - [`Min`](#min)
  - [`Max`](#max)
- [Traits](#traits)
  - [`Kind`](#kind)
- [Type Aliases](#type-aliases)
  - [`BinaryHeap`](#binaryheap)
  - [`BinaryHeapView`](#binaryheapview)
  - [`PeekMut`](#peekmut)
  - [`PeekMutView`](#peekmutview)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | Sealed traits |
| [`BinaryHeapInner`](#binaryheapinner) | struct | Base struct for [`BinaryHeap`] and [`BinaryHeapView`], generic over the [`VecStorage`]. |
| [`Hole`](#hole) | struct | Hole represents a hole in a slice i.e. an index without valid value (because it was moved from or duplicated). |
| [`PeekMutInner`](#peekmutinner) | struct | Structure wrapping a mutable reference to the greatest item on a `BinaryHeap`. |
| [`Min`](#min) | enum | Min-heap |
| [`Max`](#max) | enum | Max-heap |
| [`Kind`](#kind) | trait | The binary heap kind: min-heap or max-heap |
| [`BinaryHeap`](#binaryheap) | type | A priority queue implemented with a binary heap. |
| [`BinaryHeapView`](#binaryheapview) | type | A priority queue implemented with a binary heap. |
| [`PeekMut`](#peekmut) | type | Structure wrapping a mutable reference to the greatest item on a `BinaryHeap`. |
| [`PeekMutView`](#peekmutview) | type | Structure wrapping a mutable reference to the greatest item on a `BinaryHeap`. |

## Modules

- [`private`](private/index.md) — Sealed traits

## Structs

### `BinaryHeapInner<T, K, S: VecStorage<T> + ?Sized>`

```rust
struct BinaryHeapInner<T, K, S: VecStorage<T> + ?Sized> {
    _kind: core::marker::PhantomData<K>,
    data: crate::vec::VecInner<T, usize, S>,
}
```

Base struct for [`BinaryHeap`](#binaryheap) and [`BinaryHeapView`](#binaryheapview), generic over the [`VecStorage`](../vec/storage/index.md).

In most cases you should use [`BinaryHeap`](#binaryheap) or [`BinaryHeapView`](#binaryheapview) directly. Only use this
struct if you want to write code that's generic over both.

#### Implementations

- <span id="binaryheapinner-new"></span>`const fn new() -> Self`

  Creates an empty `BinaryHeap` as a $K-heap.

  

  ```rust

  use heapless::binary_heap::{BinaryHeap, Max};

  

  // allocate the binary heap on the stack

  let mut heap: BinaryHeap<_, Max, 8> = BinaryHeap::new();

  heap.push(4).unwrap();

  

  // allocate the binary heap in a static variable

  static mut HEAP: BinaryHeap<i32, Max, 8> = BinaryHeap::new();

  ```

#### Trait Implementations

##### `impl<T, K, S> Debug for BinaryHeapInner<T, K, S>`

- <span id="binaryheapinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DeserializeOwned for BinaryHeapInner<T, K, S>`

##### `impl<T, K, S> IntoIterator for &'a BinaryHeapInner<T, K, S>`

- <span id="a-binaryheapinner-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-binaryheapinner-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-binaryheapinner-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, KIND, S> Serialize for crate::binary_heap::BinaryHeapInner<T, KIND, S>`

- <span id="cratebinary-heapbinaryheapinner-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `Hole<'a, T>`

```rust
struct Hole<'a, T> {
    data: &'a mut [T],
    elt: core::mem::ManuallyDrop<T>,
    pos: usize,
}
```

Hole represents a hole in a slice i.e. an index without valid value
(because it was moved from or duplicated).
In drop, `Hole` will restore the slice by filling the hole
position with the value that was originally removed.

#### Fields

- **`elt`**: `core::mem::ManuallyDrop<T>`

  `elt` is always `Some` from new until drop.

#### Implementations

- <span id="hole-new"></span>`unsafe fn new(data: &'a mut [T], pos: usize) -> Self`

  Create a new Hole at index `pos`.

  

  Unsafe because pos must be within the data slice.

- <span id="hole-pos"></span>`fn pos(&self) -> usize`

- <span id="hole-element"></span>`fn element(&self) -> &T`

  Returns a reference to the element removed.

- <span id="hole-get"></span>`unsafe fn get(&self, index: usize) -> &T`

  Returns a reference to the element at `index`.

  

  Unsafe because index must be within the data slice and not equal to pos.

- <span id="hole-move-to"></span>`unsafe fn move_to(&mut self, index: usize)`

  Move hole to new location

  

  Unsafe because index must be within the data slice and not equal to pos.

#### Trait Implementations

##### `impl<T> Drop for Hole<'_, T>`

- <span id="hole-drop"></span>`fn drop(&mut self)`

### `PeekMutInner<'a, T, K, S>`

```rust
struct PeekMutInner<'a, T, K, S>
where
    T: Ord,
    K: Kind,
    S: VecStorage<T> + ?Sized {
    heap: &'a mut BinaryHeapInner<T, K, S>,
    sift: bool,
}
```

Structure wrapping a mutable reference to the greatest item on a
`BinaryHeap`.

This `struct` is created by `BinaryHeapInner::peek_mut`.
See its documentation for more.

#### Implementations

- <span id="peekmutinner-pop"></span>`fn pop(this: Self) -> T`

  Removes the peeked value from the heap and returns it.

#### Trait Implementations

##### `impl<T, K, S> Deref for PeekMutInner<'_, T, K, S>`

- <span id="peekmutinner-deref-type-target"></span>`type Target = T`

- <span id="peekmutinner-deref"></span>`fn deref(&self) -> &T`

##### `impl<T, K, S> DerefMut for PeekMutInner<'_, T, K, S>`

- <span id="peekmutinner-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T, K, S> Drop for PeekMutInner<'_, T, K, S>`

- <span id="peekmutinner-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for PeekMutInner<'a, T, K, S>`

- <span id="peekmutinner-receiver-type-target"></span>`type Target = T`

## Enums

### `Min`

```rust
enum Min {
}
```

Min-heap

#### Trait Implementations

##### `impl Kind for Min`

##### `impl Sealed for Min`

### `Max`

```rust
enum Max {
}
```

Max-heap

#### Trait Implementations

##### `impl Kind for Max`

##### `impl Sealed for Max`

## Traits

### `Kind`

```rust
trait Kind: private::Sealed { ... }
```

The binary heap kind: min-heap or max-heap

#### Implementors

- [`Max`](#max)
- [`Min`](#min)

## Type Aliases

### `BinaryHeap<T, K, const N: usize>`

```rust
type BinaryHeap<T, K, const N: usize> = BinaryHeapInner<T, K, crate::vec::OwnedVecStorage<T, N>>;
```

A priority queue implemented with a binary heap.

This can be either a min-heap or a max-heap.

It is a logic error for an item to be modified in such a way that the item's ordering relative
to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.

```rust
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

### `BinaryHeapView<T, K>`

```rust
type BinaryHeapView<T, K> = BinaryHeapInner<T, K, crate::vec::ViewVecStorage<T>>;
```

A priority queue implemented with a binary heap.

This can be either a min-heap or a max-heap.

It is a logic error for an item to be modified in such a way that the item's ordering relative
to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.

```rust
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

### `PeekMut<'a, T, K, const N: usize>`

```rust
type PeekMut<'a, T, K, const N: usize> = PeekMutInner<'a, T, K, crate::vec::OwnedVecStorage<T, N>>;
```

Structure wrapping a mutable reference to the greatest item on a
`BinaryHeap`.

This `struct` is created by `BinaryHeap::peek_mut`.
See its documentation for more.

### `PeekMutView<'a, T, K>`

```rust
type PeekMutView<'a, T, K> = PeekMutInner<'a, T, K, crate::vec::ViewVecStorage<T>>;
```

Structure wrapping a mutable reference to the greatest item on a
`BinaryHeap`.

This `struct` is created by `BinaryHeapView::peek_mut`.
See its documentation for more.

