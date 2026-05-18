*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [buffered](index.md)*

---

# Module `buffered`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buffered`](#buffered) | struct | Stream for the [`buffered`](super::StreamExt::buffered) method. |

## Structs

### `Buffered<St>`

```rust
struct Buffered<St>
where
    St: Stream,
    <St as >::Item: Future {
    stream: crate::stream::Fuse<St>,
    in_progress_queue: crate::stream::FuturesOrdered<<St as >::Item>,
    max: usize,
}
```

Stream for the [`buffered`](super::StreamExt::buffered) method.

#### Implementations

- <span id="buffered-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="buffered-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="buffered-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffered-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffered-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for Buffered<St>`

- <span id="buffered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for Buffered<St>`

- <span id="buffered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Buffered<S>`

- <span id="buffered-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="buffered-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="buffered-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="buffered-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="buffered-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Buffered<St>`

##### `impl<St> Stream for Buffered<St>`

- <span id="buffered-stream-type-item"></span>`type Item = <<St as Stream>::Item as Future>::Output`

- <span id="buffered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="buffered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Buffered<St>`

##### `impl TryStream for Buffered<St>`

- <span id="buffered-trystream-type-ok"></span>`type Ok = T`

- <span id="buffered-trystream-type-error"></span>`type Error = E`

- <span id="buffered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Buffered<St>`

##### `impl<St> Unpin for Buffered<St>`

