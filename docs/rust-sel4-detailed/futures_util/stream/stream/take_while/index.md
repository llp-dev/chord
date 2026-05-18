*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [take_while](index.md)*

---

# Module `take_while`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TakeWhile`](#takewhile) | struct | Stream for the [`take_while`](super::StreamExt::take_while) method. |

## Structs

### `TakeWhile<St: Stream, Fut, F>`

```rust
struct TakeWhile<St: Stream, Fut, F> {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Item>,
    done_taking: bool,
}
```

Stream for the [`take_while`](super::StreamExt::take_while) method.

#### Implementations

- <span id="takewhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="takewhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="takewhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takewhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takewhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TakeWhile<St, Fut, F>`

- <span id="takewhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for TakeWhile<St, Fut, F>`

- <span id="takewhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for TakeWhile<S, Fut, F>`

- <span id="takewhile-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="takewhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="takewhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="takewhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="takewhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TakeWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for TakeWhile<St, Fut, F>`

- <span id="takewhile-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="takewhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="takewhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TakeWhile<St, Fut, F>`

##### `impl TryStream for TakeWhile<St, Fut, F>`

- <span id="takewhile-trystream-type-ok"></span>`type Ok = T`

- <span id="takewhile-trystream-type-error"></span>`type Error = E`

- <span id="takewhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for TakeWhile<St, Fut, F>`

##### `impl<St: Stream, Fut, F> Unpin for TakeWhile<St, Fut, F>`

