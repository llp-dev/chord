**futures_util > stream > poll_fn**

# Module: stream::poll_fn

## Contents

**Structs**

- [`PollFn`](#pollfn) - Stream for the [`poll_fn`] function.

**Functions**

- [`poll_fn`](#poll_fn) - Creates a new stream wrapping a function returning `Poll<Option<T>>`.

---

## futures_util::stream::poll_fn::PollFn

*Struct*

Stream for the [`poll_fn`] function.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<T>>`



## futures_util::stream::poll_fn::poll_fn

*Function*

Creates a new stream wrapping a function returning `Poll<Option<T>>`.

Polling the returned stream calls the wrapped function.

# Examples

```
use futures::stream::poll_fn;
use futures::task::Poll;

let mut counter = 1usize;

let read_stream = poll_fn(move |_| -> Poll<Option<String>> {
    if counter == 0 { return Poll::Ready(None); }
    counter -= 1;
    Poll::Ready(Some("Hello, World!".to_owned()))
});
```

```rust
fn poll_fn<T, F>(f: F) -> PollFn<F>
```



