*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [chunks](index.md)*

---

# Module `chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Chunks`](#chunks) | struct | Stream for the [`chunks`](super::StreamExt::chunks) method. |

## Structs

### `Chunks<St: Stream>`

```rust
struct Chunks<St: Stream> {
    stream: crate::stream::Fuse<St>,
    items: alloc::vec::Vec<<St as >::Item>,
    cap: usize,
}
```

Stream for the [`chunks`](super::StreamExt::chunks) method.

#### Implementations

- <span id="chunks-new"></span>`fn new(stream: St, capacity: usize) -> Self`

- <span id="chunks-take"></span>`fn take(self: Pin<&mut Self>) -> Vec<<St as >::Item>` — [`Stream`](../../index.md#stream)

- <span id="chunks-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="chunks-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="chunks-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="chunks-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug + Stream> Debug for Chunks<St>`

- <span id="chunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: FusedStream> FusedStream for Chunks<St>`

- <span id="chunks-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Chunks<S>`

- <span id="chunks-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="chunks-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="chunks-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="chunks-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="chunks-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Chunks<St>`

##### `impl<St: Stream> Stream for Chunks<St>`

- <span id="chunks-stream-type-item"></span>`type Item = Vec<<St as Stream>::Item>`

- <span id="chunks-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="chunks-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Chunks<St>`

##### `impl<St: Stream> Unpin for Chunks<St>`

