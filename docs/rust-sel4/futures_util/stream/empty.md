**futures_util > stream > empty**

# Module: stream::empty

## Contents

**Structs**

- [`Empty`](#empty) - Stream for the [`empty`] function.

**Functions**

- [`empty`](#empty) - Creates a stream which contains no elements.

---

## futures_util::stream::empty::Empty

*Struct*

Stream for the [`empty`] function.

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
  - `fn clone(self: &Self) -> Self`



## futures_util::stream::empty::empty

*Function*

Creates a stream which contains no elements.

The returned stream will always return `Ready(None)` when polled.

```rust
fn empty<T>() -> Empty<T>
```



