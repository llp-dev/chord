**futures_util > future > try_select**

# Module: future::try_select

## Contents

**Structs**

- [`TrySelect`](#tryselect) - Future for the [`try_select()`] function.

**Functions**

- [`try_select`](#try_select) - Waits for either one of two differently-typed futures to complete.

---

## futures_util::future::try_select::TrySelect

*Struct*

Future for the [`try_select()`] function.

**Generic Parameters:**
- A
- B

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_util::future::try_select::try_select

*Function*

Waits for either one of two differently-typed futures to complete.

This function will return a new future which awaits for either one of both
futures to complete. The returned future will finish with both the value
resolved and a future representing the completion of the other work.

Note that this function consumes the receiving futures and returns a
wrapped version of them.

Also note that if both this and the second future have the same
success/error type you can use the `Either::factor_first` method to
conveniently extract out the value at the end.

# Examples

```
use futures::future::{self, Either, Future, FutureExt, TryFuture, TryFutureExt};

// A poor-man's try_join implemented on top of select

fn try_join<A, B, E>(a: A, b: B) -> impl TryFuture<Ok=(A::Ok, B::Ok), Error=E>
     where A: TryFuture<Error = E> + Unpin + 'static,
           B: TryFuture<Error = E> + Unpin + 'static,
           E: 'static,
{
    future::try_select(a, b).then(|res| -> Box<dyn Future<Output = Result<_, _>> + Unpin> {
        match res {
            Ok(Either::Left((x, b))) => Box::new(b.map_ok(move |y| (x, y))),
            Ok(Either::Right((y, a))) => Box::new(a.map_ok(move |x| (x, y))),
            Err(Either::Left((e, _))) => Box::new(future::err(e)),
            Err(Either::Right((e, _))) => Box::new(future::err(e)),
        }
    })
}
```

```rust
fn try_select<A, B>(future1: A, future2: B) -> TrySelect<A, B>
```



