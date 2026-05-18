*[heapless](../index.md) / [deque](index.md)*

---

# Module `deque`

A fixed capacity double-ended queue.

# Examples

```rust
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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DequeInner`](#dequeinner) | struct | Base struct for [`Deque`] and [`DequeView`], generic over the [`VecStorage`]. |
| [`Iter`](#iter) | struct | Iterator over the contents of a [`Deque`] |
| [`IterMut`](#itermut) | struct | Iterator over the contents of a [`Deque`] |
| [`IntoIter`](#intoiter) | struct | An iterator that moves out of a [`Deque`]. |
| [`Deque`](#deque) | type | A fixed capacity double-ended queue. |
| [`DequeView`](#dequeview) | type | A double-ended queue with dynamic capacity. |

## Structs

### `DequeInner<T, S: VecStorage<T> + ?Sized>`

```rust
struct DequeInner<T, S: VecStorage<T> + ?Sized> {
    phantom: core::marker::PhantomData<T>,
    front: usize,
    back: usize,
    full: bool,
    buffer: S,
}
```

Base struct for [`Deque`](#deque) and [`DequeView`](#dequeview), generic over the [`VecStorage`](../vec/storage/index.md).

In most cases you should use [`Deque`](#deque) or [`DequeView`](#dequeview) directly. Only use this
struct if you want to write code that's generic over both.

#### Fields

- **`front`**: `usize`

  Front index. Always 0..=(N-1)

- **`back`**: `usize`

  Back index. Always 0..=(N-1).

- **`full`**: `bool`

  Used to distinguish "empty" and "full" cases when `front == back`.
  May only be `true` if `front == back`, always `false` otherwise.

#### Implementations

- <span id="dequeinner-const-init"></span>`const INIT: MaybeUninit<T>`

- <span id="dequeinner-new"></span>`const fn new() -> Self`

  Constructs a new, empty deque with a fixed capacity of `N`

  

  # Examples

  

  ```rust

  use heapless::Deque;

  

  // allocate the deque on the stack

  let mut x: Deque<u8, 16> = Deque::new();

  

  // allocate the deque in a static variable

  static mut X: Deque<u8, 16> = Deque::new();

  ```

- <span id="dequeinner-capacity"></span>`const fn capacity(&self) -> usize`

  Returns the maximum number of elements the deque can hold.

  

  This method is not available on a `DequeView`, use [`storage_capacity`](DequeInner::storage_capacity) instead.

- <span id="dequeinner-len"></span>`const fn len(&self) -> usize`

  Returns the number of elements currently in the deque.

  

  This method is not available on a `DequeView`, use [`storage_len`](DequeInner::storage_len) instead.

#### Trait Implementations

##### `impl<T: fmt::Debug, S: VecStorage<T> + ?Sized> Debug for DequeInner<T, S>`

- <span id="dequeinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DeserializeOwned for DequeInner<T, S>`

##### `impl<T, S: VecStorage<T> + ?Sized> Drop for DequeInner<T, S>`

- <span id="dequeinner-drop"></span>`fn drop(&mut self)`

##### `impl<T, S: VecStorage<T> + ?Sized> Extend for DequeInner<T, S>`

- <span id="dequeinner-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T, S: VecStorage<T> + ?Sized> IntoIterator for &'a DequeInner<T, S>`

- <span id="a-dequeinner-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-dequeinner-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-dequeinner-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, S: VecStorage<T> + ?Sized> Serialize for crate::deque::DequeInner<T, S>`

- <span id="cratedequedequeinner-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: core::iter::Chain<core::slice::Iter<'a, T>, core::slice::Iter<'a, T>>,
}
```

Iterator over the contents of a [`Deque`](#deque)

#### Trait Implementations

##### `impl<T> DoubleEndedIterator for Iter<'_, T>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Iter<'_, T>`

##### `impl<T> FusedIterator for Iter<'_, T>`

##### `impl IntoIterator for Iter<'a, T>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Iter<'a, T>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a T`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    inner: core::iter::Chain<core::slice::IterMut<'a, T>, core::slice::IterMut<'a, T>>,
}
```

Iterator over the contents of a [`Deque`](#deque)

#### Trait Implementations

##### `impl<T> DoubleEndedIterator for IterMut<'_, T>`

- <span id="itermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IterMut<'_, T>`

##### `impl<T> FusedIterator for IterMut<'_, T>`

##### `impl IntoIterator for IterMut<'a, T>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterMut<'a, T>`

- <span id="itermut-iterator-type-item"></span>`type Item = &'a mut T`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IntoIter<T, const N: usize>`

```rust
struct IntoIter<T, const N: usize> {
    deque: Deque<T, N>,
}
```

An iterator that moves out of a [`Deque`](#deque).

This struct is created by calling the `into_iter` method.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for IntoIter<T, N>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T, N>` — [`IntoIter`](#intoiter)

##### `impl IntoIterator for IntoIter<T, N>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IntoIter<T, N>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `Deque<T, const N: usize>`

```rust
type Deque<T, const N: usize> = DequeInner<T, crate::vec::OwnedVecStorage<T, N>>;
```

A fixed capacity double-ended queue.

# Examples

```rust
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

### `DequeView<T>`

```rust
type DequeView<T> = DequeInner<T, crate::vec::ViewVecStorage<T>>;
```

A double-ended queue with dynamic capacity.

# Examples

```rust
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

