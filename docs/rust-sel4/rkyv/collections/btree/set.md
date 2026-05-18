**rkyv > collections > btree > set**

# Module: collections::btree::set

## Contents

**Structs**

- [`ArchivedBTreeSet`](#archivedbtreeset) - An archived `BTreeSet`. This is a wrapper around a B-tree map with the same
- [`BTreeSetResolver`](#btreesetresolver) - The resolver for archived B-tree sets.

---

## rkyv::collections::btree::set::ArchivedBTreeSet

*Struct*

An archived `BTreeSet`. This is a wrapper around a B-tree map with the same
key and a value of `()`.

**Generic Parameters:**
- K
- const E

**Tuple Struct**: `()`

**Methods:**

- `fn iter(self: &Self) -> Iter<K, E>` - Returns an iterator over the items of the archived B-tree set.
- `fn contains_key<Q>(self: &Self, key: &Q) -> bool` - Returns `true` if the set contains a value for the specified key.
- `fn get<Q>(self: &Self, value: &Q) -> Option<&K>` - Returns a reference to the value in the set, if any, that is equal to
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the set contains no elements.
- `fn len(self: &Self) -> usize` - Returns the number of items in the archived B-tree set.
- `fn resolve_from_len(len: usize, resolver: BTreeSetResolver, out: Place<Self>)` - Resolves a B-tree set from its length.
- `fn serialize_from_ordered_iter<I, KU, S>(iter: I, serializer: & mut S) -> Result<BTreeSetResolver, <S as >::Error>` - Serializes an `ArchivedBTreeSet` from the given iterator and serializer.
- `fn visit<T, impl FnMut(&K) -> ControlFlow<T>>(self: &Self, f: impl Trait) -> Option<T>` - Visits every key in the B-tree with a function.

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &BTreeSet<K>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<BTreeSet<K>, <D as >::Error>`



## rkyv::collections::btree::set::BTreeSetResolver

*Struct*

The resolver for archived B-tree sets.

**Tuple Struct**: `()`



