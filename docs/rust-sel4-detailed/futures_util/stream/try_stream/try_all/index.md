*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_all](index.md)*

---

# Module `try_all`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryAll`](#tryall) | struct | Future for the [`try_all`](super::TryStreamExt::try_all) method. |

## Structs

### `TryAll<St, Fut, F>`

```rust
struct TryAll<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`try_all`](super::TryStreamExt::try_all) method.

#### Implementations

- <span id="tryall-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryAll<St, Fut, F>`

- <span id="tryall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryAll<St, Fut, F>`

- <span id="tryall-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryAll<St, Fut, F>`

- <span id="tryall-future-type-output"></span>`type Output = Result<bool, <St as TryStream>::Error>`

- <span id="tryall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<bool, <St as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl FutureExt for TryAll<St, Fut, F>`

##### `impl<F> IntoFuture for TryAll<St, Fut, F>`

- <span id="tryall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryAll<St, Fut, F>`

- <span id="tryall-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryall-tryfuture-type-error"></span>`type Error = E`

- <span id="tryall-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryAll<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryAll<St, Fut, F>`

