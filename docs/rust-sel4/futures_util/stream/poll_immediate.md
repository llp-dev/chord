**futures_util > stream > poll_immediate**

# Module: stream::poll_immediate

## Contents

**Structs**

- [`PollImmediate`](#pollimmediate) - Stream for the [poll_immediate](poll_immediate()) function.

**Functions**

- [`poll_immediate`](#poll_immediate) - Creates a new stream that always immediately returns [Poll::Ready](core::task::Poll::Ready) when awaiting it.

---

## futures_util::stream::poll_immediate::PollImmediate

*Struct*

Stream for the [poll_immediate](poll_immediate()) function.

It will never return [Poll::Pending](core::task::Poll::Pending)

**Generic Parameters:**
- S

**Trait Implementations:**

- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PollImmediate<S>`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## futures_util::stream::poll_immediate::poll_immediate

*Function*

Creates a new stream that always immediately returns [Poll::Ready](core::task::Poll::Ready) when awaiting it.

This is useful when immediacy is more important than waiting for the next item to be ready.

# Examples

```
# futures::executor::block_on(async {
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
# });
```

```rust
fn poll_immediate<S>(s: S) -> PollImmediate<S>
```



