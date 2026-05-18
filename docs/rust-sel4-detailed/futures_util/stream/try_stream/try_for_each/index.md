*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_for_each](index.md)*

---

# Module `try_for_each`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryForEach`](#tryforeach) | struct | Future for the [`try_for_each`](super::TryStreamExt::try_for_each) method. |

## Structs

### `TryForEach<St, Fut, F>`

```rust
struct TryForEach<St, Fut, F> {
    stream: St,
    f: F,
    future: Option<Fut>,
}
```

Future for the [`try_for_each`](super::TryStreamExt::try_for_each) method.

#### Implementations

- <span id="tryforeach-new"></span>`fn new(stream: St, f: F) -> Self`

#### Trait Implementations

##### `impl<St, Fut, F> Debug for TryForEach<St, Fut, F>`

- <span id="tryforeach-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut, F> Future for TryForEach<St, Fut, F>`

- <span id="tryforeach-future-type-output"></span>`type Output = Result<(), <St as TryStream>::Error>`

- <span id="tryforeach-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for TryForEach<St, Fut, F>`

##### `impl<F> IntoFuture for TryForEach<St, Fut, F>`

- <span id="tryforeach-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryforeach-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryforeach-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<F> TryFuture for TryForEach<St, Fut, F>`

- <span id="tryforeach-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryforeach-tryfuture-type-error"></span>`type Error = E`

- <span id="tryforeach-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<Fut> TryFutureExt for TryForEach<St, Fut, F>`

##### `impl<St, Fut, F> Unpin for TryForEach<St, Fut, F>`

