**rkyv > collections > util**

# Module: collections::util

## Contents

**Structs**

- [`Entry`](#entry) - A key-value entry.
- [`EntryAdapter`](#entryadapter) - An adapter which serializes and resolves its key and value references.
- [`EntryResolver`](#entryresolver) - A resolver for a key-value pair.
- [`IteratorLengthMismatch`](#iteratorlengthmismatch) - An error describing that an iterator's length did not match the number of

---

## rkyv::collections::util::Entry

*Struct*

A key-value entry.

**Generic Parameters:**
- K
- V

**Fields:**
- `key: K` - The entry's key.
- `value: V` - The entry's value.

**Traits:** Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Entry<K, V>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Entry<K, V>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Entry<K, V>) -> $crate::cmp::Ordering`



## rkyv::collections::util::EntryAdapter

*Struct*

An adapter which serializes and resolves its key and value references.

**Generic Parameters:**
- BK
- BV
- K
- V

**Fields:**
- `key: BK` - The key to serialize and resolve.
- `value: BV` - The value to serialize and resolve.

**Methods:**

- `fn new(key: BK, value: BV) -> Self` - Returns a new `EntryAdapter` for the given key and value.

**Trait Implementations:**

- **Serialize**
  - `fn serialize(self: &Self, serializer: & mut S) -> Result<<Self as >::Resolver, <S as >::Error>`
- **Archive**
  - `fn resolve(self: &Self, resolver: <Self as >::Resolver, out: Place<<Self as >::Archived>)`



## rkyv::collections::util::EntryResolver

*Struct*

A resolver for a key-value pair.

**Generic Parameters:**
- K
- V

**Fields:**
- `key: K` - The key resolver.
- `value: V` - The value resolver.



## rkyv::collections::util::IteratorLengthMismatch

*Struct*

An error describing that an iterator's length did not match the number of
elements it yielded.

**Fields:**
- `expected: usize` - The number of expected elements.
- `actual: usize` - The actual number of elements.

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



