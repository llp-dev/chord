**heapless > deque**

# Module: deque

## Contents

**Structs**

- [`DequeInner`](#dequeinner) - Base struct for [`Deque`] and [`DequeView`], generic over the [`VecStorage`].
- [`IntoIter`](#intoiter) - An iterator that moves out of a [`Deque`].
- [`Iter`](#iter) - Iterator over the contents of a [`Deque`]
- [`IterMut`](#itermut) - Iterator over the contents of a [`Deque`]

**Type Aliases**

- [`Deque`](#deque) - A fixed capacity double-ended queue.
- [`DequeView`](#dequeview) - A double-ended queue with dynamic capacity.

---

## heapless::deque::Deque

*Type Alias*: `DequeInner<T, crate::vec::OwnedVecStorage<T, N>>`

A fixed capacity double-ended queue.

# Examples

```
use heapless::Deque;

// A deque with a fixed capacity of 8 elements allocated on the stack
let mut deque = Deque::<_, 8>::new();

// You can use it as a good old FIFO queue.
deque.push_back(1);
deque.push_back(2);
assert_eq!(deque.len(), 2);

assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.len(), 0);

// Deque is double-ended, you can push and pop from the front and back.
deque.push_back(1);
deque.push_front(2);
deque.push_back(3);
deque.push_front(4);
assert_eq!(deque.pop_front(), Some(4));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(3));

// You can iterate it, yielding all the elements front-to-back.
for x in &deque {
    println!("{}", x);
}
```



## heapless::deque::DequeInner

*Struct*

Base struct for [`Deque`] and [`DequeView`], generic over the [`VecStorage`].

In most cases you should use [`Deque`] or [`DequeView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- T
- S

**Methods:**

- `fn as_view(self: &Self) -> &DequeView<T>` - Get a reference to the `Deque`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut DequeView<T>` - Get a mutable reference to the `Deque`, erasing the `N` const-generic.
- `fn storage_capacity(self: &Self) -> usize` - Returns the maximum number of elements the deque can hold.
- `fn storage_len(self: &Self) -> usize` - Returns the number of elements currently in the deque.
- `fn clear(self: & mut Self)` - Clears the deque, removing all values.
- `fn is_empty(self: &Self) -> bool` - Returns whether the deque is empty.
- `fn is_full(self: &Self) -> bool` - Returns whether the deque is full (i.e. if `len() == capacity()`.
- `fn as_slices(self: &Self) -> (&[T], &[T])` - Returns a pair of slices which contain, in order, the contents of the `Deque`.
- `fn as_mut_slices(self: & mut Self) -> (& mut [T], & mut [T])` - Returns a pair of mutable slices which contain, in order, the contents of the `Deque`.
- `fn make_contiguous(self: & mut Self) -> & mut [T]` - Rearranges the internal storage of the [`Deque`] to make it into a contiguous slice,
- `fn front(self: &Self) -> Option<&T>` - Provides a reference to the front element, or None if the `Deque` is empty.
- `fn front_mut(self: & mut Self) -> Option<& mut T>` - Provides a mutable reference to the front element, or None if the `Deque` is empty.
- `fn back(self: &Self) -> Option<&T>` - Provides a reference to the back element, or None if the `Deque` is empty.
- `fn back_mut(self: & mut Self) -> Option<& mut T>` - Provides a mutable reference to the back element, or None if the `Deque` is empty.
- `fn pop_front(self: & mut Self) -> Option<T>` - Removes the item from the front of the deque and returns it, or `None` if it's empty
- `fn pop_back(self: & mut Self) -> Option<T>` - Removes the item from the back of the deque and returns it, or `None` if it's empty
- `fn push_front(self: & mut Self, item: T) -> Result<(), T>` - Appends an `item` to the front of the deque
- `fn push_back(self: & mut Self, item: T) -> Result<(), T>` - Appends an `item` to the back of the deque
- `fn pop_front_unchecked(self: & mut Self) -> T` - Removes an item from the front of the deque and returns it, without checking that the deque
- `fn pop_back_unchecked(self: & mut Self) -> T` - Removes an item from the back of the deque and returns it, without checking that the deque
- `fn push_front_unchecked(self: & mut Self, item: T)` - Appends an `item` to the front of the deque
- `fn push_back_unchecked(self: & mut Self, item: T)` - Appends an `item` to the back of the deque
- `fn get(self: &Self, index: usize) -> Option<&T>` - Returns a reference to the element at the given index.
- `fn get_mut(self: & mut Self, index: usize) -> Option<& mut T>` - Returns a mutable reference to the element at the given index.
- `fn get_unchecked(self: &Self, index: usize) -> &T` - Returns a reference to the element at the given index without checking if it exists.
- `fn get_unchecked_mut(self: & mut Self, index: usize) -> & mut T` - Returns a mutable reference to the element at the given index without checking if it exists.
- `fn swap(self: & mut Self, i: usize, j: usize)` - Swaps elements at indices `i` and `j`.
- `fn swap_unchecked(self: & mut Self, i: usize, j: usize)` - Swaps elements at indices `i` and `j` without checking that they exist.
- `fn swap_remove_front(self: & mut Self, index: usize) -> Option<T>` - Removes an element from anywhere in the deque and returns it, replacing it with the first
- `fn swap_remove_back(self: & mut Self, index: usize) -> Option<T>` - Removes an element from anywhere in the deque and returns it, replacing it with the last
- `fn iter(self: &Self) -> Iter<T>` - Returns an iterator over the deque.
- `fn iter_mut(self: & mut Self) -> IterMut<T>` - Returns an iterator that allows modifying each value.
- `fn new() -> Self` - Constructs a new, empty deque with a fixed capacity of `N`
- `fn capacity(self: &Self) -> usize` - Returns the maximum number of elements the deque can hold.
- `fn len(self: &Self) -> usize` - Returns the number of elements currently in the deque.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **Serialize**
  - `fn serialize<SER>(self: &Self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`



## heapless::deque::DequeView

*Type Alias*: `DequeInner<T, crate::vec::ViewVecStorage<T>>`

A double-ended queue with dynamic capacity.

# Examples

```
use heapless::deque::{Deque, DequeView};

// A deque with a fixed capacity of 8 elements allocated on the stack
let mut deque_buf = Deque::<_, 8>::new();

// A DequeView can be obtained through unsized coercion of a `Deque`
let deque: &mut DequeView<_> = &mut deque_buf;

// You can use it as a good old FIFO queue.
deque.push_back(1);
deque.push_back(2);
assert_eq!(deque.storage_len(), 2);

assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.storage_len(), 0);

// DequeView is double-ended, you can push and pop from the front and back.
deque.push_back(1);
deque.push_front(2);
deque.push_back(3);
deque.push_front(4);
assert_eq!(deque.pop_front(), Some(4));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(3));

// You can iterate it, yielding all the elements front-to-back.
for x in deque {
    println!("{}", x);
}
```



## heapless::deque::IntoIter

*Struct*

An iterator that moves out of a [`Deque`].

This struct is created by calling the `into_iter` method.

**Generic Parameters:**
- T
- const N

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> IntoIter<T, N>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::deque::Iter

*Struct*

Iterator over the contents of a [`Deque`]

**Generic Parameters:**
- 'a
- T

**Traits:** FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## heapless::deque::IterMut

*Struct*

Iterator over the contents of a [`Deque`]

**Generic Parameters:**
- 'a
- T

**Traits:** FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



