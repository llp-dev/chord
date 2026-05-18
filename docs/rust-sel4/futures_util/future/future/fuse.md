**futures_util > future > future > fuse**

# Module: future::future::fuse

## Contents

**Structs**

- [`Fuse`](#fuse) - Future for the [`fuse`](super::FutureExt::fuse) method.

---

## futures_util::future::future::fuse::Fuse

*Struct*

Future for the [`fuse`](super::FutureExt::fuse) method.

**Generic Parameters:**
- Fut

**Methods:**

- `fn terminated() -> Self` - Creates a new `Fuse`-wrapped future which is already terminated.

**Trait Implementations:**

- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Fut as >::Output>`



