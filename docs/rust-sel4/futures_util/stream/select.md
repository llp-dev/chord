**futures_util > stream > select**

# Module: stream::select

## Contents

**Structs**

- [`Select`](#select) - Stream for the [`select()`] function.

**Functions**

- [`select`](#select) - This function will attempt to pull items from both streams. Each

---

## futures_util::stream::select::Select

*Struct*

Stream for the [`select()`] function.

**Generic Parameters:**
- St1
- St2

**Methods:**

- `fn get_ref(self: &Self) -> (&St1, &St2)` - Acquires a reference to the underlying streams that this combinator is
- `fn get_mut(self: & mut Self) -> (& mut St1, & mut St2)` - Acquires a mutable reference to the underlying streams that this
- `fn get_pin_mut(self: Pin<& mut Self>) -> (Pin<& mut St1>, Pin<& mut St2>)` - Acquires a pinned mutable reference to the underlying streams that this
- `fn into_inner(self: Self) -> (St1, St2)` - Consumes this combinator, returning the underlying streams.

**Trait Implementations:**

- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<St1 as >::Item>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::select::select

*Function*

This function will attempt to pull items from both streams. Each
stream will be polled in a round-robin fashion, and whenever a stream is
ready to yield an item that item is yielded.

After one of the two input streams completes, the remaining one will be
polled exclusively. The returned stream completes when both input
streams have completed.

Note that this function consumes both streams and returns a wrapped
version of them.

## Examples

```rust
# futures::executor::block_on(async {
use futures::stream::{ repeat, select, StreamExt };

let left = repeat(1);
let right = repeat(2);

let mut out = select(left, right);

for _ in 0..100 {
    // We should be alternating.
    assert_eq!(1, out.select_next_some().await);
    assert_eq!(2, out.select_next_some().await);
}
# });
```

```rust
fn select<St1, St2>(stream1: St1, stream2: St2) -> Select<St1, St2>
```



