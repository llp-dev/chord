**fallible_iterator**

# Module: fallible_iterator

## Contents

**Structs**

- [`Chain`](#chain) - An iterator which yields the elements of one iterator followed by another.
- [`Cloned`](#cloned) - An iterator which clones the elements of the underlying iterator.
- [`Convert`](#convert) - A fallible iterator that wraps a normal iterator over `Result`s.
- [`Cycle`](#cycle) - An iterator which cycles another endlessly.
- [`Empty`](#empty) - An iterator that yields nothing.
- [`Enumerate`](#enumerate) - An iterator that yields the iteration count as well as the values of the
- [`Filter`](#filter) - An iterator which uses a fallible predicate to determine which values of the
- [`FilterMap`](#filtermap) - An iterator which both filters and maps the values of the underlying
- [`FlatMap`](#flatmap) - An iterator which maps each element to another iterator, yielding those iterator's elements.
- [`Flatten`](#flatten) - An iterator which flattens an iterator of iterators, yielding those iterators' elements.
- [`FromFn`](#fromfn) - An iterator using a function to generate new values.
- [`Fuse`](#fuse) - An iterator that yields `Ok(None)` forever after the underlying iterator
- [`Inspect`](#inspect) - An iterator which passes each element to a closure before returning it.
- [`IntoFallible`](#intofallible) - A fallible iterator that wraps a normal iterator over `Result`s.
- [`Iterator`](#iterator) - A normal (non-fallible) iterator which wraps a fallible iterator.
- [`Map`](#map) - An iterator which applies a fallible transform to the elements of the
- [`MapErr`](#maperr) - An iterator which applies a transform to the errors of the underlying
- [`Once`](#once) - An iterator that yields something exactly once.
- [`OnceErr`](#onceerr) - An iterator that fails with a predetermined error exactly once.
- [`Peekable`](#peekable) - An iterator which can look at the next element without consuming it.
- [`Repeat`](#repeat) - An iterator that endlessly repeats a single element.
- [`RepeatErr`](#repeaterr) - An iterator that endlessly repeats a single error.
- [`Rev`](#rev) - An iterator which yields elements of the underlying iterator in reverse
- [`Scan`](#scan) - An iterator which applies a stateful closure.
- [`Skip`](#skip) - An iterator which skips initial elements.
- [`SkipWhile`](#skipwhile) - An iterator which skips initial elements based on a predicate.
- [`StepBy`](#stepby) - An iterator which steps through the elements of the underlying iterator by a certain amount.
- [`Take`](#take) - An iterator which yields a limited number of elements from the underlying
- [`TakeWhile`](#takewhile) - An iterator which yields elements based on a predicate.
- [`Unwrap`](#unwrap) - An iterator that unwraps every element yielded by the underlying
- [`Zip`](#zip) - An iterator that yields pairs of this iterator's and another iterator's

**Functions**

- [`convert`](#convert) - Converts an `Iterator<Item = Result<T, E>>` into a `FallibleIterator<Item = T, Error = E>`.
- [`empty`](#empty) - Creates an iterator that yields nothing.
- [`from_fn`](#from_fn) - Creates an iterator from a fallible function generating values.
- [`once`](#once) - Creates an iterator that yields an element exactly once.
- [`once_err`](#once_err) - Creates an iterator that fails with a predetermined error exactly once.
- [`repeat`](#repeat) - Creates an iterator that endlessly repeats a single element.
- [`repeat_err`](#repeat_err) - Creates an iterator that endlessly repeats a single error.

**Traits**

- [`DoubleEndedFallibleIterator`](#doubleendedfallibleiterator) - A fallible iterator able to yield elements from both ends.
- [`FallibleIterator`](#fallibleiterator) - An `Iterator`-like trait that allows for calculation of items to fail.
- [`IntoFallibleIterator`](#intofallibleiterator) - Conversion into a `FallibleIterator`.
- [`IteratorExt`](#iteratorext) - An extnsion-trait with set of useful methods to convert [`core::iter::Iterator`]

---

## fallible_iterator::Chain

*Struct*

An iterator which yields the elements of one iterator followed by another.

**Generic Parameters:**
- T
- U

**Trait Implementations:**

- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<<T as >::Item>, <T as >::Error>`
  - `fn try_rfold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<T as >::Item>, <T as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> Result<usize, <T as >::Error>`
  - `fn try_fold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
  - `fn find<F>(self: & mut Self, f: F) -> Result<Option<<T as >::Item>, <T as >::Error>`
  - `fn last(self: Self) -> Result<Option<<T as >::Item>, <T as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Chain<T, U>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::Cloned

*Struct*

An iterator which clones the elements of the underlying iterator.

**Generic Parameters:**
- I

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<T>, <I as >::Error>`
  - `fn try_rfold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<T>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
- **Clone**
  - `fn clone(self: &Self) -> Cloned<I>`



## fallible_iterator::Convert

*Struct*

A fallible iterator that wraps a normal iterator over `Result`s.

**Generic Parameters:**
- I

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<T>, E>`
  - `fn try_rfold<B, E2, F>(self: & mut Self, init: B, f: F) -> Result<B, E2>`
- **Clone**
  - `fn clone(self: &Self) -> Convert<I>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<T>, E>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<B, E2, F>(self: & mut Self, init: B, f: F) -> Result<B, E2>`



## fallible_iterator::Cycle

*Struct*

An iterator which cycles another endlessly.

**Generic Parameters:**
- I

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Cycle<I>`



## fallible_iterator::DoubleEndedFallibleIterator

*Trait*

A fallible iterator able to yield elements from both ends.

**Methods:**

- `next_back`: Advances the end of the iterator, returning the last value.
- `rfold`: Applies a function over the elements of the iterator in reverse order, producing a single final value.
- `try_rfold`: Applies a function over the elements of the iterator in reverse, producing a single final value.



## fallible_iterator::Empty

*Struct*

An iterator that yields nothing.

**Generic Parameters:**
- T
- E

**Tuple Struct**: `()`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Empty<T, E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<T>, E>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## fallible_iterator::Enumerate

*Struct*

An iterator that yields the iteration count as well as the values of the
underlying iterator.

**Generic Parameters:**
- I

**Trait Implementations:**

- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<(usize, <I as >::Item)>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> Result<usize, <I as >::Error>`
  - `fn nth(self: & mut Self, n: usize) -> Result<Option<(usize, <I as >::Item)>, <I as >::Error>`
  - `fn try_fold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
- **Clone**
  - `fn clone(self: &Self) -> Enumerate<I>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::FallibleIterator

*Trait*

An `Iterator`-like trait that allows for calculation of items to fail.

**Methods:**

- `Item`: The type being iterated over.
- `Error`: The error type.
- `next`: Advances the iterator and returns the next value.
- `size_hint`: Returns bounds on the remaining length of the iterator.
- `count`: Consumes the iterator, returning the number of remaining items.
- `last`: Returns the last element of the iterator.
- `nth`: Returns the `n`th element of the iterator.
- `step_by`: Returns an iterator starting at the same point, but stepping by the given amount at each iteration.
- `chain`: Returns an iterator which yields the elements of this iterator followed
- `zip`: Returns an iterator that yields pairs of this iterator's and another
- `map`: Returns an iterator which applies a fallible transform to the elements
- `for_each`: Calls a fallible closure on each element of an iterator.
- `filter`: Returns an iterator which uses a predicate to determine which values
- `filter_map`: Returns an iterator which both filters and maps. The closure may fail;
- `enumerate`: Returns an iterator which yields the current iteration count as well
- `peekable`: Returns an iterator that can peek at the next element without consuming
- `skip_while`: Returns an iterator that skips elements based on a predicate.
- `take_while`: Returns an iterator that yields elements based on a predicate.
- `skip`: Returns an iterator which skips the first `n` values of this iterator.
- `take`: Returns an iterator that yields only the first `n` values of this
- `scan`: Returns an iterator which applies a stateful map to values of this
- `flat_map`: Returns an iterator which maps this iterator's elements to iterators, yielding those iterators' values.
- `flatten`: Returns an iterator which flattens an iterator of iterators, yielding those iterators' values.
- `fuse`: Returns an iterator which yields this iterator's elements and ends after
- `inspect`: Returns an iterator which passes each element to a closure before returning it.
- `by_ref`: Borrow an iterator rather than consuming it.
- `collect`: Transforms the iterator into a collection.
- `partition`: Transforms the iterator into two collections, partitioning elements by a closure.
- `fold`: Applies a function over the elements of the iterator, producing a single
- `try_fold`: Applies a function over the elements of the iterator, producing a single final value.
- `all`: Determines if all elements of this iterator match a predicate.
- `any`: Determines if any element of this iterator matches a predicate.
- `find`: Returns the first element of the iterator that matches a predicate.
- `find_map`: Applies a function to the elements of the iterator, returning the first non-`None` result.
- `position`: Returns the position of the first element of this iterator that matches
- `max`: Returns the maximal element of the iterator.
- `max_by_key`: Returns the element of the iterator which gives the maximum value from
- `max_by`: Returns the element that gives the maximum value with respect to the function.
- `min`: Returns the minimal element of the iterator.
- `min_by_key`: Returns the element of the iterator which gives the minimum value from
- `min_by`: Returns the element that gives the minimum value with respect to the function.
- `rev`: Returns an iterator that yields this iterator's items in the opposite
- `unzip`: Converts an iterator of pairs into a pair of containers.
- `cloned`: Returns an iterator which clones all of its elements.
- `cycle`: Returns an iterator which repeats this iterator endlessly.
- `cmp`: Lexicographically compares the elements of this iterator to that of
- `partial_cmp`: Lexicographically compares the elements of this iterator to that of
- `eq`: Determines if the elements of this iterator are equal to those of
- `ne`: Determines if the elements of this iterator are not equal to those of
- `lt`: Determines if the elements of this iterator are lexicographically less
- `le`: Determines if the elements of this iterator are lexicographically less
- `gt`: Determines if the elements of this iterator are lexicographically
- `ge`: Determines if the elements of this iterator are lexicographically
- `iterator`: Returns a normal (non-fallible) iterator over `Result<Item, Error>`.
- `map_err`: Returns an iterator which applies a transform to the errors of the
- `unwrap`: Returns an iterator which unwraps all of its elements.



## fallible_iterator::Filter

*Struct*

An iterator which uses a fallible predicate to determine which values of the
underlying iterator should be yielded.

**Generic Parameters:**
- I
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Filter<I, F>`
- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn try_rfold<B, E, G>(self: & mut Self, init: B, f: G) -> Result<B, E>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<B, E, G>(self: & mut Self, init: B, f: G) -> Result<B, E>`



## fallible_iterator::FilterMap

*Struct*

An iterator which both filters and maps the values of the underlying
iterator.

**Generic Parameters:**
- I
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<B>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<C, E, G>(self: & mut Self, init: C, f: G) -> Result<C, E>`
- **Clone**
  - `fn clone(self: &Self) -> FilterMap<I, F>`
- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<B>, <I as >::Error>`
  - `fn try_rfold<C, E, G>(self: & mut Self, init: C, f: G) -> Result<C, E>`



## fallible_iterator::FlatMap

*Struct*

An iterator which maps each element to another iterator, yielding those iterator's elements.

**Generic Parameters:**
- I
- U
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FlatMap<I, U, F>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<U as >::Item>, <U as >::Error>`
  - `fn try_fold<B, E, G>(self: & mut Self, init: B, f: G) -> Result<B, E>`



## fallible_iterator::Flatten

*Struct*

An iterator which flattens an iterator of iterators, yielding those iterators' elements.

**Generic Parameters:**
- I

**Trait Implementations:**

- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`
  - `fn try_fold<B, E, G>(self: & mut Self, init: B, f: G) -> Result<B, E>`
- **Clone**
  - `fn clone(self: &Self) -> Flatten<I>`



## fallible_iterator::FromFn

*Struct*

An iterator using a function to generate new values.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FromFn<F>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<I>, E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::Fuse

*Struct*

An iterator that yields `Ok(None)` forever after the underlying iterator
yields `Ok(None)` once.

**Generic Parameters:**
- I

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Fuse<I>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> Result<usize, <I as >::Error>`
  - `fn last(self: Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn nth(self: & mut Self, n: usize) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn try_fold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`



## fallible_iterator::Inspect

*Struct*

An iterator which passes each element to a closure before returning it.

**Generic Parameters:**
- I
- F

**Trait Implementations:**

- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn try_rfold<B, E, G>(self: & mut Self, init: B, f: G) -> Result<B, E>`
- **Clone**
  - `fn clone(self: &Self) -> Inspect<I, F>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<B, E, G>(self: & mut Self, init: B, f: G) -> Result<B, E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::IntoFallible

*Struct*

A fallible iterator that wraps a normal iterator over `Result`s.

**Generic Parameters:**
- I

**Tuple Struct**: `()`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> IntoFallible<I>`
- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<T>, Infallible>`
  - `fn try_rfold<B, E2, F>(self: & mut Self, init: B, f: F) -> Result<B, E2>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<T>, <Self as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<B, E2, F>(self: & mut Self, init: B, f: F) -> Result<B, E2>`
- **From**
  - `fn from(value: I) -> Self`



## fallible_iterator::IntoFallibleIterator

*Trait*

Conversion into a `FallibleIterator`.

**Methods:**

- `Item`: The elements of the iterator.
- `Error`: The error value of the iterator.
- `IntoFallibleIter`: The iterator.
- `into_fallible_iter`: Creates a fallible iterator from a value.



## fallible_iterator::Iterator

*Struct*

A normal (non-fallible) iterator which wraps a fallible iterator.

**Generic Parameters:**
- I

**Tuple Struct**: `()`

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<Result<<I as >::Item, <I as >::Error>>`
- **Clone**
  - `fn clone(self: &Self) -> Iterator<I>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<Result<<I as >::Item, <I as >::Error>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::IteratorExt

*Trait*

An extnsion-trait with set of useful methods to convert [`core::iter::Iterator`]
into [`FallibleIterator`]

**Methods:**

- `transpose_into_fallible`: Convert an iterator of `Result`s into `FallibleIterator` by transposition
- `into_fallible`: Convert an iterator of anything into `FallibleIterator` by wrapping



## fallible_iterator::Map

*Struct*

An iterator which applies a fallible transform to the elements of the
underlying iterator.

**Generic Parameters:**
- T
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Map<T, F>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<B>, <T as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<C, E, G>(self: & mut Self, init: C, f: G) -> Result<C, E>`
- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<B>, <I as >::Error>`
  - `fn try_rfold<C, E, G>(self: & mut Self, init: C, f: G) -> Result<C, E>`



## fallible_iterator::MapErr

*Struct*

An iterator which applies a transform to the errors of the underlying
iterator.

**Generic Parameters:**
- I
- F

**Trait Implementations:**

- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<<I as >::Item>, B>`
  - `fn try_rfold<C, E, G>(self: & mut Self, init: C, f: G) -> Result<C, E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MapErr<I, F>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, B>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> Result<usize, B>`
  - `fn last(self: Self) -> Result<Option<<I as >::Item>, B>`
  - `fn nth(self: & mut Self, n: usize) -> Result<Option<<I as >::Item>, B>`
  - `fn try_fold<C, E, G>(self: & mut Self, init: C, f: G) -> Result<C, E>`



## fallible_iterator::Once

*Struct*

An iterator that yields something exactly once.

**Generic Parameters:**
- T
- E

**Tuple Struct**: `()`

**Trait Implementations:**

- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Once<T, E>`



## fallible_iterator::OnceErr

*Struct*

An iterator that fails with a predetermined error exactly once.

**Generic Parameters:**
- T
- E

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OnceErr<T, E>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## fallible_iterator::Peekable

*Struct*

An iterator which can look at the next element without consuming it.

**Generic Parameters:**
- I

**Methods:**

- `fn peek(self: & mut Self) -> Result<Option<&<I as >::Item>, <I as >::Error>` - Returns a reference to the next value without advancing the iterator.
- `fn next_if<impl Fn(&I::Item) -> bool>(self: & mut Self, f: impl Trait) -> Result<Option<<I as >::Item>, <I as >::Error>` - Consume and return the next value of this iterator if a condition is true.
- `fn next_if_eq<T>(self: & mut Self, expected: &T) -> Result<Option<<I as >::Item>, <I as >::Error>` - Consume and return the next item if it is equal to `expected`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn try_fold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
- **Clone**
  - `fn clone(self: &Self) -> Peekable<I>`



## fallible_iterator::Repeat

*Struct*

An iterator that endlessly repeats a single element.

**Generic Parameters:**
- T
- E

**Tuple Struct**: `()`

**Trait Implementations:**

- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Repeat<T, E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::RepeatErr

*Struct*

An iterator that endlessly repeats a single error.

**Generic Parameters:**
- T
- E

**Tuple Struct**: `()`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RepeatErr<T, E>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<Self as >::Item>, <Self as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::Rev

*Struct*

An iterator which yields elements of the underlying iterator in reverse
order.

**Generic Parameters:**
- I

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedFallibleIterator**
  - `fn next_back(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn try_rfold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> Result<usize, <I as >::Error>`
  - `fn try_fold<B, E, F>(self: & mut Self, init: B, f: F) -> Result<B, E>`
- **Clone**
  - `fn clone(self: &Self) -> Rev<I>`



## fallible_iterator::Scan

*Struct*

An iterator which applies a stateful closure.

**Generic Parameters:**
- I
- St
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<B>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Scan<I, St, F>`



## fallible_iterator::Skip

*Struct*

An iterator which skips initial elements.

**Generic Parameters:**
- I

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Skip<I>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## fallible_iterator::SkipWhile

*Struct*

An iterator which skips initial elements based on a predicate.

**Generic Parameters:**
- I
- P

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SkipWhile<I, P>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::StepBy

*Struct*

An iterator which steps through the elements of the underlying iterator by a certain amount.

**Generic Parameters:**
- I

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> StepBy<I>`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::Take

*Struct*

An iterator which yields a limited number of elements from the underlying
iterator.

**Generic Parameters:**
- I

**Trait Implementations:**

- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> Take<I>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::TakeWhile

*Struct*

An iterator which yields elements based on a predicate.

**Generic Parameters:**
- I
- P

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<<I as >::Item>, <I as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Clone**
  - `fn clone(self: &Self) -> TakeWhile<I, P>`



## fallible_iterator::Unwrap

*Struct*

An iterator that unwraps every element yielded by the underlying
FallibleIterator

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Unwrap<T>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<T as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## fallible_iterator::Zip

*Struct*

An iterator that yields pairs of this iterator's and another iterator's
values.

**Generic Parameters:**
- T
- U

**Tuple Struct**: `()`

**Trait Implementations:**

- **FallibleIterator**
  - `fn next(self: & mut Self) -> Result<Option<(<T as >::Item, <U as >::Item)>, <T as >::Error>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Zip<T, U>`



## fallible_iterator::convert

*Function*

Converts an `Iterator<Item = Result<T, E>>` into a `FallibleIterator<Item = T, Error = E>`.

```rust
fn convert<T, E, I>(it: I) -> Convert<I>
```



## fallible_iterator::empty

*Function*

Creates an iterator that yields nothing.

```rust
fn empty<T, E>() -> Empty<T, E>
```



## fallible_iterator::from_fn

*Function*

Creates an iterator from a fallible function generating values.

```
# use fallible_iterator::{from_fn, FallibleIterator};
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

```rust
fn from_fn<I, E, F>(fun: F) -> FromFn<F>
```



## fallible_iterator::once

*Function*

Creates an iterator that yields an element exactly once.

```rust
fn once<T, E>(value: T) -> Once<T, E>
```



## fallible_iterator::once_err

*Function*

Creates an iterator that fails with a predetermined error exactly once.

```rust
fn once_err<T, E>(value: E) -> OnceErr<T, E>
```



## fallible_iterator::repeat

*Function*

Creates an iterator that endlessly repeats a single element.

```rust
fn repeat<T, E>(value: T) -> Repeat<T, E>
```



## fallible_iterator::repeat_err

*Function*

Creates an iterator that endlessly repeats a single error.

```rust
fn repeat_err<T, E>(value: E) -> RepeatErr<T, E>
```



