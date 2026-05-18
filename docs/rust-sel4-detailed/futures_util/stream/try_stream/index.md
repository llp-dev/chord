*[futures_util](../../index.md) / [stream](../index.md) / [try_stream](index.md)*

---

# Module `try_stream`

Streams

This module contains a number of functions for working with `Streams`s
that return `Result`s, allowing for short-circuiting computations.

## Contents

- [Modules](#modules)
  - [`and_then`](#and-then)
  - [`into_stream`](#into-stream)
  - [`or_else`](#or-else)
  - [`try_next`](#try-next)
  - [`try_for_each`](#try-for-each)
  - [`try_filter`](#try-filter)
  - [`try_filter_map`](#try-filter-map)
  - [`try_flatten`](#try-flatten)
  - [`try_flatten_unordered`](#try-flatten-unordered)
  - [`try_collect`](#try-collect)
  - [`try_concat`](#try-concat)
  - [`try_chunks`](#try-chunks)
  - [`try_ready_chunks`](#try-ready-chunks)
  - [`try_fold`](#try-fold)
  - [`try_unfold`](#try-unfold)
  - [`try_skip_while`](#try-skip-while)
  - [`try_take_while`](#try-take-while)
  - [`try_buffer_unordered`](#try-buffer-unordered)
  - [`try_buffered`](#try-buffered)
  - [`try_for_each_concurrent`](#try-for-each-concurrent)
  - [`try_all`](#try-all)
  - [`try_any`](#try-any)
- [Structs](#structs)
  - [`AndThen`](#andthen)
  - [`ErrInto`](#errinto)
  - [`InspectOk`](#inspectok)
  - [`InspectErr`](#inspecterr)
  - [`IntoStream`](#intostream)
  - [`MapOk`](#mapok)
  - [`MapErr`](#maperr)
  - [`OrElse`](#orelse)
  - [`TryNext`](#trynext)
  - [`TryForEach`](#tryforeach)
  - [`TryFilter`](#tryfilter)
  - [`TryFilterMap`](#tryfiltermap)
  - [`TryFlatten`](#tryflatten)
  - [`TryFlattenUnordered`](#tryflattenunordered)
  - [`TryCollect`](#trycollect)
  - [`TryConcat`](#tryconcat)
  - [`TryChunks`](#trychunks)
  - [`TryChunksError`](#trychunkserror)
  - [`TryReadyChunks`](#tryreadychunks)
  - [`TryReadyChunksError`](#tryreadychunkserror)
  - [`TryFold`](#tryfold)
  - [`TryUnfold`](#tryunfold)
  - [`TrySkipWhile`](#tryskipwhile)
  - [`TryTakeWhile`](#trytakewhile)
  - [`TryBufferUnordered`](#trybufferunordered)
  - [`TryBuffered`](#trybuffered)
  - [`TryForEachConcurrent`](#tryforeachconcurrent)
  - [`TryAll`](#tryall)
  - [`TryAny`](#tryany)
- [Traits](#traits)
  - [`TryStreamExt`](#trystreamext)
- [Functions](#functions)
  - [`try_unfold`](#try-unfold)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`and_then`](#and-then) | mod |  |
| [`into_stream`](#into-stream) | mod |  |
| [`or_else`](#or-else) | mod |  |
| [`try_next`](#try-next) | mod |  |
| [`try_for_each`](#try-for-each) | mod |  |
| [`try_filter`](#try-filter) | mod |  |
| [`try_filter_map`](#try-filter-map) | mod |  |
| [`try_flatten`](#try-flatten) | mod |  |
| [`try_flatten_unordered`](#try-flatten-unordered) | mod |  |
| [`try_collect`](#try-collect) | mod |  |
| [`try_concat`](#try-concat) | mod |  |
| [`try_chunks`](#try-chunks) | mod |  |
| [`try_ready_chunks`](#try-ready-chunks) | mod |  |
| [`try_fold`](#try-fold) | mod |  |
| [`try_unfold`](#try-unfold) | mod |  |
| [`try_skip_while`](#try-skip-while) | mod |  |
| [`try_take_while`](#try-take-while) | mod |  |
| [`try_buffer_unordered`](#try-buffer-unordered) | mod |  |
| [`try_buffered`](#try-buffered) | mod |  |
| [`try_for_each_concurrent`](#try-for-each-concurrent) | mod |  |
| [`try_all`](#try-all) | mod |  |
| [`try_any`](#try-any) | mod |  |
| [`AndThen`](#andthen) | struct |  |
| [`ErrInto`](#errinto) | struct | Stream for the [`err_into`](super::TryStreamExt::err_into) method. |
| [`InspectOk`](#inspectok) | struct | Stream for the [`inspect_ok`](super::TryStreamExt::inspect_ok) method. |
| [`InspectErr`](#inspecterr) | struct | Stream for the [`inspect_err`](super::TryStreamExt::inspect_err) method. |
| [`IntoStream`](#intostream) | struct |  |
| [`MapOk`](#mapok) | struct | Stream for the [`map_ok`](super::TryStreamExt::map_ok) method. |
| [`MapErr`](#maperr) | struct | Stream for the [`map_err`](super::TryStreamExt::map_err) method. |
| [`OrElse`](#orelse) | struct |  |
| [`TryNext`](#trynext) | struct |  |
| [`TryForEach`](#tryforeach) | struct |  |
| [`TryFilter`](#tryfilter) | struct |  |
| [`TryFilterMap`](#tryfiltermap) | struct |  |
| [`TryFlatten`](#tryflatten) | struct |  |
| [`TryFlattenUnordered`](#tryflattenunordered) | struct |  |
| [`TryCollect`](#trycollect) | struct |  |
| [`TryConcat`](#tryconcat) | struct |  |
| [`TryChunks`](#trychunks) | struct |  |
| [`TryChunksError`](#trychunkserror) | struct |  |
| [`TryReadyChunks`](#tryreadychunks) | struct |  |
| [`TryReadyChunksError`](#tryreadychunkserror) | struct |  |
| [`TryFold`](#tryfold) | struct |  |
| [`TryUnfold`](#tryunfold) | struct |  |
| [`TrySkipWhile`](#tryskipwhile) | struct |  |
| [`TryTakeWhile`](#trytakewhile) | struct |  |
| [`TryBufferUnordered`](#trybufferunordered) | struct |  |
| [`TryBuffered`](#trybuffered) | struct |  |
| [`TryForEachConcurrent`](#tryforeachconcurrent) | struct |  |
| [`TryAll`](#tryall) | struct |  |
| [`TryAny`](#tryany) | struct |  |
| [`TryStreamExt`](#trystreamext) | trait | Adapters specific to `Result`-returning streams |
| [`try_unfold`](#try-unfold) | fn |  |

## Modules

- [`and_then`](and_then/index.md)
- [`into_stream`](into_stream/index.md)
- [`or_else`](or_else/index.md)
- [`try_next`](try_next/index.md)
- [`try_for_each`](try_for_each/index.md)
- [`try_filter`](try_filter/index.md)
- [`try_filter_map`](try_filter_map/index.md)
- [`try_flatten`](try_flatten/index.md)
- [`try_flatten_unordered`](try_flatten_unordered/index.md)
- [`try_collect`](try_collect/index.md)
- [`try_concat`](try_concat/index.md)
- [`try_chunks`](try_chunks/index.md)
- [`try_ready_chunks`](try_ready_chunks/index.md)
- [`try_fold`](try_fold/index.md)
- [`try_unfold`](try_unfold/index.md)
- [`try_skip_while`](try_skip_while/index.md)
- [`try_take_while`](try_take_while/index.md)
- [`try_buffer_unordered`](try_buffer_unordered/index.md)
- [`try_buffered`](try_buffered/index.md)
- [`try_for_each_concurrent`](try_for_each_concurrent/index.md)
- [`try_all`](try_all/index.md)
- [`try_any`](try_any/index.md)

## Structs

### `AndThen<St, Fut, F>`

```rust
struct AndThen<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`and_then`](super::TryStreamExt::and_then) method.

#### Implementations

- <span id="andthen-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="andthen-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="andthen-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="andthen-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="andthen-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for AndThen<St, Fut, F>`

- <span id="andthen-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for AndThen<St, Fut, F>`

- <span id="andthen-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for AndThen<S, Fut, F>`

- <span id="andthen-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="andthen-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="andthen-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="andthen-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="andthen-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for AndThen<St, Fut, F>`

##### `impl<St, Fut, F> Stream for AndThen<St, Fut, F>`

- <span id="andthen-stream-type-item"></span>`type Item = Result<<Fut as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="andthen-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="andthen-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for AndThen<St, Fut, F>`

##### `impl TryStream for AndThen<St, Fut, F>`

- <span id="andthen-trystream-type-ok"></span>`type Ok = T`

- <span id="andthen-trystream-type-error"></span>`type Error = E`

- <span id="andthen-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for AndThen<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for AndThen<St, Fut, F>`

### `ErrInto<St, E>`

```rust
struct ErrInto<St, E> {
    inner: MapErr<St, crate::fns::IntoFn<E>>,
}
```

Stream for the [`err_into`](super::TryStreamExt::err_into) method.

#### Implementations

- <span id="errinto-new"></span>`fn new(x: St) -> Self`

#### Trait Implementations

##### `impl<St, E> Debug for ErrInto<St, E>`

- <span id="errinto-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, E> FusedStream for ErrInto<St, E>`

- <span id="errinto-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, E> Sink for ErrInto<St, E>`

- <span id="errinto-sink-type-error"></span>`type Error = <MapErr<St, IntoFn<E>> as Sink>::Error`

- <span id="errinto-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="errinto-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="errinto-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="errinto-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for ErrInto<St, E>`

##### `impl<St, E> Stream for ErrInto<St, E>`

- <span id="errinto-stream-type-item"></span>`type Item = <MapErr<St, IntoFn<E>> as Stream>::Item`

- <span id="errinto-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="errinto-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for ErrInto<St, E>`

##### `impl<E> TryStream for ErrInto<St, E>`

- <span id="errinto-trystream-type-ok"></span>`type Ok = T`

- <span id="errinto-trystream-type-error"></span>`type Error = E`

- <span id="errinto-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for ErrInto<St, E>`

##### `impl<St, E> Unpin for ErrInto<St, E>`

### `InspectOk<St, F>`

```rust
struct InspectOk<St, F> {
    inner: crate::stream::Inspect<IntoStream<St>, crate::fns::InspectOkFn<F>>,
}
```

Stream for the [`inspect_ok`](super::TryStreamExt::inspect_ok) method.

#### Implementations

- <span id="inspectok-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for InspectOk<St, F>`

- <span id="inspectok-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for InspectOk<St, F>`

- <span id="inspectok-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for InspectOk<St, F>`

- <span id="inspectok-sink-type-error"></span>`type Error = <Inspect<IntoStream<St>, InspectOkFn<F>> as Sink>::Error`

- <span id="inspectok-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="inspectok-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="inspectok-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="inspectok-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for InspectOk<St, F>`

##### `impl<St, F> Stream for InspectOk<St, F>`

- <span id="inspectok-stream-type-item"></span>`type Item = <Inspect<IntoStream<St>, InspectOkFn<F>> as Stream>::Item`

- <span id="inspectok-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="inspectok-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for InspectOk<St, F>`

##### `impl TryStream for InspectOk<St, F>`

- <span id="inspectok-trystream-type-ok"></span>`type Ok = T`

- <span id="inspectok-trystream-type-error"></span>`type Error = E`

- <span id="inspectok-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for InspectOk<St, F>`

##### `impl<St, F> Unpin for InspectOk<St, F>`

### `InspectErr<St, F>`

```rust
struct InspectErr<St, F> {
    inner: crate::stream::Inspect<IntoStream<St>, crate::fns::InspectErrFn<F>>,
}
```

Stream for the [`inspect_err`](super::TryStreamExt::inspect_err) method.

#### Implementations

- <span id="inspecterr-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for InspectErr<St, F>`

- <span id="inspecterr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for InspectErr<St, F>`

- <span id="inspecterr-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for InspectErr<St, F>`

- <span id="inspecterr-sink-type-error"></span>`type Error = <Inspect<IntoStream<St>, InspectErrFn<F>> as Sink>::Error`

- <span id="inspecterr-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="inspecterr-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="inspecterr-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="inspecterr-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for InspectErr<St, F>`

##### `impl<St, F> Stream for InspectErr<St, F>`

- <span id="inspecterr-stream-type-item"></span>`type Item = <Inspect<IntoStream<St>, InspectErrFn<F>> as Stream>::Item`

- <span id="inspecterr-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="inspecterr-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for InspectErr<St, F>`

##### `impl TryStream for InspectErr<St, F>`

- <span id="inspecterr-trystream-type-ok"></span>`type Ok = T`

- <span id="inspecterr-trystream-type-error"></span>`type Error = E`

- <span id="inspecterr-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for InspectErr<St, F>`

##### `impl<St, F> Unpin for InspectErr<St, F>`

### `IntoStream<St>`

```rust
struct IntoStream<St> {
    stream: St,
}
```

Stream for the [`into_stream`](super::TryStreamExt::into_stream) method.

#### Implementations

- <span id="intostream-new"></span>`fn new(stream: St) -> Self`

- <span id="intostream-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="intostream-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="intostream-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="intostream-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for IntoStream<St>`

- <span id="intostream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: TryStream + FusedStream> FusedStream for IntoStream<St>`

- <span id="intostream-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S: Sink<Item>, Item> Sink for IntoStream<S>`

- <span id="intostream-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="intostream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="intostream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="intostream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="intostream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for IntoStream<St>`

##### `impl<St: TryStream> Stream for IntoStream<St>`

- <span id="intostream-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="intostream-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="intostream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for IntoStream<St>`

##### `impl TryStream for IntoStream<St>`

- <span id="intostream-trystream-type-ok"></span>`type Ok = T`

- <span id="intostream-trystream-type-error"></span>`type Error = E`

- <span id="intostream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for IntoStream<St>`

##### `impl<St> Unpin for IntoStream<St>`

### `MapOk<St, F>`

```rust
struct MapOk<St, F> {
    inner: crate::stream::Map<IntoStream<St>, crate::fns::MapOkFn<F>>,
}
```

Stream for the [`map_ok`](super::TryStreamExt::map_ok) method.

#### Implementations

- <span id="mapok-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for MapOk<St, F>`

- <span id="mapok-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for MapOk<St, F>`

- <span id="mapok-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for MapOk<St, F>`

- <span id="mapok-sink-type-error"></span>`type Error = <Map<IntoStream<St>, MapOkFn<F>> as Sink>::Error`

- <span id="mapok-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="mapok-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="mapok-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="mapok-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for MapOk<St, F>`

##### `impl<St, F> Stream for MapOk<St, F>`

- <span id="mapok-stream-type-item"></span>`type Item = <Map<IntoStream<St>, MapOkFn<F>> as Stream>::Item`

- <span id="mapok-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="mapok-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for MapOk<St, F>`

##### `impl TryStream for MapOk<St, F>`

- <span id="mapok-trystream-type-ok"></span>`type Ok = T`

- <span id="mapok-trystream-type-error"></span>`type Error = E`

- <span id="mapok-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for MapOk<St, F>`

##### `impl<St, F> Unpin for MapOk<St, F>`

### `MapErr<St, F>`

```rust
struct MapErr<St, F> {
    inner: crate::stream::Map<IntoStream<St>, crate::fns::MapErrFn<F>>,
}
```

Stream for the [`map_err`](super::TryStreamExt::map_err) method.

#### Implementations

- <span id="maperr-new"></span>`fn new(x: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, F> Debug for MapErr<St, F>`

- <span id="maperr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St, F> FusedStream for MapErr<St, F>`

- <span id="maperr-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St, F> Sink for MapErr<St, F>`

- <span id="maperr-sink-type-error"></span>`type Error = <Map<IntoStream<St>, MapErrFn<F>> as Sink>::Error`

- <span id="maperr-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="maperr-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="maperr-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="maperr-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for MapErr<St, F>`

##### `impl<St, F> Stream for MapErr<St, F>`

- <span id="maperr-stream-type-item"></span>`type Item = <Map<IntoStream<St>, MapErrFn<F>> as Stream>::Item`

- <span id="maperr-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="maperr-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for MapErr<St, F>`

##### `impl TryStream for MapErr<St, F>`

- <span id="maperr-trystream-type-ok"></span>`type Ok = T`

- <span id="maperr-trystream-type-error"></span>`type Error = E`

- <span id="maperr-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for MapErr<St, F>`

##### `impl<St, F> Unpin for MapErr<St, F>`

### `OrElse<St, Fut, F>`

```rust
struct OrElse<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`or_else`](super::TryStreamExt::or_else) method.

#### Implementations

- <span id="orelse-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="orelse-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="orelse-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="orelse-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="orelse-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for OrElse<St, Fut, F>`

- <span id="orelse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for OrElse<St, Fut, F>`

- <span id="orelse-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for OrElse<S, Fut, F>`

- <span id="orelse-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="orelse-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="orelse-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="orelse-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="orelse-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for OrElse<St, Fut, F>`

##### `impl<St, Fut, F> Stream for OrElse<St, Fut, F>`

- <span id="orelse-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <Fut as TryFuture>::Error>`

- <span id="orelse-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="orelse-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for OrElse<St, Fut, F>`

##### `impl TryStream for OrElse<St, Fut, F>`

- <span id="orelse-trystream-type-ok"></span>`type Ok = T`

- <span id="orelse-trystream-type-error"></span>`type Error = E`

- <span id="orelse-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for OrElse<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for OrElse<St, Fut, F>`

### `TryNext<'a, St: ?Sized>`

```rust
struct TryNext<'a, St: ?Sized> {
    stream: &'a mut St,
}
```

Future for the [`try_next`](super::TryStreamExt::try_next) method.

#### Implementations

- <span id="trynext-new"></span>`fn new(stream: &'a mut St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + ?Sized> Debug for TryNext<'a, St>`

- <span id="trynext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: ?Sized + TryStream + Unpin + FusedStream> FusedFuture for TryNext<'_, St>`

- <span id="trynext-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: ?Sized + TryStream + Unpin> Future for TryNext<'_, St>`

- <span id="trynext-future-type-output"></span>`type Output = Result<Option<<St as TryStream>::Ok>, <St as TryStream>::Error>`

- <span id="trynext-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for TryNext<'a, St>`

##### `impl IntoFuture for TryNext<'a, St>`

- <span id="trynext-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trynext-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trynext-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryNext<'a, St>`

- <span id="trynext-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trynext-tryfuture-type-error"></span>`type Error = E`

- <span id="trynext-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for TryNext<'a, St>`

##### `impl<St: ?Sized + Unpin> Unpin for TryNext<'_, St>`

### `TryForEach<St, Fut, F>`

```rust
struct TryForEach<St, Fut, F> {
    stream: St,
    f: F,
    future: Option<Fut>,
}
```

Future for the [`try_for_each`](super::TryStreamExt::try_for_each) method.

#### Implementations

- <span id="tryforeach-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryForEach<St, Fut, F>`

- <span id="tryforeach-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> Future for TryForEach<St, Fut, F>`

- <span id="tryforeach-future-type-output"></span>`type Output = Result<(), <St as TryStream>::Error>`

- <span id="tryforeach-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for TryForEach<St, Fut, F>`

##### `impl<F> IntoFuture for TryForEach<St, Fut, F>`

- <span id="tryforeach-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryforeach-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryforeach-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryForEach<St, Fut, F>`

- <span id="tryforeach-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryforeach-tryfuture-type-error"></span>`type Error = E`

- <span id="tryforeach-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryForEach<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryForEach<St, Fut, F>`

### `TryFilter<St, Fut, F>`

```rust
struct TryFilter<St, Fut, F>
where
    St: TryStream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Ok>,
}
```

Stream for the [`try_filter`](super::TryStreamExt::try_filter)
method.

#### Implementations

- <span id="tryfilter-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryfilter-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryfilter-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfilter-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfilter-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryFilter<St, Fut, F>`

- <span id="tryfilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryFilter<St, Fut, F>`

- <span id="tryfilter-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryFilter<S, Fut, F>`

- <span id="tryfilter-sink-type-error"></span>`type Error = E`

- <span id="tryfilter-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryfilter-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="tryfilter-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryfilter-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFilter<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryFilter<St, Fut, F>`

- <span id="tryfilter-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryfilter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="tryfilter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFilter<St, Fut, F>`

##### `impl TryStream for TryFilter<St, Fut, F>`

- <span id="tryfilter-trystream-type-ok"></span>`type Ok = T`

- <span id="tryfilter-trystream-type-error"></span>`type Error = E`

- <span id="tryfilter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryFilter<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryFilter<St, Fut, F>`

### `TryFilterMap<St, Fut, F>`

```rust
struct TryFilterMap<St, Fut, F> {
    stream: St,
    f: F,
    pending: Option<Fut>,
}
```

Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map)
method.

#### Implementations

- <span id="tryfiltermap-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryfiltermap-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryfiltermap-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfiltermap-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfiltermap-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryFilterMap<S, Fut, F>`

- <span id="tryfiltermap-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryfiltermap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryfiltermap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="tryfiltermap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryfiltermap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-stream-type-item"></span>`type Item = Result<T, <St as TryStream>::Error>`

- <span id="tryfiltermap-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="tryfiltermap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFilterMap<St, Fut, F>`

##### `impl TryStream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-trystream-type-ok"></span>`type Ok = T`

- <span id="tryfiltermap-trystream-type-error"></span>`type Error = E`

- <span id="tryfiltermap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryFilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryFilterMap<St, Fut, F>`

### `TryFlatten<St>`

```rust
struct TryFlatten<St>
where
    St: TryStream {
    stream: St,
    next: Option<<St as >::Ok>,
}
```

Stream for the [`try_flatten`](super::TryStreamExt::try_flatten) method.

#### Implementations

- <span id="tryflatten-new"></span>`fn new(stream: St) -> Self`

- <span id="tryflatten-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryflatten-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryflatten-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryflatten-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for TryFlatten<St>`

- <span id="tryflatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for TryFlatten<St>`

- <span id="tryflatten-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for TryFlatten<S>`

- <span id="tryflatten-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryflatten-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryflatten-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="tryflatten-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryflatten-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlatten<St>`

##### `impl<St> Stream for TryFlatten<St>`

- <span id="tryflatten-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryStream>::Ok, <<St as TryStream>::Ok as TryStream>::Error>`

- <span id="tryflatten-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl StreamExt for TryFlatten<St>`

##### `impl TryStream for TryFlatten<St>`

- <span id="tryflatten-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflatten-trystream-type-error"></span>`type Error = E`

- <span id="tryflatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryFlatten<St>`

##### `impl<St> Unpin for TryFlatten<St>`

### `TryFlattenUnordered<St>`

```rust
struct TryFlattenUnordered<St>
where
    St: TryStream,
    <St as >::Ok: TryStream + Unpin,
    <<St as >::Ok as TryStream>::Error: From<<St as >::Error> {
    inner: crate::stream::stream::flatten_unordered::FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>>,
}
```

Stream for the [`try_flatten_unordered`](super::TryStreamExt::try_flatten_unordered) method.

#### Implementations

- <span id="tryflattenunordered-new"></span>`fn new(stream: St, limit: impl Into<Option<usize>>) -> Self`

#### Trait Implementations

##### `impl<St> Debug for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<St> FusedStream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, St> Sink for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-sink-type-error"></span>`type Error = <FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>> as Sink>::Error`

- <span id="tryflattenunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryflattenunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="tryflattenunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryflattenunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlattenUnordered<St>`

##### `impl<St> Stream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-stream-type-item"></span>`type Item = <FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>> as Stream>::Item`

- <span id="tryflattenunordered-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="tryflattenunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFlattenUnordered<St>`

##### `impl TryStream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflattenunordered-trystream-type-error"></span>`type Error = E`

- <span id="tryflattenunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryFlattenUnordered<St>`

##### `impl<St> Unpin for TryFlattenUnordered<St>`

### `TryCollect<St, C>`

```rust
struct TryCollect<St, C> {
    stream: St,
    items: C,
}
```

Future for the [`try_collect`](super::TryStreamExt::try_collect) method.

#### Implementations

- <span id="trycollect-new"></span>`fn new(s: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, C: fmt::Debug> Debug for TryCollect<St, C>`

- <span id="trycollect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, C> FusedFuture for TryCollect<St, C>`

- <span id="trycollect-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, C> Future for TryCollect<St, C>`

- <span id="trycollect-future-type-output"></span>`type Output = Result<C, <St as TryStream>::Error>`

- <span id="trycollect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for TryCollect<St, C>`

##### `impl IntoFuture for TryCollect<St, C>`

- <span id="trycollect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trycollect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trycollect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryCollect<St, C>`

- <span id="trycollect-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trycollect-tryfuture-type-error"></span>`type Error = E`

- <span id="trycollect-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for TryCollect<St, C>`

##### `impl<St, C> Unpin for TryCollect<St, C>`

### `TryConcat<St: TryStream>`

```rust
struct TryConcat<St: TryStream> {
    stream: St,
    accum: Option<<St as >::Ok>,
}
```

Future for the [`try_concat`](super::TryStreamExt::try_concat) method.

#### Implementations

- <span id="tryconcat-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + TryStream> Debug for TryConcat<St>`

- <span id="tryconcat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> Future for TryConcat<St>`

- <span id="tryconcat-future-type-output"></span>`type Output = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryconcat-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for TryConcat<St>`

##### `impl IntoFuture for TryConcat<St>`

- <span id="tryconcat-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryconcat-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryconcat-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryConcat<St>`

- <span id="tryconcat-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryconcat-tryfuture-type-error"></span>`type Error = E`

- <span id="tryconcat-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for TryConcat<St>`

##### `impl<St: TryStream> Unpin for TryConcat<St>`

### `TryChunks<St: TryStream>`

```rust
struct TryChunks<St: TryStream> {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    items: alloc::vec::Vec<<St as >::Ok>,
    cap: usize,
}
```

Stream for the [`try_chunks`](super::TryStreamExt::try_chunks) method.

#### Implementations

- <span id="trychunks-new"></span>`fn new(stream: St, capacity: usize) -> Self`

- <span id="trychunks-take"></span>`fn take(self: Pin<&mut Self>) -> Vec<<St as >::Ok>` — [`TryStream`](../index.md#trystream)

- <span id="trychunks-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trychunks-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trychunks-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trychunks-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug + TryStream> Debug for TryChunks<St>`

- <span id="trychunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: TryStream + FusedStream> FusedStream for TryChunks<St>`

- <span id="trychunks-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for TryChunks<S>`

- <span id="trychunks-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="trychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="trychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryChunks<St>`

##### `impl<St: TryStream> Stream for TryChunks<St>`

- <span id="trychunks-stream-type-item"></span>`type Item = Result<Vec<<St as TryStream>::Ok>, TryChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>>`

- <span id="trychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="trychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryChunks<St>`

##### `impl TryStream for TryChunks<St>`

- <span id="trychunks-trystream-type-ok"></span>`type Ok = T`

- <span id="trychunks-trystream-type-error"></span>`type Error = E`

- <span id="trychunks-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryChunks<St>`

##### `impl<St: TryStream> Unpin for TryChunks<St>`

### `TryChunksError<T, E>`

```rust
struct TryChunksError<T, E>(alloc::vec::Vec<T>, E);
```

Error indicating, that while chunk was collected inner stream produced an error.

Contains all items that were collected before an error occurred, and the stream error itself.

#### Trait Implementations

##### `impl<T, E: fmt::Debug> Debug for TryChunksError<T, E>`

- <span id="trychunkserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E: fmt::Display> Display for TryChunksError<T, E>`

- <span id="trychunkserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq, E: cmp::Eq> Eq for TryChunksError<T, E>`

##### `impl<T: cmp::PartialEq, E: cmp::PartialEq> PartialEq for TryChunksError<T, E>`

- <span id="trychunkserror-partialeq-eq"></span>`fn eq(&self, other: &TryChunksError<T, E>) -> bool` — [`TryChunksError`](try_chunks/index.md#trychunkserror)

##### `impl<T, E> StructuralPartialEq for TryChunksError<T, E>`

##### `impl<T> ToString for TryChunksError<T, E>`

- <span id="trychunkserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `TryReadyChunks<St: TryStream>`

```rust
struct TryReadyChunks<St: TryStream> {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    cap: usize,
}
```

Stream for the [`try_ready_chunks`](super::TryStreamExt::try_ready_chunks) method.

#### Implementations

- <span id="tryreadychunks-new"></span>`fn new(stream: St, capacity: usize) -> Self`

- <span id="tryreadychunks-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryreadychunks-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryreadychunks-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryreadychunks-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug + TryStream> Debug for TryReadyChunks<St>`

- <span id="tryreadychunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: TryStream + FusedStream> FusedStream for TryReadyChunks<St>`

- <span id="tryreadychunks-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for TryReadyChunks<S>`

- <span id="tryreadychunks-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryreadychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryreadychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="tryreadychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryreadychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryReadyChunks<St>`

##### `impl<St: TryStream> Stream for TryReadyChunks<St>`

- <span id="tryreadychunks-stream-type-item"></span>`type Item = Result<Vec<<St as TryStream>::Ok>, TryReadyChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>>`

- <span id="tryreadychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="tryreadychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryReadyChunks<St>`

##### `impl TryStream for TryReadyChunks<St>`

- <span id="tryreadychunks-trystream-type-ok"></span>`type Ok = T`

- <span id="tryreadychunks-trystream-type-error"></span>`type Error = E`

- <span id="tryreadychunks-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryReadyChunks<St>`

##### `impl<St: TryStream> Unpin for TryReadyChunks<St>`

### `TryReadyChunksError<T, E>`

```rust
struct TryReadyChunksError<T, E>(alloc::vec::Vec<T>, E);
```

Error indicating, that while chunk was collected inner stream produced an error.

Contains all items that were collected before an error occurred, and the stream error itself.

#### Trait Implementations

##### `impl<T, E: fmt::Debug> Debug for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, E: fmt::Display> Display for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq, E: cmp::Eq> Eq for TryReadyChunksError<T, E>`

##### `impl<T: cmp::PartialEq, E: cmp::PartialEq> PartialEq for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-partialeq-eq"></span>`fn eq(&self, other: &TryReadyChunksError<T, E>) -> bool` — [`TryReadyChunksError`](try_ready_chunks/index.md#tryreadychunkserror)

##### `impl<T, E> StructuralPartialEq for TryReadyChunksError<T, E>`

##### `impl<T> ToString for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `TryFold<St, Fut, T, F>`

```rust
struct TryFold<St, Fut, T, F> {
    stream: St,
    f: F,
    accum: Option<T>,
    future: Option<Fut>,
}
```

Future for the [`try_fold`](super::TryStreamExt::try_fold) method.

#### Implementations

- <span id="tryfold-new"></span>`fn new(stream: St, f: F, t: T) -> Self`

#### Trait Implementations

##### `impl<St, Fut, T, F> Debug for TryFold<St, Fut, T, F>`

- <span id="tryfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, T, F> FusedFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, T, F> Future for TryFold<St, Fut, T, F>`

- <span id="tryfold-future-type-output"></span>`type Output = Result<T, <St as TryStream>::Error>`

- <span id="tryfold-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<T> FutureExt for TryFold<St, Fut, T, F>`

##### `impl<F> IntoFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryfold-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryfold-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryfold-tryfuture-type-error"></span>`type Error = E`

- <span id="tryfold-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryFold<St, Fut, T, F>`

##### `impl<St, Fut, T, F> Unpin for TryFold<St, Fut, T, F>`

### `TryUnfold<T, F, Fut>`

```rust
struct TryUnfold<T, F, Fut> {
    f: F,
    state: Option<T>,
    fut: Option<Fut>,
}
```

Stream for the [`try_unfold`](try_unfold/index.md) function.

#### Trait Implementations

##### `impl<T, F, Fut> Debug for TryUnfold<T, F, Fut>`

- <span id="tryunfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F, Fut> Stream for TryUnfold<T, F, Fut>`

- <span id="tryunfold-stream-type-item"></span>`type Item = Result<Item, <Fut as TryFuture>::Error>`

- <span id="tryunfold-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl<T> StreamExt for TryUnfold<T, F, Fut>`

##### `impl<T> TryStream for TryUnfold<T, F, Fut>`

- <span id="tryunfold-trystream-type-ok"></span>`type Ok = T`

- <span id="tryunfold-trystream-type-error"></span>`type Error = E`

- <span id="tryunfold-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryUnfold<T, F, Fut>`

##### `impl<T, F, Fut> Unpin for TryUnfold<T, F, Fut>`

### `TrySkipWhile<St, Fut, F>`

```rust
struct TrySkipWhile<St, Fut, F>
where
    St: TryStream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Ok>,
    done_skipping: bool,
}
```

Stream for the [`try_skip_while`](super::TryStreamExt::try_skip_while)
method.

#### Implementations

- <span id="tryskipwhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryskipwhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryskipwhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryskipwhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryskipwhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TrySkipWhile<S, Fut, F>`

- <span id="tryskipwhile-sink-type-error"></span>`type Error = E`

- <span id="tryskipwhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryskipwhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="tryskipwhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryskipwhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TrySkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryskipwhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="tryskipwhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TrySkipWhile<St, Fut, F>`

##### `impl TryStream for TrySkipWhile<St, Fut, F>`

- <span id="tryskipwhile-trystream-type-ok"></span>`type Ok = T`

- <span id="tryskipwhile-trystream-type-error"></span>`type Error = E`

- <span id="tryskipwhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TrySkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TrySkipWhile<St, Fut, F>`

### `TryTakeWhile<St, Fut, F>`

```rust
struct TryTakeWhile<St, Fut, F>
where
    St: TryStream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Ok>,
    done_taking: bool,
}
```

Stream for the [`try_take_while`](super::TryStreamExt::try_take_while)
method.

#### Implementations

- <span id="trytakewhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="trytakewhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trytakewhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trytakewhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trytakewhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryTakeWhile<S, Fut, F>`

- <span id="trytakewhile-sink-type-error"></span>`type Error = E`

- <span id="trytakewhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trytakewhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="trytakewhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trytakewhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryTakeWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="trytakewhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="trytakewhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryTakeWhile<St, Fut, F>`

##### `impl TryStream for TryTakeWhile<St, Fut, F>`

- <span id="trytakewhile-trystream-type-ok"></span>`type Ok = T`

- <span id="trytakewhile-trystream-type-error"></span>`type Error = E`

- <span id="trytakewhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryTakeWhile<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryTakeWhile<St, Fut, F>`

### `TryBufferUnordered<St>`

```rust
struct TryBufferUnordered<St>
where
    St: TryStream {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    in_progress_queue: crate::stream::FuturesUnordered<crate::future::IntoFuture<<St as >::Ok>>,
    max: usize,
}
```

Stream for the
[`try_buffer_unordered`](super::TryStreamExt::try_buffer_unordered) method.

#### Implementations

- <span id="trybufferunordered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="trybufferunordered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trybufferunordered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybufferunordered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybufferunordered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for TryBufferUnordered<St>`

- <span id="trybufferunordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item> Sink for TryBufferUnordered<S>`

- <span id="trybufferunordered-sink-type-error"></span>`type Error = E`

- <span id="trybufferunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trybufferunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="trybufferunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trybufferunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryBufferUnordered<St>`

##### `impl<St> Stream for TryBufferUnordered<St>`

- <span id="trybufferunordered-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="trybufferunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl StreamExt for TryBufferUnordered<St>`

##### `impl TryStream for TryBufferUnordered<St>`

- <span id="trybufferunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="trybufferunordered-trystream-type-error"></span>`type Error = E`

- <span id="trybufferunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryBufferUnordered<St>`

##### `impl<St> Unpin for TryBufferUnordered<St>`

### `TryBuffered<St>`

```rust
struct TryBuffered<St>
where
    St: TryStream,
    <St as >::Ok: TryFuture {
    stream: crate::stream::Fuse<crate::stream::IntoStream<St>>,
    in_progress_queue: crate::stream::FuturesOrdered<crate::future::IntoFuture<<St as >::Ok>>,
    max: usize,
}
```

Stream for the [`try_buffered`](super::TryStreamExt::try_buffered) method.

#### Implementations

- <span id="trybuffered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="trybuffered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="trybuffered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybuffered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="trybuffered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for TryBuffered<St>`

- <span id="trybuffered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item> Sink for TryBuffered<S>`

- <span id="trybuffered-sink-type-error"></span>`type Error = E`

- <span id="trybuffered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trybuffered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="trybuffered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="trybuffered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryBuffered<St>`

##### `impl<St> Stream for TryBuffered<St>`

- <span id="trybuffered-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="trybuffered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

##### `impl StreamExt for TryBuffered<St>`

##### `impl TryStream for TryBuffered<St>`

- <span id="trybuffered-trystream-type-ok"></span>`type Ok = T`

- <span id="trybuffered-trystream-type-error"></span>`type Error = E`

- <span id="trybuffered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for TryBuffered<St>`

##### `impl<St> Unpin for TryBuffered<St>`

### `TryForEachConcurrent<St, Fut, F>`

```rust
struct TryForEachConcurrent<St, Fut, F> {
    stream: Option<St>,
    f: F,
    futures: crate::stream::FuturesUnordered<Fut>,
    limit: Option<core::num::NonZeroUsize>,
}
```

Future for the
[`try_for_each_concurrent`](super::TryStreamExt::try_for_each_concurrent)
method.

#### Implementations

- <span id="tryforeachconcurrent-new"></span>`fn new(stream: St, limit: Option<usize>, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-future-type-output"></span>`type Output = Result<(), <St as TryStream>::Error>`

- <span id="tryforeachconcurrent-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for TryForEachConcurrent<St, Fut, F>`

##### `impl<F> IntoFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryforeachconcurrent-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryforeachconcurrent-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryforeachconcurrent-tryfuture-type-error"></span>`type Error = E`

- <span id="tryforeachconcurrent-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryForEachConcurrent<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryForEachConcurrent<St, Fut, F>`

### `TryAll<St, Fut, F>`

```rust
struct TryAll<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`try_all`](super::TryStreamExt::try_all) method.

#### Implementations

- <span id="tryall-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryAll<St, Fut, F>`

- <span id="tryall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryAll<St, Fut, F>`

- <span id="tryall-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryAll<St, Fut, F>`

- <span id="tryall-future-type-output"></span>`type Output = Result<bool, <St as TryStream>::Error>`

- <span id="tryall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<bool, <St as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl FutureExt for TryAll<St, Fut, F>`

##### `impl<F> IntoFuture for TryAll<St, Fut, F>`

- <span id="tryall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryAll<St, Fut, F>`

- <span id="tryall-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryall-tryfuture-type-error"></span>`type Error = E`

- <span id="tryall-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryAll<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryAll<St, Fut, F>`

### `TryAny<St, Fut, F>`

```rust
struct TryAny<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`try_any`](super::TryStreamExt::try_any) method.

#### Implementations

- <span id="tryany-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryAny<St, Fut, F>`

- <span id="tryany-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryAny<St, Fut, F>`

- <span id="tryany-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryAny<St, Fut, F>`

- <span id="tryany-future-type-output"></span>`type Output = Result<bool, <St as TryStream>::Error>`

- <span id="tryany-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<bool, <St as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl FutureExt for TryAny<St, Fut, F>`

##### `impl<F> IntoFuture for TryAny<St, Fut, F>`

- <span id="tryany-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryany-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryany-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryAny<St, Fut, F>`

- <span id="tryany-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryany-tryfuture-type-error"></span>`type Error = E`

- <span id="tryany-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryAny<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryAny<St, Fut, F>`

## Traits

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

  Wraps a [`TryStream`](../index.md) into a type that implements

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

## Functions

### `try_unfold`

```rust
fn try_unfold<T, F, Fut, Item>(init: T, f: F) -> TryUnfold<T, F, Fut>
where
    F: FnMut(T) -> Fut,
    Fut: TryFuture<Ok = Option<(Item, T)>>
```

Creates a `TryStream` from a seed and a closure returning a `TryFuture`.

This function is the dual for the `TryStream::try_fold()` adapter: while
`TryStream::try_fold()` reduces a `TryStream` to one single value,
`try_unfold()` creates a `TryStream` from a seed value.

`try_unfold()` will call the provided closure with the provided seed, then
wait for the returned `TryFuture` to complete with `(a, b)`. It will then
yield the value `a`, and use `b` as the next internal state.

If the closure returns `None` instead of `Some(TryFuture)`, then the
`try_unfold()` will stop producing items and return `Poll::Ready(None)` in
future calls to `poll()`.

In case of error generated by the returned `TryFuture`, the error will be
returned by the `TryStream`. The `TryStream` will then yield
`Poll::Ready(None)` in future calls to `poll()`.

This function can typically be used when wanting to go from the "world of
futures" to the "world of streams": the provided closure can build a
`TryFuture` using other library functions working on futures, and
`try_unfold()` will turn it into a `TryStream` by repeating the operation.

# Example

```rust
#[derive(Debug, PartialEq)]
struct SomeError;
futures::executor::block_on(async {
use futures::stream::{self, TryStreamExt};

let stream = stream::try_unfold(0, |state| async move {
    if state < 0 {
        return Err(SomeError);
    }

    if state <= 2 {
        let next_state = state + 1;
        let yielded = state * 2;
        Ok(Some((yielded, next_state)))
    } else {
        Ok(None)
    }
});

let result: Result<Vec<i32>, _> = stream.try_collect().await;
assert_eq!(result, Ok(vec![0, 2, 4]));
});
```

