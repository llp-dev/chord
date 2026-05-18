*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [then](index.md)*

---

# Module `then`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Then`](#then) | struct | Stream for the [`then`](super::StreamExt::then) method. |

## Structs

### `Then<St, Fut, F>`

```rust
struct Then<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`then`](super::StreamExt::then) method.

#### Implementations

- <span id="then-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="then-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="then-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="then-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="then-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for Then<St, Fut, F>`

- <span id="then-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for Then<St, Fut, F>`

- <span id="then-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for Then<S, Fut, F>`

- <span id="then-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="then-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="then-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="then-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="then-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Then<St, Fut, F>`

##### `impl<St, Fut, F> Stream for Then<St, Fut, F>`

- <span id="then-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="then-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="then-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Then<St, Fut, F>`

##### `impl TryStream for Then<St, Fut, F>`

- <span id="then-trystream-type-ok"></span>`type Ok = T`

- <span id="then-trystream-type-error"></span>`type Error = E`

- <span id="then-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Then<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for Then<St, Fut, F>`

