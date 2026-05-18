*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [select_next_some](index.md)*

---

# Module `select_next_some`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SelectNextSome`](#selectnextsome) | struct | Future for the [`select_next_some`](super::StreamExt::select_next_some) method. |

## Structs

### `SelectNextSome<'a, St: ?Sized>`

```rust
struct SelectNextSome<'a, St: ?Sized> {
    stream: &'a mut St,
}
```

Future for the [`select_next_some`](super::StreamExt::select_next_some)
method.

#### Implementations

- <span id="selectnextsome-new"></span>`fn new(stream: &'a mut St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + ?Sized> Debug for SelectNextSome<'a, St>`

- <span id="selectnextsome-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: ?Sized + FusedStream + Unpin> FusedFuture for SelectNextSome<'_, St>`

- <span id="selectnextsome-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: ?Sized + FusedStream + Unpin> Future for SelectNextSome<'_, St>`

- <span id="selectnextsome-future-type-output"></span>`type Output = <St as Stream>::Item`

- <span id="selectnextsome-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for SelectNextSome<'a, St>`

##### `impl IntoFuture for SelectNextSome<'a, St>`

- <span id="selectnextsome-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="selectnextsome-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="selectnextsome-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for SelectNextSome<'a, St>`

- <span id="selectnextsome-tryfuture-type-ok"></span>`type Ok = T`

- <span id="selectnextsome-tryfuture-type-error"></span>`type Error = E`

- <span id="selectnextsome-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl TryFutureExt for SelectNextSome<'a, St>`

