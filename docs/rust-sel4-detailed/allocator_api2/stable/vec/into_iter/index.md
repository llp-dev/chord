*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [into_iter](index.md)*

---

# Module `into_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | An iterator that moves out of a vector. |

## Structs

### `IntoIter<T, A: Allocator>`

```rust
struct IntoIter<T, A: Allocator> {
    buf: core::ptr::NonNull<T>,
    phantom: core::marker::PhantomData<T>,
    cap: usize,
    alloc: core::mem::ManuallyDrop<A>,
    ptr: *const T,
    end: *const T,
}
```

An iterator that moves out of a vector.

This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)
(provided by the `IntoIterator` trait).

# Example

```rust
let v = vec![0, 1, 2];
let iter: std::vec::IntoIter<_> = v.into_iter();
```

#### Implementations

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Returns the remaining items of this iterator as a slice.

  

  # Examples

  

  ```rust

  let vec = vec!['a', 'b', 'c'];

  let mut into_iter = vec.into_iter();

  assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);

  let _ = into_iter.next().unwrap();

  assert_eq!(into_iter.as_slice(), &['b', 'c']);

  ```

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

  Returns the remaining items of this iterator as a mutable slice.

  

  # Examples

  

  ```rust

  let vec = vec!['a', 'b', 'c'];

  let mut into_iter = vec.into_iter();

  assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);

  into_iter.as_mut_slice()[2] = 'z';

  assert_eq!(into_iter.next().unwrap(), 'a');

  assert_eq!(into_iter.next().unwrap(), 'b');

  assert_eq!(into_iter.next().unwrap(), 'z');

  ```

- <span id="intoiter-allocator"></span>`fn allocator(&self) -> &A`

  Returns a reference to the underlying allocator.

- <span id="intoiter-as-raw-mut-slice"></span>`fn as_raw_mut_slice(&mut self) -> *mut [T]`

#### Trait Implementations

##### `impl<T, A: Allocator> AsRef for IntoIter<T, A>`

- <span id="intoiter-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug, A: Allocator> Debug for IntoIter<T, A>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for IntoIter<T, A>`

- <span id="intoiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A>`

##### `impl<T, A: Allocator> FusedIterator for IntoIter<T, A>`

##### `impl IntoIterator for IntoIter<T, A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for IntoIter<T, A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-count"></span>`fn count(self) -> usize`

##### `impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A>`

##### `impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A>`

