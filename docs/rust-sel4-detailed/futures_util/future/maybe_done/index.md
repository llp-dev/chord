*[futures_util](../../index.md) / [future](../index.md) / [maybe_done](index.md)*

---

# Module `maybe_done`

Definition of the MaybeDone combinator

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MaybeDone`](#maybedone) | enum | A future that may have completed. |
| [`maybe_done`](#maybe-done) | fn | Wraps a future into a `MaybeDone` |

## Enums

### `MaybeDone<Fut: Future>`

```rust
enum MaybeDone<Fut: Future> {
    Future(Fut),
    Done(<Fut as >::Output),
    Gone,
}
```

A future that may have completed.

This is created by the [`maybe_done()`](#maybe-done) function.

#### Variants

- **`Future`**

  A not-yet-completed future

- **`Done`**

  The output of the completed future

- **`Gone`**

  The empty variant after the result of a [`MaybeDone`](#maybedone) has been
  taken using the [`take_output`](MaybeDone::take_output) method.

#### Implementations

- <span id="maybedone-output-mut"></span>`fn output_mut(self: Pin<&mut Self>) -> Option<&mut <Fut as >::Output>` — [`Future`](../index.md#future)

  Returns an [`Option`](../../../serde_core/index.md) containing a mutable reference to the output of the future.

  The output of this method will be [`Some`](../../../managed/index.md) if and only if the inner

  future has been completed and [`take_output`](MaybeDone::take_output)

  has not yet been called.

- <span id="maybedone-take-output"></span>`fn take_output(self: Pin<&mut Self>) -> Option<<Fut as >::Output>` — [`Future`](../index.md#future)

  Attempt to take the output of a `MaybeDone` without driving it

  towards completion.

#### Trait Implementations

##### `impl<Fut: fmt::Debug + Future> Debug for MaybeDone<Fut>`

- <span id="maybedone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> FusedFuture for MaybeDone<Fut>`

- <span id="maybedone-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Future for MaybeDone<Fut>`

- <span id="maybedone-future-type-output"></span>`type Output = ()`

- <span id="maybedone-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl FutureExt for MaybeDone<Fut>`

##### `impl IntoFuture for MaybeDone<Fut>`

- <span id="maybedone-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="maybedone-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="maybedone-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<Fut: Future + Unpin> Unpin for MaybeDone<Fut>`

## Functions

### `maybe_done`

```rust
fn maybe_done<Fut: Future>(future: Fut) -> MaybeDone<Fut>
```

Wraps a future into a `MaybeDone`

# Examples

```rust
futures::executor::block_on(async {
use core::pin::pin;

use futures::future;

let future = future::maybe_done(async { 5 });
let mut future = pin!(future);
assert_eq!(future.as_mut().take_output(), None);
let () = future.as_mut().await;
assert_eq!(future.as_mut().take_output(), Some(5));
assert_eq!(future.as_mut().take_output(), None);
});
```

