**futures_util > stream > stream > for_each_concurrent**

# Module: stream::stream::for_each_concurrent

## Contents

**Structs**

- [`ForEachConcurrent`](#foreachconcurrent) - Future for the [`for_each_concurrent`](super::StreamExt::for_each_concurrent)

---

## futures_util::stream::stream::for_each_concurrent::ForEachConcurrent

*Struct*

Future for the [`for_each_concurrent`](super::StreamExt::for_each_concurrent)
method.

**Generic Parameters:**
- St
- Fut
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



