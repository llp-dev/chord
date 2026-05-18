**futures_util > sink > with_flat_map**

# Module: sink::with_flat_map

## Contents

**Structs**

- [`WithFlatMap`](#withflatmap) - Sink for the [`with_flat_map`](super::SinkExt::with_flat_map) method.

---

## futures_util::sink::with_flat_map::WithFlatMap

*Struct*

Sink for the [`with_flat_map`](super::SinkExt::with_flat_map) method.

**Generic Parameters:**
- Si
- Item
- U
- St
- F

**Methods:**

- `fn get_ref(self: &Self) -> &Si` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut Si` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut Si>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> Si` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Sink**
  - `fn poll_ready(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: Pin<& mut Self>, item: U) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<(), <Self as >::Error>>`



