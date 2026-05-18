**futures_util > stream > stream > zip**

# Module: stream::stream::zip

## Contents

**Structs**

- [`Zip`](#zip) - Stream for the [`zip`](super::StreamExt::zip) method.

---

## futures_util::stream::stream::zip::Zip

*Struct*

Stream for the [`zip`](super::StreamExt::zip) method.

**Generic Parameters:**
- St1
- St2

**Methods:**

- `fn get_ref(self: &Self) -> (&St1, &St2)` - Acquires a reference to the underlying streams that this combinator is
- `fn get_mut(self: & mut Self) -> (& mut St1, & mut St2)` - Acquires a mutable reference to the underlying streams that this
- `fn get_pin_mut(self: Pin<& mut Self>) -> (Pin<& mut St1>, Pin<& mut St2>)` - Acquires a pinned mutable reference to the underlying streams that this
- `fn into_inner(self: Self) -> (St1, St2)` - Consumes this combinator, returning the underlying streams.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



