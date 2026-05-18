**rkyv > collections > btree > map**

# Module: collections::btree::map

## Contents

**Structs**

- [`ArchivedBTreeMap`](#archivedbtreemap) - An archived [`BTreeMap`](crate::alloc::collections::BTreeMap).
- [`BTreeMapResolver`](#btreemapresolver) - The resolver for [`ArchivedBTreeMap`].

---

## rkyv::collections::btree::map::ArchivedBTreeMap

*Struct*

An archived [`BTreeMap`](crate::alloc::collections::BTreeMap).

**Generic Parameters:**
- K
- V
- const E

**Methods:**

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool` - Returns whether the B-tree map contains the given key.
- `fn get<Q>(self: &Self, key: &Q) -> Option<&V>` - Returns the value associated with the given key, or `None` if the key is
- `fn get_seal<'a, Q>(this: Seal<'a, Self>, key: &Q) -> Option<Seal<'a, V>>` - Returns the mutable value associated with the given key, or `None` if
- `fn is_empty(self: &Self) -> bool` - Returns true if the B-tree map contains no entries.
- `fn len(self: &Self) -> usize` - Returns the number of entries in the B-tree map.
- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&K, &V)>` - Gets the key-value pair associated with the given key, or `None` if the
- `fn get_key_value_with<Q, C>(self: &Self, key: &Q, cmp: C) -> Option<(&K, &V)>` - Gets the key-value pair associated with the given key, or `None` if the
- `fn get_key_value_seal<'a, Q>(this: Seal<'a, Self>, key: &Q) -> Option<(&'a K, Seal<'a, V>)>` - Gets the mutable key-value pair associated with the given key, or `None`
- `fn get_key_value_seal_with<'a, Q, C>(this: Seal<'a, Self>, key: &Q, cmp: C) -> Option<(&'a K, Seal<'a, V>)>` - Gets the mutable key-value pair associated with the given key, or `None`
- `fn resolve_from_len(len: usize, resolver: BTreeMapResolver, out: Place<Self>)` - Resolves an `ArchivedBTreeMap` from the given length, resolver, and
- `fn serialize_from_ordered_iter<I, BKU, BVU, KU, VU, S>(iter: I, serializer: & mut S) -> Result<BTreeMapResolver, <S as >::Error>` - Serializes an `ArchivedBTreeMap` from the given iterator and serializer.
- `fn visit<T, impl FnMut(&K, &V) -> ControlFlow<T>>(self: &Self, f: impl Trait) -> Option<T>` - Visits every key-value pair in the B-tree with a function.
- `fn visit_seal<T, impl FnMut(&K, Seal<'_, V>) -> ControlFlow<T>>(this: Seal<Self>, f: impl Trait) -> Option<T>` - Visits every mutable key-value pair in the B-tree with a function.
- `fn iter(self: &Self) -> Iter<K, V, E>` - Gets an iterator over the entries of the map, sorted by key.
- `fn iter_seal(this: Seal<Self>) -> IterSeal<K, V, E>` - Gets a mutable iterator over the entires of the map, sorted by key.
- `fn keys(self: &Self) -> Keys<K, V, E>` - Gets an iterator over the sorted keys of the map.
- `fn values(self: &Self) -> Values<K, V, E>` - Gets an iterator over the values of the map.
- `fn values_seal(this: Seal<Self>) -> ValuesSeal<K, V, E>` - Gets a mutable iterator over the values of the map.
- `fn range<Q, R>(self: &Self, range: R) -> Range<K, V, E>` - Gets an iterator over a sub-range of entries, sorted by key.
- `fn range_with<Q, R, C>(self: &Self, range: R, cmp: C) -> Range<K, V, E>` - Gets an iterator over a sub-range of entries, sorted by key.
- `fn range_seal<Q, R>(this: Seal<Self>, range: R) -> RangeSeal<K, V, E>` - Gets a mutable iterator over a sub-range of entries, sorted by key.
- `fn range_seal_with<Q, R, C>(this: Seal<Self>, range: R, cmp: C) -> RangeSeal<K, V, E>` - Gets a mutable iterator over a sub-range of entries, sorted by key.

**Traits:** Eq, Portable

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &BTreeMap<K, V>) -> bool`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedBTreeMap<K, V, E2>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<BTreeMap<K, V>, <D as >::Error>`
- **Index**
  - `fn index(self: &Self, key: &Q) -> &<Self as >::Output`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`



## rkyv::collections::btree::map::BTreeMapResolver

*Struct*

The resolver for [`ArchivedBTreeMap`].



