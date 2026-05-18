*[generic_array](../index.md) / [iter](index.md)*

---

# Module `iter`

`GenericArray` iterator implementation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GenericArrayIter`](#genericarrayiter) | struct | An iterator that moves out of a `GenericArray` |

## Structs

### `GenericArrayIter<T, N: ArrayLength<T>>`

```rust
struct GenericArrayIter<T, N: ArrayLength<T>> {
    array: core::mem::ManuallyDrop<super::GenericArray<T, N>>,
    index: usize,
    index_back: usize,
}
```

An iterator that moves out of a `GenericArray`

#### Implementations

- <span id="genericarrayiter-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Returns the remaining items of this iterator as a slice

- <span id="genericarrayiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

  Returns the remaining items of this iterator as a mutable slice

#### Trait Implementations

##### `impl<T: Clone, N> Clone for GenericArrayIter<T, N>`

- <span id="genericarrayiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug, N> Debug for GenericArrayIter<T, N>`

- <span id="genericarrayiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, N> DoubleEndedIterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

- <span id="genericarrayiter-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, N> Drop for GenericArrayIter<T, N>`

- <span id="genericarrayiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, N> ExactSizeIterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T, N> FusedIterator for GenericArrayIter<T, N>`

##### `impl IntoIterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="genericarrayiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="genericarrayiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, N> Iterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-iterator-type-item"></span>`type Item = T`

- <span id="genericarrayiter-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="genericarrayiter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

- <span id="genericarrayiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="genericarrayiter-iterator-count"></span>`fn count(self) -> usize`

- <span id="genericarrayiter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<T>`

- <span id="genericarrayiter-iterator-last"></span>`fn last(self) -> Option<T>`

##### `impl<T> Same for GenericArrayIter<T, N>`

- <span id="genericarrayiter-same-type-output"></span>`type Output = T`

