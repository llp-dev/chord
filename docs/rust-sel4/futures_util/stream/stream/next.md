**futures_util > stream > stream > next**

# Module: stream::stream::next

## Contents

**Structs**

- [`Next`](#next) - Future for the [`next`](super::StreamExt::next) method.

---

## futures_util::stream::stream::next::Next

*Struct*

Future for the [`next`](super::StreamExt::next) method.

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



