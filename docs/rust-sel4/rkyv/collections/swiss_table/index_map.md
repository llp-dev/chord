**rkyv > collections > swiss_table > index_map**

# Module: collections::swiss_table::index_map

## Contents

**Structs**

- [`ArchivedIndexMap`](#archivedindexmap) - An archived `IndexMap`.
- [`IndexMapResolver`](#indexmapresolver) - The resolver for an `IndexMap`.
- [`Iter`](#iter) - An iterator over the key-value pairs of an index map.
- [`Keys`](#keys) - An iterator over the keys of an index map.
- [`Values`](#values) - An iterator over the values of an index map.

---

## rkyv::collections::swiss_table::index_map::ArchivedIndexMap

*Struct*

An archived `IndexMap`.

**Generic Parameters:**
- K
- V
- H

**Methods:**

- `fn is_empty(self: &Self) -> bool` - Returns `true` if the map contains no elements.
- `fn iter(self: &Self) -> Iter<K, V>` - Returns an iterator over the key-value pairs of the map in order
- `fn keys(self: &Self) -> Keys<K, V>` - Returns an iterator over the keys of the map in order
- `fn len(self: &Self) -> usize` - Gets the number of items in the index map.
- `fn values(self: &Self) -> Values<K, V>` - Returns an iterator over the values of the map in order.
- `fn get_full_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<(usize, &K, &V)>` - Gets the index, key, and value corresponding to the supplied key using
- `fn get_full<Q>(self: &Self, key: &Q) -> Option<(usize, &K, &V)>` - Gets the index, key, and value corresponding to the supplied key.
- `fn get_key_value_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<(&K, &V)>` - Returns the key-value pair corresponding to the supplied key using the
- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>` - Returns the key-value pair corresponding to the supplied key.
- `fn get_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<&V>` - Returns a reference to the value corresponding to the supplied key using
- `fn get<Q>(self: &Self, key: &Q) -> Option<&V>` - Returns a reference to the value corresponding to the supplied key.
- `fn get_full_seal_with<'a, Q, C>(this: Seal<'a, Self>, key: &Q, cmp: C) -> Option<(usize, &'a K, Seal<'a, V>)>` - Gets the mutable index, key, and value corresponding to the supplied key
- `fn get_full_seal<'a, Q>(this: Seal<'a, Self>, key: &Q) -> Option<(usize, &'a K, Seal<'a, V>)>` - Gets the mutable index, key, and value corresponding to the supplied
- `fn get_key_value_seal_with<'a, Q, C>(this: Seal<'a, Self>, key: &Q, cmp: C) -> Option<(&'a K, Seal<'a, V>)>` - Returns the mutable key-value pair corresponding to the supplied key
- `fn get_key_value_seal<'a, Q>(this: Seal<'a, Self>, key: &Q) -> Option<(&'a K, Seal<'a, V>)>` - Returns the mutable key-value pair corresponding to the supplied key.
- `fn get_seal_with<'a, Q, C>(this: Seal<'a, Self>, key: &Q, cmp: C) -> Option<Seal<'a, V>>` - Returns a mutable reference to the value corresponding to the supplied
- `fn get_seal<'a, Q>(this: Seal<'a, Self>, key: &Q) -> Option<Seal<'a, V>>` - Returns a mutable reference to the value corresponding to the supplied
- `fn contains_key<Q>(self: &Self, key: &Q) -> bool` - Returns whether a key is present in the hash map.
- `fn get_index(self: &Self, index: usize) -> Option<(&K, &V)>` - Gets a key-value pair by index.
- `fn get_index_of_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<usize>` - Gets the index of a key if it exists in the map using the given
- `fn get_index_of<Q>(self: &Self, key: &Q) -> Option<usize>` - Gets the index of a key if it exists in the map.
- `fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: IndexMapResolver, out: Place<Self>)` - Resolves an archived index map from a given length and parameters.
- `fn serialize_from_iter<I, BKU, BVU, KU, VU, S>(iter: I, load_factor: (usize, usize), serializer: & mut S) -> Result<IndexMapResolver, <S as >::Error>` - Serializes an iterator of key-value pairs as an index map.

**Traits:** Eq, Portable

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as Fallible>::Error>`



## rkyv::collections::swiss_table::index_map::IndexMapResolver

*Struct*

The resolver for an `IndexMap`.



## rkyv::collections::swiss_table::index_map::Iter

*Struct*

An iterator over the key-value pairs of an index map.

**Generic Parameters:**
- 'a
- K
- V

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::swiss_table::index_map::Keys

*Struct*

An iterator over the keys of an index map.

**Generic Parameters:**
- 'a
- K
- V

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## rkyv::collections::swiss_table::index_map::Values

*Struct*

An iterator over the values of an index map.

**Generic Parameters:**
- 'a
- K
- V

**Traits:** ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



