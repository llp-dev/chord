*[futures_util](../../index.md) / [sink](../index.md) / [buffer](index.md)*

---

# Module `buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buffer`](#buffer) | struct | Sink for the [`buffer`](super::SinkExt::buffer) method. |

## Structs

### `Buffer<Si, Item>`

```rust
struct Buffer<Si, Item> {
    sink: Si,
    buf: alloc::collections::VecDeque<Item>,
    capacity: usize,
}
```

Sink for the [`buffer`](super::SinkExt::buffer) method.

#### Implementations

- <span id="buffer-new"></span>`fn new(sink: Si, capacity: usize) -> Self`

- <span id="buffer-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="buffer-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffer-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="buffer-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="buffer-try-empty-buffer"></span>`fn try_empty_buffer(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Si as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

#### Trait Implementations

##### `impl<Si: fmt::Debug, Item: fmt::Debug> Debug for Buffer<Si, Item>`

- <span id="buffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S, Item> FusedStream for Buffer<S, Item>`

- <span id="buffer-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si: Sink<Item>, Item> Sink for Buffer<Si, Item>`

- <span id="buffer-sink-type-error"></span>`type Error = <Si as Sink>::Error`

- <span id="buffer-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="buffer-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="buffer-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="buffer-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<Item> SinkExt for Buffer<Si, Item>`

##### `impl<S, Item> Stream for Buffer<S, Item>`

- <span id="buffer-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="buffer-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<S as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="buffer-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Buffer<Si, Item>`

##### `impl TryStream for Buffer<Si, Item>`

- <span id="buffer-trystream-type-ok"></span>`type Ok = T`

- <span id="buffer-trystream-type-error"></span>`type Error = E`

- <span id="buffer-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for Buffer<Si, Item>`

##### `impl<Si, Item> Unpin for Buffer<Si, Item>`

