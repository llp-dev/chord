*[futures_util](../../index.md) / [sink](../index.md) / [with_flat_map](index.md)*

---

# Module `with_flat_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WithFlatMap`](#withflatmap) | struct | Sink for the [`with_flat_map`](super::SinkExt::with_flat_map) method. |

## Structs

### `WithFlatMap<Si, Item, U, St, F>`

```rust
struct WithFlatMap<Si, Item, U, St, F> {
    sink: Si,
    f: F,
    stream: Option<St>,
    buffer: Option<Item>,
    _marker: core::marker::PhantomData<fn(U)>,
}
```

Sink for the [`with_flat_map`](super::SinkExt::with_flat_map) method.

#### Implementations

- <span id="withflatmap-new"></span>`fn new(sink: Si, f: F) -> Self`

- <span id="withflatmap-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="withflatmap-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="withflatmap-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="withflatmap-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="withflatmap-try-empty-stream"></span>`fn try_empty_stream(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Si as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

#### Trait Implementations

##### `impl<Si, Item, U, St, F> Debug for WithFlatMap<Si, Item, U, St, F>`

- <span id="withflatmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item, U, St, F> FusedStream for WithFlatMap<S, Item, U, St, F>`

- <span id="withflatmap-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, Item, U, St, F> Sink for WithFlatMap<Si, Item, U, St, F>`

- <span id="withflatmap-sink-type-error"></span>`type Error = <Si as Sink>::Error`

- <span id="withflatmap-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="withflatmap-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: U) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="withflatmap-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="withflatmap-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<Item> SinkExt for WithFlatMap<Si, Item, U, St, F>`

##### `impl<S, Item, U, St, F> Stream for WithFlatMap<S, Item, U, St, F>`

- <span id="withflatmap-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="withflatmap-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="withflatmap-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for WithFlatMap<Si, Item, U, St, F>`

##### `impl TryStream for WithFlatMap<Si, Item, U, St, F>`

- <span id="withflatmap-trystream-type-ok"></span>`type Ok = T`

- <span id="withflatmap-trystream-type-error"></span>`type Error = E`

- <span id="withflatmap-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for WithFlatMap<Si, Item, U, St, F>`

##### `impl<Si, Item, U, St, F> Unpin for WithFlatMap<Si, Item, U, St, F>`

