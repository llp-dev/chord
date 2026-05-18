*[indexmap](../../index.md) / [map](../index.md) / [entry](index.md)*

---

# Module `entry`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IndexedEntry`](#indexedentry) | struct | A view into an occupied entry in an [`IndexMap`][crate::IndexMap] obtained by index. |
| [`Entry`](#entry) | enum | Entry for an existing key-value pair in an [`IndexMap`][crate::IndexMap] or a vacant location to insert one. |

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

- <span id="indexedentry-new"></span>`fn new(map: &'a mut Core<K, V>, index: usize) -> Option<Self>` — [`Core`](../../inner/index.md#core)

- <span id="indexedentry-index"></span>`fn index(&self) -> usize`

  Return the index of the key-value pair

- <span id="indexedentry-into-core"></span>`fn into_core(self) -> &'a mut Core<K, V>` — [`Core`](../../inner/index.md#core)

- <span id="indexedentry-get-bucket"></span>`fn get_bucket(&self) -> &Bucket<K, V>` — [`Bucket`](../../index.md#bucket)

- <span id="indexedentry-get-bucket-mut"></span>`fn get_bucket_mut(&mut self) -> &mut Bucket<K, V>` — [`Bucket`](../../index.md#bucket)

- <span id="indexedentry-into-bucket"></span>`fn into_bucket(self) -> &'a mut Bucket<K, V>` — [`Bucket`](../../index.md#bucket)

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

  

  This is equivalent to [`IndexMap::move_index`][`crate::IndexMap::move_index`](../index.md#move-index)

  coming `from` the current `.index()`.

  

  * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.

  * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.

  

  ***Panics*** if `to` is out of bounds.

  

  Computes in **O(n)** time (average).

- <span id="indexedentry-swap-indices"></span>`fn swap_indices(self, other: usize)`

  Swaps the position of entry with another.

  

  This is equivalent to [`IndexMap::swap_indices`][`crate::IndexMap::swap_indices`](../index.md#swap-indices)

  with the current `.index()` as one of the two being swapped.

  

  ***Panics*** if the `other` index is out of bounds.

  

  Computes in **O(1)** time (average).

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IndexedEntry<'_, K, V>`

- <span id="indexedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::IndexedEntry<'_, K, V>`

- <span id="superindexedentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="superindexedentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](../mutable/index.md#mutableentrykey)

##### `impl<K, V> Sealed for super::IndexedEntry<'_, K, V>`

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

- <span id="cratemapentry-new"></span>`fn new(map: &'a mut Core<K, V>, hash: HashValue, key: K) -> Self` — [`Core`](../../inner/index.md#core), [`HashValue`](../../index.md#hashvalue)

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Entry<'_, K, V>`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::Entry<'_, K, V>`

- <span id="superentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="superentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](../mutable/index.md#mutableentrykey)

##### `impl<K, V> Sealed for super::Entry<'_, K, V>`

