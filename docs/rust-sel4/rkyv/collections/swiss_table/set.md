**rkyv > collections > swiss_table > set**

# Module: collections::swiss_table::set

## Contents

**Structs**

- [`ArchivedHashSet`](#archivedhashset) - An archived `HashSet`. This is a wrapper around a hash map with the same key
- [`HashSetResolver`](#hashsetresolver) - The resolver for archived hash sets.

---

## rkyv::collections::swiss_table::set::ArchivedHashSet

*Struct*

An archived `HashSet`. This is a wrapper around a hash map with the same key
and unit value.

**Generic Parameters:**
- K
- H

**Methods:**

- `fn len(self: &Self) -> usize` - Gets the number of items in the hash set.
- `fn is_empty(self: &Self) -> bool` - Returns whether there are no items in the hash set.
- `fn iter(self: &Self) -> Keys<K, (), H>` - Gets an iterator over the keys of the underlying hash map.
- `fn get<Q>(self: &Self, k: &Q) -> Option<&K>` - Gets the key corresponding to the given key in the hash set.
- `fn contains<Q>(self: &Self, k: &Q) -> bool` - Returns whether the given key is in the hash set.
- `fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: HashSetResolver, out: Place<Self>)` - Resolves an archived hash set from the given length and parameters.
- `fn serialize_from_iter<I, KU, S>(iter: I, load_factor: (usize, usize), serializer: & mut S) -> Result<HashSetResolver, <S as >::Error>` - Serializes an iterator of keys as a hash set.

**Traits:** Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &HashSet<K, S>) -> bool`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<HashSet<K, S>, <D as >::Error>`



## rkyv::collections::swiss_table::set::HashSetResolver

*Struct*

The resolver for archived hash sets.

**Tuple Struct**: `()`



