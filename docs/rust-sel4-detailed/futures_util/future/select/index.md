*[futures_util](../../index.md) / [future](../index.md) / [select](index.md)*

---

# Module `select`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Select`](#select) | struct | Future for the [`select()`] function. |
| [`select`](#select) | fn | Waits for either one of two differently-typed futures to complete. |

## Structs

### `Select<A, B>`

```rust
struct Select<A, B> {
    inner: Option<(A, B)>,
}
```

Future for the [`select()`](#select) function.

#### Trait Implementations

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Select<A, B>`

- <span id="select-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> FusedFuture for Select<A, B>`

- <span id="select-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<A, B> Future for Select<A, B>`

- <span id="select-future-type-output"></span>`type Output = Either<(<A as Future>::Output, B), (<B as Future>::Output, A)>`

- <span id="select-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Select<A, B>`

##### `impl IntoFuture for Select<A, B>`

- <span id="select-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="select-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="select-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<A: Unpin, B: Unpin> Unpin for Select<A, B>`

## Functions

### `select`

```rust
fn select<A, B>(future1: A, future2: B) -> Select<A, B>
where
    A: Future + Unpin,
    B: Future + Unpin
```

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

```rust
futures::executor::block_on(async {
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
});
```

A more complex example

```rust
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

