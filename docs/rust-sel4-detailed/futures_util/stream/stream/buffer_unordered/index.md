*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [buffer_unordered](index.md)*

---

# Module `buffer_unordered`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BufferUnordered`](#bufferunordered) | struct | Stream for the [`buffer_unordered`](super::StreamExt::buffer_unordered) method. |

## Structs

### `BufferUnordered<St>`

```rust
struct BufferUnordered<St>
where
    St: Stream {
    stream: crate::stream::Fuse<St>,
    in_progress_queue: crate::stream::FuturesUnordered<<St as >::Item>,
    max: usize,
}
```

Stream for the [`buffer_unordered`](super::StreamExt::buffer_unordered)
method.

#### Implementations

- <span id="bufferunordered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="bufferunordered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="bufferunordered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="bufferunordered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="bufferunordered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for BufferUnordered<St>`

- <span id="bufferunordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for BufferUnordered<St>`

- <span id="bufferunordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for BufferUnordered<S>`

- <span id="bufferunordered-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="bufferunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="bufferunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="bufferunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="bufferunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for BufferUnordered<St>`

##### `impl<St> Stream for BufferUnordered<St>`

- <span id="bufferunordered-stream-type-item"></span>`type Item = <<St as Stream>::Item as Future>::Output`

- <span id="bufferunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="bufferunordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for BufferUnordered<St>`

##### `impl TryStream for BufferUnordered<St>`

- <span id="bufferunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="bufferunordered-trystream-type-error"></span>`type Error = E`

- <span id="bufferunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for BufferUnordered<St>`

##### `impl<St> Unpin for BufferUnordered<St>`

