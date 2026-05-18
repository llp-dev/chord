**futures_util > sink > drain**

# Module: sink::drain

## Contents

**Structs**

- [`Drain`](#drain) - Sink for the [`drain`] function.

**Functions**

- [`drain`](#drain) - Create a sink that will just discard all items given to it.

---

## futures_util::sink::drain::Drain

*Struct*

Sink for the [`drain`] function.

**Generic Parameters:**
- T

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, _cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, _item: T) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, _cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, _cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`



## futures_util::sink::drain::drain

*Function*

Create a sink that will just discard all items given to it.

Similar to [`io::Sink`](::std::io::Sink).

# Examples

```
# futures::executor::block_on(async {
use futures::sink::{self, SinkExt};

let mut drain = sink::drain();
drain.send(5).await?;
# Ok::<(), futures::never::Never>(()) }).unwrap();
```

```rust
fn drain<T>() -> Drain<T>
```



