*[futures_util](../../index.md) / [future](../index.md) / [lazy](index.md)*

---

# Module `lazy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Lazy`](#lazy) | struct | Future for the [`lazy`] function. |
| [`lazy`](#lazy) | fn | Creates a new future that allows delayed execution of a closure. |

## Structs

### `Lazy<F>`

```rust
struct Lazy<F> {
    f: Option<F>,
}
```

Future for the [`lazy`](#lazy) function.

#### Trait Implementations

##### `impl<F: fmt::Debug> Debug for Lazy<F>`

- <span id="lazy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F> FusedFuture for Lazy<F>`

- <span id="lazy-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<F> Future for Lazy<F>`

- <span id="lazy-future-type-output"></span>`type Output = R`

- <span id="lazy-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<R>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl FutureExt for Lazy<F>`

##### `impl<F> IntoFuture for Lazy<F>`

- <span id="lazy-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="lazy-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="lazy-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for Lazy<F>`

- <span id="lazy-tryfuture-type-ok"></span>`type Ok = T`

- <span id="lazy-tryfuture-type-error"></span>`type Error = E`

- <span id="lazy-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for Lazy<F>`

##### `impl<F> Unpin for Lazy<F>`

## Functions

### `lazy`

```rust
fn lazy<F, R>(f: F) -> Lazy<F>
where
    F: FnOnce(&mut futures_core::task::Context<'_>) -> R
```

Creates a new future that allows delayed execution of a closure.

The provided closure is only run once the future is polled.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::lazy(|_| 1);
assert_eq!(a.await, 1);

let b = future::lazy(|_| -> i32 {
    panic!("oh no!")
});
drop(b); // closure is never run
});
```

