*[futures_util](../../../index.md) / [stream](../../index.md) / [try_stream](../index.md) / [try_next](index.md)*

---

# Module `try_next`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryNext`](#trynext) | struct | Future for the [`try_next`](super::TryStreamExt::try_next) method. |

## Structs

### `TryNext<'a, St: ?Sized>`

```rust
struct TryNext<'a, St: ?Sized> {
    stream: &'a mut St,
}
```

Future for the [`try_next`](super::TryStreamExt::try_next) method.

#### Implementations

- <span id="trynext-new"></span>`fn new(stream: &'a mut St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + ?Sized> Debug for TryNext<'a, St>`

- <span id="trynext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: ?Sized + TryStream + Unpin + FusedStream> FusedFuture for TryNext<'_, St>`

- <span id="trynext-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: ?Sized + TryStream + Unpin> Future for TryNext<'_, St>`

- <span id="trynext-future-type-output"></span>`type Output = Result<Option<<St as TryStream>::Ok>, <St as TryStream>::Error>`

- <span id="trynext-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for TryNext<'a, St>`

##### `impl IntoFuture for TryNext<'a, St>`

- <span id="trynext-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="trynext-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="trynext-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for TryNext<'a, St>`

- <span id="trynext-tryfuture-type-ok"></span>`type Ok = T`

- <span id="trynext-tryfuture-type-error"></span>`type Error = E`

- <span id="trynext-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl TryFutureExt for TryNext<'a, St>`

##### `impl<St: ?Sized + Unpin> Unpin for TryNext<'_, St>`

