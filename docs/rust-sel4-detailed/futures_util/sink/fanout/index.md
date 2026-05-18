*[futures_util](../../index.md) / [sink](../index.md) / [fanout](index.md)*

---

# Module `fanout`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Fanout`](#fanout) | struct | Sink that clones incoming items and forwards them to two sinks at the same time. |

## Structs

### `Fanout<Si1, Si2>`

```rust
struct Fanout<Si1, Si2> {
    sink1: Si1,
    sink2: Si2,
}
```

Sink that clones incoming items and forwards them to two sinks at the same time.

Backpressure from any downstream sink propagates up, which means that this sink
can only process items as fast as its _slowest_ downstream sink.

#### Implementations

- <span id="fanout-new"></span>`fn new(sink1: Si1, sink2: Si2) -> Self`

- <span id="fanout-get-ref"></span>`fn get_ref(&self) -> (&Si1, &Si2)`

  Get a shared reference to the inner sinks.

- <span id="fanout-get-mut"></span>`fn get_mut(&mut self) -> (&mut Si1, &mut Si2)`

  Get a mutable reference to the inner sinks.

- <span id="fanout-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut Si1>, Pin<&mut Si2>)`

  Get a pinned mutable reference to the inner sinks.

- <span id="fanout-into-inner"></span>`fn into_inner(self) -> (Si1, Si2)`

  Consumes this combinator, returning the underlying sinks.

  

  Note that this may discard intermediate state of this combinator,

  so care should be taken to avoid losing resources when this is called.

#### Trait Implementations

##### `impl<Si1: Debug, Si2: Debug> Debug for Fanout<Si1, Si2>`

- <span id="fanout-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult`

##### `impl<Si1, Si2, Item> Sink for Fanout<Si1, Si2>`

- <span id="fanout-sink-type-error"></span>`type Error = <Si1 as Sink>::Error`

- <span id="fanout-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="fanout-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="fanout-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="fanout-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<Item> SinkExt for Fanout<Si1, Si2>`

##### `impl<Si1, Si2> Unpin for Fanout<Si1, Si2>`

