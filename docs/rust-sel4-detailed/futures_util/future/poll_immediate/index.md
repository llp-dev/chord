*[futures_util](../../index.md) / [future](../index.md) / [poll_immediate](index.md)*

---

# Module `poll_immediate`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PollImmediate`](#pollimmediate) | struct | Future for the [`poll_immediate`](poll_immediate()) function. |
| [`poll_immediate`](#poll-immediate) | fn | Creates a future that is immediately ready with an Option of a value. |

## Structs

### `PollImmediate<T>`

```rust
struct PollImmediate<T> {
    future: Option<T>,
}
```

Future for the [`poll_immediate`](poll_immediate()) function.

It will never return [Poll::Pending](core::task::Poll::Pending)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for PollImmediate<T>`

- <span id="pollimmediate-clone"></span>`fn clone(&self) -> PollImmediate<T>` — [`PollImmediate`](#pollimmediate)

##### `impl<T: fmt::Debug> Debug for PollImmediate<T>`

- <span id="pollimmediate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Future> FusedFuture for PollImmediate<T>`

- <span id="pollimmediate-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F> Future for PollImmediate<F>`

- <span id="pollimmediate-future-type-output"></span>`type Output = Option<T>`

- <span id="pollimmediate-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl<T> FutureExt for PollImmediate<T>`

##### `impl IntoFuture for PollImmediate<T>`

- <span id="pollimmediate-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="pollimmediate-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="pollimmediate-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> Stream for PollImmediate<F>`

- <span id="pollimmediate-stream-type-item"></span>`type Item = Poll<T>`

- <span id="pollimmediate-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../../stream/index.md#stream)

##### `impl<T> StreamExt for PollImmediate<T>`

##### `impl<T> Unpin for PollImmediate<T>`

## Functions

### `poll_immediate`

```rust
fn poll_immediate<F: Future>(f: F) -> PollImmediate<F>
```

Creates a future that is immediately ready with an Option of a value.
Specifically this means that [poll](core::future::Future::poll()) always returns [Poll::Ready](core::task::Poll::Ready).

# Caution

When consuming the future by this function, note the following:

- This function does not guarantee that the future will run to completion, so it is generally incompatible with passing the non-cancellation-safe future by value.
- Even if the future is cancellation-safe, creating and dropping new futures frequently may lead to performance problems.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let r = future::poll_immediate(async { 1_u32 });
assert_eq!(r.await, Some(1));

let p = future::poll_immediate(future::pending::<i32>());
assert_eq!(p.await, None);
});
```

### Reusing a future

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::future;

let f = async {futures::pending!(); 42_u8};
let mut f = pin!(f);
assert_eq!(None, future::poll_immediate(&mut f).await);
assert_eq!(42, f.await);
});
```

