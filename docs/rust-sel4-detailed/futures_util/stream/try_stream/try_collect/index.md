*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_collect](index.md)*

---

# Module `try_collect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryCollect`](#trycollect) | struct | Future for the [`try_collect`](super::TryStreamExt::try_collect) method. |

## Structs

### `TryCollect<St, C>`

```rust
struct TryCollect<St, C> {
    stream: St,
    items: C,
}
```

Future for the [`try_collect`](super::TryStreamExt::try_collect) method.

#### Implementations

- <span id="trycollect-new"></span>`fn new(s: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, C: fmt::Debug> Debug for TryCollect<St, C>`

- <span id="trycollect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, C> FusedFuture for TryCollect<St, C>`

- <span id="trycollect-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, C> Future for TryCollect<St, C>`

- <span id="trycollect-future-type-output"></span>`type Output = Result<C, <St as TryStream>::Error>`

- <span id="trycollect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for TryCollect<St, C>`

##### `impl IntoFuture for TryCollect<St, C>`

- <span id="trycollect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trycollect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trycollect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryCollect<St, C>`

- <span id="trycollect-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trycollect-tryfuture-type-error"></span>`type Error = E`

- <span id="trycollect-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl TryFutureExt for TryCollect<St, C>`

##### `impl<St, C> Unpin for TryCollect<St, C>`

