*[ring](../../index.md) / [polyfill](../index.md) / [array_flat_map](index.md)*

---

# Module `array_flat_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArrayFlatMap`](#arrayflatmap) | struct | A specialized version of `core::iter::FlatMap` for mapping over exact-sized iterators with a function that returns an array. |

## Structs

### `ArrayFlatMap<I, Item, F, const LEN: usize>`

```rust
struct ArrayFlatMap<I, Item, F, const LEN: usize> {
    inner: core::iter::FlatMap<I, [Item; LEN], F>,
    remaining: usize,
}
```

A specialized version of `core::iter::FlatMap` for mapping over exact-sized
iterators with a function that returns an array.

`ArrayFlatMap` differs from `FlatMap` in that `ArrayFlatMap` implements
`ExactSizeIterator`. Since the result of `F` always has `LEN` elements, if
`I` is an exact-sized iterator of length `inner_len` then we know the
length of the flat-mapped result is `inner_len * LEN`. (The constructor
verifies that this multiplication doesn't overflow `usize`.)

#### Implementations

- <span id="arrayflatmap-new"></span>`fn new(inner: I, f: F) -> Option<Self>`

  Constructs an `ArrayFlatMap` wrapping the given iterator, using the

  given function

#### Trait Implementations

##### `impl<I: clone::Clone, Item: clone::Clone, F: clone::Clone> Clone for ArrayFlatMap<I, Item, F, LEN>`

- <span id="arrayflatmap-clone"></span>`fn clone(&self) -> ArrayFlatMap<I, Item, F, LEN>` — [`ArrayFlatMap`](#arrayflatmap)

##### `impl<I, Item, F> ExactSizeIterator for ArrayFlatMap<I, Item, F, LEN>`

##### `impl<I> IntoIterator for ArrayFlatMap<I, Item, F, LEN>`

- <span id="arrayflatmap-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayflatmap-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayflatmap-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, Item, F> Iterator for ArrayFlatMap<I, Item, F, LEN>`

- <span id="arrayflatmap-iterator-type-item"></span>`type Item = Item`

- <span id="arrayflatmap-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayflatmap-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

  Required for implementing `ExactSizeIterator`.

