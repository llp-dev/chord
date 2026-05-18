*[futures_util](../../../index.md) / [future](../../index.md) / [try_future](../index.md) / [try_flatten](index.md)*

---

# Module `try_flatten`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFlatten`](#tryflatten) | enum |  |

## Enums

### `TryFlatten<Fut1, Fut2>`

```rust
enum TryFlatten<Fut1, Fut2> {
    First {
        f: Fut1,
    },
    Second {
        f: Fut2,
    },
    Empty,
}
```

#### Implementations

- <span id="tryflatten-new"></span>`fn new(future: Fut1) -> Self`

#### Trait Implementations

##### `impl<Fut1: fmt::Debug, Fut2: fmt::Debug> Debug for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> FusedFuture for TryFlatten<Fut, <Fut as >::Ok>`

- <span id="tryflatten-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut> FusedStream for TryFlatten<Fut, <Fut as >::Ok>`

- <span id="tryflatten-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut> Future for TryFlatten<Fut, <Fut as >::Ok>`

- <span id="tryflatten-future-type-output"></span>`type Output = Result<<<Fut as TryFuture>::Ok as TryFuture>::Ok, <Fut as TryFuture>::Error>`

- <span id="tryflatten-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl FutureExt for TryFlatten<Fut1, Fut2>`

##### `impl IntoFuture for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryflatten-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryflatten-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut, Item> Sink for TryFlatten<Fut, <Fut as >::Ok>`

- <span id="tryflatten-sink-type-error"></span>`type Error = <Fut as TryFuture>::Error`

- <span id="tryflatten-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryflatten-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="tryflatten-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryflatten-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlatten<Fut1, Fut2>`

##### `impl<Fut> Stream for TryFlatten<Fut, <Fut as >::Ok>`

- <span id="tryflatten-stream-type-item"></span>`type Item = Result<<<Fut as TryFuture>::Ok as TryStream>::Ok, <Fut as TryFuture>::Error>`

- <span id="tryflatten-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../../stream/index.md#stream)

##### `impl StreamExt for TryFlatten<Fut1, Fut2>`

##### `impl TryFuture for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryflatten-tryfuture-type-error"></span>`type Error = E`

- <span id="tryflatten-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl TryFutureExt for TryFlatten<Fut1, Fut2>`

##### `impl TryStream for TryFlatten<Fut1, Fut2>`

- <span id="tryflatten-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflatten-trystream-type-error"></span>`type Error = E`

- <span id="tryflatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../../stream/index.md#trystream)

##### `impl TryStreamExt for TryFlatten<Fut1, Fut2>`

##### `impl<Fut1, Fut2> Unpin for TryFlatten<Fut1, Fut2>`

