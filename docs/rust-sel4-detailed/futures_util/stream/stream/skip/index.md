*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [skip](index.md)*

---

# Module `skip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Skip`](#skip) | struct | Stream for the [`skip`](super::StreamExt::skip) method. |

## Structs

### `Skip<St>`

```rust
struct Skip<St> {
    stream: St,
    remaining: usize,
}
```

Stream for the [`skip`](super::StreamExt::skip) method.

#### Implementations

- <span id="skip-new"></span>`fn new(stream: St, n: usize) -> Self`

- <span id="skip-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="skip-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skip-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skip-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Skip<St>`

- <span id="skip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: FusedStream> FusedStream for Skip<St>`

- <span id="skip-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Skip<S>`

- <span id="skip-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="skip-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="skip-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="skip-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="skip-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Skip<St>`

##### `impl<St: Stream> Stream for Skip<St>`

- <span id="skip-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="skip-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="skip-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Skip<St>`

##### `impl TryStream for Skip<St>`

- <span id="skip-trystream-type-ok"></span>`type Ok = T`

- <span id="skip-trystream-type-error"></span>`type Error = E`

- <span id="skip-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Skip<St>`

##### `impl<St> Unpin for Skip<St>`

