*[futures_util](../../index.md) / [future](../index.md) / [always_ready](index.md)*

---

# Module `always_ready`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AlwaysReady`](#alwaysready) | struct | Future for the [`always_ready`](always_ready()) function. |
| [`always_ready`](#always-ready) | fn | Creates a future that is always immediately ready with a value. |

## Structs

### `AlwaysReady<T, F: Fn() -> T>`

```rust
struct AlwaysReady<T, F: Fn() -> T>(F);
```

Future for the [`always_ready`](always_ready()) function.

#### Trait Implementations

##### `impl<T, F: Fn() -> T + Clone> Clone for AlwaysReady<T, F>`

- <span id="alwaysready-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, F: Fn() -> T + Copy> Copy for AlwaysReady<T, F>`

##### `impl<T, F: Fn() -> T> Debug for AlwaysReady<T, F>`

- <span id="alwaysready-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T, F: Fn() -> T> FusedFuture for AlwaysReady<T, F>`

- <span id="alwaysready-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T, F: Fn() -> T> Future for AlwaysReady<T, F>`

- <span id="alwaysready-future-type-output"></span>`type Output = T`

- <span id="alwaysready-future-poll"></span>`fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl<T> FutureExt for AlwaysReady<T, F>`

##### `impl<F> IntoFuture for AlwaysReady<T, F>`

- <span id="alwaysready-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="alwaysready-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="alwaysready-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for AlwaysReady<T, F>`

- <span id="alwaysready-tryfuture-type-ok"></span>`type Ok = T`

- <span id="alwaysready-tryfuture-type-error"></span>`type Error = E`

- <span id="alwaysready-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for AlwaysReady<T, F>`

##### `impl<T, F: Fn() -> T> Unpin for AlwaysReady<T, F>`

## Functions

### `always_ready`

```rust
fn always_ready<T, F: Fn() -> T>(prod: F) -> AlwaysReady<T, F>
```

Creates a future that is always immediately ready with a value.

This is particularly useful in avoiding a heap allocation when an API needs `Box<dyn Future<Output = T>>`,
as [`AlwaysReady`](#alwaysready) does not have to store a boolean for `is_finished`.

# Examples

```rust
futures::executor::block_on(async {
use std::mem::size_of_val;

use futures::future;

let a = future::always_ready(|| 1);
assert_eq!(size_of_val(&a), 0);
assert_eq!(a.await, 1);
assert_eq!(a.await, 1);
});
```

