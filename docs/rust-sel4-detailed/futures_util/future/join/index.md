*[futures_util](../../index.md) / [future](../index.md) / [join](index.md)*

---

# Module `join`

## Contents

- [Structs](#structs)
  - [`Join`](#join)
  - [`Join3`](#join3)
  - [`Join4`](#join4)
  - [`Join5`](#join5)
- [Functions](#functions)
  - [`join`](#join)
  - [`join3`](#join3)
  - [`join4`](#join4)
  - [`join5`](#join5)
- [Macros](#macros)
  - [`generate!`](#generate)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Join`](#join) | struct | Future for the [`join`](join()) function. |
| [`Join3`](#join3) | struct | Future for the [`join3`] function. |
| [`Join4`](#join4) | struct | Future for the [`join4`] function. |
| [`Join5`](#join5) | struct | Future for the [`join5`] function. |
| [`join`](#join) | fn | Joins the result of two futures, waiting for them both to complete. |
| [`join3`](#join3) | fn | Same as [`join`](join()), but with more futures. |
| [`join4`](#join4) | fn | Same as [`join`](join()), but with more futures. |
| [`join5`](#join5) | fn | Same as [`join`](join()), but with more futures. |
| [`generate!`](#generate) | macro |  |

## Structs

### `Join<Fut1: Future, Fut2: Future>`

```rust
struct Join<Fut1: Future, Fut2: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
}
```

Future for the [`join`](join()) function.

#### Implementations

- <span id="join-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2> Debug for Join<Fut1, Fut2>`

- <span id="join-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture> FusedFuture for Join<Fut1, Fut2>`

- <span id="join-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future> Future for Join<Fut1, Fut2>`

- <span id="join-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output)`

- <span id="join-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Join<Fut1, Fut2>`

##### `impl IntoFuture for Join<Fut1, Fut2>`

- <span id="join-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future> Unpin for Join<Fut1, Fut2>`

### `Join3<Fut1: Future, Fut2: Future, Fut3: Future>`

```rust
struct Join3<Fut1: Future, Fut2: Future, Fut3: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
    Fut3: MaybeDone<Fut3>,
}
```

Future for the [`join3`](#join3) function.

#### Implementations

- <span id="join3-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3> Debug for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture, Fut3: FusedFuture> FusedFuture for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future> Future for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output, <Fut3 as Future>::Output)`

- <span id="join3-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Join3<Fut1, Fut2, Fut3>`

##### `impl IntoFuture for Join3<Fut1, Fut2, Fut3>`

- <span id="join3-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join3-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join3-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future> Unpin for Join3<Fut1, Fut2, Fut3>`

### `Join4<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future>`

```rust
struct Join4<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
    Fut3: MaybeDone<Fut3>,
    Fut4: MaybeDone<Fut4>,
}
```

Future for the [`join4`](#join4) function.

#### Implementations

- <span id="join4-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4> Debug for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture, Fut3: FusedFuture, Fut4: FusedFuture> FusedFuture for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future> Future for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output, <Fut3 as Future>::Output, <Fut4 as Future>::Output)`

- <span id="join4-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Join4<Fut1, Fut2, Fut3, Fut4>`

##### `impl IntoFuture for Join4<Fut1, Fut2, Fut3, Fut4>`

- <span id="join4-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join4-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join4-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future> Unpin for Join4<Fut1, Fut2, Fut3, Fut4>`

### `Join5<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future>`

```rust
struct Join5<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future> {
    Fut1: MaybeDone<Fut1>,
    Fut2: MaybeDone<Fut2>,
    Fut3: MaybeDone<Fut3>,
    Fut4: MaybeDone<Fut4>,
    Fut5: MaybeDone<Fut5>,
}
```

Future for the [`join5`](#join5) function.

#### Implementations

- <span id="join5-new"></span>`fn new(Fut1: Fut1, Fut2: Fut2, Fut3: Fut3, Fut4: Fut4, Fut5: Fut5) -> Self`

#### Trait Implementations

##### `impl<Fut1, Fut2, Fut3, Fut4, Fut5> Debug for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut1: FusedFuture, Fut2: FusedFuture, Fut3: FusedFuture, Fut4: FusedFuture, Fut5: FusedFuture> FusedFuture for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future> Future for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-future-type-output"></span>`type Output = (<Fut1 as Future>::Output, <Fut2 as Future>::Output, <Fut3 as Future>::Output, <Fut4 as Future>::Output, <Fut5 as Future>::Output)`

- <span id="join5-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

##### `impl IntoFuture for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

- <span id="join5-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="join5-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="join5-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut1: Future, Fut2: Future, Fut3: Future, Fut4: Future, Fut5: Future> Unpin for Join5<Fut1, Fut2, Fut3, Fut4, Fut5>`

## Functions

### `join`

```rust
fn join<Fut1, Fut2>(future1: Fut1, future2: Fut2) -> Join<Fut1, Fut2>
where
    Fut1: Future,
    Fut2: Future
```

Joins the result of two futures, waiting for them both to complete.

This function will return a new future which awaits both futures to
complete. The returned future will finish with a tuple of both results.

Note that this function consumes the passed futures and returns a
wrapped version of it.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let pair = future::join(a, b);

assert_eq!(pair.await, (1, 2));
});
```

### `join3`

```rust
fn join3<Fut1, Fut2, Fut3>(future1: Fut1, future2: Fut2, future3: Fut3) -> Join3<Fut1, Fut2, Fut3>
where
    Fut1: Future,
    Fut2: Future,
    Fut3: Future
```

Same as [`join`](join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let tuple = future::join3(a, b, c);

assert_eq!(tuple.await, (1, 2, 3));
});
```

### `join4`

```rust
fn join4<Fut1, Fut2, Fut3, Fut4>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4) -> Join4<Fut1, Fut2, Fut3, Fut4>
where
    Fut1: Future,
    Fut2: Future,
    Fut3: Future,
    Fut4: Future
```

Same as [`join`](join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let d = async { 4 };
let tuple = future::join4(a, b, c, d);

assert_eq!(tuple.await, (1, 2, 3, 4));
});
```

### `join5`

```rust
fn join5<Fut1, Fut2, Fut3, Fut4, Fut5>(future1: Fut1, future2: Fut2, future3: Fut3, future4: Fut4, future5: Fut5) -> Join5<Fut1, Fut2, Fut3, Fut4, Fut5>
where
    Fut1: Future,
    Fut2: Future,
    Fut3: Future,
    Fut4: Future,
    Fut5: Future
```

Same as [`join`](join()), but with more futures.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = async { 1 };
let b = async { 2 };
let c = async { 3 };
let d = async { 4 };
let e = async { 5 };
let tuple = future::join5(a, b, c, d, e);

assert_eq!(tuple.await, (1, 2, 3, 4, 5));
});
```

## Macros

### `generate!`

