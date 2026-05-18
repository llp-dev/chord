**futures_util > stream > stream > take_until**

# Module: stream::stream::take_until

## Contents

**Structs**

- [`TakeUntil`](#takeuntil) - Stream for the [`take_until`](super::StreamExt::take_until) method.

---

## futures_util::stream::stream::take_until::TakeUntil

*Struct*

Stream for the [`take_until`](super::StreamExt::take_until) method.

**Generic Parameters:**
- St
- Fut

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.
- `fn take_future(self: & mut Self) -> Option<Fut>` - Extract the stopping future out of the combinator.
- `fn take_result(self: & mut Self) -> Option<<Fut as >::Output>` - Once the stopping future is resolved, this method can be used
- `fn is_stopped(self: &Self) -> bool` - Whether the stream was stopped yet by the stopping future

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<St as >::Item>>`
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



