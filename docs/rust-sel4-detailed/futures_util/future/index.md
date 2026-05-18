*[futures_util](../index.md) / [future](index.md)*

---

# Module `future`

Asynchronous values.

This module contains:

- The [`Future`](#future) trait.
- The [`FutureExt`](future/index.md) and [`TryFutureExt`](try_future/index.md) trait, which provides adapters for
  chaining and composing futures.
- Top-level future combinators like [`lazy`](lazy()) which creates a future
  from a closure that defines its return value, and [`ready`](ready()),
  which constructs a future with an immediate defined value.

## Contents

- [Modules](#modules)
  - [`future`](#future)
  - [`try_future`](#try-future)
  - [`lazy`](#lazy)
  - [`pending`](#pending)
  - [`maybe_done`](#maybe-done)
  - [`try_maybe_done`](#try-maybe-done)
  - [`option`](#option)
  - [`poll_fn`](#poll-fn)
  - [`poll_immediate`](#poll-immediate)
  - [`ready`](#ready)
  - [`always_ready`](#always-ready)
  - [`join`](#join)
  - [`join_all`](#join-all)
  - [`select`](#select)
  - [`select_all`](#select-all)
  - [`try_join`](#try-join)
  - [`try_join_all`](#try-join-all)
  - [`try_select`](#try-select)
  - [`select_ok`](#select-ok)
  - [`either`](#either)
  - [`abortable`](#abortable)
- [Structs](#structs)
  - [`Flatten`](#flatten)
  - [`Fuse`](#fuse)
  - [`Inspect`](#inspect)
  - [`IntoStream`](#intostream)
  - [`Map`](#map)
  - [`MapInto`](#mapinto)
  - [`NeverError`](#nevererror)
  - [`Then`](#then)
  - [`UnitError`](#uniterror)
  - [`FlattenStream`](#flattenstream)
  - [`AndThen`](#andthen)
  - [`ErrInto`](#errinto)
  - [`InspectErr`](#inspecterr)
  - [`InspectOk`](#inspectok)
  - [`IntoFuture`](#intofuture)
  - [`MapErr`](#maperr)
  - [`MapOk`](#mapok)
  - [`MapOkOrElse`](#mapokorelse)
  - [`OkInto`](#okinto)
  - [`OrElse`](#orelse)
  - [`TryFlatten`](#tryflatten)
  - [`TryFlattenStream`](#tryflattenstream)
  - [`UnwrapOrElse`](#unwraporelse)
  - [`FlattenSink`](#flattensink)
  - [`Lazy`](#lazy)
  - [`Pending`](#pending)
  - [`OptionFuture`](#optionfuture)
  - [`PollFn`](#pollfn)
  - [`PollImmediate`](#pollimmediate)
  - [`Ready`](#ready)
  - [`AlwaysReady`](#alwaysready)
  - [`Join`](#join)
  - [`Join3`](#join3)
  - [`Join4`](#join4)
  - [`Join5`](#join5)
  - [`JoinAll`](#joinall)
  - [`Select`](#select)
  - [`SelectAll`](#selectall)
  - [`TryJoin`](#tryjoin)
  - [`TryJoin3`](#tryjoin3)
  - [`TryJoin4`](#tryjoin4)
  - [`TryJoin5`](#tryjoin5)
  - [`TryJoinAll`](#tryjoinall)
  - [`TrySelect`](#tryselect)
  - [`SelectOk`](#selectok)
  - [`AbortHandle`](#aborthandle)
  - [`AbortRegistration`](#abortregistration)
  - [`Abortable`](#abortable)
  - [`Aborted`](#aborted)
- [Enums](#enums)
  - [`MaybeDone`](#maybedone)
  - [`TryMaybeDone`](#trymaybedone)
  - [`Either`](#either)
- [Traits](#traits)
  - [`FutureExt`](#futureext)
  - [`TryFutureExt`](#tryfutureext)
- [Functions](#functions)
  - [`FutureObj`](#futureobj)
  - [`LocalFutureObj`](#localfutureobj)
  - [`UnsafeFutureObj`](#unsafefutureobj)
  - [`lazy`](#lazy)
  - [`pending`](#pending)
  - [`maybe_done`](#maybe-done)
  - [`try_maybe_done`](#try-maybe-done)
  - [`poll_fn`](#poll-fn)
  - [`poll_immediate`](#poll-immediate)
  - [`err`](#err)
  - [`ok`](#ok)
  - [`ready`](#ready)
  - [`always_ready`](#always-ready)
  - [`join`](#join)
  - [`join3`](#join3)
  - [`join4`](#join4)
  - [`join5`](#join5)
  - [`join_all`](#join-all)
  - [`select`](#select)
  - [`select_all`](#select-all)
  - [`try_join`](#try-join)
  - [`try_join3`](#try-join3)
  - [`try_join4`](#try-join4)
  - [`try_join5`](#try-join5)
  - [`try_join_all`](#try-join-all)
  - [`try_select`](#try-select)
  - [`select_ok`](#select-ok)
  - [`abortable`](#abortable)
  - [`assert_future`](#assert-future)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`future`](#future) | mod | Futures |
| [`try_future`](#try-future) | mod | Futures |
| [`lazy`](#lazy) | mod |  |
| [`pending`](#pending) | mod |  |
| [`maybe_done`](#maybe-done) | mod | Definition of the MaybeDone combinator |
| [`try_maybe_done`](#try-maybe-done) | mod | Definition of the TryMaybeDone combinator |
| [`option`](#option) | mod | Definition of the `Option` (optional step) combinator |
| [`poll_fn`](#poll-fn) | mod | Definition of the `PollFn` adapter combinator |
| [`poll_immediate`](#poll-immediate) | mod |  |
| [`ready`](#ready) | mod |  |
| [`always_ready`](#always-ready) | mod |  |
| [`join`](#join) | mod |  |
| [`join_all`](#join-all) | mod | Definition of the `JoinAll` combinator, waiting for all of a list of futures to finish. |
| [`select`](#select) | mod |  |
| [`select_all`](#select-all) | mod |  |
| [`try_join`](#try-join) | mod |  |
| [`try_join_all`](#try-join-all) | mod | Definition of the `TryJoinAll` combinator, waiting for all of a list of futures to finish with either success or error. |
| [`try_select`](#try-select) | mod |  |
| [`select_ok`](#select-ok) | mod |  |
| [`either`](#either) | mod |  |
| [`abortable`](#abortable) | mod |  |
| [`Flatten`](#flatten) | struct |  |
| [`Fuse`](#fuse) | struct |  |
| [`Inspect`](#inspect) | struct |  |
| [`IntoStream`](#intostream) | struct |  |
| [`Map`](#map) | struct |  |
| [`MapInto`](#mapinto) | struct |  |
| [`NeverError`](#nevererror) | struct |  |
| [`Then`](#then) | struct |  |
| [`UnitError`](#uniterror) | struct |  |
| [`FlattenStream`](#flattenstream) | struct |  |
| [`AndThen`](#andthen) | struct |  |
| [`ErrInto`](#errinto) | struct |  |
| [`InspectErr`](#inspecterr) | struct |  |
| [`InspectOk`](#inspectok) | struct |  |
| [`IntoFuture`](#intofuture) | struct |  |
| [`MapErr`](#maperr) | struct |  |
| [`MapOk`](#mapok) | struct |  |
| [`MapOkOrElse`](#mapokorelse) | struct |  |
| [`OkInto`](#okinto) | struct |  |
| [`OrElse`](#orelse) | struct |  |
| [`TryFlatten`](#tryflatten) | struct |  |
| [`TryFlattenStream`](#tryflattenstream) | struct |  |
| [`UnwrapOrElse`](#unwraporelse) | struct |  |
| [`FlattenSink`](#flattensink) | struct |  |
| [`Lazy`](#lazy) | struct |  |
| [`Pending`](#pending) | struct |  |
| [`OptionFuture`](#optionfuture) | struct |  |
| [`PollFn`](#pollfn) | struct |  |
| [`PollImmediate`](#pollimmediate) | struct |  |
| [`Ready`](#ready) | struct |  |
| [`AlwaysReady`](#alwaysready) | struct |  |
| [`Join`](#join) | struct |  |
| [`Join3`](#join3) | struct |  |
| [`Join4`](#join4) | struct |  |
| [`Join5`](#join5) | struct |  |
| [`JoinAll`](#joinall) | struct |  |
| [`Select`](#select) | struct |  |
| [`SelectAll`](#selectall) | struct |  |
| [`TryJoin`](#tryjoin) | struct |  |
| [`TryJoin3`](#tryjoin3) | struct |  |
| [`TryJoin4`](#tryjoin4) | struct |  |
| [`TryJoin5`](#tryjoin5) | struct |  |
| [`TryJoinAll`](#tryjoinall) | struct |  |
| [`TrySelect`](#tryselect) | struct |  |
| [`SelectOk`](#selectok) | struct |  |
| [`AbortHandle`](#aborthandle) | struct |  |
| [`AbortRegistration`](#abortregistration) | struct |  |
| [`Abortable`](#abortable) | struct |  |
| [`Aborted`](#aborted) | struct |  |
| [`MaybeDone`](#maybedone) | enum |  |
| [`TryMaybeDone`](#trymaybedone) | enum |  |
| [`Either`](#either) | enum |  |
| [`FutureExt`](#futureext) | trait |  |
| [`TryFutureExt`](#tryfutureext) | trait |  |
| [`FutureObj`](#futureobj) | fn |  |
| [`LocalFutureObj`](#localfutureobj) | fn |  |
| [`UnsafeFutureObj`](#unsafefutureobj) | fn |  |
| [`lazy`](#lazy) | fn |  |
| [`pending`](#pending) | fn |  |
| [`maybe_done`](#maybe-done) | fn |  |
| [`try_maybe_done`](#try-maybe-done) | fn |  |
| [`poll_fn`](#poll-fn) | fn |  |
| [`poll_immediate`](#poll-immediate) | fn |  |
| [`err`](#err) | fn |  |
| [`ok`](#ok) | fn |  |
| [`ready`](#ready) | fn |  |
| [`always_ready`](#always-ready) | fn |  |
| [`join`](#join) | fn |  |
| [`join3`](#join3) | fn |  |
| [`join4`](#join4) | fn |  |
| [`join5`](#join5) | fn |  |
| [`join_all`](#join-all) | fn |  |
| [`select`](#select) | fn |  |
| [`select_all`](#select-all) | fn |  |
| [`try_join`](#try-join) | fn |  |
| [`try_join3`](#try-join3) | fn |  |
| [`try_join4`](#try-join4) | fn |  |
| [`try_join5`](#try-join5) | fn |  |
| [`try_join_all`](#try-join-all) | fn |  |
| [`try_select`](#try-select) | fn |  |
| [`select_ok`](#select-ok) | fn |  |
| [`abortable`](#abortable) | fn |  |
| [`assert_future`](#assert-future) | fn |  |

## Modules

- [`future`](future/index.md) — Futures
- [`try_future`](try_future/index.md) — Futures
- [`lazy`](lazy/index.md)
- [`pending`](pending/index.md)
- [`maybe_done`](maybe_done/index.md) — Definition of the MaybeDone combinator
- [`try_maybe_done`](try_maybe_done/index.md) — Definition of the TryMaybeDone combinator
- [`option`](option/index.md) — Definition of the `Option` (optional step) combinator
- [`poll_fn`](poll_fn/index.md) — Definition of the `PollFn` adapter combinator
- [`poll_immediate`](poll_immediate/index.md)
- [`ready`](ready/index.md)
- [`always_ready`](always_ready/index.md)
- [`join`](join/index.md)
- [`join_all`](join_all/index.md) — Definition of the `JoinAll` combinator, waiting for all of a list of futures
- [`select`](select/index.md)
- [`select_all`](select_all/index.md)
- [`try_join`](try_join/index.md)
- [`try_join_all`](try_join_all/index.md) — Definition of the `TryJoinAll` combinator, waiting for all of a list of
- [`try_select`](try_select/index.md)
- [`select_ok`](select_ok/index.md)
- [`either`](either/index.md)
- [`abortable`](abortable/index.md)

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

- <span id="flatten-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Flatten<F>`

##### `impl<F> IntoFuture for Flatten<F>`

- <span id="flatten-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="flatten-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="flatten-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Flatten<F>`

- <span id="flatten-tryfuture-type-ok"></span>`type Ok = T`

- <span id="flatten-tryfuture-type-error"></span>`type Error = E`

- <span id="flatten-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for Flatten<F>`

##### `impl<F> Unpin for Flatten<F>`

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

- <span id="fuse-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Fut as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Fuse<Fut>`

##### `impl IntoFuture for Fuse<Fut>`

- <span id="fuse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="fuse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="fuse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Fuse<Fut>`

- <span id="fuse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="fuse-tryfuture-type-error"></span>`type Error = E`

- <span id="fuse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for Fuse<Fut>`

##### `impl<Fut> Unpin for Fuse<Fut>`

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

- <span id="inspect-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Inspect<Fut, F>`

##### `impl<F> IntoFuture for Inspect<Fut, F>`

- <span id="inspect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="inspect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="inspect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Inspect<Fut, F>`

- <span id="inspect-tryfuture-type-ok"></span>`type Ok = T`

- <span id="inspect-tryfuture-type-error"></span>`type Error = E`

- <span id="inspect-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for Inspect<Fut, F>`

##### `impl<Fut, F> Unpin for Inspect<Fut, F>`

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

- <span id="intostream-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="intostream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for IntoStream<F>`

##### `impl TryStream for IntoStream<F>`

- <span id="intostream-trystream-type-ok"></span>`type Ok = T`

- <span id="intostream-trystream-type-error"></span>`type Error = E`

- <span id="intostream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for IntoStream<F>`

##### `impl<F> Unpin for IntoStream<F>`

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

- <span id="map-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Map<Fut, F>`

##### `impl<F> IntoFuture for Map<Fut, F>`

- <span id="map-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="map-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="map-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Map<Fut, F>`

- <span id="map-tryfuture-type-ok"></span>`type Ok = T`

- <span id="map-tryfuture-type-error"></span>`type Error = E`

- <span id="map-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for Map<Fut, F>`

##### `impl<Fut, F> Unpin for Map<Fut, F>`

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

- <span id="mapinto-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<T> FutureExt for MapInto<Fut, T>`

##### `impl IntoFuture for MapInto<Fut, T>`

- <span id="mapinto-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="mapinto-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="mapinto-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> TryFuture for MapInto<Fut, T>`

- <span id="mapinto-tryfuture-type-ok"></span>`type Ok = T`

- <span id="mapinto-tryfuture-type-error"></span>`type Error = E`

- <span id="mapinto-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for MapInto<Fut, T>`

##### `impl<Fut, T> Unpin for MapInto<Fut, T>`

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

- <span id="nevererror-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for NeverError<Fut>`

##### `impl IntoFuture for NeverError<Fut>`

- <span id="nevererror-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="nevererror-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="nevererror-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for NeverError<Fut>`

- <span id="nevererror-tryfuture-type-ok"></span>`type Ok = T`

- <span id="nevererror-tryfuture-type-error"></span>`type Error = E`

- <span id="nevererror-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for NeverError<Fut>`

##### `impl<Fut> Unpin for NeverError<Fut>`

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

- <span id="then-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Then<Fut1, Fut2, F>`

##### `impl<F> IntoFuture for Then<Fut1, Fut2, F>`

- <span id="then-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="then-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="then-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Then<Fut1, Fut2, F>`

- <span id="then-tryfuture-type-ok"></span>`type Ok = T`

- <span id="then-tryfuture-type-error"></span>`type Error = E`

- <span id="then-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for Then<Fut1, Fut2, F>`

##### `impl<Fut1, Fut2, F> Unpin for Then<Fut1, Fut2, F>`

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

- <span id="uniterror-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for UnitError<Fut>`

##### `impl IntoFuture for UnitError<Fut>`

- <span id="uniterror-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="uniterror-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="uniterror-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for UnitError<Fut>`

- <span id="uniterror-tryfuture-type-ok"></span>`type Ok = T`

- <span id="uniterror-tryfuture-type-error"></span>`type Error = E`

- <span id="uniterror-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for UnitError<Fut>`

##### `impl<Fut> Unpin for UnitError<Fut>`

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

- <span id="flattenstream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flattenstream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="flattenstream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flattenstream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for FlattenStream<F>`

##### `impl<F> Stream for FlattenStream<F>`

- <span id="flattenstream-stream-type-item"></span>`type Item = <Flatten<F, <F as Future>::Output> as Stream>::Item`

- <span id="flattenstream-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="flattenstream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlattenStream<F>`

##### `impl TryStream for FlattenStream<F>`

- <span id="flattenstream-trystream-type-ok"></span>`type Ok = T`

- <span id="flattenstream-trystream-type-error"></span>`type Error = E`

- <span id="flattenstream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for FlattenStream<F>`

##### `impl<F> Unpin for FlattenStream<F>`

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

- <span id="andthen-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for AndThen<Fut1, Fut2, F>`

##### `impl<F> IntoFuture for AndThen<Fut1, Fut2, F>`

- <span id="andthen-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="andthen-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="andthen-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for AndThen<Fut1, Fut2, F>`

- <span id="andthen-tryfuture-type-ok"></span>`type Ok = T`

- <span id="andthen-tryfuture-type-error"></span>`type Error = E`

- <span id="andthen-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for AndThen<Fut1, Fut2, F>`

##### `impl<Fut1, Fut2, F> Unpin for AndThen<Fut1, Fut2, F>`

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

- <span id="errinto-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for ErrInto<Fut, E>`

##### `impl IntoFuture for ErrInto<Fut, E>`

- <span id="errinto-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="errinto-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="errinto-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<E> TryFuture for ErrInto<Fut, E>`

- <span id="errinto-tryfuture-type-ok"></span>`type Ok = T`

- <span id="errinto-tryfuture-type-error"></span>`type Error = E`

- <span id="errinto-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for ErrInto<Fut, E>`

##### `impl<Fut, E> Unpin for ErrInto<Fut, E>`

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

- <span id="inspecterr-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for InspectErr<Fut, F>`

##### `impl<F> IntoFuture for InspectErr<Fut, F>`

- <span id="inspecterr-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="inspecterr-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="inspecterr-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for InspectErr<Fut, F>`

- <span id="inspecterr-tryfuture-type-ok"></span>`type Ok = T`

- <span id="inspecterr-tryfuture-type-error"></span>`type Error = E`

- <span id="inspecterr-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for InspectErr<Fut, F>`

##### `impl<Fut, F> Unpin for InspectErr<Fut, F>`

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

- <span id="inspectok-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for InspectOk<Fut, F>`

##### `impl<F> IntoFuture for InspectOk<Fut, F>`

- <span id="inspectok-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="inspectok-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="inspectok-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for InspectOk<Fut, F>`

- <span id="inspectok-tryfuture-type-ok"></span>`type Ok = T`

- <span id="inspectok-tryfuture-type-error"></span>`type Error = E`

- <span id="inspectok-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for InspectOk<Fut, F>`

##### `impl<Fut, F> Unpin for InspectOk<Fut, F>`

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

- <span id="intofuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for IntoFuture<Fut>`

##### `impl IntoFuture for IntoFuture<Fut>`

- <span id="intofuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="intofuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="intofuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for IntoFuture<Fut>`

- <span id="intofuture-tryfuture-type-ok"></span>`type Ok = T`

- <span id="intofuture-tryfuture-type-error"></span>`type Error = E`

- <span id="intofuture-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for IntoFuture<Fut>`

##### `impl<Fut> Unpin for IntoFuture<Fut>`

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

- <span id="maperr-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for MapErr<Fut, F>`

##### `impl<F> IntoFuture for MapErr<Fut, F>`

- <span id="maperr-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="maperr-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="maperr-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for MapErr<Fut, F>`

- <span id="maperr-tryfuture-type-ok"></span>`type Ok = T`

- <span id="maperr-tryfuture-type-error"></span>`type Error = E`

- <span id="maperr-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for MapErr<Fut, F>`

##### `impl<Fut, F> Unpin for MapErr<Fut, F>`

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

- <span id="mapok-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for MapOk<Fut, F>`

##### `impl<F> IntoFuture for MapOk<Fut, F>`

- <span id="mapok-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="mapok-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="mapok-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for MapOk<Fut, F>`

- <span id="mapok-tryfuture-type-ok"></span>`type Ok = T`

- <span id="mapok-tryfuture-type-error"></span>`type Error = E`

- <span id="mapok-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for MapOk<Fut, F>`

##### `impl<Fut, F> Unpin for MapOk<Fut, F>`

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

- <span id="mapokorelse-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for MapOkOrElse<Fut, F, G>`

##### `impl<F> IntoFuture for MapOkOrElse<Fut, F, G>`

- <span id="mapokorelse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="mapokorelse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="mapokorelse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for MapOkOrElse<Fut, F, G>`

- <span id="mapokorelse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="mapokorelse-tryfuture-type-error"></span>`type Error = E`

- <span id="mapokorelse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for MapOkOrElse<Fut, F, G>`

##### `impl<Fut, F, G> Unpin for MapOkOrElse<Fut, F, G>`

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

- <span id="okinto-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for OkInto<Fut, E>`

##### `impl IntoFuture for OkInto<Fut, E>`

- <span id="okinto-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="okinto-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="okinto-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<E> TryFuture for OkInto<Fut, E>`

- <span id="okinto-tryfuture-type-ok"></span>`type Ok = T`

- <span id="okinto-tryfuture-type-error"></span>`type Error = E`

- <span id="okinto-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for OkInto<Fut, E>`

##### `impl<Fut, E> Unpin for OkInto<Fut, E>`

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

- <span id="orelse-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for OrElse<Fut1, Fut2, F>`

##### `impl<F> IntoFuture for OrElse<Fut1, Fut2, F>`

- <span id="orelse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="orelse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="orelse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for OrElse<Fut1, Fut2, F>`

- <span id="orelse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="orelse-tryfuture-type-error"></span>`type Error = E`

- <span id="orelse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for OrElse<Fut1, Fut2, F>`

##### `impl<Fut1, Fut2, F> Unpin for OrElse<Fut1, Fut2, F>`

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

- <span id="tryflatten-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TryFlatten<Fut1, Fut2>`

##### `impl IntoFuture for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryflatten-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryflatten-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryflatten-tryfuture-type-error"></span>`type Error = E`

- <span id="tryflatten-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for TryFlatten<Fut1, Fut2>`

##### `impl<Fut1, Fut2> Unpin for TryFlatten<Fut1, Fut2>`

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

- <span id="tryflattenstream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryflattenstream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="tryflattenstream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="tryflattenstream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlattenStream<Fut>`

##### `impl<Fut> Stream for TryFlattenStream<Fut>`

- <span id="tryflattenstream-stream-type-item"></span>`type Item = <TryFlatten<Fut, <Fut as TryFuture>::Ok> as Stream>::Item`

- <span id="tryflattenstream-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="tryflattenstream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFlattenStream<Fut>`

##### `impl TryStream for TryFlattenStream<Fut>`

- <span id="tryflattenstream-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflattenstream-trystream-type-error"></span>`type Error = E`

- <span id="tryflattenstream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for TryFlattenStream<Fut>`

##### `impl<Fut> Unpin for TryFlattenStream<Fut>`

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

- <span id="unwraporelse-future-poll"></span>`fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for UnwrapOrElse<Fut, F>`

##### `impl<F> IntoFuture for UnwrapOrElse<Fut, F>`

- <span id="unwraporelse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="unwraporelse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="unwraporelse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for UnwrapOrElse<Fut, F>`

- <span id="unwraporelse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="unwraporelse-tryfuture-type-error"></span>`type Error = E`

- <span id="unwraporelse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for UnwrapOrElse<Fut, F>`

##### `impl<Fut, F> Unpin for UnwrapOrElse<Fut, F>`

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

- <span id="flattensink-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flattensink-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="flattensink-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="flattensink-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for FlattenSink<Fut, Si>`

##### `impl<Fut, Si> Stream for FlattenSink<Fut, Si>`

- <span id="flattensink-stream-type-item"></span>`type Item = <TryFlatten<Fut, Si> as Stream>::Item`

- <span id="flattensink-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="flattensink-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FlattenSink<Fut, Si>`

##### `impl TryStream for FlattenSink<Fut, Si>`

- <span id="flattensink-trystream-type-ok"></span>`type Ok = T`

- <span id="flattensink-trystream-type-error"></span>`type Error = E`

- <span id="flattensink-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for FlattenSink<Fut, Si>`

##### `impl<Fut, Si> Unpin for FlattenSink<Fut, Si>`

### `Lazy<F>`

```rust
struct Lazy<F> {
    f: Option<F>,
}
```

Future for the [`lazy`](lazy/index.md) function.

#### Trait Implementations

##### `impl<F: fmt::Debug> Debug for Lazy<F>`

- <span id="lazy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> FusedFuture for Lazy<F>`

- <span id="lazy-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F> Future for Lazy<F>`

- <span id="lazy-future-type-output"></span>`type Output = R`

- <span id="lazy-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<R>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for Lazy<F>`

##### `impl<F> IntoFuture for Lazy<F>`

- <span id="lazy-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="lazy-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="lazy-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Lazy<F>`

- <span id="lazy-tryfuture-type-ok"></span>`type Ok = T`

- <span id="lazy-tryfuture-type-error"></span>`type Error = E`

- <span id="lazy-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for Lazy<F>`

##### `impl<F> Unpin for Lazy<F>`

### `Pending<T>`

```rust
struct Pending<T> {
    _data: marker::PhantomData<T>,
}
```

Future for the [`pending()`](pending/index.md) function.

#### Trait Implementations

##### `impl<T> Clone for Pending<T>`

- <span id="pending-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Pending<T>`

- <span id="pending-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedFuture for Pending<T>`

- <span id="pending-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Future for Pending<T>`

- <span id="pending-future-type-output"></span>`type Output = T`

- <span id="pending-future-poll"></span>`fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<T>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl<T> FutureExt for Pending<T>`

##### `impl IntoFuture for Pending<T>`

- <span id="pending-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="pending-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="pending-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> TryFuture for Pending<T>`

- <span id="pending-tryfuture-type-ok"></span>`type Ok = T`

- <span id="pending-tryfuture-type-error"></span>`type Error = E`

- <span id="pending-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for Pending<T>`

##### `impl<T> Unpin for Pending<T>`

### `OptionFuture<F>`

```rust
struct OptionFuture<F> {
    inner: Option<F>,
}
```

A future representing a value which may or may not be present.

Created by the [`From`](../../thiserror_impl/attr/index.md) implementation for [`Option`](std::option::Option).

# Examples

```rust
futures::executor::block_on(async {
use futures::future::OptionFuture;

let mut a: OptionFuture<_> = Some(async { 123 }).into();
assert_eq!(a.await, Some(123));

a = None.into();
assert_eq!(a.await, None);
});
```

#### Trait Implementations

##### `impl<F: clone::Clone> Clone for OptionFuture<F>`

- <span id="optionfuture-clone"></span>`fn clone(&self) -> OptionFuture<F>` — [`OptionFuture`](option/index.md#optionfuture)

##### `impl<F: fmt::Debug> Debug for OptionFuture<F>`

- <span id="optionfuture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> Default for OptionFuture<F>`

- <span id="optionfuture-default"></span>`fn default() -> Self`

##### `impl<F: FusedFuture> FusedFuture for OptionFuture<F>`

- <span id="optionfuture-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F: Future> Future for OptionFuture<F>`

- <span id="optionfuture-future-type-output"></span>`type Output = Option<<F as Future>::Output>`

- <span id="optionfuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for OptionFuture<F>`

##### `impl<F> IntoFuture for OptionFuture<F>`

- <span id="optionfuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="optionfuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="optionfuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> Unpin for OptionFuture<F>`

### `PollFn<F>`

```rust
struct PollFn<F> {
    f: F,
}
```

Future for the [`poll_fn`](poll_fn/index.md) function.

#### Trait Implementations

##### `impl<F> Debug for PollFn<F>`

- <span id="pollfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> Future for PollFn<F>`

- <span id="pollfn-future-type-output"></span>`type Output = T`

- <span id="pollfn-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl FutureExt for PollFn<F>`

##### `impl<F> IntoFuture for PollFn<F>`

- <span id="pollfn-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="pollfn-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="pollfn-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for PollFn<F>`

- <span id="pollfn-tryfuture-type-ok"></span>`type Ok = T`

- <span id="pollfn-tryfuture-type-error"></span>`type Error = E`

- <span id="pollfn-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for PollFn<F>`

##### `impl<F> Unpin for PollFn<F>`

### `PollImmediate<T>`

```rust
struct PollImmediate<T> {
    future: Option<T>,
}
```

Future for the [`poll_immediate`](poll_immediate()) function.

It will never return [Poll::Pending](core::task::Poll::Pending)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for PollImmediate<T>`

- <span id="pollimmediate-clone"></span>`fn clone(&self) -> PollImmediate<T>` — [`PollImmediate`](poll_immediate/index.md#pollimmediate)

##### `impl<T: fmt::Debug> Debug for PollImmediate<T>`

- <span id="pollimmediate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Future> FusedFuture for PollImmediate<T>`

- <span id="pollimmediate-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F> Future for PollImmediate<F>`

- <span id="pollimmediate-future-type-output"></span>`type Output = Option<T>`

- <span id="pollimmediate-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl<T> FutureExt for PollImmediate<T>`

##### `impl IntoFuture for PollImmediate<T>`

- <span id="pollimmediate-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="pollimmediate-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="pollimmediate-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> Stream for PollImmediate<F>`

- <span id="pollimmediate-stream-type-item"></span>`type Item = Poll<T>`

- <span id="pollimmediate-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

##### `impl<T> StreamExt for PollImmediate<T>`

##### `impl<T> Unpin for PollImmediate<T>`

### `Ready<T>`

```rust
struct Ready<T>(Option<T>);
```

Future for the [`ready`](ready()) function.

#### Implementations

- <span id="ready-into-inner"></span>`fn into_inner(self) -> T`

  Unwraps the value from this immediately ready future.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Ready<T>`

- <span id="ready-clone"></span>`fn clone(&self) -> Ready<T>` — [`Ready`](ready/index.md#ready)

##### `impl<T: fmt::Debug> Debug for Ready<T>`

- <span id="ready-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedFuture for Ready<T>`

- <span id="ready-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Future for Ready<T>`

- <span id="ready-future-type-output"></span>`type Output = T`

- <span id="ready-future-poll"></span>`fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl<T> FutureExt for Ready<T>`

##### `impl IntoFuture for Ready<T>`

- <span id="ready-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="ready-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="ready-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> TryFuture for Ready<T>`

- <span id="ready-tryfuture-type-ok"></span>`type Ok = T`

- <span id="ready-tryfuture-type-error"></span>`type Error = E`

- <span id="ready-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for Ready<T>`

##### `impl<T> Unpin for Ready<T>`

### `AlwaysReady<T, F: Fn() -> T>`

```rust
struct AlwaysReady<T, F: Fn() -> T>(F);
```

Future for the [`always_ready`](always_ready()) function.

#### Trait Implementations

##### `impl<T, F: Fn() -> T + Clone> Clone for AlwaysReady<T, F>`

- <span id="alwaysready-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, F: Fn() -> T + Copy> Copy for AlwaysReady<T, F>`

##### `impl<T, F: Fn() -> T> Debug for AlwaysReady<T, F>`

- <span id="alwaysready-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T, F: Fn() -> T> FusedFuture for AlwaysReady<T, F>`

- <span id="alwaysready-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T, F: Fn() -> T> Future for AlwaysReady<T, F>`

- <span id="alwaysready-future-type-output"></span>`type Output = T`

- <span id="alwaysready-future-poll"></span>`fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll)

##### `impl<T> FutureExt for AlwaysReady<T, F>`

##### `impl<F> IntoFuture for AlwaysReady<T, F>`

- <span id="alwaysready-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="alwaysready-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="alwaysready-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for AlwaysReady<T, F>`

- <span id="alwaysready-tryfuture-type-ok"></span>`type Ok = T`

- <span id="alwaysready-tryfuture-type-error"></span>`type Error = E`

- <span id="alwaysready-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for AlwaysReady<T, F>`

##### `impl<T, F: Fn() -> T> Unpin for AlwaysReady<T, F>`

### `Join<Fut1: Future, Fut2: Future>`

```rust
struct Join<Fut1: Future, Fut2: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
}
```

Future for the [`join`](join()) function.

#### Implementations

- <span id="join-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2> Debug for Join<Fut1, Fut2>`

- <span id="join-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture> FusedFuture for Join<Fut1, Fut2>`

- <span id="join-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future> Future for Join<Fut1, Fut2>`

- <span id="join-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output)`

- <span id="join-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Join<Fut1, Fut2>`

##### `impl IntoFuture for Join<Fut1, Fut2>`

- <span id="join-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future> Unpin for Join<Fut1, Fut2>`

### `Join3<Fut1: Future, Fut2: Future, Fut3: Future>`

```rust
struct Join3<Fut1: Future, Fut2: Future, Fut3: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
    Fut3: MaybeDone<Fut3>,
}
```

Future for the [`join3`](join/index.md) function.

#### Implementations

- <span id="join3-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3> Debug for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture, Fut3: FusedFuture> FusedFuture for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future> Future for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output, <Fut3 as Future>::Output)`

- <span id="join3-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Join3<Fut1, Fut2, Fut3>`

##### `impl IntoFuture for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join3-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join3-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future> Unpin for Join3<Fut1, Fut2, Fut3>`

### `Join4<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future>`

```rust
struct Join4<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
    Fut3: MaybeDone<Fut3>,
    Fut4: MaybeDone<Fut4>,
}
```

Future for the [`join4`](join/index.md) function.

#### Implementations

- <span id="join4-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4> Debug for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture, Fut3: FusedFuture, Fut4: FusedFuture> FusedFuture for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future> Future for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output, <Fut3 as Future>::Output, <Fut4 as Future>::Output)`

- <span id="join4-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Join4<Fut1, Fut2, Fut3, Fut4>`

##### `impl IntoFuture for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join4-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join4-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future> Unpin for Join4<Fut1, Fut2, Fut3, Fut4>`

### `Join5<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future>`

```rust
struct Join5<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
    Fut3: MaybeDone<Fut3>,
    Fut4: MaybeDone<Fut4>,
    Fut5: MaybeDone<Fut5>,
}
```

Future for the [`join5`](join/index.md) function.

#### Implementations

- <span id="join5-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4, Fut5: Fut5) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4, Fut5> Debug for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture, Fut3: FusedFuture, Fut4: FusedFuture, Fut5: FusedFuture> FusedFuture for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future> Future for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output, <Fut3 as Future>::Output, <Fut4 as Future>::Output, <Fut5 as Future>::Output)`

- <span id="join5-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

##### `impl IntoFuture for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join5-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join5-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future> Unpin for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

### `JoinAll<F>`

```rust
struct JoinAll<F>
where
    F: Future {
    kind: JoinAllKind<F>,
}
```

Future for the [`join_all`](join_all/index.md) function.

#### Trait Implementations

##### `impl<F> Debug for JoinAll<F>`

- <span id="joinall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: Future> FromIterator for JoinAll<F>`

- <span id="joinall-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = F>>(iter: T) -> Self`

##### `impl<F> Future for JoinAll<F>`

- <span id="joinall-future-type-output"></span>`type Output = Vec<<F as Future>::Output>`

- <span id="joinall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for JoinAll<F>`

##### `impl<F> IntoFuture for JoinAll<F>`

- <span id="joinall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="joinall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="joinall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

### `Select<A, B>`

```rust
struct Select<A, B> {
    inner: Option<(A, B)>,
}
```

Future for the [`select()`](select/index.md) function.

#### Trait Implementations

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Select<A, B>`

- <span id="select-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> FusedFuture for Select<A, B>`

- <span id="select-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<A, B> Future for Select<A, B>`

- <span id="select-future-type-output"></span>`type Output = Either<(<A as Future>::Output, B), (<B as Future>::Output, A)>`

- <span id="select-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Select<A, B>`

##### `impl IntoFuture for Select<A, B>`

- <span id="select-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="select-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="select-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<A: Unpin, B: Unpin> Unpin for Select<A, B>`

### `SelectAll<Fut>`

```rust
struct SelectAll<Fut> {
    inner: alloc::vec::Vec<Fut>,
}
```

Future for the [`select_all`](select_all/index.md) function.

#### Implementations

- <span id="selectall-into-inner"></span>`fn into_inner(self) -> Vec<Fut>`

  Consumes this combinator, returning the underlying futures.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for SelectAll<Fut>`

- <span id="selectall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future + Unpin> FromIterator for SelectAll<Fut>`

- <span id="selectall-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Fut>>(iter: T) -> Self`

##### `impl<Fut: Future + Unpin> Future for SelectAll<Fut>`

- <span id="selectall-future-type-output"></span>`type Output = (<Fut as Future>::Output, usize, Vec<Fut>)`

- <span id="selectall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for SelectAll<Fut>`

##### `impl IntoFuture for SelectAll<Fut>`

- <span id="selectall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="selectall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="selectall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut: Unpin> Unpin for SelectAll<Fut>`

### `TryJoin<Fut1: TryFuture, Fut2: TryFuture>`

```rust
struct TryJoin<Fut1: TryFuture, Fut2: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
}
```

Future for the [`try_join`](try_join()) function.

#### Implementations

- <span id="tryjoin-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2> Debug for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2> Future for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TryJoin<Fut1, Fut2>`

##### `impl IntoFuture for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for TryJoin<Fut1, Fut2>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture> Unpin for TryJoin<Fut1, Fut2>`

### `TryJoin3<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture>`

```rust
struct TryJoin3<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
    Fut3: TryMaybeDone<Fut3>,
}
```

Future for the [`try_join3`](try_join/index.md) function.

#### Implementations

- <span id="tryjoin3-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3> Debug for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2, Fut3> Future for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok, <Fut3 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin3-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TryJoin3<Fut1, Fut2, Fut3>`

##### `impl IntoFuture for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin3-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin3-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin3-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin3-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for TryJoin3<Fut1, Fut2, Fut3>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture> Unpin for TryJoin3<Fut1, Fut2, Fut3>`

### `TryJoin4<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture>`

```rust
struct TryJoin4<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
    Fut3: TryMaybeDone<Fut3>,
    Fut4: TryMaybeDone<Fut4>,
}
```

Future for the [`try_join4`](try_join/index.md) function.

#### Implementations

- <span id="tryjoin4-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4> Debug for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2, Fut3, Fut4> Future for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok, <Fut3 as TryFuture>::Ok, <Fut4 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin4-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

##### `impl IntoFuture for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin4-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin4-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin4-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin4-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture> Unpin for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

### `TryJoin5<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture, Fut5: TryFuture>`

```rust
struct TryJoin5<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture, Fut5: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
    Fut3: TryMaybeDone<Fut3>,
    Fut4: TryMaybeDone<Fut4>,
    Fut5: TryMaybeDone<Fut5>,
}
```

Future for the [`try_join5`](try_join/index.md) function.

#### Implementations

- <span id="tryjoin5-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4, Fut5: Fut5) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4, Fut5> Debug for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2, Fut3, Fut4, Fut5> Future for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok, <Fut3 as TryFuture>::Ok, <Fut4 as TryFuture>::Ok, <Fut5 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin5-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

##### `impl IntoFuture for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin5-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin5-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin5-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin5-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture, Fut5: TryFuture> Unpin for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

### `TryJoinAll<F>`

```rust
struct TryJoinAll<F>
where
    F: TryFuture {
    kind: TryJoinAllKind<F>,
}
```

Future for the [`try_join_all`](try_join_all/index.md) function.

#### Trait Implementations

##### `impl<F> Debug for TryJoinAll<F>`

- <span id="tryjoinall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> FromIterator for TryJoinAll<F>`

- <span id="tryjoinall-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = F>>(iter: T) -> Self`

##### `impl<F> Future for TryJoinAll<F>`

- <span id="tryjoinall-future-type-output"></span>`type Output = Result<Vec<<F as TryFuture>::Ok>, <F as TryFuture>::Error>`

- <span id="tryjoinall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TryJoinAll<F>`

##### `impl<F> IntoFuture for TryJoinAll<F>`

- <span id="tryjoinall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoinall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoinall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryJoinAll<F>`

- <span id="tryjoinall-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoinall-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoinall-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for TryJoinAll<F>`

### `TrySelect<A, B>`

```rust
struct TrySelect<A, B> {
    inner: Option<(A, B)>,
}
```

Future for the [`try_select()`](try_select/index.md) function.

#### Trait Implementations

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for TrySelect<A, B>`

- <span id="tryselect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> Future for TrySelect<A, B>`

- <span id="tryselect-future-type-output"></span>`type Output = Result<Either<(<A as TryFuture>::Ok, B), (<B as TryFuture>::Ok, A)>, Either<(<A as TryFuture>::Error, B), (<B as TryFuture>::Error, A)>>`

- <span id="tryselect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TrySelect<A, B>`

##### `impl IntoFuture for TrySelect<A, B>`

- <span id="tryselect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryselect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryselect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TrySelect<A, B>`

- <span id="tryselect-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryselect-tryfuture-type-error"></span>`type Error = E`

- <span id="tryselect-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for TrySelect<A, B>`

##### `impl<A: Unpin, B: Unpin> Unpin for TrySelect<A, B>`

### `SelectOk<Fut>`

```rust
struct SelectOk<Fut> {
    inner: alloc::vec::Vec<Fut>,
}
```

Future for the [`select_ok`](select_ok/index.md) function.

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for SelectOk<Fut>`

- <span id="selectok-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: TryFuture + Unpin> FromIterator for SelectOk<Fut>`

- <span id="selectok-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Fut>>(iter: T) -> Self`

##### `impl<Fut: TryFuture + Unpin> Future for SelectOk<Fut>`

- <span id="selectok-future-type-output"></span>`type Output = Result<(<Fut as TryFuture>::Ok, Vec<Fut>), <Fut as TryFuture>::Error>`

- <span id="selectok-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for SelectOk<Fut>`

##### `impl IntoFuture for SelectOk<Fut>`

- <span id="selectok-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="selectok-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="selectok-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for SelectOk<Fut>`

- <span id="selectok-tryfuture-type-ok"></span>`type Ok = T`

- <span id="selectok-tryfuture-type-error"></span>`type Error = E`

- <span id="selectok-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for SelectOk<Fut>`

##### `impl<Fut: Unpin> Unpin for SelectOk<Fut>`

### `AbortHandle`

```rust
struct AbortHandle {
    inner: alloc::sync::Arc<AbortInner>,
}
```

A handle to an `Abortable` task.

#### Implementations

- <span id="aborthandle-new-pair"></span>`fn new_pair() -> (Self, AbortRegistration)` — [`AbortRegistration`](../abortable/index.md#abortregistration)

  Creates an (`AbortHandle`, `AbortRegistration`) pair which can be used

  to abort a running future or stream.

  

  This function is usually paired with a call to `Abortable::new`.

#### Trait Implementations

##### `impl Clone for AbortHandle`

- <span id="aborthandle-clone"></span>`fn clone(&self) -> AbortHandle` — [`AbortHandle`](../abortable/index.md#aborthandle)

##### `impl Debug for AbortHandle`

- <span id="aborthandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AbortRegistration`

```rust
struct AbortRegistration {
    inner: alloc::sync::Arc<AbortInner>,
}
```

A registration handle for an `Abortable` task.
Values of this type can be acquired from `AbortHandle::new` and are used
in calls to `Abortable::new`.

#### Implementations

- <span id="abortregistration-handle"></span>`fn handle(&self) -> AbortHandle` — [`AbortHandle`](../abortable/index.md#aborthandle)

  Create an [`AbortHandle`](../abortable/index.md) from the given [`AbortRegistration`](../abortable/index.md).

  

  The created [`AbortHandle`](../abortable/index.md) is functionally the same as any other

  [`AbortHandle`](../abortable/index.md)s that are associated with the same [`AbortRegistration`](../abortable/index.md),

  such as the one created by `AbortHandle::new_pair`.

#### Trait Implementations

##### `impl Debug for AbortRegistration`

- <span id="abortregistration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Abortable<T>`

```rust
struct Abortable<T> {
    task: T,
    inner: alloc::sync::Arc<AbortInner>,
}
```

A future/stream which can be remotely short-circuited using an `AbortHandle`.

#### Implementations

- <span id="abortable-new"></span>`fn new(task: T, reg: AbortRegistration) -> Self` — [`AbortRegistration`](../abortable/index.md#abortregistration)

  Creates a new `Abortable` future/stream using an existing `AbortRegistration`.

  `AbortRegistration`s can be acquired through `AbortHandle::new`.

  

  When `abort` is called on the handle tied to `reg` or if `abort` has

  already been called, the future/stream will complete immediately without making

  any further progress.

  

  # Examples:

  

  Usage with futures:

  

  ```rust

  futures::executor::block_on(async {

  use futures::future::{Abortable, AbortHandle, Aborted};

  

  let (abort_handle, abort_registration) = AbortHandle::new_pair();

  let future = Abortable::new(async { 2 }, abort_registration);

  abort_handle.abort();

  assert_eq!(future.await, Err(Aborted));

  });

  ```

  

  Usage with streams:

  

  ```rust

  futures::executor::block_on(async {

  use futures::future::{Abortable, AbortHandle};

  use futures::stream::{self, StreamExt};

  

  let (abort_handle, abort_registration) = AbortHandle::new_pair();

  let mut stream = Abortable::new(stream::iter(vec![1, 2, 3]), abort_registration);

  abort_handle.abort();

  assert_eq!(stream.next().await, None);

  });

  ```

- <span id="abortable-is-aborted"></span>`fn is_aborted(&self) -> bool`

  Checks whether the task has been aborted. Note that all this

  method indicates is whether `AbortHandle::abort` was *called*.

  This means that it will return `true` even if:

  * `abort` was called after the task had completed.

  * `abort` was called while the task was being polled - the task may still be running and

    will not be stopped until `poll` returns.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Abortable<T>`

- <span id="abortable-clone"></span>`fn clone(&self) -> Abortable<T>` — [`Abortable`](../abortable/index.md#abortable)

##### `impl<T: fmt::Debug> Debug for Abortable<T>`

- <span id="abortable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> Future for Abortable<Fut>`

- <span id="abortable-future-type-output"></span>`type Output = Result<<Fut as Future>::Output, Aborted>`

- <span id="abortable-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<T> FutureExt for Abortable<T>`

##### `impl IntoFuture for Abortable<T>`

- <span id="abortable-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="abortable-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="abortable-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St> Stream for Abortable<St>`

- <span id="abortable-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="abortable-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

##### `impl<T> StreamExt for Abortable<T>`

##### `impl<T> TryFuture for Abortable<T>`

- <span id="abortable-tryfuture-type-ok"></span>`type Ok = T`

- <span id="abortable-tryfuture-type-error"></span>`type Error = E`

- <span id="abortable-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for Abortable<T>`

##### `impl<T> TryStream for Abortable<T>`

- <span id="abortable-trystream-type-ok"></span>`type Ok = T`

- <span id="abortable-trystream-type-error"></span>`type Error = E`

- <span id="abortable-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for Abortable<T>`

##### `impl<T> Unpin for Abortable<T>`

### `Aborted`

```rust
struct Aborted;
```

Indicator that the `Abortable` task was aborted.

#### Trait Implementations

##### `impl Clone for Aborted`

- <span id="aborted-clone"></span>`fn clone(&self) -> Aborted` — [`Aborted`](../abortable/index.md#aborted)

##### `impl Copy for Aborted`

##### `impl Debug for Aborted`

- <span id="aborted-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Aborted`

- <span id="aborted-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Aborted`

##### `impl PartialEq for Aborted`

- <span id="aborted-partialeq-eq"></span>`fn eq(&self, other: &Aborted) -> bool` — [`Aborted`](../abortable/index.md#aborted)

##### `impl StructuralPartialEq for Aborted`

##### `impl ToString for Aborted`

- <span id="aborted-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `MaybeDone<Fut: Future>`

```rust
enum MaybeDone<Fut: Future> {
    Future(Fut),
    Done(<Fut as >::Output),
    Gone,
}
```

A future that may have completed.

This is created by the [`maybe_done()`](maybe_done/index.md) function.

#### Variants

- **`Future`**

  A not-yet-completed future

- **`Done`**

  The output of the completed future

- **`Gone`**

  The empty variant after the result of a [`MaybeDone`](maybe_done/index.md) has been
  taken using the [`take_output`](MaybeDone::take_output) method.

#### Implementations

- <span id="maybedone-output-mut"></span>`fn output_mut(self: Pin<&mut Self>) -> Option<&mut <Fut as >::Output>` — [`Future`](#future)

  Returns an [`Option`](../../serde_core/index.md) containing a mutable reference to the output of the future.

  The output of this method will be [`Some`](../../managed/index.md) if and only if the inner

  future has been completed and [`take_output`](MaybeDone::take_output)

  has not yet been called.

- <span id="maybedone-take-output"></span>`fn take_output(self: Pin<&mut Self>) -> Option<<Fut as >::Output>` — [`Future`](#future)

  Attempt to take the output of a `MaybeDone` without driving it

  towards completion.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Future> Debug for MaybeDone<Fut>`

- <span id="maybedone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> FusedFuture for MaybeDone<Fut>`

- <span id="maybedone-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Future for MaybeDone<Fut>`

- <span id="maybedone-future-type-output"></span>`type Output = ()`

- <span id="maybedone-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for MaybeDone<Fut>`

##### `impl IntoFuture for MaybeDone<Fut>`

- <span id="maybedone-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="maybedone-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="maybedone-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut: Future + Unpin> Unpin for MaybeDone<Fut>`

### `TryMaybeDone<Fut: TryFuture>`

```rust
enum TryMaybeDone<Fut: TryFuture> {
    Future(Fut),
    Done(<Fut as >::Ok),
    Gone,
}
```

A future that may have completed with an error.

This is created by the [`try_maybe_done()`](try_maybe_done/index.md) function.

#### Variants

- **`Future`**

  A not-yet-completed future

- **`Done`**

  The output of the completed future

- **`Gone`**

  The empty variant after the result of a [`TryMaybeDone`](try_maybe_done/index.md) has been
  taken using the [`take_output`](TryMaybeDone::take_output) method,
  or if the future returned an error.

#### Implementations

- <span id="trymaybedone-output-mut"></span>`fn output_mut(self: Pin<&mut Self>) -> Option<&mut <Fut as >::Ok>` — [`TryFuture`](#tryfuture)

  Returns an [`Option`](../../serde_core/index.md) containing a mutable reference to the output of the future.

  The output of this method will be [`Some`](../../managed/index.md) if and only if the inner

  future has completed successfully and [`take_output`](TryMaybeDone::take_output)

  has not yet been called.

- <span id="trymaybedone-take-output"></span>`fn take_output(self: Pin<&mut Self>) -> Option<<Fut as >::Ok>` — [`TryFuture`](#tryfuture)

  Attempt to take the output of a `TryMaybeDone` without driving it

  towards completion.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + TryFuture> Debug for TryMaybeDone<Fut>`

- <span id="trymaybedone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: TryFuture> FusedFuture for TryMaybeDone<Fut>`

- <span id="trymaybedone-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: TryFuture> Future for TryMaybeDone<Fut>`

- <span id="trymaybedone-future-type-output"></span>`type Output = Result<(), <Fut as TryFuture>::Error>`

- <span id="trymaybedone-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for TryMaybeDone<Fut>`

##### `impl IntoFuture for TryMaybeDone<Fut>`

- <span id="trymaybedone-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trymaybedone-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trymaybedone-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryMaybeDone<Fut>`

- <span id="trymaybedone-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trymaybedone-tryfuture-type-error"></span>`type Error = E`

- <span id="trymaybedone-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl<Fut> TryFutureExt for TryMaybeDone<Fut>`

##### `impl<Fut: TryFuture + Unpin> Unpin for TryMaybeDone<Fut>`

### `Either<A, B>`

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

Combines two different futures, streams, or sinks having the same associated types into a single type.

This is useful when conditionally choosing between two distinct future types:

```rust
use futures::future::Either;

futures::executor::block_on(async {
let cond = true;

let fut = if cond {
    Either::Left(async move { 12 })
} else {
    Either::Right(async move { 44 })
};

assert_eq!(fut.await, 12);
})
```

#### Variants

- **`Left`**

  First branch of the type

- **`Right`**

  Second branch of the type

#### Implementations

- <span id="either-as-pin-ref"></span>`fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&A>, Pin<&B>>` — [`Either`](either/index.md#either)

  Convert `Pin<&Either<A, B>>` to `Either<Pin<&A>, Pin<&B>>`,

  pinned projections of the inner variants.

- <span id="either-as-pin-mut"></span>`fn as_pin_mut(self: Pin<&mut Self>) -> Either<Pin<&mut A>, Pin<&mut B>>` — [`Either`](either/index.md#either)

  Convert `Pin<&mut Either<A, B>>` to `Either<Pin<&mut A>, Pin<&mut B>>`,

  pinned projections of the inner variants.

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Either<A, B>`

- <span id="either-clone"></span>`fn clone(&self) -> Either<A, B>` — [`Either`](either/index.md#either)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Either<A, B>`

- <span id="either-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> FusedFuture for Either<A, B>`

- <span id="either-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<A, B> FusedStream for Either<A, B>`

- <span id="either-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<A, B> Future for Either<A, B>`

- <span id="either-future-type-output"></span>`type Output = <A as Future>::Output`

- <span id="either-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl FutureExt for Either<A, B>`

##### `impl IntoFuture for Either<A, B>`

- <span id="either-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="either-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="either-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<A, B, Item> Sink for Either<A, B>`

- <span id="either-sink-type-error"></span>`type Error = <A as Sink>::Error`

- <span id="either-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="either-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../sink/index.md#sink)

- <span id="either-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

- <span id="either-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Sink`](../sink/index.md#sink)

##### `impl<Item> SinkExt for Either<A, B>`

##### `impl<A, B> Stream for Either<A, B>`

- <span id="either-stream-type-item"></span>`type Item = <A as Stream>::Item`

- <span id="either-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

- <span id="either-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Either<A, B>`

##### `impl TryFuture for Either<A, B>`

- <span id="either-tryfuture-type-ok"></span>`type Ok = T`

- <span id="either-tryfuture-type-error"></span>`type Error = E`

- <span id="either-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](#future)

##### `impl TryFutureExt for Either<A, B>`

##### `impl TryStream for Either<A, B>`

- <span id="either-trystream-type-ok"></span>`type Ok = T`

- <span id="either-trystream-type-error"></span>`type Error = E`

- <span id="either-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for Either<A, B>`

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

Adapters specific to [`Result`](../../sel4/error/index.md)-returning futures

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

  Wraps a [`TryFuture`](#tryfuture) into a type that implements

- `fn try_poll_unpin(&mut self, cx: &mut Context<'_>) -> Poll<Result<<Self as >::Ok, <Self as >::Error>>`

  A convenience method for calling `TryFuture::try_poll` on `Unpin`

#### Implementors

- `Fut`

## Functions

### `FutureObj`

```rust
const fn FutureObj(self, rhs: FixedOffset) -> Option<NaiveDateTime>
```

Adds given `FixedOffset` to the current datetime.
Returns `None` if the result would be outside the valid range for `NaiveDateTime`.

This method is similar to [`checked_add_signed`](#method.checked_add_offset), but preserves
leap seconds.

### `LocalFutureObj`

```rust
fn LocalFutureObj(self, rhs: FixedOffset) -> NaiveDateTime
```

Adds given `FixedOffset` to the current datetime.
The resulting value may be outside the valid range of `NaiveDateTime`.

This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`
should not be exposed to library users.

### `UnsafeFutureObj`

```rust
const fn UnsafeFutureObj(self, rhs: Months) -> Option<NaiveDateTime>
```

Subtracts given `Months` from the current date and time.

Uses the last day of the month if the day does not exist in the resulting month.

# Errors

Returns `None` if the resulting date would be out of range.

# Example

```rust
use chrono::{Months, NaiveDate};

assert_eq!(
    NaiveDate::from_ymd_opt(2014, 1, 1)
        .unwrap()
        .and_hms_opt(1, 0, 0)
        .unwrap()
        .checked_sub_months(Months::new(1)),
    Some(NaiveDate::from_ymd_opt(2013, 12, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())
);

assert_eq!(
    NaiveDate::from_ymd_opt(2014, 1, 1)
        .unwrap()
        .and_hms_opt(1, 0, 0)
        .unwrap()
        .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),
    None
);
```

### `lazy`

```rust
fn lazy<F, R>(f: F) -> Lazy<F>
where
    F: FnOnce(&mut futures_core::task::Context<'_>) -> R
```

Creates a new future that allows delayed execution of a closure.

The provided closure is only run once the future is polled.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::lazy(|_| 1);
assert_eq!(a.await, 1);

let b = future::lazy(|_| -> i32 {
    panic!("oh no!")
});
drop(b); // closure is never run
});
```

### `pending`

```rust
fn pending<T>() -> Pending<T>
```

Creates a future which never resolves, representing a computation that never
finishes.

The returned future will forever return `Poll::Pending`.

# Examples

```ignore
futures::executor::block_on(async {
use futures::future;

let future = future::pending();
let () = future.await;
unreachable!();
});
```

### `maybe_done`

```rust
fn maybe_done<Fut: Future>(future: Fut) -> MaybeDone<Fut>
```

Wraps a future into a `MaybeDone`

# Examples

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::future;

let future = future::maybe_done(async { 5 });
let mut future = pin!(future);
assert_eq!(future.as_mut().take_output(), None);
let () = future.as_mut().await;
assert_eq!(future.as_mut().take_output(), Some(5));
assert_eq!(future.as_mut().take_output(), None);
});
```

### `try_maybe_done`

```rust
fn try_maybe_done<Fut: TryFuture>(future: Fut) -> TryMaybeDone<Fut>
```

Wraps a future into a `TryMaybeDone`

### `poll_fn`

```rust
fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut futures_core::task::Context<'_>) -> futures_core::task::Poll<T>
```

Creates a new future wrapping around a function returning [`Poll`](../task/index.md).

Polling the returned future delegates to the wrapped function.

# Examples

```rust
futures::executor::block_on(async {
use futures::future::poll_fn;
use futures::task::{Context, Poll};

fn read_line(_cx: &mut Context<'_>) -> Poll<String> {
    Poll::Ready("Hello, World!".into())
}

let read_future = poll_fn(read_line);
assert_eq!(read_future.await, "Hello, World!".to_owned());
});
```

### `poll_immediate`

```rust
fn poll_immediate<F: Future>(f: F) -> PollImmediate<F>
```

Creates a future that is immediately ready with an Option of a value.
Specifically this means that [poll](core::future::Future::poll()) always returns [Poll::Ready](core::task::Poll::Ready).

# Caution

When consuming the future by this function, note the following:

- This function does not guarantee that the future will run to completion, so it is generally incompatible with passing the non-cancellation-safe future by value.
- Even if the future is cancellation-safe, creating and dropping new futures frequently may lead to performance problems.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let r = future::poll_immediate(async { 1_u32 });
assert_eq!(r.await, Some(1));

let p = future::poll_immediate(future::pending::<i32>());
assert_eq!(p.await, None);
});
```

### Reusing a future

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::future;

let f = async {futures::pending!(); 42_u8};
let mut f = pin!(f);
assert_eq!(None, future::poll_immediate(&mut f).await);
assert_eq!(42, f.await);
});
```

### `err`

```rust
fn err<T, E>(err: E) -> Ready<Result<T, E>>
```

Create a future that is immediately ready with an error value.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::err::<i32, i32>(1);
assert_eq!(a.await, Err(1));
});
```

### `ok`

```rust
fn ok<T, E>(t: T) -> Ready<Result<T, E>>
```

Create a future that is immediately ready with a success value.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ok::<i32, i32>(1);
assert_eq!(a.await, Ok(1));
});
```

### `ready`

```rust
fn ready<T>(t: T) -> Ready<T>
```

Creates a future that is immediately ready with a value.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(1);
assert_eq!(a.await, 1);
});
```

### `always_ready`

```rust
fn always_ready<T, F: Fn() -> T>(prod: F) -> AlwaysReady<T, F>
```

Creates a future that is always immediately ready with a value.

This is particularly useful in avoiding a heap allocation when an API needs `Box<dyn Future<Output = T>>`,
as [`AlwaysReady`](always_ready/index.md) does not have to store a boolean for `is_finished`.

# Examples

```rust
futures::executor::block_on(async {
use std::mem::size_of_val;

use futures::future;

let a = future::always_ready(|| 1);
assert_eq!(size_of_val(&a), 0);
assert_eq!(a.await, 1);
assert_eq!(a.await, 1);
});
```

### `join`

```rust
fn join<Fut1, Fut2>(future1: Fut1, future2: Fut2) -> Join<Fut1, Fut2>
where
    Fut1: Future,
    Fut2: Future
```

Joins the result of two futures, waiting for them both to complete.

This function will return a new future which awaits both futures to
complete. The returned future will finish with a tuple of both results.

Note that this function consumes the passed futures and returns a
wrapped version of it.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let pair = future::join(a, b);

assert_eq!(pair.await, (1, 2));
});
```

### `join3`

```rust
fn join3<Fut1, Fut2, Fut3>(future1: Fut1, future2: Fut2, future3: Fut3) -> Join3<Fut1, Fut2, Fut3>
where
    Fut1: Future,
    Fut2: Future,
    Fut3: Future
```

Same as [`join`](join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let tuple = future::join3(a, b, c);

assert_eq!(tuple.await, (1, 2, 3));
});
```

### `join4`

```rust
fn join4<Fut1, Fut2, Fut3, Fut4>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4) -> Join4<Fut1, Fut2, Fut3, Fut4>
where
    Fut1: Future,
    Fut2: Future,
    Fut3: Future,
    Fut4: Future
```

Same as [`join`](join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let d = async { 4 };
let tuple = future::join4(a, b, c, d);

assert_eq!(tuple.await, (1, 2, 3, 4));
});
```

### `join5`

```rust
fn join5<Fut1, Fut2, Fut3, Fut4, Fut5>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4, future5: Fut5) -> Join5<Fut1, Fut2, Fut3, Fut4, Fut5>
where
    Fut1: Future,
    Fut2: Future,
    Fut3: Future,
    Fut4: Future,
    Fut5: Future
```

Same as [`join`](join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let d = async { 4 };
let e = async { 5 };
let tuple = future::join5(a, b, c, d, e);

assert_eq!(tuple.await, (1, 2, 3, 4, 5));
});
```

### `join_all`

```rust
fn join_all<I>(iter: I) -> JoinAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: Future
```

Creates a future which represents a collection of the outputs of the futures
given.

The returned future will drive execution for all of its underlying futures,
collecting the results into a destination `Vec<T>` in the same order as they
were provided.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# See Also

`join_all` will switch to the more powerful [`FuturesOrdered`](../stream/futures_ordered/index.md) for performance
reasons if the number of futures is large. You may want to look into using it or
its counterpart `FuturesUnordered` directly.

Some examples for additional functionality provided by these are:

 * Adding new futures to the set even after it has been started.

 * Only polling the specific futures that have been woken. In cases where
   you have a lot of futures this will result in much more efficient polling.

# Examples

```rust
futures::executor::block_on(async {
use futures::future::join_all;

async fn foo(i: u32) -> u32 { i }

let futures = vec![foo(1), foo(2), foo(3)];

assert_eq!(join_all(futures).await, [1, 2, 3]);
});
```

### `select`

```rust
fn select<A, B>(future1: A, future2: B) -> Select<A, B>
where
    A: Future + Unpin,
    B: Future + Unpin
```

Waits for either one of two differently-typed futures to complete.

This function will return a new future which awaits for either one of both
futures to complete. The returned future will finish with both the value
resolved and a future representing the completion of the other work.

Note that this function consumes the receiving futures and returns a
wrapped version of them.

Also note that if both this and the second future have the same
output type you can use the `Either::factor_first` method to
conveniently extract out the value at the end.

# Examples

A simple example

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::future;
use futures::future::Either;

// These two futures have different types even though their outputs have the same type.
let future1 = async {
    future::pending::<()>().await; // will never finish
    1
};
let future2 = async {
    future::ready(2).await
};

// 'select' requires Future + Unpin bounds
let future1 = pin!(future1);
let future2 = pin!(future2);

let value = match future::select(future1, future2).await {
    Either::Left((value1, _)) => value1,  // `value1` is resolved from `future1`
                                          // `_` represents `future2`
    Either::Right((value2, _)) => value2, // `value2` is resolved from `future2`
                                          // `_` represents `future1`
};

assert!(value == 2);
});
```

A more complex example

```rust
use futures::future::{self, Either, Future, FutureExt};

// A poor-man's join implemented on top of select

fn join<A, B>(a: A, b: B) -> impl Future<Output=(A::Output, B::Output)>
    where A: Future + Unpin,
          B: Future + Unpin,
{
    future::select(a, b).then(|either| {
        match either {
            Either::Left((x, b)) => b.map(move |y| (x, y)).left_future(),
            Either::Right((y, a)) => a.map(move |x| (x, y)).right_future(),
        }
    })
}
```

### `select_all`

```rust
fn select_all<I>(iter: I) -> SelectAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: Future + Unpin
```

Creates a new future which will select over a list of futures.

The returned future will wait for any future within `iter` to be ready. Upon
completion the item resolved will be returned, along with the index of the
future that was ready and the list of all the remaining futures.

There are no guarantees provided on the order of the list with the remaining
futures. They might be swapped around, reversed, or completely random.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# Panics

This function will panic if the iterator specified contains no items.

### `try_join`

```rust
fn try_join<Fut1, Fut2>(future1: Fut1, future2: Fut2) -> TryJoin<Fut1, Fut2>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>
```

Joins the result of two futures, waiting for them both to complete or
for one to produce an error.

This function will return a new future which awaits both futures to
complete. If successful, the returned future will finish with a tuple of
both results. If unsuccessful, it will complete with the first error
encountered.

Note that this function consumes the passed futures and returns a
wrapped version of it.

# Examples

When used on multiple futures that return [`Ok`](../../flate2/index.md), `try_join` will return
[`Ok`](../../flate2/index.md) of a tuple of the values:

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let pair = future::try_join(a, b);

assert_eq!(pair.await, Ok((1, 2)));
});
```

If one of the futures resolves to an error, `try_join` will return
that error:

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Err::<i32, i32>(2));
let pair = future::try_join(a, b);

assert_eq!(pair.await, Err(2));
});
```

### `try_join3`

```rust
fn try_join3<Fut1, Fut2, Fut3>(future1: Fut1, future2: Fut2, future3: Fut3) -> TryJoin3<Fut1, Fut2, Fut3>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>,
    Fut3: TryFuture<Error = <Fut1 as >::Error>
```

Same as [`try_join`](try_join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let tuple = future::try_join3(a, b, c);

assert_eq!(tuple.await, Ok((1, 2, 3)));
});
```

### `try_join4`

```rust
fn try_join4<Fut1, Fut2, Fut3, Fut4>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4) -> TryJoin4<Fut1, Fut2, Fut3, Fut4>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>,
    Fut3: TryFuture<Error = <Fut1 as >::Error>,
    Fut4: TryFuture<Error = <Fut1 as >::Error>
```

Same as [`try_join`](try_join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let d = future::ready(Ok::<i32, i32>(4));
let tuple = future::try_join4(a, b, c, d);

assert_eq!(tuple.await, Ok((1, 2, 3, 4)));
});
```

### `try_join5`

```rust
fn try_join5<Fut1, Fut2, Fut3, Fut4, Fut5>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4, future5: Fut5) -> TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>,
    Fut3: TryFuture<Error = <Fut1 as >::Error>,
    Fut4: TryFuture<Error = <Fut1 as >::Error>,
    Fut5: TryFuture<Error = <Fut1 as >::Error>
```

Same as [`try_join`](try_join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let d = future::ready(Ok::<i32, i32>(4));
let e = future::ready(Ok::<i32, i32>(5));
let tuple = future::try_join5(a, b, c, d, e);

assert_eq!(tuple.await, Ok((1, 2, 3, 4, 5)));
});
```

### `try_join_all`

```rust
fn try_join_all<I>(iter: I) -> TryJoinAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: TryFuture
```

Creates a future which represents either a collection of the results of the
futures given or an error.

The returned future will drive execution for all of its underlying futures,
collecting the results into a destination `Vec<T>` in the same order as they
were provided.

If any future returns an error then all other futures will be canceled and
an error will be returned immediately. If all futures complete successfully,
however, then the returned future will succeed with a `Vec` of all the
successful results.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# See Also

`try_join_all` will switch to the more powerful [`FuturesOrdered`](../stream/futures_ordered/index.md) for performance
reasons if the number of futures is large. You may want to look into using it or
it's counterpart `FuturesUnordered` directly.

Some examples for additional functionality provided by these are:

 * Adding new futures to the set even after it has been started.

 * Only polling the specific futures that have been woken. In cases where
   you have a lot of futures this will result in much more efficient polling.


# Examples

```rust
futures::executor::block_on(async {
use futures::future::{self, try_join_all};

let futures = vec![
    future::ok::<u32, u32>(1),
    future::ok::<u32, u32>(2),
    future::ok::<u32, u32>(3),
];

assert_eq!(try_join_all(futures).await, Ok(vec![1, 2, 3]));

let futures = vec![
    future::ok::<u32, u32>(1),
    future::err::<u32, u32>(2),
    future::ok::<u32, u32>(3),
];

assert_eq!(try_join_all(futures).await, Err(2));
});
```

### `try_select`

```rust
fn try_select<A, B>(future1: A, future2: B) -> TrySelect<A, B>
where
    A: TryFuture + Unpin,
    B: TryFuture + Unpin
```

Waits for either one of two differently-typed futures to complete.

This function will return a new future which awaits for either one of both
futures to complete. The returned future will finish with both the value
resolved and a future representing the completion of the other work.

Note that this function consumes the receiving futures and returns a
wrapped version of them.

Also note that if both this and the second future have the same
success/error type you can use the `Either::factor_first` method to
conveniently extract out the value at the end.

# Examples

```rust
use futures::future::{self, Either, Future, FutureExt, TryFuture, TryFutureExt};

// A poor-man's try_join implemented on top of select

fn try_join<A, B, E>(a: A, b: B) -> impl TryFuture<Ok=(A::Ok, B::Ok), Error=E>
     where A: TryFuture<Error = E> + Unpin + 'static,
           B: TryFuture<Error = E> + Unpin + 'static,
           E: 'static,
{
    future::try_select(a, b).then(|res| -> Box<dyn Future<Output = Result<_, _>> + Unpin> {
        match res {
            Ok(Either::Left((x, b))) => Box::new(b.map_ok(move |y| (x, y))),
            Ok(Either::Right((y, a))) => Box::new(a.map_ok(move |x| (x, y))),
            Err(Either::Left((e, _))) => Box::new(future::err(e)),
            Err(Either::Right((e, _))) => Box::new(future::err(e)),
        }
    })
}
```

### `select_ok`

```rust
fn select_ok<I>(iter: I) -> SelectOk<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: TryFuture + Unpin
```

Creates a new future which will select the first successful future over a list of futures.

The returned future will wait for any future within `iter` to be ready and Ok. Unlike
`select_all`, this will only return the first successful completion, or the last
failure. This is useful in contexts where any success is desired and failures
are ignored, unless all the futures fail.

 This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# Panics

This function will panic if the iterator specified contains no items.

### `abortable`

```rust
fn abortable<Fut>(future: Fut) -> (crate::future::Abortable<Fut>, crate::future::AbortHandle)
where
    Fut: Future
```

Creates a new `Abortable` future and an `AbortHandle` which can be used to stop it.

This function is a convenient (but less flexible) alternative to calling
`AbortHandle::new` and `Abortable::new` manually.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

### `assert_future`

```rust
fn assert_future<T, F>(future: F) -> F
where
    F: Future<Output = T>
```

