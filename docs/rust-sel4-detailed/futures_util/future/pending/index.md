*[futures_util](../../index.md) / [future](../index.md) / [pending](index.md)*

---

# Module `pending`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pending`](#pending) | struct | Future for the [`pending()`] function. |
| [`pending`](#pending) | fn | Creates a future which never resolves, representing a computation that never finishes. |

## Structs

### `Pending<T>`

```rust
struct Pending<T> {
    _data: marker::PhantomData<T>,
}
```

Future for the [`pending()`](#pending) function.

#### Trait Implementations

##### `impl<T> Clone for Pending<T>`

- <span id="pending-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Pending<T>`

- <span id="pending-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedFuture for Pending<T>`

- <span id="pending-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Future for Pending<T>`

- <span id="pending-future-type-output"></span>`type Output = T`

- <span id="pending-future-poll"></span>`fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<T>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl<T> FutureExt for Pending<T>`

##### `impl IntoFuture for Pending<T>`

- <span id="pending-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="pending-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="pending-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> TryFuture for Pending<T>`

- <span id="pending-tryfuture-type-ok"></span>`type Ok = T`

- <span id="pending-tryfuture-type-error"></span>`type Error = E`

- <span id="pending-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for Pending<T>`

##### `impl<T> Unpin for Pending<T>`

## Functions

### `pending`

```rust
fn pending<T>() -> Pending<T>
```

Creates a future which never resolves, representing a computation that never
finishes.

The returned future will forever return `Poll::Pending`.

# Examples

```ignore
futures::executor::block_on(async {
use futures::future;

let future = future::pending();
let () = future.await;
unreachable!();
});
```

