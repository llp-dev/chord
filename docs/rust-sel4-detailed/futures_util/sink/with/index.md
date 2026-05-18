*[futures_util](../../index.md) / [sink](../index.md) / [with](index.md)*

---

# Module `with`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`With`](#with) | struct | Sink for the [`with`](super::SinkExt::with) method. |

## Structs

### `With<Si, Item, U, Fut, F>`

```rust
struct With<Si, Item, U, Fut, F> {
    sink: Si,
    f: F,
    state: Option<Fut>,
    _phantom: core::marker::PhantomData<fn(U) -> Item>,
}
```

Sink for the [`with`](super::SinkExt::with) method.

#### Implementations

- <span id="with-new"></span>`fn new<E>(sink: Si, f: F) -> Self`

#### Trait Implementations

##### `impl<Si, Item, U, Fut, F> Clone for With<Si, Item, U, Fut, F>`

- <span id="with-clone"></span>`fn clone(&self) -> Self`

##### `impl<Si, Item, U, Fut, F> Debug for With<Si, Item, U, Fut, F>`

- <span id="with-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item, U, Fut, F> FusedStream for With<S, Item, U, Fut, F>`

- <span id="with-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, Item, U, Fut, F> Sink for With<Si, Item, U, Fut, F>`

- <span id="with-sink-type-error"></span>`type Error = E`

- <span id="with-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="with-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: U) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="with-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="with-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<Item> SinkExt for With<Si, Item, U, Fut, F>`

##### `impl<S, Item, U, Fut, F> Stream for With<S, Item, U, Fut, F>`

- <span id="with-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="with-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="with-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for With<Si, Item, U, Fut, F>`

##### `impl TryStream for With<Si, Item, U, Fut, F>`

- <span id="with-trystream-type-ok"></span>`type Ok = T`

- <span id="with-trystream-type-error"></span>`type Error = E`

- <span id="with-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for With<Si, Item, U, Fut, F>`

##### `impl<Si, Item, U, Fut, F> Unpin for With<Si, Item, U, Fut, F>`

