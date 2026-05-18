**futures_util > sink > feed**

# Module: sink::feed

## Contents

**Structs**

- [`Feed`](#feed) - Future for the [`feed`](super::SinkExt::feed) method.

---

## futures_util::sink::feed::Feed

*Struct*

Future for the [`feed`](super::SinkExt::feed) method.

**Generic Parameters:**
- 'a
- Si
- Item

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



