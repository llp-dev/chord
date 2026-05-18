*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_filter_map](index.md)*

---

# Module `try_filter_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFilterMap`](#tryfiltermap) | struct | Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map) method. |

## Structs

### `TryFilterMap<St, Fut, F>`

```rust
struct TryFilterMap<St, Fut, F> {
    stream: St,
    f: F,
    pending: Option<Fut>,
}
```

Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map)
method.

#### Implementations

- <span id="tryfiltermap-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryfiltermap-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryfiltermap-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfiltermap-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfiltermap-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryFilterMap<S, Fut, F>`

- <span id="tryfiltermap-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryfiltermap-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryfiltermap-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="tryfiltermap-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryfiltermap-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-stream-type-item"></span>`type Item = Result<T, <St as TryStream>::Error>`

- <span id="tryfiltermap-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="tryfiltermap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFilterMap<St, Fut, F>`

##### `impl TryStream for TryFilterMap<St, Fut, F>`

- <span id="tryfiltermap-trystream-type-ok"></span>`type Ok = T`

- <span id="tryfiltermap-trystream-type-error"></span>`type Error = E`

- <span id="tryfiltermap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for TryFilterMap<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryFilterMap<St, Fut, F>`

