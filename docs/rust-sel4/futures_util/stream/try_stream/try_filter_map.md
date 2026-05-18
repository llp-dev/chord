**futures_util > stream > try_stream > try_filter_map**

# Module: stream::try_stream::try_filter_map

## Contents

**Structs**

- [`TryFilterMap`](#tryfiltermap) - Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map)

---

## futures_util::stream::try_stream::try_filter_map::TryFilterMap

*Struct*

Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map)
method.

**Generic Parameters:**
- St
- Fut
- F

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



