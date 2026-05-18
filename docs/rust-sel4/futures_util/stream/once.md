**futures_util > stream > once**

# Module: stream::once

## Contents

**Structs**

- [`Once`](#once) - A stream which emits single element and then EOF.

**Functions**

- [`once`](#once) - Creates a stream of a single element.

---

## futures_util::stream::once::Once

*Struct*

A stream which emits single element and then EOF.

**Generic Parameters:**
- Fut

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::stream::once::once

*Function*

Creates a stream of a single element.

```
# futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::once(async { 17 });
let collected = stream.collect::<Vec<i32>>().await;
assert_eq!(collected, vec![17]);
# });
```

```rust
fn once<Fut>(future: Fut) -> Once<Fut>
```



