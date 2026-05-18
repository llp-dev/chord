**futures_util > future > pending**

# Module: future::pending

## Contents

**Structs**

- [`Pending`](#pending) - Future for the [`pending()`] function.

**Functions**

- [`pending`](#pending) - Creates a future which never resolves, representing a computation that never

---

## futures_util::future::pending::Pending

*Struct*

Future for the [`pending()`] function.

**Generic Parameters:**
- T

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, _: & mut Context) -> Poll<T>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## futures_util::future::pending::pending

*Function*

Creates a future which never resolves, representing a computation that never
finishes.

The returned future will forever return [`Poll::Pending`].

# Examples

```ignore
# futures::executor::block_on(async {
use futures::future;

let future = future::pending();
let () = future.await;
unreachable!();
# });
```

```rust
fn pending<T>() -> Pending<T>
```



