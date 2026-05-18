*[heapless](../index.md) / [linear_map](index.md)*

---

# Module `linear_map`

A fixed capacity map/dictionary that performs lookups via linear search.

Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).

## Contents

- [Modules](#modules)
  - [`storage`](#storage)
- [Structs](#structs)
  - [`LinearMapInner`](#linearmapinner)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
- [Traits](#traits)
  - [`LinearMapStorage`](#linearmapstorage)
- [Type Aliases](#type-aliases)
  - [`OwnedStorage`](#ownedstorage)
  - [`ViewStorage`](#viewstorage)
  - [`LinearMap`](#linearmap)
  - [`LinearMapView`](#linearmapview)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`storage`](#storage) | mod |  |
| [`LinearMapInner`](#linearmapinner) | struct | Base struct for [`LinearMap`] and [`LinearMapView`] |
| [`IntoIter`](#intoiter) | struct | An iterator that moves out of a [`LinearMap`]. |
| [`Iter`](#iter) | struct | An iterator over the items of a [`LinearMap`] |
| [`IterMut`](#itermut) | struct | An iterator over the items of a [`LinearMap`] that allows modifying the items |
| [`LinearMapStorage`](#linearmapstorage) | trait |  |
| [`OwnedStorage`](#ownedstorage) | type | Implementation of [`LinearMapStorage`] that stores the data in an array whose size is known at compile time. |
| [`ViewStorage`](#viewstorage) | type | Implementation of [`LinearMapStorage`] that stores the data in an unsized slice. |
| [`LinearMap`](#linearmap) | type | A fixed capacity map/dictionary that performs lookups via linear search. |
| [`LinearMapView`](#linearmapview) | type | A dynamic capacity map/dictionary that performs lookups via linear search. |

## Modules

- [`storage`](storage/index.md)

## Structs

### `LinearMapInner<K, V, S: LinearMapStorage<K, V> + ?Sized>`

```rust
struct LinearMapInner<K, V, S: LinearMapStorage<K, V> + ?Sized> {
    buffer: crate::vec::VecInner<(K, V), usize, S>,
}
```

Base struct for [`LinearMap`](#linearmap) and [`LinearMapView`](#linearmapview)

#### Implementations

- <span id="linearmapinner-new"></span>`const fn new() -> Self`

  Creates an empty `LinearMap`.

  

  # Examples

  

  ```rust

  use heapless::LinearMap;

  

  // allocate the map on the stack

  let mut map: LinearMap<&str, isize, 8> = LinearMap::new();

  

  // allocate the map in a static variable

  static mut MAP: LinearMap<&str, isize, 8> = LinearMap::new();

  ```

#### Trait Implementations

##### `impl<K, V, S: LinearMapStorage<K, V> + ?Sized> Debug for LinearMapInner<K, V, S>`

- <span id="linearmapinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DeserializeOwned for LinearMapInner<K, V, S>`

##### `impl<K, V, S: LinearMapStorage<K, V> + ?Sized> Eq for LinearMapInner<K, V, S>`

##### `impl<K, V, Q, S: LinearMapStorage<K, V> + ?Sized> Index for LinearMapInner<K, V, S>`

- <span id="linearmapinner-index-type-output"></span>`type Output = V`

- <span id="linearmapinner-index"></span>`fn index(&self, key: &Q) -> &V`

##### `impl<K, V, Q, S: LinearMapStorage<K, V> + ?Sized> IndexMut for LinearMapInner<K, V, S>`

- <span id="linearmapinner-indexmut-index-mut"></span>`fn index_mut(&mut self, key: &Q) -> &mut V`

##### `impl<K, V, S: LinearMapStorage<K, V> + ?Sized> IntoIterator for &'a LinearMapInner<K, V, S>`

- <span id="a-linearmapinner-intoiterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="a-linearmapinner-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, K, V>`

- <span id="a-linearmapinner-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<K, V1, V2, S1: LinearMapStorage<K, V1> + ?Sized, S2: LinearMapStorage<K, V2> + ?Sized> PartialEq for LinearMapInner<K, V1, S1>`

- <span id="linearmapinner-partialeq-eq"></span>`fn eq(&self, other: &LinearMapInner<K, V2, S2>) -> bool` — [`LinearMapInner`](#linearmapinner)

##### `impl<K, V, S: LinearMapStorage<K, V> + ?Sized> Serialize for crate::linear_map::LinearMapInner<K, V, S>`

- <span id="cratelinear-maplinearmapinner-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `IntoIter<K, V, const N: usize>`

```rust
struct IntoIter<K, V, const N: usize>
where
    K: Eq {
    inner: <crate::vec::Vec<(K, V), N, usize> as IntoIterator>::IntoIter,
}
```

An iterator that moves out of a [`LinearMap`](#linearmap).

This struct is created by calling the [`into_iter`](LinearMap::into_iter) method on [`LinearMap`](#linearmap).

#### Trait Implementations

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
    iter: slice::Iter<'a, (K, V)>,
}
```

An iterator over the items of a [`LinearMap`](#linearmap)

This struct is created by calling the [`iter`](LinearMap::iter) method on [`LinearMap`](#linearmap).

#### Trait Implementations

##### `impl<K: clone::Clone, V: clone::Clone> Clone for Iter<'a, K, V>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'a, K, V>` — [`Iter`](#iter)

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Iter<'a, K, V>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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
    iter: slice::IterMut<'a, (K, V)>,
}
```

An iterator over the items of a [`LinearMap`](#linearmap) that allows modifying the items

This struct is created by calling the [`iter_mut`](LinearMap::iter_mut) method on [`LinearMap`](#linearmap).

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IterMut<'a, K, V>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IterMut<'a, K, V>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Traits

### `LinearMapStorage<K, V>`

```rust
trait LinearMapStorage<K, V>: LinearMapStorageSealed<K, V> { ... }
```

Trait defining how data for a [`LinearMap`](super::LinearMap) is stored.

There's two implementations available:

- [`OwnedStorage`](#ownedstorage): stores the data in an array whose size is known at compile time.
- [`ViewStorage`](../string/index.md): stores the data in an unsized slice

This allows [`LinearMap`](#linearmap) to be generic over either sized or unsized storage. The [`linear_map`](super)
module contains a [`LinearMapInner`](#linearmapinner) struct that's generic on [`LinearMapStorage`](storage/index.md),
and two type aliases for convenience:

- [`LinearMap<N>`](crate::linear_map::LinearMap) = `LinearMapInner<OwnedStorage<u8, N>>`
- [`LinearMapView<T>`](crate::linear_map::LinearMapView) = `LinearMapInner<ViewStorage<u8>>`

`LinearMap` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut LinearMap -> &mut LinearMapView` or
`Box<LinearMap> -> Box<LinearMapView>`, or explicitly with [`.as_view()`](crate::linear_map::LinearMap::as_view) or [`.as_mut_view()`](crate::linear_map::LinearMap::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.





#### Implementors

- [`OwnedVecStorage`](../vec/storage/index.md#ownedvecstorage)
- [`ViewVecStorage`](../vec/storage/index.md#viewvecstorage)

## Type Aliases

### `OwnedStorage<K, V, const N: usize>`

```rust
type OwnedStorage<K, V, const N: usize> = crate::vec::OwnedVecStorage<(K, V), N>;
```

Implementation of [`LinearMapStorage`](storage/index.md) that stores the data in an array whose size is known at compile time.

### `ViewStorage<K, V>`

```rust
type ViewStorage<K, V> = crate::vec::ViewVecStorage<(K, V)>;
```

Implementation of [`LinearMapStorage`](storage/index.md) that stores the data in an unsized slice.

### `LinearMap<K, V, const N: usize>`

```rust
type LinearMap<K, V, const N: usize> = LinearMapInner<K, V, OwnedStorage<K, V, N>>;
```

A fixed capacity map/dictionary that performs lookups via linear search.

Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).

### `LinearMapView<K, V>`

```rust
type LinearMapView<K, V> = LinearMapInner<K, V, ViewStorage<K, V>>;
```

A dynamic capacity map/dictionary that performs lookups via linear search.

Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).

