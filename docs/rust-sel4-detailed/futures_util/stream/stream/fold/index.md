*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [fold](index.md)*

---

# Module `fold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Fold`](#fold) | struct | Future for the [`fold`](super::StreamExt::fold) method. |

## Structs

### `Fold<St, Fut, T, F>`

```rust
struct Fold<St, Fut, T, F> {
    stream: St,
    f: F,
    accum: Option<T>,
    future: Option<Fut>,
}
```

Future for the [`fold`](super::StreamExt::fold) method.

#### Implementations

- <span id="fold-new"></span>`fn new(stream: St, f: F, t: T) -> Self`

#### Trait Implementations

##### `impl<St, Fut, T, F> Debug for Fold<St, Fut, T, F>`

- <span id="fold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, T, F> FusedFuture for Fold<St, Fut, T, F>`

- <span id="fold-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, T, F> Future for Fold<St, Fut, T, F>`

- <span id="fold-future-type-output"></span>`type Output = T`

- <span id="fold-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl<T> FutureExt for Fold<St, Fut, T, F>`

##### `impl<F> IntoFuture for Fold<St, Fut, T, F>`

- <span id="fold-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="fold-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="fold-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for Fold<St, Fut, T, F>`

- <span id="fold-tryfuture-type-ok"></span>`type Ok = T`

- <span id="fold-tryfuture-type-error"></span>`type Error = E`

- <span id="fold-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<Fut> TryFutureExt for Fold<St, Fut, T, F>`

##### `impl<St, Fut, T, F> Unpin for Fold<St, Fut, T, F>`

