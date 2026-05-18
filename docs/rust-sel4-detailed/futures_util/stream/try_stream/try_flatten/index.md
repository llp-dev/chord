*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_flatten](index.md)*

---

# Module `try_flatten`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFlatten`](#tryflatten) | struct | Stream for the [`try_flatten`](super::TryStreamExt::try_flatten) method. |

## Structs

### `TryFlatten<St>`

```rust
struct TryFlatten<St>
where
    St: TryStream {
    stream: St,
    next: Option<<St as >::Ok>,
}
```

Stream for the [`try_flatten`](super::TryStreamExt::try_flatten) method.

#### Implementations

- <span id="tryflatten-new"></span>`fn new(stream: St) -> Self`

- <span id="tryflatten-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="tryflatten-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryflatten-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="tryflatten-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St> Debug for TryFlatten<St>`

- <span id="tryflatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for TryFlatten<St>`

- <span id="tryflatten-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for TryFlatten<S>`

- <span id="tryflatten-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="tryflatten-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryflatten-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="tryflatten-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="tryflatten-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TryFlatten<St>`

##### `impl<St> Stream for TryFlatten<St>`

- <span id="tryflatten-stream-type-item"></span>`type Item = Result<<<St as TryStream>::Ok as TryStream>::Ok, <<St as TryStream>::Ok as TryStream>::Error>`

- <span id="tryflatten-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

##### `impl StreamExt for TryFlatten<St>`

##### `impl TryStream for TryFlatten<St>`

- <span id="tryflatten-trystream-type-ok"></span>`type Ok = T`

- <span id="tryflatten-trystream-type-error"></span>`type Error = E`

- <span id="tryflatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for TryFlatten<St>`

##### `impl<St> Unpin for TryFlatten<St>`

