*[futures_util](../../index.md) / [future](../index.md) / [future](index.md)*

---

# Module `future`

Futures

This module contains a number of functions for working with `Future`s,
including the `FutureExt` trait which adds methods to `Future` types.

## Contents

- [Modules](#modules)
  - [`flatten`](#flatten)
  - [`fuse`](#fuse)
  - [`map`](#map)
- [Structs](#structs)
  - [`Flatten`](#flatten)
  - [`FlattenStream`](#flattenstream)
  - [`Fuse`](#fuse)
  - [`Map`](#map)
  - [`IntoStream`](#intostream)
  - [`MapInto`](#mapinto)
  - [`Then`](#then)
  - [`Inspect`](#inspect)
  - [`NeverError`](#nevererror)
  - [`UnitError`](#uniterror)
- [Traits](#traits)
  - [`FutureExt`](#futureext)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`flatten`](#flatten) | mod |  |
| [`fuse`](#fuse) | mod |  |
| [`map`](#map) | mod |  |
| [`Flatten`](#flatten) | struct | Future for the [`flatten`](super::FutureExt::flatten) method. |
| [`FlattenStream`](#flattenstream) | struct | Stream for the [`flatten_stream`](FutureExt::flatten_stream) method. |
| [`Fuse`](#fuse) | struct |  |
| [`Map`](#map) | struct | Future for the [`map`](super::FutureExt::map) method. |
| [`IntoStream`](#intostream) | struct | Stream for the [`into_stream`](FutureExt::into_stream) method. |
| [`MapInto`](#mapinto) | struct | Future for the [`map_into`](FutureExt::map_into) combinator. |
| [`Then`](#then) | struct | Future for the [`then`](FutureExt::then) method. |
| [`Inspect`](#inspect) | struct | Future for the [`inspect`](FutureExt::inspect) method. |
| [`NeverError`](#nevererror) | struct | Future for the [`never_error`](super::FutureExt::never_error) combinator. |
| [`UnitError`](#uniterror) | struct | Future for the [`unit_error`](super::FutureExt::unit_error) combinator. |
| [`FutureExt`](#futureext) | trait | An extension trait for `Future`s that provides a variety of convenient adapters. |

## Modules

- [`flatten`](flatten/index.md)
- [`fuse`](fuse/index.md)
- [`map`](map/index.md)

## Structs

### `Flatten<F>`

```rust
struct Flatten<F>
where
    F: Future {
    inner: flatten::Flatten<F, <F as Future>::Output>,
}
```

Future for the [`flatten`](super::FutureExt::flatten) method.

#### Implementations

- <span id="flatten-new"></span>`fn new(x: F) -> Self`

#### Trait Implementations

##### `impl<F> Debug for Flatten<F>`

- <span id="flatten-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<F> FusedFuture for Flatten<F>`

- <span id="flatten-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F> Future for Flatten<F>`

- <span id="flatten-future-type-output"></span>`type Output = <Flatten<F, <F as Future>::Output> as Future>::Output`

- <span id="flatten-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Flatten<F>`

##### `impl<F> IntoFuture for Flatten<F>`

- <span id="flatten-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="flatten-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="flatten-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Flatten<F>`

- <span id="flatten-tryfuture-type-ok"></span>`type Ok = T`

- <span id="flatten-tryfuture-type-error"></span>`type Error = E`

- <span id="flatten-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for Flatten<F>`

##### `impl<F> Unpin for Flatten<F>`

### `FlattenStream<F>`

```rust
struct FlattenStream<F>
where
    F: Future {
    inner: flatten::Flatten<F, <F as Future>::Output>,
}
```

Stream for the [`flatten_stream`](FutureExt::flatten_stream) method.

#### Implementations

- <span id="flattenstream-new"></span>`fn new(x: F) -> Self`

#### Trait Implementations

##### `impl<F> Debug for FlattenStream<F>`

- <span id="flattenstream-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<F> FusedStream for FlattenStream<F>`

- <span id="flattenstream-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<_Item, F> Sink for FlattenStream<F>`

- <span id="flattenstream-sink-type-error"></span>`type Error = <Flatten<F, <F as Future>::Output> as Sink>::Error`

- <span id="flattenstream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flattenstream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../sink/index.md#sink)

- <span id="flattenstream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

- <span id="flattenstream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../../sink/index.md#sink)

##### `impl<Item> SinkExt for FlattenStream<F>`

##### `impl<F> Stream for FlattenStream<F>`

- <span id="flattenstream-stream-type-item"></span>`type Item = <Flatten<F, <F as Future>::Output> as Stream>::Item`

- <span id="flattenstream-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="flattenstream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlattenStream<F>`

##### `impl TryStream for FlattenStream<F>`

- <span id="flattenstream-trystream-type-ok"></span>`type Ok = T`

- <span id="flattenstream-trystream-type-error"></span>`type Error = E`

- <span id="flattenstream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for FlattenStream<F>`

##### `impl<F> Unpin for FlattenStream<F>`

### `Fuse<Fut>`

```rust
struct Fuse<Fut> {
    inner: Option<Fut>,
}
```

Future for the [`fuse`](super::FutureExt::fuse) method.

#### Implementations

- <span id="fuse-new"></span>`fn new(f: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for Fuse<Fut>`

- <span id="fuse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> FusedFuture for Fuse<Fut>`

- <span id="fuse-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Future for Fuse<Fut>`

- <span id="fuse-future-type-output"></span>`type Output = <Fut as Future>::Output`

- <span id="fuse-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Fut as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Fuse<Fut>`

##### `impl IntoFuture for Fuse<Fut>`

- <span id="fuse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="fuse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="fuse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Fuse<Fut>`

- <span id="fuse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="fuse-tryfuture-type-error"></span>`type Error = E`

- <span id="fuse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for Fuse<Fut>`

##### `impl<Fut> Unpin for Fuse<Fut>`

### `Map<Fut, F>`

```rust
struct Map<Fut, F> {
    inner: map::Map<Fut, F>,
}
```

Future for the [`map`](super::FutureExt::map) method.

#### Implementations

- <span id="map-new"></span>`fn new(x: Fut, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut, F> Debug for Map<Fut, F>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F> FusedFuture for Map<Fut, F>`

- <span id="map-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for Map<Fut, F>`

- <span id="map-future-type-output"></span>`type Output = <Map<Fut, F> as Future>::Output`

- <span id="map-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Map<Fut, F>`

##### `impl<F> IntoFuture for Map<Fut, F>`

- <span id="map-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="map-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="map-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Map<Fut, F>`

- <span id="map-tryfuture-type-ok"></span>`type Ok = T`

- <span id="map-tryfuture-type-error"></span>`type Error = E`

- <span id="map-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for Map<Fut, F>`

##### `impl<Fut, F> Unpin for Map<Fut, F>`

### `IntoStream<F>`

```rust
struct IntoStream<F> {
    inner: crate::stream::Once<F>,
}
```

Stream for the [`into_stream`](FutureExt::into_stream) method.

#### Implementations

- <span id="intostream-new"></span>`fn new(x: F) -> Self`

#### Trait Implementations

##### `impl<F> Debug for IntoStream<F>`

- <span id="intostream-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<F> FusedStream for IntoStream<F>`

- <span id="intostream-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F> Stream for IntoStream<F>`

- <span id="intostream-stream-type-item"></span>`type Item = <Once<F> as Stream>::Item`

- <span id="intostream-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="intostream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for IntoStream<F>`

##### `impl TryStream for IntoStream<F>`

- <span id="intostream-trystream-type-ok"></span>`type Ok = T`

- <span id="intostream-trystream-type-error"></span>`type Error = E`

- <span id="intostream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for IntoStream<F>`

##### `impl<F> Unpin for IntoStream<F>`

### `MapInto<Fut, T>`

```rust
struct MapInto<Fut, T> {
    inner: Map<Fut, crate::fns::IntoFn<T>>,
}
```

Future for the [`map_into`](FutureExt::map_into) combinator.

#### Implementations

- <span id="mapinto-new"></span>`fn new(x: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut, T> Debug for MapInto<Fut, T>`

- <span id="mapinto-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, T> FusedFuture for MapInto<Fut, T>`

- <span id="mapinto-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, T> Future for MapInto<Fut, T>`

- <span id="mapinto-future-type-output"></span>`type Output = <Map<Fut, IntoFn<T>> as Future>::Output`

- <span id="mapinto-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<T> FutureExt for MapInto<Fut, T>`

##### `impl IntoFuture for MapInto<Fut, T>`

- <span id="mapinto-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="mapinto-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="mapinto-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> TryFuture for MapInto<Fut, T>`

- <span id="mapinto-tryfuture-type-ok"></span>`type Ok = T`

- <span id="mapinto-tryfuture-type-error"></span>`type Error = E`

- <span id="mapinto-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for MapInto<Fut, T>`

##### `impl<Fut, T> Unpin for MapInto<Fut, T>`

### `Then<Fut1, Fut2, F>`

```rust
struct Then<Fut1, Fut2, F> {
    inner: flatten::Flatten<Map<Fut1, F>, Fut2>,
}
```

Future for the [`then`](FutureExt::then) method.

#### Implementations

- <span id="then-new"></span>`fn new(x: Fut1, y: F) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, F> Debug for Then<Fut1, Fut2, F>`

- <span id="then-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut1, Fut2, F> FusedFuture for Then<Fut1, Fut2, F>`

- <span id="then-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1, Fut2, F> Future for Then<Fut1, Fut2, F>`

- <span id="then-future-type-output"></span>`type Output = <Flatten<Map<Fut1, F>, Fut2> as Future>::Output`

- <span id="then-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Then<Fut1, Fut2, F>`

##### `impl<F> IntoFuture for Then<Fut1, Fut2, F>`

- <span id="then-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="then-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="then-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Then<Fut1, Fut2, F>`

- <span id="then-tryfuture-type-ok"></span>`type Ok = T`

- <span id="then-tryfuture-type-error"></span>`type Error = E`

- <span id="then-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for Then<Fut1, Fut2, F>`

##### `impl<Fut1, Fut2, F> Unpin for Then<Fut1, Fut2, F>`

### `Inspect<Fut, F>`

```rust
struct Inspect<Fut, F> {
    inner: map::Map<Fut, crate::fns::InspectFn<F>>,
}
```

Future for the [`inspect`](FutureExt::inspect) method.

#### Implementations

- <span id="inspect-new"></span>`fn new(x: Fut, f: F) -> Self`

#### Trait Implementations

##### `impl<Fut, F> Debug for Inspect<Fut, F>`

- <span id="inspect-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut, F> FusedFuture for Inspect<Fut, F>`

- <span id="inspect-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut, F> Future for Inspect<Fut, F>`

- <span id="inspect-future-type-output"></span>`type Output = <Map<Fut, InspectFn<F>> as Future>::Output`

- <span id="inspect-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Inspect<Fut, F>`

##### `impl<F> IntoFuture for Inspect<Fut, F>`

- <span id="inspect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="inspect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="inspect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Inspect<Fut, F>`

- <span id="inspect-tryfuture-type-ok"></span>`type Ok = T`

- <span id="inspect-tryfuture-type-error"></span>`type Error = E`

- <span id="inspect-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for Inspect<Fut, F>`

##### `impl<Fut, F> Unpin for Inspect<Fut, F>`

### `NeverError<Fut>`

```rust
struct NeverError<Fut> {
    inner: Map<Fut, crate::fns::OkFn<crate::never::Never>>,
}
```

Future for the [`never_error`](super::FutureExt::never_error) combinator.

#### Implementations

- <span id="nevererror-new"></span>`fn new(x: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut> Debug for NeverError<Fut>`

- <span id="nevererror-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut> FusedFuture for NeverError<Fut>`

- <span id="nevererror-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut> Future for NeverError<Fut>`

- <span id="nevererror-future-type-output"></span>`type Output = <Map<Fut, OkFn<Infallible>> as Future>::Output`

- <span id="nevererror-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for NeverError<Fut>`

##### `impl IntoFuture for NeverError<Fut>`

- <span id="nevererror-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="nevererror-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="nevererror-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for NeverError<Fut>`

- <span id="nevererror-tryfuture-type-ok"></span>`type Ok = T`

- <span id="nevererror-tryfuture-type-error"></span>`type Error = E`

- <span id="nevererror-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for NeverError<Fut>`

##### `impl<Fut> Unpin for NeverError<Fut>`

### `UnitError<Fut>`

```rust
struct UnitError<Fut> {
    inner: Map<Fut, crate::fns::OkFn<()>>,
}
```

Future for the [`unit_error`](super::FutureExt::unit_error) combinator.

#### Implementations

- <span id="uniterror-new"></span>`fn new(x: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut> Debug for UnitError<Fut>`

- <span id="uniterror-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<Fut> FusedFuture for UnitError<Fut>`

- <span id="uniterror-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut> Future for UnitError<Fut>`

- <span id="uniterror-future-type-output"></span>`type Output = <Map<Fut, OkFn<()>> as Future>::Output`

- <span id="uniterror-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for UnitError<Fut>`

##### `impl IntoFuture for UnitError<Fut>`

- <span id="uniterror-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="uniterror-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="uniterror-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for UnitError<Fut>`

- <span id="uniterror-tryfuture-type-ok"></span>`type Ok = T`

- <span id="uniterror-tryfuture-type-error"></span>`type Error = E`

- <span id="uniterror-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl<Fut> TryFutureExt for UnitError<Fut>`

##### `impl<Fut> Unpin for UnitError<Fut>`

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

