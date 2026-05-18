*[ring](../index.md) / [polyfill](index.md)*

---

# Module `polyfill`

Polyfills for functionality that will (hopefully) be added to Rust's
standard library soon.

## Contents

- [Modules](#modules)
  - [`array_flat_map`](#array-flat-map)
  - [`array_flatten`](#array-flatten)
  - [`array_split_map`](#array-split-map)
  - [`leading_zeros_skipped`](#leading-zeros-skipped)
  - [`unwrap_const`](#unwrap-const)
- [Structs](#structs)
  - [`ArrayFlatMap`](#arrayflatmap)
  - [`LeadingZerosStripped`](#leadingzerosstripped)
- [Traits](#traits)
  - [`ArrayFlatten`](#arrayflatten)
  - [`ArraySplitMap`](#arraysplitmap)
- [Functions](#functions)
  - [`u64_from_usize`](#u64-from-usize)
  - [`usize_from_u32`](#usize-from-u32)
  - [`usize_from_u64_saturated`](#usize-from-u64-saturated)
  - [`unwrap_const`](#unwrap-const)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`array_flat_map`](#array-flat-map) | mod |  |
| [`array_flatten`](#array-flatten) | mod |  |
| [`array_split_map`](#array-split-map) | mod |  |
| [`leading_zeros_skipped`](#leading-zeros-skipped) | mod |  |
| [`unwrap_const`](#unwrap-const) | mod |  |
| [`ArrayFlatMap`](#arrayflatmap) | struct |  |
| [`LeadingZerosStripped`](#leadingzerosstripped) | struct |  |
| [`ArrayFlatten`](#arrayflatten) | trait |  |
| [`ArraySplitMap`](#arraysplitmap) | trait |  |
| [`u64_from_usize`](#u64-from-usize) | fn |  |
| [`usize_from_u32`](#usize-from-u32) | fn |  |
| [`usize_from_u64_saturated`](#usize-from-u64-saturated) | fn | const-capable `x.try_into().unwrap_or(usize::MAX)` |
| [`unwrap_const`](#unwrap-const) | fn |  |

## Modules

- [`array_flat_map`](array_flat_map/index.md)
- [`array_flatten`](array_flatten/index.md)
- [`array_split_map`](array_split_map/index.md)
- [`leading_zeros_skipped`](leading_zeros_skipped/index.md)
- [`unwrap_const`](unwrap_const/index.md)

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

- <span id="arrayflatmap-clone"></span>`fn clone(&self) -> ArrayFlatMap<I, Item, F, LEN>` — [`ArrayFlatMap`](array_flat_map/index.md#arrayflatmap)

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

### `LeadingZerosStripped<I>`

```rust
struct LeadingZerosStripped<I>
where
    I: Iterator {
    inner: core::iter::Peekable<I>,
}
```

An iterator that skips all leading zeros.

When the wrapped iterator is all zeros, then the last item is retained.

#### Implementations

- <span id="leadingzerosstripped-new"></span>`fn new(inner: I) -> Self`

#### Trait Implementations

##### `impl<I> Clone for LeadingZerosStripped<I>`

- <span id="leadingzerosstripped-clone"></span>`fn clone(&self) -> Self`

##### `impl<I> ExactSizeIterator for LeadingZerosStripped<I>`

##### `impl<I> IntoIterator for LeadingZerosStripped<I>`

- <span id="leadingzerosstripped-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="leadingzerosstripped-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="leadingzerosstripped-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for LeadingZerosStripped<I>`

- <span id="leadingzerosstripped-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="leadingzerosstripped-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="leadingzerosstripped-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Traits

### `ArrayFlatten`

```rust
trait ArrayFlatten { ... }
```

#### Associated Types

- `type Output`

#### Required Methods

- `fn array_flatten(self) -> <Self as >::Output`

  Returns the flattened form of `a`

#### Implementors

- `[[T; 4]; 4]`
- `[[T; 8]; 2]`

### `ArraySplitMap<I, O, const CN: usize, const ON: usize>`

```rust
trait ArraySplitMap<I, O, const CN: usize, const ON: usize> { ... }
```

#### Required Methods

- `fn array_split_map(self, f: impl Fn([I; CN]) -> O) -> [O; ON]`

#### Implementors

- `[I; 12]`
- `[I; 16]`
- `[I; 32]`

## Functions

### `u64_from_usize`

```rust
const fn u64_from_usize(x: usize) -> u64
```

### `usize_from_u32`

```rust
fn usize_from_u32(x: u32) -> usize
```

### `usize_from_u64_saturated`

```rust
const fn usize_from_u64_saturated(x: u64) -> usize
```

const-capable `x.try_into().unwrap_or(usize::MAX)`

### `unwrap_const`

```rust
const fn unwrap_const<T>(x: Option<T>) -> T
where
    T: Copy
```

Polyfill for `Option::unwrap()` as a const fn; feature `const_option`.
https://github.com/rust-lang/rust/issues/67441.
TODO(MSRV): Replace this with `x.unwrap()`.

`T: Copy` avoids "constant functions cannot evaluate destructors."

