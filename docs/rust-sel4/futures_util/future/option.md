**futures_util > future > option**

# Module: future::option

## Contents

**Structs**

- [`OptionFuture`](#optionfuture) - A future representing a value which may or may not be present.

---

## futures_util::future::option::OptionFuture

*Struct*

A future representing a value which may or may not be present.

Created by the [`From`] implementation for [`Option`](std::option::Option).

# Examples

```
# futures::executor::block_on(async {
use futures::future::OptionFuture;

let mut a: OptionFuture<_> = Some(async { 123 }).into();
assert_eq!(a.await, Some(123));

a = None.into();
assert_eq!(a.await, None);
# });
```

**Generic Parameters:**
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> OptionFuture<F>`
- **From**
  - `fn from(option: Option<T>) -> Self`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



