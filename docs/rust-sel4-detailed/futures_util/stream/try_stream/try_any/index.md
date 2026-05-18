*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_any](index.md)*

---

# Module `try_any`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryAny`](#tryany) | struct | Future for the [`try_any`](super::TryStreamExt::try_any) method. |

## Structs

### `TryAny<St, Fut, F>`

```rust
struct TryAny<St, Fut, F> {
    stream: St,
    f: F,
    done: bool,
    future: Option<Fut>,
}
```

Future for the [`try_any`](super::TryStreamExt::try_any) method.

#### Implementations

- <span id="tryany-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryAny<St, Fut, F>`

- <span id="tryany-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryAny<St, Fut, F>`

- <span id="tryany-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryAny<St, Fut, F>`

- <span id="tryany-future-type-output"></span>`type Output = Result<bool, <St as TryStream>::Error>`

- <span id="tryany-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<bool, <St as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl FutureExt for TryAny<St, Fut, F>`

##### `impl<F> IntoFuture for TryAny<St, Fut, F>`

- <span id="tryany-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryany-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryany-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryAny<St, Fut, F>`

- <span id="tryany-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryany-tryfuture-type-error"></span>`type Error = E`

- <span id="tryany-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryAny<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryAny<St, Fut, F>`

