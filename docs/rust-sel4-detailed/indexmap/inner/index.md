*[indexmap](../index.md) / [inner](index.md)*

---

# Module `inner`

This is the core implementation that doesn't depend on the hasher at all.

The methods of `Core` don't use any Hash properties of K.

It's cleaner to separate them out, then the compiler checks that we are not
using Hash at all in these methods.

However, we should probably not let this show in the public API or docs.

## Contents

- [Modules](#modules)
  - [`entry`](#entry)
  - [`extract`](#extract)
- [Structs](#structs)
  - [`OccupiedEntry`](#occupiedentry)
  - [`VacantEntry`](#vacantentry)
  - [`Core`](#core)
- [Functions](#functions)
  - [`get_hash`](#get-hash)
  - [`equivalent`](#equivalent)
  - [`erase_index`](#erase-index)
  - [`update_index`](#update-index)
  - [`insert_bulk_no_grow`](#insert-bulk-no-grow)
- [Type Aliases](#type-aliases)
  - [`Indices`](#indices)
  - [`Entries`](#entries)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`entry`](#entry) | mod |  |
| [`extract`](#extract) | mod |  |
| [`OccupiedEntry`](#occupiedentry) | struct |  |
| [`VacantEntry`](#vacantentry) | struct |  |
| [`Core`](#core) | struct | Core of the map that does not depend on S |
| [`get_hash`](#get-hash) | fn |  |
| [`equivalent`](#equivalent) | fn |  |
| [`erase_index`](#erase-index) | fn |  |
| [`update_index`](#update-index) | fn |  |
| [`insert_bulk_no_grow`](#insert-bulk-no-grow) | fn | Inserts many entries into the indices table without reallocating, and without regard for duplication. |
| [`Indices`](#indices) | type |  |
| [`Entries`](#entries) | type |  |

## Modules

- [`entry`](entry/index.md)
- [`extract`](extract/index.md)

## Structs

### `OccupiedEntry<'a, K, V>`

```rust
struct OccupiedEntry<'a, K, V> {
    map: &'a mut super::Core<K, V>,
    index: usize,
    bucket: usize,
}
```

A view into an occupied entry in an `IndexMap`.
It is part of the [`Entry`](../map/entry/index.md) enum.

#### Implementations

- <span id="occupiedentry-from-hash"></span>`fn from_hash<F>(map: &'a mut Core<K, V>, hash: HashValue, is_match: F) -> Result<Self, &'a mut Core<K, V>>` — [`Core`](#core), [`HashValue`](../index.md#hashvalue)

  Constructor for `RawEntryMut::from_hash`

- <span id="occupiedentry-into-core"></span>`fn into_core(self) -> &'a mut Core<K, V>` — [`Core`](#core)

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

  [`Entry`](../map/entry/index.md) value, see `into_mut`.

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

  

  This is equivalent to [`IndexMap::move_index`][`crate::IndexMap::move_index`](../map/index.md#move-index)

  coming `from` the current `.index()`.

  

  * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.

  * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.

  

  ***Panics*** if `to` is out of bounds.

  

  Computes in **O(n)** time (average).

- <span id="occupiedentry-swap-indices"></span>`fn swap_indices(self, other: usize)`

  Swaps the position of entry with another.

  

  This is equivalent to [`IndexMap::swap_indices`][`crate::IndexMap::swap_indices`](../map/index.md#swap-indices)

  with the current `.index()` as one of the two being swapped.

  

  ***Panics*** if the `other` index is out of bounds.

  

  Computes in **O(1)** time (average).

- <span id="occupiedentry-update-index"></span>`fn update_index(self, to: usize)`

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for crate::inner::OccupiedEntry<'_, K, V>`

- <span id="crateinneroccupiedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::OccupiedEntry<'_, K, V>`

- <span id="superoccupiedentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="superoccupiedentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](../map/mutable/index.md#mutableentrykey)

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
It is part of the [`Entry`](../map/entry/index.md) enum.

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

- <span id="vacantentry-insert-entry"></span>`fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V>` — [`OccupiedEntry`](entry/index.md#occupiedentry)

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

- <span id="vacantentry-replace-index"></span>`fn replace_index(self, index: usize) -> (K, OccupiedEntry<'a, K, V>)` — [`OccupiedEntry`](entry/index.md#occupiedentry)

  Replaces the key at the given index with this entry's key, returning the

  old key and an `OccupiedEntry` for that index.

  

  ***Panics*** if `index` is out of bounds.

  

  Computes in **O(1)** time (average).

#### Trait Implementations

##### `impl<K: fmt::Debug, V> Debug for crate::inner::VacantEntry<'_, K, V>`

- <span id="crateinnervacantentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> MutableEntryKey for super::VacantEntry<'_, K, V>`

- <span id="supervacantentry-mutableentrykey-type-key"></span>`type Key = K`

- <span id="supervacantentry-mutableentrykey-key-mut"></span>`fn key_mut(&mut self) -> &mut <Self as >::Key` — [`MutableEntryKey`](../map/mutable/index.md#mutableentrykey)

##### `impl<K, V> Sealed for super::VacantEntry<'_, K, V>`

### `Core<K, V>`

```rust
struct Core<K, V> {
    indices: hash_table::HashTable<usize>,
    entries: alloc::vec::Vec<crate::Bucket<K, V>>,
}
```

Core of the map that does not depend on S

#### Fields

- **`indices`**: `hash_table::HashTable<usize>`

  indices mapping from the entry hash to its index.

- **`entries`**: `alloc::vec::Vec<crate::Bucket<K, V>>`

  entries is a dense vec maintaining entry order.

#### Implementations

- <span id="supercore-extract"></span>`fn extract<R>(&mut self, range: R) -> ExtractCore<'_, K, V>` — [`ExtractCore`](extract/index.md#extractcore)

#### Trait Implementations

##### `impl<K, V> Clone for Core<K, V>`

- <span id="core-clone"></span>`fn clone(&self) -> Self`

- <span id="core-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

## Functions

### `get_hash`

```rust
fn get_hash<K, V>(entries: &[crate::Bucket<K, V>]) -> impl Fn(&usize) -> u64
```

### `equivalent`

```rust
fn equivalent<'a, K, V, Q: ?Sized + Equivalent<K>>(key: &'a Q, entries: &'a [crate::Bucket<K, V>]) -> impl Fn(&usize) -> bool
```

### `erase_index`

```rust
fn erase_index(table: &mut hash_table::HashTable<usize>, hash: crate::HashValue, index: usize)
```

### `update_index`

```rust
fn update_index(table: &mut hash_table::HashTable<usize>, hash: crate::HashValue, old: usize, new: usize)
```

### `insert_bulk_no_grow`

```rust
fn insert_bulk_no_grow<K, V>(indices: &mut hash_table::HashTable<usize>, entries: &[crate::Bucket<K, V>])
```

Inserts many entries into the indices table without reallocating,
and without regard for duplication.

***Panics*** if there is not sufficient capacity already.

## Type Aliases

### `Indices`

```rust
type Indices = hash_table::HashTable<usize>;
```

### `Entries<K, V>`

```rust
type Entries<K, V> = alloc::vec::Vec<crate::Bucket<K, V>>;
```

