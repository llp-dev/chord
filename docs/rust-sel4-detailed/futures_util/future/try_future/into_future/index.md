*[futures_util](../../../index.md) / [future](../../index.md) / [try_future](../index.md) / [into_future](index.md)*

---

# Module `into_future`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoFuture`](#intofuture) | struct | Future for the [`into_future`](super::TryFutureExt::into_future) method. |

## Structs

### `IntoFuture<Fut>`

```rust
struct IntoFuture<Fut> {
    future: Fut,
}
```

Future for the [`into_future`](super::TryFutureExt::into_future) method.

#### Implementations

- <span id="intofuture-new"></span>`fn new(future: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for IntoFuture<Fut>`

- <span id="intofuture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: TryFuture + FusedFuture> FusedFuture for IntoFuture<Fut>`

- <span id="intofuture-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: TryFuture> Future for IntoFuture<Fut>`

- <span id="intofuture-future-type-output"></span>`type Output = Result<<Fut as TryFuture>::Ok, <Fut as TryFuture>::Error>`

- <span id="intofuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl FutureExt for IntoFuture<Fut>`

##### `impl IntoFuture for IntoFuture<Fut>`

- <span id="intofuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="intofuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="intofuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for IntoFuture<Fut>`

- <span id="intofuture-tryfuture-type-ok"></span>`type Ok = T`

- <span id="intofuture-tryfuture-type-error"></span>`type Error = E`

- <span id="intofuture-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl<Fut> TryFutureExt for IntoFuture<Fut>`

##### `impl<Fut> Unpin for IntoFuture<Fut>`

