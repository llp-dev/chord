**futures_util > sink > unfold**

# Module: sink::unfold

## Contents

**Structs**

- [`Unfold`](#unfold) - Sink for the [`unfold`] function.

**Functions**

- [`unfold`](#unfold) - Create a sink from a function which processes one item at a time.

---

## futures_util::sink::unfold::Unfold

*Struct*

Sink for the [`unfold`] function.

**Generic Parameters:**
- T
- F
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`



## futures_util::sink::unfold::unfold

*Function*

Create a sink from a function which processes one item at a time.

# Examples

```
# futures::executor::block_on(async {
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
# Ok::<(), futures::never::Never>(()) }).unwrap();
```

```rust
fn unfold<T, F, R, Item, E>(init: T, function: F) -> Unfold<T, F, R>
```



