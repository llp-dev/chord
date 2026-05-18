**futures_util > stream > iter**

# Module: stream::iter

## Contents

**Structs**

- [`Iter`](#iter) - Stream for the [`iter`] function.

**Functions**

- [`iter`](#iter) - Converts an `Iterator` into a `Stream` which is always ready

---

## futures_util::stream::iter::Iter

*Struct*

Stream for the [`iter`] function.

**Generic Parameters:**
- I

**Methods:**

- `fn get_ref(self: &Self) -> &I` - Acquires a reference to the underlying iterator that this stream is pulling from.
- `fn get_mut(self: & mut Self) -> & mut I` - Acquires a mutable reference to the underlying iterator that this stream is pulling from.
- `fn into_inner(self: Self) -> I` - Consumes this stream, returning the underlying iterator.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Iter<I>`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, _: & mut Context) -> Poll<Option<<I as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::stream::iter::iter

*Function*

Converts an `Iterator` into a `Stream` which is always ready
to yield the next value.

Iterators in Rust don't express the ability to block, so this adapter
simply always calls `iter.next()` and returns that.

```
# futures::executor::block_on(async {
use futures::stream::{self, StreamExt};

let stream = stream::iter(vec![17, 19]);
assert_eq!(vec![17, 19], stream.collect::<Vec<i32>>().await);
# });
```

```rust
fn iter<I>(i: I) -> Iter<<I as >::IntoIter>
```



