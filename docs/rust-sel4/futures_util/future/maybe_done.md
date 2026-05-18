**futures_util > future > maybe_done**

# Module: future::maybe_done

## Contents

**Enums**

- [`MaybeDone`](#maybedone) - A future that may have completed.

**Functions**

- [`maybe_done`](#maybe_done) - Wraps a future into a `MaybeDone`

---

## futures_util::future::maybe_done::MaybeDone

*Enum*

A future that may have completed.

This is created by the [`maybe_done()`] function.

**Generic Parameters:**
- Fut

**Variants:**
- `Future(Fut)` - A not-yet-completed future
- `Done(<Fut as >::Output)` - The output of the completed future
- `Gone` - The empty variant after the result of a [`MaybeDone`] has been

**Methods:**

- `fn output_mut(self: Pin<& mut Self>) -> Option<& mut <Fut as >::Output>` - Returns an [`Option`] containing a mutable reference to the output of the future.
- `fn take_output(self: Pin<& mut Self>) -> Option<<Fut as >::Output>` - Attempt to take the output of a `MaybeDone` without driving it

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::future::maybe_done::maybe_done

*Function*

Wraps a future into a `MaybeDone`

# Examples

```
# futures::executor::block_on(async {
use core::pin::pin;

use futures::future;

let future = future::maybe_done(async { 5 });
let mut future = pin!(future);
assert_eq!(future.as_mut().take_output(), None);
let () = future.as_mut().await;
assert_eq!(future.as_mut().take_output(), Some(5));
assert_eq!(future.as_mut().take_output(), None);
# });
```

```rust
fn maybe_done<Fut>(future: Fut) -> MaybeDone<Fut>
```



