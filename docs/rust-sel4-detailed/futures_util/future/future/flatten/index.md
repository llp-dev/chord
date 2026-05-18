*[futures_util](../../../index.md) / [future](../../index.md) / [future](../index.md) / [flatten](index.md)*

---

# Module `flatten`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flatten`](#flatten) | enum |  |

## Enums

### `Flatten<Fut1, Fut2>`

```rust
enum Flatten<Fut1, Fut2> {
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

- <span id="flatten-new"></span>`fn new(future: Fut1) -> Self`

#### Trait Implementations

##### `impl<Fut1: fmt::Debug, Fut2: fmt::Debug> Debug for Flatten<Fut1, Fut2>`

- <span id="flatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> FusedFuture for Flatten<Fut, <Fut as >::Output>`

- <span id="flatten-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut> FusedStream for Flatten<Fut, <Fut as >::Output>`

- <span id="flatten-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut> Future for Flatten<Fut, <Fut as >::Output>`

- <span id="flatten-future-type-output"></span>`type Output = <<Fut as Future>::Output as Future>::Output`

- <span id="flatten-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl FutureExt for Flatten<Fut1, Fut2>`

##### `impl IntoFuture for Flatten<Fut1, Fut2>`

- <span id="flatten-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="flatten-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="flatten-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut, Item> Sink for Flatten<Fut, <Fut as >::Output>`

- <span id="flatten-sink-type-error"></span>`type Error = <<Fut as Future>::Output as Sink>::Error`

- <span id="flatten-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="flatten-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="flatten-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="flatten-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Flatten<Fut1, Fut2>`

##### `impl<Fut> Stream for Flatten<Fut, <Fut as >::Output>`

- <span id="flatten-stream-type-item"></span>`type Item = <<Fut as Future>::Output as Stream>::Item`

- <span id="flatten-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../../stream/index.md#stream)

##### `impl StreamExt for Flatten<Fut1, Fut2>`

##### `impl TryFuture for Flatten<Fut1, Fut2>`

- <span id="flatten-tryfuture-type-ok"></span>`type Ok = T`

- <span id="flatten-tryfuture-type-error"></span>`type Error = E`

- <span id="flatten-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl TryFutureExt for Flatten<Fut1, Fut2>`

##### `impl TryStream for Flatten<Fut1, Fut2>`

- <span id="flatten-trystream-type-ok"></span>`type Ok = T`

- <span id="flatten-trystream-type-error"></span>`type Error = E`

- <span id="flatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../../stream/index.md#trystream)

##### `impl TryStreamExt for Flatten<Fut1, Fut2>`

##### `impl<Fut1, Fut2> Unpin for Flatten<Fut1, Fut2>`

