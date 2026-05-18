*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [skip_while](index.md)*

---

# Module `skip_while`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SkipWhile`](#skipwhile) | struct | Stream for the [`skip_while`](super::StreamExt::skip_while) method. |

## Structs

### `SkipWhile<St, Fut, F>`

```rust
struct SkipWhile<St, Fut, F>
where
    St: Stream {
    stream: St,
    f: F,
    pending_fut: Option<Fut>,
    pending_item: Option<<St as >::Item>,
    done_skipping: bool,
}
```

Stream for the [`skip_while`](super::StreamExt::skip_while) method.

#### Implementations

- <span id="skipwhile-new"></span>`fn new(stream: St, f: F) -> Self`

- <span id="skipwhile-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="skipwhile-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skipwhile-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="skipwhile-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St, Fut, F> Debug for SkipWhile<St, Fut, F>`

- <span id="skipwhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedStream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, F, Item> Sink for SkipWhile<S, Fut, F>`

- <span id="skipwhile-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="skipwhile-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="skipwhile-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="skipwhile-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="skipwhile-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for SkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Stream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="skipwhile-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="skipwhile-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for SkipWhile<St, Fut, F>`

##### `impl TryStream for SkipWhile<St, Fut, F>`

- <span id="skipwhile-trystream-type-ok"></span>`type Ok = T`

- <span id="skipwhile-trystream-type-error"></span>`type Error = E`

- <span id="skipwhile-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for SkipWhile<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for SkipWhile<St, Fut, F>`

