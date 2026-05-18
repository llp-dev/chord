*[futures_util](../../../index.md) / [future](../../index.md) / [future](../index.md) / [fuse](index.md)*

---

# Module `fuse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Fuse`](#fuse) | struct | Future for the [`fuse`](super::FutureExt::fuse) method. |

## Structs

### `Fuse<Fut>`

```rust
struct Fuse<Fut> {
    inner: Option<Fut>,
}
```

Future for the [`fuse`](super::FutureExt::fuse) method.

#### Implementations

- <span id="fuse-new"></span>`fn new(f: Fut) -> Self`

#### Trait Implementations

##### `impl<Fut: fmt::Debug> Debug for Fuse<Fut>`

- <span id="fuse-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> FusedFuture for Fuse<Fut>`

- <span id="fuse-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Future for Fuse<Fut>`

- <span id="fuse-future-type-output"></span>`type Output = <Fut as Future>::Output`

- <span id="fuse-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Fut as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl FutureExt for Fuse<Fut>`

##### `impl IntoFuture for Fuse<Fut>`

- <span id="fuse-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="fuse-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="fuse-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Fuse<Fut>`

- <span id="fuse-tryfuture-type-ok"></span>`type Ok = T`

- <span id="fuse-tryfuture-type-error"></span>`type Error = E`

- <span id="fuse-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../index.md#future)

##### `impl<Fut> TryFutureExt for Fuse<Fut>`

##### `impl<Fut> Unpin for Fuse<Fut>`

