**futures_util > stream > try_stream > try_flatten_unordered**

# Module: stream::try_stream::try_flatten_unordered

## Contents

**Structs**

- [`NestedTryStreamIntoEitherTryStream`](#nestedtrystreamintoeithertrystream) - Emits either successful streams or single-item streams containing the underlying errors.
- [`PropagateBaseStreamError`](#propagatebasestreamerror) - Immediately propagates errors occurred in the base stream.
- [`Single`](#single) - Emits a single item immediately, then stream will be terminated.
- [`TryFlattenUnordered`](#tryflattenunordered) - Stream for the [`try_flatten_unordered`](super::TryStreamExt::try_flatten_unordered) method.

---

## futures_util::stream::try_stream::try_flatten_unordered::NestedTryStreamIntoEitherTryStream

*Struct*

Emits either successful streams or single-item streams containing the underlying errors.
This's a wrapper for `FlattenUnordered` to reuse its logic over `TryStream`.

**Generic Parameters:**
- St



## futures_util::stream::try_stream::try_flatten_unordered::PropagateBaseStreamError

*Struct*

Immediately propagates errors occurred in the base stream.

**Generic Parameters:**
- St

**Tuple Struct**: `()`



## futures_util::stream::try_stream::try_flatten_unordered::Single

*Struct*

Emits a single item immediately, then stream will be terminated.

**Generic Parameters:**
- T

**Tuple Struct**: `()`



## futures_util::stream::try_stream::try_flatten_unordered::TryFlattenUnordered

*Struct*

Stream for the [`try_flatten_unordered`](super::TryStreamExt::try_flatten_unordered) method.

**Generic Parameters:**
- St

**Methods:**

- `fn get_ref(self: &Self) -> &St` - Acquires a reference to the underlying sink or stream that this combinator is
- `fn get_mut(self: & mut Self) -> & mut St` - Acquires a mutable reference to the underlying sink or stream that this
- `fn get_pin_mut(self: core::pin::Pin<& mut Self>) -> core::pin::Pin<& mut St>` - Acquires a pinned mutable reference to the underlying sink or stream that this
- `fn into_inner(self: Self) -> St` - Consumes this combinator, returning the underlying sink or stream.

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Sink**
  - `fn poll_ready(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn start_send(self: core::pin::Pin<& mut Self>, item: _Item) -> Result<(), <Self as >::Error>`
  - `fn poll_flush(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
  - `fn poll_close(self: core::pin::Pin<& mut Self>, cx: & mut core::task::Context) -> core::task::Poll<Result<(), <Self as >::Error>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



