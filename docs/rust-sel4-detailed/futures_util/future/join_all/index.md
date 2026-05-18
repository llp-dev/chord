*[futures_util](../../index.md) / [future](../index.md) / [join_all](index.md)*

---

# Module `join_all`

Definition of the `JoinAll` combinator, waiting for all of a list of futures
to finish.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`JoinAll`](#joinall) | struct | Future for the [`join_all`] function. |
| [`JoinAllKind`](#joinallkind) | enum |  |
| [`iter_pin_mut`](#iter-pin-mut) | fn |  |
| [`join_all`](#join-all) | fn | Creates a future which represents a collection of the outputs of the futures given. |
| [`SMALL`](#small) | const |  |

## Structs

### `JoinAll<F>`

```rust
struct JoinAll<F>
where
    F: Future {
    kind: JoinAllKind<F>,
}
```

Future for the [`join_all`](#join-all) function.

#### Trait Implementations

##### `impl<F> Debug for JoinAll<F>`

- <span id="joinall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: Future> FromIterator for JoinAll<F>`

- <span id="joinall-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = F>>(iter: T) -> Self`

##### `impl<F> Future for JoinAll<F>`

- <span id="joinall-future-type-output"></span>`type Output = Vec<<F as Future>::Output>`

- <span id="joinall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for JoinAll<F>`

##### `impl<F> IntoFuture for JoinAll<F>`

- <span id="joinall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="joinall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="joinall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

## Enums

### `JoinAllKind<F>`

```rust
enum JoinAllKind<F>
where
    F: Future {
    Small {
        elems: core::pin::Pin<alloc::boxed::Box<[super::MaybeDone<F>]>>,
    },
    Big {
        fut: crate::stream::Collect<crate::stream::FuturesOrdered<F>, alloc::vec::Vec<<F as >::Output>>,
    },
}
```

## Functions

### `iter_pin_mut`

```rust
fn iter_pin_mut<T>(slice: core::pin::Pin<&mut [T]>) -> impl Iterator<Item = core::pin::Pin<&mut T>>
```

### `join_all`

```rust
fn join_all<I>(iter: I) -> JoinAll<<I as >::Item>
where
    I: IntoIterator,
    <I as >::Item: Future
```

Creates a future which represents a collection of the outputs of the futures
given.

The returned future will drive execution for all of its underlying futures,
collecting the results into a destination `Vec<T>` in the same order as they
were provided.

This function is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# See Also

`join_all` will switch to the more powerful [`FuturesOrdered`](../../stream/futures_ordered/index.md) for performance
reasons if the number of futures is large. You may want to look into using it or
its counterpart `FuturesUnordered` directly.

Some examples for additional functionality provided by these are:

 * Adding new futures to the set even after it has been started.

 * Only polling the specific futures that have been woken. In cases where
   you have a lot of futures this will result in much more efficient polling.

# Examples

```rust
futures::executor::block_on(async {
use futures::future::join_all;

async fn foo(i: u32) -> u32 { i }

let futures = vec![foo(1), foo(2), foo(3)];

assert_eq!(join_all(futures).await, [1, 2, 3]);
});
```

## Constants

### `SMALL`
```rust
const SMALL: usize = 30usize;
```

