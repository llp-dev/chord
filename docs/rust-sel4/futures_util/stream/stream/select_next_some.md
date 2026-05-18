**futures_util > stream > stream > select_next_some**

# Module: stream::stream::select_next_some

## Contents

**Structs**

- [`SelectNextSome`](#selectnextsome) - Future for the [`select_next_some`](super::StreamExt::select_next_some)

---

## futures_util::stream::stream::select_next_some::SelectNextSome

*Struct*

Future for the [`select_next_some`](super::StreamExt::select_next_some)
method.

**Generic Parameters:**
- 'a
- St

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



