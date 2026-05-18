**heapless > linear_map**

# Module: linear_map

## Contents

**Structs**

- [`IntoIter`](#intoiter) - An iterator that moves out of a [`LinearMap`].
- [`Iter`](#iter) - An iterator over the items of a [`LinearMap`]
- [`IterMut`](#itermut) - An iterator over the items of a [`LinearMap`] that allows modifying the items
- [`LinearMapInner`](#linearmapinner) - Base struct for [`LinearMap`] and [`LinearMapView`]

**Type Aliases**

- [`LinearMap`](#linearmap) - A fixed capacity map/dictionary that performs lookups via linear search.
- [`LinearMapView`](#linearmapview) - A dynamic capacity map/dictionary that performs lookups via linear search.
- [`OwnedStorage`](#ownedstorage) - Implementation of [`LinearMapStorage`] that stores the data in an array whose size is known at compile time.
- [`ViewStorage`](#viewstorage) - Implementation of [`LinearMapStorage`] that stores the data in an unsized slice.

---

## heapless::linear_map::IntoIter

*Struct*

An iterator that moves out of a [`LinearMap`].

This struct is created by calling the [`into_iter`](LinearMap::into_iter) method on [`LinearMap`].

**Generic Parameters:**
- K
- V
- const N

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::linear_map::Iter

*Struct*

An iterator over the items of a [`LinearMap`]

This struct is created by calling the [`iter`](LinearMap::iter) method on [`LinearMap`].

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> Iter<'a, K, V>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## heapless::linear_map::IterMut

*Struct*

An iterator over the items of a [`LinearMap`] that allows modifying the items

This struct is created by calling the [`iter_mut`](LinearMap::iter_mut) method on [`LinearMap`].

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::linear_map::LinearMap

*Type Alias*: `LinearMapInner<K, V, OwnedStorage<K, V, N>>`

A fixed capacity map/dictionary that performs lookups via linear search.

Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).



## heapless::linear_map::LinearMapInner

*Struct*

Base struct for [`LinearMap`] and [`LinearMapView`]

**Generic Parameters:**
- K
- V
- S

**Methods:**

- `fn new() -> Self` - Creates an empty `LinearMap`.
- `fn as_view(self: &Self) -> &LinearMapView<K, V>` - Get a reference to the `LinearMap`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut LinearMapView<K, V>` - Get a mutable reference to the `LinearMap`, erasing the `N` const-generic.
- `fn capacity(self: &Self) -> usize` - Returns the number of elements that the map can hold.
- `fn clear(self: & mut Self)` - Clears the map, removing all key-value pairs.
- `fn contains_key(self: &Self, key: &K) -> bool` - Returns true if the map contains a value for the specified key.
- `fn get<Q>(self: &Self, key: &Q) -> Option<&V>` - Returns a reference to the value corresponding to the key.
- `fn get_mut<Q>(self: & mut Self, key: &Q) -> Option<& mut V>` - Returns a mutable reference to the value corresponding to the key.
- `fn len(self: &Self) -> usize` - Returns the number of elements in this map.
- `fn insert(self: & mut Self, key: K, value: V) -> Result<Option<V>, (K, V)>` - Inserts a key-value pair into the map.
- `fn is_empty(self: &Self) -> bool` - Returns true if the map contains no elements.
- `fn is_full(self: &Self) -> bool` - Returns true if the map is full.
- `fn iter(self: &Self) -> Iter<K, V>` - An iterator visiting all key-value pairs in arbitrary order.
- `fn iter_mut(self: & mut Self) -> IterMut<K, V>` - An iterator visiting all key-value pairs in arbitrary order,
- `fn keys(self: &Self) -> impl Trait` - An iterator visiting all keys in arbitrary order.
- `fn remove<Q>(self: & mut Self, key: &Q) -> Option<V>` - Removes a key from the map, returning the value at
- `fn values(self: &Self) -> impl Trait` - An iterator visiting all values in arbitrary order.
- `fn values_mut(self: & mut Self) -> impl Trait` - An iterator visiting all values mutably in arbitrary order.

**Traits:** Eq

**Trait Implementations:**

- **IndexMut**
  - `fn index_mut(self: & mut Self, key: &Q) -> & mut V`
- **Serialize**
  - `fn serialize<SER>(self: &Self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Index**
  - `fn index(self: &Self, key: &Q) -> &V`
- **PartialEq**
  - `fn eq(self: &Self, other: &LinearMapInner<K, V2, S2>) -> bool`



## heapless::linear_map::LinearMapView

*Type Alias*: `LinearMapInner<K, V, ViewStorage<K, V>>`

A dynamic capacity map/dictionary that performs lookups via linear search.

Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).



## heapless::linear_map::OwnedStorage

*Type Alias*: `crate::vec::OwnedVecStorage<(K, V), N>`

Implementation of [`LinearMapStorage`] that stores the data in an array whose size is known at compile time.



## heapless::linear_map::ViewStorage

*Type Alias*: `crate::vec::ViewVecStorage<(K, V)>`

Implementation of [`LinearMapStorage`] that stores the data in an unsized slice.



