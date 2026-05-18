*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [into_stream](index.md)*

---

# Module `into_stream`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoStream`](#intostream) | struct | Stream for the [`into_stream`](super::TryStreamExt::into_stream) method. |

## Structs

### `IntoStream<St>`

```rust
struct IntoStream<St> {
    stream: St,
}
```

Stream for the [`into_stream`](super::TryStreamExt::into_stream) method.

#### Implementations

- <span id="intostream-new"></span>`fn new(stream: St) -> Self`

- <span id="intostream-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="intostream-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="intostream-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="intostream-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for IntoStream<St>`

- <span id="intostream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: TryStream + FusedStream> FusedStream for IntoStream<St>`

- <span id="intostream-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S: Sink<Item>, Item> Sink for IntoStream<S>`

- <span id="intostream-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="intostream-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="intostream-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="intostream-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="intostream-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for IntoStream<St>`

##### `impl<St: TryStream> Stream for IntoStream<St>`

- <span id="intostream-stream-type-item"></span>`type Item = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="intostream-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="intostream-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for IntoStream<St>`

##### `impl TryStream for IntoStream<St>`

- <span id="intostream-trystream-type-ok"></span>`type Ok = T`

- <span id="intostream-trystream-type-error"></span>`type Error = E`

- <span id="intostream-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for IntoStream<St>`

##### `impl<St> Unpin for IntoStream<St>`

