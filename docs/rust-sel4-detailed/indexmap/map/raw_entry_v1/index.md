*[indexmap](../../index.md) / [map](../index.md) / [raw_entry_v1](index.md)*

---

# Module `raw_entry_v1`

Opt-in access to the experimental raw entry API.

This module is designed to mimic the raw entry API of `HashMap`,
matching its unstable state as of Rust 1.75. See the tracking issue
[rust#56167](https://github.com/rust-lang/rust/issues/56167) for more details.

The trait [`RawEntryApiV1`](#rawentryapiv1) and the `_v1` suffix on its methods are meant to insulate this for
the future, in case later breaking changes are needed. If the standard library stabilizes its
`hash_raw_entry` feature (or some replacement), matching *inherent* methods will be added to
`IndexMap` without such an opt-in trait.

## Contents

- [Structs](#structs)
  - [`RawEntryBuilder`](#rawentrybuilder)
  - [`RawEntryBuilderMut`](#rawentrybuildermut)
  - [`RawOccupiedEntryMut`](#rawoccupiedentrymut)
  - [`RawVacantEntryMut`](#rawvacantentrymut)
- [Enums](#enums)
  - [`RawEntryMut`](#rawentrymut)
- [Traits](#traits)
  - [`RawEntryApiV1`](#rawentryapiv1)
  - [`Sealed`](#sealed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RawEntryBuilder`](#rawentrybuilder) | struct | A builder for computing where in an [`IndexMap`] a key-value pair would be stored. |
| [`RawEntryBuilderMut`](#rawentrybuildermut) | struct | A builder for computing where in an [`IndexMap`] a key-value pair would be stored. |
| [`RawOccupiedEntryMut`](#rawoccupiedentrymut) | struct | A raw view into an occupied entry in an [`IndexMap`]. |
| [`RawVacantEntryMut`](#rawvacantentrymut) | struct | A view into a vacant raw entry in an [`IndexMap`]. |
| [`RawEntryMut`](#rawentrymut) | enum | Raw entry for an existing key-value pair or a vacant location to insert one. |
| [`RawEntryApiV1`](#rawentryapiv1) | trait | Opt-in access to the experimental raw entry API. |
| [`Sealed`](#sealed) | trait |  |

## Structs

### `RawEntryBuilder<'a, K, V, S>`

```rust
struct RawEntryBuilder<'a, K, V, S> {
    map: &'a crate::IndexMap<K, V, S>,
}
```

A builder for computing where in an [`IndexMap`](../index.md) a key-value pair would be stored.

This `struct` is created by the `IndexMap::raw_entry_v1` method, provided by the
[`RawEntryApiV1`](#rawentryapiv1) trait. See its documentation for more.

#### Implementations

- <span id="rawentrybuilder-from-key"></span>`fn from_key<Q>(self, key: &Q) -> Option<(&'a K, &'a V)>`

  Access an entry by key.

- <span id="rawentrybuilder-from-key-hashed-nocheck"></span>`fn from_key_hashed_nocheck<Q>(self, hash: u64, key: &Q) -> Option<(&'a K, &'a V)>`

  Access an entry by a key and its hash.

- <span id="rawentrybuilder-from-hash"></span>`fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>`

  Access an entry by hash.

- <span id="rawentrybuilder-from-hash-full"></span>`fn from_hash_full<F>(self, hash: u64, is_match: F) -> Option<(usize, &'a K, &'a V)>`

  Access an entry by hash, including its index.

- <span id="rawentrybuilder-index-from-hash"></span>`fn index_from_hash<F>(self, hash: u64, is_match: F) -> Option<usize>`

  Access the index of an entry by hash.

#### Trait Implementations

##### `impl<K, V, S> Debug for RawEntryBuilder<'_, K, V, S>`

- <span id="rawentrybuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RawEntryBuilderMut<'a, K, V, S>`

```rust
struct RawEntryBuilderMut<'a, K, V, S> {
    map: &'a mut crate::IndexMap<K, V, S>,
}
```

A builder for computing where in an [`IndexMap`](../index.md) a key-value pair would be stored.

This `struct` is created by the `IndexMap::raw_entry_mut_v1` method, provided by the
[`RawEntryApiV1`](#rawentryapiv1) trait. See its documentation for more.

#### Implementations

- <span id="rawentrybuildermut-from-key"></span>`fn from_key<Q>(self, key: &Q) -> RawEntryMut<'a, K, V, S>` — [`RawEntryMut`](#rawentrymut)

  Access an entry by key.

- <span id="rawentrybuildermut-from-key-hashed-nocheck"></span>`fn from_key_hashed_nocheck<Q>(self, hash: u64, key: &Q) -> RawEntryMut<'a, K, V, S>` — [`RawEntryMut`](#rawentrymut)

  Access an entry by a key and its hash.

- <span id="rawentrybuildermut-from-hash"></span>`fn from_hash<F>(self, hash: u64, is_match: F) -> RawEntryMut<'a, K, V, S>` — [`RawEntryMut`](#rawentrymut)

  Access an entry by hash.

#### Trait Implementations

##### `impl<K, V, S> Debug for RawEntryBuilderMut<'_, K, V, S>`

- <span id="rawentrybuildermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RawOccupiedEntryMut<'a, K, V, S>`

```rust
struct RawOccupiedEntryMut<'a, K, V, S> {
    inner: super::OccupiedEntry<'a, K, V>,
    hash_builder: core::marker::PhantomData<&'a S>,
}
```

A raw view into an occupied entry in an [`IndexMap`](../index.md).
It is part of the [`RawEntryMut`](#rawentrymut) enum.

#### Implementations

- <span id="rawoccupiedentrymut-index"></span>`fn index(&self) -> usize`

  Return the index of the key-value pair

- <span id="rawoccupiedentrymut-key"></span>`fn key(&self) -> &K`

  Gets a reference to the entry's key in the map.

  

  Note that this is not the key that was used to find the entry. There may be an observable

  difference if the key type has any distinguishing features outside of `Hash` and `Eq`, like

  extra fields or the memory address of an allocation.

- <span id="rawoccupiedentrymut-key-mut"></span>`fn key_mut(&mut self) -> &mut K`

  Gets a mutable reference to the entry's key in the map.

  

  Note that this is not the key that was used to find the entry. There may be an observable

  difference if the key type has any distinguishing features outside of `Hash` and `Eq`, like

  extra fields or the memory address of an allocation.

- <span id="rawoccupiedentrymut-into-key"></span>`fn into_key(self) -> &'a mut K`

  Converts into a mutable reference to the entry's key in the map,

  with a lifetime bound to the map itself.

  

  Note that this is not the key that was used to find the entry. There may be an observable

  difference if the key type has any distinguishing features outside of `Hash` and `Eq`, like

  extra fields or the memory address of an allocation.

- <span id="rawoccupiedentrymut-get"></span>`fn get(&self) -> &V`

  Gets a reference to the entry's value in the map.

- <span id="rawoccupiedentrymut-get-mut"></span>`fn get_mut(&mut self) -> &mut V`

  Gets a mutable reference to the entry's value in the map.

  

  If you need a reference which may outlive the destruction of the

  [`RawEntryMut`](#rawentrymut) value, see `into_mut`.

- <span id="rawoccupiedentrymut-into-mut"></span>`fn into_mut(self) -> &'a mut V`

  Converts into a mutable reference to the entry's value in the map,

  with a lifetime bound to the map itself.

- <span id="rawoccupiedentrymut-get-key-value"></span>`fn get_key_value(&self) -> (&K, &V)`

  Gets a reference to the entry's key and value in the map.

- <span id="rawoccupiedentrymut-get-key-value-mut"></span>`fn get_key_value_mut(&mut self) -> (&mut K, &mut V)`

  Gets a reference to the entry's key and value in the map.

- <span id="rawoccupiedentrymut-into-key-value-mut"></span>`fn into_key_value_mut(self) -> (&'a mut K, &'a mut V)`

  Converts into a mutable reference to the entry's key and value in the map,

  with a lifetime bound to the map itself.

- <span id="rawoccupiedentrymut-insert"></span>`fn insert(&mut self, value: V) -> V`

  Sets the value of the entry, and returns the entry's old value.

- <span id="rawoccupiedentrymut-insert-key"></span>`fn insert_key(&mut self, key: K) -> K`

  Sets the key of the entry, and returns the entry's old key.

- <span id="rawoccupiedentrymut-remove"></span>`fn remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  **NOTE:** This is equivalent to `.swap_remove()`, replacing this

  entry's position with the last element, and it is deprecated in favor of calling that

  explicitly. If you need to preserve the relative order of the keys in the map, use

  `.shift_remove()` instead.

- <span id="rawoccupiedentrymut-swap-remove"></span>`fn swap_remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  Like `Vec::swap_remove`, the pair is removed by swapping it

  with the last element of the map and popping it off.

  **This perturbs the position of what used to be the last element!**

  

  Computes in **O(1)** time (average).

- <span id="rawoccupiedentrymut-shift-remove"></span>`fn shift_remove(self) -> V`

  Remove the key, value pair stored in the map for this entry, and return the value.

  

  Like `Vec::remove`, the pair is removed by shifting all of the

  elements that follow it, preserving their relative order.

  **This perturbs the index of all of those elements!**

  

  Computes in **O(n)** time (average).

- <span id="rawoccupiedentrymut-remove-entry"></span>`fn remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  **NOTE:** This is equivalent to `.swap_remove_entry()`,

  replacing this entry's position with the last element, and it is deprecated in favor of

  calling that explicitly. If you need to preserve the relative order of the keys in the map,

  use `.shift_remove_entry()` instead.

- <span id="rawoccupiedentrymut-swap-remove-entry"></span>`fn swap_remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  Like `Vec::swap_remove`, the pair is removed by swapping it

  with the last element of the map and popping it off.

  **This perturbs the position of what used to be the last element!**

  

  Computes in **O(1)** time (average).

- <span id="rawoccupiedentrymut-shift-remove-entry"></span>`fn shift_remove_entry(self) -> (K, V)`

  Remove and return the key, value pair stored in the map for this entry

  

  Like `Vec::remove`, the pair is removed by shifting all of the

  elements that follow it, preserving their relative order.

  **This perturbs the index of all of those elements!**

  

  Computes in **O(n)** time (average).

- <span id="rawoccupiedentrymut-move-index"></span>`fn move_index(self, to: usize)`

  Moves the position of the entry to a new index

  by shifting all other entries in-between.

  

  This is equivalent to `IndexMap::move_index`

  coming `from` the current `.index()`.

  

  * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.

  * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.

  

  ***Panics*** if `to` is out of bounds.

  

  Computes in **O(n)** time (average).

- <span id="rawoccupiedentrymut-swap-indices"></span>`fn swap_indices(self, other: usize)`

  Swaps the position of entry with another.

  

  This is equivalent to `IndexMap::swap_indices`

  with the current `.index()` as one of the two being swapped.

  

  ***Panics*** if the `other` index is out of bounds.

  

  Computes in **O(1)** time (average).

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug, S> Debug for RawOccupiedEntryMut<'_, K, V, S>`

- <span id="rawoccupiedentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RawVacantEntryMut<'a, K, V, S>`

```rust
struct RawVacantEntryMut<'a, K, V, S> {
    map: &'a mut crate::inner::Core<K, V>,
    hash_builder: &'a S,
}
```

A view into a vacant raw entry in an [`IndexMap`](../index.md).
It is part of the [`RawEntryMut`](#rawentrymut) enum.

#### Implementations

- <span id="rawvacantentrymut-index"></span>`fn index(&self) -> usize`

  Return the index where a key-value pair may be inserted.

- <span id="rawvacantentrymut-insert"></span>`fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Inserts the given key and value into the map,

  and returns mutable references to them.

- <span id="rawvacantentrymut-insert-hashed-nocheck"></span>`fn insert_hashed_nocheck(self, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Inserts the given key and value into the map with the provided hash,

  and returns mutable references to them.

- <span id="rawvacantentrymut-shift-insert"></span>`fn shift_insert(self, index: usize, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Inserts the given key and value into the map at the given index,

  shifting others to the right, and returns mutable references to them.

  

  ***Panics*** if `index` is out of bounds.

  

  Computes in **O(n)** time (average).

- <span id="rawvacantentrymut-shift-insert-hashed-nocheck"></span>`fn shift_insert_hashed_nocheck(self, index: usize, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Inserts the given key and value into the map with the provided hash

  at the given index, and returns mutable references to them.

  

  ***Panics*** if `index` is out of bounds.

  

  Computes in **O(n)** time (average).

#### Trait Implementations

##### `impl<K, V, S> Debug for RawVacantEntryMut<'_, K, V, S>`

- <span id="rawvacantentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `RawEntryMut<'a, K, V, S>`

```rust
enum RawEntryMut<'a, K, V, S> {
    Occupied(RawOccupiedEntryMut<'a, K, V, S>),
    Vacant(RawVacantEntryMut<'a, K, V, S>),
}
```

Raw entry for an existing key-value pair or a vacant location to
insert one.

#### Variants

- **`Occupied`**

  Existing slot with equivalent key.

- **`Vacant`**

  Vacant slot (no equivalent key in the map).

#### Implementations

- <span id="rawentrymut-index"></span>`fn index(&self) -> usize`

  Return the index where the key-value pair exists or may be inserted.

- <span id="rawentrymut-or-insert"></span>`fn or_insert(self, default_key: K, default_value: V) -> (&'a mut K, &'a mut V)`

  Inserts the given default key and value in the entry if it is vacant and returns mutable

  references to them. Otherwise mutable references to an already existent pair are returned.

- <span id="rawentrymut-or-insert-with"></span>`fn or_insert_with<F>(self, call: F) -> (&'a mut K, &'a mut V)`

  Inserts the result of the `call` function in the entry if it is vacant and returns mutable

  references to them. Otherwise mutable references to an already existent pair are returned.

- <span id="rawentrymut-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

  Modifies the entry if it is occupied.

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug, S> Debug for RawEntryMut<'_, K, V, S>`

- <span id="rawentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `RawEntryApiV1<K, V, S>`

```rust
trait RawEntryApiV1<K, V, S>: Sealed { ... }
```

Opt-in access to the experimental raw entry API.

See the [`raw_entry_v1`][self] module documentation for more information.

#### Required Methods

- `fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S>`

  Creates a raw immutable entry builder for the [`IndexMap`](../index.md).

- `fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S>`

  Creates a raw entry builder for the [`IndexMap`](../index.md).

#### Implementors

- [`IndexMap`](../index.md#indexmap)

### `Sealed`

```rust
trait Sealed { ... }
```

#### Implementors

- [`IndexMap`](../index.md#indexmap)

