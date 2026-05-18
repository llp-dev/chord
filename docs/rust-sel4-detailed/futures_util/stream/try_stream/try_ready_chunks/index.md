*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_ready_chunks](index.md)*

---

# Module `try_ready_chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReadyChunks`](#tryreadychunks) | struct | Stream for the [`try_ready_chunks`](super::TryStreamExt::try_ready_chunks) method. |
| [`TryReadyChunksError`](#tryreadychunkserror) | struct | Error indicating, that while chunk was collected inner stream produced an error. |
| [`TryReadyChunksStreamError`](#tryreadychunksstreamerror) | type |  |

## Structs

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

- <span id="tryreadychunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryreadychunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="tryreadychunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryreadychunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryReadyChunks<St>`

##### `impl<St: TryStream> Stream for TryReadyChunks<St>`

- <span id="tryreadychunks-stream-type-item"></span>`type Item = Result<Vec<<St as TryStream>::Ok>, TryReadyChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>>`

- <span id="tryreadychunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="tryreadychunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryReadyChunks<St>`

##### `impl TryStream for TryReadyChunks<St>`

- <span id="tryreadychunks-trystream-type-ok"></span>`type Ok = T`

- <span id="tryreadychunks-trystream-type-error"></span>`type Error = E`

- <span id="tryreadychunks-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

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

- <span id="tryreadychunkserror-partialeq-eq"></span>`fn eq(&self, other: &TryReadyChunksError<T, E>) -> bool` — [`TryReadyChunksError`](#tryreadychunkserror)

##### `impl<T, E> StructuralPartialEq for TryReadyChunksError<T, E>`

##### `impl<T> ToString for TryReadyChunksError<T, E>`

- <span id="tryreadychunkserror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `TryReadyChunksStreamError<St>`

```rust
type TryReadyChunksStreamError<St> = TryReadyChunksError<<St as TryStream>::Ok, <St as TryStream>::Error>;
```

