**futures_util > future > select**

# Module: future::select

## Contents

**Structs**

- [`Select`](#select) - Future for the [`select()`] function.

**Functions**

- [`select`](#select) - Waits for either one of two differently-typed futures to complete.

---

## futures_util::future::select::Select

*Struct*

Future for the [`select()`] function.

**Generic Parameters:**
- A
- B

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`



## futures_util::future::select::select

*Function*

Waits for either one of two differently-typed futures to complete.

This function will return a new future which awaits for either one of both
futures to complete. The returned future will finish with both the value
resolved and a future representing the completion of the other work.

Note that this function consumes the receiving futures and returns a
wrapped version of them.

Also note that if both this and the second future have the same
output type you can use the `Either::factor_first` method to
conveniently extract out the value at the end.

# Examples

A simple example

```
# futures::executor::block_on(async {
use core::pin::pin;

use futures::future;
use futures::future::Either;

// These two futures have different types even though their outputs have the same type.
let future1 = async {
    future::pending::<()>().await; // will never finish
    1
};
let future2 = async {
    future::ready(2).await
};

// 'select' requires Future + Unpin bounds
let future1 = pin!(future1);
let future2 = pin!(future2);

let value = match future::select(future1, future2).await {
    Either::Left((value1, _)) => value1,  // `value1` is resolved from `future1`
                                          // `_` represents `future2`
    Either::Right((value2, _)) => value2, // `value2` is resolved from `future2`
                                          // `_` represents `future1`
};

assert!(value == 2);
# });
```

A more complex example

```
use futures::future::{self, Either, Future, FutureExt};

// A poor-man's join implemented on top of select

fn join<A, B>(a: A, b: B) -> impl Future<Output=(A::Output, B::Output)>
    where A: Future + Unpin,
          B: Future + Unpin,
{
    future::select(a, b).then(|either| {
        match either {
            Either::Left((x, b)) => b.map(move |y| (x, y)).left_future(),
            Either::Right((y, a)) => a.map(move |x| (x, y)).right_future(),
        }
    })
}
```

```rust
fn select<A, B>(future1: A, future2: B) -> Select<A, B>
```



