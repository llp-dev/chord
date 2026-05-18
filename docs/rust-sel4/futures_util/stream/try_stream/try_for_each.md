**futures_util > stream > try_stream > try_for_each**

# Module: stream::try_stream::try_for_each

## Contents

**Structs**

- [`TryForEach`](#tryforeach) - Future for the [`try_for_each`](super::TryStreamExt::try_for_each) method.

---

## futures_util::stream::try_stream::try_for_each::TryForEach

*Struct*

Future for the [`try_for_each`](super::TryStreamExt::try_for_each) method.

**Generic Parameters:**
- St
- Fut
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



