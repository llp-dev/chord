**futures_util > sink > send_all**

# Module: sink::send_all

## Contents

**Structs**

- [`SendAll`](#sendall) - Future for the [`send_all`](super::SinkExt::send_all) method.

---

## futures_util::sink::send_all::SendAll

*Struct*

Future for the [`send_all`](super::SinkExt::send_all) method.

**Generic Parameters:**
- 'a
- Si
- St

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



