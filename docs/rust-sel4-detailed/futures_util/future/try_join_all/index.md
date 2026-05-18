*[futures_util](../../index.md) / [future](../index.md) / [try_join_all](index.md)*

---

# Module `try_join_all`

Definition of the `TryJoinAll` combinator, waiting for all of a list of
futures to finish with either success or error.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryJoinAll`](#tryjoinall) | struct | Future for the [`try_join_all`] function. |
| [`FinalState`](#finalstate) | enum |  |
| [`TryJoinAllKind`](#tryjoinallkind) | enum |  |
| [`try_join_all`](#try-join-all) | fn | Creates a future which represents either a collection of the results of the futures given or an error. |

## Structs

### `TryJoinAll<F>`

```rust
struct TryJoinAll<F>
where
    F: TryFuture {
    kind: TryJoinAllKind<F>,
}
```

Future for the [`try_join_all`](#try-join-all) function.

#### Trait Implementations

##### `impl<F> Debug for TryJoinAll<F>`

- <span id="tryjoinall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> FromIterator for TryJoinAll<F>`

- <span id="tryjoinall-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = F>>(iter: T) -> Self`

##### `impl<F> Future for TryJoinAll<F>`

- <span id="tryjoinall-future-type-output"></span>`type Output = Result<Vec<<F as TryFuture>::Ok>, <F as TryFuture>::Error>`

- <span id="tryjoinall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryJoinAll<F>`

##### `impl<F> IntoFuture for TryJoinAll<F>`

- <span id="tryjoinall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoinall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoinall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryJoinAll<F>`

- <span id="tryjoinall-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoinall-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoinall-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TryJoinAll<F>`

## Enums

### `FinalState<E>`

```rust
enum FinalState<E> {
    Pending,
    AllDone,
    Error(E),
}
```

### `TryJoinAllKind<F>`

```rust
enum TryJoinAllKind<F>
where
    F: TryFuture {
    Small {
        elems: core::pin::Pin<alloc::boxed::Box<[super::TryMaybeDone<super::IntoFuture<F>>]>>,
    },
    Big {
        fut: crate::stream::TryCollect<crate::stream::FuturesOrdered<super::IntoFuture<F>>, alloc::vec::Vec<<F as >::Ok>>,
    },
}
```

## Functions

### `try_join_all`

```rust
fn try_join_all<I>(iter: I) -> TryJoinAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: TryFuture
```

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

`try_join_all` will switch to the more powerful [`FuturesOrdered`](../../stream/futures_ordered/index.md) for performance
reasons if the number of futures is large. You may want to look into using it or
it's counterpart `FuturesUnordered` directly.

Some examples for additional functionality provided by these are:

 * Adding new futures to the set even after it has been started.

 * Only polling the specific futures that have been woken. In cases where
   you have a lot of futures this will result in much more efficient polling.


# Examples

```rust
futures::executor::block_on(async {
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
});
```

