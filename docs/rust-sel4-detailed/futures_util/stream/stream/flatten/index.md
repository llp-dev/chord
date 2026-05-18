*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [flatten](index.md)*

---

# Module `flatten`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flatten`](#flatten) | struct | Stream for the [`flatten`](super::StreamExt::flatten) method. |

## Structs

### `Flatten<St, U>`

```rust
struct Flatten<St, U> {
    stream: St,
    next: Option<U>,
}
```

Stream for the [`flatten`](super::StreamExt::flatten) method.

#### Implementations

- <span id="flatten-new"></span>`fn new(stream: St) -> Self`

- <span id="flatten-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="flatten-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="flatten-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="flatten-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<St: fmt::Debug, U: fmt::Debug> Debug for Flatten<St, U>`

- <span id="flatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> FusedStream for Flatten<St, <St as >::Item>`

- <span id="flatten-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Flatten<S, <S as >::Item>`

- <span id="flatten-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="flatten-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="flatten-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="flatten-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="flatten-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Flatten<St, U>`

##### `impl<St> Stream for Flatten<St, <St as >::Item>`

- <span id="flatten-stream-type-item"></span>`type Item = <<St as Stream>::Item as Stream>::Item`

- <span id="flatten-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

##### `impl StreamExt for Flatten<St, U>`

##### `impl TryStream for Flatten<St, U>`

- <span id="flatten-trystream-type-ok"></span>`type Ok = T`

- <span id="flatten-trystream-type-error"></span>`type Error = E`

- <span id="flatten-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Flatten<St, U>`

##### `impl<St, U> Unpin for Flatten<St, U>`

