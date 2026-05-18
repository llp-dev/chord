**futures_util > future > try_future > into_future**

# Module: future::try_future::into_future

## Contents

**Structs**

- [`IntoFuture`](#intofuture) - Future for the [`into_future`](super::TryFutureExt::into_future) method.

---

## futures_util::future::try_future::into_future::IntoFuture

*Struct*

Future for the [`into_future`](super::TryFutureExt::into_future) method.

**Generic Parameters:**
- Fut

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



