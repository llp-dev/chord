**futures_util > stream > stream > buffered**

# Module: stream::stream::buffered

## Contents

**Structs**

- [`Buffered`](#buffered) - Stream for the [`buffered`](super::StreamExt::buffered) method.

---

## futures_util::stream::stream::buffered::Buffered

*Struct*

Stream for the [`buffered`](super::StreamExt::buffered) method.

**Generic Parameters:**
- St

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`



