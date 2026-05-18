**futures_util > stream > try_stream > try_concat**

# Module: stream::try_stream::try_concat

## Contents

**Structs**

- [`TryConcat`](#tryconcat) - Future for the [`try_concat`](super::TryStreamExt::try_concat) method.

---

## futures_util::stream::try_stream::try_concat::TryConcat

*Struct*

Future for the [`try_concat`](super::TryStreamExt::try_concat) method.

**Generic Parameters:**
- St

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



