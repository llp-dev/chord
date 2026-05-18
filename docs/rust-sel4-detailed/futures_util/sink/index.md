*[futures_util](../index.md) / [sink](index.md)*

---

# Module `sink`

Asynchronous sinks.

This module contains:

- The [`Sink`](#sink) trait, which allows you to asynchronously write data.
- The [`SinkExt`](#sinkext) trait, which provides adapters for chaining and composing
  sinks.

## Contents

- [Modules](#modules)
  - [`close`](#close)
  - [`drain`](#drain)
  - [`fanout`](#fanout)
  - [`feed`](#feed)
  - [`flush`](#flush)
  - [`err_into`](#err-into)
  - [`map_err`](#map-err)
  - [`send`](#send)
  - [`send_all`](#send-all)
  - [`unfold`](#unfold)
  - [`with`](#with)
  - [`with_flat_map`](#with-flat-map)
  - [`buffer`](#buffer)
- [Structs](#structs)
  - [`Sink`](#sink)
  - [`Close`](#close)
  - [`Drain`](#drain)
  - [`Fanout`](#fanout)
  - [`Feed`](#feed)
  - [`Flush`](#flush)
  - [`SinkErrInto`](#sinkerrinto)
  - [`SinkMapErr`](#sinkmaperr)
  - [`Send`](#send)
  - [`SendAll`](#sendall)
  - [`Unfold`](#unfold)
  - [`With`](#with)
  - [`WithFlatMap`](#withflatmap)
  - [`Buffer`](#buffer)
- [Traits](#traits)
  - [`SinkExt`](#sinkext)
- [Functions](#functions)
  - [`drain`](#drain)
  - [`unfold`](#unfold)
  - [`assert_sink`](#assert-sink)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`close`](#close) | mod |  |
| [`drain`](#drain) | mod |  |
| [`fanout`](#fanout) | mod |  |
| [`feed`](#feed) | mod |  |
| [`flush`](#flush) | mod |  |
| [`err_into`](#err-into) | mod |  |
| [`map_err`](#map-err) | mod |  |
| [`send`](#send) | mod |  |
| [`send_all`](#send-all) | mod |  |
| [`unfold`](#unfold) | mod |  |
| [`with`](#with) | mod |  |
| [`with_flat_map`](#with-flat-map) | mod |  |
| [`buffer`](#buffer) | mod |  |
| [`Sink`](#sink) | struct |  |
| [`Close`](#close) | struct |  |
| [`Drain`](#drain) | struct |  |
| [`Fanout`](#fanout) | struct |  |
| [`Feed`](#feed) | struct |  |
| [`Flush`](#flush) | struct |  |
| [`SinkErrInto`](#sinkerrinto) | struct |  |
| [`SinkMapErr`](#sinkmaperr) | struct |  |
| [`Send`](#send) | struct |  |
| [`SendAll`](#sendall) | struct |  |
| [`Unfold`](#unfold) | struct |  |
| [`With`](#with) | struct |  |
| [`WithFlatMap`](#withflatmap) | struct |  |
| [`Buffer`](#buffer) | struct |  |
| [`SinkExt`](#sinkext) | trait | An extension trait for `Sink`s that provides a variety of convenient combinator functions. |
| [`drain`](#drain) | fn |  |
| [`unfold`](#unfold) | fn |  |
| [`assert_sink`](#assert-sink) | fn |  |

## Modules

- [`close`](close/index.md)
- [`drain`](drain/index.md)
- [`fanout`](fanout/index.md)
- [`feed`](feed/index.md)
- [`flush`](flush/index.md)
- [`err_into`](err_into/index.md)
- [`map_err`](map_err/index.md)
- [`send`](send/index.md)
- [`send_all`](send_all/index.md)
- [`unfold`](unfold/index.md)
- [`with`](with/index.md)
- [`with_flat_map`](with_flat_map/index.md)
- [`buffer`](buffer/index.md)

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

### `Close<'a, Si: ?Sized, Item>`

```rust
struct Close<'a, Si: ?Sized, Item> {
    sink: &'a mut Si,
    _phantom: core::marker::PhantomData<fn(Item)>,
}
```

Future for the [`close`](super::SinkExt::close) method.

#### Implementations

- <span id="close-new"></span>`fn new(sink: &'a mut Si) -> Self`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Close<'a, Si, Item>`

- <span id="close-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Close<'_, Si, Item>`

- <span id="close-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="close-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Close<'a, Si, Item>`

##### `impl IntoFuture for Close<'a, Si, Item>`

- <span id="close-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="close-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="close-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Close<'a, Si, Item>`

- <span id="close-tryfuture-type-ok"></span>`type Ok = T`

- <span id="close-tryfuture-type-error"></span>`type Error = E`

- <span id="close-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Close<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Close<'_, Si, Item>`

### `Drain<T>`

```rust
struct Drain<T> {
    marker: core::marker::PhantomData<T>,
}
```

Sink for the [`drain`](drain/index.md) function.

#### Trait Implementations

##### `impl<T> Clone for Drain<T>`

- <span id="drain-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Drain<T>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Sink for Drain<T>`

- <span id="drain-sink-type-error"></span>`type Error = Infallible`

- <span id="drain-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="drain-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, _item: T) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="drain-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="drain-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<T, Item> SinkExt for Drain<T>`

##### `impl<T> Unpin for Drain<T>`

### `Fanout<Si1, Si2>`

```rust
struct Fanout<Si1, Si2> {
    sink1: Si1,
    sink2: Si2,
}
```

Sink that clones incoming items and forwards them to two sinks at the same time.

Backpressure from any downstream sink propagates up, which means that this sink
can only process items as fast as its _slowest_ downstream sink.

#### Implementations

- <span id="fanout-new"></span>`fn new(sink1: Si1, sink2: Si2) -> Self`

- <span id="fanout-get-ref"></span>`fn get_ref(&self) -> (&Si1, &Si2)`

  Get a shared reference to the inner sinks.

- <span id="fanout-get-mut"></span>`fn get_mut(&mut self) -> (&mut Si1, &mut Si2)`

  Get a mutable reference to the inner sinks.

- <span id="fanout-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut Si1>, Pin<&mut Si2>)`

  Get a pinned mutable reference to the inner sinks.

- <span id="fanout-into-inner"></span>`fn into_inner(self) -> (Si1, Si2)`

  Consumes this combinator, returning the underlying sinks.

  

  Note that this may discard intermediate state of this combinator,

  so care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<Si1: Debug, Si2: Debug> Debug for Fanout<Si1, Si2>`

- <span id="fanout-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult`

##### `impl<Si1, Si2, Item> Sink for Fanout<Si1, Si2>`

- <span id="fanout-sink-type-error"></span>`type Error = <Si1 as Sink>::Error`

- <span id="fanout-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="fanout-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="fanout-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="fanout-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<Item> SinkExt for Fanout<Si1, Si2>`

##### `impl<Si1, Si2> Unpin for Fanout<Si1, Si2>`

### `Feed<'a, Si: ?Sized, Item>`

```rust
struct Feed<'a, Si: ?Sized, Item> {
    sink: &'a mut Si,
    item: Option<Item>,
}
```

Future for the [`feed`](super::SinkExt::feed) method.

#### Implementations

- <span id="feed-new"></span>`fn new(sink: &'a mut Si, item: Item) -> Self`

- <span id="feed-sink-pin-mut"></span>`fn sink_pin_mut(&mut self) -> Pin<&mut Si>`

- <span id="feed-is-item-pending"></span>`fn is_item_pending(&self) -> bool`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Feed<'a, Si, Item>`

- <span id="feed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Feed<'_, Si, Item>`

- <span id="feed-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="feed-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Feed<'a, Si, Item>`

##### `impl IntoFuture for Feed<'a, Si, Item>`

- <span id="feed-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="feed-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="feed-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Feed<'a, Si, Item>`

- <span id="feed-tryfuture-type-ok"></span>`type Ok = T`

- <span id="feed-tryfuture-type-error"></span>`type Error = E`

- <span id="feed-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Feed<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Feed<'_, Si, Item>`

### `Flush<'a, Si: ?Sized, Item>`

```rust
struct Flush<'a, Si: ?Sized, Item> {
    sink: &'a mut Si,
    _phantom: core::marker::PhantomData<fn(Item)>,
}
```

Future for the [`flush`](super::SinkExt::flush) method.

#### Implementations

- <span id="flush-new"></span>`fn new(sink: &'a mut Si) -> Self`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Flush<'a, Si, Item>`

- <span id="flush-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Flush<'_, Si, Item>`

- <span id="flush-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="flush-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Flush<'a, Si, Item>`

##### `impl IntoFuture for Flush<'a, Si, Item>`

- <span id="flush-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="flush-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="flush-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Flush<'a, Si, Item>`

- <span id="flush-tryfuture-type-ok"></span>`type Ok = T`

- <span id="flush-tryfuture-type-error"></span>`type Error = E`

- <span id="flush-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Flush<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Flush<'_, Si, Item>`

### `SinkErrInto<Si: Sink<Item>, Item, E>`

```rust
struct SinkErrInto<Si: Sink<Item>, Item, E> {
    sink: crate::sink::SinkMapErr<Si, fn(<Si as >::Error) -> E>,
}
```

Sink for the [`sink_err_into`](super::SinkExt::sink_err_into) method.

#### Implementations

- <span id="sinkerrinto-new"></span>`fn new(sink: Si) -> Self`

- <span id="sinkerrinto-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="sinkerrinto-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkerrinto-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkerrinto-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<Si: fmt::Debug + Sink<Item>, Item: fmt::Debug, E: fmt::Debug> Debug for SinkErrInto<Si, Item, E>`

- <span id="sinkerrinto-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item, E> FusedStream for SinkErrInto<S, Item, E>`

- <span id="sinkerrinto-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, Item, E> Sink for SinkErrInto<Si, Item, E>`

- <span id="sinkerrinto-sink-type-error"></span>`type Error = E`

- <span id="sinkerrinto-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="sinkerrinto-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="sinkerrinto-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="sinkerrinto-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<Item> SinkExt for SinkErrInto<Si, Item, E>`

##### `impl<S, Item, E> Stream for SinkErrInto<S, Item, E>`

- <span id="sinkerrinto-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="sinkerrinto-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="sinkerrinto-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for SinkErrInto<Si, Item, E>`

##### `impl<E> TryStream for SinkErrInto<Si, Item, E>`

- <span id="sinkerrinto-trystream-type-ok"></span>`type Ok = T`

- <span id="sinkerrinto-trystream-type-error"></span>`type Error = E`

- <span id="sinkerrinto-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for SinkErrInto<Si, Item, E>`

##### `impl<Si: Sink<Item>, Item, E> Unpin for SinkErrInto<Si, Item, E>`

### `SinkMapErr<Si, F>`

```rust
struct SinkMapErr<Si, F> {
    sink: Si,
    f: Option<F>,
}
```

Sink for the [`sink_map_err`](super::SinkExt::sink_map_err) method.

#### Implementations

- <span id="sinkmaperr-new"></span>`fn new(sink: Si, f: F) -> Self`

- <span id="sinkmaperr-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="sinkmaperr-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkmaperr-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkmaperr-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="sinkmaperr-take-f"></span>`fn take_f(self: Pin<&mut Self>) -> F`

#### Trait Implementations

##### `impl<Si: clone::Clone, F: clone::Clone> Clone for SinkMapErr<Si, F>`

- <span id="sinkmaperr-clone"></span>`fn clone(&self) -> SinkMapErr<Si, F>` — [`SinkMapErr`](map_err/index.md#sinkmaperr)

##### `impl<Si: fmt::Debug, F: fmt::Debug> Debug for SinkMapErr<Si, F>`

- <span id="sinkmaperr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: FusedStream, F> FusedStream for SinkMapErr<S, F>`

- <span id="sinkmaperr-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, F, Item> Sink for SinkMapErr<Si, F>`

- <span id="sinkmaperr-sink-type-error"></span>`type Error = E`

- <span id="sinkmaperr-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="sinkmaperr-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="sinkmaperr-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="sinkmaperr-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<Item> SinkExt for SinkMapErr<Si, F>`

##### `impl<S: Stream, F> Stream for SinkMapErr<S, F>`

- <span id="sinkmaperr-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="sinkmaperr-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="sinkmaperr-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for SinkMapErr<Si, F>`

##### `impl TryStream for SinkMapErr<Si, F>`

- <span id="sinkmaperr-trystream-type-ok"></span>`type Ok = T`

- <span id="sinkmaperr-trystream-type-error"></span>`type Error = E`

- <span id="sinkmaperr-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for SinkMapErr<Si, F>`

##### `impl<Si, F> Unpin for SinkMapErr<Si, F>`

### `Send<'a, Si: ?Sized, Item>`

```rust
struct Send<'a, Si: ?Sized, Item> {
    feed: super::Feed<'a, Si, Item>,
}
```

Future for the [`send`](super::SinkExt::send) method.

#### Implementations

- <span id="send-new"></span>`fn new(sink: &'a mut Si, item: Item) -> Self`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Send<'a, Si, Item>`

- <span id="send-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Send<'_, Si, Item>`

- <span id="send-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="send-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for Send<'a, Si, Item>`

##### `impl IntoFuture for Send<'a, Si, Item>`

- <span id="send-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="send-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="send-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Send<'a, Si, Item>`

- <span id="send-tryfuture-type-ok"></span>`type Ok = T`

- <span id="send-tryfuture-type-error"></span>`type Error = E`

- <span id="send-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Send<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Send<'_, Si, Item>`

### `SendAll<'a, Si, St>`

```rust
struct SendAll<'a, Si, St>
where
    Si: ?Sized,
    St: ?Sized + TryStream {
    sink: &'a mut Si,
    stream: crate::stream::Fuse<&'a mut St>,
    buffered: Option<<St as >::Ok>,
}
```

Future for the [`send_all`](super::SinkExt::send_all) method.

#### Implementations

- <span id="sendall-new"></span>`fn new(sink: &'a mut Si, stream: &'a mut St) -> Self`

- <span id="sendall-try-start-send"></span>`fn try_start_send(&mut self, cx: &mut Context<'_>, item: <St as >::Ok) -> Poll<Result<(), <Si as >::Error>>` — [`Context`](../task/index.md#context), [`TryStream`](../stream/index.md#trystream), [`Poll`](../task/index.md#poll)

#### Trait Implementations

##### `impl<Si, St> Debug for SendAll<'_, Si, St>`

- <span id="sendall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si, St> Future for SendAll<'_, Si, St>`

- <span id="sendall-future-type-output"></span>`type Output = Result<(), Error>`

- <span id="sendall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl FutureExt for SendAll<'a, Si, St>`

##### `impl IntoFuture for SendAll<'a, Si, St>`

- <span id="sendall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="sendall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="sendall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for SendAll<'a, Si, St>`

- <span id="sendall-tryfuture-type-ok"></span>`type Ok = T`

- <span id="sendall-tryfuture-type-error"></span>`type Error = E`

- <span id="sendall-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for SendAll<'a, Si, St>`

##### `impl<Si, St> Unpin for SendAll<'_, Si, St>`

### `Unfold<T, F, R>`

```rust
struct Unfold<T, F, R> {
    function: F,
    state: crate::unfold_state::UnfoldState<T, R>,
}
```

Sink for the [`unfold`](unfold/index.md) function.

#### Trait Implementations

##### `impl<T: fmt::Debug, F: fmt::Debug, R: fmt::Debug> Debug for Unfold<T, F, R>`

- <span id="unfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F, R, Item> Sink for Unfold<T, F, R>`

- <span id="unfold-sink-type-error"></span>`type Error = E`

- <span id="unfold-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="unfold-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="unfold-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="unfold-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<T, Item> SinkExt for Unfold<T, F, R>`

##### `impl<T, F, R> Unpin for Unfold<T, F, R>`

### `With<Si, Item, U, Fut, F>`

```rust
struct With<Si, Item, U, Fut, F> {
    sink: Si,
    f: F,
    state: Option<Fut>,
    _phantom: core::marker::PhantomData<fn(U) -> Item>,
}
```

Sink for the [`with`](super::SinkExt::with) method.

#### Implementations

- <span id="with-new"></span>`fn new<E>(sink: Si, f: F) -> Self`

#### Trait Implementations

##### `impl<Si, Item, U, Fut, F> Clone for With<Si, Item, U, Fut, F>`

- <span id="with-clone"></span>`fn clone(&self) -> Self`

##### `impl<Si, Item, U, Fut, F> Debug for With<Si, Item, U, Fut, F>`

- <span id="with-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item, U, Fut, F> FusedStream for With<S, Item, U, Fut, F>`

- <span id="with-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, Item, U, Fut, F> Sink for With<Si, Item, U, Fut, F>`

- <span id="with-sink-type-error"></span>`type Error = E`

- <span id="with-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="with-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: U) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="with-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="with-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<Item> SinkExt for With<Si, Item, U, Fut, F>`

##### `impl<S, Item, U, Fut, F> Stream for With<S, Item, U, Fut, F>`

- <span id="with-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="with-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="with-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for With<Si, Item, U, Fut, F>`

##### `impl TryStream for With<Si, Item, U, Fut, F>`

- <span id="with-trystream-type-ok"></span>`type Ok = T`

- <span id="with-trystream-type-error"></span>`type Error = E`

- <span id="with-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for With<Si, Item, U, Fut, F>`

##### `impl<Si, Item, U, Fut, F> Unpin for With<Si, Item, U, Fut, F>`

### `WithFlatMap<Si, Item, U, St, F>`

```rust
struct WithFlatMap<Si, Item, U, St, F> {
    sink: Si,
    f: F,
    stream: Option<St>,
    buffer: Option<Item>,
    _marker: core::marker::PhantomData<fn(U)>,
}
```

Sink for the [`with_flat_map`](super::SinkExt::with_flat_map) method.

#### Implementations

- <span id="withflatmap-new"></span>`fn new(sink: Si, f: F) -> Self`

- <span id="withflatmap-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="withflatmap-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="withflatmap-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="withflatmap-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="withflatmap-try-empty-stream"></span>`fn try_empty_stream(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Si as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

#### Trait Implementations

##### `impl<Si, Item, U, St, F> Debug for WithFlatMap<Si, Item, U, St, F>`

- <span id="withflatmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item, U, St, F> FusedStream for WithFlatMap<S, Item, U, St, F>`

- <span id="withflatmap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, Item, U, St, F> Sink for WithFlatMap<Si, Item, U, St, F>`

- <span id="withflatmap-sink-type-error"></span>`type Error = <Si as Sink>::Error`

- <span id="withflatmap-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="withflatmap-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: U) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="withflatmap-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="withflatmap-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<Item> SinkExt for WithFlatMap<Si, Item, U, St, F>`

##### `impl<S, Item, U, St, F> Stream for WithFlatMap<S, Item, U, St, F>`

- <span id="withflatmap-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="withflatmap-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="withflatmap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for WithFlatMap<Si, Item, U, St, F>`

##### `impl TryStream for WithFlatMap<Si, Item, U, St, F>`

- <span id="withflatmap-trystream-type-ok"></span>`type Ok = T`

- <span id="withflatmap-trystream-type-error"></span>`type Error = E`

- <span id="withflatmap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for WithFlatMap<Si, Item, U, St, F>`

##### `impl<Si, Item, U, St, F> Unpin for WithFlatMap<Si, Item, U, St, F>`

### `Buffer<Si, Item>`

```rust
struct Buffer<Si, Item> {
    sink: Si,
    buf: alloc::collections::VecDeque<Item>,
    capacity: usize,
}
```

Sink for the [`buffer`](super::SinkExt::buffer) method.

#### Implementations

- <span id="buffer-new"></span>`fn new(sink: Si, capacity: usize) -> Self`

- <span id="buffer-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="buffer-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffer-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffer-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="buffer-try-empty-buffer"></span>`fn try_empty_buffer(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Si as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

#### Trait Implementations

##### `impl<Si: fmt::Debug, Item: fmt::Debug> Debug for Buffer<Si, Item>`

- <span id="buffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item> FusedStream for Buffer<S, Item>`

- <span id="buffer-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si: Sink<Item>, Item> Sink for Buffer<Si, Item>`

- <span id="buffer-sink-type-error"></span>`type Error = <Si as Sink>::Error`

- <span id="buffer-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="buffer-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](#sink)

- <span id="buffer-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

- <span id="buffer-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](#sink)

##### `impl<Item> SinkExt for Buffer<Si, Item>`

##### `impl<S, Item> Stream for Buffer<S, Item>`

- <span id="buffer-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="buffer-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="buffer-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Buffer<Si, Item>`

##### `impl TryStream for Buffer<Si, Item>`

- <span id="buffer-trystream-type-ok"></span>`type Ok = T`

- <span id="buffer-trystream-type-error"></span>`type Error = E`

- <span id="buffer-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for Buffer<Si, Item>`

##### `impl<Si, Item> Unpin for Buffer<Si, Item>`

## Traits

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

### `drain`

```rust
fn drain<T>() -> Drain<T>
```

Create a sink that will just discard all items given to it.

Similar to [`io::Sink`](::std::io::Sink).

# Examples

```rust
futures::executor::block_on(async {
use futures::sink::{self, SinkExt};

let mut drain = sink::drain();
drain.send(5).await?;
Ok::<(), futures::never::Never>(()) }).unwrap();
```

### `unfold`

```rust
fn unfold<T, F, R, Item, E>(init: T, function: F) -> Unfold<T, F, R>
where
    F: FnMut(T, Item) -> R,
    R: Future<Output = Result<T, E>>
```

Create a sink from a function which processes one item at a time.

# Examples

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::sink;
use futures::sink::SinkExt;

let unfold = sink::unfold(0, |mut sum, i: i32| {
    async move {
        sum += i;
        eprintln!("{}", i);
        Ok::<_, futures::never::Never>(sum)
    }
});
let mut unfold = pin!(unfold);
unfold.send(5).await?;
Ok::<(), futures::never::Never>(()) }).unwrap();
```

### `assert_sink`

```rust
fn assert_sink<T, E, S>(sink: S) -> S
where
    S: Sink<T, Error = E>
```

