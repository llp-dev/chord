*[futures_util](../../../index.md) / [stream](../../index.md) / [futures_unordered](../index.md) / [iter](index.md)*

---

# Module `iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IterPinMut`](#iterpinmut) | struct | Mutable iterator over all futures in the unordered set. |
| [`IterMut`](#itermut) | struct | Mutable iterator over all futures in the unordered set. |
| [`IterPinRef`](#iterpinref) | struct | Immutable iterator over all futures in the unordered set. |
| [`Iter`](#iter) | struct | Immutable iterator over all the futures in the unordered set. |
| [`IntoIter`](#intoiter) | struct | Owned iterator over all futures in the unordered set. |

## Structs

### `IterPinMut<'a, Fut>`

```rust
struct IterPinMut<'a, Fut> {
    task: *const super::task::Task<Fut>,
    len: usize,
    _marker: core::marker::PhantomData<&'a mut super::FuturesUnordered<Fut>>,
}
```

Mutable iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for IterPinMut<'a, Fut>`

- <span id="iterpinmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> ExactSizeIterator for IterPinMut<'_, Fut>`

##### `impl IntoIterator for IterPinMut<'a, Fut>`

- <span id="iterpinmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterpinmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterpinmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut> Iterator for IterPinMut<'a, Fut>`

- <span id="iterpinmut-iterator-type-item"></span>`type Item = Pin<&'a mut Fut>`

- <span id="iterpinmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterpinmut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<Fut: Send> Send for IterPinMut<'_, Fut>`

##### `impl<Fut: Sync> Sync for IterPinMut<'_, Fut>`

### `IterMut<'a, Fut: Unpin>`

```rust
struct IterMut<'a, Fut: Unpin>(IterPinMut<'a, Fut>);
```

Mutable iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Unpin> Debug for IterMut<'a, Fut>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Unpin> ExactSizeIterator for IterMut<'_, Fut>`

##### `impl IntoIterator for IterMut<'a, Fut>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut: Unpin> Iterator for IterMut<'a, Fut>`

- <span id="itermut-iterator-type-item"></span>`type Item = &'a mut Fut`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterPinRef<'a, Fut>`

```rust
struct IterPinRef<'a, Fut> {
    task: *const super::task::Task<Fut>,
    len: usize,
    pending_next_all: *mut super::task::Task<Fut>,
    _marker: core::marker::PhantomData<&'a super::FuturesUnordered<Fut>>,
}
```

Immutable iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for IterPinRef<'a, Fut>`

- <span id="iterpinref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> ExactSizeIterator for IterPinRef<'_, Fut>`

##### `impl IntoIterator for IterPinRef<'a, Fut>`

- <span id="iterpinref-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterpinref-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterpinref-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut> Iterator for IterPinRef<'a, Fut>`

- <span id="iterpinref-iterator-type-item"></span>`type Item = Pin<&'a Fut>`

- <span id="iterpinref-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterpinref-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<Fut: Send> Send for IterPinRef<'_, Fut>`

##### `impl<Fut: Sync> Sync for IterPinRef<'_, Fut>`

### `Iter<'a, Fut: Unpin>`

```rust
struct Iter<'a, Fut: Unpin>(IterPinRef<'a, Fut>);
```

Immutable iterator over all the futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Unpin> Debug for Iter<'a, Fut>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Unpin> ExactSizeIterator for Iter<'_, Fut>`

##### `impl IntoIterator for Iter<'a, Fut>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut: Unpin> Iterator for Iter<'a, Fut>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a Fut`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IntoIter<Fut: Unpin>`

```rust
struct IntoIter<Fut: Unpin> {
    len: usize,
    inner: super::FuturesUnordered<Fut>,
}
```

Owned iterator over all futures in the unordered set.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Unpin> Debug for IntoIter<Fut>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Unpin> ExactSizeIterator for IntoIter<Fut>`

##### `impl IntoIterator for IntoIter<Fut>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Fut: Unpin> Iterator for IntoIter<Fut>`

- <span id="intoiter-iterator-type-item"></span>`type Item = Fut`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<Fut: Send + Unpin> Send for IntoIter<Fut>`

##### `impl<Fut: Sync + Unpin> Sync for IntoIter<Fut>`

