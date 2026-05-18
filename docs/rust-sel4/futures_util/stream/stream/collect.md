**futures_util > stream > stream > collect**

# Module: stream::stream::collect

## Contents

**Structs**

- [`Collect`](#collect) - Future for the [`collect`](super::StreamExt::collect) method.

---

## futures_util::stream::stream::collect::Collect

*Struct*

Future for the [`collect`](super::StreamExt::collect) method.

**Generic Parameters:**
- St
- C

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<C>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



