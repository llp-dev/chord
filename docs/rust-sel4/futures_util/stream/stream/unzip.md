**futures_util > stream > stream > unzip**

# Module: stream::stream::unzip

## Contents

**Structs**

- [`Unzip`](#unzip) - Future for the [`unzip`](super::StreamExt::unzip) method.

---

## futures_util::stream::stream::unzip::Unzip

*Struct*

Future for the [`unzip`](super::StreamExt::unzip) method.

**Generic Parameters:**
- St
- FromA
- FromB

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<(FromA, FromB)>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



