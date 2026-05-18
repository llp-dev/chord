*[futures_util](../../index.md) / [sink](../index.md) / [map_err](index.md)*

---

# Module `map_err`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SinkMapErr`](#sinkmaperr) | struct | Sink for the [`sink_map_err`](super::SinkExt::sink_map_err) method. |

## Structs

### `SinkMapErr<Si, F>`

```rust
struct SinkMapErr<Si, F> {
    sink: Si,
    f: Option<F>,
}
```

Sink for the [`sink_map_err`](super::SinkExt::sink_map_err) method.

#### Implementations

- <span id="sinkmaperr-new"></span>`fn new(sink: Si, f: F) -> Self`

- <span id="sinkmaperr-get-ref"></span>`fn get_ref(&self) -> &Si`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="sinkmaperr-get-mut"></span>`fn get_mut(&mut self) -> &mut Si`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkmaperr-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Si>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="sinkmaperr-into-inner"></span>`fn into_inner(self) -> Si`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="sinkmaperr-take-f"></span>`fn take_f(self: Pin<&mut Self>) -> F`

#### Trait Implementations

##### `impl<Si: clone::Clone, F: clone::Clone> Clone for SinkMapErr<Si, F>`

- <span id="sinkmaperr-clone"></span>`fn clone(&self) -> SinkMapErr<Si, F>` — [`SinkMapErr`](#sinkmaperr)

##### `impl<Si: fmt::Debug, F: fmt::Debug> Debug for SinkMapErr<Si, F>`

- <span id="sinkmaperr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: FusedStream, F> FusedStream for SinkMapErr<S, F>`

- <span id="sinkmaperr-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Si, F, Item> Sink for SinkMapErr<Si, F>`

- <span id="sinkmaperr-sink-type-error"></span>`type Error = E`

- <span id="sinkmaperr-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="sinkmaperr-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="sinkmaperr-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="sinkmaperr-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<Item> SinkExt for SinkMapErr<Si, F>`

##### `impl<S: Stream, F> Stream for SinkMapErr<S, F>`

- <span id="sinkmaperr-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="sinkmaperr-stream-poll-next"></span>`fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

- <span id="sinkmaperr-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for SinkMapErr<Si, F>`

##### `impl TryStream for SinkMapErr<Si, F>`

- <span id="sinkmaperr-trystream-type-ok"></span>`type Ok = T`

- <span id="sinkmaperr-trystream-type-error"></span>`type Error = E`

- <span id="sinkmaperr-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../../stream/index.md#trystream)

##### `impl TryStreamExt for SinkMapErr<Si, F>`

##### `impl<Si, F> Unpin for SinkMapErr<Si, F>`

