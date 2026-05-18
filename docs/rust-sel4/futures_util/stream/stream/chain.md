**futures_util > stream > stream > chain**

# Module: stream::stream::chain

## Contents

**Structs**

- [`Chain`](#chain) - Stream for the [`chain`](super::StreamExt::chain) method.

---

## futures_util::stream::stream::chain::Chain

*Struct*

Stream for the [`chain`](super::StreamExt::chain) method.

**Generic Parameters:**
- St1
- St2

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`



