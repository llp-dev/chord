**futures_util > stream > try_stream > try_next**

# Module: stream::try_stream::try_next

## Contents

**Structs**

- [`TryNext`](#trynext) - Future for the [`try_next`](super::TryStreamExt::try_next) method.

---

## futures_util::stream::try_stream::try_next::TryNext

*Struct*

Future for the [`try_next`](super::TryStreamExt::try_next) method.

**Generic Parameters:**
- 'a
- St

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



