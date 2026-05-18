**futures_util > sink > buffer**

# Module: sink::buffer

## Contents

**Structs**

- [`Buffer`](#buffer) - Sink for the [`buffer`](super::SinkExt::buffer) method.

---

## futures_util::sink::buffer::Buffer

*Struct*

Sink for the [`buffer`](super::SinkExt::buffer) method.

**Generic Parameters:**
- Si
- Item

**Methods:**

- `fn get_ref(self: &Self) -> &Si` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut Si` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut Si>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> Si` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<S as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`



