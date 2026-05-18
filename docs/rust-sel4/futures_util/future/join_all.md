**futures_util > future > join_all**

# Module: future::join_all

## Contents

**Structs**

- [`JoinAll`](#joinall) - Future for the [`join_all`] function.

**Functions**

- [`join_all`](#join_all) - Creates a future which represents a collection of the outputs of the futures

---

## futures_util::future::join_all::JoinAll

*Struct*

Future for the [`join_all`] function.

**Generic Parameters:**
- F

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## futures_util::future::join_all::join_all

*Function*

Creates a future which represents a collection of the outputs of the futures
given.

The returned future will drive execution for all of its underlying futures,
collecting the results into a destination `Vec<T>` in the same order as they
were provided.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# See Also

`join_all` will switch to the more powerful [`FuturesOrdered`] for performance
reasons if the number of futures is large. You may want to look into using it or
its counterpart [`FuturesUnordered`][crate::stream::FuturesUnordered] directly.

Some examples for additional functionality provided by these are:

 * Adding new futures to the set even after it has been started.

 * Only polling the specific futures that have been woken. In cases where
   you have a lot of futures this will result in much more efficient polling.

# Examples

```
# futures::executor::block_on(async {
use futures::future::join_all;

async fn foo(i: u32) -> u32 { i }

let futures = vec![foo(1), foo(2), foo(3)];

assert_eq!(join_all(futures).await, [1, 2, 3]);
# });
```

```rust
fn join_all<I>(iter: I) -> JoinAll<<I as >::Item>
```



