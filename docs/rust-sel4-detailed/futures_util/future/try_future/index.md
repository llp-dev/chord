*[futures_util](../../index.md) / [future](../index.md) / [try_future](index.md)*

---

# Module `try_future`

Futures

This module contains a number of functions for working with `Future`s,
including the `FutureExt` trait which adds methods to `Future` types.

## Contents

- [Modules](#modules)
  - [`into_future`](#into-future)
  - [`try_flatten`](#try-flatten)
  - [`try_flatten_err`](#try-flatten-err)
- [Structs](#structs)
  - [`TryFlatten`](#tryflatten)
  - [`TryFlattenErr`](#tryflattenerr)
  - [`TryFlattenStream`](#tryflattenstream)
  - [`FlattenSink`](#flattensink)
  - [`AndThen`](#andthen)
  - [`OrElse`](#orelse)
  - [`ErrInto`](#errinto)
  - [`OkInto`](#okinto)
  - [`InspectOk`](#inspectok)
  - [`InspectErr`](#inspecterr)
  - [`IntoFuture`](#intofuture)
  - [`MapOk`](#mapok)
  - [`MapErr`](#maperr)
  - [`MapOkOrElse`](#mapokorelse)
  - [`UnwrapOrElse`](#unwraporelse)
- [Traits](#traits)
  - [`TryFutureExt`](#tryfutureext)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`into_future`](#into-future) | mod |  |
| [`try_flatten`](#try-flatten) | mod |  |
| [`try_flatten_err`](#try-flatten-err) | mod |  |
| [`TryFlatten`](#tryflatten) | struct | Future for the [`try_flatten`](TryFutureExt::try_flatten) method. |
| [`TryFlattenErr`](#tryflattenerr) | struct | Future for the [`try_flatten_err`](TryFutureExt::try_flatten_err) method. |
| [`TryFlattenStream`](#tryflattenstream) | struct | Future for the [`try_flatten_stream`](TryFutureExt::try_flatten_stream) method. |
| [`FlattenSink`](#flattensink) | struct | Sink for the [`flatten_sink`](TryFutureExt::flatten_sink) method. |
| [`AndThen`](#andthen) | struct | Future for the [`and_then`](TryFutureExt::and_then) method. |
| [`OrElse`](#orelse) | struct | Future for the [`or_else`](TryFutureExt::or_else) method. |
| [`ErrInto`](#errinto) | struct | Future for the [`err_into`](TryFutureExt::err_into) method. |
| [`OkInto`](#okinto) | struct | Future for the [`ok_into`](TryFutureExt::ok_into) method. |
| [`InspectOk`](#inspectok) | struct | Future for the [`inspect_ok`](super::TryFutureExt::inspect_ok) method. |
| [`InspectErr`](#inspecterr) | struct | Future for the [`inspect_err`](super::TryFutureExt::inspect_err) method. |
| [`IntoFuture`](#intofuture) | struct |  |
| [`MapOk`](#mapok) | struct | Future for the [`map_ok`](TryFutureExt::map_ok) method. |
| [`MapErr`](#maperr) | struct | Future for the [`map_err`](TryFutureExt::map_err) method. |
| [`MapOkOrElse`](#mapokorelse) | struct | Future for the [`map_ok_or_else`](TryFutureExt::map_ok_or_else) method. |
| [`UnwrapOrElse`](#unwraporelse) | struct | Future for the [`unwrap_or_else`](TryFutureExt::unwrap_or_else) method. |
| [`TryFutureExt`](#tryfutureext) | trait | Adapters specific to [`Result`]-returning futures |

## Modules

- [`into_future`](into_future/index.md)
- [`try_flatten`](try_flatten/index.md)
- [`try_flatten_err`](try_flatten_err/index.md)

## Structs

### `TryFlatten<Fut1, Fut2>`

```rust
struct TryFlatten<Fut1, Fut2> {
    inner: try_flatten::TryFlatten<Fut1, Fut2>,
}
```

Future for the [`try_flatten`](TryFutureExt::try_flatten) method.

#### Implementations

- <span id="tryflatten-new"></span>`fn new(x: Fut1) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2> Debug for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut1, Fut2> FusedFuture for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1, Fut2> Future for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-future-type-output"></span>`type Output = <TryFlatten<Fut1, Fut2> as Future>::Output`

- <span id="tryflatten-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryFlatten<Fut1, Fut2>`

##### `impl IntoFuture for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryflatten-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryflatten-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryflatten-tryfuture-type-error"></span>`type Error = E`

- <span id="tryflatten-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TryFlatten<Fut1, Fut2>`

##### `impl<Fut1, Fut2> Unpin for TryFlatten<Fut1, Fut2>`

### `TryFlattenErr<Fut1, Fut2>`

```rust
struct TryFlattenErr<Fut1, Fut2> {
    inner: try_flatten_err::TryFlattenErr<Fut1, Fut2>,
}
```

Future for the [`try_flatten_err`](TryFutureExt::try_flatten_err) method.

#### Implementations

- <span id="tryflattenerr-new"></span>`fn new(x: Fut1) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2> Debug for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut1, Fut2> FusedFuture for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1, Fut2> Future for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-future-type-output"></span>`type Output = <TryFlattenErr<Fut1, Fut2> as Future>::Output`

- <span id="tryflattenerr-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryFlattenErr<Fut1, Fut2>`

##### `impl IntoFuture for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryflattenerr-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryflattenerr-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryFlattenErr<Fut1, Fut2>`

- <span id="tryflattenerr-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryflattenerr-tryfuture-type-error"></span>`type Error = E`

- <span id="tryflattenerr-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TryFlattenErr<Fut1, Fut2>`

##### `impl<Fut1, Fut2> Unpin for TryFlattenErr<Fut1, Fut2>`

### `TryFlattenStream<Fut>`

```rust
struct TryFlattenStream<Fut>
where
    Fut: TryFuture {
    inner: try_flatten::TryFlatten<Fut, <Fut as >::Ok>,
}
```

Future for the [`try_flatten_stream`](TryFutureExt::try_flatten_stream) method.

#### Implementations

- <span id="tryflattenstream-new"></span>`fn new(x: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut> Debug for TryFlattenStream<Fut>`

- <span id="tryflattenstream-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut> FusedStream for TryFlattenStream<Fut>`

- <span id="tryflattenstream-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, Fut> Sink for TryFlattenStream<Fut>`

- <span id="tryflattenstream-sink-type-error"></span>`type Error = <TryFlatten<Fut, <Fut as TryFuture>::Ok> as Sink>::Error`

- <span id="tryflattenstream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryflattenstream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="tryflattenstream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="tryflattenstream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlattenStream<Fut>`

##### `impl<Fut> Stream for TryFlattenStream<Fut>`

- <span id="tryflattenstream-stream-type-item"></span>`type Item = <TryFlatten<Fut, <Fut as TryFuture>::Ok> as Stream>::Item`

- <span id="tryflattenstream-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="tryflattenstream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFlattenStream<Fut>`

##### `impl TryStream for TryFlattenStream<Fut>`

- <span id="tryflattenstream-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflattenstream-trystream-type-error"></span>`type Error = E`

- <span id="tryflattenstream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for TryFlattenStream<Fut>`

##### `impl<Fut> Unpin for TryFlattenStream<Fut>`

### `FlattenSink<Fut, Si>`

```rust
struct FlattenSink<Fut, Si> {
    inner: try_flatten::TryFlatten<Fut, Si>,
}
```

Sink for the [`flatten_sink`](TryFutureExt::flatten_sink) method.

#### Implementations

- <span id="flattensink-new"></span>`fn new(x: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut, Si> Debug for FlattenSink<Fut, Si>`

- <span id="flattensink-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, Si> FusedStream for FlattenSink<Fut, Si>`

- <span id="flattensink-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, Fut, Si> Sink for FlattenSink<Fut, Si>`

- <span id="flattensink-sink-type-error"></span>`type Error = <TryFlatten<Fut, Si> as Sink>::Error`

- <span id="flattensink-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flattensink-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="flattensink-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flattensink-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for FlattenSink<Fut, Si>`

##### `impl<Fut, Si> Stream for FlattenSink<Fut, Si>`

- <span id="flattensink-stream-type-item"></span>`type Item = <TryFlatten<Fut, Si> as Stream>::Item`

- <span id="flattensink-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="flattensink-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlattenSink<Fut, Si>`

##### `impl TryStream for FlattenSink<Fut, Si>`

- <span id="flattensink-trystream-type-ok"></span>`type Ok = T`

- <span id="flattensink-trystream-type-error"></span>`type Error = E`

- <span id="flattensink-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for FlattenSink<Fut, Si>`

##### `impl<Fut, Si> Unpin for FlattenSink<Fut, Si>`

### `AndThen<Fut1, Fut2, F>`

```rust
struct AndThen<Fut1, Fut2, F> {
    inner: TryFlatten<MapOk<Fut1, F>, Fut2>,
}
```

Future for the [`and_then`](TryFutureExt::and_then) method.

#### Implementations

- <span id="andthen-new"></span>`fn new(x: Fut1, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, F> Debug for AndThen<Fut1, Fut2, F>`

- <span id="andthen-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut1, Fut2, F> FusedFuture for AndThen<Fut1, Fut2, F>`

- <span id="andthen-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1, Fut2, F> Future for AndThen<Fut1, Fut2, F>`

- <span id="andthen-future-type-output"></span>`type Output = <TryFlatten<MapOk<Fut1, F>, Fut2> as Future>::Output`

- <span id="andthen-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for AndThen<Fut1, Fut2, F>`

##### `impl<F> IntoFuture for AndThen<Fut1, Fut2, F>`

- <span id="andthen-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="andthen-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="andthen-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for AndThen<Fut1, Fut2, F>`

- <span id="andthen-tryfuture-type-ok"></span>`type Ok = T`

- <span id="andthen-tryfuture-type-error"></span>`type Error = E`

- <span id="andthen-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for AndThen<Fut1, Fut2, F>`

##### `impl<Fut1, Fut2, F> Unpin for AndThen<Fut1, Fut2, F>`

### `OrElse<Fut1, Fut2, F>`

```rust
struct OrElse<Fut1, Fut2, F> {
    inner: TryFlattenErr<MapErr<Fut1, F>, Fut2>,
}
```

Future for the [`or_else`](TryFutureExt::or_else) method.

#### Implementations

- <span id="orelse-new"></span>`fn new(x: Fut1, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, F> Debug for OrElse<Fut1, Fut2, F>`

- <span id="orelse-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut1, Fut2, F> FusedFuture for OrElse<Fut1, Fut2, F>`

- <span id="orelse-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1, Fut2, F> Future for OrElse<Fut1, Fut2, F>`

- <span id="orelse-future-type-output"></span>`type Output = <TryFlattenErr<MapErr<Fut1, F>, Fut2> as Future>::Output`

- <span id="orelse-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for OrElse<Fut1, Fut2, F>`

##### `impl<F> IntoFuture for OrElse<Fut1, Fut2, F>`

- <span id="orelse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="orelse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="orelse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for OrElse<Fut1, Fut2, F>`

- <span id="orelse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="orelse-tryfuture-type-error"></span>`type Error = E`

- <span id="orelse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for OrElse<Fut1, Fut2, F>`

##### `impl<Fut1, Fut2, F> Unpin for OrElse<Fut1, Fut2, F>`

### `ErrInto<Fut, E>`

```rust
struct ErrInto<Fut, E> {
    inner: MapErr<Fut, crate::fns::IntoFn<E>>,
}
```

Future for the [`err_into`](TryFutureExt::err_into) method.

#### Implementations

- <span id="errinto-new"></span>`fn new(x: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut, E> Debug for ErrInto<Fut, E>`

- <span id="errinto-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, E> FusedFuture for ErrInto<Fut, E>`

- <span id="errinto-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, E> Future for ErrInto<Fut, E>`

- <span id="errinto-future-type-output"></span>`type Output = <MapErr<Fut, IntoFn<E>> as Future>::Output`

- <span id="errinto-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for ErrInto<Fut, E>`

##### `impl IntoFuture for ErrInto<Fut, E>`

- <span id="errinto-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="errinto-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="errinto-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<E> TryFuture for ErrInto<Fut, E>`

- <span id="errinto-tryfuture-type-ok"></span>`type Ok = T`

- <span id="errinto-tryfuture-type-error"></span>`type Error = E`

- <span id="errinto-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for ErrInto<Fut, E>`

##### `impl<Fut, E> Unpin for ErrInto<Fut, E>`

### `OkInto<Fut, E>`

```rust
struct OkInto<Fut, E> {
    inner: MapOk<Fut, crate::fns::IntoFn<E>>,
}
```

Future for the [`ok_into`](TryFutureExt::ok_into) method.

#### Implementations

- <span id="okinto-new"></span>`fn new(x: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut, E> Debug for OkInto<Fut, E>`

- <span id="okinto-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, E> FusedFuture for OkInto<Fut, E>`

- <span id="okinto-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, E> Future for OkInto<Fut, E>`

- <span id="okinto-future-type-output"></span>`type Output = <MapOk<Fut, IntoFn<E>> as Future>::Output`

- <span id="okinto-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for OkInto<Fut, E>`

##### `impl IntoFuture for OkInto<Fut, E>`

- <span id="okinto-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="okinto-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="okinto-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<E> TryFuture for OkInto<Fut, E>`

- <span id="okinto-tryfuture-type-ok"></span>`type Ok = T`

- <span id="okinto-tryfuture-type-error"></span>`type Error = E`

- <span id="okinto-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for OkInto<Fut, E>`

##### `impl<Fut, E> Unpin for OkInto<Fut, E>`

### `InspectOk<Fut, F>`

```rust
struct InspectOk<Fut, F> {
    inner: crate::future::Inspect<IntoFuture<Fut>, crate::fns::InspectOkFn<F>>,
}
```

Future for the [`inspect_ok`](super::TryFutureExt::inspect_ok) method.

#### Implementations

- <span id="inspectok-new"></span>`fn new(x: Fut, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut, F> Debug for InspectOk<Fut, F>`

- <span id="inspectok-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F> FusedFuture for InspectOk<Fut, F>`

- <span id="inspectok-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for InspectOk<Fut, F>`

- <span id="inspectok-future-type-output"></span>`type Output = <Inspect<IntoFuture<Fut>, InspectOkFn<F>> as Future>::Output`

- <span id="inspectok-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for InspectOk<Fut, F>`

##### `impl<F> IntoFuture for InspectOk<Fut, F>`

- <span id="inspectok-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="inspectok-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="inspectok-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for InspectOk<Fut, F>`

- <span id="inspectok-tryfuture-type-ok"></span>`type Ok = T`

- <span id="inspectok-tryfuture-type-error"></span>`type Error = E`

- <span id="inspectok-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for InspectOk<Fut, F>`

##### `impl<Fut, F> Unpin for InspectOk<Fut, F>`

### `InspectErr<Fut, F>`

```rust
struct InspectErr<Fut, F> {
    inner: crate::future::Inspect<IntoFuture<Fut>, crate::fns::InspectErrFn<F>>,
}
```

Future for the [`inspect_err`](super::TryFutureExt::inspect_err) method.

#### Implementations

- <span id="inspecterr-new"></span>`fn new(x: Fut, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut, F> Debug for InspectErr<Fut, F>`

- <span id="inspecterr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F> FusedFuture for InspectErr<Fut, F>`

- <span id="inspecterr-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for InspectErr<Fut, F>`

- <span id="inspecterr-future-type-output"></span>`type Output = <Inspect<IntoFuture<Fut>, InspectErrFn<F>> as Future>::Output`

- <span id="inspecterr-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for InspectErr<Fut, F>`

##### `impl<F> IntoFuture for InspectErr<Fut, F>`

- <span id="inspecterr-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="inspecterr-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="inspecterr-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for InspectErr<Fut, F>`

- <span id="inspecterr-tryfuture-type-ok"></span>`type Ok = T`

- <span id="inspecterr-tryfuture-type-error"></span>`type Error = E`

- <span id="inspecterr-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for InspectErr<Fut, F>`

##### `impl<Fut, F> Unpin for InspectErr<Fut, F>`

### `IntoFuture<Fut>`

```rust
struct IntoFuture<Fut> {
    future: Fut,
}
```

Future for the [`into_future`](super::TryFutureExt::into_future) method.

#### Implementations

- <span id="intofuture-new"></span>`fn new(future: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for IntoFuture<Fut>`

- <span id="intofuture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: TryFuture + FusedFuture> FusedFuture for IntoFuture<Fut>`

- <span id="intofuture-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: TryFuture> Future for IntoFuture<Fut>`

- <span id="intofuture-future-type-output"></span>`type Output = Result<<Fut as TryFuture>::Ok, <Fut as TryFuture>::Error>`

- <span id="intofuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for IntoFuture<Fut>`

##### `impl IntoFuture for IntoFuture<Fut>`

- <span id="intofuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="intofuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="intofuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for IntoFuture<Fut>`

- <span id="intofuture-tryfuture-type-ok"></span>`type Ok = T`

- <span id="intofuture-tryfuture-type-error"></span>`type Error = E`

- <span id="intofuture-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for IntoFuture<Fut>`

##### `impl<Fut> Unpin for IntoFuture<Fut>`

### `MapOk<Fut, F>`

```rust
struct MapOk<Fut, F> {
    inner: crate::future::Map<IntoFuture<Fut>, crate::fns::MapOkFn<F>>,
}
```

Future for the [`map_ok`](TryFutureExt::map_ok) method.

#### Implementations

- <span id="mapok-new"></span>`fn new(x: Fut, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut, F> Debug for MapOk<Fut, F>`

- <span id="mapok-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F> FusedFuture for MapOk<Fut, F>`

- <span id="mapok-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for MapOk<Fut, F>`

- <span id="mapok-future-type-output"></span>`type Output = <Map<IntoFuture<Fut>, MapOkFn<F>> as Future>::Output`

- <span id="mapok-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for MapOk<Fut, F>`

##### `impl<F> IntoFuture for MapOk<Fut, F>`

- <span id="mapok-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="mapok-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="mapok-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for MapOk<Fut, F>`

- <span id="mapok-tryfuture-type-ok"></span>`type Ok = T`

- <span id="mapok-tryfuture-type-error"></span>`type Error = E`

- <span id="mapok-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for MapOk<Fut, F>`

##### `impl<Fut, F> Unpin for MapOk<Fut, F>`

### `MapErr<Fut, F>`

```rust
struct MapErr<Fut, F> {
    inner: crate::future::Map<IntoFuture<Fut>, crate::fns::MapErrFn<F>>,
}
```

Future for the [`map_err`](TryFutureExt::map_err) method.

#### Implementations

- <span id="maperr-new"></span>`fn new(x: Fut, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut, F> Debug for MapErr<Fut, F>`

- <span id="maperr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F> FusedFuture for MapErr<Fut, F>`

- <span id="maperr-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for MapErr<Fut, F>`

- <span id="maperr-future-type-output"></span>`type Output = <Map<IntoFuture<Fut>, MapErrFn<F>> as Future>::Output`

- <span id="maperr-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for MapErr<Fut, F>`

##### `impl<F> IntoFuture for MapErr<Fut, F>`

- <span id="maperr-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="maperr-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="maperr-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for MapErr<Fut, F>`

- <span id="maperr-tryfuture-type-ok"></span>`type Ok = T`

- <span id="maperr-tryfuture-type-error"></span>`type Error = E`

- <span id="maperr-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for MapErr<Fut, F>`

##### `impl<Fut, F> Unpin for MapErr<Fut, F>`

### `MapOkOrElse<Fut, F, G>`

```rust
struct MapOkOrElse<Fut, F, G> {
    inner: crate::future::Map<IntoFuture<Fut>, ChainFn<MapOkFn<F>, ChainFn<MapErrFn<G>, MergeResultFn>>>,
}
```

Future for the [`map_ok_or_else`](TryFutureExt::map_ok_or_else) method.

#### Implementations

- <span id="mapokorelse-new"></span>`fn new(x: Fut, f: F, g: G) -> Self`

#### Trait Implementations

##### `impl<Fut, F, G> Debug for MapOkOrElse<Fut, F, G>`

- <span id="mapokorelse-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F, G> FusedFuture for MapOkOrElse<Fut, F, G>`

- <span id="mapokorelse-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F, G> Future for MapOkOrElse<Fut, F, G>`

- <span id="mapokorelse-future-type-output"></span>`type Output = <Map<IntoFuture<Fut>, ChainFn<MapOkFn<F>, ChainFn<MapErrFn<G>, MergeResultFn>>> as Future>::Output`

- <span id="mapokorelse-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for MapOkOrElse<Fut, F, G>`

##### `impl<F> IntoFuture for MapOkOrElse<Fut, F, G>`

- <span id="mapokorelse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="mapokorelse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="mapokorelse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for MapOkOrElse<Fut, F, G>`

- <span id="mapokorelse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="mapokorelse-tryfuture-type-error"></span>`type Error = E`

- <span id="mapokorelse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for MapOkOrElse<Fut, F, G>`

##### `impl<Fut, F, G> Unpin for MapOkOrElse<Fut, F, G>`

### `UnwrapOrElse<Fut, F>`

```rust
struct UnwrapOrElse<Fut, F> {
    inner: crate::future::Map<IntoFuture<Fut>, crate::fns::UnwrapOrElseFn<F>>,
}
```

Future for the [`unwrap_or_else`](TryFutureExt::unwrap_or_else) method.

#### Implementations

- <span id="unwraporelse-new"></span>`fn new(x: Fut, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut, F> Debug for UnwrapOrElse<Fut, F>`

- <span id="unwraporelse-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F> FusedFuture for UnwrapOrElse<Fut, F>`

- <span id="unwraporelse-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for UnwrapOrElse<Fut, F>`

- <span id="unwraporelse-future-type-output"></span>`type Output = <Map<IntoFuture<Fut>, UnwrapOrElseFn<F>> as Future>::Output`

- <span id="unwraporelse-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for UnwrapOrElse<Fut, F>`

##### `impl<F> IntoFuture for UnwrapOrElse<Fut, F>`

- <span id="unwraporelse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="unwraporelse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="unwraporelse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for UnwrapOrElse<Fut, F>`

- <span id="unwraporelse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="unwraporelse-tryfuture-type-error"></span>`type Error = E`

- <span id="unwraporelse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for UnwrapOrElse<Fut, F>`

##### `impl<Fut, F> Unpin for UnwrapOrElse<Fut, F>`

## Traits

### `TryFutureExt`

```rust
trait TryFutureExt: TryFuture { ... }
```

Adapters specific to [`Result`](../../../sel4/error/index.md)-returning futures

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

  Wraps a [`TryFuture`](../index.md) into a type that implements

- `fn try_poll_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Result<<Self as >::Ok, <Self as >::Error>>`

  A convenience method for calling `TryFuture::try_poll` on `Unpin`

#### Implementors

- `Fut`

