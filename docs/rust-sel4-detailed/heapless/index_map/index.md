*[heapless](../index.md) / [index_map](index.md)*

---

# Module `index_map`

A fixed-capacity hash table where the iteration order is independent of the hash of the keys.

## Contents

- [Structs](#structs)
  - [`HashValue`](#hashvalue)
  - [`Inserted`](#inserted)
  - [`CoreMap`](#coremap)
  - [`OccupiedEntry`](#occupiedentry)
  - [`VacantEntry`](#vacantentry)
  - [`IndexMap`](#indexmap)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`Keys`](#keys)
  - [`Values`](#values)
  - [`ValuesMut`](#valuesmut)
- [Enums](#enums)
  - [`Insert`](#insert)
  - [`Entry`](#entry)
- [Functions](#functions)
  - [`hash_with`](#hash-with)
- [Type Aliases](#type-aliases)
  - [`FnvIndexMap`](#fnvindexmap)
- [Macros](#macros)
  - [`probe_loop!`](#probe-loop)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HashValue`](#hashvalue) | struct |  |
| [`Inserted`](#inserted) | struct |  |
| [`CoreMap`](#coremap) | struct |  |
| [`OccupiedEntry`](#occupiedentry) | struct | An occupied entry which can be manipulated |
| [`VacantEntry`](#vacantentry) | struct | A view into an empty slot in the underlying map |
| [`IndexMap`](#indexmap) | struct | Fixed capacity [`IndexMap`](https://docs.rs/indexmap/2/indexmap/map/struct.IndexMap.html) |
| [`IntoIter`](#intoiter) | struct | An owning iterator over the entries of an `IndexMap`. |
| [`Iter`](#iter) | struct | An iterator over the items of a [`IndexMap`]. |
| [`IterMut`](#itermut) | struct | A mutable iterator over the items of a [`IndexMap`]. |
| [`Keys`](#keys) | struct | An iterator over the keys of a [`IndexMap`]. |
| [`Values`](#values) | struct | An iterator over the values of a [`IndexMap`]. |
| [`ValuesMut`](#valuesmut) | struct | A mutable iterator over the values of a [`IndexMap`]. |
| [`Insert`](#insert) | enum |  |
| [`Entry`](#entry) | enum | A view into an entry in the map |
| [`hash_with`](#hash-with) | fn |  |
| [`FnvIndexMap`](#fnvindexmap) | type | An [`IndexMap`] using the default FNV hasher. |
| [`probe_loop!`](#probe-loop) | macro |  |

## Structs

### `HashValue`

```rust
struct HashValue(u16);
```

#### Implementations

- <span id="hashvalue-desired-pos"></span>`fn desired_pos(&self, mask: usize) -> usize`

- <span id="hashvalue-probe-distance"></span>`fn probe_distance(&self, mask: usize, current: usize) -> usize`

#### Trait Implementations

##### `impl Clone for HashValue`

- <span id="hashvalue-clone"></span>`fn clone(&self) -> HashValue` — [`HashValue`](#hashvalue)

##### `impl Copy for HashValue`

##### `impl Eq for HashValue`

##### `impl PartialEq for HashValue`

- <span id="hashvalue-partialeq-eq"></span>`fn eq(&self, other: &HashValue) -> bool` — [`HashValue`](#hashvalue)

##### `impl StructuralPartialEq for HashValue`

### `Inserted<V>`

```rust
struct Inserted<V> {
    index: usize,
    old_value: Option<V>,
}
```

### `CoreMap<K, V, const N: usize>`

```rust
struct CoreMap<K, V, const N: usize> {
    entries: crate::Vec<Bucket<K, V>, N, usize>,
    indices: [Option<Pos>; N],
}
```

#### Implementations

- <span id="coremap-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl<K, V> Clone for CoreMap<K, V, N>`

- <span id="coremap-clone"></span>`fn clone(&self) -> Self`

### `OccupiedEntry<'a, K, V, const N: usize>`

```rust
struct OccupiedEntry<'a, K, V, const N: usize> {
    key: K,
    probe: usize,
    pos: usize,
    core: &'a mut CoreMap<K, V, N>,
}
```

An occupied entry which can be manipulated

#### Implementations

- <span id="occupiedentry-key"></span>`fn key(&self) -> &K`

  Gets a reference to the key that this entity corresponds to

- <span id="occupiedentry-remove-entry"></span>`fn remove_entry(self) -> (K, V)`

  Removes this entry from the map and yields its corresponding key and value

- <span id="occupiedentry-get"></span>`fn get(&self) -> &V`

  Gets a reference to the value associated with this entry

- <span id="occupiedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut V`

  Gets a mutable reference to the value associated with this entry

- <span id="occupiedentry-into-mut"></span>`fn into_mut(self) -> &'a mut V`

  Consumes this entry and yields a reference to the underlying value

- <span id="occupiedentry-insert"></span>`fn insert(self, value: V) -> V`

  Overwrites the underlying map's value with this entry's value

- <span id="occupiedentry-remove"></span>`fn remove(self) -> V`

  Removes this entry from the map and yields its value

### `VacantEntry<'a, K, V, const N: usize>`

```rust
struct VacantEntry<'a, K, V, const N: usize> {
    key: K,
    hash_val: HashValue,
    core: &'a mut CoreMap<K, V, N>,
}
```

A view into an empty slot in the underlying map

#### Implementations

- <span id="vacantentry-key"></span>`fn key(&self) -> &K`

  Get the key associated with this entry

- <span id="vacantentry-into-key"></span>`fn into_key(self) -> K`

  Consumes this entry to yield to key associated with it

- <span id="vacantentry-insert"></span>`fn insert(self, value: V) -> Result<&'a mut V, V>`

  Inserts this entry into to underlying map, yields a mutable reference to the inserted value.

  If the map is at capacity the value is returned instead.

### `IndexMap<K, V, S, const N: usize>`

```rust
struct IndexMap<K, V, S, const N: usize> {
    core: CoreMap<K, V, N>,
    build_hasher: S,
}
```

Fixed capacity [`IndexMap`](https://docs.rs/indexmap/2/indexmap/map/struct.IndexMap.html)

Note that you cannot use `IndexMap` directly, since it is generic around the hashing algorithm
in use. Pick a concrete instantiation like [`FnvIndexMap`](#fnvindexmap) instead
or create your own.

Note that the capacity of the `IndexMap` must be a power of 2.

# Examples

Since `IndexMap` cannot be used directly, we're using its `FnvIndexMap` instantiation
for this example.

```rust
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

#### Implementations

- <span id="indexmap-new"></span>`const fn new() -> Self`

  Creates an empty `IndexMap`.

#### Trait Implementations

##### `impl<K, V, S> Clone for IndexMap<K, V, S, N>`

- <span id="indexmap-clone"></span>`fn clone(&self) -> Self`

##### `impl<K, V, S> Debug for IndexMap<K, V, S, N>`

- <span id="indexmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S> Default for IndexMap<K, V, S, N>`

- <span id="indexmap-default"></span>`fn default() -> Self`

##### `impl<K, V, S> Deserialize for crate::IndexMap<K, V, hash32::BuildHasherDefault<S>, N>`

- <span id="crateindexmap-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for IndexMap<K, V, S, N>`

##### `impl<K, V, S> Eq for IndexMap<K, V, S, N>`

##### `impl<K, V, S> Extend for IndexMap<K, V, S, N>`

- <span id="indexmap-extend"></span>`fn extend<I>(&mut self, iterable: I)`

##### `impl<K, V, S> FromIterator for IndexMap<K, V, S, N>`

- <span id="indexmap-fromiterator-from-iter"></span>`fn from_iter<I>(iterable: I) -> Self`

##### `impl<K, Q, V, S> Index for IndexMap<K, V, S, N>`

- <span id="indexmap-index-type-output"></span>`type Output = V`

- <span id="indexmap-index"></span>`fn index(&self, key: &Q) -> &V`

##### `impl<K, Q, V, S> IndexMut for IndexMap<K, V, S, N>`

- <span id="indexmap-indexmut-index-mut"></span>`fn index_mut(&mut self, key: &Q) -> &mut V`

##### `impl<K, V, S> IntoIterator for IndexMap<K, V, S, N>`

- <span id="indexmap-intoiterator-type-item"></span>`type Item = (K, V)`

- <span id="indexmap-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<K, V, N>`

- <span id="indexmap-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<K, V1, V2, S1, S2> PartialEq for IndexMap<K, V1, S1, N1>`

- <span id="indexmap-partialeq-eq"></span>`fn eq(&self, other: &IndexMap<K, V2, S2, N2>) -> bool` — [`IndexMap`](#indexmap)

##### `impl<K, V, S> Serialize for crate::IndexMap<K, V, S, N>`

- <span id="crateindexmap-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `IntoIter<K, V, const N: usize>`

```rust
struct IntoIter<K, V, const N: usize> {
    entries: crate::Vec<Bucket<K, V>, N, usize>,
}
```

An owning iterator over the entries of an `IndexMap`.

This `struct` is created by the `into_iter` method on [`IndexMap`](#indexmap)
(provided by the `IntoIterator` trait). See its documentation for more.

# Example

```rust
use heapless::index_map::FnvIndexMap;

let mut map = FnvIndexMap::<_, _, 16>::new();
map.insert("a", 1).unwrap();

let iter = map.into_iter();
```

#### Trait Implementations

##### `impl<K: clone::Clone, V: clone::Clone> Clone for IntoIter<K, V, N>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<K, V, N>` — [`IntoIter`](#intoiter)

##### `impl IntoIterator for IntoIter<K, V, N>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IntoIter<K, V, N>`

- <span id="intoiter-iterator-type-item"></span>`type Item = (K, V)`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
```

An iterator over the items of a [`IndexMap`](#indexmap).

This `struct` is created by the [`iter`](IndexMap::iter) method on [`IndexMap`](#indexmap). See its
documentation for more.

#### Trait Implementations

##### `impl<K, V> Clone for Iter<'_, K, V>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl IntoIterator for Iter<'a, K, V>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Iter<'a, K, V>`

- <span id="iter-iterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    iter: slice::IterMut<'a, Bucket<K, V>>,
}
```

A mutable iterator over the items of a [`IndexMap`](#indexmap).

This `struct` is created by the [`iter_mut`](IndexMap::iter_mut) method on [`IndexMap`](#indexmap). See its
documentation for more.

#### Trait Implementations

##### `impl IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IterMut<'a, K, V>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Keys<'a, K, V>`

```rust
struct Keys<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
```

An iterator over the keys of a [`IndexMap`](#indexmap).

This `struct` is created by the [`keys`](IndexMap::keys) method on [`IndexMap`](#indexmap). See its
documentation for more.

#### Trait Implementations

##### `impl IntoIterator for Keys<'a, K, V>`

- <span id="keys-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="keys-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="keys-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Keys<'a, K, V>`

- <span id="keys-iterator-type-item"></span>`type Item = &'a K`

- <span id="keys-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Values<'a, K, V>`

```rust
struct Values<'a, K, V> {
    iter: slice::Iter<'a, Bucket<K, V>>,
}
```

An iterator over the values of a [`IndexMap`](#indexmap).

This `struct` is created by the [`values`](IndexMap::values) method on [`IndexMap`](#indexmap). See its
documentation for more.

#### Trait Implementations

##### `impl IntoIterator for Values<'a, K, V>`

- <span id="values-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="values-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Values<'a, K, V>`

- <span id="values-iterator-type-item"></span>`type Item = &'a V`

- <span id="values-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ValuesMut<'a, K, V>`

```rust
struct ValuesMut<'a, K, V> {
    iter: slice::IterMut<'a, Bucket<K, V>>,
}
```

A mutable iterator over the values of a [`IndexMap`](#indexmap).

This `struct` is created by the [`values_mut`](IndexMap::values_mut) method on [`IndexMap`](#indexmap). See its
documentation for more.

#### Trait Implementations

##### `impl IntoIterator for ValuesMut<'a, K, V>`

- <span id="valuesmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for ValuesMut<'a, K, V>`

- <span id="valuesmut-iterator-type-item"></span>`type Item = &'a mut V`

- <span id="valuesmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `Insert<K, V>`

```rust
enum Insert<K, V> {
    Success(Inserted<V>),
    Full((K, V)),
}
```

### `Entry<'a, K, V, const N: usize>`

```rust
enum Entry<'a, K, V, const N: usize> {
    Occupied(OccupiedEntry<'a, K, V, N>),
    Vacant(VacantEntry<'a, K, V, N>),
}
```

A view into an entry in the map

#### Variants

- **`Occupied`**

  The entry corresponding to the key `K` exists in the map

- **`Vacant`**

  The entry corresponding to the key `K` does not exist in the map

#### Implementations

- <span id="entry-or-insert"></span>`fn or_insert(self, default: V) -> Result<&'a mut V, V>`

  Ensures a value is in the entry by inserting the default if empty, and

  returns a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use heapless::index_map::FnvIndexMap;

  

  // A hash map with a capacity of 16 key-value pairs allocated on the stack

  let mut book_reviews = FnvIndexMap::<_, _, 16>::new();

  let result = book_reviews

      .entry("Adventures of Huckleberry Finn")

      .or_insert("My favorite book.");

  

  assert_eq!(result, Ok(&mut "My favorite book."));

  assert_eq!(

      book_reviews["Adventures of Huckleberry Finn"],

      "My favorite book."

  );

  ```

- <span id="entry-or-insert-with"></span>`fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> Result<&'a mut V, V>`

  Ensures a value is in the entry by inserting the result of the default

  function if empty, and returns a mutable reference to the value in the

  entry.

  

  # Examples

  

  ```rust

  use heapless::index_map::FnvIndexMap;

  

  // A hash map with a capacity of 16 key-value pairs allocated on the stack

  let mut book_reviews = FnvIndexMap::<_, _, 16>::new();

  let s = "Masterpiece.".to_string();

  

  book_reviews

      .entry("Grimms' Fairy Tales")

      .or_insert_with(|| s);

  

  assert_eq!(

      book_reviews["Grimms' Fairy Tales"],

      "Masterpiece.".to_string()

  );

  ```

- <span id="entry-or-insert-with-key"></span>`fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> Result<&'a mut V, V>`

  Ensures a value is in the entry by inserting, if empty, the result of

  the default function. This method allows for generating key-derived

  values for insertion by providing the default function a reference to

  the key that was moved during the `.entry(key)` method call.

  

  The reference to the moved key is provided so that cloning or copying

  the key is unnecessary, unlike with `.or_insert_with(|| ... )`.

  

  # Examples

  

  ```rust

  use heapless::index_map::FnvIndexMap;

  

  // A hash map with a capacity of 16 key-value pairs allocated on the stack

  let mut book_reviews = FnvIndexMap::<_, _, 16>::new();

  

  book_reviews

      .entry("Pride and Prejudice")

      .or_insert_with_key(|key| key.chars().count());

  

  assert_eq!(book_reviews["Pride and Prejudice"], 19);

  ```

- <span id="entry-key"></span>`fn key(&self) -> &K`

  Returns a reference to this entry's key.

  

  # Examples

  

  ```rust

  use heapless::index_map::FnvIndexMap;

  

  // A hash map with a capacity of 16 key-value pairs allocated on the stack

  let mut book_reviews = FnvIndexMap::<&str, &str, 16>::new();

  assert_eq!(

      book_reviews

          .entry("The Adventures of Sherlock Holmes")

          .key(),

      &"The Adventures of Sherlock Holmes"

  );

  ```

- <span id="entry-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

  Provides in-place mutable access to an occupied entry before any

  potential inserts into the map.

  

  # Examples

  

  ```rust

  use heapless::index_map::FnvIndexMap;

  

  // A hash map with a capacity of 16 key-value pairs allocated on the stack

  let mut book_reviews = FnvIndexMap::<_, _, 16>::new();

  

  book_reviews

      .entry("Grimms' Fairy Tales")

      .and_modify(|e| *e = "Masterpiece.")

      .or_insert("Very enjoyable.");

  assert_eq!(book_reviews["Grimms' Fairy Tales"], "Very enjoyable.");

  ```

## Functions

### `hash_with`

```rust
fn hash_with<K, S>(key: &K, build_hasher: &S) -> HashValue
where
    K: ?Sized + Hash,
    S: BuildHasher
```

## Type Aliases

### `FnvIndexMap<K, V, const N: usize>`

```rust
type FnvIndexMap<K, V, const N: usize> = IndexMap<K, V, hash32::BuildHasherDefault<hash32::FnvHasher>, N>;
```

An [`IndexMap`](#indexmap) using the default FNV hasher.

A list of all Methods and Traits available for `FnvIndexMap` can be found in
the [`IndexMap`](#indexmap) documentation.

# Examples
```rust
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

## Macros

### `probe_loop!`

