**futures_util > stream > repeat**

# Module: stream::repeat

## Contents

**Structs**

- [`Repeat`](#repeat) - Stream for the [`repeat`] function.

**Functions**

- [`repeat`](#repeat) - Create a stream which produces the same item repeatedly.

---

## futures_util::stream::repeat::Repeat

*Struct*

Stream for the [`repeat`] function.

**Generic Parameters:**
- T

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, _: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Repeat<T>`



## futures_util::stream::repeat::repeat

*Function*

Create a stream which produces the same item repeatedly.

The stream never terminates. Note that you likely want to avoid
usage of `collect` or such on the returned stream as it will exhaust
available memory as it tries to just fill up all RAM.

```
# futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::repeat(9);
assert_eq!(vec![9, 9, 9], stream.take(3).collect::<Vec<i32>>().await);
# });
```

```rust
fn repeat<T>(item: T) -> Repeat<T>
```



