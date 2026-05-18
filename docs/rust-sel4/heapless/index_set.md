**heapless > index_set**

# Module: index_set

## Contents

**Structs**

- [`Difference`](#difference) - An iterator over the difference of two `IndexSet`s.
- [`IndexSet`](#indexset) - Fixed capacity [`IndexSet`](https://docs.rs/indexmap/2/indexmap/set/struct.IndexSet.html).
- [`Intersection`](#intersection) - An iterator over the intersection of two `IndexSet`s.
- [`Iter`](#iter) - An iterator over the items of a [`IndexSet`].

**Type Aliases**

- [`FnvIndexSet`](#fnvindexset) - An [`IndexSet`] using the default FNV hasher.

---

## heapless::index_set::Difference

*Struct*

An iterator over the difference of two `IndexSet`s.

This is created by the [`IndexSet::difference`] method.

**Generic Parameters:**
- 'a
- T
- S
- const N

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::index_set::FnvIndexSet

*Type Alias*: `IndexSet<T, hash32::BuildHasherDefault<hash32::FnvHasher>, N>`

An [`IndexSet`] using the default FNV hasher.

A list of all Methods and Traits available for `FnvIndexSet` can be found in
the [`IndexSet`] documentation.

# Examples
```
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



## heapless::index_set::IndexSet

*Struct*

Fixed capacity [`IndexSet`](https://docs.rs/indexmap/2/indexmap/set/struct.IndexSet.html).

Note that you cannot use `IndexSet` directly, since it is generic around the hashing algorithm
in use. Pick a concrete instantiation like [`FnvIndexSet`] instead
or create your own.

Note that the capacity of the `IndexSet` must be a power of 2.

# Examples
Since `IndexSet` cannot be used directly, we're using its `FnvIndexSet` instantiation
for this example.

```
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

**Generic Parameters:**
- T
- S
- const N

**Methods:**

- `fn capacity(self: &Self) -> usize` - Returns the number of elements the set can hold
- `fn iter(self: &Self) -> Iter<T>` - Return an iterator over the values of the set, in insertion order
- `fn first(self: &Self) -> Option<&T>` - Get the first value
- `fn last(self: &Self) -> Option<&T>` - Get the last value
- `fn len(self: &Self) -> usize` - Returns the number of elements in the set.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the set contains no elements.
- `fn is_full(self: &Self) -> bool` - Returns `true` if the set is full.
- `fn clear(self: & mut Self)` - Clears the set, removing all values.
- `fn difference<'a, S2, const N2>(self: &'a Self, other: &'a IndexSet<T, S2, N2>) -> Difference<'a, T, S2, N2>` - Visits the values representing the difference, i.e. the values that are in `self` but not in
- `fn symmetric_difference<'a, S2, const N2>(self: &'a Self, other: &'a IndexSet<T, S2, N2>) -> impl Trait` - Visits the values representing the symmetric difference, i.e. the values that are in `self`
- `fn intersection<'a, S2, const N2>(self: &'a Self, other: &'a IndexSet<T, S2, N2>) -> Intersection<'a, T, S2, N2>` - Visits the values representing the intersection, i.e. the values that are both in `self` and
- `fn union<'a, S2, const N2>(self: &'a Self, other: &'a IndexSet<T, S2, N2>) -> impl Trait` - Visits the values representing the union, i.e. all the values in `self` or `other`, without
- `fn contains<Q>(self: &Self, value: &Q) -> bool` - Returns `true` if the set contains a value.
- `fn is_disjoint<S2, const N2>(self: &Self, other: &IndexSet<T, S2, N2>) -> bool` - Returns `true` if `self` has no elements in common with `other`. This is equivalent to
- `fn is_subset<S2, const N2>(self: &Self, other: &IndexSet<T, S2, N2>) -> bool` - Returns `true` if the set is a subset of another, i.e. `other` contains at least all the
- `fn is_superset<S2, const N2>(self: &Self, other: &IndexSet<T, S2, N2>) -> bool`
- `fn insert(self: & mut Self, value: T) -> Result<bool, T>` - Adds a value to the set.
- `fn remove<Q>(self: & mut Self, value: &Q) -> bool` - Removes a value from the set. Returns `true` if the value was present in the set.
- `fn retain<F>(self: & mut Self, f: F)` - Retains only the elements specified by the predicate.
- `fn new() -> Self` - Creates an empty `IndexSet`

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IndexSet<T, S2, N2>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Serialize**
  - `fn serialize<SER>(self: &Self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`
- **Extend**
  - `fn extend<I>(self: & mut Self, iterable: I)`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **Extend**
  - `fn extend<I>(self: & mut Self, iterable: I)`



## heapless::index_set::Intersection

*Struct*

An iterator over the intersection of two `IndexSet`s.

This is created by the [`IndexSet::intersection`] method.

**Generic Parameters:**
- 'a
- T
- S
- const N

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::index_set::Iter

*Struct*

An iterator over the items of a [`IndexSet`].

This `struct` is created by the [`iter`](IndexSet::iter) method on [`IndexSet`]. See its
documentation for more.

**Generic Parameters:**
- 'a
- T

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



