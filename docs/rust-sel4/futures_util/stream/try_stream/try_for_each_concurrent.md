**futures_util > stream > try_stream > try_for_each_concurrent**

# Module: stream::try_stream::try_for_each_concurrent

## Contents

**Structs**

- [`TryForEachConcurrent`](#tryforeachconcurrent) - Future for the

---

## futures_util::stream::try_stream::try_for_each_concurrent::TryForEachConcurrent

*Struct*

Future for the
[`try_for_each_concurrent`](super::TryStreamExt::try_for_each_concurrent)
method.

**Generic Parameters:**
- St
- Fut
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



