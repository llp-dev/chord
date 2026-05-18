**futures_util > stream > unfold**

# Module: stream::unfold

## Contents

**Structs**

- [`Unfold`](#unfold) - Stream for the [`unfold`] function.

**Functions**

- [`unfold`](#unfold) - Creates a `Stream` from a seed and a closure returning a `Future`.

---

## futures_util::stream::unfold::Unfold

*Struct*

Stream for the [`unfold`] function.

**Generic Parameters:**
- T
- F
- Fut

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`



## futures_util::stream::unfold::unfold

*Function*

Creates a `Stream` from a seed and a closure returning a `Future`.

This function is the dual for the `Stream::fold()` adapter: while
`Stream::fold()` reduces a `Stream` to one single value, `unfold()` creates a
`Stream` from a seed value.

`unfold()` will call the provided closure with the provided seed, then wait
for the returned `Future` to complete with `(a, b)`. It will then yield the
value `a`, and use `b` as the next internal state.

If the closure returns `None` instead of `Some(Future)`, then the `unfold()`
will stop producing items and return `Poll::Ready(None)` in future
calls to `poll()`.

This function can typically be used when wanting to go from the "world of
futures" to the "world of streams": the provided closure can build a
`Future` using other library functions working on futures, and `unfold()`
will turn it into a `Stream` by repeating the operation.

# Example

```
# futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::unfold(0, |state| async move {
    if state <= 2 {
        let next_state = state + 1;
        let yielded = state * 2;
        Some((yielded, next_state))
    } else {
        None
    }
});

let result = stream.collect::<Vec<i32>>().await;
assert_eq!(result, vec![0, 2, 4]);
# });
```

```rust
fn unfold<T, F, Fut, Item>(init: T, f: F) -> Unfold<T, F, Fut>
```



