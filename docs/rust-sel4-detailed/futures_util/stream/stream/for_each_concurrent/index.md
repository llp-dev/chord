*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [for_each_concurrent](index.md)*

---

# Module `for_each_concurrent`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ForEachConcurrent`](#foreachconcurrent) | struct | Future for the [`for_each_concurrent`](super::StreamExt::for_each_concurrent) method. |

## Structs

### `ForEachConcurrent<St, Fut, F>`

```rust
struct ForEachConcurrent<St, Fut, F> {
    stream: Option<St>,
    f: F,
    futures: crate::stream::FuturesUnordered<Fut>,
    limit: Option<core::num::NonZeroUsize>,
}
```

Future for the [`for_each_concurrent`](super::StreamExt::for_each_concurrent)
method.

#### Implementations

- <span id="foreachconcurrent-new"></span>`fn new(stream: St, limit: Option<usize>, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> FusedFuture for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Fut, F> Future for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-future-type-output"></span>`type Output = ()`

- <span id="foreachconcurrent-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl FutureExt for ForEachConcurrent<St, Fut, F>`

##### `impl<F> IntoFuture for ForEachConcurrent<St, Fut, F>`

- <span id="foreachconcurrent-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="foreachconcurrent-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="foreachconcurrent-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, Fut, F> Unpin for ForEachConcurrent<St, Fut, F>`

