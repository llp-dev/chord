*[futures_util](../../index.md) / [stream](../index.md) / [poll_immediate](index.md)*

---

# Module `poll_immediate`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PollImmediate`](#pollimmediate) | struct | Stream for the [poll_immediate](poll_immediate()) function. |
| [`poll_immediate`](#poll-immediate) | fn | Creates a new stream that always immediately returns [Poll::Ready](core::task::Poll::Ready) when awaiting it. |

## Structs

### `PollImmediate<S>`

```rust
struct PollImmediate<S> {
    stream: Option<S>,
}
```

Stream for the [poll_immediate](poll_immediate()) function.

It will never return [Poll::Pending](core::task::Poll::Pending)

#### Trait Implementations

##### `impl<S: clone::Clone> Clone for PollImmediate<S>`

- <span id="pollimmediate-clone"></span>`fn clone(&self) -> PollImmediate<S>` — [`PollImmediate`](#pollimmediate)

##### `impl<S: fmt::Debug> Debug for PollImmediate<S>`

- <span id="pollimmediate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S: Stream> FusedStream for PollImmediate<S>`

- <span id="pollimmediate-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S> Stream for PollImmediate<S>`

- <span id="pollimmediate-stream-type-item"></span>`type Item = Poll<T>`

- <span id="pollimmediate-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="pollimmediate-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for PollImmediate<S>`

##### `impl<S> Unpin for PollImmediate<S>`

## Functions

### `poll_immediate`

```rust
fn poll_immediate<S: Stream>(s: S) -> PollImmediate<S>
```

Creates a new stream that always immediately returns [Poll::Ready](core::task::Poll::Ready) when awaiting it.

This is useful when immediacy is more important than waiting for the next item to be ready.

# Examples

```rust
futures::executor::block_on(async {
use futures::stream::{self, StreamExt};
use futures::task::Poll;

let mut r = stream::poll_immediate(Box::pin(stream::iter(1_u32..3)));
assert_eq!(r.next().await, Some(Poll::Ready(1)));
assert_eq!(r.next().await, Some(Poll::Ready(2)));
assert_eq!(r.next().await, None);

let mut p = stream::poll_immediate(Box::pin(stream::once(async {
    futures::pending!();
    42_u8
})));
assert_eq!(p.next().await, Some(Poll::Pending));
assert_eq!(p.next().await, Some(Poll::Ready(42)));
assert_eq!(p.next().await, None);
});
```

