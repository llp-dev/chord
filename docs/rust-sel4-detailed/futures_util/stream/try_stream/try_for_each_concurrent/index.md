*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_for_each_concurrent](index.md)*

---

# Module `try_for_each_concurrent`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryForEachConcurrent`](#tryforeachconcurrent) | struct | Future for the [`try_for_each_concurrent`](super::TryStreamExt::try_for_each_concurrent) method. |

## Structs

### `TryForEachConcurrent<St, Fut, F>`

```rust
struct TryForEachConcurrent<St, Fut, F> {
    stream: Option<St>,
    f: F,
    futures: crate::stream::FuturesUnordered<Fut>,
    limit: Option<core::num::NonZeroUsize>,
}
```

Future for the
[`try_for_each_concurrent`](super::TryStreamExt::try_for_each_concurrent)
method.

#### Implementations

- <span id="tryforeachconcurrent-new"></span>`fn new(stream: St, limit: Option<usize>, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-future-type-output"></span>`type Output = Result<(), <St as TryStream>::Error>`

- <span id="tryforeachconcurrent-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for TryForEachConcurrent<St, Fut, F>`

##### `impl<F> IntoFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryforeachconcurrent-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryforeachconcurrent-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryForEachConcurrent<St, Fut, F>`

- <span id="tryforeachconcurrent-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryforeachconcurrent-tryfuture-type-error"></span>`type Error = E`

- <span id="tryforeachconcurrent-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryForEachConcurrent<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryForEachConcurrent<St, Fut, F>`

