*[futures_util](../../index.md) / [future](../index.md) / [try_join](index.md)*

---

# Module `try_join`

## Contents

- [Structs](#structs)
  - [`TryJoin`](#tryjoin)
  - [`TryJoin3`](#tryjoin3)
  - [`TryJoin4`](#tryjoin4)
  - [`TryJoin5`](#tryjoin5)
- [Functions](#functions)
  - [`try_join`](#try-join)
  - [`try_join3`](#try-join3)
  - [`try_join4`](#try-join4)
  - [`try_join5`](#try-join5)
- [Macros](#macros)
  - [`generate!`](#generate)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryJoin`](#tryjoin) | struct | Future for the [`try_join`](try_join()) function. |
| [`TryJoin3`](#tryjoin3) | struct | Future for the [`try_join3`] function. |
| [`TryJoin4`](#tryjoin4) | struct | Future for the [`try_join4`] function. |
| [`TryJoin5`](#tryjoin5) | struct | Future for the [`try_join5`] function. |
| [`try_join`](#try-join) | fn | Joins the result of two futures, waiting for them both to complete or for one to produce an error. |
| [`try_join3`](#try-join3) | fn | Same as [`try_join`](try_join()), but with more futures. |
| [`try_join4`](#try-join4) | fn | Same as [`try_join`](try_join()), but with more futures. |
| [`try_join5`](#try-join5) | fn | Same as [`try_join`](try_join()), but with more futures. |
| [`generate!`](#generate) | macro |  |

## Structs

### `TryJoin<Fut1: TryFuture, Fut2: TryFuture>`

```rust
struct TryJoin<Fut1: TryFuture, Fut2: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
}
```

Future for the [`try_join`](try_join()) function.

#### Implementations

- <span id="tryjoin-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2> Debug for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2> Future for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryJoin<Fut1, Fut2>`

##### `impl IntoFuture for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin<Fut1, Fut2>`

- <span id="tryjoin-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TryJoin<Fut1, Fut2>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture> Unpin for TryJoin<Fut1, Fut2>`

### `TryJoin3<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture>`

```rust
struct TryJoin3<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
    Fut3: TryMaybeDone<Fut3>,
}
```

Future for the [`try_join3`](#try-join3) function.

#### Implementations

- <span id="tryjoin3-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3> Debug for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2, Fut3> Future for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok, <Fut3 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin3-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryJoin3<Fut1, Fut2, Fut3>`

##### `impl IntoFuture for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin3-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin3-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin3<Fut1, Fut2, Fut3>`

- <span id="tryjoin3-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin3-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin3-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TryJoin3<Fut1, Fut2, Fut3>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture> Unpin for TryJoin3<Fut1, Fut2, Fut3>`

### `TryJoin4<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture>`

```rust
struct TryJoin4<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
    Fut3: TryMaybeDone<Fut3>,
    Fut4: TryMaybeDone<Fut4>,
}
```

Future for the [`try_join4`](#try-join4) function.

#### Implementations

- <span id="tryjoin4-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4> Debug for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2, Fut3, Fut4> Future for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok, <Fut3 as TryFuture>::Ok, <Fut4 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin4-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

##### `impl IntoFuture for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin4-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin4-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

- <span id="tryjoin4-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin4-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin4-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture> Unpin for TryJoin4<Fut1, Fut2, Fut3, Fut4>`

### `TryJoin5<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture, Fut5: TryFuture>`

```rust
struct TryJoin5<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture, Fut5: TryFuture> {
    Fut1: TryMaybeDone<Fut1>,
    Fut2: TryMaybeDone<Fut2>,
    Fut3: TryMaybeDone<Fut3>,
    Fut4: TryMaybeDone<Fut4>,
    Fut5: TryMaybeDone<Fut5>,
}
```

Future for the [`try_join5`](#try-join5) function.

#### Implementations

- <span id="tryjoin5-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4, Fut5: Fut5) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4, Fut5> Debug for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1, Fut2, Fut3, Fut4, Fut5> Future for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-future-type-output"></span>`type Output = Result<(<Fut1 as TryFuture>::Ok, <Fut2 as TryFuture>::Ok, <Fut3 as TryFuture>::Ok, <Fut4 as TryFuture>::Ok, <Fut5 as TryFuture>::Ok), <Fut1 as TryFuture>::Error>`

- <span id="tryjoin5-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

##### `impl IntoFuture for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryjoin5-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryjoin5-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="tryjoin5-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryjoin5-tryfuture-type-error"></span>`type Error = E`

- <span id="tryjoin5-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

##### `impl<Fut1: TryFuture, Fut2: TryFuture, Fut3: TryFuture, Fut4: TryFuture, Fut5: TryFuture> Unpin for TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>`

## Functions

### `try_join`

```rust
fn try_join<Fut1, Fut2>(future1: Fut1, future2: Fut2) -> TryJoin<Fut1, Fut2>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>
```

Joins the result of two futures, waiting for them both to complete or
for one to produce an error.

This function will return a new future which awaits both futures to
complete. If successful, the returned future will finish with a tuple of
both results. If unsuccessful, it will complete with the first error
encountered.

Note that this function consumes the passed futures and returns a
wrapped version of it.

# Examples

When used on multiple futures that return [`Ok`](../../../flate2/index.md), `try_join` will return
[`Ok`](../../../flate2/index.md) of a tuple of the values:

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let pair = future::try_join(a, b);

assert_eq!(pair.await, Ok((1, 2)));
});
```

If one of the futures resolves to an error, `try_join` will return
that error:

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Err::<i32, i32>(2));
let pair = future::try_join(a, b);

assert_eq!(pair.await, Err(2));
});
```

### `try_join3`

```rust
fn try_join3<Fut1, Fut2, Fut3>(future1: Fut1, future2: Fut2, future3: Fut3) -> TryJoin3<Fut1, Fut2, Fut3>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>,
    Fut3: TryFuture<Error = <Fut1 as >::Error>
```

Same as [`try_join`](try_join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let tuple = future::try_join3(a, b, c);

assert_eq!(tuple.await, Ok((1, 2, 3)));
});
```

### `try_join4`

```rust
fn try_join4<Fut1, Fut2, Fut3, Fut4>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4) -> TryJoin4<Fut1, Fut2, Fut3, Fut4>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>,
    Fut3: TryFuture<Error = <Fut1 as >::Error>,
    Fut4: TryFuture<Error = <Fut1 as >::Error>
```

Same as [`try_join`](try_join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let d = future::ready(Ok::<i32, i32>(4));
let tuple = future::try_join4(a, b, c, d);

assert_eq!(tuple.await, Ok((1, 2, 3, 4)));
});
```

### `try_join5`

```rust
fn try_join5<Fut1, Fut2, Fut3, Fut4, Fut5>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4, future5: Fut5) -> TryJoin5<Fut1, Fut2, Fut3, Fut4, Fut5>
where
    Fut1: TryFuture,
    Fut2: TryFuture<Error = <Fut1 as >::Error>,
    Fut3: TryFuture<Error = <Fut1 as >::Error>,
    Fut4: TryFuture<Error = <Fut1 as >::Error>,
    Fut5: TryFuture<Error = <Fut1 as >::Error>
```

Same as [`try_join`](try_join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(Ok::<i32, i32>(1));
let b = future::ready(Ok::<i32, i32>(2));
let c = future::ready(Ok::<i32, i32>(3));
let d = future::ready(Ok::<i32, i32>(4));
let e = future::ready(Ok::<i32, i32>(5));
let tuple = future::try_join5(a, b, c, d, e);

assert_eq!(tuple.await, Ok((1, 2, 3, 4, 5)));
});
```

## Macros

### `generate!`

