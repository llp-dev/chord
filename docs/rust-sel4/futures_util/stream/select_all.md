**futures_util > stream > select_all**

# Module: stream::select_all

## Contents

**Structs**

- [`IntoIter`](#intoiter) - Owned iterator over all streams in the unordered set.
- [`Iter`](#iter) - Immutable iterator over all streams in the unordered set.
- [`IterMut`](#itermut) - Mutable iterator over all streams in the unordered set.
- [`SelectAll`](#selectall) - An unbounded set of streams

**Functions**

- [`select_all`](#select_all) - Convert a list of streams into a `Stream` of results from the streams.

---

## futures_util::stream::select_all::IntoIter

*Struct*

Owned iterator over all streams in the unordered set.

**Generic Parameters:**
- St

**Tuple Struct**: `()`

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::select_all::Iter

*Struct*

Immutable iterator over all streams in the unordered set.

**Generic Parameters:**
- 'a
- St

**Tuple Struct**: `()`

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## futures_util::stream::select_all::IterMut

*Struct*

Mutable iterator over all streams in the unordered set.

**Generic Parameters:**
- 'a
- St

**Tuple Struct**: `()`

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::select_all::SelectAll

*Struct*

An unbounded set of streams

This "combinator" provides the ability to maintain a set of streams
and drive them all to completion.

Streams are pushed into this set and their realized values are
yielded as they become ready. Streams will only be polled when they
generate notifications. This allows to coordinate a large number of streams.

Note that you can create a ready-made `SelectAll` via the
`select_all` function in the `stream` module, or you can start with an
empty set with the `SelectAll::new` constructor.

**Generic Parameters:**
- St

**Methods:**

- `fn new() -> Self` - Constructs a new, empty `SelectAll`
- `fn len(self: &Self) -> usize` - Returns the number of streams contained in the set.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the set contains no streams
- `fn push(self: & mut Self, stream: St)` - Push a stream into the set.
- `fn iter(self: &Self) -> Iter<St>` - Returns an iterator that allows inspecting each stream in the set.
- `fn iter_mut(self: & mut Self) -> IterMut<St>` - Returns an iterator that allows modifying each stream in the set.
- `fn clear(self: & mut Self)` - Clears the set, removing all streams.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`



## futures_util::stream::select_all::select_all

*Function*

Convert a list of streams into a `Stream` of results from the streams.

This essentially takes a list of streams (e.g. a vector, an iterator, etc.)
and bundles them together into a single stream.
The stream will yield items as they become available on the underlying
streams internally, in the order they become available.

Note that the returned set can also be used to dynamically push more
streams into the set as they become available.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

```rust
fn select_all<I>(streams: I) -> SelectAll<<I as >::Item>
```



