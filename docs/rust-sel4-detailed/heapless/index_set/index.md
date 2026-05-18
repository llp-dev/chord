*[heapless](../index.md) / [index_set](index.md)*

---

# Module `index_set`

A fixed-capacity hash set where the iteration order is independent of the hash values.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IndexSet`](#indexset) | struct | Fixed capacity [`IndexSet`](https://docs.rs/indexmap/2/indexmap/set/struct.IndexSet.html). |
| [`Iter`](#iter) | struct | An iterator over the items of a [`IndexSet`]. |
| [`Difference`](#difference) | struct | An iterator over the difference of two `IndexSet`s. |
| [`Intersection`](#intersection) | struct | An iterator over the intersection of two `IndexSet`s. |
| [`FnvIndexSet`](#fnvindexset) | type | An [`IndexSet`] using the default FNV hasher. |

## Structs

### `IndexSet<T, S, const N: usize>`

```rust
struct IndexSet<T, S, const N: usize> {
    map: crate::index_map::IndexMap<T, (), S, N>,
}
```

Fixed capacity [`IndexSet`](https://docs.rs/indexmap/2/indexmap/set/struct.IndexSet.html).

Note that you cannot use `IndexSet` directly, since it is generic around the hashing algorithm
in use. Pick a concrete instantiation like [`FnvIndexSet`](#fnvindexset) instead
or create your own.

Note that the capacity of the `IndexSet` must be a power of 2.

# Examples
Since `IndexSet` cannot be used directly, we're using its `FnvIndexSet` instantiation
for this example.

```rust
use heapless::index_set::FnvIndexSet;

// A hash set with a capacity of 16 elements allocated on the stack
let mut books = FnvIndexSet::<_, 16>::new();

// Add some books.
books.insert("A Dance With Dragons").unwrap();
books.insert("To Kill a Mockingbird").unwrap();
books.insert("The Odyssey").unwrap();
books.insert("The Great Gatsby").unwrap();

// Check for a specific one.
if !books.contains("The Winds of Winter") {
    println!(
        "We have {} books, but The Winds of Winter ain't one.",
        books.len()
    );
}

// Remove a book.
books.remove("The Odyssey");

// Iterate over everything.
for book in &books {
    println!("{}", book);
}
```

#### Implementations

- <span id="indexset-new"></span>`const fn new() -> Self`

  Creates an empty `IndexSet`

#### Trait Implementations

##### `impl<T, S> Clone for IndexSet<T, S, N>`

- <span id="indexset-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for IndexSet<T, S, N>`

- <span id="indexset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for IndexSet<T, S, N>`

- <span id="indexset-default"></span>`fn default() -> Self`

##### `impl<T, S> Deserialize for crate::IndexSet<T, hash32::BuildHasherDefault<S>, N>`

- <span id="crateindexset-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl<T> DeserializeOwned for IndexSet<T, S, N>`

##### `impl<T, S> Extend for IndexSet<T, S, N>`

- <span id="indexset-extend"></span>`fn extend<I>(&mut self, iterable: I)`

##### `impl<T, S> FromIterator for IndexSet<T, S, N>`

- <span id="indexset-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Self`

##### `impl<T, S> IntoIterator for &'a IndexSet<T, S, N>`

- <span id="a-indexset-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-indexset-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-indexset-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, S1, S2> PartialEq for IndexSet<T, S1, N1>`

- <span id="indexset-partialeq-eq"></span>`fn eq(&self, other: &IndexSet<T, S2, N2>) -> bool` — [`IndexSet`](#indexset)

##### `impl<T, S> Serialize for crate::IndexSet<T, S, N>`

- <span id="crateindexset-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    iter: index_map::Iter<'a, T, ()>,
}
```

An iterator over the items of a [`IndexSet`](#indexset).

This `struct` is created by the [`iter`](IndexSet::iter) method on [`IndexSet`](#indexset). See its
documentation for more.

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl IntoIterator for Iter<'a, T>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Iter<'a, T>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a T`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Difference<'a, T, S, const N: usize>`

```rust
struct Difference<'a, T, S, const N: usize>
where
    S: BuildHasher,
    T: Eq + Hash {
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S, N>,
}
```

An iterator over the difference of two `IndexSet`s.

This is created by the `IndexSet::difference` method.

#### Trait Implementations

##### `impl IntoIterator for Difference<'a, T, S, N>`

- <span id="difference-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="difference-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="difference-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S> Iterator for Difference<'a, T, S, N>`

- <span id="difference-iterator-type-item"></span>`type Item = &'a T`

- <span id="difference-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Intersection<'a, T, S, const N: usize>`

```rust
struct Intersection<'a, T, S, const N: usize>
where
    S: BuildHasher,
    T: Eq + Hash {
    iter: Iter<'a, T>,
    other: &'a IndexSet<T, S, N>,
}
```

An iterator over the intersection of two `IndexSet`s.

This is created by the `IndexSet::intersection` method.

#### Trait Implementations

##### `impl IntoIterator for Intersection<'a, T, S, N>`

- <span id="intersection-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersection-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intersection-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S> Iterator for Intersection<'a, T, S, N>`

- <span id="intersection-iterator-type-item"></span>`type Item = &'a T`

- <span id="intersection-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `FnvIndexSet<T, const N: usize>`

```rust
type FnvIndexSet<T, const N: usize> = IndexSet<T, hash32::BuildHasherDefault<hash32::FnvHasher>, N>;
```

An [`IndexSet`](#indexset) using the default FNV hasher.

A list of all Methods and Traits available for `FnvIndexSet` can be found in
the [`IndexSet`](#indexset) documentation.

# Examples
```rust
use heapless::index_set::FnvIndexSet;

// A hash set with a capacity of 16 elements allocated on the stack
let mut books = FnvIndexSet::<_, 16>::new();

// Add some books.
books.insert("A Dance With Dragons").unwrap();
books.insert("To Kill a Mockingbird").unwrap();
books.insert("The Odyssey").unwrap();
books.insert("The Great Gatsby").unwrap();

// Check for a specific one.
if !books.contains("The Winds of Winter") {
    println!(
        "We have {} books, but The Winds of Winter ain't one.",
        books.len()
    );
}

// Remove a book.
books.remove("The Odyssey");

// Iterate over everything.
for book in &books {
    println!("{}", book);
}
```

