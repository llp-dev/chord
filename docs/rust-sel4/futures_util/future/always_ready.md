**futures_util > future > always_ready**

# Module: future::always_ready

## Contents

**Structs**

- [`AlwaysReady`](#alwaysready) - Future for the [`always_ready`](always_ready()) function.

**Functions**

- [`always_ready`](#always_ready) - Creates a future that is always immediately ready with a value.

---

## futures_util::future::always_ready::AlwaysReady

*Struct*

Future for the [`always_ready`](always_ready()) function.

**Generic Parameters:**
- T
- F

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, _cx: & mut Context) -> Poll<T>`



## futures_util::future::always_ready::always_ready

*Function*

Creates a future that is always immediately ready with a value.

This is particularly useful in avoiding a heap allocation when an API needs `Box<dyn Future<Output = T>>`,
as [`AlwaysReady`] does not have to store a boolean for `is_finished`.

# Examples

```
# futures::executor::block_on(async {
use std::mem::size_of_val;

use futures::future;

let a = future::always_ready(|| 1);
assert_eq!(size_of_val(&a), 0);
assert_eq!(a.await, 1);
assert_eq!(a.await, 1);
# });
```

```rust
fn always_ready<T, F>(prod: F) -> AlwaysReady<T, F>
```



