*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_buffer_unordered](index.md)*

---

# Module `try_buffer_unordered`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryBufferUnordered`](#trybufferunordered) | struct | Stream for the [`try_buffer_unordered`](super::TryStreamExt::try_buffer_unordered) method. |

## Structs

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

- <span id="trybufferunordered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="trybufferunordered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="trybufferunordered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="trybufferunordered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryBufferUnordered<St>`

##### `impl<St> Stream for TryBufferUnordered<St>`

- <span id="trybufferunordered-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="trybufferunordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

##### `impl StreamExt for TryBufferUnordered<St>`

##### `impl TryStream for TryBufferUnordered<St>`

- <span id="trybufferunordered-trystream-type-ok"></span>`type Ok = T`

- <span id="trybufferunordered-trystream-type-error"></span>`type Error = E`

- <span id="trybufferunordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for TryBufferUnordered<St>`

##### `impl<St> Unpin for TryBufferUnordered<St>`

