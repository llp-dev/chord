**futures_util > stream > try_stream > try_collect**

# Module: stream::try_stream::try_collect

## Contents

**Structs**

- [`TryCollect`](#trycollect) - Future for the [`try_collect`](super::TryStreamExt::try_collect) method.

---

## futures_util::stream::try_stream::try_collect::TryCollect

*Struct*

Future for the [`try_collect`](super::TryStreamExt::try_collect) method.

**Generic Parameters:**
- St
- C

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



