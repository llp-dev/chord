**futures_util > future > poll_fn**

# Module: future::poll_fn

## Contents

**Structs**

- [`PollFn`](#pollfn) - Future for the [`poll_fn`] function.

**Functions**

- [`poll_fn`](#poll_fn) - Creates a new future wrapping around a function returning [`Poll`].

---

## futures_util::future::poll_fn::PollFn

*Struct*

Future for the [`poll_fn`] function.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<T>`



## futures_util::future::poll_fn::poll_fn

*Function*

Creates a new future wrapping around a function returning [`Poll`].

Polling the returned future delegates to the wrapped function.

# Examples

```
# futures::executor::block_on(async {
use futures::future::poll_fn;
use futures::task::{Context, Poll};

fn read_line(_cx: &mut Context<'_>) -> Poll<String> {
    Poll::Ready("Hello, World!".into())
}

let read_future = poll_fn(read_line);
assert_eq!(read_future.await, "Hello, World!".to_owned());
# });
```

```rust
fn poll_fn<T, F>(f: F) -> PollFn<F>
```



