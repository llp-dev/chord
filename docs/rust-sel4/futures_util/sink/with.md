**futures_util > sink > with**

# Module: sink::with

## Contents

**Structs**

- [`With`](#with) - Sink for the [`with`](super::SinkExt::with) method.

---

## futures_util::sink::with::With

*Struct*

Sink for the [`with`](super::SinkExt::with) method.

**Generic Parameters:**
- Si
- Item
- U
- Fut
- F

**Methods:**

- `fn get_ref(self: &Self) -> &Si` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut Si` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut Si>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> Si` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, item: U) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



