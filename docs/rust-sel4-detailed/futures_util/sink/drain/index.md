*[futures_util](../../index.md) / [sink](../index.md) / [drain](index.md)*

---

# Module `drain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | Sink for the [`drain`] function. |
| [`drain`](#drain) | fn | Create a sink that will just discard all items given to it. |

## Structs

### `Drain<T>`

```rust
struct Drain<T> {
    marker: core::marker::PhantomData<T>,
}
```

Sink for the [`drain`](#drain) function.

#### Trait Implementations

##### `impl<T> Clone for Drain<T>`

- <span id="drain-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Drain<T>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Sink for Drain<T>`

- <span id="drain-sink-type-error"></span>`type Error = Infallible`

- <span id="drain-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="drain-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, _item: T) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="drain-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="drain-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<T, Item> SinkExt for Drain<T>`

##### `impl<T> Unpin for Drain<T>`

## Functions

### `drain`

```rust
fn drain<T>() -> Drain<T>
```

Create a sink that will just discard all items given to it.

Similar to [`io::Sink`](::std::io::Sink).

# Examples

```rust
futures::executor::block_on(async {
use futures::sink::{self, SinkExt};

let mut drain = sink::drain();
drain.send(5).await?;
Ok::<(), futures::never::Never>(()) }).unwrap();
```

