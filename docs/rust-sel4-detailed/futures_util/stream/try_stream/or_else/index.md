*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [or_else](index.md)*

---

# Module `or_else`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OrElse`](#orelse) | struct | Stream for the [`or_else`](super::TryStreamExt::or_else) method. |

## Structs

### `OrElse<St, Fut, F>`

```rust
struct OrElse<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`or_else`](super::TryStreamExt::or_else) method.

#### Implementations

- <span id="orelse-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="orelse-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="orelse-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="orelse-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="orelse-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for OrElse<St, Fut, F>`

- <span id="orelse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for OrElse<St, Fut, F>`

- <span id="orelse-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for OrElse<S, Fut, F>`

- <span id="orelse-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="orelse-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="orelse-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="orelse-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="orelse-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for OrElse<St, Fut, F>`

##### `impl<St, Fut, F> Stream for OrElse<St, Fut, F>`

- <span id="orelse-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <Fut as TryFuture>::Error>`

- <span id="orelse-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="orelse-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for OrElse<St, Fut, F>`

##### `impl TryStream for OrElse<St, Fut, F>`

- <span id="orelse-trystream-type-ok"></span>`type Ok = T`

- <span id="orelse-trystream-type-error"></span>`type Error = E`

- <span id="orelse-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for OrElse<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for OrElse<St, Fut, F>`

