# Crate `fallible_iterator`

"Fallible" iterators.

The iterator APIs in the Rust standard library do not support iteration
that can fail in a first class manner. These iterators are typically modeled
as iterating over `Result<T, E>` values; for example, the `Lines` iterator
returns `io::Result<String>`s. When simply iterating over these types, the
value being iterated over must be unwrapped in some way before it can be
used:

```ignore
for line in reader.lines() {
    let line = line?;
    // work with line
}
```

In addition, many of the additional methods on the `Iterator` trait will
not behave properly in the presence of errors when working with these kinds
of iterators. For example, if one wanted to count the number of lines of
text in a `Read`er, this might be a way to go about it:

```ignore
let count = reader.lines().count();
```

This will return the proper value when the reader operates successfully, but
if it encounters an IO error, the result will either be slightly higher than
expected if the error is transient, or it may run forever if the error is
returned repeatedly!

In contrast, a fallible iterator is built around the concept that a call to
`next` can fail. The trait has an additional `Error` associated type in
addition to the `Item` type, and `next` returns `Result<Option<Self::Item>,
Self::Error>` rather than `Option<Self::Item>`. Methods like `count` return
`Result`s as well.

This does mean that fallible iterators are incompatible with Rust's `for`
loop syntax, but `while let` loops offer a similar level of ergonomics:

```ignore
while let Some(item) = iter.next()? {
    // work with item
}
```

## Fallible closure arguments

Like `Iterator`, many `FallibleIterator` methods take closures as arguments.
These use the same signatures as their `Iterator` counterparts, except that
`FallibleIterator` expects the closures to be fallible: they return
`Result<T, Self::Error>` instead of simply `T`.

For example, the standard library's `Iterator::filter` adapter method
filters the underlying iterator according to a predicate provided by the
user, whose return type is `bool`. In `FallibleIterator::filter`, however,
the predicate returns `Result<bool, Self::Error>`:

```rust
use std::error::Error;
use std::str::FromStr;
use fallible_iterator::{convert, FallibleIterator};
let numbers = convert("100\n200\nfern\n400".lines().map(Ok::<&str, Box<Error>>));
let big_numbers = numbers.filter(|n| Ok(u64::from_str(n)? > 100));
assert!(big_numbers.count().is_err());
```

## Contents

- [Structs](#structs)
  - [`Map`](#map)
  - [`Chain`](#chain)
  - [`Cloned`](#cloned)
  - [`Convert`](#convert)
  - [`IntoFallible`](#intofallible)
  - [`Enumerate`](#enumerate)
  - [`Filter`](#filter)
  - [`FilterMap`](#filtermap)
  - [`FlatMap`](#flatmap)
  - [`Flatten`](#flatten)
  - [`FromFn`](#fromfn)
  - [`Fuse`](#fuse)
  - [`Inspect`](#inspect)
  - [`Iterator`](#iterator)
  - [`MapErr`](#maperr)
  - [`Peekable`](#peekable)
  - [`Rev`](#rev)
  - [`Scan`](#scan)
  - [`Skip`](#skip)
  - [`SkipWhile`](#skipwhile)
  - [`StepBy`](#stepby)
  - [`Take`](#take)
  - [`TakeWhile`](#takewhile)
  - [`Cycle`](#cycle)
  - [`Zip`](#zip)
  - [`Unwrap`](#unwrap)
  - [`Empty`](#empty)
  - [`Once`](#once)
  - [`OnceErr`](#onceerr)
  - [`Repeat`](#repeat)
  - [`RepeatErr`](#repeaterr)
- [Enums](#enums)
  - [`FoldStop`](#foldstop)
  - [`ChainState`](#chainstate)
  - [`MappedErr`](#mappederr)
- [Traits](#traits)
  - [`ResultExt`](#resultext)
  - [`FallibleIterator`](#fallibleiterator)
  - [`DoubleEndedFallibleIterator`](#doubleendedfallibleiterator)
  - [`IntoFallibleIterator`](#intofallibleiterator)
  - [`IteratorExt`](#iteratorext)
- [Functions](#functions)
  - [`convert`](#convert)
  - [`from_fn`](#from-fn)
  - [`_is_object_safe`](#is-object-safe)
  - [`empty`](#empty)
  - [`once`](#once)
  - [`once_err`](#once-err)
  - [`repeat`](#repeat)
  - [`repeat_err`](#repeat-err)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Map`](#map) | struct | An iterator which applies a fallible transform to the elements of the underlying iterator. |
| [`Chain`](#chain) | struct | An iterator which yields the elements of one iterator followed by another. |
| [`Cloned`](#cloned) | struct | An iterator which clones the elements of the underlying iterator. |
| [`Convert`](#convert) | struct | A fallible iterator that wraps a normal iterator over `Result`s. |
| [`IntoFallible`](#intofallible) | struct | A fallible iterator that wraps a normal iterator over `Result`s. |
| [`Enumerate`](#enumerate) | struct | An iterator that yields the iteration count as well as the values of the underlying iterator. |
| [`Filter`](#filter) | struct | An iterator which uses a fallible predicate to determine which values of the underlying iterator should be yielded. |
| [`FilterMap`](#filtermap) | struct | An iterator which both filters and maps the values of the underlying iterator. |
| [`FlatMap`](#flatmap) | struct | An iterator which maps each element to another iterator, yielding those iterator's elements. |
| [`Flatten`](#flatten) | struct | An iterator which flattens an iterator of iterators, yielding those iterators' elements. |
| [`FromFn`](#fromfn) | struct | An iterator using a function to generate new values. |
| [`Fuse`](#fuse) | struct | An iterator that yields `Ok(None)` forever after the underlying iterator yields `Ok(None)` once. |
| [`Inspect`](#inspect) | struct | An iterator which passes each element to a closure before returning it. |
| [`Iterator`](#iterator) | struct | A normal (non-fallible) iterator which wraps a fallible iterator. |
| [`MapErr`](#maperr) | struct | An iterator which applies a transform to the errors of the underlying iterator. |
| [`Peekable`](#peekable) | struct | An iterator which can look at the next element without consuming it. |
| [`Rev`](#rev) | struct | An iterator which yields elements of the underlying iterator in reverse order. |
| [`Scan`](#scan) | struct | An iterator which applies a stateful closure. |
| [`Skip`](#skip) | struct | An iterator which skips initial elements. |
| [`SkipWhile`](#skipwhile) | struct | An iterator which skips initial elements based on a predicate. |
| [`StepBy`](#stepby) | struct | An iterator which steps through the elements of the underlying iterator by a certain amount. |
| [`Take`](#take) | struct | An iterator which yields a limited number of elements from the underlying iterator. |
| [`TakeWhile`](#takewhile) | struct | An iterator which yields elements based on a predicate. |
| [`Cycle`](#cycle) | struct | An iterator which cycles another endlessly. |
| [`Zip`](#zip) | struct | An iterator that yields pairs of this iterator's and another iterator's values. |
| [`Unwrap`](#unwrap) | struct | An iterator that unwraps every element yielded by the underlying FallibleIterator |
| [`Empty`](#empty) | struct | An iterator that yields nothing. |
| [`Once`](#once) | struct | An iterator that yields something exactly once. |
| [`OnceErr`](#onceerr) | struct | An iterator that fails with a predetermined error exactly once. |
| [`Repeat`](#repeat) | struct | An iterator that endlessly repeats a single element. |
| [`RepeatErr`](#repeaterr) | struct | An iterator that endlessly repeats a single error. |
| [`FoldStop`](#foldstop) | enum |  |
| [`ChainState`](#chainstate) | enum |  |
| [`MappedErr`](#mappederr) | enum |  |
| [`ResultExt`](#resultext) | trait |  |
| [`FallibleIterator`](#fallibleiterator) | trait | An `Iterator`-like trait that allows for calculation of items to fail. |
| [`DoubleEndedFallibleIterator`](#doubleendedfallibleiterator) | trait | A fallible iterator able to yield elements from both ends. |
| [`IntoFallibleIterator`](#intofallibleiterator) | trait | Conversion into a `FallibleIterator`. |
| [`IteratorExt`](#iteratorext) | trait | An extnsion-trait with set of useful methods to convert [`core::iter::Iterator`] into [`FallibleIterator`] |
| [`convert`](#convert) | fn | Converts an `Iterator<Item = Result<T, E>>` into a `FallibleIterator<Item = T, Error = E>`. |
| [`from_fn`](#from-fn) | fn | Creates an iterator from a fallible function generating values. |
| [`_is_object_safe`](#is-object-safe) | fn |  |
| [`empty`](#empty) | fn | Creates an iterator that yields nothing. |
| [`once`](#once) | fn | Creates an iterator that yields an element exactly once. |
| [`once_err`](#once-err) | fn | Creates an iterator that fails with a predetermined error exactly once. |
| [`repeat`](#repeat) | fn | Creates an iterator that endlessly repeats a single element. |
| [`repeat_err`](#repeat-err) | fn | Creates an iterator that endlessly repeats a single error. |

## Structs

### `Map<T, F>`

```rust
struct Map<T, F> {
    it: T,
    f: F,
}
```

An iterator which applies a fallible transform to the elements of the
underlying iterator.

#### Trait Implementations

##### `impl<T: clone::Clone, F: clone::Clone> Clone for Map<T, F>`

- <span id="map-clone"></span>`fn clone(&self) -> Map<T, F>` — [`Map`](#map)

##### `impl<I: core::fmt::Debug, F> Debug for Map<I, F>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<F, I> DoubleEndedFallibleIterator for Map<I, F>`

- <span id="map-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<B>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="map-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<C, E, G>(&mut self, init: C, f: G) -> Result<C, E>`

##### `impl<T, F> FallibleIterator for Map<T, F>`

- <span id="map-fallibleiterator-type-item"></span>`type Item = B`

- <span id="map-fallibleiterator-type-error"></span>`type Error = <T as FallibleIterator>::Error`

- <span id="map-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<B>, <T as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="map-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="map-fallibleiterator-try-fold"></span>`fn try_fold<C, E, G>(&mut self, init: C, f: G) -> Result<C, E>`

##### `impl IntoFallibleIterator for Map<T, F>`

- <span id="map-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="map-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="map-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="map-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Chain<T, U>`

```rust
struct Chain<T, U> {
    front: T,
    back: U,
    state: ChainState,
}
```

An iterator which yields the elements of one iterator followed by another.

#### Trait Implementations

##### `impl<T: clone::Clone, U: clone::Clone> Clone for Chain<T, U>`

- <span id="chain-clone"></span>`fn clone(&self) -> Chain<T, U>` — [`Chain`](#chain)

##### `impl<T: fmt::Debug, U: fmt::Debug> Debug for Chain<T, U>`

- <span id="chain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> DoubleEndedFallibleIterator for Chain<T, U>`

- <span id="chain-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<<T as >::Item>, <T as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="chain-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<T, U> FallibleIterator for Chain<T, U>`

- <span id="chain-fallibleiterator-type-item"></span>`type Item = <T as FallibleIterator>::Item`

- <span id="chain-fallibleiterator-type-error"></span>`type Error = <T as FallibleIterator>::Error`

- <span id="chain-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<T as >::Item>, <T as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="chain-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="chain-fallibleiterator-count"></span>`fn count(self) -> Result<usize, <T as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="chain-fallibleiterator-try-fold"></span>`fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

- <span id="chain-fallibleiterator-find"></span>`fn find<F>(&mut self, f: F) -> Result<Option<<T as >::Item>, <T as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="chain-fallibleiterator-last"></span>`fn last(self) -> Result<Option<<T as >::Item>, <T as >::Error>` — [`FallibleIterator`](#fallibleiterator)

##### `impl IntoFallibleIterator for Chain<T, U>`

- <span id="chain-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="chain-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="chain-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="chain-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Cloned<I>`

```rust
struct Cloned<I>(I);
```

An iterator which clones the elements of the underlying iterator.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Cloned<I>`

- <span id="cloned-clone"></span>`fn clone(&self) -> Cloned<I>` — [`Cloned`](#cloned)

##### `impl<I: fmt::Debug> Debug for Cloned<I>`

- <span id="cloned-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> DoubleEndedFallibleIterator for Cloned<I>`

- <span id="cloned-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<T>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="cloned-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<I> FallibleIterator for Cloned<I>`

- <span id="cloned-fallibleiterator-type-item"></span>`type Item = T`

- <span id="cloned-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="cloned-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<T>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="cloned-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="cloned-fallibleiterator-try-fold"></span>`fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Cloned<I>`

- <span id="cloned-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="cloned-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="cloned-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="cloned-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Convert<I>`

```rust
struct Convert<I>(I);
```

A fallible iterator that wraps a normal iterator over `Result`s.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Convert<I>`

- <span id="convert-clone"></span>`fn clone(&self) -> Convert<I>` — [`Convert`](#convert)

##### `impl<I: fmt::Debug> Debug for Convert<I>`

- <span id="convert-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> DoubleEndedFallibleIterator for Convert<I>`

- <span id="convert-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<T>, E>`

- <span id="convert-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<B, E2, F>(&mut self, init: B, f: F) -> Result<B, E2>`

##### `impl<I> FallibleIterator for Convert<I>`

- <span id="convert-fallibleiterator-type-item"></span>`type Item = T`

- <span id="convert-fallibleiterator-type-error"></span>`type Error = E`

- <span id="convert-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<T>, E>`

- <span id="convert-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="convert-fallibleiterator-try-fold"></span>`fn try_fold<B, E2, F>(&mut self, init: B, f: F) -> Result<B, E2>`

##### `impl<I> IntoFallibleIterator for Convert<I>`

- <span id="convert-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="convert-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="convert-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="convert-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `IntoFallible<I>`

```rust
struct IntoFallible<I>(I);
```

A fallible iterator that wraps a normal iterator over `Result`s.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for IntoFallible<I>`

- <span id="intofallible-clone"></span>`fn clone(&self) -> IntoFallible<I>` — [`IntoFallible`](#intofallible)

##### `impl<I: fmt::Debug> Debug for IntoFallible<I>`

- <span id="intofallible-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> DoubleEndedFallibleIterator for IntoFallible<I>`

- <span id="intofallible-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<T>, Infallible>`

- <span id="intofallible-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<B, E2, F>(&mut self, init: B, f: F) -> Result<B, E2>`

##### `impl<I> FallibleIterator for IntoFallible<I>`

- <span id="intofallible-fallibleiterator-type-item"></span>`type Item = T`

- <span id="intofallible-fallibleiterator-type-error"></span>`type Error = Infallible`

- <span id="intofallible-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<T>, <Self as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="intofallible-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intofallible-fallibleiterator-try-fold"></span>`fn try_fold<B, E2, F>(&mut self, init: B, f: F) -> Result<B, E2>`

##### `impl<I> IntoFallibleIterator for IntoFallible<I>`

- <span id="intofallible-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="intofallible-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="intofallible-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="intofallible-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Enumerate<I>`

```rust
struct Enumerate<I> {
    it: I,
    n: usize,
}
```

An iterator that yields the iteration count as well as the values of the
underlying iterator.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Enumerate<I>`

- <span id="enumerate-clone"></span>`fn clone(&self) -> Enumerate<I>` — [`Enumerate`](#enumerate)

##### `impl<I: fmt::Debug> Debug for Enumerate<I>`

- <span id="enumerate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> FallibleIterator for Enumerate<I>`

- <span id="enumerate-fallibleiterator-type-item"></span>`type Item = (usize, <I as FallibleIterator>::Item)`

- <span id="enumerate-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="enumerate-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<(usize, <I as >::Item)>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="enumerate-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="enumerate-fallibleiterator-count"></span>`fn count(self) -> Result<usize, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="enumerate-fallibleiterator-nth"></span>`fn nth(&mut self, n: usize) -> Result<Option<(usize, <I as >::Item)>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="enumerate-fallibleiterator-try-fold"></span>`fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Enumerate<I>`

- <span id="enumerate-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="enumerate-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="enumerate-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="enumerate-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Filter<I, F>`

```rust
struct Filter<I, F> {
    it: I,
    f: F,
}
```

An iterator which uses a fallible predicate to determine which values of the
underlying iterator should be yielded.

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Filter<I, F>`

- <span id="filter-clone"></span>`fn clone(&self) -> Filter<I, F>` — [`Filter`](#filter)

##### `impl<I: fmt::Debug, F: fmt::Debug> Debug for Filter<I, F>`

- <span id="filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> DoubleEndedFallibleIterator for Filter<I, F>`

- <span id="filter-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="filter-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<B, E, G>(&mut self, init: B, f: G) -> Result<B, E>`

##### `impl<I, F> FallibleIterator for Filter<I, F>`

- <span id="filter-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="filter-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="filter-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="filter-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="filter-fallibleiterator-try-fold"></span>`fn try_fold<B, E, G>(&mut self, init: B, f: G) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Filter<I, F>`

- <span id="filter-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="filter-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="filter-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="filter-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `FilterMap<I, F>`

```rust
struct FilterMap<I, F> {
    it: I,
    f: F,
}
```

An iterator which both filters and maps the values of the underlying
iterator.

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FilterMap<I, F>`

- <span id="filtermap-clone"></span>`fn clone(&self) -> FilterMap<I, F>` — [`FilterMap`](#filtermap)

##### `impl<I: fmt::Debug, F: fmt::Debug> Debug for FilterMap<I, F>`

- <span id="filtermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> DoubleEndedFallibleIterator for FilterMap<I, F>`

- <span id="filtermap-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<B>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="filtermap-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<C, E, G>(&mut self, init: C, f: G) -> Result<C, E>`

##### `impl<I, F> FallibleIterator for FilterMap<I, F>`

- <span id="filtermap-fallibleiterator-type-item"></span>`type Item = B`

- <span id="filtermap-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="filtermap-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<B>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="filtermap-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="filtermap-fallibleiterator-try-fold"></span>`fn try_fold<C, E, G>(&mut self, init: C, f: G) -> Result<C, E>`

##### `impl<I> IntoFallibleIterator for FilterMap<I, F>`

- <span id="filtermap-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="filtermap-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="filtermap-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="filtermap-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `FlatMap<I, U, F>`

```rust
struct FlatMap<I, U, F>
where
    U: IntoFallibleIterator {
    it: Map<I, F>,
    cur: Option<<U as >::IntoFallibleIter>,
}
```

An iterator which maps each element to another iterator, yielding those iterator's elements.

#### Trait Implementations

##### `impl<I: clone::Clone, U, F: clone::Clone> Clone for FlatMap<I, U, F>`

- <span id="flatmap-clone"></span>`fn clone(&self) -> FlatMap<I, U, F>` — [`FlatMap`](#flatmap)

##### `impl<I: fmt::Debug, U, F: fmt::Debug> Debug for FlatMap<I, U, F>`

- <span id="flatmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, U, F> FallibleIterator for FlatMap<I, U, F>`

- <span id="flatmap-fallibleiterator-type-item"></span>`type Item = <U as IntoFallibleIterator>::Item`

- <span id="flatmap-fallibleiterator-type-error"></span>`type Error = <U as IntoFallibleIterator>::Error`

- <span id="flatmap-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<U as >::Item>, <U as >::Error>` — [`IntoFallibleIterator`](#intofallibleiterator)

- <span id="flatmap-fallibleiterator-try-fold"></span>`fn try_fold<B, E, G>(&mut self, init: B, f: G) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for FlatMap<I, U, F>`

- <span id="flatmap-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="flatmap-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="flatmap-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="flatmap-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Flatten<I>`

```rust
struct Flatten<I>
where
    I: FallibleIterator,
    <I as >::Item: IntoFallibleIterator {
    it: I,
    cur: Option<<<I as >::Item as IntoFallibleIterator>::IntoFallibleIter>,
}
```

An iterator which flattens an iterator of iterators, yielding those iterators' elements.

#### Trait Implementations

##### `impl<I> Clone for Flatten<I>`

- <span id="flatten-clone"></span>`fn clone(&self) -> Flatten<I>` — [`Flatten`](#flatten)

##### `impl<I> FallibleIterator for Flatten<I>`

- <span id="flatten-fallibleiterator-type-item"></span>`type Item = <<I as FallibleIterator>::Item as IntoFallibleIterator>::Item`

- <span id="flatten-fallibleiterator-type-error"></span>`type Error = <<I as FallibleIterator>::Item as IntoFallibleIterator>::Error`

- <span id="flatten-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="flatten-fallibleiterator-try-fold"></span>`fn try_fold<B, E, G>(&mut self, init: B, f: G) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Flatten<I>`

- <span id="flatten-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="flatten-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="flatten-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="flatten-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `FromFn<F>`

```rust
struct FromFn<F> {
    fun: F,
}
```

An iterator using a function to generate new values.

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for FromFn<F>`

- <span id="fromfn-clone"></span>`fn clone(&self) -> FromFn<F>` — [`FromFn`](#fromfn)

##### `impl<F: fmt::Debug> Debug for FromFn<F>`

- <span id="fromfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> FallibleIterator for FromFn<F>`

- <span id="fromfn-fallibleiterator-type-item"></span>`type Item = I`

- <span id="fromfn-fallibleiterator-type-error"></span>`type Error = E`

- <span id="fromfn-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<I>, E>`

##### `impl IntoFallibleIterator for FromFn<F>`

- <span id="fromfn-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="fromfn-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="fromfn-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="fromfn-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Fuse<I>`

```rust
struct Fuse<I> {
    it: I,
    done: bool,
}
```

An iterator that yields `Ok(None)` forever after the underlying iterator
yields `Ok(None)` once.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Fuse<I>`

- <span id="fuse-clone"></span>`fn clone(&self) -> Fuse<I>` — [`Fuse`](#fuse)

##### `impl<I: fmt::Debug> Debug for Fuse<I>`

- <span id="fuse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> FallibleIterator for Fuse<I>`

- <span id="fuse-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="fuse-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="fuse-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="fuse-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="fuse-fallibleiterator-count"></span>`fn count(self) -> Result<usize, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="fuse-fallibleiterator-last"></span>`fn last(self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="fuse-fallibleiterator-nth"></span>`fn nth(&mut self, n: usize) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="fuse-fallibleiterator-try-fold"></span>`fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Fuse<I>`

- <span id="fuse-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="fuse-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="fuse-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="fuse-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Inspect<I, F>`

```rust
struct Inspect<I, F> {
    it: I,
    f: F,
}
```

An iterator which passes each element to a closure before returning it.

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Inspect<I, F>`

- <span id="inspect-clone"></span>`fn clone(&self) -> Inspect<I, F>` — [`Inspect`](#inspect)

##### `impl<I: fmt::Debug, F: fmt::Debug> Debug for Inspect<I, F>`

- <span id="inspect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> DoubleEndedFallibleIterator for Inspect<I, F>`

- <span id="inspect-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="inspect-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<B, E, G>(&mut self, init: B, f: G) -> Result<B, E>`

##### `impl<I, F> FallibleIterator for Inspect<I, F>`

- <span id="inspect-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="inspect-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="inspect-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="inspect-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="inspect-fallibleiterator-try-fold"></span>`fn try_fold<B, E, G>(&mut self, init: B, f: G) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Inspect<I, F>`

- <span id="inspect-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="inspect-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="inspect-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="inspect-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Iterator<I>`

```rust
struct Iterator<I>(I);
```

A normal (non-fallible) iterator which wraps a fallible iterator.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Iterator<I>`

- <span id="iterator-clone"></span>`fn clone(&self) -> Iterator<I>` — [`Iterator`](#iterator)

##### `impl<I: fmt::Debug> Debug for Iterator<I>`

- <span id="iterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> DoubleEndedIterator for Iterator<I>`

- <span id="iterator-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<Result<<I as >::Item, <I as >::Error>>` — [`FallibleIterator`](#fallibleiterator)

##### `impl<I> IntoIterator for Iterator<I>`

- <span id="iterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Iterator<I>`

- <span id="iterator-iterator-type-item"></span>`type Item = Result<<I as FallibleIterator>::Item, <I as FallibleIterator>::Error>`

- <span id="iterator-iterator-next"></span>`fn next(&mut self) -> Option<Result<<I as >::Item, <I as >::Error>>` — [`FallibleIterator`](#fallibleiterator)

- <span id="iterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IteratorExt for Iterator<I>`

- <span id="iterator-iteratorext-transpose-into-fallible"></span>`fn transpose_into_fallible<T, E>(self) -> Convert<Self>` — [`Convert`](#convert)

  Convert an iterator of `Result`s into `FallibleIterator` by transposition

- <span id="iterator-iteratorext-into-fallible"></span>`fn into_fallible<T>(self) -> IntoFallible<Self>` — [`IntoFallible`](#intofallible)

  Convert an iterator of anything into `FallibleIterator` by wrapping

  into `Result<T, Infallible>` where `Infallible` is an error that can never actually

  happen.

### `MapErr<I, F>`

```rust
struct MapErr<I, F> {
    it: I,
    f: F,
}
```

An iterator which applies a transform to the errors of the underlying
iterator.

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for MapErr<I, F>`

- <span id="maperr-clone"></span>`fn clone(&self) -> MapErr<I, F>` — [`MapErr`](#maperr)

##### `impl<I: fmt::Debug, F: fmt::Debug> Debug for MapErr<I, F>`

- <span id="maperr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F, I> DoubleEndedFallibleIterator for MapErr<I, F>`

- <span id="maperr-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<<I as >::Item>, B>` — [`FallibleIterator`](#fallibleiterator)

- <span id="maperr-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<C, E, G>(&mut self, init: C, f: G) -> Result<C, E>`

##### `impl<F, I> FallibleIterator for MapErr<I, F>`

- <span id="maperr-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="maperr-fallibleiterator-type-error"></span>`type Error = B`

- <span id="maperr-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, B>` — [`FallibleIterator`](#fallibleiterator)

- <span id="maperr-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="maperr-fallibleiterator-count"></span>`fn count(self) -> Result<usize, B>`

- <span id="maperr-fallibleiterator-last"></span>`fn last(self) -> Result<Option<<I as >::Item>, B>` — [`FallibleIterator`](#fallibleiterator)

- <span id="maperr-fallibleiterator-nth"></span>`fn nth(&mut self, n: usize) -> Result<Option<<I as >::Item>, B>` — [`FallibleIterator`](#fallibleiterator)

- <span id="maperr-fallibleiterator-try-fold"></span>`fn try_fold<C, E, G>(&mut self, init: C, f: G) -> Result<C, E>`

##### `impl<I> IntoFallibleIterator for MapErr<I, F>`

- <span id="maperr-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="maperr-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="maperr-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="maperr-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Peekable<I: FallibleIterator>`

```rust
struct Peekable<I: FallibleIterator> {
    it: I,
    next: Option<<I as >::Item>,
}
```

An iterator which can look at the next element without consuming it.

#### Implementations

- <span id="peekable-peek"></span>`fn peek(&mut self) -> Result<Option<&<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

  Returns a reference to the next value without advancing the iterator.

- <span id="peekable-next-if"></span>`fn next_if(&mut self, f: impl Fn(&<I as >::Item) -> bool) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

  Consume and return the next value of this iterator if a condition is true.

  

  If func returns true for the next value of this iterator, consume and return it. Otherwise, return None.

- <span id="peekable-next-if-eq"></span>`fn next_if_eq<T>(&mut self, expected: &T) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

  Consume and return the next item if it is equal to `expected`.

#### Trait Implementations

##### `impl<I: clone::Clone + FallibleIterator> Clone for Peekable<I>`

- <span id="peekable-clone"></span>`fn clone(&self) -> Peekable<I>` — [`Peekable`](#peekable)

##### `impl<I: fmt::Debug + FallibleIterator> Debug for Peekable<I>`

- <span id="peekable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> FallibleIterator for Peekable<I>`

- <span id="peekable-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="peekable-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="peekable-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="peekable-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="peekable-fallibleiterator-try-fold"></span>`fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Peekable<I>`

- <span id="peekable-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="peekable-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="peekable-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="peekable-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Rev<I>`

```rust
struct Rev<I>(I);
```

An iterator which yields elements of the underlying iterator in reverse
order.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Rev<I>`

- <span id="rev-clone"></span>`fn clone(&self) -> Rev<I>` — [`Rev`](#rev)

##### `impl<I: fmt::Debug> Debug for Rev<I>`

- <span id="rev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> DoubleEndedFallibleIterator for Rev<I>`

- <span id="rev-doubleendedfallibleiterator-next-back"></span>`fn next_back(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="rev-doubleendedfallibleiterator-try-rfold"></span>`fn try_rfold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<I> FallibleIterator for Rev<I>`

- <span id="rev-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="rev-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="rev-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="rev-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="rev-fallibleiterator-count"></span>`fn count(self) -> Result<usize, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="rev-fallibleiterator-try-fold"></span>`fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

##### `impl<I> IntoFallibleIterator for Rev<I>`

- <span id="rev-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="rev-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="rev-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="rev-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Scan<I, St, F>`

```rust
struct Scan<I, St, F> {
    it: I,
    f: F,
    state: St,
}
```

An iterator which applies a stateful closure.

#### Trait Implementations

##### `impl<I: clone::Clone, St: clone::Clone, F: clone::Clone> Clone for Scan<I, St, F>`

- <span id="scan-clone"></span>`fn clone(&self) -> Scan<I, St, F>` — [`Scan`](#scan)

##### `impl<I: fmt::Debug, St: fmt::Debug, F: fmt::Debug> Debug for Scan<I, St, F>`

- <span id="scan-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, St, F> FallibleIterator for Scan<I, St, F>`

- <span id="scan-fallibleiterator-type-item"></span>`type Item = B`

- <span id="scan-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="scan-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<B>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="scan-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IntoFallibleIterator for Scan<I, St, F>`

- <span id="scan-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="scan-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="scan-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="scan-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Skip<I>`

```rust
struct Skip<I> {
    it: I,
    n: usize,
}
```

An iterator which skips initial elements.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Skip<I>`

- <span id="skip-clone"></span>`fn clone(&self) -> Skip<I>` — [`Skip`](#skip)

##### `impl<I: fmt::Debug> Debug for Skip<I>`

- <span id="skip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> FallibleIterator for Skip<I>`

- <span id="skip-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="skip-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="skip-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="skip-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IntoFallibleIterator for Skip<I>`

- <span id="skip-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="skip-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="skip-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="skip-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `SkipWhile<I, P>`

```rust
struct SkipWhile<I, P> {
    it: I,
    flag: bool,
    predicate: P,
}
```

An iterator which skips initial elements based on a predicate.

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for SkipWhile<I, P>`

- <span id="skipwhile-clone"></span>`fn clone(&self) -> SkipWhile<I, P>` — [`SkipWhile`](#skipwhile)

##### `impl<I: fmt::Debug, P: fmt::Debug> Debug for SkipWhile<I, P>`

- <span id="skipwhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, P> FallibleIterator for SkipWhile<I, P>`

- <span id="skipwhile-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="skipwhile-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="skipwhile-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="skipwhile-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IntoFallibleIterator for SkipWhile<I, P>`

- <span id="skipwhile-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="skipwhile-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="skipwhile-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="skipwhile-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `StepBy<I>`

```rust
struct StepBy<I> {
    it: I,
    step: usize,
    first_take: bool,
}
```

An iterator which steps through the elements of the underlying iterator by a certain amount.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for StepBy<I>`

- <span id="stepby-clone"></span>`fn clone(&self) -> StepBy<I>` — [`StepBy`](#stepby)

##### `impl<I: fmt::Debug> Debug for StepBy<I>`

- <span id="stepby-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> FallibleIterator for StepBy<I>`

- <span id="stepby-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="stepby-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="stepby-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="stepby-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IntoFallibleIterator for StepBy<I>`

- <span id="stepby-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="stepby-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="stepby-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="stepby-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Take<I>`

```rust
struct Take<I> {
    it: I,
    remaining: usize,
}
```

An iterator which yields a limited number of elements from the underlying
iterator.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Take<I>`

- <span id="take-clone"></span>`fn clone(&self) -> Take<I>` — [`Take`](#take)

##### `impl<I: fmt::Debug> Debug for Take<I>`

- <span id="take-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> FallibleIterator for Take<I>`

- <span id="take-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="take-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="take-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="take-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IntoFallibleIterator for Take<I>`

- <span id="take-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="take-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="take-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="take-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `TakeWhile<I, P>`

```rust
struct TakeWhile<I, P> {
    it: I,
    flag: bool,
    predicate: P,
}
```

An iterator which yields elements based on a predicate.

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for TakeWhile<I, P>`

- <span id="takewhile-clone"></span>`fn clone(&self) -> TakeWhile<I, P>` — [`TakeWhile`](#takewhile)

##### `impl<I: fmt::Debug, P: fmt::Debug> Debug for TakeWhile<I, P>`

- <span id="takewhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, P> FallibleIterator for TakeWhile<I, P>`

- <span id="takewhile-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="takewhile-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="takewhile-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="takewhile-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IntoFallibleIterator for TakeWhile<I, P>`

- <span id="takewhile-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="takewhile-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="takewhile-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="takewhile-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Cycle<I>`

```rust
struct Cycle<I> {
    it: I,
    cur: I,
}
```

An iterator which cycles another endlessly.

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Cycle<I>`

- <span id="cycle-clone"></span>`fn clone(&self) -> Cycle<I>` — [`Cycle`](#cycle)

##### `impl<I: fmt::Debug> Debug for Cycle<I>`

- <span id="cycle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> FallibleIterator for Cycle<I>`

- <span id="cycle-fallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="cycle-fallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="cycle-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<I as >::Item>, <I as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="cycle-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<I> IntoFallibleIterator for Cycle<I>`

- <span id="cycle-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="cycle-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="cycle-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="cycle-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Zip<T, U>`

```rust
struct Zip<T, U>(T, U);
```

An iterator that yields pairs of this iterator's and another iterator's
values.

#### Trait Implementations

##### `impl<T: clone::Clone, U: clone::Clone> Clone for Zip<T, U>`

- <span id="zip-clone"></span>`fn clone(&self) -> Zip<T, U>` — [`Zip`](#zip)

##### `impl<T: fmt::Debug, U: fmt::Debug> Debug for Zip<T, U>`

- <span id="zip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> FallibleIterator for Zip<T, U>`

- <span id="zip-fallibleiterator-type-item"></span>`type Item = (<T as FallibleIterator>::Item, <U as FallibleIterator>::Item)`

- <span id="zip-fallibleiterator-type-error"></span>`type Error = <T as FallibleIterator>::Error`

- <span id="zip-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<(<T as >::Item, <U as >::Item)>, <T as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="zip-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl IntoFallibleIterator for Zip<T, U>`

- <span id="zip-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="zip-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="zip-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="zip-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Unwrap<T>`

```rust
struct Unwrap<T>(T);
```

An iterator that unwraps every element yielded by the underlying
FallibleIterator

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Unwrap<T>`

- <span id="unwrap-clone"></span>`fn clone(&self) -> Unwrap<T>` — [`Unwrap`](#unwrap)

##### `impl<T: fmt::Debug> Debug for Unwrap<T>`

- <span id="unwrap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for Unwrap<T>`

- <span id="unwrap-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unwrap-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unwrap-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Unwrap<T>`

- <span id="unwrap-iterator-type-item"></span>`type Item = <T as FallibleIterator>::Item`

- <span id="unwrap-iterator-next"></span>`fn next(&mut self) -> Option<<T as >::Item>` — [`FallibleIterator`](#fallibleiterator)

- <span id="unwrap-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl IteratorExt for Unwrap<T>`

- <span id="unwrap-iteratorext-transpose-into-fallible"></span>`fn transpose_into_fallible<T, E>(self) -> Convert<Self>` — [`Convert`](#convert)

  Convert an iterator of `Result`s into `FallibleIterator` by transposition

- <span id="unwrap-iteratorext-into-fallible"></span>`fn into_fallible<T>(self) -> IntoFallible<Self>` — [`IntoFallible`](#intofallible)

  Convert an iterator of anything into `FallibleIterator` by wrapping

  into `Result<T, Infallible>` where `Infallible` is an error that can never actually

  happen.

### `Empty<T, E>`

```rust
struct Empty<T, E>(core::marker::PhantomData<T>, core::marker::PhantomData<E>);
```

An iterator that yields nothing.

#### Trait Implementations

##### `impl<T: clone::Clone, E: clone::Clone> Clone for Empty<T, E>`

- <span id="empty-clone"></span>`fn clone(&self) -> Empty<T, E>` — [`Empty`](#empty)

##### `impl<T: fmt::Debug, E: fmt::Debug> Debug for Empty<T, E>`

- <span id="empty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E> FallibleIterator for Empty<T, E>`

- <span id="empty-fallibleiterator-type-item"></span>`type Item = T`

- <span id="empty-fallibleiterator-type-error"></span>`type Error = E`

- <span id="empty-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<T>, E>`

- <span id="empty-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl IntoFallibleIterator for Empty<T, E>`

- <span id="empty-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="empty-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="empty-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="empty-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Once<T, E>`

```rust
struct Once<T, E>(Option<T>, core::marker::PhantomData<E>);
```

An iterator that yields something exactly once.

#### Trait Implementations

##### `impl<T: clone::Clone, E: clone::Clone> Clone for Once<T, E>`

- <span id="once-clone"></span>`fn clone(&self) -> Once<T, E>` — [`Once`](#once)

##### `impl<T: fmt::Debug, E: fmt::Debug> Debug for Once<T, E>`

- <span id="once-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E> FallibleIterator for Once<T, E>`

- <span id="once-fallibleiterator-type-item"></span>`type Item = T`

- <span id="once-fallibleiterator-type-error"></span>`type Error = E`

- <span id="once-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="once-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl IntoFallibleIterator for Once<T, E>`

- <span id="once-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="once-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="once-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="once-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `OnceErr<T, E>`

```rust
struct OnceErr<T, E>(core::marker::PhantomData<T>, Option<E>);
```

An iterator that fails with a predetermined error exactly once.

#### Trait Implementations

##### `impl<T: clone::Clone, E: clone::Clone> Clone for OnceErr<T, E>`

- <span id="onceerr-clone"></span>`fn clone(&self) -> OnceErr<T, E>` — [`OnceErr`](#onceerr)

##### `impl<T: fmt::Debug, E: fmt::Debug> Debug for OnceErr<T, E>`

- <span id="onceerr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E> FallibleIterator for OnceErr<T, E>`

- <span id="onceerr-fallibleiterator-type-item"></span>`type Item = T`

- <span id="onceerr-fallibleiterator-type-error"></span>`type Error = E`

- <span id="onceerr-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="onceerr-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl IntoFallibleIterator for OnceErr<T, E>`

- <span id="onceerr-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="onceerr-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="onceerr-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="onceerr-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `Repeat<T: Clone, E>`

```rust
struct Repeat<T: Clone, E>(T, core::marker::PhantomData<E>);
```

An iterator that endlessly repeats a single element.

#### Trait Implementations

##### `impl<T: clone::Clone + Clone, E: clone::Clone> Clone for Repeat<T, E>`

- <span id="repeat-clone"></span>`fn clone(&self) -> Repeat<T, E>` — [`Repeat`](#repeat)

##### `impl<T: fmt::Debug + Clone, E: fmt::Debug> Debug for Repeat<T, E>`

- <span id="repeat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Clone, E> FallibleIterator for Repeat<T, E>`

- <span id="repeat-fallibleiterator-type-item"></span>`type Item = T`

- <span id="repeat-fallibleiterator-type-error"></span>`type Error = E`

- <span id="repeat-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="repeat-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl IntoFallibleIterator for Repeat<T, E>`

- <span id="repeat-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="repeat-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="repeat-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="repeat-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

### `RepeatErr<T, E: Clone>`

```rust
struct RepeatErr<T, E: Clone>(core::marker::PhantomData<T>, E);
```

An iterator that endlessly repeats a single error.

#### Trait Implementations

##### `impl<T: clone::Clone, E: clone::Clone + Clone> Clone for RepeatErr<T, E>`

- <span id="repeaterr-clone"></span>`fn clone(&self) -> RepeatErr<T, E>` — [`RepeatErr`](#repeaterr)

##### `impl<T: fmt::Debug, E: fmt::Debug + Clone> Debug for RepeatErr<T, E>`

- <span id="repeaterr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E: Clone> FallibleIterator for RepeatErr<T, E>`

- <span id="repeaterr-fallibleiterator-type-item"></span>`type Item = T`

- <span id="repeaterr-fallibleiterator-type-error"></span>`type Error = E`

- <span id="repeaterr-fallibleiterator-next"></span>`fn next(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>` — [`FallibleIterator`](#fallibleiterator)

- <span id="repeaterr-fallibleiterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl IntoFallibleIterator for RepeatErr<T, E>`

- <span id="repeaterr-intofallibleiterator-type-item"></span>`type Item = <I as FallibleIterator>::Item`

- <span id="repeaterr-intofallibleiterator-type-error"></span>`type Error = <I as FallibleIterator>::Error`

- <span id="repeaterr-intofallibleiterator-type-intofallibleiter"></span>`type IntoFallibleIter = I`

- <span id="repeaterr-intofallibleiterator-into-fallible-iter"></span>`fn into_fallible_iter(self) -> I`

## Enums

### `FoldStop<T, E>`

```rust
enum FoldStop<T, E> {
    Break(T),
    Err(E),
}
```

### `ChainState`

```rust
enum ChainState {
    Both,
    Front,
    Back,
}
```

#### Trait Implementations

##### `impl Clone for ChainState`

- <span id="chainstate-clone"></span>`fn clone(&self) -> ChainState` — [`ChainState`](#chainstate)

##### `impl Debug for ChainState`

- <span id="chainstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MappedErr<T, U>`

```rust
enum MappedErr<T, U> {
    It(T),
    Fold(U),
}
```

## Traits

### `ResultExt<T, E>`

```rust
trait ResultExt<T, E> { ... }
```

#### Required Methods

- `fn unpack_fold(self) -> Result<T, E>`

#### Implementors

- `Result<T, FoldStop<T, E>>`

### `FallibleIterator`

```rust
trait FallibleIterator { ... }
```

An `Iterator`-like trait that allows for calculation of items to fail.

#### Associated Types

- `type Item`

- `type Error`

#### Required Methods

- `fn next(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Advances the iterator and returns the next value.

#### Provided Methods

- `fn size_hint(&self) -> (usize, Option<usize>)`

  Returns bounds on the remaining length of the iterator.

- `fn count(self) -> Result<usize, <Self as >::Error>`

  Consumes the iterator, returning the number of remaining items.

- `fn last(self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the last element of the iterator.

- `fn nth(&mut self, n: usize) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the `n`th element of the iterator.

- `fn step_by(self, step: usize) -> StepBy<Self>`

  Returns an iterator starting at the same point, but stepping by the given amount at each iteration.

- `fn chain<I>(self, it: I) -> Chain<Self, I>`

  Returns an iterator which yields the elements of this iterator followed

- `fn zip<I>(self, o: I) -> Zip<Self, <I as >::IntoFallibleIter>`

  Returns an iterator that yields pairs of this iterator's and another

- `fn map<F, B>(self, f: F) -> Map<Self, F>`

  Returns an iterator which applies a fallible transform to the elements

- `fn for_each<F>(self, f: F) -> Result<(), <Self as >::Error>`

  Calls a fallible closure on each element of an iterator.

- `fn filter<F>(self, f: F) -> Filter<Self, F>`

  Returns an iterator which uses a predicate to determine which values

- `fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>`

  Returns an iterator which both filters and maps. The closure may fail;

- `fn enumerate(self) -> Enumerate<Self>`

  Returns an iterator which yields the current iteration count as well

- `fn peekable(self) -> Peekable<Self>`

  Returns an iterator that can peek at the next element without consuming

- `fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>`

  Returns an iterator that skips elements based on a predicate.

- `fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>`

  Returns an iterator that yields elements based on a predicate.

- `fn skip(self, n: usize) -> Skip<Self>`

  Returns an iterator which skips the first `n` values of this iterator.

- `fn take(self, n: usize) -> Take<Self>`

  Returns an iterator that yields only the first `n` values of this

- `fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>`

  Returns an iterator which applies a stateful map to values of this

- `fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>`

  Returns an iterator which maps this iterator's elements to iterators, yielding those iterators' values.

- `fn flatten(self) -> Flatten<Self>`

  Returns an iterator which flattens an iterator of iterators, yielding those iterators' values.

- `fn fuse(self) -> Fuse<Self>`

  Returns an iterator which yields this iterator's elements and ends after

- `fn inspect<F>(self, f: F) -> Inspect<Self, F>`

  Returns an iterator which passes each element to a closure before returning it.

- `fn by_ref(&mut self) -> &mut Self`

  Borrow an iterator rather than consuming it.

- `fn collect<T>(self) -> Result<T, <Self as >::Error>`

  Transforms the iterator into a collection.

- `fn partition<B, F>(self, f: F) -> Result<(B, B), <Self as >::Error>`

  Transforms the iterator into two collections, partitioning elements by a closure.

- `fn fold<B, F>(self, init: B, f: F) -> Result<B, <Self as >::Error>`

  Applies a function over the elements of the iterator, producing a single

- `fn try_fold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

  Applies a function over the elements of the iterator, producing a single final value.

- `fn all<F>(&mut self, f: F) -> Result<bool, <Self as >::Error>`

  Determines if all elements of this iterator match a predicate.

- `fn any<F>(&mut self, f: F) -> Result<bool, <Self as >::Error>`

  Determines if any element of this iterator matches a predicate.

- `fn find<F>(&mut self, f: F) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the first element of the iterator that matches a predicate.

- `fn find_map<B, F>(&mut self, f: F) -> Result<Option<B>, <Self as >::Error>`

  Applies a function to the elements of the iterator, returning the first non-`None` result.

- `fn position<F>(&mut self, f: F) -> Result<Option<usize>, <Self as >::Error>`

  Returns the position of the first element of this iterator that matches

- `fn max(self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the maximal element of the iterator.

- `fn max_by_key<B, F>(self, f: F) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the element of the iterator which gives the maximum value from

- `fn max_by<F>(self, f: F) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the element that gives the maximum value with respect to the function.

- `fn min(self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the minimal element of the iterator.

- `fn min_by_key<B, F>(self, f: F) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the element of the iterator which gives the minimum value from

- `fn min_by<F>(self, f: F) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Returns the element that gives the minimum value with respect to the function.

- `fn rev(self) -> Rev<Self>`

  Returns an iterator that yields this iterator's items in the opposite

- `fn unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), <Self as >::Error>`

  Converts an iterator of pairs into a pair of containers.

- `fn cloned<'a, T>(self) -> Cloned<Self>`

  Returns an iterator which clones all of its elements.

- `fn cycle(self) -> Cycle<Self>`

  Returns an iterator which repeats this iterator endlessly.

- `fn cmp<I>(self, other: I) -> Result<Ordering, <Self as >::Error>`

  Lexicographically compares the elements of this iterator to that of

- `fn partial_cmp<I>(self, other: I) -> Result<Option<Ordering>, <Self as >::Error>`

  Lexicographically compares the elements of this iterator to that of

- `fn eq<I>(self, other: I) -> Result<bool, <Self as >::Error>`

  Determines if the elements of this iterator are equal to those of

- `fn ne<I>(self, other: I) -> Result<bool, <Self as >::Error>`

  Determines if the elements of this iterator are not equal to those of

- `fn lt<I>(self, other: I) -> Result<bool, <Self as >::Error>`

  Determines if the elements of this iterator are lexicographically less

- `fn le<I>(self, other: I) -> Result<bool, <Self as >::Error>`

  Determines if the elements of this iterator are lexicographically less

- `fn gt<I>(self, other: I) -> Result<bool, <Self as >::Error>`

  Determines if the elements of this iterator are lexicographically

- `fn ge<I>(self, other: I) -> Result<bool, <Self as >::Error>`

  Determines if the elements of this iterator are lexicographically

- `fn iterator(self) -> Iterator<Self>`

  Returns a normal (non-fallible) iterator over `Result<Item, Error>`.

- `fn map_err<B, F>(self, f: F) -> MapErr<Self, F>`

  Returns an iterator which applies a transform to the errors of the

- `fn unwrap<T>(self) -> Unwrap<Self>`

  Returns an iterator which unwraps all of its elements.

#### Implementors

- [`Chain`](#chain)
- [`Cloned`](#cloned)
- [`Convert`](#convert)
- [`Cycle`](#cycle)
- [`Empty`](#empty)
- [`Enumerate`](#enumerate)
- [`FilterMap`](#filtermap)
- [`Filter`](#filter)
- [`FlatMap`](#flatmap)
- [`Flatten`](#flatten)
- [`FromFn`](#fromfn)
- [`Fuse`](#fuse)
- [`Inspect`](#inspect)
- [`IntoFallible`](#intofallible)
- [`MapErr`](#maperr)
- [`Map`](#map)
- [`OnceErr`](#onceerr)
- [`Once`](#once)
- [`Peekable`](#peekable)
- [`RepeatErr`](#repeaterr)
- [`Repeat`](#repeat)
- [`Rev`](#rev)
- [`Scan`](#scan)
- [`SkipWhile`](#skipwhile)
- [`Skip`](#skip)
- [`StepBy`](#stepby)
- [`TakeWhile`](#takewhile)
- [`Take`](#take)
- [`Zip`](#zip)
- `&mut I`
- `alloc::boxed::Box<I>`

### `DoubleEndedFallibleIterator`

```rust
trait DoubleEndedFallibleIterator: FallibleIterator { ... }
```

A fallible iterator able to yield elements from both ends.

#### Required Methods

- `fn next_back(&mut self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`

  Advances the end of the iterator, returning the last value.

#### Provided Methods

- `fn rfold<B, F>(self, init: B, f: F) -> Result<B, <Self as >::Error>`

  Applies a function over the elements of the iterator in reverse order, producing a single final value.

- `fn try_rfold<B, E, F>(&mut self, init: B, f: F) -> Result<B, E>`

  Applies a function over the elements of the iterator in reverse, producing a single final value.

#### Implementors

- [`Chain`](#chain)
- [`Cloned`](#cloned)
- [`Convert`](#convert)
- [`FilterMap`](#filtermap)
- [`Filter`](#filter)
- [`Inspect`](#inspect)
- [`IntoFallible`](#intofallible)
- [`MapErr`](#maperr)
- [`Map`](#map)
- [`Rev`](#rev)
- `&mut I`
- `alloc::boxed::Box<I>`

### `IntoFallibleIterator`

```rust
trait IntoFallibleIterator { ... }
```

Conversion into a `FallibleIterator`.

#### Associated Types

- `type Item`

- `type Error`

- `type IntoFallibleIter: 1`

#### Required Methods

- `fn into_fallible_iter(self) -> <Self as >::IntoFallibleIter`

  Creates a fallible iterator from a value.

#### Implementors

- `I`

### `IteratorExt`

```rust
trait IteratorExt { ... }
```

An extnsion-trait with set of useful methods to convert [`core::iter::Iterator`](#coreiteriterator)
into [`FallibleIterator`](#fallibleiterator)

#### Required Methods

- `fn transpose_into_fallible<T, E>(self) -> Convert<Self>`

  Convert an iterator of `Result`s into `FallibleIterator` by transposition

- `fn into_fallible<T>(self) -> IntoFallible<Self>`

  Convert an iterator of anything into `FallibleIterator` by wrapping

#### Implementors

- `I`

## Functions

### `convert`

```rust
fn convert<T, E, I>(it: I) -> Convert<I>
where
    I: iter::Iterator<Item = Result<T, E>>
```

Converts an `Iterator<Item = Result<T, E>>` into a `FallibleIterator<Item = T, Error = E>`.

### `from_fn`

```rust
fn from_fn<I, E, F>(fun: F) -> FromFn<F>
where
    F: FnMut() -> Result<Option<I>, E>
```

Creates an iterator from a fallible function generating values.

```rust
use fallible_iterator::{from_fn, FallibleIterator};
let mut count = 0;
let counter = from_fn(move || {
    // Increment our count. This is why we started at zero.
    count += 1;

    // Check to see if we've finished counting or not.
    if count < 6 {
        Ok(Some(count))
    } else if count < 7 {
        Ok(None)
    } else {
        Err(())
    }
});
assert_eq!(&counter.collect::<Vec<_>>().unwrap(), &[1, 2, 3, 4, 5]);
```

### `_is_object_safe`

```rust
fn _is_object_safe(_: &dyn DoubleEndedFallibleIterator<Item = (), Error = ()>)
```

### `empty`

```rust
fn empty<T, E>() -> Empty<T, E>
```

Creates an iterator that yields nothing.

### `once`

```rust
fn once<T, E>(value: T) -> Once<T, E>
```

Creates an iterator that yields an element exactly once.

### `once_err`

```rust
fn once_err<T, E>(value: E) -> OnceErr<T, E>
```

Creates an iterator that fails with a predetermined error exactly once.

### `repeat`

```rust
fn repeat<T: Clone, E>(value: T) -> Repeat<T, E>
```

Creates an iterator that endlessly repeats a single element.

### `repeat_err`

```rust
fn repeat_err<T, E: Clone>(value: E) -> RepeatErr<T, E>
```

Creates an iterator that endlessly repeats a single error.

