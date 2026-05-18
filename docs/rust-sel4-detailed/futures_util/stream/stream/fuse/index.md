*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [fuse](index.md)*

---

# Module `fuse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Fuse`](#fuse) | struct | Stream for the [`fuse`](super::StreamExt::fuse) method. |

## Structs

### `Fuse<St>`

```rust
struct Fuse<St> {
    stream: St,
    done: bool,
}
```

Stream for the [`fuse`](super::StreamExt::fuse) method.

#### Implementations

- <span id="fuse-new"></span>`fn new(stream: St) -> Self`

- <span id="fuse-is-done"></span>`fn is_done(&self) -> bool`

  Returns whether the underlying stream has finished or not.

  

  If this method returns `true`, then all future calls to poll are

  guaranteed to return `None`. If this returns `false`, then the

  underlying stream is still in use.

- <span id="fuse-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="fuse-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="fuse-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="fuse-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for Fuse<St>`

- <span id="fuse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Stream> FusedStream for Fuse<S>`

- <span id="fuse-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S: Stream + Sink<Item>, Item> Sink for Fuse<S>`

- <span id="fuse-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="fuse-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="fuse-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="fuse-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="fuse-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Fuse<St>`

##### `impl<S: Stream> Stream for Fuse<S>`

- <span id="fuse-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="fuse-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="fuse-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Fuse<St>`

##### `impl TryStream for Fuse<St>`

- <span id="fuse-trystream-type-ok"></span>`type Ok = T`

- <span id="fuse-trystream-type-error"></span>`type Error = E`

- <span id="fuse-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Fuse<St>`

##### `impl<St> Unpin for Fuse<St>`

