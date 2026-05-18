*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_flatten_unordered](index.md)*

---

# Module `try_flatten_unordered`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFlattenUnordered`](#tryflattenunordered) | struct | Stream for the [`try_flatten_unordered`](super::TryStreamExt::try_flatten_unordered) method. |
| [`NestedTryStreamIntoEitherTryStream`](#nestedtrystreamintoeithertrystream) | struct | Emits either successful streams or single-item streams containing the underlying errors. |
| [`Single`](#single) | struct | Emits a single item immediately, then stream will be terminated. |
| [`PropagateBaseStreamError`](#propagatebasestreamerror) | struct | Immediately propagates errors occurred in the base stream. |
| [`BaseStreamItem`](#basestreamitem) | type |  |
| [`InnerStreamItem`](#innerstreamitem) | type |  |
| [`SingleStreamResult`](#singlestreamresult) | type |  |

## Structs

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

- <span id="tryflattenunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryflattenunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: _Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="tryflattenunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryflattenunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlattenUnordered<St>`

##### `impl<St> Stream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-stream-type-item"></span>`type Item = <FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>> as Stream>::Item`

- <span id="tryflattenunordered-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="tryflattenunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFlattenUnordered<St>`

##### `impl TryStream for TryFlattenUnordered<St>`

- <span id="tryflattenunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflattenunordered-trystream-type-error"></span>`type Error = E`

- <span id="tryflattenunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for TryFlattenUnordered<St>`

##### `impl<St> Unpin for TryFlattenUnordered<St>`

### `NestedTryStreamIntoEitherTryStream<St>`

```rust
struct NestedTryStreamIntoEitherTryStream<St>
where
    St: TryStream,
    <St as >::Ok: TryStream + Unpin,
    <<St as >::Ok as TryStream>::Error: From<<St as >::Error> {
    stream: St,
}
```

Emits either successful streams or single-item streams containing the underlying errors.
This's a wrapper for `FlattenUnordered` to reuse its logic over `TryStream`.

#### Implementations

- <span id="nestedtrystreamintoeithertrystream-new"></span>`fn new(stream: St) -> Self`

- <span id="nestedtrystreamintoeithertrystream-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="nestedtrystreamintoeithertrystream-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="nestedtrystreamintoeithertrystream-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="nestedtrystreamintoeithertrystream-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for NestedTryStreamIntoEitherTryStream<St>`

- <span id="nestedtrystreamintoeithertrystream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FlowController for PropagateBaseStreamError<St>`

- <span id="propagatebasestreamerror-flowcontroller-next-step"></span>`fn next_step(item: <NestedTryStreamIntoEitherTryStream<St> as Stream>::Item) -> FlowStep<<NestedTryStreamIntoEitherTryStream<St> as Stream>::Item, <<NestedTryStreamIntoEitherTryStream<St> as Stream>::Item as Stream>::Item>` — [`NestedTryStreamIntoEitherTryStream`](#nestedtrystreamintoeithertrystream), [`Stream`](../../index.md#stream), [`FlowStep`](../../stream/flatten_unordered/index.md#flowstep)

##### `impl<St> FusedStream for NestedTryStreamIntoEitherTryStream<St>`

- <span id="nestedtrystreamintoeithertrystream-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Item> Sink for NestedTryStreamIntoEitherTryStream<St>`

- <span id="nestedtrystreamintoeithertrystream-sink-type-error"></span>`type Error = <St as Sink>::Error`

- <span id="nestedtrystreamintoeithertrystream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="nestedtrystreamintoeithertrystream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="nestedtrystreamintoeithertrystream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="nestedtrystreamintoeithertrystream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for NestedTryStreamIntoEitherTryStream<St>`

##### `impl<St> Stream for NestedTryStreamIntoEitherTryStream<St>`

- <span id="nestedtrystreamintoeithertrystream-stream-type-item"></span>`type Item = Either<IntoStream<<St as TryStream>::Ok>, Single<Result<<<St as TryStream>::Ok as TryStream>::Ok, <<St as TryStream>::Ok as TryStream>::Error>>>`

- <span id="nestedtrystreamintoeithertrystream-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

##### `impl StreamExt for NestedTryStreamIntoEitherTryStream<St>`

##### `impl<St> Unpin for NestedTryStreamIntoEitherTryStream<St>`

### `Single<T>`

```rust
struct Single<T>(Option<T>);
```

Emits a single item immediately, then stream will be terminated.

#### Implementations

- <span id="single-new"></span>`fn new(val: T) -> Self`

  Constructs new `Single` with the given value.

- <span id="single-next-immediate"></span>`fn next_immediate(&mut self) -> Option<T>`

  Attempts to take inner item immediately. Will always succeed if the stream isn't terminated.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Single<T>`

- <span id="single-clone"></span>`fn clone(&self) -> Single<T>` — [`Single`](#single)

##### `impl<T: fmt::Debug> Debug for Single<T>`

- <span id="single-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Stream for Single<T>`

- <span id="single-stream-type-item"></span>`type Item = T`

- <span id="single-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="single-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for Single<T>`

##### `impl<T> TryStream for Single<T>`

- <span id="single-trystream-type-ok"></span>`type Ok = T`

- <span id="single-trystream-type-error"></span>`type Error = E`

- <span id="single-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Single<T>`

##### `impl<T> Unpin for Single<T>`

### `PropagateBaseStreamError<St>`

```rust
struct PropagateBaseStreamError<St>(core::marker::PhantomData<St>);
```

Immediately propagates errors occurred in the base stream.

#### Trait Implementations

##### `impl<St: clone::Clone> Clone for PropagateBaseStreamError<St>`

- <span id="propagatebasestreamerror-clone"></span>`fn clone(&self) -> PropagateBaseStreamError<St>` — [`PropagateBaseStreamError`](#propagatebasestreamerror)

##### `impl<St: marker::Copy> Copy for PropagateBaseStreamError<St>`

##### `impl<St: fmt::Debug> Debug for PropagateBaseStreamError<St>`

- <span id="propagatebasestreamerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FlowController for PropagateBaseStreamError<St>`

- <span id="propagatebasestreamerror-flowcontroller-next-step"></span>`fn next_step(item: <NestedTryStreamIntoEitherTryStream<St> as Stream>::Item) -> FlowStep<<NestedTryStreamIntoEitherTryStream<St> as Stream>::Item, <<NestedTryStreamIntoEitherTryStream<St> as Stream>::Item as Stream>::Item>` — [`NestedTryStreamIntoEitherTryStream`](#nestedtrystreamintoeithertrystream), [`Stream`](../../index.md#stream), [`FlowStep`](../../stream/flatten_unordered/index.md#flowstep)

## Type Aliases

### `BaseStreamItem<St>`

```rust
type BaseStreamItem<St> = <NestedTryStreamIntoEitherTryStream<St> as Stream>::Item;
```

### `InnerStreamItem<St>`

```rust
type InnerStreamItem<St> = <<NestedTryStreamIntoEitherTryStream<St> as Stream>::Item as Stream>::Item;
```

### `SingleStreamResult<St>`

```rust
type SingleStreamResult<St> = Single<Result<<St as TryStream>::Ok, <St as TryStream>::Error>>;
```

