**futures_util > stream > stream > into_future**

# Module: stream::stream::into_future

## Contents

**Structs**

- [`StreamFuture`](#streamfuture) - Future for the [`into_future`](super::StreamExt::into_future) method.

---

## futures_util::stream::stream::into_future::StreamFuture

*Struct*

Future for the [`into_future`](super::StreamExt::into_future) method.

**Generic Parameters:**
- St

**Methods:**

- `fn get_ref(self: &Self) -> Option<&St>` - Acquires a reference to the underlying stream that this combinator is
- `fn get_mut(self: & mut Self) -> Option<& mut St>` - Acquires a mutable reference to the underlying stream that this
- `fn get_pin_mut(self: Pin<& mut Self>) -> Option<Pin<& mut St>>` - Acquires a pinned mutable reference to the underlying stream that this
- `fn into_inner(self: Self) -> Option<St>` - Consumes this combinator, returning the underlying stream.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



