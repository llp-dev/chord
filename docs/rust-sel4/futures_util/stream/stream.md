**futures_util > stream > stream**

# Module: stream::stream

## Contents

**Structs**

- [`FlatMap`](#flatmap) - Stream for the [`flat_map`](StreamExt::flat_map) method.
- [`FlatMapUnordered`](#flatmapunordered) - Stream for the [`flat_map_unordered`](StreamExt::flat_map_unordered) method.
- [`Flatten`](#flatten) - Stream for the [`flatten`](StreamExt::flatten) method.
- [`Forward`](#forward) - Future for the [`forward`](super::StreamExt::forward) method.
- [`Inspect`](#inspect) - Stream for the [`inspect`](StreamExt::inspect) method.

**Traits**

- [`StreamExt`](#streamext) - An extension trait for `Stream`s that provides a variety of convenient

---

## futures_util::stream::stream::FlatMap

*Struct*

Stream for the [`flat_map`](StreamExt::flat_map) method.

**Generic Parameters:**
- St
- U
- F

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::stream::stream::FlatMapUnordered

*Struct*

Stream for the [`flat_map_unordered`](StreamExt::flat_map_unordered) method.

**Generic Parameters:**
- St
- U
- F

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## futures_util::stream::stream::Flatten

*Struct*

Stream for the [`flatten`](StreamExt::flatten) method.

**Generic Parameters:**
- St

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`



## futures_util::stream::stream::Forward

*Struct*

Future for the [`forward`](super::StreamExt::forward) method.

**Generic Parameters:**
- St
- Si

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<<Self as >::Output>`



## futures_util::stream::stream::Inspect

*Struct*

Stream for the [`inspect`](StreamExt::inspect) method.

**Generic Parameters:**
- St
- F

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`



## futures_util::stream::stream::StreamExt

*Trait*

An extension trait for `Stream`s that provides a variety of convenient
combinator functions.

**Methods:**

- `next`: Creates a future that resolves to the next item in the stream.
- `into_future`: Converts this stream into a future of `(next_item, tail_of_stream)`.
- `map`: Maps this stream's items to a different type, returning a new stream of
- `enumerate`: Creates a stream which gives the current iteration count as well as
- `filter`: Filters the values produced by this stream according to the provided
- `filter_map`: Filters the values produced by this stream while simultaneously mapping
- `then`: Computes from this stream's items new items of a different type using
- `collect`: Transforms a stream into a collection, returning a
- `unzip`: Converts a stream of pairs into a future, which
- `concat`: Concatenate all items of a stream into a single extendable
- `count`: Drives the stream to completion, counting the number of items.
- `cycle`: Repeats a stream endlessly.
- `fold`: Execute an accumulating asynchronous computation over a stream,
- `any`: Execute predicate over asynchronous stream, and return `true` if any element in stream satisfied a predicate.
- `all`: Execute predicate over asynchronous stream, and return `true` if all element in stream satisfied a predicate.
- `flatten`: Flattens a stream of streams into just one continuous stream.
- `flatten_unordered`: Flattens a stream of streams into just one continuous stream. Polls
- `flat_map`: Maps a stream like [`StreamExt::map`] but flattens nested `Stream`s.
- `flat_map_unordered`: Maps a stream like [`StreamExt::map`] but flattens nested `Stream`s
- `scan`: Combinator similar to [`StreamExt::fold`] that holds internal state
- `skip_while`: Skip elements on this stream while the provided asynchronous predicate
- `take_while`: Take elements from this stream while the provided asynchronous predicate
- `take_until`: Take elements from this stream until the provided future resolves.
- `for_each`: Runs this stream to completion, executing the provided asynchronous
- `for_each_concurrent`: Runs this stream to completion, executing the provided asynchronous
- `take`: Creates a new stream of at most `n` items of the underlying stream.
- `skip`: Creates a new stream which skips `n` items of the underlying stream.
- `fuse`: Fuse a stream such that [`poll_next`](Stream::poll_next) will never
- `by_ref`: Borrows a stream, rather than consuming it.
- `boxed`: Wrap the stream in a Box, pinning it.
- `boxed_local`: Wrap the stream in a Box, pinning it.
- `buffered`: An adaptor for creating a buffered list of pending futures.
- `buffer_unordered`: An adaptor for creating a buffered list of pending futures (unordered).
- `zip`: An adapter for zipping two streams together.
- `chain`: Adapter for chaining two streams.
- `peekable`: Creates a new stream which exposes a `peek` method.
- `chunks`: An adaptor for chunking up items of the stream inside a vector.
- `ready_chunks`: An adaptor for chunking up ready items of the stream inside a vector.
- `forward`: A future that completes after the given stream has been fully processed
- `split`: Splits this `Stream + Sink` object into separate `Sink` and `Stream`
- `inspect`: Do something with each item of this stream, afterwards passing it on.
- `left_stream`: Wrap this stream in an `Either` stream, making it the left-hand variant
- `right_stream`: Wrap this stream in an `Either` stream, making it the right-hand variant
- `poll_next_unpin`: A convenience method for calling [`Stream::poll_next`] on [`Unpin`]
- `select_next_some`: Returns a [`Future`] that resolves when the next item in this stream is



