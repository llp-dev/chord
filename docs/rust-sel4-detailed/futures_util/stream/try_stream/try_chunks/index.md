*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_chunks](index.md)*

---

# Module `try_chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryChunks`](#trychunks) | struct | Stream for the [`try_chunks`](super::TryStreamExt::try_chunks) method. |
| [`TryChunksError`](#trychunkserror) | struct | Error indicating, that while chunk was collected inner stream produced an error. |
| [`TryChunksStreamError`](#trychunksstreamerror) | type |  |

## Structs

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

- <span id="trychunks-take"></span>`fn take(self: Pin<&mut Self>) -> Vec<<St as >::Ok>` — [`TryStream`](../../index.md#trystream)

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

- <span id="trychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="trychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="trychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="trychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryChunks<St>`

##### `impl<St: TryStream> Stream for TryChunks<St>`

- <span id="trychunks-stream-type-item"></span>`type Item = Result<Vec<<St as TryStream>::Ok>, TryChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>>`

- <span id="trychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="trychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryChunks<St>`

##### `impl TryStream for TryChunks<St>`

- <span id="trychunks-trystream-type-ok"></span>`type Ok = T`

- <span id="trychunks-trystream-type-error"></span>`type Error = E`

- <span id="trychunks-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

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

- <span id="trychunkserror-partialeq-eq"></span>`fn eq(&self, other: &TryChunksError<T, E>) -> bool` — [`TryChunksError`](#trychunkserror)

##### `impl<T, E> StructuralPartialEq for TryChunksError<T, E>`

##### `impl<T> ToString for TryChunksError<T, E>`

- <span id="trychunkserror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `TryChunksStreamError<St>`

```rust
type TryChunksStreamError<St> = TryChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>;
```

