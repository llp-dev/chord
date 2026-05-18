**slab**

# Module: slab

## Contents

**Structs**

- [`Drain`](#drain) - A draining iterator for `Slab`
- [`IntoIter`](#intoiter) - A consuming iterator over the values stored in a `Slab`
- [`Iter`](#iter) - An iterator over the values stored in the `Slab`
- [`IterMut`](#itermut) - A mutable iterator over the values stored in the `Slab`
- [`Slab`](#slab) - Pre-allocated storage for a uniform data type
- [`VacantEntry`](#vacantentry) - A handle to a vacant entry in a `Slab`.

**Enums**

- [`GetDisjointMutError`](#getdisjointmuterror) - The error type returned by [`Slab::get_disjoint_mut`].

---

## slab::Drain

*Struct*

A draining iterator for `Slab`

**Generic Parameters:**
- 'a
- T

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`



## slab::GetDisjointMutError

*Enum*

The error type returned by [`Slab::get_disjoint_mut`].

**Variants:**
- `IndexVacant` - An index provided was not associated with a value.
- `IndexOutOfBounds` - An index provided was out-of-bounds for the slab.
- `OverlappingIndices` - Two indices provided were overlapping.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &GetDisjointMutError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> GetDisjointMutError`



## slab::IntoIter

*Struct*

A consuming iterator over the values stored in a `Slab`

**Generic Parameters:**
- T

**Traits:** FusedIterator

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## slab::Iter

*Struct*

An iterator over the values stored in the `Slab`

**Generic Parameters:**
- 'a
- T

**Traits:** FusedIterator

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## slab::IterMut

*Struct*

A mutable iterator over the values stored in the `Slab`

**Generic Parameters:**
- 'a
- T

**Traits:** FusedIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`



## slab::Slab

*Struct*

Pre-allocated storage for a uniform data type

See the [module documentation] for more details.

[module documentation]: index.html

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Construct a new, empty `Slab`.
- `fn with_capacity(capacity: usize) -> Slab<T>` - Construct a new, empty `Slab` with the specified capacity.
- `fn capacity(self: &Self) -> usize` - Return the number of values the slab can store without reallocating.
- `fn reserve(self: & mut Self, additional: usize)` - Reserve capacity for at least `additional` more values to be stored
- `fn reserve_exact(self: & mut Self, additional: usize)` - Reserve the minimum capacity required to store exactly `additional`
- `fn shrink_to_fit(self: & mut Self)` - Shrink the capacity of the slab as much as possible without invalidating keys.
- `fn compact<F>(self: & mut Self, rekey: F)` - Reduce the capacity as much as possible, changing the key for elements when necessary.
- `fn clear(self: & mut Self)` - Clear the slab of all values.
- `fn len(self: &Self) -> usize` - Return the number of stored values.
- `fn is_empty(self: &Self) -> bool` - Return `true` if there are no values stored in the slab.
- `fn iter(self: &Self) -> Iter<T>` - Return an iterator over the slab.
- `fn iter_mut(self: & mut Self) -> IterMut<T>` - Return an iterator that allows modifying each value.
- `fn get(self: &Self, key: usize) -> Option<&T>` - Return a reference to the value associated with the given key.
- `fn get_mut(self: & mut Self, key: usize) -> Option<& mut T>` - Return a mutable reference to the value associated with the given key.
- `fn get2_mut(self: & mut Self, key1: usize, key2: usize) -> Option<(& mut T, & mut T)>` - Return two mutable references to the values associated with the two
- `fn get_disjoint_mut<const N>(self: & mut Self, keys: [usize; N]) -> Result<[& mut T; N], GetDisjointMutError>` - Returns mutable references to many indices at once.
- `fn get_unchecked(self: &Self, key: usize) -> &T` - Return a reference to the value associated with the given key without
- `fn get_unchecked_mut(self: & mut Self, key: usize) -> & mut T` - Return a mutable reference to the value associated with the given key
- `fn get2_unchecked_mut(self: & mut Self, key1: usize, key2: usize) -> (& mut T, & mut T)` - Return two mutable references to the values associated with the two
- `fn key_of(self: &Self, present_element: &T) -> usize` - Get the key for an element in the slab.
- `fn insert(self: & mut Self, val: T) -> usize` - Insert a value in the slab, returning key assigned to the value.
- `fn vacant_key(self: &Self) -> usize` - Returns the key of the next vacant entry.
- `fn vacant_entry(self: & mut Self) -> VacantEntry<T>` - Return a handle to a vacant entry allowing for further manipulation.
- `fn try_remove(self: & mut Self, key: usize) -> Option<T>` - Tries to remove the value associated with the given key,
- `fn remove(self: & mut Self, key: usize) -> T` - Remove and return the value associated with the given key.
- `fn contains(self: &Self, key: usize) -> bool` - Return `true` if a value is associated with the given key.
- `fn retain<F>(self: & mut Self, f: F)` - Retain only the elements specified by the predicate.
- `fn drain(self: & mut Self) -> Drain<T>` - Return a draining iterator that removes all elements from the slab and

**Trait Implementations:**

- **Index**
  - `fn index(self: &Self, key: usize) -> &T`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`
- **FromIterator**
  - `fn from_iter<I>(iterable: I) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
  - `fn clone_from(self: & mut Self, source: &Self)`
- **IntoIterator**
  - `fn into_iter(self: Self) -> IntoIter<T>`
- **IndexMut**
  - `fn index_mut(self: & mut Self, key: usize) -> & mut T`



## slab::VacantEntry

*Struct*

A handle to a vacant entry in a `Slab`.

`VacantEntry` allows constructing values with the key that they will be
assigned to.

# Examples

```
# use slab::*;
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab[hello].0);
assert_eq!("hello", slab[hello].1);
```

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn insert(self: Self, val: T) -> &'a  mut T` - Insert a value in the entry, returning a mutable reference to the value.
- `fn key(self: &Self) -> usize` - Return the key associated with this entry.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



