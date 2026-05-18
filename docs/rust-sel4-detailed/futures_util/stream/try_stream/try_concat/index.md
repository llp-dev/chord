*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_concat](index.md)*

---

# Module `try_concat`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryConcat`](#tryconcat) | struct | Future for the [`try_concat`](super::TryStreamExt::try_concat) method. |

## Structs

### `TryConcat<St: TryStream>`

```rust
struct TryConcat<St: TryStream> {
    stream: St,
    accum: Option<<St as >::Ok>,
}
```

Future for the [`try_concat`](super::TryStreamExt::try_concat) method.

#### Implementations

- <span id="tryconcat-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + TryStream> Debug for TryConcat<St>`

- <span id="tryconcat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St> Future for TryConcat<St>`

- <span id="tryconcat-future-type-output"></span>`type Output = Result<<St as TryStream>::Ok, <St as TryStream>::Error>`

- <span id="tryconcat-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for TryConcat<St>`

##### `impl IntoFuture for TryConcat<St>`

- <span id="tryconcat-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="tryconcat-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="tryconcat-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryConcat<St>`

- <span id="tryconcat-tryfuture-type-ok"></span>`type Ok = T`

- <span id="tryconcat-tryfuture-type-error"></span>`type Error = E`

- <span id="tryconcat-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl TryFutureExt for TryConcat<St>`

##### `impl<St: TryStream> Unpin for TryConcat<St>`

