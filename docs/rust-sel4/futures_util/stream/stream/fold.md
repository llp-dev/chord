**futures_util > stream > stream > fold**

# Module: stream::stream::fold

## Contents

**Structs**

- [`Fold`](#fold) - Future for the [`fold`](super::StreamExt::fold) method.

---

## futures_util::stream::stream::fold::Fold

*Struct*

Future for the [`fold`](super::StreamExt::fold) method.

**Generic Parameters:**
- St
- Fut
- T
- F

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



