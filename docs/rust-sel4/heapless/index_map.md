**heapless > index_map**

# Module: index_map

## Contents

**Structs**

- [`IndexMap`](#indexmap) - Fixed capacity [`IndexMap`](https://docs.rs/indexmap/2/indexmap/map/struct.IndexMap.html)
- [`IntoIter`](#intoiter) - An owning iterator over the entries of an `IndexMap`.
- [`Iter`](#iter) - An iterator over the items of a [`IndexMap`].
- [`IterMut`](#itermut) - A mutable iterator over the items of a [`IndexMap`].
- [`Keys`](#keys) - An iterator over the keys of a [`IndexMap`].
- [`OccupiedEntry`](#occupiedentry) - An occupied entry which can be manipulated
- [`VacantEntry`](#vacantentry) - A view into an empty slot in the underlying map
- [`Values`](#values) - An iterator over the values of a [`IndexMap`].
- [`ValuesMut`](#valuesmut) - A mutable iterator over the values of a [`IndexMap`].

**Enums**

- [`Entry`](#entry) - A view into an entry in the map

**Type Aliases**

- [`FnvIndexMap`](#fnvindexmap) - An [`IndexMap`] using the default FNV hasher.

---

## heapless::index_map::Entry

*Enum*

A view into an entry in the map

**Generic Parameters:**
- 'a
- K
- V
- const N

**Variants:**
- `Occupied(OccupiedEntry<'a, K, V, N>)` - The entry corresponding to the key `K` exists in the map
- `Vacant(VacantEntry<'a, K, V, N>)` - The entry corresponding to the key `K` does not exist in the map

**Methods:**

- `fn or_default(self: Self) -> Result<&'a  mut V, V>` - Ensures a value is in the entry by inserting the default value if empty,
- `fn or_insert(self: Self, default: V) -> Result<&'a  mut V, V>` - Ensures a value is in the entry by inserting the default if empty, and
- `fn or_insert_with<F>(self: Self, default: F) -> Result<&'a  mut V, V>` - Ensures a value is in the entry by inserting the result of the default
- `fn or_insert_with_key<F>(self: Self, default: F) -> Result<&'a  mut V, V>` - Ensures a value is in the entry by inserting, if empty, the result of
- `fn key(self: &Self) -> &K` - Returns a reference to this entry's key.
- `fn and_modify<F>(self: Self, f: F) -> Self` - Provides in-place mutable access to an occupied entry before any



## heapless::index_map::FnvIndexMap

*Type Alias*: `IndexMap<K, V, hash32::BuildHasherDefault<hash32::FnvHasher>, N>`

An [`IndexMap`] using the default FNV hasher.

A list of all Methods and Traits available for `FnvIndexMap` can be found in
the [`IndexMap`] documentation.

# Examples
```
use heapless::index_map::FnvIndexMap;

// A hash map with a capacity of 16 key-value pairs allocated on the stack
let mut book_reviews = FnvIndexMap::<_, _, 16>::new();

// review some books.
book_reviews
    .insert("Adventures of Huckleberry Finn", "My favorite book.")
    .unwrap();
book_reviews
    .insert("Grimms' Fairy Tales", "Masterpiece.")
    .unwrap();
book_reviews
    .insert("Pride and Prejudice", "Very enjoyable.")
    .unwrap();
book_reviews
    .insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.")
    .unwrap();

// check for a specific one.
if !book_reviews.contains_key("Les Misérables") {
    println!(
        "We've got {} reviews, but Les Misérables ain't one.",
        book_reviews.len()
    );
}

// oops, this review has a lot of spelling mistakes, let's delete it.
book_reviews.remove("The Adventures of Sherlock Holmes");

// look up the values associated with some keys.
let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
for book in &to_find {
    match book_reviews.get(book) {
        Some(review) => println!("{}: {}", book, review),
        None => println!("{} is unreviewed.", book),
    }
}

// iterate over everything.
for (book, review) in &book_reviews {
    println!("{}: \"{}\"", book, review);
}
```



## heapless::index_map::IndexMap

*Struct*

Fixed capacity [`IndexMap`](https://docs.rs/indexmap/2/indexmap/map/struct.IndexMap.html)

Note that you cannot use `IndexMap` directly, since it is generic around the hashing algorithm
in use. Pick a concrete instantiation like [`FnvIndexMap`] instead
or create your own.

Note that the capacity of the `IndexMap` must be a power of 2.

# Examples

Since `IndexMap` cannot be used directly, we're using its `FnvIndexMap` instantiation
for this example.

```
use heapless::index_map::FnvIndexMap;

// A hash map with a capacity of 16 key-value pairs allocated on the stack
let mut book_reviews = FnvIndexMap::<_, _, 16>::new();

// review some books.
book_reviews
    .insert("Adventures of Huckleberry Finn", "My favorite book.")
    .unwrap();
book_reviews
    .insert("Grimms' Fairy Tales", "Masterpiece.")
    .unwrap();
book_reviews
    .insert("Pride and Prejudice", "Very enjoyable.")
    .unwrap();
book_reviews
    .insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.")
    .unwrap();

// check for a specific one.
if !book_reviews.contains_key("Les Misérables") {
    println!(
        "We've got {} reviews, but Les Misérables ain't one.",
        book_reviews.len()
    );
}

// oops, this review has a lot of spelling mistakes, let's delete it.
book_reviews.remove("The Adventures of Sherlock Holmes");

// look up the values associated with some keys.
let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
for book in &to_find {
    match book_reviews.get(book) {
        Some(review) => println!("{}: {}", book, review),
        None => println!("{} is unreviewed.", book),
    }
}

// iterate over everything.
for (book, review) in &book_reviews {
    println!("{}: \"{}\"", book, review);
}
```

**Generic Parameters:**
- K
- V
- S
- const N

**Methods:**

- `fn capacity(self: &Self) -> usize` - Returns the number of elements the map can hold
- `fn keys(self: &Self) -> Keys<K, V>` - Return an iterator over the keys of the map, in insertion order
- `fn values(self: &Self) -> Values<K, V>` - Return an iterator over the values of the map, in insertion order
- `fn values_mut(self: & mut Self) -> ValuesMut<K, V>` - Return an iterator over mutable references to the the values of the map, in insertion order
- `fn iter(self: &Self) -> Iter<K, V>` - Return an iterator over the key-value pairs of the map, in insertion order
- `fn iter_mut(self: & mut Self) -> IterMut<K, V>` - Return an iterator over the key-value pairs of the map, in insertion order
- `fn first(self: &Self) -> Option<(&K, &V)>` - Get the first key-value pair
- `fn first_mut(self: & mut Self) -> Option<(&K, & mut V)>` - Get the first key-value pair, with mutable access to the value
- `fn last(self: &Self) -> Option<(&K, &V)>` - Get the last key-value pair
- `fn last_mut(self: & mut Self) -> Option<(&K, & mut V)>` - Get the last key-value pair, with mutable access to the value
- `fn len(self: &Self) -> usize` - Return the number of key-value pairs in the map.
- `fn is_empty(self: &Self) -> bool` - Returns true if the map contains no elements.
- `fn is_full(self: &Self) -> bool` - Returns true if the map is full.
- `fn clear(self: & mut Self)` - Remove all key-value pairs in the map, while preserving its capacity.
- `fn new() -> Self` - Creates an empty `IndexMap`.
- `fn entry(self: & mut Self, key: K) -> Entry<K, V, N>` - Returns an entry for the corresponding key
- `fn get<Q>(self: &Self, key: &Q) -> Option<&V>` - Returns a reference to the value corresponding to the key.
- `fn contains_key<Q>(self: &Self, key: &Q) -> bool` - Returns true if the map contains a value for the specified key.
- `fn get_mut<'v, Q>(self: &'v  mut Self, key: &Q) -> Option<&'v  mut V>` - Returns a mutable reference to the value corresponding to the key.
- `fn get_index(self: &Self, index: usize) -> Option<(&K, &V)>` - Returns a tuple of references to the key and the value corresponding to the index.
- `fn get_index_mut(self: & mut Self, index: usize) -> Option<(&K, & mut V)>` - Returns a tuple of references to the key and the mutable value corresponding to the index.
- `fn get_index_of<Q>(self: &Self, key: &Q) -> Option<usize>` - Returns the index of the key-value pair corresponding to the key.
- `fn insert(self: & mut Self, key: K, value: V) -> Result<Option<V>, (K, V)>` - Inserts a key-value pair into the map.
- `fn remove<Q>(self: & mut Self, key: &Q) -> Option<V>` - Same as [`swap_remove`](Self::swap_remove)
- `fn swap_remove<Q>(self: & mut Self, key: &Q) -> Option<V>` - Remove the key-value pair equivalent to `key` and return its value.
- `fn retain<F>(self: & mut Self, f: F)` - Retains only the elements specified by the predicate.
- `fn truncate(self: & mut Self, len: usize)` - Shortens the map, keeping the first `len` elements and dropping the rest.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IndexMap<K, V2, S2, N2>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Index**
  - `fn index(self: &Self, key: &Q) -> &V`
- **FromIterator**
  - `fn from_iter<I>(iterable: I) -> Self`
- **Extend**
  - `fn extend<I>(self: & mut Self, iterable: I)`
- **Serialize**
  - `fn serialize<SER>(self: &Self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`
- **IndexMut**
  - `fn index_mut(self: & mut Self, key: &Q) -> & mut V`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Extend**
  - `fn extend<I>(self: & mut Self, iterable: I)`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`



## heapless::index_map::IntoIter

*Struct*

An owning iterator over the entries of an `IndexMap`.

This `struct` is created by the [`into_iter`] method on [`IndexMap`]
(provided by the [`IntoIterator`] trait). See its documentation for more.

[`into_iter`]: IntoIterator::into_iter

# Example

```
use heapless::index_map::FnvIndexMap;

let mut map = FnvIndexMap::<_, _, 16>::new();
map.insert("a", 1).unwrap();

let iter = map.into_iter();
```

**Generic Parameters:**
- K
- V
- const N

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> IntoIter<K, V, N>`



## heapless::index_map::Iter

*Struct*

An iterator over the items of a [`IndexMap`].

This `struct` is created by the [`iter`](IndexMap::iter) method on [`IndexMap`]. See its
documentation for more.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::index_map::IterMut

*Struct*

A mutable iterator over the items of a [`IndexMap`].

This `struct` is created by the [`iter_mut`](IndexMap::iter_mut) method on [`IndexMap`]. See its
documentation for more.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::index_map::Keys

*Struct*

An iterator over the keys of a [`IndexMap`].

This `struct` is created by the [`keys`](IndexMap::keys) method on [`IndexMap`]. See its
documentation for more.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::index_map::OccupiedEntry

*Struct*

An occupied entry which can be manipulated

**Generic Parameters:**
- 'a
- K
- V
- const N

**Methods:**

- `fn key(self: &Self) -> &K` - Gets a reference to the key that this entity corresponds to
- `fn remove_entry(self: Self) -> (K, V)` - Removes this entry from the map and yields its corresponding key and value
- `fn get(self: &Self) -> &V` - Gets a reference to the value associated with this entry
- `fn get_mut(self: & mut Self) -> & mut V` - Gets a mutable reference to the value associated with this entry
- `fn into_mut(self: Self) -> &'a  mut V` - Consumes this entry and yields a reference to the underlying value
- `fn insert(self: Self, value: V) -> V` - Overwrites the underlying map's value with this entry's value
- `fn remove(self: Self) -> V` - Removes this entry from the map and yields its value



## heapless::index_map::VacantEntry

*Struct*

A view into an empty slot in the underlying map

**Generic Parameters:**
- 'a
- K
- V
- const N

**Methods:**

- `fn key(self: &Self) -> &K` - Get the key associated with this entry
- `fn into_key(self: Self) -> K` - Consumes this entry to yield to key associated with it
- `fn insert(self: Self, value: V) -> Result<&'a  mut V, V>` - Inserts this entry into to underlying map, yields a mutable reference to the inserted value.



## heapless::index_map::Values

*Struct*

An iterator over the values of a [`IndexMap`].

This `struct` is created by the [`values`](IndexMap::values) method on [`IndexMap`]. See its
documentation for more.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::index_map::ValuesMut

*Struct*

A mutable iterator over the values of a [`IndexMap`].

This `struct` is created by the [`values_mut`](IndexMap::values_mut) method on [`IndexMap`]. See its
documentation for more.

**Generic Parameters:**
- 'a
- K
- V

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



