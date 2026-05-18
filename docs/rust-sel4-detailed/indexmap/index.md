# Crate `indexmap`

[`IndexMap`](map/index.md) is a hash table where the iteration order of the key-value
pairs is independent of the hash values of the keys.

[`IndexSet`](set/index.md) is a corresponding hash set using the same implementation and
with similar properties.

### Highlights

[`IndexMap`](map/index.md) and [`IndexSet`](set/index.md) are drop-in compatible with the std `HashMap`
and `HashSet`, but they also have some features of note:

- The ordering semantics (see their documentation for details)
- Sorting methods and the `.pop()` methods.
- The [`Equivalent`](#equivalent) trait, which offers more flexible equality definitions
  between borrowed and owned versions of keys.
- The `MutableKeys` trait, which gives opt-in mutable
  access to map keys, and `MutableValues` for sets.

### Feature Flags

To reduce the amount of compiled code in the crate by default, certain
features are gated behind [feature flags]. These allow you to opt in to (or
out of) functionality. Below is a list of the features available in this
crate.

* `std`: Enables features which require the Rust standard library. For more
  information see the section on `no_std`.
* `rayon`: Enables parallel iteration and other parallel methods.
* `serde`: Adds implementations for `Serialize` and `Deserialize`
  to [`IndexMap`](map/index.md) and [`IndexSet`](set/index.md). Alternative implementations for
  (de)serializing [`IndexMap`](map/index.md) as an ordered sequence are available in the
  `map::serde_seq` module.
* `arbitrary`: Adds implementations for the `arbitrary::Arbitrary` trait
  to [`IndexMap`](map/index.md) and [`IndexSet`](set/index.md).
* `quickcheck`: Adds implementations for the `quickcheck::Arbitrary` trait
  to [`IndexMap`](map/index.md) and [`IndexSet`](set/index.md).
* `borsh` (**deprecated**): Adds implementations for `BorshSerialize` and
  `BorshDeserialize` to [`IndexMap`](map/index.md) and [`IndexSet`](set/index.md). Due to a cyclic
  dependency that arose between `borsh` and `indexmap`, `borsh v1.5.6`
  added an `indexmap` feature that should be used instead of enabling the
  feature here.

_Note: only the `std` feature is enabled by default._









### Alternate Hashers

[`IndexMap`](map/index.md) and [`IndexSet`](set/index.md) have a default hasher type
`S = RandomState`,
just like the standard `HashMap` and `HashSet`, which is resistant to
HashDoS attacks but not the most performant. Type aliases can make it easier
to use alternate hashers:

```rust
use fnv::FnvBuildHasher;
use indexmap::{IndexMap, IndexSet};

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;
type FnvIndexSet<T> = IndexSet<T, FnvBuildHasher>;

let std: IndexSet<i32> = (0..100).collect();
let fnv: FnvIndexSet<i32> = (0..100).collect();
assert_eq!(std, fnv);
```

### Rust Version

This version of indexmap requires Rust 1.82 or later.

The indexmap 2.x release series will use a carefully considered version
upgrade policy, where in a later 2.x version, we will raise the minimum
required Rust version.

## No Standard Library Targets

This crate supports being built without `std`, requiring `alloc` instead.
This is chosen by disabling the default "std" cargo feature, by adding
`default-features = false` to your dependency specification.

- Creating maps and sets using `new` and
  `with_capacity` is unavailable without `std`.
  Use methods `IndexMap::default`, `with_hasher`,
  `with_capacity_and_hasher` instead.
  A no-std compatible hasher will be needed as well, for example
  from the crate `twox-hash`.
- Macros [`indexmap!`](#indexmap) and [`indexset!`](#indexset) are unavailable without `std`. Use
  the macros [`indexmap_with_default!`](#indexmap-with-default) and [`indexset_with_default!`](#indexset-with-default) instead.

## Contents

- [Modules](#modules)
  - [`arbitrary`](#arbitrary)
  - [`inner`](#inner)
  - [`macros`](#macros)
  - [`util`](#util)
  - [`map`](#map)
  - [`set`](#set)
- [Structs](#structs)
  - [`IndexMap`](#indexmap)
  - [`IndexSet`](#indexset)
  - [`Equivalent`](#equivalent)
  - [`HashValue`](#hashvalue)
  - [`Bucket`](#bucket)
  - [`TryReserveError`](#tryreserveerror)
- [Enums](#enums)
  - [`TryReserveErrorKind`](#tryreserveerrorkind)
  - [`GetDisjointMutError`](#getdisjointmuterror)
- [Macros](#macros)
  - [`indexmap_with_default!`](#indexmap-with-default)
  - [`indexmap!`](#indexmap)
  - [`indexset_with_default!`](#indexset-with-default)
  - [`indexset!`](#indexset)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`arbitrary`](#arbitrary) | mod |  |
| [`inner`](#inner) | mod | This is the core implementation that doesn't depend on the hasher at all. |
| [`macros`](#macros) | mod |  |
| [`util`](#util) | mod |  |
| [`map`](#map) | mod | [`IndexMap`] is a hash table where the iteration order of the key-value pairs is independent of the hash values of the keys. |
| [`set`](#set) | mod | A hash set implemented using [`IndexMap`] |
| [`IndexMap`](#indexmap) | struct |  |
| [`IndexSet`](#indexset) | struct |  |
| [`Equivalent`](#equivalent) | struct |  |
| [`HashValue`](#hashvalue) | struct | Hash value newtype. |
| [`Bucket`](#bucket) | struct |  |
| [`TryReserveError`](#tryreserveerror) | struct | The error type for [`try_reserve`][IndexMap::try_reserve] methods. |
| [`TryReserveErrorKind`](#tryreserveerrorkind) | enum |  |
| [`GetDisjointMutError`](#getdisjointmuterror) | enum | The error type returned by [`get_disjoint_indices_mut`][`IndexMap::get_disjoint_indices_mut`]. |
| [`indexmap_with_default!`](#indexmap-with-default) | macro | Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher. |
| [`indexmap!`](#indexmap) | macro | Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs |
| [`indexset_with_default!`](#indexset-with-default) | macro | Create an [`IndexSet`][crate::IndexSet] from a list of values and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher. |
| [`indexset!`](#indexset) | macro | Create an [`IndexSet`][crate::IndexSet] from a list of values |

## Modules

- [`arbitrary`](arbitrary/index.md)
- [`inner`](inner/index.md) — This is the core implementation that doesn't depend on the hasher at all.
- [`macros`](macros/index.md)
- [`util`](util/index.md)
- [`map`](map/index.md) — [`IndexMap`] is a hash table where the iteration order of the key-value
- [`set`](set/index.md) — A hash set implemented using [`IndexMap`]

## Structs

### `IndexMap<K, V, S>`

```rust
struct IndexMap<K, V, S> {
    core: crate::inner::Core<K, V>,
    hash_builder: S,
}
```

A hash table where the iteration order of the key-value pairs is independent
of the hash values of the keys.

The interface is closely compatible with the standard
`HashMap`,
but also has additional features.

# Order

The key-value pairs have a consistent order that is determined by
the sequence of insertion and removal calls on the map. The order does
not depend on the keys or the hash function at all.

All iterators traverse the map in *the order*.

The insertion order is preserved, with **notable exceptions** like the
`.remove()` or `.swap_remove()` methods.
Methods such as `.sort_by()` of
course result in a new order, depending on the sorting order.

# Indices

The key-value pairs are indexed in a compact range without holes in the
range `0..self.len()`. For example, the method `.get_full` looks up the
index for a key, and the method `.get_index` looks up the key-value pair by
index.

# Examples

```rust
use indexmap::IndexMap;

// count the frequency of each letter in a sentence.
let mut letters = IndexMap::new();
for ch in "a short treatise on fungi".chars() {
    *letters.entry(ch).or_insert(0) += 1;
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

#### Implementations

- <span id="indexmap-new"></span>`fn new() -> Self`

  Create a new map. (Does not allocate.)

- <span id="indexmap-with-capacity"></span>`fn with_capacity(n: usize) -> Self`

  Create a new map with capacity for `n` key-value pairs. (Does not

  allocate if `n` is zero.)

  

  Computes in **O(n)** time.

#### Trait Implementations

##### `impl<K, V, S> Clone for IndexMap<K, V, S>`

- <span id="indexmap-clone"></span>`fn clone(&self) -> Self`

- <span id="indexmap-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<K, V, S> Debug for IndexMap<K, V, S>`

- <span id="indexmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S> Default for IndexMap<K, V, S>`

- <span id="indexmap-default"></span>`fn default() -> Self`

  Return an empty [`IndexMap`](map/index.md)

##### `impl<K, V, S> Eq for IndexMap<K, V, S>`

##### `impl<K> Equivalent for IndexMap<K, V, S>`

- <span id="indexmap-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<K, V, S> Extend for IndexMap<K, V, S>`

- <span id="indexmap-extend"></span>`fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iterable: I)`

  Extend the map with all key-value pairs in the iterable.

  

  This is equivalent to calling `insert` for each of

  them in order, which means that for keys that already existed

  in the map, their value is updated but it keeps the existing order.

  

  New keys are inserted in the order they appear in the sequence. If

  equivalents of a key occur more than once, the last corresponding value

  prevails.

##### `impl<K, V, S> FromIterator for IndexMap<K, V, S>`

- <span id="indexmap-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self`

  Create an `IndexMap` from the sequence of key-value pairs in the

  iterable.

  

  `from_iter` uses the same logic as `extend`. See

  `extend` for more details.

##### `impl<K, V, S> Index for IndexMap<K, V, S>`

- <span id="indexmap-index-type-output"></span>`type Output = Slice<K, V>`

- <span id="indexmap-index"></span>`fn index(&self, range: ops::Range<usize>) -> &<Self as >::Output`

##### `impl<K, V, S> IndexMut for IndexMap<K, V, S>`

- <span id="indexmap-indexmut-index-mut"></span>`fn index_mut(&mut self, range: ops::Range<usize>) -> &mut <Self as >::Output`

##### `impl<K, V, S> IntoIterator for &'a super::IndexMap<K, V, S>`

- <span id="a-superindexmap-intoiterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="a-superindexmap-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, K, V>`

- <span id="a-superindexmap-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<K, V, S> MutableKeys for super::IndexMap<K, V, S>`

- <span id="superindexmap-mutablekeys-type-key"></span>`type Key = K`

- <span id="superindexmap-mutablekeys-type-value"></span>`type Value = V`

- <span id="superindexmap-mutablekeys-get-full-mut2"></span>`fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut K, &mut V)>`

- <span id="superindexmap-mutablekeys-get-index-mut2"></span>`fn get_index_mut2(&mut self, index: usize) -> Option<(&mut K, &mut V)>`

- <span id="superindexmap-mutablekeys-iter-mut2"></span>`fn iter_mut2(&mut self) -> IterMut2<'_, <Self as >::Key, <Self as >::Value>` — [`IterMut2`](map/iter/index.md#itermut2), [`MutableKeys`](map/mutable/index.md#mutablekeys)

- <span id="superindexmap-mutablekeys-retain2"></span>`fn retain2<F>(&mut self, keep: F)`

##### `impl<K, V1, S1, V2, S2> PartialEq for IndexMap<K, V1, S1>`

- <span id="indexmap-partialeq-eq"></span>`fn eq(&self, other: &IndexMap<K, V2, S2>) -> bool` — [`IndexMap`](map/index.md#indexmap)

##### `impl<K, V, S> RawEntryApiV1 for crate::IndexMap<K, V, S>`

- <span id="crateindexmap-rawentryapiv1-raw-entry-v1"></span>`fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S>` — [`RawEntryBuilder`](map/raw_entry_v1/index.md#rawentrybuilder)

- <span id="crateindexmap-rawentryapiv1-raw-entry-mut-v1"></span>`fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S>` — [`RawEntryBuilderMut`](map/raw_entry_v1/index.md#rawentrybuildermut)

##### `impl<K, V, S> Sealed for super::IndexMap<K, V, S>`

### `IndexSet<T, S>`

```rust
struct IndexSet<T, S> {
    map: super::IndexMap<T, (), S>,
}
```

A hash set where the iteration order of the values is independent of their
hash values.

The interface is closely compatible with the standard
`HashSet`,
but also has additional features.

# Order

The values have a consistent order that is determined by the sequence of
insertion and removal calls on the set. The order does not depend on the
values or the hash function at all. Note that insertion order and value
are not affected if a re-insertion is attempted once an element is
already present.

All iterators traverse the set *in order*.  Set operation iterators like
`IndexSet::union` produce a concatenated order, as do their matching "bitwise"
operators.  See their documentation for specifics.

The insertion order is preserved, with **notable exceptions** like the
`.remove()` or `.swap_remove()` methods.
Methods such as `.sort_by()` of
course result in a new order, depending on the sorting order.

# Indices

The values are indexed in a compact range without holes in the range
`0..self.len()`. For example, the method `.get_full` looks up the index for
a value, and the method `.get_index` looks up the value by index.

# Complexity

Internally, `IndexSet<T, S>` just holds an [`IndexMap<T, (), S>`](IndexMap). Thus the complexity
of the two are the same for most methods.

# Examples

```rust
use indexmap::IndexSet;

// Collects which letters appear in a sentence.
let letters: IndexSet<_> = "a short treatise on fungi".chars().collect();

assert!(letters.contains(&'s'));
assert!(letters.contains(&'t'));
assert!(letters.contains(&'u'));
assert!(!letters.contains(&'y'));
```

#### Implementations

- <span id="indexset-new"></span>`fn new() -> Self`

  Create a new set. (Does not allocate.)

- <span id="indexset-with-capacity"></span>`fn with_capacity(n: usize) -> Self`

  Create a new set with capacity for `n` elements.

  (Does not allocate if `n` is zero.)

  

  Computes in **O(n)** time.

#### Trait Implementations

##### `impl<T, S1, S2> BitAnd for &IndexSet<T, S1>`

- <span id="indexset-bitand-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-bitand"></span>`fn bitand(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](set/index.md#indexset)

  Returns the set intersection, cloned into a new set.

  

  Values are collected in the same order that they appear in `self`.

##### `impl<T, S1, S2> BitOr for &IndexSet<T, S1>`

- <span id="indexset-bitor-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-bitor"></span>`fn bitor(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](set/index.md#indexset)

  Returns the set union, cloned into a new set.

  

  Values from `self` are collected in their original order, followed by

  values that are unique to `other` in their original order.

##### `impl<T, S1, S2> BitXor for &IndexSet<T, S1>`

- <span id="indexset-bitxor-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-bitxor"></span>`fn bitxor(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](set/index.md#indexset)

  Returns the set symmetric-difference, cloned into a new set.

  

  Values from `self` are collected in their original order, followed by

  values from `other` in their original order.

##### `impl<T, S> Clone for IndexSet<T, S>`

- <span id="indexset-clone"></span>`fn clone(&self) -> Self`

- <span id="indexset-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<T, S> Debug for IndexSet<T, S>`

- <span id="indexset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for IndexSet<T, S>`

- <span id="indexset-default"></span>`fn default() -> Self`

  Return an empty [`IndexSet`](set/index.md)

##### `impl<T, S> Eq for IndexSet<T, S>`

##### `impl<K> Equivalent for IndexSet<T, S>`

- <span id="indexset-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T, S> Extend for IndexSet<T, S>`

- <span id="indexset-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I)`

##### `impl<T, S> FromIterator for IndexSet<T, S>`

- <span id="indexset-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self`

##### `impl<T, S> Index for IndexSet<T, S>`

- <span id="indexset-index-type-output"></span>`type Output = Slice<T>`

- <span id="indexset-index"></span>`fn index(&self, range: ops::Range<usize>) -> &<Self as >::Output`

##### `impl<T, S> IntoIterator for &'a super::IndexSet<T, S>`

- <span id="a-superindexset-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-superindexset-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-superindexset-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, S> MutableValues for super::IndexSet<T, S>`

- <span id="superindexset-mutablevalues-type-value"></span>`type Value = T`

- <span id="superindexset-mutablevalues-get-full-mut2"></span>`fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut T)>`

- <span id="superindexset-mutablevalues-get-index-mut2"></span>`fn get_index_mut2(&mut self, index: usize) -> Option<&mut T>`

- <span id="superindexset-mutablevalues-retain2"></span>`fn retain2<F>(&mut self, keep: F)`

##### `impl<T, S1, S2> PartialEq for IndexSet<T, S1>`

- <span id="indexset-partialeq-eq"></span>`fn eq(&self, other: &IndexSet<T, S2>) -> bool` — [`IndexSet`](set/index.md#indexset)

##### `impl<T, S> Sealed for super::IndexSet<T, S>`

##### `impl<T, S1, S2> Sub for &IndexSet<T, S1>`

- <span id="indexset-sub-type-output"></span>`type Output = IndexSet<T, S1>`

- <span id="indexset-sub"></span>`fn sub(self, other: &IndexSet<T, S2>) -> <Self as >::Output` — [`IndexSet`](set/index.md#indexset)

  Returns the set difference, cloned into a new set.

  

  Values are collected in the same order that they appear in `self`.

### `Equivalent<R: gimli::Reader>`

```rust
struct Equivalent<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    lazy: core::cell::OnceCell<Result<Function<R>, gimli::Error>>,
}
```

*Re-exported from `addr2line`*

#### Implementations

- <span id="lazyfunction-new"></span>`fn new(dw_die_offset: gimli::UnitOffset<<R as >::Offset>) -> Self`

- <span id="lazyfunction-borrow"></span>`fn borrow(&self, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<&Function<R>, gimli::Error>`

### `HashValue`

```rust
struct HashValue(usize);
```

Hash value newtype. Not larger than usize, since anything larger
isn't used for selecting position anyway.

#### Implementations

- <span id="hashvalue-get"></span>`fn get(self) -> u64`

#### Trait Implementations

##### `impl Clone for HashValue`

- <span id="hashvalue-clone"></span>`fn clone(&self) -> HashValue` — [`HashValue`](#hashvalue)

##### `impl Copy for HashValue`

##### `impl Debug for HashValue`

- <span id="hashvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for HashValue`

- <span id="hashvalue-partialeq-eq"></span>`fn eq(&self, other: &HashValue) -> bool` — [`HashValue`](#hashvalue)

##### `impl StructuralPartialEq for HashValue`

### `Bucket<K, V>`

```rust
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
```

#### Implementations

- <span id="bucket-key-ref"></span>`fn key_ref(&self) -> &K`

- <span id="bucket-value-ref"></span>`fn value_ref(&self) -> &V`

- <span id="bucket-value-mut"></span>`fn value_mut(&mut self) -> &mut V`

- <span id="bucket-key"></span>`fn key(self) -> K`

- <span id="bucket-value"></span>`fn value(self) -> V`

- <span id="bucket-key-value"></span>`fn key_value(self) -> (K, V)`

- <span id="bucket-refs"></span>`fn refs(&self) -> (&K, &V)`

- <span id="bucket-ref-mut"></span>`fn ref_mut(&mut self) -> (&K, &mut V)`

- <span id="bucket-muts"></span>`fn muts(&mut self) -> (&mut K, &mut V)`

#### Trait Implementations

##### `impl<K, V> Clone for Bucket<K, V>`

- <span id="bucket-clone"></span>`fn clone(&self) -> Self`

- <span id="bucket-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<K: marker::Copy, V: marker::Copy> Copy for Bucket<K, V>`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Bucket<K, V>`

- <span id="bucket-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TryReserveError`

```rust
struct TryReserveError {
    kind: TryReserveErrorKind,
}
```

The error type for `try_reserve` methods.

#### Implementations

- <span id="tryreserveerror-from-alloc"></span>`fn from_alloc(error: alloc::collections::TryReserveError) -> Self`

- <span id="tryreserveerror-from-hashbrown"></span>`fn from_hashbrown(error: hashbrown::TryReserveError) -> Self`

#### Trait Implementations

##### `impl Clone for TryReserveError`

- <span id="tryreserveerror-clone"></span>`fn clone(&self) -> TryReserveError` — [`TryReserveError`](#tryreserveerror)

##### `impl Debug for TryReserveError`

- <span id="tryreserveerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryReserveError`

- <span id="tryreserveerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for TryReserveError`

##### `impl<K> Equivalent for TryReserveError`

- <span id="tryreserveerror-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Error for TryReserveError`

##### `impl PartialEq for TryReserveError`

- <span id="tryreserveerror-partialeq-eq"></span>`fn eq(&self, other: &TryReserveError) -> bool` — [`TryReserveError`](#tryreserveerror)

##### `impl StructuralPartialEq for TryReserveError`

##### `impl ToString for TryReserveError`

- <span id="tryreserveerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `TryReserveErrorKind`

```rust
enum TryReserveErrorKind {
    Std(alloc::collections::TryReserveError),
    CapacityOverflow,
    AllocError {
        layout: alloc::alloc::Layout,
    },
}
```

#### Trait Implementations

##### `impl Clone for TryReserveErrorKind`

- <span id="tryreserveerrorkind-clone"></span>`fn clone(&self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](#tryreserveerrorkind)

##### `impl Debug for TryReserveErrorKind`

- <span id="tryreserveerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryReserveErrorKind`

##### `impl<K> Equivalent for TryReserveErrorKind`

- <span id="tryreserveerrorkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for TryReserveErrorKind`

- <span id="tryreserveerrorkind-partialeq-eq"></span>`fn eq(&self, other: &TryReserveErrorKind) -> bool` — [`TryReserveErrorKind`](#tryreserveerrorkind)

##### `impl StructuralPartialEq for TryReserveErrorKind`

### `GetDisjointMutError`

```rust
enum GetDisjointMutError {
    IndexOutOfBounds,
    OverlappingIndices,
}
```

The error type returned by [`get_disjoint_indices_mut`]`IndexMap::get_disjoint_indices_mut`.

It indicates one of two possible errors:
- An index is out-of-bounds.
- The same index appeared multiple times in the array.

#### Variants

- **`IndexOutOfBounds`**

  An index provided was out-of-bounds for the slice.

- **`OverlappingIndices`**

  Two indices provided were overlapping.

#### Trait Implementations

##### `impl Clone for GetDisjointMutError`

- <span id="getdisjointmuterror-clone"></span>`fn clone(&self) -> GetDisjointMutError` — [`GetDisjointMutError`](#getdisjointmuterror)

##### `impl Debug for GetDisjointMutError`

- <span id="getdisjointmuterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for GetDisjointMutError`

- <span id="getdisjointmuterror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for GetDisjointMutError`

##### `impl<K> Equivalent for GetDisjointMutError`

- <span id="getdisjointmuterror-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Error for GetDisjointMutError`

##### `impl PartialEq for GetDisjointMutError`

- <span id="getdisjointmuterror-partialeq-eq"></span>`fn eq(&self, other: &GetDisjointMutError) -> bool` — [`GetDisjointMutError`](#getdisjointmuterror)

##### `impl StructuralPartialEq for GetDisjointMutError`

##### `impl ToString for GetDisjointMutError`

- <span id="getdisjointmuterror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Macros

### `indexmap_with_default!`

Create an `IndexMap` from a list of key-value pairs
and a `BuildHasherDefault`-wrapped custom hasher.

## Example

```rust
use indexmap::indexmap_with_default;
use fnv::FnvHasher;

let map = indexmap_with_default!{
    FnvHasher;
    "a" => 1,
    "b" => 2,
};
assert_eq!(map["a"], 1);
assert_eq!(map["b"], 2);
assert_eq!(map.get("c"), None);

// "a" is the first key
assert_eq!(map.keys().next(), Some(&"a"));
```

### `indexmap!`

Create an `IndexMap` from a list of key-value pairs

## Example

```rust
use indexmap::indexmap;

let map = indexmap!{
    "a" => 1,
    "b" => 2,
};
assert_eq!(map["a"], 1);
assert_eq!(map["b"], 2);
assert_eq!(map.get("c"), None);

// "a" is the first key
assert_eq!(map.keys().next(), Some(&"a"));
```

### `indexset_with_default!`

Create an `IndexSet` from a list of values
and a `BuildHasherDefault`-wrapped custom hasher.

## Example

```rust
use indexmap::indexset_with_default;
use fnv::FnvHasher;

let set = indexset_with_default!{
    FnvHasher;
    "a",
    "b",
};
assert!(set.contains("a"));
assert!(set.contains("b"));
assert!(!set.contains("c"));

// "a" is the first value
assert_eq!(set.iter().next(), Some(&"a"));
```

### `indexset!`

Create an `IndexSet` from a list of values

## Example

```rust
use indexmap::indexset;

let set = indexset!{
    "a",
    "b",
};
assert!(set.contains("a"));
assert!(set.contains("b"));
assert!(!set.contains("c"));

// "a" is the first value
assert_eq!(set.iter().next(), Some(&"a"));
```

