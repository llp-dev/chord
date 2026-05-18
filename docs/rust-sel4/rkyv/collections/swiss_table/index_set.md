**rkyv > collections > swiss_table > index_set**

# Module: collections::swiss_table::index_set

## Contents

**Structs**

- [`ArchivedIndexSet`](#archivedindexset) - An archived `IndexSet`.
- [`IndexSetResolver`](#indexsetresolver) - The resolver for archived index sets.

---

## rkyv::collections::swiss_table::index_set::ArchivedIndexSet

*Struct*

An archived `IndexSet`.

**Generic Parameters:**
- K
- H

**Methods:**

- `fn is_empty(self: &Self) -> bool` - Returns whether the index set contains no values.
- `fn iter(self: &Self) -> Keys<K, ()>` - Returns an iterator over the keys of the index set in order.
- `fn len(self: &Self) -> usize` - Returns the number of elements in the index set.
- `fn contains<Q>(self: &Self, k: &Q) -> bool` - Returns whether a key is present in the hash set.
- `fn get<Q>(self: &Self, k: &Q) -> Option<&K>` - Returns the value stored in the set, if any.
- `fn get_full<Q>(self: &Self, k: &Q) -> Option<(usize, &K)>` - Returns the item index and value stored in the set, if any.
- `fn get_index(self: &Self, index: usize) -> Option<&K>` - Gets a key by index.
- `fn get_index_of<Q>(self: &Self, key: &Q) -> Option<usize>` - Returns the index of a key if it exists in the set.
- `fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: IndexSetResolver, out: Place<Self>)` - Resolves an archived index map from a given length and parameters.
- `fn serialize_from_iter<I, UK, S>(iter: I, load_factor: (usize, usize), serializer: & mut S) -> Result<IndexSetResolver, <S as >::Error>` - Serializes an iterator of keys as an index set.

**Traits:** Eq, Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## rkyv::collections::swiss_table::index_set::IndexSetResolver

*Struct*

The resolver for archived index sets.

**Tuple Struct**: `()`



