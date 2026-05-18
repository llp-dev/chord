*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [filter_map](index.md)*

---

# Module `filter_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FilterMap`](#filtermap) | struct | Stream for the [`filter_map`](super::StreamExt::filter_map) method. |

## Structs

### `FilterMap<St, Fut, F>`

```rust
struct FilterMap<St, Fut, F> {
    stream: St,
    f: F,
    pending: Option<Fut>,
}
```

Stream for the [`filter_map`](super::StreamExt::filter_map) method.

#### Implementations

- <span id="filtermap-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="filtermap-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="filtermap-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filtermap-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filtermap-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for FilterMap<St, Fut, F>`

- <span id="filtermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for FilterMap<St, Fut, F>`

- <span id="filtermap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for FilterMap<S, Fut, F>`

- <span id="filtermap-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="filtermap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="filtermap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="filtermap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="filtermap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for FilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Stream for FilterMap<St, Fut, F>`

- <span id="filtermap-stream-type-item"></span>`type Item = T`

- <span id="filtermap-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

- <span id="filtermap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for FilterMap<St, Fut, F>`

##### `impl TryStream for FilterMap<St, Fut, F>`

- <span id="filtermap-trystream-type-ok"></span>`type Ok = T`

- <span id="filtermap-trystream-type-error"></span>`type Error = E`

- <span id="filtermap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for FilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for FilterMap<St, Fut, F>`

