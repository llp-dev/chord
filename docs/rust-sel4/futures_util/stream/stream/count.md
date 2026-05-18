**futures_util > stream > stream > count**

# Module: stream::stream::count

## Contents

**Structs**

- [`Count`](#count) - Future for the [`count`](super::StreamExt::count) method.

---

## futures_util::stream::stream::count::Count

*Struct*

Future for the [`count`](super::StreamExt::count) method.

**Generic Parameters:**
- St

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



