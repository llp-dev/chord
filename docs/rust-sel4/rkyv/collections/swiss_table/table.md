**rkyv > collections > swiss_table > table**

# Module: collections::swiss_table::table

## Contents

**Structs**

- [`ArchivedHashTable`](#archivedhashtable) - A low-level archived SwissTable hash table with explicit hashing.
- [`HashTableResolver`](#hashtableresolver) - The resolver for [`ArchivedHashTable`].
- [`RawIter`](#rawiter) - An iterator over the entry pointers of an [`ArchivedHashTable`].

---

## rkyv::collections::swiss_table::table::ArchivedHashTable

*Struct*

A low-level archived SwissTable hash table with explicit hashing.

**Generic Parameters:**
- T

**Methods:**

- `fn get_with<C>(self: &Self, hash: u64, cmp: C) -> Option<&T>` - Returns the key-value pair corresponding to the supplied key.
- `fn get_seal_with<C>(this: Seal<Self>, hash: u64, cmp: C) -> Option<Seal<T>>` - Returns the mutable key-value pair corresponding to the supplied key.
- `fn is_empty(self: &Self) -> bool` - Returns whether the hash table is empty.
- `fn len(self: &Self) -> usize` - Returns the number of elements in the hash table.
- `fn capacity(self: &Self) -> usize` - Returns the total capacity of the hash table.
- `fn raw_iter(self: &Self) -> RawIter<T>` - Returns an iterator over the entry pointers in the hash table.
- `fn raw_iter_seal(this: Seal<Self>) -> RawIter<T>` - Returns a sealed iterator over the entry pointers in the hash table.
- `fn serialize_from_iter<I, U, H, S>(items: I, hashes: H, load_factor: (usize, usize), serializer: & mut S) -> Result<HashTableResolver, <S as >::Error>` - Serializes an iterator of items as a hash table.
- `fn resolve_from_len(len: usize, load_factor: (usize, usize), resolver: HashTableResolver, out: Place<Self>)` - Resolves an archived hash table from a given length and parameters.

**Traits:** Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`



## rkyv::collections::swiss_table::table::HashTableResolver

*Struct*

The resolver for [`ArchivedHashTable`].



## rkyv::collections::swiss_table::table::RawIter

*Struct*

An iterator over the entry pointers of an [`ArchivedHashTable`].

**Generic Parameters:**
- T

**Methods:**

- `fn empty() -> Self` - Returns a raw iterator which yields no elements.

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



