**futures_util > stream > pending**

# Module: stream::pending

## Contents

**Structs**

- [`Pending`](#pending) - Stream for the [`pending()`] function.

**Functions**

- [`pending`](#pending) - Creates a stream which never returns any elements.

---

## futures_util::stream::pending::Pending

*Struct*

Stream for the [`pending()`] function.

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



## futures_util::stream::pending::pending

*Function*

Creates a stream which never returns any elements.

The returned stream will always return `Pending` when polled.

```rust
fn pending<T>() -> Pending<T>
```



