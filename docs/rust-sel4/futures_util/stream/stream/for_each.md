**futures_util > stream > stream > for_each**

# Module: stream::stream::for_each

## Contents

**Structs**

- [`ForEach`](#foreach) - Future for the [`for_each`](super::StreamExt::for_each) method.

---

## futures_util::stream::stream::for_each::ForEach

*Struct*

Future for the [`for_each`](super::StreamExt::for_each) method.

**Generic Parameters:**
- St
- Fut
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<()>`



