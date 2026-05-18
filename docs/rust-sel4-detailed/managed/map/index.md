*[managed](../index.md) / [map](index.md)*

---

# Module `map`

## Contents

- [Structs](#structs)
  - [`Range`](#range)
- [Enums](#enums)
  - [`ManagedMap`](#managedmap)
  - [`RevOption`](#revoption)
  - [`RangeInner`](#rangeinner)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
- [Functions](#functions)
  - [`binary_search_by_key_range`](#binary-search-by-key-range)
  - [`binary_search_by_key`](#binary-search-by-key)
  - [`pair_by_key`](#pair-by-key)
  - [`pair_mut_by_key`](#pair-mut-by-key)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Range`](#range) | struct |  |
| [`ManagedMap`](#managedmap) | enum | A managed map. |
| [`RevOption`](#revoption) | enum | Like `Option`, but with `Some` values sorting first. |
| [`RangeInner`](#rangeinner) | enum |  |
| [`Iter`](#iter) | enum |  |
| [`IterMut`](#itermut) | enum |  |
| [`binary_search_by_key_range`](#binary-search-by-key-range) | fn |  |
| [`binary_search_by_key`](#binary-search-by-key) | fn |  |
| [`pair_by_key`](#pair-by-key) | fn |  |
| [`pair_mut_by_key`](#pair-mut-by-key) | fn |  |

## Structs

### `Range<'a, K: 'a, V: 'a>`

```rust
struct Range<'a, K: 'a, V: 'a>(RangeInner<'a, K, V>);
```

#### Trait Implementations

##### `impl<K: clone::Clone + 'a, V: clone::Clone + 'a> Clone for Range<'a, K, V>`

- <span id="range-clone"></span>`fn clone(&self) -> Range<'a, K, V>` — [`Range`](#range)

##### `impl<K: fmt::Debug + 'a, V: fmt::Debug + 'a> Debug for Range<'a, K, V>`

- <span id="range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K: 'a, V: 'a> DoubleEndedIterator for Range<'a, K, V>`

- <span id="range-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl IntoIterator for Range<'a, K, V>`

- <span id="range-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="range-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="range-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K: 'a, V: 'a> Iterator for Range<'a, K, V>`

- <span id="range-iterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="range-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `ManagedMap<'a, K: 'a, V: 'a>`

```rust
enum ManagedMap<'a, K: 'a, V: 'a> {
    Borrowed(&'a mut [Option<(K, V)>]),
    Owned(alloc::collections::btree_map::BTreeMap<K, V>),
}
```

A managed map.

This enum can be used to represent exclusive access to maps.
In Rust, exclusive access to an object is obtained by either owning the object,
or owning a mutable pointer to the object; hence, "managed".

The purpose of this enum is providing good ergonomics with `std` present while making
it possible to avoid having a heap at all (which of course means that `std` is not present).
To achieve this, the variants other than `Borrow` are only available when the corresponding
feature is opted in.

Unlike [Managed](#managed) and [ManagedSlice](#managedslice),
the managed map is implemented using a B-tree map when allocation is available,
and a sorted slice of key-value pairs when it is not. Thus, algorithmic complexity
of operations on it depends on the kind of map.

A function that requires a managed object should be generic over an `Into<ManagedMap<'a, T>>`
argument; then, it will be possible to pass either a `Vec<T>`, or a `&'a mut [T]`
without any conversion at the call site.

See also [Managed](#managed).

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Implementations

- <span id="managedmap-clear"></span>`fn clear(&mut self)`

- <span id="managedmap-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&V>`

- <span id="managedmap-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>`

- <span id="managedmap-range"></span>`fn range<'b, 'c, Q, R>(self: &'b Self, range: R) -> Range<'a, K, V>` — [`Range`](#range)

- <span id="managedmap-insert"></span>`fn insert(&mut self, key: K, new_value: V) -> Result<Option<V>, (K, V)>`

- <span id="managedmap-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<V>`

- <span id="managedmap-is-empty"></span>`fn is_empty(&self) -> bool`

  ManagedMap contains no elements?

- <span id="managedmap-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in the ManagedMap.

- <span id="managedmap-iter"></span>`fn iter(&self) -> Iter<'_, K, V>` — [`Iter`](#iter)

- <span id="managedmap-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, K, V>` — [`IterMut`](#itermut)

#### Trait Implementations

##### `impl<K, V> Debug for ManagedMap<'a, K, V>`

- <span id="managedmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RevOption<T>`

```rust
enum RevOption<T> {
    Some(T),
    None,
}
```

Like `Option`, but with `Some` values sorting first.

#### Trait Implementations

##### `impl<T: cmp::Eq> Eq for RevOption<T>`

##### `impl<T: cmp::Ord> Ord for RevOption<T>`

- <span id="revoption-ord-cmp"></span>`fn cmp(&self, other: &RevOption<T>) -> cmp::Ordering` — [`RevOption`](#revoption)

##### `impl<T: cmp::PartialEq> PartialEq for RevOption<T>`

- <span id="revoption-partialeq-eq"></span>`fn eq(&self, other: &RevOption<T>) -> bool` — [`RevOption`](#revoption)

##### `impl<T: cmp::PartialOrd> PartialOrd for RevOption<T>`

- <span id="revoption-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &RevOption<T>) -> option::Option<cmp::Ordering>` — [`RevOption`](#revoption)

##### `impl<T> StructuralPartialEq for RevOption<T>`

### `RangeInner<'a, K: 'a, V: 'a>`

```rust
enum RangeInner<'a, K: 'a, V: 'a> {
    Borrowed {
        slice: &'a [Option<(K, V)>],
        begin: usize,
        end: usize,
    },
    Owned(alloc::collections::btree_map::Range<'a, K, V>),
}
```

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl<K: clone::Clone + 'a, V: clone::Clone + 'a> Clone for RangeInner<'a, K, V>`

- <span id="rangeinner-clone"></span>`fn clone(&self) -> RangeInner<'a, K, V>` — [`RangeInner`](#rangeinner)

##### `impl<K: fmt::Debug + 'a, V: fmt::Debug + 'a> Debug for RangeInner<'a, K, V>`

- <span id="rangeinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Iter<'a, K: 'a, V: 'a>`

```rust
enum Iter<'a, K: 'a, V: 'a> {
    Borrowed(slice::Iter<'a, Option<(K, V)>>),
    Owned(alloc::collections::btree_map::Iter<'a, K, V>),
}
```

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl IntoIterator for Iter<'a, K, V>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K: Ord + 'a, V: 'a> Iterator for Iter<'a, K, V>`

- <span id="iter-iterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterMut<'a, K: 'a, V: 'a>`

```rust
enum IterMut<'a, K: 'a, V: 'a> {
    Borrowed(slice::IterMut<'a, Option<(K, V)>>),
    Owned(alloc::collections::btree_map::IterMut<'a, K, V>),
}
```

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K: Ord + 'a, V: 'a> Iterator for IterMut<'a, K, V>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Functions

### `binary_search_by_key_range`

```rust
fn binary_search_by_key_range<'a, K, V, Q, R>(slice: &[Option<(K, V)>], range: R) -> Result<(usize, usize), ()>
where
    K: Ord + Borrow<Q>,
    Q: Ord + ?Sized + 'a,
    R: RangeBounds<Q>
```

### `binary_search_by_key`

```rust
fn binary_search_by_key<K, V, Q>(slice: &[Option<(K, V)>], key: &Q) -> Result<usize, usize>
where
    K: Ord + Borrow<Q>,
    Q: Ord + ?Sized
```

### `pair_by_key`

```rust
fn pair_by_key<'a, K, Q, V>(slice: &'a [Option<(K, V)>], key: &Q) -> Result<&'a (K, V), usize>
where
    K: Ord + Borrow<Q>,
    Q: Ord + ?Sized
```

### `pair_mut_by_key`

```rust
fn pair_mut_by_key<'a, K, Q, V>(slice: &'a mut [Option<(K, V)>], key: &Q) -> Result<&'a mut (K, V), usize>
where
    K: Ord + Borrow<Q>,
    Q: Ord + ?Sized
```

