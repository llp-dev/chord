**futures_util > stream > stream > any**

# Module: stream::stream::any

## Contents

**Structs**

- [`Any`](#any) - Future for the [`any`](super::StreamExt::any) method.

---

## futures_util::stream::stream::any::Any

*Struct*

Future for the [`any`](super::StreamExt::any) method.

**Generic Parameters:**
- St
- Fut
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<bool>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



