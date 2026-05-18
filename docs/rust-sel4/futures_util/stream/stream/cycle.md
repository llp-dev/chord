**futures_util > stream > stream > cycle**

# Module: stream::stream::cycle

## Contents

**Structs**

- [`Cycle`](#cycle) - Stream for the [`cycle`](super::StreamExt::cycle) method.

---

## futures_util::stream::stream::cycle::Cycle

*Struct*

Stream for the [`cycle`](super::StreamExt::cycle) method.

**Generic Parameters:**
- St

**Trait Implementations:**

- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



