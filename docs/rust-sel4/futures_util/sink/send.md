**futures_util > sink > send**

# Module: sink::send

## Contents

**Structs**

- [`Send`](#send) - Future for the [`send`](super::SinkExt::send) method.

---

## futures_util::sink::send::Send

*Struct*

Future for the [`send`](super::SinkExt::send) method.

**Generic Parameters:**
- 'a
- Si
- Item

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



