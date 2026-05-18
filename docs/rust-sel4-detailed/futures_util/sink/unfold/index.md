*[futures_util](../../index.md) / [sink](../index.md) / [unfold](index.md)*

---

# Module `unfold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Unfold`](#unfold) | struct | Sink for the [`unfold`] function. |
| [`unfold`](#unfold) | fn | Create a sink from a function which processes one item at a time. |

## Structs

### `Unfold<T, F, R>`

```rust
struct Unfold<T, F, R> {
    function: F,
    state: crate::unfold_state::UnfoldState<T, R>,
}
```

Sink for the [`unfold`](#unfold) function.

#### Trait Implementations

##### `impl<T: fmt::Debug, F: fmt::Debug, R: fmt::Debug> Debug for Unfold<T, F, R>`

- <span id="unfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F, R, Item> Sink for Unfold<T, F, R>`

- <span id="unfold-sink-type-error"></span>`type Error = E`

- <span id="unfold-sink-poll-ready"></span>`fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="unfold-sink-start-send"></span>`fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../index.md#sink)

- <span id="unfold-sink-poll-flush"></span>`fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

- <span id="unfold-sink-poll-close"></span>`fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Sink`](../index.md#sink)

##### `impl<T, Item> SinkExt for Unfold<T, F, R>`

##### `impl<T, F, R> Unpin for Unfold<T, F, R>`

## Functions

### `unfold`

```rust
fn unfold<T, F, R, Item, E>(init: T, function: F) -> Unfold<T, F, R>
where
    F: FnMut(T, Item) -> R,
    R: Future<Output = Result<T, E>>
```

Create a sink from a function which processes one item at a time.

# Examples

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::sink;
use futures::sink::SinkExt;

let unfold = sink::unfold(0, |mut sum, i: i32| {
    async move {
        sum += i;
        eprintln!("{}", i);
        Ok::<_, futures::never::Never>(sum)
    }
});
let mut unfold = pin!(unfold);
unfold.send(5).await?;
Ok::<(), futures::never::Never>(()) }).unwrap();
```

