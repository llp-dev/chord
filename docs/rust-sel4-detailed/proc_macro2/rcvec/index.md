*[proc_macro2](../index.md) / [rcvec](index.md)*

---

# Module `rcvec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RcVec`](#rcvec) | struct |  |
| [`RcVecBuilder`](#rcvecbuilder) | struct |  |
| [`RcVecMut`](#rcvecmut) | struct |  |
| [`RcVecIntoIter`](#rcvecintoiter) | struct |  |

## Structs

### `RcVec<T>`

```rust
struct RcVec<T> {
    inner: alloc::rc::Rc<alloc::vec::Vec<T>>,
}
```

#### Implementations

- <span id="rcvec-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="rcvec-len"></span>`fn len(&self) -> usize`

- <span id="rcvec-iter"></span>`fn iter(&self) -> slice::Iter<'_, T>`

- <span id="rcvec-make-mut"></span>`fn make_mut(&mut self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvec-get-mut"></span>`fn get_mut(&mut self) -> Option<RcVecMut<'_, T>>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvec-make-owned"></span>`fn make_owned(self) -> RcVecBuilder<T>` — [`RcVecBuilder`](#rcvecbuilder)

#### Trait Implementations

##### `impl<T> Clone for RcVec<T>`

- <span id="rcvec-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> RefUnwindSafe for RcVec<T>`

### `RcVecBuilder<T>`

```rust
struct RcVecBuilder<T> {
    inner: alloc::vec::Vec<T>,
}
```

#### Implementations

- <span id="rcvecbuilder-new"></span>`fn new() -> Self`

- <span id="rcvecbuilder-with-capacity"></span>`fn with_capacity(cap: usize) -> Self`

- <span id="rcvecbuilder-push"></span>`fn push(&mut self, element: T)`

- <span id="rcvecbuilder-extend"></span>`fn extend(&mut self, iter: impl IntoIterator<Item = T>)`

- <span id="rcvecbuilder-as-mut"></span>`fn as_mut(&mut self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvecbuilder-build"></span>`fn build(self) -> RcVec<T>` — [`RcVec`](#rcvec)

#### Trait Implementations

##### `impl<T> IntoIterator for RcVecBuilder<T>`

- <span id="rcvecbuilder-intoiterator-type-item"></span>`type Item = T`

- <span id="rcvecbuilder-intoiterator-type-intoiter"></span>`type IntoIter = RcVecIntoIter<T>`

- <span id="rcvecbuilder-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `RcVecMut<'a, T>`

```rust
struct RcVecMut<'a, T> {
    inner: &'a mut alloc::vec::Vec<T>,
}
```

#### Implementations

- <span id="rcvecmut-push"></span>`fn push(&mut self, element: T)`

- <span id="rcvecmut-extend"></span>`fn extend(&mut self, iter: impl IntoIterator<Item = T>)`

- <span id="rcvecmut-as-mut"></span>`fn as_mut(&mut self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvecmut-take"></span>`fn take(self) -> RcVecBuilder<T>` — [`RcVecBuilder`](#rcvecbuilder)

### `RcVecIntoIter<T>`

```rust
struct RcVecIntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RcVecIntoIter<T>`

- <span id="rcvecintoiter-clone"></span>`fn clone(&self) -> RcVecIntoIter<T>` — [`RcVecIntoIter`](#rcvecintoiter)

##### `impl IntoIterator for RcVecIntoIter<T>`

- <span id="rcvecintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rcvecintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rcvecintoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RcVecIntoIter<T>`

- <span id="rcvecintoiter-iterator-type-item"></span>`type Item = T`

- <span id="rcvecintoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="rcvecintoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

