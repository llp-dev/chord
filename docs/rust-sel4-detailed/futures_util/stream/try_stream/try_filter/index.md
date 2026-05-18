*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_filter](index.md)*

---

# Module `try_filter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFilter`](#tryfilter) | struct | Stream for the [`try_filter`](super::TryStreamExt::try_filter) method. |

## Structs

### `TryFilter<St, Fut, F>`

```rust
struct TryFilter<St, Fut, F>
where
    St: TryStream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Ok>,
}
```

Stream for the [`try_filter`](super::TryStreamExt::try_filter)
method.

#### Implementations

- <span id="tryfilter-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="tryfilter-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryfilter-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfilter-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryfilter-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryFilter<St, Fut, F>`

- <span id="tryfilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TryFilter<St, Fut, F>`

- <span id="tryfilter-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TryFilter<S, Fut, F>`

- <span id="tryfilter-sink-type-error"></span>`type Error = E`

- <span id="tryfilter-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryfilter-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="tryfilter-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryfilter-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFilter<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TryFilter<St, Fut, F>`

- <span id="tryfilter-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryfilter-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="tryfilter-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TryFilter<St, Fut, F>`

##### `impl TryStream for TryFilter<St, Fut, F>`

- <span id="tryfilter-trystream-type-ok"></span>`type Ok = T`

- <span id="tryfilter-trystream-type-error"></span>`type Error = E`

- <span id="tryfilter-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for TryFilter<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryFilter<St, Fut, F>`

