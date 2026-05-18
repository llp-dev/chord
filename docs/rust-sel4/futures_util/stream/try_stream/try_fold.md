**futures_util > stream > try_stream > try_fold**

# Module: stream::try_stream::try_fold

## Contents

**Structs**

- [`TryFold`](#tryfold) - Future for the [`try_fold`](super::TryStreamExt::try_fold) method.

---

## futures_util::stream::try_stream::try_fold::TryFold

*Struct*

Future for the [`try_fold`](super::TryStreamExt::try_fold) method.

**Generic Parameters:**
- St
- Fut
- T
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



