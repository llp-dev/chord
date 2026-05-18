*[futures_util](../../index.md) / [sink](../index.md) / [err_into](index.md)*

---

# Module `err_into`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SinkErrInto`](#sinkerrinto) | struct | Sink for the [`sink_err_into`](super::SinkExt::sink_err_into) method. |

## Structs

### `SinkErrInto<Si: Sink<Item>, Item, E>`

```rust
struct SinkErrInto<Si: Sink<Item>, Item, E> {
    sink: crate::sink::SinkMapErr<Si, fn(<Si as >::Error) -> E>,
}
```

Sink for the [`sink_err_into`](super::SinkExt::sink_err_into) method.

#### Implementations

- <span id="sinkerrinto-new"></span>`fn new(sink: Si) -> Self`

- <span id="sinkerrinto-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="sinkerrinto-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkerrinto-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkerrinto-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<Si: fmt::Debug + Sink<Item>, Item: fmt::Debug, E: fmt::Debug> Debug for SinkErrInto<Si, Item, E>`

- <span id="sinkerrinto-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item, E> FusedStream for SinkErrInto<S, Item, E>`

- <span id="sinkerrinto-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, Item, E> Sink for SinkErrInto<Si, Item, E>`

- <span id="sinkerrinto-sink-type-error"></span>`type Error = E`

- <span id="sinkerrinto-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="sinkerrinto-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="sinkerrinto-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="sinkerrinto-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<Item> SinkExt for SinkErrInto<Si, Item, E>`

##### `impl<S, Item, E> Stream for SinkErrInto<S, Item, E>`

- <span id="sinkerrinto-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="sinkerrinto-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="sinkerrinto-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for SinkErrInto<Si, Item, E>`

##### `impl<E> TryStream for SinkErrInto<Si, Item, E>`

- <span id="sinkerrinto-trystream-type-ok"></span>`type Ok = T`

- <span id="sinkerrinto-trystream-type-error"></span>`type Error = E`

- <span id="sinkerrinto-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for SinkErrInto<Si, Item, E>`

##### `impl<Si: Sink<Item>, Item, E> Unpin for SinkErrInto<Si, Item, E>`

