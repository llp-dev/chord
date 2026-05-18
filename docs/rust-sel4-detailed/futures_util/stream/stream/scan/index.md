*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [scan](index.md)*

---

# Module `scan`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StateFn`](#statefn) | struct |  |
| [`Scan`](#scan) | struct | Stream for the [`scan`](super::StreamExt::scan) method. |

## Structs

### `StateFn<S, F>`

```rust
struct StateFn<S, F> {
    state: S,
    f: F,
}
```

### `Scan<St: Stream, S, Fut, F>`

```rust
struct Scan<St: Stream, S, Fut, F> {
    stream: St,
    state_f: Option<StateFn<S, F>>,
    future: Option<Fut>,
}
```

Stream for the [`scan`](super::StreamExt::scan) method.

#### Implementations

- <span id="scan-is-done-taking"></span>`fn is_done_taking(&self) -> bool`

  Checks if internal state is `None`.

#### Trait Implementations

##### `impl<St, S, Fut, F> Debug for Scan<St, S, Fut, F>`

- <span id="scan-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, S, Fut, F> FusedStream for Scan<St, S, Fut, F>`

- <span id="scan-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, S, Fut, F, Item> Sink for Scan<St, S, Fut, F>`

- <span id="scan-sink-type-error"></span>`type Error = <St as Sink>::Error`

- <span id="scan-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="scan-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="scan-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="scan-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Scan<St, S, Fut, F>`

##### `impl<St, S, Fut, F> Stream for Scan<St, S, Fut, F>`

- <span id="scan-stream-type-item"></span>`type Item = B`

- <span id="scan-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<B>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

- <span id="scan-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Scan<St, S, Fut, F>`

##### `impl<S> TryStream for Scan<St, S, Fut, F>`

- <span id="scan-trystream-type-ok"></span>`type Ok = T`

- <span id="scan-trystream-type-error"></span>`type Error = E`

- <span id="scan-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl<S> TryStreamExt for Scan<St, S, Fut, F>`

##### `impl<St: Stream, S, Fut, F> Unpin for Scan<St, S, Fut, F>`

