*[futures_util](../../index.md) / [future](../index.md) / [poll_fn](index.md)*

---

# Module `poll_fn`

Definition of the `PollFn` adapter combinator

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PollFn`](#pollfn) | struct | Future for the [`poll_fn`] function. |
| [`poll_fn`](#poll-fn) | fn | Creates a new future wrapping around a function returning [`Poll`]. |

## Structs

### `PollFn<F>`

```rust
struct PollFn<F> {
    f: F,
}
```

Future for the [`poll_fn`](#poll-fn) function.

#### Trait Implementations

##### `impl<F> Debug for PollFn<F>`

- <span id="pollfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> Future for PollFn<F>`

- <span id="pollfn-future-type-output"></span>`type Output = T`

- <span id="pollfn-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl FutureExt for PollFn<F>`

##### `impl<F> IntoFuture for PollFn<F>`

- <span id="pollfn-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="pollfn-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="pollfn-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for PollFn<F>`

- <span id="pollfn-tryfuture-type-ok"></span>`type Ok = T`

- <span id="pollfn-tryfuture-type-error"></span>`type Error = E`

- <span id="pollfn-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for PollFn<F>`

##### `impl<F> Unpin for PollFn<F>`

## Functions

### `poll_fn`

```rust
fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut futures_core::task::Context<'_>) -> futures_core::task::Poll<T>
```

Creates a new future wrapping around a function returning [`Poll`](../../task/index.md).

Polling the returned future delegates to the wrapped function.

# Examples

```rust
futures::executor::block_on(async {
use futures::future::poll_fn;
use futures::task::{Context, Poll};

fn read_line(_cx: &mut Context<'_>) -> Poll<String> {
    Poll::Ready("Hello, World!".into())
}

let read_future = poll_fn(read_line);
assert_eq!(read_future.await, "Hello, World!".to_owned());
});
```

