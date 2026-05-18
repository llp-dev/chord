**futures_util > stream > try_stream**

# Module: stream::try_stream

## Contents

**Structs**

- [`ErrInto`](#errinto) - Stream for the [`err_into`](super::TryStreamExt::err_into) method.
- [`InspectErr`](#inspecterr) - Stream for the [`inspect_err`](super::TryStreamExt::inspect_err) method.
- [`InspectOk`](#inspectok) - Stream for the [`inspect_ok`](super::TryStreamExt::inspect_ok) method.
- [`MapErr`](#maperr) - Stream for the [`map_err`](super::TryStreamExt::map_err) method.
- [`MapOk`](#mapok) - Stream for the [`map_ok`](super::TryStreamExt::map_ok) method.

**Traits**

- [`TryStreamExt`](#trystreamext) - Adapters specific to `Result`-returning streams

---

## futures_util::stream::try_stream::ErrInto

*Struct*

Stream for the [`err_into`](super::TryStreamExt::err_into) method.

**Generic Parameters:**
- St
- E

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
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::stream::try_stream::InspectErr

*Struct*

Stream for the [`inspect_err`](super::TryStreamExt::inspect_err) method.

**Generic Parameters:**
- St
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



## futures_util::stream::try_stream::InspectOk

*Struct*

Stream for the [`inspect_ok`](super::TryStreamExt::inspect_ok) method.

**Generic Parameters:**
- St
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



## futures_util::stream::try_stream::MapErr

*Struct*

Stream for the [`map_err`](super::TryStreamExt::map_err) method.

**Generic Parameters:**
- St
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



## futures_util::stream::try_stream::MapOk

*Struct*

Stream for the [`map_ok`](super::TryStreamExt::map_ok) method.

**Generic Parameters:**
- St
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



## futures_util::stream::try_stream::TryStreamExt

*Trait*

Adapters specific to `Result`-returning streams

**Methods:**

- `err_into`: Wraps the current stream in a new stream which converts the error type
- `map_ok`: Wraps the current stream in a new stream which maps the success value
- `map_err`: Wraps the current stream in a new stream which maps the error value
- `and_then`: Chain on a computation for when a value is ready, passing the successful
- `or_else`: Chain on a computation for when an error happens, passing the
- `inspect_ok`: Do something with the success value of this stream, afterwards passing
- `inspect_err`: Do something with the error value of this stream, afterwards passing it on.
- `into_stream`: Wraps a [`TryStream`] into a type that implements
- `try_next`: Creates a future that attempts to resolve the next item in the stream.
- `try_for_each`: Attempts to run this stream to completion, executing the provided
- `try_skip_while`: Skip elements on this stream while the provided asynchronous predicate
- `try_take_while`: Take elements on this stream while the provided asynchronous predicate
- `try_for_each_concurrent`: Attempts to run this stream to completion, executing the provided asynchronous
- `try_collect`: Attempt to transform a stream into a collection,
- `try_chunks`: An adaptor for chunking up successful items of the stream inside a vector.
- `try_ready_chunks`: An adaptor for chunking up successful, ready items of the stream inside a vector.
- `try_filter`: Attempt to filter the values produced by this stream according to the
- `try_filter_map`: Attempt to filter the values produced by this stream while
- `try_flatten_unordered`: Flattens a stream of streams into just one continuous stream. Produced streams
- `try_flatten`: Flattens a stream of streams into just one continuous stream.
- `try_fold`: Attempt to execute an accumulating asynchronous computation over a
- `try_concat`: Attempt to concatenate all items of a stream into a single
- `try_buffer_unordered`: Attempt to execute several futures from a stream concurrently (unordered).
- `try_buffered`: Attempt to execute several futures from a stream concurrently.
- `try_poll_next_unpin`: A convenience method for calling [`TryStream::try_poll_next`] on [`Unpin`]
- `try_all`: Attempt to execute a predicate over an asynchronous stream and evaluate if all items
- `try_any`: Attempt to execute a predicate over an asynchronous stream and evaluate if any items



