**futures_util > future > try_join_all**

# Module: future::try_join_all

## Contents

**Structs**

- [`TryJoinAll`](#tryjoinall) - Future for the [`try_join_all`] function.

**Functions**

- [`try_join_all`](#try_join_all) - Creates a future which represents either a collection of the results of the

---

## futures_util::future::try_join_all::TryJoinAll

*Struct*

Future for the [`try_join_all`] function.

**Generic Parameters:**
- F

**Trait Implementations:**

- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## futures_util::future::try_join_all::try_join_all

*Function*

Creates a future which represents either a collection of the results of the
futures given or an error.

The returned future will drive execution for all of its underlying futures,
collecting the results into a destination `Vec<T>` in the same order as they
were provided.

If any future returns an error then all other futures will be canceled and
an error will be returned immediately. If all futures complete successfully,
however, then the returned future will succeed with a `Vec` of all the
successful results.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# See Also

`try_join_all` will switch to the more powerful [`FuturesOrdered`] for performance
reasons if the number of futures is large. You may want to look into using it or
it's counterpart [`FuturesUnordered`][crate::stream::FuturesUnordered] directly.

Some examples for additional functionality provided by these are:

 * Adding new futures to the set even after it has been started.

 * Only polling the specific futures that have been woken. In cases where
   you have a lot of futures this will result in much more efficient polling.


# Examples

```
# futures::executor::block_on(async {
use futures::future::{self, try_join_all};

let futures = vec![
    future::ok::<u32, u32>(1),
    future::ok::<u32, u32>(2),
    future::ok::<u32, u32>(3),
];

assert_eq!(try_join_all(futures).await, Ok(vec![1, 2, 3]));

let futures = vec![
    future::ok::<u32, u32>(1),
    future::err::<u32, u32>(2),
    future::ok::<u32, u32>(3),
];

assert_eq!(try_join_all(futures).await, Err(2));
# });
```

```rust
fn try_join_all<I>(iter: I) -> TryJoinAll<<I as >::Item>
```



