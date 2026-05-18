*[indexmap](../index.md) / [map](index.md)*

---

# Module `map`

[`IndexMap`](#indexmap) is a hash table where the iteration order of the key-value
pairs is independent of the hash values of the keys.

## Contents

- [Modules](#modules)
  - [`entry`](#entry)
  - [`iter`](#iter)
  - [`mutable`](#mutable)
  - [`slice`](#slice)
  - [`raw_entry_v1`](#raw-entry-v1)
- [Structs](#structs)
  - [`IndexedEntry`](#indexedentry)
  - [`OccupiedEntry`](#occupiedentry)
  - [`VacantEntry`](#vacantentry)
  - [`Drain`](#drain)
  - [`ExtractIf`](#extractif)
  - [`IntoIter`](#intoiter)
  - [`IntoKeys`](#intokeys)
  - [`IntoValues`](#intovalues)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`IterMut2`](#itermut2)
  - [`Keys`](#keys)
  - [`Splice`](#splice)
  - [`Values`](#values)
  - [`ValuesMut`](#valuesmut)
  - [`Slice`](#slice)
  - [`IndexMap`](#indexmap)
- [Enums](#enums)
  - [`Entry`](#entry)
- [Traits](#traits)
  - [`MutableEntryKey`](#mutableentrykey)
  - [`MutableKeys`](#mutablekeys)
  - [`RawEntryApiV1`](#rawentryapiv1)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`entry`](#entry) | mod |  |
| [`iter`](#iter) | mod |  |
| [`mutable`](#mutable) | mod |  |
| [`slice`](#slice) | mod |  |
| [`raw_entry_v1`](#raw-entry-v1) | mod | Opt-in access to the experimental raw entry API. |
| [`IndexedEntry`](#indexedentry) | struct |  |
| [`OccupiedEntry`](#occupiedentry) | struct |  |
| [`VacantEntry`](#vacantentry) | struct |  |
| [`Drain`](#drain) | struct |  |
| [`ExtractIf`](#extractif) | struct |  |
| [`IntoIter`](#intoiter) | struct |  |
| [`IntoKeys`](#intokeys) | struct |  |
| [`IntoValues`](#intovalues) | struct |  |
| [`Iter`](#iter) | struct |  |
| [`IterMut`](#itermut) | struct |  |
| [`IterMut2`](#itermut2) | struct |  |
| [`Keys`](#keys) | struct |  |
| [`Splice`](#splice) | struct |  |
| [`Values`](#values) | struct |  |
| [`ValuesMut`](#valuesmut) | struct |  |
| [`Slice`](#slice) | struct |  |
| [`IndexMap`](#indexmap) | struct | A hash table where the iteration order of the key-value pairs is independent of the hash values of the keys. |
| [`Entry`](#entry) | enum |  |
| [`MutableEntryKey`](#mutableentrykey) | trait |  |
| [`MutableKeys`](#mutablekeys) | trait |  |
| [`RawEntryApiV1`](#rawentryapiv1) | trait |  |

## Modules

- [`entry`](entry/index.md)
- [`iter`](iter/index.md)
- [`mutable`](mutable/index.md)
- [`slice`](slice/index.md)
- [`raw_entry_v1`](raw_entry_v1/index.md) — Opt-in access to the experimental raw entry API.

## Structs

### `IndexedEntry<'a, K, V>`

```rust
struct IndexedEntry<'a, K, V> {
    map: &'a mut crate::inner::Core<K, V>,
    index: usize,
}
```

A view into an occupied entry in an `IndexMap` obtained by index.

This `struct` is created from the `get_index_entry` method.

#### Implementations

- <span id="indexedentry-new"></span>`fn new(map: &'a mut Core<K, V>, index: usize) -> Option<Self>` — [`Core`](../inner/index.md#core)

- <span id="indexedentry-index"></span>`fn index(&self) -> usize`

  Return the index of the key-value pair

- <span id="indexedentry-into-core"></span>`fn into_core(self) -> &'a mut Core<K, V>` — [`Core`](../inner/index.md#core)

- <span id="indexedentry-get-bucket"></span>`fn get_bucket(&self) -> &Bucket<K, V>` — [`Bucket`](../index.md#bucket)

- <span id="indexedentry-get-bucket-mut"></span>`fn get_bucket_mut(&mut self) -> &mut Bucket<K, V>` — [`Bucket`](../index.md#bucket)

- <span id="indexedentry-into-bucket"></span>`fn into_bucket(self) -> &'a mut Bucket<K, V>` — [`Bucket`](../index.md#bucket)

- <span id="indexedentry-key"></span>`fn key(&self) -> &K`

  Gets a reference to the entry's key in the map.

- <span id="indexedentry-key-mut"></span>`fn key_mut(&mut self) -> &mut K`

- <span id="indexedentry-get"></span>`fn get(&self) -> &V`

  Gets a reference to the entry's value in the map.

- <span id="indexedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut V`

  Gets a mutable reference to the entry's value in the map.

  

  If you need a reference which may outlive the destruction of the

  `IndexedEntry` value, see `into_mut`.

- <span id="indexedentry-insert"></span>`fn insert(&mut self, value: V) -> V`

  Sets the value of the entry to `value`, and returns the entry's old value.

- <span id="indexedentry-into-mut"></span>`fn into_mut(self) -> &'a mut V`

  Converts into a mutable reference to the entry's value in the map,

  with a lifetime bound to the map itself.

- <span id="indexedentry-swap-remove-entry"></span>`fn swap_remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  Like `Vec::swap_remove`, the pair is removed by swapping it

  with the last element of the map and popping it off.

  **This perturbs the position of what used to be the last element!**

  

  Computes in **O(1)** time (average).

- <span id="indexedentry-shift-remove-entry"></span>`fn shift_remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  Like `Vec::remove`, the pair is removed by shifting all of the

  elements that follow it, preserving their relative order.

  **This perturbs the index of all of those elements!**

  

  Computes in **O(n)** time (average).

- <span id="indexedentry-swap-remove"></span>`fn swap_remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  Like `Vec::swap_remove`, the pair is removed by swapping it

  with the last element of the map and popping it off.

  **This perturbs the position of what used to be the last element!**

  

  Computes in **O(1)** time (average).

- <span id="indexedentry-shift-remove"></span>`fn shift_remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  Like `Vec::remove`, the pair is removed by shifting all of the

  elements that follow it, preserving their relative order.

  **This perturbs the index of all of those elements!**

  

  Computes in **O(n)** time (average).

- <span id="indexedentry-move-index"></span>`fn move_index(self, to: usize)`

  Moves the position of the entry to a new index

  by shifting all other entries in-between.

  

  This is equivalent to [`IndexMap::move_index`][`crate::IndexMap::move_index`](#move-index)

  coming `from` the current `.index()`.

  

  * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.

  * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.

  

  ***Panics*** if `to` is out of bounds.

  

  Computes in **O(n)** time (average).

- <span id="indexedentry-swap-indices"></span>`fn swap_indices(self, other: usize)`

  Swaps the position of entry with another.

  

  This is equivalent to [`IndexMap::swap_indices`][`crate::IndexMap::swap_indices`](#swap-indices)

  with the current `.index()` as one of the two being swapped.

  

  ***Panics*** if the `other` index is out of bounds.

  

  Computes in **O(1)** time (average).

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IndexedEntry<'_, K, V>`

- <span id="indexedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::IndexedEntry<'_, K, V>`

- <span id="superindexedentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="superindexedentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](mutable/index.md#mutableentrykey)

##### `impl<K, V> Sealed for super::IndexedEntry<'_, K, V>`

### `OccupiedEntry<'a, K, V>`

```rust
struct OccupiedEntry<'a, K, V> {
    map: &'a mut super::Core<K, V>,
    index: usize,
    bucket: usize,
}
```

A view into an occupied entry in an `IndexMap`.
It is part of the [`Entry`](entry/index.md) enum.

#### Implementations

- <span id="occupiedentry-from-hash"></span>`fn from_hash<F>(map: &'a mut Core<K, V>, hash: HashValue, is_match: F) -> Result<Self, &'a mut Core<K, V>>` — [`Core`](../inner/index.md#core), [`HashValue`](../index.md#hashvalue)

  Constructor for `RawEntryMut::from_hash`

- <span id="occupiedentry-into-core"></span>`fn into_core(self) -> &'a mut Core<K, V>` — [`Core`](../inner/index.md#core)

- <span id="occupiedentry-get-bucket"></span>`fn get_bucket(&self) -> &Bucket<K, V>` — [`Bucket`](../index.md#bucket)

- <span id="occupiedentry-get-bucket-mut"></span>`fn get_bucket_mut(&mut self) -> &mut Bucket<K, V>` — [`Bucket`](../index.md#bucket)

- <span id="occupiedentry-into-bucket"></span>`fn into_bucket(self) -> &'a mut Bucket<K, V>` — [`Bucket`](../index.md#bucket)

- <span id="occupiedentry-index"></span>`fn index(&self) -> usize`

  Return the index of the key-value pair

- <span id="occupiedentry-key"></span>`fn key(&self) -> &K`

  Gets a reference to the entry's key in the map.

  

  Note that this is not the key that was used to find the entry. There may be an observable

  difference if the key type has any distinguishing features outside of `Hash` and `Eq`, like

  extra fields or the memory address of an allocation.

- <span id="occupiedentry-get"></span>`fn get(&self) -> &V`

  Gets a reference to the entry's value in the map.

- <span id="occupiedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut V`

  Gets a mutable reference to the entry's value in the map.

  

  If you need a reference which may outlive the destruction of the

  [`Entry`](entry/index.md) value, see `into_mut`.

- <span id="occupiedentry-into-mut"></span>`fn into_mut(self) -> &'a mut V`

  Converts into a mutable reference to the entry's value in the map,

  with a lifetime bound to the map itself.

- <span id="occupiedentry-insert"></span>`fn insert(&mut self, value: V) -> V`

  Sets the value of the entry to `value`, and returns the entry's old value.

- <span id="occupiedentry-remove"></span>`fn remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  **NOTE:** This is equivalent to `.swap_remove()`, replacing this

  entry's position with the last element, and it is deprecated in favor of calling that

  explicitly. If you need to preserve the relative order of the keys in the map, use

  `.shift_remove()` instead.

- <span id="occupiedentry-swap-remove"></span>`fn swap_remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  Like `Vec::swap_remove`, the pair is removed by swapping it

  with the last element of the map and popping it off.

  **This perturbs the position of what used to be the last element!**

  

  Computes in **O(1)** time (average).

- <span id="occupiedentry-shift-remove"></span>`fn shift_remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  Like `Vec::remove`, the pair is removed by shifting all of the

  elements that follow it, preserving their relative order.

  **This perturbs the index of all of those elements!**

  

  Computes in **O(n)** time (average).

- <span id="occupiedentry-remove-entry"></span>`fn remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  **NOTE:** This is equivalent to `.swap_remove_entry()`,

  replacing this entry's position with the last element, and it is deprecated in favor of

  calling that explicitly. If you need to preserve the relative order of the keys in the map,

  use `.shift_remove_entry()` instead.

- <span id="occupiedentry-swap-remove-entry"></span>`fn swap_remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  Like `Vec::swap_remove`, the pair is removed by swapping it

  with the last element of the map and popping it off.

  **This perturbs the position of what used to be the last element!**

  

  Computes in **O(1)** time (average).

- <span id="occupiedentry-shift-remove-entry"></span>`fn shift_remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  Like `Vec::remove`, the pair is removed by shifting all of the

  elements that follow it, preserving their relative order.

  **This perturbs the index of all of those elements!**

  

  Computes in **O(n)** time (average).

- <span id="occupiedentry-remove-index"></span>`fn remove_index(&mut self)`

- <span id="occupiedentry-move-index"></span>`fn move_index(self, to: usize)`

  Moves the position of the entry to a new index

  by shifting all other entries in-between.

  

  This is equivalent to [`IndexMap::move_index`][`crate::IndexMap::move_index`](#move-index)

  coming `from` the current `.index()`.

  

  * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.

  * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.

  

  ***Panics*** if `to` is out of bounds.

  

  Computes in **O(n)** time (average).

- <span id="occupiedentry-swap-indices"></span>`fn swap_indices(self, other: usize)`

  Swaps the position of entry with another.

  

  This is equivalent to [`IndexMap::swap_indices`][`crate::IndexMap::swap_indices`](#swap-indices)

  with the current `.index()` as one of the two being swapped.

  

  ***Panics*** if the `other` index is out of bounds.

  

  Computes in **O(1)** time (average).

- <span id="occupiedentry-update-index"></span>`fn update_index(self, to: usize)`

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for crate::inner::OccupiedEntry<'_, K, V>`

- <span id="crateinneroccupiedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::OccupiedEntry<'_, K, V>`

- <span id="superoccupiedentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="superoccupiedentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](mutable/index.md#mutableentrykey)

##### `impl<K, V> Sealed for super::OccupiedEntry<'_, K, V>`

### `VacantEntry<'a, K, V>`

```rust
struct VacantEntry<'a, K, V> {
    map: &'a mut super::Core<K, V>,
    hash: crate::HashValue,
    key: K,
}
```

A view into a vacant entry in an `IndexMap`.
It is part of the [`Entry`](entry/index.md) enum.

#### Implementations

- <span id="vacantentry-index"></span>`fn index(&self) -> usize`

  Return the index where a key-value pair may be inserted.

- <span id="vacantentry-key"></span>`fn key(&self) -> &K`

  Gets a reference to the key that was used to find the entry.

- <span id="vacantentry-key-mut"></span>`fn key_mut(&mut self) -> &mut K`

- <span id="vacantentry-into-key"></span>`fn into_key(self) -> K`

  Takes ownership of the key, leaving the entry vacant.

- <span id="vacantentry-insert"></span>`fn insert(self, value: V) -> &'a mut V`

  Inserts the entry's key and the given value into the map, and returns a mutable reference

  to the value.

  

  Computes in **O(1)** time (amortized average).

- <span id="vacantentry-insert-entry"></span>`fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V>` — [`OccupiedEntry`](../inner/entry/index.md#occupiedentry)

  Inserts the entry's key and the given value into the map, and returns an `OccupiedEntry`.

  

  Computes in **O(1)** time (amortized average).

- <span id="vacantentry-insert-sorted"></span>`fn insert_sorted(self, value: V) -> (usize, &'a mut V)`

  Inserts the entry's key and the given value into the map at its ordered

  position among sorted keys, and returns the new index and a mutable

  reference to the value.

  

  If the existing keys are **not** already sorted, then the insertion

  index is unspecified (like `slice::binary_search`), but the key-value

  pair is inserted at that position regardless.

  

  Computes in **O(n)** time (average).

- <span id="vacantentry-insert-sorted-by"></span>`fn insert_sorted_by<F>(self, value: V, cmp: F) -> (usize, &'a mut V)`

  Inserts the entry's key and the given value into the map at its ordered

  position among keys sorted by `cmp`, and returns the new index and a

  mutable reference to the value.

  

  If the existing keys are **not** already sorted, then the insertion

  index is unspecified (like `slice::binary_search`), but the key-value

  pair is inserted at that position regardless.

  

  Computes in **O(n)** time (average).

- <span id="vacantentry-insert-sorted-by-key"></span>`fn insert_sorted_by_key<B, F>(self, value: V, sort_key: F) -> (usize, &'a mut V)`

  Inserts the entry's key and the given value into the map at its ordered

  position using a sort-key extraction function, and returns the new index

  and a mutable reference to the value.

  

  If the existing keys are **not** already sorted, then the insertion

  index is unspecified (like `slice::binary_search`), but the key-value

  pair is inserted at that position regardless.

  

  Computes in **O(n)** time (average).

- <span id="vacantentry-shift-insert"></span>`fn shift_insert(self, index: usize, value: V) -> &'a mut V`

  Inserts the entry's key and the given value into the map at the given index,

  shifting others to the right, and returns a mutable reference to the value.

  

  ***Panics*** if `index` is out of bounds.

  

  Computes in **O(n)** time (average).

- <span id="vacantentry-replace-index"></span>`fn replace_index(self, index: usize) -> (K, OccupiedEntry<'a, K, V>)` — [`OccupiedEntry`](../inner/entry/index.md#occupiedentry)

  Replaces the key at the given index with this entry's key, returning the

  old key and an `OccupiedEntry` for that index.

  

  ***Panics*** if `index` is out of bounds.

  

  Computes in **O(1)** time (average).

#### Trait Implementations

##### `impl<K: fmt::Debug, V> Debug for crate::inner::VacantEntry<'_, K, V>`

- <span id="crateinnervacantentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::VacantEntry<'_, K, V>`

- <span id="supervacantentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="supervacantentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](mutable/index.md#mutableentrykey)

##### `impl<K, V> Sealed for super::VacantEntry<'_, K, V>`

### `Drain<'a, K, V>`

```rust
struct Drain<'a, K, V> {
    iter: vec::Drain<'a, crate::Bucket<K, V>>,
}
```

A draining iterator over the entries of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::drain` method.
See its documentation for more.

#### Implementations

- <span id="drain-new"></span>`fn new(iter: vec::Drain<'a, Bucket<K, V>>) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="drain-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](slice/index.md#slice)

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

- <span id="extractif-new"></span>`fn new<R>(core: &mut Core<K, V>, range: R, pred: F) -> ExtractIf<'_, K, V, F>` — [`Core`](../inner/index.md#core), [`ExtractIf`](iter/index.md#extractif)

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

### `IntoIter<K, V>`

```rust
struct IntoIter<K, V> {
    iter: vec::IntoIter<crate::Bucket<K, V>>,
}
```

An owning iterator over the entries of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::into_iter` method
(provided by the `IntoIterator` trait). See its documentation for more.

#### Implementations

- <span id="intoiter-new"></span>`fn new(entries: Vec<Bucket<K, V>>) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut Slice<K, V>` — [`Slice`](slice/index.md#slice)

  Returns a mutable slice of the remaining entries in the iterator.

#### Trait Implementations

##### `impl<K: clone::Clone, V: clone::Clone> Clone for IntoIter<K, V>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<K, V>` — [`IntoIter`](iter/index.md#intoiter)

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

### `IntoKeys<K, V>`

```rust
struct IntoKeys<K, V> {
    iter: vec::IntoIter<crate::Bucket<K, core::mem::MaybeUninit<V>>>,
}
```

An owning iterator over the keys of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::into_keys` method.
See its documentation for more.

#### Implementations

- <span id="intokeys-new"></span>`fn new(entries: Vec<Bucket<K, V>>) -> Self` — [`Bucket`](../index.md#bucket)

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

### `IntoValues<K, V>`

```rust
struct IntoValues<K, V> {
    iter: vec::IntoIter<crate::Bucket<core::mem::MaybeUninit<K>, V>>,
}
```

An owning iterator over the values of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::into_values` method.
See its documentation for more.

#### Implementations

- <span id="intovalues-new"></span>`fn new(entries: Vec<Bucket<K, V>>) -> Self` — [`Bucket`](../index.md#bucket)

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

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    iter: slice::Iter<'a, crate::Bucket<K, V>>,
}
```

An iterator over the entries of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::iter` method.
See its documentation for more.

#### Implementations

- <span id="iter-new"></span>`fn new(entries: &'a [Bucket<K, V>]) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="iter-as-slice"></span>`fn as_slice(&self) -> &'a Slice<K, V>` — [`Slice`](slice/index.md#slice)

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

A mutable iterator over the entries of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::iter_mut` method.
See its documentation for more.

#### Implementations

- <span id="itermut-new"></span>`fn new(entries: &'a mut [Bucket<K, V>]) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="itermut-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

- <span id="itermut-into-slice"></span>`fn into_slice(self) -> &'a mut Slice<K, V>` — [`Slice`](slice/index.md#slice)

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

A mutable iterator over the entries of an [`IndexMap`](#indexmap).

This `struct` is created by the `MutableKeys::iter_mut2` method.
See its documentation for more.

#### Implementations

- <span id="itermut2-new"></span>`fn new(entries: &'a mut [Bucket<K, V>]) -> Self` — [`Bucket`](../index.md#bucket)

- <span id="itermut2-as-slice"></span>`fn as_slice(&self) -> &Slice<K, V>` — [`Slice`](slice/index.md#slice)

  Returns a slice of the remaining entries in the iterator.

- <span id="itermut2-into-slice"></span>`fn into_slice(self) -> &'a mut Slice<K, V>` — [`Slice`](slice/index.md#slice)

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

### `Keys<'a, K, V>`

```rust
struct Keys<'a, K, V> {
    iter: slice::Iter<'a, crate::Bucket<K, V>>,
}
```

An iterator over the keys of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::keys` method.
See its documentation for more.

#### Implementations

- <span id="keys-new"></span>`fn new(entries: &'a [Bucket<K, V>]) -> Self` — [`Bucket`](../index.md#bucket)

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

- <span id="splice-new"></span>`fn new<R>(map: &'a mut IndexMap<K, V, S>, range: R, replace_with: I) -> Self` — [`IndexMap`](#indexmap)

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

### `Values<'a, K, V>`

```rust
struct Values<'a, K, V> {
    iter: slice::Iter<'a, crate::Bucket<K, V>>,
}
```

An iterator over the values of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::values` method.
See its documentation for more.

#### Implementations

- <span id="values-new"></span>`fn new(entries: &'a [Bucket<K, V>]) -> Self` — [`Bucket`](../index.md#bucket)

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

A mutable iterator over the values of an [`IndexMap`](#indexmap).

This `struct` is created by the `IndexMap::values_mut` method.
See its documentation for more.

#### Implementations

- <span id="valuesmut-new"></span>`fn new(entries: &'a mut [Bucket<K, V>]) -> Self` — [`Bucket`](../index.md#bucket)

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

### `Slice<K, V>`

```rust
struct Slice<K, V> {
    entries: [crate::Bucket<K, V>],
}
```

A dynamically-sized slice of key-value pairs in an [`IndexMap`](#indexmap).

This supports indexed operations much like a `[(K, V)]` slice,
but not any hashed operations on the map keys.

Unlike `IndexMap`, `Slice` does consider the order for `PartialEq`
and [`Eq`](../../base64ct/index.md), and it also implements `PartialOrd`, `Ord`, and [`Hash`](../../memchr/arch/all/rabinkarp/index.md).

#### Implementations

- <span id="slice-from-slice"></span>`const fn from_slice(entries: &[Bucket<K, V>]) -> &Self` — [`Bucket`](../index.md#bucket)

- <span id="slice-from-mut-slice"></span>`fn from_mut_slice(entries: &mut [Bucket<K, V>]) -> &mut Self` — [`Bucket`](../index.md#bucket)

- <span id="slice-from-boxed"></span>`fn from_boxed(entries: Box<[Bucket<K, V>]>) -> Box<Self>` — [`Bucket`](../index.md#bucket)

- <span id="slice-into-boxed"></span>`fn into_boxed(self: Box<Self>) -> Box<[Bucket<K, V>]>` — [`Bucket`](../index.md#bucket)

#### Trait Implementations

##### `impl<K: Clone, V: Clone> Clone for alloc::boxed::Box<Slice<K, V>>`

- <span id="allocboxedbox-clone"></span>`fn clone(&self) -> Self`

##### `impl<K> Comparable for Slice<K, V>`

- <span id="slice-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Slice<K, V>`

- <span id="slice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for &Slice<K, V>`

- <span id="slice-default"></span>`fn default() -> Self`

##### `impl<K: Eq, V: Eq> Eq for Slice<K, V>`

##### `impl<K> Equivalent for Slice<K, V>`

- <span id="slice-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<K: Hash, V: Hash> Hash for Slice<K, V>`

- <span id="slice-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<K, V> Index for Slice<K, V>`

- <span id="slice-index-type-output"></span>`type Output = V`

- <span id="slice-index"></span>`fn index(&self, index: usize) -> &V`

##### `impl<K, V> IndexMut for Slice<K, V>`

- <span id="slice-indexmut-index-mut"></span>`fn index_mut(&mut self, index: usize) -> &mut V`

##### `impl<K, V> IntoIterator for &'a Slice<K, V>`

- <span id="a-slice-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, K, V>`

- <span id="a-slice-intoiterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="a-slice-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<K: Ord, V: Ord> Ord for Slice<K, V>`

- <span id="slice-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<K, V, K2, V2> PartialEq for Slice<K, V>`

- <span id="slice-partialeq-eq"></span>`fn eq(&self, other: &Slice<K2, V2>) -> bool` — [`Slice`](slice/index.md#slice)

##### `impl<K: PartialOrd, V: PartialOrd> PartialOrd for Slice<K, V>`

- <span id="slice-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

### `IndexMap<K, V, S>`

```rust
struct IndexMap<K, V, S> {
    core: crate::inner::Core<K, V>,
    hash_builder: S,
}
```

A hash table where the iteration order of the key-value pairs is independent
of the hash values of the keys.

The interface is closely compatible with the standard
`HashMap`,
but also has additional features.

# Order

The key-value pairs have a consistent order that is determined by
the sequence of insertion and removal calls on the map. The order does
not depend on the keys or the hash function at all.

All iterators traverse the map in *the order*.

The insertion order is preserved, with **notable exceptions** like the
`.remove()` or `.swap_remove()` methods.
Methods such as `.sort_by()` of
course result in a new order, depending on the sorting order.

# Indices

The key-value pairs are indexed in a compact range without holes in the
range `0..self.len()`. For example, the method `.get_full` looks up the
index for a key, and the method `.get_index` looks up the key-value pair by
index.

# Examples

```rust
use indexmap::IndexMap;

// count the frequency of each letter in a sentence.
let mut letters = IndexMap::new();
for ch in "a short treatise on fungi".chars() {
    *letters.entry(ch).or_insert(0) += 1;
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

#### Implementations

- <span id="indexmap-new"></span>`fn new() -> Self`

  Create a new map. (Does not allocate.)

- <span id="indexmap-with-capacity"></span>`fn with_capacity(n: usize) -> Self`

  Create a new map with capacity for `n` key-value pairs. (Does not

  allocate if `n` is zero.)

  

  Computes in **O(n)** time.

#### Trait Implementations

##### `impl<K, V, S> Clone for IndexMap<K, V, S>`

- <span id="indexmap-clone"></span>`fn clone(&self) -> Self`

- <span id="indexmap-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<K, V, S> Debug for IndexMap<K, V, S>`

- <span id="indexmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S> Default for IndexMap<K, V, S>`

- <span id="indexmap-default"></span>`fn default() -> Self`

  Return an empty [`IndexMap`](#indexmap)

##### `impl<K, V, S> Eq for IndexMap<K, V, S>`

##### `impl<K> Equivalent for IndexMap<K, V, S>`

- <span id="indexmap-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<K, V, S> Extend for IndexMap<K, V, S>`

- <span id="indexmap-extend"></span>`fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iterable: I)`

  Extend the map with all key-value pairs in the iterable.

  

  This is equivalent to calling `insert` for each of

  them in order, which means that for keys that already existed

  in the map, their value is updated but it keeps the existing order.

  

  New keys are inserted in the order they appear in the sequence. If

  equivalents of a key occur more than once, the last corresponding value

  prevails.

##### `impl<K, V, S> FromIterator for IndexMap<K, V, S>`

- <span id="indexmap-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self`

  Create an `IndexMap` from the sequence of key-value pairs in the

  iterable.

  

  `from_iter` uses the same logic as `extend`. See

  `extend` for more details.

##### `impl<K, V, S> Index for IndexMap<K, V, S>`

- <span id="indexmap-index-type-output"></span>`type Output = Slice<K, V>`

- <span id="indexmap-index"></span>`fn index(&self, range: ops::Range<usize>) -> &<Self as >::Output`

##### `impl<K, V, S> IndexMut for IndexMap<K, V, S>`

- <span id="indexmap-indexmut-index-mut"></span>`fn index_mut(&mut self, range: ops::Range<usize>) -> &mut <Self as >::Output`

##### `impl<K, V, S> IntoIterator for &'a super::IndexMap<K, V, S>`

- <span id="a-superindexmap-intoiterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="a-superindexmap-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, K, V>`

- <span id="a-superindexmap-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<K, V, S> MutableKeys for super::IndexMap<K, V, S>`

- <span id="superindexmap-mutablekeys-type-key"></span>`type Key = K`

- <span id="superindexmap-mutablekeys-type-value"></span>`type Value = V`

- <span id="superindexmap-mutablekeys-get-full-mut2"></span>`fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut K, &mut V)>`

- <span id="superindexmap-mutablekeys-get-index-mut2"></span>`fn get_index_mut2(&mut self, index: usize) -> Option<(&mut K, &mut V)>`

- <span id="superindexmap-mutablekeys-iter-mut2"></span>`fn iter_mut2(&mut self) -> IterMut2<'_, <Self as >::Key, <Self as >::Value>` — [`IterMut2`](iter/index.md#itermut2), [`MutableKeys`](mutable/index.md#mutablekeys)

- <span id="superindexmap-mutablekeys-retain2"></span>`fn retain2<F>(&mut self, keep: F)`

##### `impl<K, V1, S1, V2, S2> PartialEq for IndexMap<K, V1, S1>`

- <span id="indexmap-partialeq-eq"></span>`fn eq(&self, other: &IndexMap<K, V2, S2>) -> bool` — [`IndexMap`](#indexmap)

##### `impl<K, V, S> RawEntryApiV1 for crate::IndexMap<K, V, S>`

- <span id="crateindexmap-rawentryapiv1-raw-entry-v1"></span>`fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S>` — [`RawEntryBuilder`](raw_entry_v1/index.md#rawentrybuilder)

- <span id="crateindexmap-rawentryapiv1-raw-entry-mut-v1"></span>`fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S>` — [`RawEntryBuilderMut`](raw_entry_v1/index.md#rawentrybuildermut)

##### `impl<K, V, S> Sealed for super::IndexMap<K, V, S>`

## Enums

### `Entry<'a, K, V>`

```rust
enum Entry<'a, K, V> {
    Occupied(crate::inner::OccupiedEntry<'a, K, V>),
    Vacant(crate::inner::VacantEntry<'a, K, V>),
}
```

Entry for an existing key-value pair in an `IndexMap`
or a vacant location to insert one.

#### Variants

- **`Occupied`**

  Existing slot with equivalent key.

- **`Vacant`**

  Vacant slot (no equivalent key in the map).

#### Implementations

- <span id="cratemapentry-new"></span>`fn new(map: &'a mut Core<K, V>, hash: HashValue, key: K) -> Self` — [`Core`](../inner/index.md#core), [`HashValue`](../index.md#hashvalue)

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Entry<'_, K, V>`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::Entry<'_, K, V>`

- <span id="superentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="superentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](mutable/index.md#mutableentrykey)

##### `impl<K, V> Sealed for super::Entry<'_, K, V>`

## Traits

### `MutableEntryKey`

```rust
trait MutableEntryKey: Sealed { ... }
```

Opt-in mutable access to [`Entry`](entry/index.md) keys.

These methods expose `&mut K`, mutable references to the key as it is stored
in the map.
You are allowed to modify the keys in the map **if the modification
does not change the key's hash and equality**.

If keys are modified erroneously, you can no longer look them up.
This is sound (memory safe) but a logical error hazard (just like
implementing `PartialEq`, `Eq`, or `Hash` incorrectly would be).

`use` this trait to enable its methods for `Entry`.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Key`

#### Required Methods

- `fn key_mut(&mut self) -> &mut <Self as >::Key`

  Gets a mutable reference to the entry's key, either within the map if occupied,

#### Implementors

- [`Entry`](entry/index.md#entry)
- [`IndexedEntry`](entry/index.md#indexedentry)
- [`OccupiedEntry`](../inner/entry/index.md#occupiedentry)
- [`VacantEntry`](../inner/entry/index.md#vacantentry)

### `MutableKeys`

```rust
trait MutableKeys: Sealed { ... }
```

Opt-in mutable access to [`IndexMap`](#indexmap) keys.

These methods expose `&mut K`, mutable references to the key as it is stored
in the map.
You are allowed to modify the keys in the map **if the modification
does not change the key's hash and equality**.

If keys are modified erroneously, you can no longer look them up.
This is sound (memory safe) but a logical error hazard (just like
implementing `PartialEq`, `Eq`, or `Hash` incorrectly would be).

`use` this trait to enable its methods for `IndexMap`.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Key`

- `type Value`

#### Required Methods

- `fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut <Self as >::Key, &mut <Self as >::Value)>`

  Return item index, mutable reference to key and value

- `fn get_index_mut2(&mut self, index: usize) -> Option<(&mut <Self as >::Key, &mut <Self as >::Value)>`

  Return mutable reference to key and value at an index.

- `fn iter_mut2(&mut self) -> IterMut2<'_, <Self as >::Key, <Self as >::Value>`

  Return an iterator over the key-value pairs of the map, in their order

- `fn retain2<F>(&mut self, keep: F)`

  Scan through each key-value pair in the map and keep those where the

#### Implementors

- [`IndexMap`](#indexmap)

### `RawEntryApiV1<K, V, S>`

```rust
trait RawEntryApiV1<K, V, S>: Sealed { ... }
```

Opt-in access to the experimental raw entry API.

See the [`raw_entry_v1`][self] module documentation for more information.

#### Required Methods

- `fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S>`

  Creates a raw immutable entry builder for the [`IndexMap`](#indexmap).

- `fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S>`

  Creates a raw entry builder for the [`IndexMap`](#indexmap).

#### Implementors

- [`IndexMap`](#indexmap)

