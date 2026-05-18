**futures_util > stream > try_stream > try_any**

# Module: stream::try_stream::try_any

## Contents

**Structs**

- [`TryAny`](#tryany) - Future for the [`try_any`](super::TryStreamExt::try_any) method.

---

## futures_util::stream::try_stream::try_any::TryAny

*Struct*

Future for the [`try_any`](super::TryStreamExt::try_any) method.

**Generic Parameters:**
- St
- Fut
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<bool, <St as >::Error>>`



