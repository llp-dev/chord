*[heapless](../index.md) / [sorted_linked_list](index.md)*

---

# Module `sorted_linked_list`

A fixed sorted priority linked list, similar to [`BinaryHeap`](../binary_heap/index.md) but with different properties
on `push`, `pop`, etc.

For example, the sorting of the list will never `memcpy` the underlying value, so having large
objects in the list will not cause a performance hit.

# Examples

```rust
use heapless::sorted_linked_list::{Max, SortedLinkedList};
let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();

// The largest value will always be first
ll.push(1).unwrap();
assert_eq!(ll.peek(), Some(&1));

ll.push(2).unwrap();
assert_eq!(ll.peek(), Some(&2));

ll.push(3).unwrap();
assert_eq!(ll.peek(), Some(&3));

// This will not fit in the queue.
assert_eq!(ll.push(4), Err(4));
```


## Contents

- [Modules](#modules)
  - [`storage`](#storage)
  - [`private`](#private)
- [Structs](#structs)
  - [`Min`](#min)
  - [`Max`](#max)
  - [`Node`](#node)
  - [`SortedLinkedListInner`](#sortedlinkedlistinner)
  - [`IterView`](#iterview)
  - [`FindMutView`](#findmutview)
- [Traits](#traits)
  - [`SortedLinkedListStorage`](#sortedlinkedliststorage)
  - [`Kind`](#kind)
- [Type Aliases](#type-aliases)
  - [`OwnedSortedLinkedListStorage`](#ownedsortedlinkedliststorage)
  - [`ViewSortedLinkedListStorage`](#viewsortedlinkedliststorage)
  - [`SortedLinkedList`](#sortedlinkedlist)
  - [`SortedLinkedListView`](#sortedlinkedlistview)
- [Macros](#macros)
  - [`impl_const_new!`](#impl-const-new)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`storage`](#storage) | mod |  |
| [`private`](#private) | mod | Sealed traits |
| [`Min`](#min) | struct | Marker for Min sorted [`SortedLinkedList`]. |
| [`Max`](#max) | struct | Marker for Max sorted [`SortedLinkedList`]. |
| [`Node`](#node) | struct | A node in the [`SortedLinkedList`]. |
| [`SortedLinkedListInner`](#sortedlinkedlistinner) | struct | Base struct for [`SortedLinkedList`] and [`SortedLinkedListView`], generic over the [`SortedLinkedListStorage`]. |
| [`IterView`](#iterview) | struct | Iterator for the linked list. |
| [`FindMutView`](#findmutview) | struct | Comes from [`SortedLinkedList::find_mut`]. |
| [`SortedLinkedListStorage`](#sortedlinkedliststorage) | trait |  |
| [`Kind`](#kind) | trait | The linked list kind: min-list or max-list |
| [`OwnedSortedLinkedListStorage`](#ownedsortedlinkedliststorage) | type |  |
| [`ViewSortedLinkedListStorage`](#viewsortedlinkedliststorage) | type |  |
| [`SortedLinkedList`](#sortedlinkedlist) | type | The linked list. |
| [`SortedLinkedListView`](#sortedlinkedlistview) | type | The linked list. |
| [`impl_const_new!`](#impl-const-new) | macro |  |

## Modules

- [`storage`](storage/index.md)
- [`private`](private/index.md) — Sealed traits

## Structs

### `Min`

```rust
struct Min;
```

Marker for Min sorted [`SortedLinkedList`](#sortedlinkedlist).

#### Trait Implementations

##### `impl Kind for Min`

##### `impl Sealed for Min`

### `Max`

```rust
struct Max;
```

Marker for Max sorted [`SortedLinkedList`](#sortedlinkedlist).

#### Trait Implementations

##### `impl Kind for Max`

##### `impl Sealed for Max`

### `Node<T, Idx>`

```rust
struct Node<T, Idx> {
    val: core::mem::MaybeUninit<T>,
    next: Idx,
}
```

A node in the [`SortedLinkedList`](#sortedlinkedlist).

### `SortedLinkedListInner<T, Idx, K, S>`

```rust
struct SortedLinkedListInner<T, Idx, K, S>
where
    Idx: LenType,
    S: SortedLinkedListStorage<T, Idx> + ?Sized {
    head: Idx,
    free: Idx,
    phantom: core::marker::PhantomData<(K, T)>,
    list: S,
}
```

Base struct for [`SortedLinkedList`](#sortedlinkedlist) and [`SortedLinkedListView`](#sortedlinkedlistview), generic over the [`SortedLinkedListStorage`](storage/index.md).

In most cases you should use [`SortedLinkedList`](#sortedlinkedlist) or [`SortedLinkedListView`](#sortedlinkedlistview) directly. Only use this
struct if you want to write code that's generic over both.

#### Implementations

- <span id="sortedlinkedlistinner-new-u8"></span>`const fn new_u8() -> Self`

  Create a new linked list.

#### Trait Implementations

##### `impl<T, Idx, K, S> Debug for SortedLinkedListInner<T, Idx, K, S>`

- <span id="sortedlinkedlistinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, Idx, K, S> Drop for SortedLinkedListInner<T, Idx, K, S>`

- <span id="sortedlinkedlistinner-drop"></span>`fn drop(&mut self)`

### `IterView<'a, T, Idx, K>`

```rust
struct IterView<'a, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind {
    list: &'a SortedLinkedListInner<T, Idx, K, ViewSortedLinkedListStorage<T, Idx>>,
    index: Idx,
}
```

Iterator for the linked list.

#### Trait Implementations

##### `impl IntoIterator for IterView<'a, T, Idx, K>`

- <span id="iterview-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterview-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterview-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, Idx, K> Iterator for IterView<'a, T, Idx, K>`

- <span id="iterview-iterator-type-item"></span>`type Item = &'a T`

- <span id="iterview-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `FindMutView<'a, T, Idx, K>`

```rust
struct FindMutView<'a, T, Idx, K>
where
    T: Ord,
    Idx: LenType,
    K: Kind {
    list: &'a mut SortedLinkedListView<T, K, Idx>,
    is_head: bool,
    prev_index: Idx,
    index: Idx,
    maybe_changed: bool,
}
```

Comes from `SortedLinkedList::find_mut`.

#### Implementations

- <span id="findmutview-pop-internal"></span>`fn pop_internal(&mut self) -> T`

- <span id="findmutview-pop"></span>`fn pop(self) -> T`

  This will pop the element from the list.

  

  Complexity is worst-case *O*(1).

  

  # Example

  

  ```rust

  use heapless::sorted_linked_list::{Max, SortedLinkedList};

  let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();

  

  ll.push(1).unwrap();

  ll.push(2).unwrap();

  ll.push(3).unwrap();

  

  // Find a value and update it

  let mut find = ll.find_mut(|v| *v == 2).unwrap();

  find.pop();

  

  assert_eq!(ll.pop(), Some(3));

  assert_eq!(ll.pop(), Some(1));

  assert_eq!(ll.pop(), None);

  ```

- <span id="findmutview-finish"></span>`fn finish(self)`

  This will resort the element into the correct position in the list if needed. The resorting

  will only happen if the element has been accessed mutably.

  

  Same as calling `drop`.

  

  Complexity is worst-case *O*(n).

  

  # Example

  

  ```rust

  use heapless::sorted_linked_list::{Max, SortedLinkedList};

  let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();

  

  ll.push(1).unwrap();

  ll.push(2).unwrap();

  ll.push(3).unwrap();

  

  let mut find = ll.find_mut(|v| *v == 2).unwrap();

  find.finish(); // No resort, we did not access the value.

  

  let mut find = ll.find_mut(|v| *v == 2).unwrap();

  *find += 1000;

  find.finish(); // Will resort, we accessed (and updated) the value.

  

  assert_eq!(ll.pop(), Some(1002));

  assert_eq!(ll.pop(), Some(3));

  assert_eq!(ll.pop(), Some(1));

  assert_eq!(ll.pop(), None);

  ```

#### Trait Implementations

##### `impl<T, Idx, K> Deref for FindMutView<'_, T, Idx, K>`

- <span id="findmutview-deref-type-target"></span>`type Target = T`

- <span id="findmutview-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T, Idx, K> DerefMut for FindMutView<'_, T, Idx, K>`

- <span id="findmutview-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T, Idx, K> Drop for FindMutView<'_, T, Idx, K>`

- <span id="findmutview-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for FindMutView<'a, T, Idx, K>`

- <span id="findmutview-receiver-type-target"></span>`type Target = T`

## Traits

### `SortedLinkedListStorage<T, Idx>`

```rust
trait SortedLinkedListStorage<T, Idx>: SortedLinkedListSealedStorage<T, Idx> { ... }
```

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedSortedLinkedListStorage`](storage/index.md): stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewSortedLinkedListStorage`](storage/index.md): stores the data in an unsized `[T]`.

This allows [`SortedLinkedList`](#sortedlinkedlist) to be generic over either sized or unsized storage. The [`sorted_linked_list`](super)
module contains a [`SortedLinkedListInner`](#sortedlinkedlistinner) struct that's generic on [`SortedLinkedListStorage`](storage/index.md),
and two type aliases for convenience:

- [`SortedLinkedList<T, Idx, N>`](super::SortedLinkedList) = `SortedLinkedListInner<T, OwnedSortedLinkedListStorage<T, Idx, N>>`
- [`SortedLinkedListView<T, Idx>`](super::SortedLinkedListView) = `SortedLinkedListInner<T, ViewSortedLinkedListStorage<T, Idx>>`

`SortedLinkedList` can be unsized into `SortedLinkedListView`, either by unsizing coercions such as `&mut SortedLinkedList -> &mut SortedLinkedListView` or
`Box<SortedLinkedList> -> Box<SortedLinkedListView>`, or explicitly with [`.as_view()`](super::SortedLinkedList::as_view) or [`.as_mut_view()`](super::SortedLinkedList::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.




#### Implementors

- [`OwnedSortedLinkedListStorage`](storage/index.md#ownedsortedlinkedliststorage)
- [`ViewSortedLinkedListStorage`](storage/index.md#viewsortedlinkedliststorage)

### `Kind`

```rust
trait Kind: private::Sealed { ... }
```

The linked list kind: min-list or max-list

#### Implementors

- [`Max`](#max)
- [`Min`](#min)

## Type Aliases

### `OwnedSortedLinkedListStorage<T, Idx, const N: usize>`

```rust
type OwnedSortedLinkedListStorage<T, Idx, const N: usize> = SortedLinkedListStorageInner<[super::Node<T, Idx>; N]>;
```

Implementation of [`SortedLinkedListStorage`](storage/index.md) that stores the data in an array `[T; N]` whose size is known at compile time.

### `ViewSortedLinkedListStorage<T, Idx>`

```rust
type ViewSortedLinkedListStorage<T, Idx> = SortedLinkedListStorageInner<[super::Node<T, Idx>]>;
```

Implementation of [`SortedLinkedListStorage`](storage/index.md) that stores the data in an unsized `[T]`.

### `SortedLinkedList<T, K, const N: usize, Idx>`

```rust
type SortedLinkedList<T, K, const N: usize, Idx> = SortedLinkedListInner<T, Idx, K, OwnedSortedLinkedListStorage<T, Idx, N>>;
```

The linked list.

### `SortedLinkedListView<T, K, Idx>`

```rust
type SortedLinkedListView<T, K, Idx> = SortedLinkedListInner<T, Idx, K, ViewSortedLinkedListStorage<T, Idx>>;
```

The linked list.

## Macros

### `impl_const_new!`

