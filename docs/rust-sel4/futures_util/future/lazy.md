**futures_util > future > lazy**

# Module: future::lazy

## Contents

**Structs**

- [`Lazy`](#lazy) - Future for the [`lazy`] function.

**Functions**

- [`lazy`](#lazy) - Creates a new future that allows delayed execution of a closure.

---

## futures_util::future::lazy::Lazy

*Struct*

Future for the [`lazy`] function.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<R>`



## futures_util::future::lazy::lazy

*Function*

Creates a new future that allows delayed execution of a closure.

The provided closure is only run once the future is polled.

# Examples

```
# futures::executor::block_on(async {
use futures::future;

let a = future::lazy(|_| 1);
assert_eq!(a.await, 1);

let b = future::lazy(|_| -> i32 {
    panic!("oh no!")
});
drop(b); // closure is never run
# });
```

```rust
fn lazy<F, R>(f: F) -> Lazy<F>
```



