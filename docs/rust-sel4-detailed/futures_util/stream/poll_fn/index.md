*[futures_util](../../index.md) / [stream](../index.md) / [poll_fn](index.md)*

---

# Module `poll_fn`

Definition of the `PollFn` combinator

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PollFn`](#pollfn) | struct | Stream for the [`poll_fn`] function. |
| [`poll_fn`](#poll-fn) | fn | Creates a new stream wrapping a function returning `Poll<Option<T>>`. |

## Structs

### `PollFn<F>`

```rust
struct PollFn<F> {
    f: F,
}
```

Stream for the [`poll_fn`](#poll-fn) function.

#### Trait Implementations

##### `impl<F> Debug for PollFn<F>`

- <span id="pollfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> Stream for PollFn<F>`

- <span id="pollfn-stream-type-item"></span>`type Item = T`

- <span id="pollfn-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl StreamExt for PollFn<F>`

##### `impl TryStream for PollFn<F>`

- <span id="pollfn-trystream-type-ok"></span>`type Ok = T`

- <span id="pollfn-trystream-type-error"></span>`type Error = E`

- <span id="pollfn-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for PollFn<F>`

##### `impl<F> Unpin for PollFn<F>`

## Functions

### `poll_fn`

```rust
fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut futures_core::task::Context<'_>) -> futures_core::task::Poll<Option<T>>
```

Creates a new stream wrapping a function returning `Poll<Option<T>>`.

Polling the returned stream calls the wrapped function.

# Examples

```rust
use futures::stream::poll_fn;
use futures::task::Poll;

let mut counter = 1usize;

let read_stream = poll_fn(move |_| -> Poll<Option<String>> {
    if counter == 0 { return Poll::Ready(None); }
    counter -= 1;
    Poll::Ready(Some("Hello, World!".to_owned()))
});
```

