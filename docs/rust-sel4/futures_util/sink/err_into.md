**futures_util > sink > err_into**

# Module: sink::err_into

## Contents

**Structs**

- [`SinkErrInto`](#sinkerrinto) - Sink for the [`sink_err_into`](super::SinkExt::sink_err_into) method.

---

## futures_util::sink::err_into::SinkErrInto

*Struct*

Sink for the [`sink_err_into`](super::SinkExt::sink_err_into) method.

**Generic Parameters:**
- Si
- Item
- E

**Methods:**

- `fn get_ref(self: &Self) -> &Si` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut Si` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut Si>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> Si` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



