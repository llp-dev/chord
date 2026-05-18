*[indexmap](../../index.md) / [set](../index.md) / [slice](index.md)*

---

# Module `slice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Slice`](#slice) | struct | A dynamically-sized slice of values in an [`IndexSet`]. |
| [`impl_index!`](#impl-index) | macro |  |

## Structs

### `Slice<T>`

```rust
struct Slice<T> {
    entries: [super::Bucket<T, ()>],
}
```

A dynamically-sized slice of values in an [`IndexSet`](../index.md).

This supports indexed operations much like a `[T]` slice,
but not any hashed operations on the values.

Unlike `IndexSet`, `Slice` does consider the order for `PartialEq`
and [`Eq`](../../../base64ct/index.md), and it also implements `PartialOrd`, `Ord`, and [`Hash`](../../../memchr/arch/all/rabinkarp/index.md).

#### Implementations

- <span id="slice-from-slice"></span>`const fn from_slice(entries: &[super::Bucket<T, ()>]) -> &Self` — [`Bucket`](../../index.md#bucket)

- <span id="slice-from-boxed"></span>`fn from_boxed(entries: Box<[super::Bucket<T, ()>]>) -> Box<Self>` — [`Bucket`](../../index.md#bucket)

- <span id="slice-into-boxed"></span>`fn into_boxed(self: Box<Self>) -> Box<[super::Bucket<T, ()>]>` — [`Bucket`](../../index.md#bucket)

#### Trait Implementations

##### `impl<T: Clone> Clone for alloc::boxed::Box<Slice<T>>`

- <span id="allocboxedbox-clone"></span>`fn clone(&self) -> Self`

##### `impl<K> Comparable for Slice<T>`

- <span id="slice-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<T: fmt::Debug> Debug for Slice<T>`

- <span id="slice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for &Slice<T>`

- <span id="slice-default"></span>`fn default() -> Self`

##### `impl<T: Eq> Eq for Slice<T>`

##### `impl<K> Equivalent for Slice<T>`

- <span id="slice-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T: Hash> Hash for Slice<T>`

- <span id="slice-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T> Index for Slice<T>`

- <span id="slice-index-type-output"></span>`type Output = T`

- <span id="slice-index"></span>`fn index(&self, index: usize) -> &<Self as >::Output`

##### `impl<T> IntoIterator for &'a Slice<T>`

- <span id="a-slice-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-slice-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-slice-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T: Ord> Ord for Slice<T>`

- <span id="slice-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T, U> PartialEq for Slice<T>`

- <span id="slice-partialeq-eq"></span>`fn eq(&self, other: &Slice<U>) -> bool` — [`Slice`](#slice)

##### `impl<T: PartialOrd> PartialOrd for Slice<T>`

- <span id="slice-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

## Macros

### `impl_index!`

