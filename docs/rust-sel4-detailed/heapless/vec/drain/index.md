*[heapless](../../index.md) / [vec](../index.md) / [drain](index.md)*

---

# Module `drain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | A draining iterator for [`Vec`](super::Vec). |

## Structs

### `Drain<'a, T: 'a, LenT: LenType>`

```rust
struct Drain<'a, T: 'a, LenT: LenType> {
    tail_start: LenT,
    tail_len: LenT,
    iter: slice::Iter<'a, T>,
    vec: core::ptr::NonNull<super::VecView<T, LenT>>,
}
```

A draining iterator for [`Vec`](super::Vec).

This `struct` is created by [`Vec::drain`](super::Vec::drain).
See its documentation for more.

#### Fields

- **`tail_start`**: `LenT`

  Index of tail to preserve

- **`tail_len`**: `LenT`

  Length of tail

- **`iter`**: `slice::Iter<'a, T>`

  Current remaining range to remove

#### Implementations

- <span id="drain-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Returns the remaining items of this iterator as a slice.

  

  # Examples

  

  ```rust

  use heapless::{vec, Vec};

  

  let mut vec = Vec::<_, 3>::from_array(['a', 'b', 'c']);

  let mut drain = vec.drain(..);

  assert_eq!(drain.as_slice(), &['a', 'b', 'c']);

  let _ = drain.next().unwrap();

  assert_eq!(drain.as_slice(), &['b', 'c']);

  ```

#### Trait Implementations

##### `impl<T, LenT: LenType> AsRef for Drain<'_, T, LenT>`

- <span id="drain-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: fmt::Debug, LenT: LenType> Debug for Drain<'_, T, LenT>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, LenT: LenType> DoubleEndedIterator for Drain<'_, T, LenT>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, LenT: LenType> Drop for Drain<'_, T, LenT>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T, LenT: LenType> ExactSizeIterator for Drain<'_, T, LenT>`

##### `impl<T, LenT: LenType> FusedIterator for Drain<'_, T, LenT>`

##### `impl IntoIterator for Drain<'a, T, LenT>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, LenT: LenType> Iterator for Drain<'_, T, LenT>`

- <span id="drain-iterator-type-item"></span>`type Item = T`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T: Send, LenT: LenType> Send for Drain<'_, T, LenT>`

##### `impl<T: Sync, LenT: LenType> Sync for Drain<'_, T, LenT>`

