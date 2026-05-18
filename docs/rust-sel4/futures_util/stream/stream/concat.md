**futures_util > stream > stream > concat**

# Module: stream::stream::concat

## Contents

**Structs**

- [`Concat`](#concat) - Future for the [`concat`](super::StreamExt::concat) method.

---

## futures_util::stream::stream::concat::Concat

*Struct*

Future for the [`concat`](super::StreamExt::concat) method.

**Generic Parameters:**
- St

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`



