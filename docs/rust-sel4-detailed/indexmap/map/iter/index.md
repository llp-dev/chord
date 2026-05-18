*[indexmap](../../index.md) / [map](../index.md) / [iter](index.md)*

---

# Module `iter`

## Contents

- [Structs](#structs)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`IterMut2`](#itermut2)
  - [`IntoIter`](#intoiter)
  - [`Drain`](#drain)
  - [`Keys`](#keys)
  - [`IntoKeys`](#intokeys)
  - [`Values`](#values)
  - [`ValuesMut`](#valuesmut)
  - [`IntoValues`](#intovalues)
  - [`Splice`](#splice)
  - [`ExtractIf`](#extractif)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Iter`](#iter) | struct | An iterator over the entries of an [`IndexMap`]. |
| [`IterMut`](#itermut) | struct | A mutable iterator over the entries of an [`IndexMap`]. |
| [`IterMut2`](#itermut2) | struct | A mutable iterator over the entries of an [`IndexMap`]. |
| [`IntoIter`](#intoiter) | struct | An owning iterator over the entries of an [`IndexMap`]. |
| [`Drain`](#drain) | struct | A draining iterator over the entries of an [`IndexMap`]. |
| [`Keys`](#keys) | struct | An iterator over the keys of an [`IndexMap`]. |
| [`IntoKeys`](#intokeys) | struct | An owning iterator over the keys of an [`IndexMap`]. |
| [`Values`](#values) | struct | An iterator over the values of an [`IndexMap`]. |
| [`ValuesMut`](#valuesmut) | struct | A mutable iterator over the values of an [`IndexMap`]. |
| [`IntoValues`](#intovalues) | struct | An owning iterator over the values of an [`IndexMap`]. |
| [`Splice`](#splice) | struct | A splicing iterator for `IndexMap`. |
| [`ExtractIf`](#extractif) | struct | An extracting iterator for `IndexMap`. |

## Structs

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    iter: slice::Iter<'a, crate::Bucket<K, V>>,
}
```

An iterator over the entries of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::iter` method.
See its documentation for more.

#### Implementations

- <span id="iter-new"></span>`fn new(entries: &'a [Bucket<K, V>]) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="iter-as-slice"></span>`fn as_slice(&self) -> &'a Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<K, V> Clone for Iter<'_, K, V>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Iter<'_, K, V>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Iter<'_, K, V>`

- <span id="iter-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for Iter<'_, K, V>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for Iter<'_, K, V>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for Iter<'_, K, V>`

##### `impl IntoIterator for Iter<'a, K, V>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Iter<'a, K, V>`

- <span id="iter-iterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="iter-iterator-count"></span>`fn count(self) -> usize`

- <span id="iter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="iter-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-collect"></span>`fn collect<C>(self) -> C`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    iter: slice::IterMut<'a, crate::Bucket<K, V>>,
}
```

A mutable iterator over the entries of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::iter_mut` method.
See its documentation for more.

#### Implementations

- <span id="itermut-new"></span>`fn new(entries: &'a mut [Bucket<K, V>]) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="itermut-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

- <span id="itermut-into-slice"></span>`fn into_slice(self) -> &'a mut Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a mutable slice of the remaining entries in the iterator.

  

  To avoid creating `&mut` references that alias, this is forced to consume the iterator.

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IterMut<'_, K, V>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for IterMut<'_, K, V>`

- <span id="itermut-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for IterMut<'_, K, V>`

- <span id="itermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for IterMut<'_, K, V>`

- <span id="itermut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for IterMut<'_, K, V>`

##### `impl IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IterMut<'a, K, V>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="itermut-iterator-count"></span>`fn count(self) -> usize`

- <span id="itermut-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-collect"></span>`fn collect<C>(self) -> C`

### `IterMut2<'a, K, V>`

```rust
struct IterMut2<'a, K, V> {
    iter: slice::IterMut<'a, crate::Bucket<K, V>>,
}
```

A mutable iterator over the entries of an [`IndexMap`](../index.md).

This `struct` is created by the `MutableKeys::iter_mut2` method.
See its documentation for more.

#### Implementations

- <span id="itermut2-new"></span>`fn new(entries: &'a mut [Bucket<K, V>]) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="itermut2-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

- <span id="itermut2-into-slice"></span>`fn into_slice(self) -> &'a mut Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a mutable slice of the remaining entries in the iterator.

  

  To avoid creating `&mut` references that alias, this is forced to consume the iterator.

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IterMut2<'_, K, V>`

- <span id="itermut2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for IterMut2<'_, K, V>`

- <span id="itermut2-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for IterMut2<'_, K, V>`

- <span id="itermut2-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut2-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for IterMut2<'_, K, V>`

- <span id="itermut2-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for IterMut2<'_, K, V>`

##### `impl IntoIterator for IterMut2<'a, K, V>`

- <span id="itermut2-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut2-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut2-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IterMut2<'a, K, V>`

- <span id="itermut2-iterator-type-item"></span>`type Item = (&'a mut K, &'a mut V)`

- <span id="itermut2-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut2-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="itermut2-iterator-count"></span>`fn count(self) -> usize`

- <span id="itermut2-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="itermut2-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="itermut2-iterator-collect"></span>`fn collect<C>(self) -> C`

### `IntoIter<K, V>`

```rust
struct IntoIter<K, V> {
    iter: vec::IntoIter<crate::Bucket<K, V>>,
}
```

An owning iterator over the entries of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::into_iter` method
(provided by the `IntoIterator` trait). See its documentation for more.

#### Implementations

- <span id="intoiter-new"></span>`fn new(entries: Vec<Bucket<K, V>>) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a mutable slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<K: clone::Clone, V: clone::Clone> Clone for IntoIter<K, V>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<K, V>` — [`IntoIter`](#intoiter)

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IntoIter<K, V>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for IntoIter<K, V>`

- <span id="intoiter-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for IntoIter<K, V>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for IntoIter<K, V>`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for IntoIter<K, V>`

##### `impl IntoIterator for IntoIter<K, V>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IntoIter<K, V>`

- <span id="intoiter-iterator-type-item"></span>`type Item = (K, V)`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-count"></span>`fn count(self) -> usize`

- <span id="intoiter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-collect"></span>`fn collect<C>(self) -> C`

### `Drain<'a, K, V>`

```rust
struct Drain<'a, K, V> {
    iter: vec::Drain<'a, crate::Bucket<K, V>>,
}
```

A draining iterator over the entries of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::drain` method.
See its documentation for more.

#### Implementations

- <span id="drain-new"></span>`fn new(iter: vec::Drain<'a, Bucket<K, V>>) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="drain-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Drain<'_, K, V>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> DoubleEndedIterator for Drain<'_, K, V>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="drain-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for Drain<'_, K, V>`

- <span id="drain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for Drain<'_, K, V>`

##### `impl IntoIterator for Drain<'a, K, V>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Drain<'_, K, V>`

- <span id="drain-iterator-type-item"></span>`type Item = (K, V)`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-count"></span>`fn count(self) -> usize`

- <span id="drain-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="drain-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="drain-iterator-collect"></span>`fn collect<C>(self) -> C`

### `Keys<'a, K, V>`

```rust
struct Keys<'a, K, V> {
    iter: slice::Iter<'a, crate::Bucket<K, V>>,
}
```

An iterator over the keys of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::keys` method.
See its documentation for more.

#### Implementations

- <span id="keys-new"></span>`fn new(entries: &'a [Bucket<K, V>]) -> Self` — [`Bucket`](../../index.md#bucket)

#### Trait Implementations

##### `impl<K, V> Clone for Keys<'_, K, V>`

- <span id="keys-clone"></span>`fn clone(&self) -> Self`

##### `impl<K: fmt::Debug, V> Debug for Keys<'_, K, V>`

- <span id="keys-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Keys<'_, K, V>`

- <span id="keys-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for Keys<'_, K, V>`

- <span id="keys-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="keys-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for Keys<'_, K, V>`

- <span id="keys-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for Keys<'_, K, V>`

##### `impl<K, V> Index for Keys<'_, K, V>`

- <span id="keys-index-type-output"></span>`type Output = K`

- <span id="keys-index"></span>`fn index(&self, index: usize) -> &K`

  Returns a reference to the key at the supplied `index`.

  

  ***Panics*** if `index` is out of bounds.

##### `impl IntoIterator for Keys<'a, K, V>`

- <span id="keys-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="keys-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="keys-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Keys<'a, K, V>`

- <span id="keys-iterator-type-item"></span>`type Item = &'a K`

- <span id="keys-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="keys-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="keys-iterator-count"></span>`fn count(self) -> usize`

- <span id="keys-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="keys-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="keys-iterator-collect"></span>`fn collect<C>(self) -> C`

### `IntoKeys<K, V>`

```rust
struct IntoKeys<K, V> {
    iter: vec::IntoIter<crate::Bucket<K, core::mem::MaybeUninit<V>>>,
}
```

An owning iterator over the keys of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::into_keys` method.
See its documentation for more.

#### Implementations

- <span id="intokeys-new"></span>`fn new(entries: Vec<Bucket<K, V>>) -> Self` — [`Bucket`](../../index.md#bucket)

#### Trait Implementations

##### `impl<K: Clone, V> Clone for IntoKeys<K, V>`

- <span id="intokeys-clone"></span>`fn clone(&self) -> Self`

##### `impl<K: fmt::Debug, V> Debug for IntoKeys<K, V>`

- <span id="intokeys-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for IntoKeys<K, V>`

- <span id="intokeys-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for IntoKeys<K, V>`

- <span id="intokeys-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="intokeys-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for IntoKeys<K, V>`

- <span id="intokeys-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for IntoKeys<K, V>`

##### `impl IntoIterator for IntoKeys<K, V>`

- <span id="intokeys-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intokeys-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intokeys-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IntoKeys<K, V>`

- <span id="intokeys-iterator-type-item"></span>`type Item = K`

- <span id="intokeys-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intokeys-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intokeys-iterator-count"></span>`fn count(self) -> usize`

- <span id="intokeys-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="intokeys-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="intokeys-iterator-collect"></span>`fn collect<C>(self) -> C`

### `Values<'a, K, V>`

```rust
struct Values<'a, K, V> {
    iter: slice::Iter<'a, crate::Bucket<K, V>>,
}
```

An iterator over the values of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::values` method.
See its documentation for more.

#### Implementations

- <span id="values-new"></span>`fn new(entries: &'a [Bucket<K, V>]) -> Self` — [`Bucket`](../../index.md#bucket)

#### Trait Implementations

##### `impl<K, V> Clone for Values<'_, K, V>`

- <span id="values-clone"></span>`fn clone(&self) -> Self`

##### `impl<K, V: fmt::Debug> Debug for Values<'_, K, V>`

- <span id="values-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Values<'_, K, V>`

- <span id="values-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for Values<'_, K, V>`

- <span id="values-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="values-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for Values<'_, K, V>`

- <span id="values-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for Values<'_, K, V>`

##### `impl IntoIterator for Values<'a, K, V>`

- <span id="values-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="values-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Values<'a, K, V>`

- <span id="values-iterator-type-item"></span>`type Item = &'a V`

- <span id="values-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="values-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="values-iterator-count"></span>`fn count(self) -> usize`

- <span id="values-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="values-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="values-iterator-collect"></span>`fn collect<C>(self) -> C`

### `ValuesMut<'a, K, V>`

```rust
struct ValuesMut<'a, K, V> {
    iter: slice::IterMut<'a, crate::Bucket<K, V>>,
}
```

A mutable iterator over the values of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::values_mut` method.
See its documentation for more.

#### Implementations

- <span id="valuesmut-new"></span>`fn new(entries: &'a mut [Bucket<K, V>]) -> Self` — [`Bucket`](../../index.md#bucket)

#### Trait Implementations

##### `impl<K, V: fmt::Debug> Debug for ValuesMut<'_, K, V>`

- <span id="valuesmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for ValuesMut<'_, K, V>`

- <span id="valuesmut-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for ValuesMut<'_, K, V>`

- <span id="valuesmut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="valuesmut-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V>`

- <span id="valuesmut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for ValuesMut<'_, K, V>`

##### `impl IntoIterator for ValuesMut<'a, K, V>`

- <span id="valuesmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for ValuesMut<'a, K, V>`

- <span id="valuesmut-iterator-type-item"></span>`type Item = &'a mut V`

- <span id="valuesmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="valuesmut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="valuesmut-iterator-count"></span>`fn count(self) -> usize`

- <span id="valuesmut-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="valuesmut-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="valuesmut-iterator-collect"></span>`fn collect<C>(self) -> C`

### `IntoValues<K, V>`

```rust
struct IntoValues<K, V> {
    iter: vec::IntoIter<crate::Bucket<core::mem::MaybeUninit<K>, V>>,
}
```

An owning iterator over the values of an [`IndexMap`](../index.md).

This `struct` is created by the `IndexMap::into_values` method.
See its documentation for more.

#### Implementations

- <span id="intovalues-new"></span>`fn new(entries: Vec<Bucket<K, V>>) -> Self` — [`Bucket`](../../index.md#bucket)

#### Trait Implementations

##### `impl<K, V: Clone> Clone for IntoValues<K, V>`

- <span id="intovalues-clone"></span>`fn clone(&self) -> Self`

##### `impl<K, V: fmt::Debug> Debug for IntoValues<K, V>`

- <span id="intovalues-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for IntoValues<K, V>`

- <span id="intovalues-default"></span>`fn default() -> Self`

##### `impl<K, V> DoubleEndedIterator for IntoValues<K, V>`

- <span id="intovalues-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="intovalues-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<K, V> ExactSizeIterator for IntoValues<K, V>`

- <span id="intovalues-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<K, V> FusedIterator for IntoValues<K, V>`

##### `impl IntoIterator for IntoValues<K, V>`

- <span id="intovalues-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intovalues-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intovalues-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IntoValues<K, V>`

- <span id="intovalues-iterator-type-item"></span>`type Item = V`

- <span id="intovalues-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intovalues-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intovalues-iterator-count"></span>`fn count(self) -> usize`

- <span id="intovalues-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="intovalues-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="intovalues-iterator-collect"></span>`fn collect<C>(self) -> C`

### `Splice<'a, I, K, V, S>`

```rust
struct Splice<'a, I, K, V, S>
where
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq,
    S: BuildHasher {
    map: &'a mut super::IndexMap<K, V, S>,
    tail: crate::inner::Core<K, V>,
    drain: vec::IntoIter<crate::Bucket<K, V>>,
    replace_with: I,
}
```

A splicing iterator for `IndexMap`.

This `struct` is created by `IndexMap::splice()`.
See its documentation for more.

#### Implementations

- <span id="splice-new"></span>`fn new<R>(map: &'a mut IndexMap<K, V, S>, range: R, replace_with: I) -> Self` — [`IndexMap`](../index.md#indexmap)

#### Trait Implementations

##### `impl<I, K, V, S> Debug for Splice<'_, I, K, V, S>`

- <span id="splice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, K, V, S> DoubleEndedIterator for Splice<'_, I, K, V, S>`

- <span id="splice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I, K, V, S> Drop for Splice<'_, I, K, V, S>`

- <span id="splice-drop"></span>`fn drop(&mut self)`

##### `impl<I, K, V, S> ExactSizeIterator for Splice<'_, I, K, V, S>`

- <span id="splice-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<I, K, V, S> FusedIterator for Splice<'_, I, K, V, S>`

##### `impl<I> IntoIterator for Splice<'a, I, K, V, S>`

- <span id="splice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splice-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, K, V, S> Iterator for Splice<'_, I, K, V, S>`

- <span id="splice-iterator-type-item"></span>`type Item = (K, V)`

- <span id="splice-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="splice-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `ExtractIf<'a, K, V, F>`

```rust
struct ExtractIf<'a, K, V, F> {
    inner: extract::ExtractCore<'a, K, V>,
    pred: F,
}
```

An extracting iterator for `IndexMap`.

This `struct` is created by `IndexMap::extract_if()`.
See its documentation for more.

#### Implementations

- <span id="extractif-new"></span>`fn new<R>(core: &mut Core<K, V>, range: R, pred: F) -> ExtractIf<'_, K, V, F>` — [`Core`](../../inner/index.md#core), [`ExtractIf`](#extractif)

#### Trait Implementations

##### `impl<K, V, F> Debug for ExtractIf<'_, K, V, F>`

- <span id="extractif-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, F> FusedIterator for ExtractIf<'_, K, V, F>`

##### `impl IntoIterator for ExtractIf<'a, K, V, F>`

- <span id="extractif-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="extractif-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="extractif-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V, F> Iterator for ExtractIf<'_, K, V, F>`

- <span id="extractif-iterator-type-item"></span>`type Item = (K, V)`

- <span id="extractif-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="extractif-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

