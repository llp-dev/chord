*[indexmap](../../index.md) / [set](../index.md) / [iter](index.md)*

---

# Module `iter`

## Contents

- [Structs](#structs)
  - [`Iter`](#iter)
  - [`IntoIter`](#intoiter)
  - [`Drain`](#drain)
  - [`Difference`](#difference)
  - [`Intersection`](#intersection)
  - [`SymmetricDifference`](#symmetricdifference)
  - [`Union`](#union)
  - [`Splice`](#splice)
  - [`UnitValue`](#unitvalue)
  - [`ExtractIf`](#extractif)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Iter`](#iter) | struct | An iterator over the items of an [`IndexSet`]. |
| [`IntoIter`](#intoiter) | struct | An owning iterator over the items of an [`IndexSet`]. |
| [`Drain`](#drain) | struct | A draining iterator over the items of an [`IndexSet`]. |
| [`Difference`](#difference) | struct | A lazy iterator producing elements in the difference of [`IndexSet`]s. |
| [`Intersection`](#intersection) | struct | A lazy iterator producing elements in the intersection of [`IndexSet`]s. |
| [`SymmetricDifference`](#symmetricdifference) | struct | A lazy iterator producing elements in the symmetric difference of [`IndexSet`]s. |
| [`Union`](#union) | struct | A lazy iterator producing elements in the union of [`IndexSet`]s. |
| [`Splice`](#splice) | struct | A splicing iterator for `IndexSet`. |
| [`UnitValue`](#unitvalue) | struct |  |
| [`ExtractIf`](#extractif) | struct | An extracting iterator for `IndexSet`. |

## Structs

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    iter: core::slice::Iter<'a, super::Bucket<T, ()>>,
}
```

An iterator over the items of an [`IndexSet`](../index.md).

This `struct` is created by the `IndexSet::iter` method.
See its documentation for more.

#### Implementations

- <span id="iter-new"></span>`fn new(entries: &'a [super::Bucket<T, ()>]) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="iter-as-slice"></span>`fn as_slice(&self) -> &'a Slice<T>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Iter<'_, T>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Iter<'_, T>`

- <span id="iter-default"></span>`fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Iter<'_, T>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Iter<'_, T>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for Iter<'_, T>`

##### `impl IntoIterator for Iter<'a, T>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Iter<'a, T>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a T`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="iter-iterator-count"></span>`fn count(self) -> usize`

- <span id="iter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="iter-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-collect"></span>`fn collect<C>(self) -> C`

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    iter: vec::IntoIter<super::Bucket<T, ()>>,
}
```

An owning iterator over the items of an [`IndexSet`](../index.md).

This `struct` is created by the `IndexSet::into_iter` method
(provided by the `IntoIterator` trait). See its documentation for more.

#### Implementations

- <span id="intoiter-new"></span>`fn new(entries: Vec<super::Bucket<T, ()>>) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &Slice<T>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T: fmt::Debug> Debug for IntoIter<T>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IntoIter<T>`

- <span id="intoiter-default"></span>`fn default() -> Self`

##### `impl<T> DoubleEndedIterator for IntoIter<T>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IntoIter<T>`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for IntoIter<T>`

##### `impl IntoIterator for IntoIter<T>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IntoIter<T>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-count"></span>`fn count(self) -> usize`

- <span id="intoiter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-collect"></span>`fn collect<C>(self) -> C`

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    iter: vec::Drain<'a, super::Bucket<T, ()>>,
}
```

A draining iterator over the items of an [`IndexSet`](../index.md).

This `struct` is created by the `IndexSet::drain` method.
See its documentation for more.

#### Implementations

- <span id="drain-new"></span>`fn new(iter: vec::Drain<'a, super::Bucket<T, ()>>) -> Self` — [`Bucket`](../../index.md#bucket)

- <span id="drain-as-slice"></span>`fn as_slice(&self) -> &Slice<T>` — [`Slice`](../slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Drain<'_, T>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DoubleEndedIterator for Drain<'_, T>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="drain-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Drain<'_, T>`

- <span id="drain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for Drain<'_, T>`

##### `impl IntoIterator for Drain<'a, T>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Drain<'_, T>`

- <span id="drain-iterator-type-item"></span>`type Item = T`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-count"></span>`fn count(self) -> usize`

- <span id="drain-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="drain-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="drain-iterator-collect"></span>`fn collect<C>(self) -> C`

### `Difference<'a, T, S>`

```rust
struct Difference<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a super::IndexSet<T, S>,
}
```

A lazy iterator producing elements in the difference of [`IndexSet`](../index.md)s.

This `struct` is created by the `IndexSet::difference` method.
See its documentation for more.

#### Implementations

- <span id="difference-new"></span>`fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self` — [`IndexSet`](../index.md#indexset)

#### Trait Implementations

##### `impl<T, S> Clone for Difference<'_, T, S>`

- <span id="difference-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for Difference<'_, T, S>`

- <span id="difference-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> DoubleEndedIterator for Difference<'_, T, S>`

- <span id="difference-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, S> FusedIterator for Difference<'_, T, S>`

##### `impl IntoIterator for Difference<'a, T, S>`

- <span id="difference-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="difference-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="difference-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S> Iterator for Difference<'a, T, S>`

- <span id="difference-iterator-type-item"></span>`type Item = &'a T`

- <span id="difference-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="difference-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Intersection<'a, T, S>`

```rust
struct Intersection<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a super::IndexSet<T, S>,
}
```

A lazy iterator producing elements in the intersection of [`IndexSet`](../index.md)s.

This `struct` is created by the `IndexSet::intersection` method.
See its documentation for more.

#### Implementations

- <span id="intersection-new"></span>`fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self` — [`IndexSet`](../index.md#indexset)

#### Trait Implementations

##### `impl<T, S> Clone for Intersection<'_, T, S>`

- <span id="intersection-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for Intersection<'_, T, S>`

- <span id="intersection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> DoubleEndedIterator for Intersection<'_, T, S>`

- <span id="intersection-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, S> FusedIterator for Intersection<'_, T, S>`

##### `impl IntoIterator for Intersection<'a, T, S>`

- <span id="intersection-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersection-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intersection-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S> Iterator for Intersection<'a, T, S>`

- <span id="intersection-iterator-type-item"></span>`type Item = &'a T`

- <span id="intersection-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intersection-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `SymmetricDifference<'a, T, S1, S2>`

```rust
struct SymmetricDifference<'a, T, S1, S2> {
    iter: core::iter::Chain<Difference<'a, T, S2>, Difference<'a, T, S1>>,
}
```

A lazy iterator producing elements in the symmetric difference of [`IndexSet`](../index.md)s.

This `struct` is created by the `IndexSet::symmetric_difference` method.
See its documentation for more.

#### Implementations

- <span id="symmetricdifference-new"></span>`fn new(set1: &'a IndexSet<T, S1>, set2: &'a IndexSet<T, S2>) -> Self` — [`IndexSet`](../index.md#indexset)

#### Trait Implementations

##### `impl<T, S1, S2> Clone for SymmetricDifference<'_, T, S1, S2>`

- <span id="symmetricdifference-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S1, S2> Debug for SymmetricDifference<'_, T, S1, S2>`

- <span id="symmetricdifference-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S1, S2> DoubleEndedIterator for SymmetricDifference<'_, T, S1, S2>`

- <span id="symmetricdifference-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="symmetricdifference-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, S1, S2> FusedIterator for SymmetricDifference<'_, T, S1, S2>`

##### `impl IntoIterator for SymmetricDifference<'a, T, S1, S2>`

- <span id="symmetricdifference-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symmetricdifference-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symmetricdifference-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S1, S2> Iterator for SymmetricDifference<'a, T, S1, S2>`

- <span id="symmetricdifference-iterator-type-item"></span>`type Item = &'a T`

- <span id="symmetricdifference-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="symmetricdifference-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="symmetricdifference-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `Union<'a, T, S>`

```rust
struct Union<'a, T, S> {
    iter: core::iter::Chain<Iter<'a, T>, Difference<'a, T, S>>,
}
```

A lazy iterator producing elements in the union of [`IndexSet`](../index.md)s.

This `struct` is created by the `IndexSet::union` method.
See its documentation for more.

#### Implementations

- <span id="union-new"></span>`fn new<S2>(set1: &'a IndexSet<T, S>, set2: &'a IndexSet<T, S2>) -> Self` — [`IndexSet`](../index.md#indexset)

#### Trait Implementations

##### `impl<T, S> Clone for Union<'_, T, S>`

- <span id="union-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for Union<'_, T, S>`

- <span id="union-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> DoubleEndedIterator for Union<'_, T, S>`

- <span id="union-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="union-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, S> FusedIterator for Union<'_, T, S>`

##### `impl IntoIterator for Union<'a, T, S>`

- <span id="union-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="union-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="union-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S> Iterator for Union<'a, T, S>`

- <span id="union-iterator-type-item"></span>`type Item = &'a T`

- <span id="union-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="union-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="union-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `Splice<'a, I, T, S>`

```rust
struct Splice<'a, I, T, S>
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
    S: BuildHasher {
    iter: crate::map::Splice<'a, UnitValue<I>, T, (), S>,
}
```

A splicing iterator for `IndexSet`.

This `struct` is created by `IndexSet::splice()`.
See its documentation for more.

#### Implementations

- <span id="splice-new"></span>`fn new<R>(set: &'a mut IndexSet<T, S>, range: R, replace_with: I) -> Self` — [`IndexSet`](../index.md#indexset)

#### Trait Implementations

##### `impl<I, T, S> Debug for Splice<'_, I, T, S>`

- <span id="splice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T, S> DoubleEndedIterator for Splice<'_, I, T, S>`

- <span id="splice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I, T, S> ExactSizeIterator for Splice<'_, I, T, S>`

- <span id="splice-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<I, T, S> FusedIterator for Splice<'_, I, T, S>`

##### `impl<I> IntoIterator for Splice<'a, I, T, S>`

- <span id="splice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splice-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T, S> Iterator for Splice<'_, I, T, S>`

- <span id="splice-iterator-type-item"></span>`type Item = T`

- <span id="splice-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="splice-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `UnitValue<I>`

```rust
struct UnitValue<I>(I);
```

#### Trait Implementations

##### `impl<I: fmt::Debug> Debug for UnitValue<I>`

- <span id="unitvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for UnitValue<I>`

- <span id="unitvalue-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unitvalue-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unitvalue-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator> Iterator for UnitValue<I>`

- <span id="unitvalue-iterator-type-item"></span>`type Item = (<I as Iterator>::Item, ())`

- <span id="unitvalue-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ExtractIf<'a, T, F>`

```rust
struct ExtractIf<'a, T, F> {
    inner: extract::ExtractCore<'a, T, ()>,
    pred: F,
}
```

An extracting iterator for `IndexSet`.

This `struct` is created by `IndexSet::extract_if()`.
See its documentation for more.

#### Implementations

- <span id="extractif-new"></span>`fn new<R>(core: &mut Core<T, ()>, range: R, pred: F) -> ExtractIf<'_, T, F>` — [`Core`](../../inner/index.md#core), [`ExtractIf`](#extractif)

#### Trait Implementations

##### `impl<T, F> Debug for ExtractIf<'_, T, F>`

- <span id="extractif-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F> FusedIterator for ExtractIf<'_, T, F>`

##### `impl IntoIterator for ExtractIf<'a, T, F>`

- <span id="extractif-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="extractif-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="extractif-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, F> Iterator for ExtractIf<'_, T, F>`

- <span id="extractif-iterator-type-item"></span>`type Item = T`

- <span id="extractif-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="extractif-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

