*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [filter](index.md)*

---

# Module `filter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Filter`](#filter) | struct | Stream for the [`filter`](super::StreamExt::filter) method. |

## Structs

### `Filter<St, Fut, F>`

```rust
struct Filter<St, Fut, F>
where
    St: Stream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Item>,
}
```

Stream for the [`filter`](super::StreamExt::filter) method.

#### Implementations

- <span id="filter-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="filter-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="filter-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filter-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="filter-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for Filter<St, Fut, F>`

- <span id="filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for Filter<St, Fut, F>`

- <span id="filter-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for Filter<S, Fut, F>`

- <span id="filter-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="filter-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="filter-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="filter-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="filter-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Filter<St, Fut, F>`

##### `impl<St, Fut, F> Stream for Filter<St, Fut, F>`

- <span id="filter-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="filter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="filter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Filter<St, Fut, F>`

##### `impl TryStream for Filter<St, Fut, F>`

- <span id="filter-trystream-type-ok"></span>`type Ok = T`

- <span id="filter-trystream-type-error"></span>`type Error = E`

- <span id="filter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Filter<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for Filter<St, Fut, F>`

