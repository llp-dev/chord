**rkyv > collections > swiss_table > map**

# Module: collections::swiss_table::map

## Contents

**Structs**

- [`ArchivedHashMap`](#archivedhashmap) - An archived SwissTable hash map.
- [`HashMapResolver`](#hashmapresolver) - The resolver for [`ArchivedHashMap`].
- [`Iter`](#iter) - An iterator over the key-value pairs of an [`ArchivedHashMap`].
- [`IterMut`](#itermut) - An iterator over the mutable key-value pairs of an [`ArchivedHashMap`].
- [`Keys`](#keys) - An iterator over the keys of an [`ArchivedHashMap`].
- [`Values`](#values) - An iterator over the values of an [`ArchivedHashMap`].
- [`ValuesMut`](#valuesmut) - An iterator over the mutable values of an [`ArchivedHashMap`].

---

## rkyv::collections::swiss_table::map::ArchivedHashMap

*Struct*

An archived SwissTable hash map.

**Generic Parameters:**
- K
- V
- H

**Methods:**

- `fn get_key_value_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<(&K, &V)>` - Returns the key-value pair corresponding to the supplied key using the
- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>` - Returns the key-value pair corresponding to the supplied key.
- `fn get_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<&V>` - Returns a reference to the value corresponding to the supplied key using
- `fn get<Q>(self: &Self, key: &Q) -> Option<&V>` - Returns a reference to the value corresponding to the supplied key.
- `fn get_key_value_seal_with<'a, Q, C>(this: Seal<'a, Self>, key: &Q, cmp: C) -> Option<(&'a K, Seal<'a, V>)>` - Returns the mutable key-value pair corresponding to the supplied key
- `fn get_key_value_seal<'a, Q>(this: Seal<'a, Self>, key: &Q) -> Option<(&'a K, Seal<'a, V>)>` - Returns the mutable key-value pair corresponding to the supplied key.
- `fn get_seal_with<'a, Q, C>(this: Seal<'a, Self>, key: &Q, cmp: C) -> Option<Seal<'a, V>>` - Returns a mutable reference to the value corresponding to the supplied
- `fn get_seal<'a, Q>(this: Seal<'a, Self>, key: &Q) -> Option<Seal<'a, V>>` - Returns a mutable reference to the value corresponding to the supplied
- `fn contains_key<Q>(self: &Self, key: &Q) -> bool` - Returns whether the hash map contains the given key.
- `fn serialize_from_iter<I, BKU, BVU, KU, VU, S>(iter: I, load_factor: (usize, usize), serializer: & mut S) -> Result<HashMapResolver, <S as >::Error>` - Serializes an iterator of key-value pairs as a hash map.
- `fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: HashMapResolver, out: Place<Self>)` - Resolves an archived hash map from a given length and parameters.
- `fn is_empty(self: &Self) -> bool` - Returns whether the hash map is empty.
- `fn len(self: &Self) -> usize` - Returns the number of elements in the hash map.
- `fn capacity(self: &Self) -> usize` - Returns the total capacity of the hash map.
- `fn iter(self: &Self) -> Iter<K, V, H>` - Returns an iterator over the key-value entries in the hash map.
- `fn iter_seal(this: Seal<Self>) -> IterMut<K, V, H>` - Returns an iterator over the sealed key-value entries in the hash map.
- `fn keys(self: &Self) -> Keys<K, V, H>` - Returns an iterator over the keys in the hash map.
- `fn values(self: &Self) -> Values<K, V, H>` - Returns an iterator over the values in the hash map.
- `fn values_seal(this: Seal<Self>) -> ValuesMut<K, V, H>` - Returns an iterator over the mutable values in the hash map.

**Traits:** Portable, Eq

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Index**
  - `fn index(self: &Self, key: &Q) -> &V`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<HashMap<K, V, S>, <D as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &HashMap<K, V, S>) -> bool`



## rkyv::collections::swiss_table::map::HashMapResolver

*Struct*

The resolver for [`ArchivedHashMap`].

**Tuple Struct**: `()`



## rkyv::collections::swiss_table::map::Iter

*Struct*

An iterator over the key-value pairs of an [`ArchivedHashMap`].

**Generic Parameters:**
- 'a
- K
- V
- H

**Traits:** FusedIterator

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## rkyv::collections::swiss_table::map::IterMut

*Struct*

An iterator over the mutable key-value pairs of an [`ArchivedHashMap`].

**Generic Parameters:**
- 'a
- K
- V
- H

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## rkyv::collections::swiss_table::map::Keys

*Struct*

An iterator over the keys of an [`ArchivedHashMap`].

**Generic Parameters:**
- 'a
- K
- V
- H

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## rkyv::collections::swiss_table::map::Values

*Struct*

An iterator over the values of an [`ArchivedHashMap`].

**Generic Parameters:**
- 'a
- K
- V
- H

**Traits:** FusedIterator

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## rkyv::collections::swiss_table::map::ValuesMut

*Struct*

An iterator over the mutable values of an [`ArchivedHashMap`].

**Generic Parameters:**
- 'a
- K
- V
- H

**Traits:** FusedIterator

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



