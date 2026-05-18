*[indexmap](../index.md) / [set](index.md)*

---

# Module `set`

A hash set implemented using [`IndexMap`](../map/index.md)

## Contents

- [Modules](#modules)
  - [`iter`](#iter)
  - [`mutable`](#mutable)
  - [`slice`](#slice)
- [Structs](#structs)
  - [`Difference`](#difference)
  - [`Drain`](#drain)
  - [`ExtractIf`](#extractif)
  - [`Intersection`](#intersection)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
  - [`Splice`](#splice)
  - [`SymmetricDifference`](#symmetricdifference)
  - [`Union`](#union)
  - [`Slice`](#slice)
  - [`IndexSet`](#indexset)
- [Traits](#traits)
  - [`MutableValues`](#mutablevalues)
- [Type Aliases](#type-aliases)
  - [`Bucket`](#bucket)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`iter`](#iter) | mod |  |
| [`mutable`](#mutable) | mod |  |
| [`slice`](#slice) | mod |  |
| [`Difference`](#difference) | struct |  |
| [`Drain`](#drain) | struct |  |
| [`ExtractIf`](#extractif) | struct |  |
| [`Intersection`](#intersection) | struct |  |
| [`IntoIter`](#intoiter) | struct |  |
| [`Iter`](#iter) | struct |  |
| [`Splice`](#splice) | struct |  |
| [`SymmetricDifference`](#symmetricdifference) | struct |  |
| [`Union`](#union) | struct |  |
| [`Slice`](#slice) | struct |  |
| [`IndexSet`](#indexset) | struct | A hash set where the iteration order of the values is independent of their hash values. |
| [`MutableValues`](#mutablevalues) | trait |  |
| [`Bucket`](#bucket) | type |  |

## Modules

- [`iter`](iter/index.md)
- [`mutable`](mutable/index.md)
- [`slice`](slice/index.md)

## Structs

### `Difference<'a, T, S>`

```rust
struct Difference<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a super::IndexSet<T, S>,
}
```

A lazy iterator producing elements in the difference of [`IndexSet`](#indexset)s.

This `struct` is created by the `IndexSet::difference` method.
See its documentation for more.

#### Implementations

- <span id="difference-new"></span>`fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self` — [`IndexSet`](#indexset)

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

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    iter: vec::Drain<'a, super::Bucket<T, ()>>,
}
```

A draining iterator over the items of an [`IndexSet`](#indexset).

This `struct` is created by the `IndexSet::drain` method.
See its documentation for more.

#### Implementations

- <span id="drain-new"></span>`fn new(iter: vec::Drain<'a, super::Bucket<T, ()>>) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="drain-as-slice"></span>`fn as_slice(&self) -> &Slice<T>` — [`Slice`](slice/index.md#slice)

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

- <span id="extractif-new"></span>`fn new<R>(core: &mut Core<T, ()>, range: R, pred: F) -> ExtractIf<'_, T, F>` — [`Core`](../inner/index.md#core), [`ExtractIf`](iter/index.md#extractif)

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

### `Intersection<'a, T, S>`

```rust
struct Intersection<'a, T, S> {
    iter: Iter<'a, T>,
    other: &'a super::IndexSet<T, S>,
}
```

A lazy iterator producing elements in the intersection of [`IndexSet`](#indexset)s.

This `struct` is created by the `IndexSet::intersection` method.
See its documentation for more.

#### Implementations

- <span id="intersection-new"></span>`fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self` — [`IndexSet`](#indexset)

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

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    iter: vec::IntoIter<super::Bucket<T, ()>>,
}
```

An owning iterator over the items of an [`IndexSet`](#indexset).

This `struct` is created by the `IndexSet::into_iter` method
(provided by the `IntoIterator` trait). See its documentation for more.

#### Implementations

- <span id="intoiter-new"></span>`fn new(entries: Vec<super::Bucket<T, ()>>) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &Slice<T>` — [`Slice`](slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T>` — [`IntoIter`](iter/index.md#intoiter)

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

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    iter: core::slice::Iter<'a, super::Bucket<T, ()>>,
}
```

An iterator over the items of an [`IndexSet`](#indexset).

This `struct` is created by the `IndexSet::iter` method.
See its documentation for more.

#### Implementations

- <span id="iter-new"></span>`fn new(entries: &'a [super::Bucket<T, ()>]) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="iter-as-slice"></span>`fn as_slice(&self) -> &'a Slice<T>` — [`Slice`](slice/index.md#slice)

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

- <span id="splice-new"></span>`fn new<R>(set: &'a mut IndexSet<T, S>, range: R, replace_with: I) -> Self` — [`IndexSet`](#indexset)

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

### `SymmetricDifference<'a, T, S1, S2>`

```rust
struct SymmetricDifference<'a, T, S1, S2> {
    iter: core::iter::Chain<Difference<'a, T, S2>, Difference<'a, T, S1>>,
}
```

A lazy iterator producing elements in the symmetric difference of [`IndexSet`](#indexset)s.

This `struct` is created by the `IndexSet::symmetric_difference` method.
See its documentation for more.

#### Implementations

- <span id="symmetricdifference-new"></span>`fn new(set1: &'a IndexSet<T, S1>, set2: &'a IndexSet<T, S2>) -> Self` — [`IndexSet`](#indexset)

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

A lazy iterator producing elements in the union of [`IndexSet`](#indexset)s.

This `struct` is created by the `IndexSet::union` method.
See its documentation for more.

#### Implementations

- <span id="union-new"></span>`fn new<S2>(set1: &'a IndexSet<T, S>, set2: &'a IndexSet<T, S2>) -> Self` — [`IndexSet`](#indexset)

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

### `Slice<T>`

```rust
struct Slice<T> {
    entries: [super::Bucket<T, ()>],
}
```

A dynamically-sized slice of values in an [`IndexSet`](#indexset).

This supports indexed operations much like a `[T]` slice,
but not any hashed operations on the values.

Unlike `IndexSet`, `Slice` does consider the order for `PartialEq`
and [`Eq`](../../base64ct/index.md), and it also implements `PartialOrd`, `Ord`, and [`Hash`](../../memchr/arch/all/rabinkarp/index.md).

#### Implementations

- <span id="slice-from-slice"></span>`const fn from_slice(entries: &[super::Bucket<T, ()>]) -> &Self` — [`Bucket`](../index.md#bucket)

- <span id="slice-from-boxed"></span>`fn from_boxed(entries: Box<[super::Bucket<T, ()>]>) -> Box<Self>` — [`Bucket`](../index.md#bucket)

- <span id="slice-into-boxed"></span>`fn into_boxed(self: Box<Self>) -> Box<[super::Bucket<T, ()>]>` — [`Bucket`](../index.md#bucket)

#### Trait Implementations

##### `impl<T: Clone> Clone for alloc::boxed::Box<Slice<T>>`

- <span id="allocboxedbox-clone"></span>`fn clone(&self) -> Self`

##### `impl<K> Comparable for Slice<T>`

- <span id="slice-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<T: fmt::Debug> Debug for Slice<T>`

- <span id="slice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for &Slice<T>`

- <span id="slice-default"></span>`fn default() -> Self`

##### `impl<T: Eq> Eq for Slice<T>`

##### `impl<K> Equivalent for Slice<T>`

- <span id="slice-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T: Hash> Hash for Slice<T>`

- <span id="slice-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T> Index for Slice<T>`

- <span id="slice-index-type-output"></span>`type Output = T`

- <span id="slice-index"></span>`fn index(&self, index: usize) -> &<Self as >::Output`

##### `impl<T> IntoIterator for &'a Slice<T>`

- <span id="a-slice-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-slice-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-slice-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T: Ord> Ord for Slice<T>`

- <span id="slice-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T, U> PartialEq for Slice<T>`

- <span id="slice-partialeq-eq"></span>`fn eq(&self, other: &Slice<U>) -> bool` — [`Slice`](slice/index.md#slice)

##### `impl<T: PartialOrd> PartialOrd for Slice<T>`

- <span id="slice-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

### `IndexSet<T, S>`

```rust
struct IndexSet<T, S> {
    map: super::IndexMap<T, (), S>,
}
```

A hash set where the iteration order of the values is independent of their
hash values.

The interface is closely compatible with the standard
`HashSet`,
but also has additional features.

# Order

The values have a consistent order that is determined by the sequence of
insertion and removal calls on the set. The order does not depend on the
values or the hash function at all. Note that insertion order and value
are not affected if a re-insertion is attempted once an element is
already present.

All iterators traverse the set *in order*.  Set operation iterators like
`IndexSet::union` produce a concatenated order, as do their matching "bitwise"
operators.  See their documentation for specifics.

The insertion order is preserved, with **notable exceptions** like the
`.remove()` or `.swap_remove()` methods.
Methods such as `.sort_by()` of
course result in a new order, depending on the sorting order.

# Indices

The values are indexed in a compact range without holes in the range
`0..self.len()`. For example, the method `.get_full` looks up the index for
a value, and the method `.get_index` looks up the value by index.

# Complexity

Internally, `IndexSet<T, S>` just holds an [`IndexMap<T, (), S>`](IndexMap). Thus the complexity
of the two are the same for most methods.

# Examples

```rust
use indexmap::IndexSet;

// Collects which letters appear in a sentence.
let letters: IndexSet<_> = "a short treatise on fungi".chars().collect();

assert!(letters.contains(&'s'));
assert!(letters.contains(&'t'));
assert!(letters.contains(&'u'));
assert!(!letters.contains(&'y'));
```

#### Implementations

- <span id="indexset-new"></span>`fn new() -> Self`

  Create a new set. (Does not allocate.)

- <span id="indexset-with-capacity"></span>`fn with_capacity(n: usize) -> Self`

  Create a new set with capacity for `n` elements.

  (Does not allocate if `n` is zero.)

  

  Computes in **O(n)** time.

#### Trait Implementations

##### `impl<T, S1, S2> BitAnd for &IndexSet<T, S1>`

- <span id="indexset-bitand-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-bitand"></span>`fn bitand(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](#indexset)

  Returns the set intersection, cloned into a new set.

  

  Values are collected in the same order that they appear in `self`.

##### `impl<T, S1, S2> BitOr for &IndexSet<T, S1>`

- <span id="indexset-bitor-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-bitor"></span>`fn bitor(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](#indexset)

  Returns the set union, cloned into a new set.

  

  Values from `self` are collected in their original order, followed by

  values that are unique to `other` in their original order.

##### `impl<T, S1, S2> BitXor for &IndexSet<T, S1>`

- <span id="indexset-bitxor-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-bitxor"></span>`fn bitxor(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](#indexset)

  Returns the set symmetric-difference, cloned into a new set.

  

  Values from `self` are collected in their original order, followed by

  values from `other` in their original order.

##### `impl<T, S> Clone for IndexSet<T, S>`

- <span id="indexset-clone"></span>`fn clone(&self) -> Self`

- <span id="indexset-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<T, S> Debug for IndexSet<T, S>`

- <span id="indexset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for IndexSet<T, S>`

- <span id="indexset-default"></span>`fn default() -> Self`

  Return an empty [`IndexSet`](#indexset)

##### `impl<T, S> Eq for IndexSet<T, S>`

##### `impl<K> Equivalent for IndexSet<T, S>`

- <span id="indexset-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T, S> Extend for IndexSet<T, S>`

- <span id="indexset-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I)`

##### `impl<T, S> FromIterator for IndexSet<T, S>`

- <span id="indexset-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self`

##### `impl<T, S> Index for IndexSet<T, S>`

- <span id="indexset-index-type-output"></span>`type Output = Slice<T>`

- <span id="indexset-index"></span>`fn index(&self, range: ops::Range<usize>) -> &<Self as >::Output`

##### `impl<T, S> IntoIterator for &'a super::IndexSet<T, S>`

- <span id="a-superindexset-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-superindexset-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-superindexset-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, S> MutableValues for super::IndexSet<T, S>`

- <span id="superindexset-mutablevalues-type-value"></span>`type Value = T`

- <span id="superindexset-mutablevalues-get-full-mut2"></span>`fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut T)>`

- <span id="superindexset-mutablevalues-get-index-mut2"></span>`fn get_index_mut2(&mut self, index: usize) -> Option<&mut T>`

- <span id="superindexset-mutablevalues-retain2"></span>`fn retain2<F>(&mut self, keep: F)`

##### `impl<T, S1, S2> PartialEq for IndexSet<T, S1>`

- <span id="indexset-partialeq-eq"></span>`fn eq(&self, other: &IndexSet<T, S2>) -> bool` — [`IndexSet`](#indexset)

##### `impl<T, S> Sealed for super::IndexSet<T, S>`

##### `impl<T, S1, S2> Sub for &IndexSet<T, S1>`

- <span id="indexset-sub-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-sub"></span>`fn sub(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](#indexset)

  Returns the set difference, cloned into a new set.

  

  Values are collected in the same order that they appear in `self`.

## Traits

### `MutableValues`

```rust
trait MutableValues: Sealed { ... }
```

Opt-in mutable access to [`IndexSet`](#indexset) values.

These methods expose `&mut T`, mutable references to the value as it is stored
in the set.
You are allowed to modify the values in the set **if the modification
does not change the value's hash and equality**.

If values are modified erroneously, you can no longer look them up.
This is sound (memory safe) but a logical error hazard (just like
implementing `PartialEq`, `Eq`, or `Hash` incorrectly would be).

`use` this trait to enable its methods for `IndexSet`.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Value`

#### Required Methods

- `fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut <Self as >::Value)>`

  Return item index and mutable reference to the value

- `fn get_index_mut2(&mut self, index: usize) -> Option<&mut <Self as >::Value>`

  Return mutable reference to the value at an index.

- `fn retain2<F>(&mut self, keep: F)`

  Scan through each value in the set and keep those where the

#### Implementors

- [`IndexSet`](#indexset)

## Type Aliases

### `Bucket<T>`

```rust
type Bucket<T> = super::Bucket<T, ()>;
```

