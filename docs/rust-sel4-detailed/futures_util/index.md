# Crate `futures_util`

Combinators and utilities for working with `Future`s, `Stream`s, `Sink`s,
and the `AsyncRead` and `AsyncWrite` traits.

## Contents

- [Modules](#modules)
  - [`async_await`](#async-await)
  - [`future`](#future)
  - [`stream`](#stream)
  - [`sink`](#sink)
  - [`task`](#task)
  - [`never`](#never)
  - [`lock`](#lock)
  - [`abortable`](#abortable)
  - [`fns`](#fns)
  - [`macros`](#macros)
  - [`unfold_state`](#unfold-state)
- [Structs](#structs)
  - [`Sink`](#sink)
- [Traits](#traits)
  - [`FutureExt`](#futureext)
  - [`TryFutureExt`](#tryfutureext)
  - [`StreamExt`](#streamext)
  - [`TryStreamExt`](#trystreamext)
  - [`SinkExt`](#sinkext)
- [Functions](#functions)
  - [`TryStream`](#trystream)
- [Macros](#macros)
  - [`delegate_sink!`](#delegate-sink)
  - [`delegate_future!`](#delegate-future)
  - [`delegate_stream!`](#delegate-stream)
  - [`delegate_access_inner!`](#delegate-access-inner)
  - [`delegate_all!`](#delegate-all)
  - [`poll!`](#poll)
  - [`pending!`](#pending)
  - [`pin_mut!`](#pin-mut)
  - [`join!`](#join)
  - [`try_join!`](#try-join)
  - [`select_biased!`](#select-biased)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`async_await`](#async-await) | mod | Await |
| [`future`](#future) | mod | Asynchronous values. |
| [`stream`](#stream) | mod | Asynchronous streams. |
| [`sink`](#sink) | mod | Asynchronous sinks. |
| [`task`](#task) | mod | Tools for working with tasks. |
| [`never`](#never) | mod | This module contains the `Never` type. |
| [`lock`](#lock) | mod | Futures-powered synchronization primitives. |
| [`abortable`](#abortable) | mod |  |
| [`fns`](#fns) | mod |  |
| [`macros`](#macros) | mod |  |
| [`unfold_state`](#unfold-state) | mod |  |
| [`Sink`](#sink) | struct |  |
| [`FutureExt`](#futureext) | trait |  |
| [`TryFutureExt`](#tryfutureext) | trait |  |
| [`StreamExt`](#streamext) | trait |  |
| [`TryStreamExt`](#trystreamext) | trait |  |
| [`SinkExt`](#sinkext) | trait |  |
| [`TryStream`](#trystream) | fn |  |
| [`delegate_sink!`](#delegate-sink) | macro |  |
| [`delegate_future!`](#delegate-future) | macro |  |
| [`delegate_stream!`](#delegate-stream) | macro |  |
| [`delegate_access_inner!`](#delegate-access-inner) | macro |  |
| [`delegate_all!`](#delegate-all) | macro |  |
| [`poll!`](#poll) | macro | A macro which returns the result of polling a future once within the current `async` context. |
| [`pending!`](#pending) | macro | A macro which yields to the event loop once. |
| [`pin_mut!`](#pin-mut) | macro | Pins a value on the stack. |
| [`join!`](#join) | macro | Polls multiple futures simultaneously, returning a tuple of all results once complete. |
| [`try_join!`](#try-join) | macro | Polls multiple futures simultaneously, resolving to a [`Result`] containing either a tuple of the successful outputs or an error. |
| [`select_biased!`](#select-biased) | macro | Polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. |

## Modules

- [`async_await`](async_await/index.md) — Await
- [`future`](future/index.md) — Asynchronous values.
- [`stream`](stream/index.md) — Asynchronous streams.
- [`sink`](sink/index.md) — Asynchronous sinks.
- [`task`](task/index.md) — Tools for working with tasks.
- [`never`](never/index.md) — This module contains the `Never` type.
- [`lock`](lock/index.md) — Futures-powered synchronization primitives.
- [`abortable`](abortable/index.md)
- [`fns`](fns/index.md)
- [`macros`](macros/index.md)
- [`unfold_state`](unfold_state/index.md)

## Structs

### `Sink<R: gimli::Reader>`

```rust
struct Sink<R: gimli::Reader> {
    pub name: R,
    pub language: Option<gimli::DwLang>,
}
```

*Re-exported from `addr2line`*

A function name.

#### Fields

- **`name`**: `R`

  The name of the function.

- **`language`**: `Option<gimli::DwLang>`

  The language of the compilation unit containing this function.

#### Implementations

- <span id="functionname-raw-name"></span>`fn raw_name(&self) -> Result<Cow<'_, str>, gimli::Error>`

  The raw name of this function before demangling.

- <span id="functionname-demangle"></span>`fn demangle(&self) -> Result<Cow<'_, str>, gimli::Error>`

  The name of this function after demangling (if applicable).

## Traits

### `FutureExt`

```rust
trait FutureExt: Future { ... }
```

An extension trait for `Future`s that provides a variety of convenient
adapters.

#### Provided Methods

- `fn map<U, F>(self, f: F) -> Map<Self, F>`

  Map this future's output to a different type, returning a new future of

- `fn map_into<U>(self) -> MapInto<Self, U>`

  Map this future's output to a different type, returning a new future of

- `fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>`

  Chain on a computation for when a future finished, passing the result of

- `fn left_future<B>(self) -> Either<Self, B>`

  Wrap this future in an `Either` future, making it the left-hand variant

- `fn right_future<A>(self) -> Either<A, Self>`

  Wrap this future in an `Either` future, making it the right-hand variant

- `fn into_stream(self) -> IntoStream<Self>`

  Convert this future into a single element stream.

- `fn flatten(self) -> Flatten<Self>`

  Flatten the execution of this future when the output of this

- `fn flatten_stream(self) -> FlattenStream<Self>`

  Flatten the execution of this future when the successful result of this

- `fn fuse(self) -> Fuse<Self>`

  Fuse a future such that `poll` will never again be called once it has

- `fn inspect<F>(self, f: F) -> Inspect<Self, F>`

  Do something with the output of a future before passing it on.

- `fn boxed<'a>(self) -> BoxFuture<'a, <Self as >::Output>`

  Wrap the future in a Box, pinning it.

- `fn boxed_local<'a>(self) -> LocalBoxFuture<'a, <Self as >::Output>`

  Wrap the future in a Box, pinning it.

- `fn unit_error(self) -> UnitError<Self>`

  Turns a [`Future<Output = T>`](Future) into a

- `fn never_error(self) -> NeverError<Self>`

  Turns a [`Future<Output = T>`](Future) into a

- `fn poll_unpin(&mut self, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

  A convenience for calling `Future::poll` on `Unpin` future types.

- `fn now_or_never(self) -> Option<<Self as >::Output>`

  Evaluates and consumes the future, returning the resulting output if

#### Implementors

- `T`

### `TryFutureExt`

```rust
trait TryFutureExt: TryFuture { ... }
```

Adapters specific to [`Result`](../sel4/error/index.md)-returning futures

#### Provided Methods

- `fn flatten_sink<Item>(self) -> FlattenSink<Self, <Self as >::Ok>`

  Flattens the execution of this future when the successful result of this

- `fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>`

  Maps this future's success value to a different value.

- `fn map_ok_or_else<T, E, F>(self, e: E, f: F) -> MapOkOrElse<Self, F, E>`

  Maps this future's success value to a different value, and permits for error handling resulting in the same type.

- `fn map_err<E, F>(self, f: F) -> MapErr<Self, F>`

  Maps this future's error value to a different value.

- `fn err_into<E>(self) -> ErrInto<Self, E>`

  Maps this future's [`Error`](TryFuture::Error) to a new error type

- `fn ok_into<U>(self) -> OkInto<Self, U>`

  Maps this future's [`Ok`](TryFuture::Ok) to a new type

- `fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>`

  Executes another future after this one resolves successfully. The

- `fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>`

  Executes another future if this one resolves to an error. The

- `fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>`

  Do something with the success value of a future before passing it on.

- `fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>`

  Do something with the error value of a future before passing it on.

- `fn try_flatten(self) -> TryFlatten<Self, <Self as >::Ok>`

  Flatten the execution of this future when the successful result of this

- `fn try_flatten_stream(self) -> TryFlattenStream<Self>`

  Flatten the execution of this future when the successful result of this

- `fn unwrap_or_else<F>(self, f: F) -> UnwrapOrElse<Self, F>`

  Unwraps this future's output, producing a future with this future's

- `fn into_future(self) -> IntoFuture<Self>`

  Wraps a [`TryFuture`](future/index.md) into a type that implements

- `fn try_poll_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Result<<Self as >::Ok, <Self as >::Error>>`

  A convenience method for calling `TryFuture::try_poll` on `Unpin`

#### Implementors

- `Fut`

### `StreamExt`

```rust
trait StreamExt: Stream { ... }
```

An extension trait for `Stream`s that provides a variety of convenient
combinator functions.

#### Provided Methods

- `fn next(&mut self) -> Next<'_, Self>`

  Creates a future that resolves to the next item in the stream.

- `fn into_future(self) -> StreamFuture<Self>`

  Converts this stream into a future of `(next_item, tail_of_stream)`.

- `fn map<T, F>(self, f: F) -> Map<Self, F>`

  Maps this stream's items to a different type, returning a new stream of

- `fn enumerate(self) -> Enumerate<Self>`

  Creates a stream which gives the current iteration count as well as

- `fn filter<Fut, F>(self, f: F) -> Filter<Self, Fut, F>`

  Filters the values produced by this stream according to the provided

- `fn filter_map<Fut, T, F>(self, f: F) -> FilterMap<Self, Fut, F>`

  Filters the values produced by this stream while simultaneously mapping

- `fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>`

  Computes from this stream's items new items of a different type using

- `fn collect<C: Default + Extend<<Self as >::Item>>(self) -> Collect<Self, C>`

  Transforms a stream into a collection, returning a

- `fn unzip<A, B, FromA, FromB>(self) -> Unzip<Self, FromA, FromB>`

  Converts a stream of pairs into a future, which

- `fn concat(self) -> Concat<Self>`

  Concatenate all items of a stream into a single extendable

- `fn count(self) -> Count<Self>`

  Drives the stream to completion, counting the number of items.

- `fn cycle(self) -> Cycle<Self>`

  Repeats a stream endlessly.

- `fn fold<T, Fut, F>(self, init: T, f: F) -> Fold<Self, Fut, T, F>`

  Execute an accumulating asynchronous computation over a stream,

- `fn any<Fut, F>(self, f: F) -> Any<Self, Fut, F>`

  Execute predicate over asynchronous stream, and return `true` if any element in stream satisfied a predicate.

- `fn all<Fut, F>(self, f: F) -> All<Self, Fut, F>`

  Execute predicate over asynchronous stream, and return `true` if all element in stream satisfied a predicate.

- `fn flatten(self) -> Flatten<Self>`

  Flattens a stream of streams into just one continuous stream.

- `fn flatten_unordered(self, limit: impl Into<Option<usize>>) -> FlattenUnordered<Self>`

  Flattens a stream of streams into just one continuous stream. Polls

- `fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>`

  Maps a stream like `StreamExt::map` but flattens nested `Stream`s.

- `fn flat_map_unordered<U, F>(self, limit: impl Into<Option<usize>>, f: F) -> FlatMapUnordered<Self, U, F>`

  Maps a stream like `StreamExt::map` but flattens nested `Stream`s

- `fn scan<S, B, Fut, F>(self, initial_state: S, f: F) -> Scan<Self, S, Fut, F>`

  Combinator similar to `StreamExt::fold` that holds internal state

- `fn skip_while<Fut, F>(self, f: F) -> SkipWhile<Self, Fut, F>`

  Skip elements on this stream while the provided asynchronous predicate

- `fn take_while<Fut, F>(self, f: F) -> TakeWhile<Self, Fut, F>`

  Take elements from this stream while the provided asynchronous predicate

- `fn take_until<Fut>(self, fut: Fut) -> TakeUntil<Self, Fut>`

  Take elements from this stream until the provided future resolves.

- `fn for_each<Fut, F>(self, f: F) -> ForEach<Self, Fut, F>`

  Runs this stream to completion, executing the provided asynchronous

- `fn for_each_concurrent<Fut, F>(self, limit: impl Into<Option<usize>>, f: F) -> ForEachConcurrent<Self, Fut, F>`

  Runs this stream to completion, executing the provided asynchronous

- `fn take(self, n: usize) -> Take<Self>`

  Creates a new stream of at most `n` items of the underlying stream.

- `fn skip(self, n: usize) -> Skip<Self>`

  Creates a new stream which skips `n` items of the underlying stream.

- `fn fuse(self) -> Fuse<Self>`

  Fuse a stream such that [`poll_next`](Stream::poll_next) will never

- `fn by_ref(&mut self) -> &mut Self`

  Borrows a stream, rather than consuming it.

- `fn boxed<'a>(self) -> BoxStream<'a, <Self as >::Item>`

  Wrap the stream in a Box, pinning it.

- `fn boxed_local<'a>(self) -> LocalBoxStream<'a, <Self as >::Item>`

  Wrap the stream in a Box, pinning it.

- `fn buffered(self, n: usize) -> Buffered<Self>`

  An adaptor for creating a buffered list of pending futures.

- `fn buffer_unordered(self, n: usize) -> BufferUnordered<Self>`

  An adaptor for creating a buffered list of pending futures (unordered).

- `fn zip<St>(self, other: St) -> Zip<Self, St>`

  An adapter for zipping two streams together.

- `fn chain<St>(self, other: St) -> Chain<Self, St>`

  Adapter for chaining two streams.

- `fn peekable(self) -> Peekable<Self>`

  Creates a new stream which exposes a `peek` method.

- `fn chunks(self, capacity: usize) -> Chunks<Self>`

  An adaptor for chunking up items of the stream inside a vector.

- `fn ready_chunks(self, capacity: usize) -> ReadyChunks<Self>`

  An adaptor for chunking up ready items of the stream inside a vector.

- `fn forward<S>(self, sink: S) -> Forward<Self, S>`

  A future that completes after the given stream has been fully processed

- `fn split<Item>(self) -> (SplitSink<Self, Item>, SplitStream<Self>)`

  Splits this `Stream + Sink` object into separate `Sink` and `Stream`

- `fn inspect<F>(self, f: F) -> Inspect<Self, F>`

  Do something with each item of this stream, afterwards passing it on.

- `fn left_stream<B>(self) -> Either<Self, B>`

  Wrap this stream in an `Either` stream, making it the left-hand variant

- `fn right_stream<B>(self) -> Either<B, Self>`

  Wrap this stream in an `Either` stream, making it the right-hand variant

- `fn poll_next_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>`

  A convenience method for calling `Stream::poll_next` on `Unpin`

- `fn select_next_some(&mut self) -> SelectNextSome<'_, Self>`

  Returns a [`Future`](future/index.md) that resolves when the next item in this stream is

#### Implementors

- `T`

### `TryStreamExt`

```rust
trait TryStreamExt: TryStream { ... }
```

Adapters specific to `Result`-returning streams

#### Provided Methods

- `fn err_into<E>(self) -> ErrInto<Self, E>`

  Wraps the current stream in a new stream which converts the error type

- `fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>`

  Wraps the current stream in a new stream which maps the success value

- `fn map_err<E, F>(self, f: F) -> MapErr<Self, F>`

  Wraps the current stream in a new stream which maps the error value

- `fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>`

  Chain on a computation for when a value is ready, passing the successful

- `fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>`

  Chain on a computation for when an error happens, passing the

- `fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>`

  Do something with the success value of this stream, afterwards passing

- `fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>`

  Do something with the error value of this stream, afterwards passing it on.

- `fn into_stream(self) -> IntoStream<Self>`

  Wraps a [`TryStream`](stream/index.md) into a type that implements

- `fn try_next(&mut self) -> TryNext<'_, Self>`

  Creates a future that attempts to resolve the next item in the stream.

- `fn try_for_each<Fut, F>(self, f: F) -> TryForEach<Self, Fut, F>`

  Attempts to run this stream to completion, executing the provided

- `fn try_skip_while<Fut, F>(self, f: F) -> TrySkipWhile<Self, Fut, F>`

  Skip elements on this stream while the provided asynchronous predicate

- `fn try_take_while<Fut, F>(self, f: F) -> TryTakeWhile<Self, Fut, F>`

  Take elements on this stream while the provided asynchronous predicate

- `fn try_for_each_concurrent<Fut, F>(self, limit: impl Into<Option<usize>>, f: F) -> TryForEachConcurrent<Self, Fut, F>`

  Attempts to run this stream to completion, executing the provided asynchronous

- `fn try_collect<C: Default + Extend<<Self as >::Ok>>(self) -> TryCollect<Self, C>`

  Attempt to transform a stream into a collection,

- `fn try_chunks(self, capacity: usize) -> TryChunks<Self>`

  An adaptor for chunking up successful items of the stream inside a vector.

- `fn try_ready_chunks(self, capacity: usize) -> TryReadyChunks<Self>`

  An adaptor for chunking up successful, ready items of the stream inside a vector.

- `fn try_filter<Fut, F>(self, f: F) -> TryFilter<Self, Fut, F>`

  Attempt to filter the values produced by this stream according to the

- `fn try_filter_map<Fut, F, T>(self, f: F) -> TryFilterMap<Self, Fut, F>`

  Attempt to filter the values produced by this stream while

- `fn try_flatten_unordered(self, limit: impl Into<Option<usize>>) -> TryFlattenUnordered<Self>`

  Flattens a stream of streams into just one continuous stream. Produced streams

- `fn try_flatten(self) -> TryFlatten<Self>`

  Flattens a stream of streams into just one continuous stream.

- `fn try_fold<T, Fut, F>(self, init: T, f: F) -> TryFold<Self, Fut, T, F>`

  Attempt to execute an accumulating asynchronous computation over a

- `fn try_concat(self) -> TryConcat<Self>`

  Attempt to concatenate all items of a stream into a single

- `fn try_buffer_unordered(self, n: usize) -> TryBufferUnordered<Self>`

  Attempt to execute several futures from a stream concurrently (unordered).

- `fn try_buffered(self, n: usize) -> TryBuffered<Self>`

  Attempt to execute several futures from a stream concurrently.

- `fn try_poll_next_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<<Self as >::Ok, <Self as >::Error>>>`

  A convenience method for calling `TryStream::try_poll_next` on `Unpin`

- `fn try_all<Fut, F>(self, f: F) -> TryAll<Self, Fut, F>`

  Attempt to execute a predicate over an asynchronous stream and evaluate if all items

- `fn try_any<Fut, F>(self, f: F) -> TryAny<Self, Fut, F>`

  Attempt to execute a predicate over an asynchronous stream and evaluate if any items

#### Implementors

- `S`

### `SinkExt<Item>`

```rust
trait SinkExt<Item>: Sink<Item> { ... }
```

An extension trait for `Sink`s that provides a variety of convenient
combinator functions.

#### Provided Methods

- `fn with<U, Fut, F, E>(self, f: F) -> With<Self, Item, U, Fut, F>`

  Composes a function *in front of* the sink.

- `fn with_flat_map<U, St, F>(self, f: F) -> WithFlatMap<Self, Item, U, St, F>`

  Composes a function *in front of* the sink.

- `fn sink_map_err<E, F>(self, f: F) -> SinkMapErr<Self, F>`

  Transforms the error returned by the sink.

- `fn sink_err_into<E>(self) -> err_into::SinkErrInto<Self, Item, E>`

  Map this sink's error to a different error type using the `Into` trait.

- `fn buffer(self, capacity: usize) -> Buffer<Self, Item>`

  Adds a fixed-size buffer to the current sink.

- `fn close(&mut self) -> Close<'_, Self, Item>`

  Close the sink.

- `fn fanout<Si>(self, other: Si) -> Fanout<Self, Si>`

  Fanout items to multiple sinks.

- `fn flush(&mut self) -> Flush<'_, Self, Item>`

  Flush the sink, processing all pending items.

- `fn send(&mut self, item: Item) -> Send<'_, Self, Item>`

  A future that completes after the given item has been fully processed

- `fn feed(&mut self, item: Item) -> Feed<'_, Self, Item>`

  A future that completes after the given item has been received

- `fn send_all<'a, St>(self: &'a mut Self, stream: &'a mut St) -> SendAll<'a, Self, St>`

  A future that completes after the given stream has been fully processed

- `fn left_sink<Si2>(self) -> Either<Self, Si2>`

  Wrap this sink in an `Either` sink, making it the left-hand variant

- `fn right_sink<Si1>(self) -> Either<Si1, Self>`

  Wrap this stream in an `Either` stream, making it the right-hand variant

- `fn poll_ready_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>`

  A convenience method for calling `Sink::poll_ready` on `Unpin`

- `fn start_send_unpin(&mut self, item: Item) -> Result<(), <Self as >::Error>`

  A convenience method for calling `Sink::start_send` on `Unpin`

- `fn poll_flush_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>`

  A convenience method for calling `Sink::poll_flush` on `Unpin`

- `fn poll_close_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>`

  A convenience method for calling `Sink::poll_close` on `Unpin`

#### Implementors

- `T`

## Functions

### `TryStream`

```rust
fn TryStream(&self) -> U32X4
```

## Macros

### `delegate_sink!`

### `delegate_future!`

### `delegate_stream!`

### `delegate_access_inner!`

### `delegate_all!`

### `poll!`

A macro which returns the result of polling a future once within the
current `async` context.

This macro is only usable inside of `async` functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

If you need the result of polling a [`Stream`](crate::stream::Stream),
you can use this macro with the [`next`](crate::stream::StreamExt::next) method:
`poll!(stream.next())`.

### `pending!`

A macro which yields to the event loop once.

This is equivalent to returning [`Poll::Pending`](futures_core::task::Poll)
from a [`Future::poll`](futures_core::future::Future::poll) implementation.
Similarly, when using this macro, it must be ensured that [`wake`](std::task::Waker::wake)
is called somewhere when further progress can be made.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

### `pin_mut!`

Pins a value on the stack.

Can safely pin values that are not `Unpin` by taking ownership.

**Note:** Since Rust 1.68, this macro is soft-deprecated in favor of
[`pin!`](https://doc.rust-lang.org/std/pin/macro.pin.html) macro
in the standard library.

# Example

```rust
use futures_util::pin_mut;
use core::pin::Pin;
struct Foo {}
let foo = Foo { /* ... */ };
pin_mut!(foo);
let _: Pin<&mut Foo> = foo;
```

### `join!`

Polls multiple futures simultaneously, returning a tuple
of all results once complete.

While `join!(a, b)` is similar to `(a.await, b.await)`,
`join!` polls both futures concurrently and therefore is more efficient.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

```rust
futures::executor::block_on(async {
use futures::join;

let a = async { 1 };
let b = async { 2 };
assert_eq!(join!(a, b), (1, 2));

// `join!` is variadic, so you can pass any number of futures
let c = async { 3 };
let d = async { 4 };
let e = async { 5 };
assert_eq!(join!(c, d, e), (3, 4, 5));
});
```

### `try_join!`

Polls multiple futures simultaneously, resolving to a [`Result`](../sel4/error/index.md) containing
either a tuple of the successful outputs or an error.

`try_join!` is similar to [`join!`](#join), but completes immediately if any of
the futures return an error.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

When used on multiple futures that return `Ok`, `try_join!` will return
`Ok` of a tuple of the values:

```rust
futures::executor::block_on(async {
use futures::try_join;

let a = async { Ok::<i32, i32>(1) };
let b = async { Ok::<i32, i32>(2) };
assert_eq!(try_join!(a, b), Ok((1, 2)));

// `try_join!` is variadic, so you can pass any number of futures
let c = async { Ok::<i32, i32>(3) };
let d = async { Ok::<i32, i32>(4) };
let e = async { Ok::<i32, i32>(5) };
assert_eq!(try_join!(c, d, e), Ok((3, 4, 5)));
});
```

If one of the futures resolves to an error, `try_join!` will return
that error:

```rust
futures::executor::block_on(async {
use futures::try_join;

let a = async { Ok::<i32, i32>(1) };
let b = async { Err::<u64, i32>(2) };

assert_eq!(try_join!(a, b), Err(2));
});
```

### `select_biased!`

Polls multiple futures and streams simultaneously, executing the branch
for the future that finishes first. Unlike `select!`, if multiple futures are ready,
one will be selected in order of declaration. Futures directly
passed to `select_biased!` must be `Unpin` and implement `FusedFuture`.

If an expression which yields a `Future` is passed to `select_biased!`
(e.g. an `async fn` call) instead of a `Future` by name the `Unpin`
requirement is relaxed, since the macro will pin the resulting `Future`
on the stack. However the `Future` returned by the expression must
still implement `FusedFuture`.

Futures and streams which are not already fused can be fused using the
`.fuse()` method. Note, though, that fusing a future or stream directly
in the call to `select_biased!` will not be enough to prevent it from being
polled after completion if the `select_biased!` call is in a loop, so when
`select_biased!`ing in a loop, users should take care to `fuse()` outside of
the loop.

`select_biased!` can be used as an expression and will return the return
value of the selected branch. For this reason the return type of every
branch in a `select_biased!` must be the same.

This macro is only usable inside of async functions, closures, and blocks.
It is also gated behind the `async-await` feature of this library, which is
activated by default.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;
use futures::select_biased;
let mut a = future::ready(4);
let mut b = future::pending::<()>();

let res = select_biased! {
    a_res = a => a_res + 1,
    _ = b => 0,
};
assert_eq!(res, 5);
});
```

```rust
futures::executor::block_on(async {
use futures::future;
use futures::stream::{self, StreamExt};
use futures::select_biased;
let mut st = stream::iter(vec![2]).fuse();
let mut fut = future::pending::<()>();

select_biased! {
    x = st.next() => assert_eq!(Some(2), x),
    _ = fut => panic!(),
}
});
```

As described earlier, `select_biased` can directly select on expressions
which return `Future`s - even if those do not implement `Unpin`:

```rust
futures::executor::block_on(async {
use futures::future::FutureExt;
use futures::select_biased;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let res = select_biased! {
    a_res = async_identity_fn(62).fuse() => a_res + 1,
    b_res = async_identity_fn(13).fuse() => b_res,
};
assert_eq!(res, 63);
});
```

If a similar async function is called outside of `select_biased` to produce
a `Future`, the `Future` must be pinned in order to be able to pass
it to `select_biased`. This can be achieved via `Box::pin` for pinning a
`Future` on the heap or the `pin!` macro for pinning a `Future`
on the stack.

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::future::FutureExt;
use futures::select_biased;

// Calling the following async fn returns a Future which does not
// implement Unpin
async fn async_identity_fn(arg: usize) -> usize {
    arg
}

let fut_1 = async_identity_fn(1).fuse();
let fut_2 = async_identity_fn(2).fuse();
let mut fut_1 = Box::pin(fut_1); // Pins the Future on the heap
let mut fut_2 = pin!(fut_2); // Pins the Future on the stack

let res = select_biased! {
    a_res = fut_1 => a_res,
    b_res = fut_2 => b_res,
};
assert!(res == 1 || res == 2);
});
```

`select_biased` also accepts a `complete` branch and a `default` branch.
`complete` will run if all futures and streams have already been
exhausted. `default` will run if no futures or streams are
immediately ready. `complete` takes priority over `default` in
the case where all futures have completed.
A motivating use-case for passing `Future`s by name as well as for
`complete` blocks is to call `select_biased!` in a loop, which is
demonstrated in the following example:

```rust
futures::executor::block_on(async {
use futures::future;
use futures::select_biased;
let mut a_fut = future::ready(4);
let mut b_fut = future::ready(6);
let mut total = 0;

loop {
    select_biased! {
        a = a_fut => total += a,
        b = b_fut => total += b,
        complete => break,
        default => panic!(), // never runs (futures run first, then complete)
    }
}
assert_eq!(total, 10);
});
```

Note that the futures that have been matched over can still be mutated
from inside the `select_biased!` block's branches. This can be used to implement
more complex behavior such as timer resets or writing into the head of
a stream.


