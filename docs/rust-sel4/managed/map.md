**managed > map**

# Module: map

## Contents

**Structs**

- [`Range`](#range)

**Enums**

- [`Iter`](#iter)
- [`IterMut`](#itermut)
- [`ManagedMap`](#managedmap) - A managed map.

---

## managed::map::Iter

*Enum*

**Generic Parameters:**
- 'a
- K
- V

**Variants:**
- `Borrowed(slice::Iter<'a, Option<(K, V)>>)` - Borrowed variant.
- `Owned(alloc::collections::btree_map::Iter<'a, K, V>)` - Owned variant, only available with the `std` or `alloc` feature enabled.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## managed::map::IterMut

*Enum*

**Generic Parameters:**
- 'a
- K
- V

**Variants:**
- `Borrowed(slice::IterMut<'a, Option<(K, V)>>)` - Borrowed variant.
- `Owned(alloc::collections::btree_map::IterMut<'a, K, V>)` - Owned variant, only available with the `std` or `alloc` feature enabled.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## managed::map::ManagedMap

*Enum*

A managed map.

This enum can be used to represent exclusive access to maps.
In Rust, exclusive access to an object is obtained by either owning the object,
or owning a mutable pointer to the object; hence, "managed".

The purpose of this enum is providing good ergonomics with `std` present while making
it possible to avoid having a heap at all (which of course means that `std` is not present).
To achieve this, the variants other than `Borrow` are only available when the corresponding
feature is opted in.

Unlike [Managed](enum.Managed.html) and [ManagedSlice](enum.ManagedSlice.html),
the managed map is implemented using a B-tree map when allocation is available,
and a sorted slice of key-value pairs when it is not. Thus, algorithmic complexity
of operations on it depends on the kind of map.

A function that requires a managed object should be generic over an `Into<ManagedMap<'a, T>>`
argument; then, it will be possible to pass either a `Vec<T>`, or a `&'a mut [T]`
without any conversion at the call site.

See also [Managed](enum.Managed.html).

**Generic Parameters:**
- 'a
- K
- V

**Variants:**
- `Borrowed(&'a  mut [Option<(K, V)>])` - Borrowed variant.
- `Owned(alloc::collections::btree_map::BTreeMap<K, V>)` - Owned variant, only available with the `std` or `alloc` feature enabled.

**Methods:**

- `fn clear(self: & mut Self)`
- `fn get<Q>(self: &Self, key: &Q) -> Option<&V>`
- `fn get_mut<Q>(self: & mut Self, key: &Q) -> Option<& mut V>`
- `fn range<'b, 'c, Q, R>(self: &'b Self, range: R) -> Range<'a, K, V>`
- `fn insert(self: & mut Self, key: K, new_value: V) -> Result<Option<V>, (K, V)>`
- `fn remove<Q>(self: & mut Self, key: &Q) -> Option<V>`
- `fn is_empty(self: &Self) -> bool` - ManagedMap contains no elements?
- `fn len(self: &Self) -> usize` - Returns the number of elements in the ManagedMap.
- `fn iter(self: &Self) -> Iter<K, V>`
- `fn iter_mut(self: & mut Self) -> IterMut<K, V>`

**Trait Implementations:**

- **From**
  - `fn from(value: BTreeMap<K, V>) -> Self`
- **From**
  - `fn from(value: &'a  mut [Option<(K, V)>]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## managed::map::Range

*Struct*

**Generic Parameters:**
- 'a
- K
- V

**Tuple Struct**: `()`



