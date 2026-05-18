*[indexmap](../../index.md) / [map](../index.md) / [slice](index.md)*

---

# Module `slice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Slice`](#slice) | struct | A dynamically-sized slice of key-value pairs in an [`IndexMap`]. |
| [`impl_index!`](#impl-index) | macro |  |

## Structs

### `Slice<K, V>`

```rust
struct Slice<K, V> {
    entries: [crate::Bucket<K, V>],
}
```

A dynamically-sized slice of key-value pairs in an [`IndexMap`](../index.md).

This supports indexed operations much like a `[(K, V)]` slice,
but not any hashed operations on the map keys.

Unlike `IndexMap`, `Slice` does consider the order for `PartialEq`
and [`Eq`](../../../base64ct/index.md), and it also implements `PartialOrd`, `Ord`, and [`Hash`](../../../memchr/arch/all/rabinkarp/index.md).

#### Implementations

- <span id="slice-from-slice"></span>`const fn from_slice(entries: &[Bucket<K, V>]) -> &Self` — [`Bucket`](../../index.md#bucket)

- <span id="slice-from-mut-slice"></span>`fn from_mut_slice(entries: &mut [Bucket<K, V>]) -> &mut Self` — [`Bucket`](../../index.md#bucket)

- <span id="slice-from-boxed"></span>`fn from_boxed(entries: Box<[Bucket<K, V>]>) -> Box<Self>` — [`Bucket`](../../index.md#bucket)

- <span id="slice-into-boxed"></span>`fn into_boxed(self: Box<Self>) -> Box<[Bucket<K, V>]>` — [`Bucket`](../../index.md#bucket)

#### Trait Implementations

##### `impl<K: Clone, V: Clone> Clone for alloc::boxed::Box<Slice<K, V>>`

- <span id="allocboxedbox-clone"></span>`fn clone(&self) -> Self`

##### `impl<K> Comparable for Slice<K, V>`

- <span id="slice-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Slice<K, V>`

- <span id="slice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for &Slice<K, V>`

- <span id="slice-default"></span>`fn default() -> Self`

##### `impl<K: Eq, V: Eq> Eq for Slice<K, V>`

##### `impl<K> Equivalent for Slice<K, V>`

- <span id="slice-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<K: Hash, V: Hash> Hash for Slice<K, V>`

- <span id="slice-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<K, V> Index for Slice<K, V>`

- <span id="slice-index-type-output"></span>`type Output = V`

- <span id="slice-index"></span>`fn index(&self, index: usize) -> &V`

##### `impl<K, V> IndexMut for Slice<K, V>`

- <span id="slice-indexmut-index-mut"></span>`fn index_mut(&mut self, index: usize) -> &mut V`

##### `impl<K, V> IntoIterator for &'a Slice<K, V>`

- <span id="a-slice-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, K, V>`

- <span id="a-slice-intoiterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="a-slice-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<K: Ord, V: Ord> Ord for Slice<K, V>`

- <span id="slice-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<K, V, K2, V2> PartialEq for Slice<K, V>`

- <span id="slice-partialeq-eq"></span>`fn eq(&self, other: &Slice<K2, V2>) -> bool` — [`Slice`](#slice)

##### `impl<K: PartialOrd, V: PartialOrd> PartialOrd for Slice<K, V>`

- <span id="slice-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

## Macros

### `impl_index!`

