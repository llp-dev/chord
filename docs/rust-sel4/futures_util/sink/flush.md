**futures_util > sink > flush**

# Module: sink::flush

## Contents

**Structs**

- [`Flush`](#flush) - Future for the [`flush`](super::SinkExt::flush) method.

---

## futures_util::sink::flush::Flush

*Struct*

Future for the [`flush`](super::SinkExt::flush) method.

**Generic Parameters:**
- 'a
- Si
- Item

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



