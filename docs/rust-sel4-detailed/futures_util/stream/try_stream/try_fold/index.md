*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_fold](index.md)*

---

# Module `try_fold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFold`](#tryfold) | struct | Future for the [`try_fold`](super::TryStreamExt::try_fold) method. |

## Structs

### `TryFold<St, Fut, T, F>`

```rust
struct TryFold<St, Fut, T, F> {
    stream: St,
    f: F,
    accum: Option<T>,
    future: Option<Fut>,
}
```

Future for the [`try_fold`](super::TryStreamExt::try_fold) method.

#### Implementations

- <span id="tryfold-new"></span>`fn new(stream: St, f: F, t: T) -> Self`

#### Trait Implementations

##### `impl<St, Fut, T, F> Debug for TryFold<St, Fut, T, F>`

- <span id="tryfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, T, F> FusedFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, T, F> Future for TryFold<St, Fut, T, F>`

- <span id="tryfold-future-type-output"></span>`type Output = Result<T, <St as TryStream>::Error>`

- <span id="tryfold-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<T> FutureExt for TryFold<St, Fut, T, F>`

##### `impl<F> IntoFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryfold-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryfold-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F, T> TryFuture for TryFold<St, Fut, T, F>`

- <span id="tryfold-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryfold-tryfuture-type-error"></span>`type Error = E`

- <span id="tryfold-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryFold<St, Fut, T, F>`

##### `impl<St, Fut, T, F> Unpin for TryFold<St, Fut, T, F>`

