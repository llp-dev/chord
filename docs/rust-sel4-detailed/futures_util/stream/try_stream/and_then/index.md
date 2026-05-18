*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [and_then](index.md)*

---

# Module `and_then`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AndThen`](#andthen) | struct | Stream for the [`and_then`](super::TryStreamExt::and_then) method. |

## Structs

### `AndThen<St, Fut, F>`

```rust
struct AndThen<St, Fut, F> {
    stream: St,
    future: Option<Fut>,
    f: F,
}
```

Stream for the [`and_then`](super::TryStreamExt::and_then) method.

#### Implementations

- <span id="andthen-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="andthen-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="andthen-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="andthen-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="andthen-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for AndThen<St, Fut, F>`

- <span id="andthen-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for AndThen<St, Fut, F>`

- <span id="andthen-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for AndThen<S, Fut, F>`

- <span id="andthen-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="andthen-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="andthen-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="andthen-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="andthen-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for AndThen<St, Fut, F>`

##### `impl<St, Fut, F> Stream for AndThen<St, Fut, F>`

- <span id="andthen-stream-type-item"></span>`type Item = Result<<Fut as TryFuture>::Ok, <St as TryStream>::Error>`

- <span id="andthen-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="andthen-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for AndThen<St, Fut, F>`

##### `impl TryStream for AndThen<St, Fut, F>`

- <span id="andthen-trystream-type-ok"></span>`type Ok = T`

- <span id="andthen-trystream-type-error"></span>`type Error = E`

- <span id="andthen-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for AndThen<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for AndThen<St, Fut, F>`

