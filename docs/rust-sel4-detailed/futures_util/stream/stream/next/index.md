*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [next](index.md)*

---

# Module `next`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Next`](#next) | struct | Future for the [`next`](super::StreamExt::next) method. |

## Structs

### `Next<'a, St: ?Sized>`

```rust
struct Next<'a, St: ?Sized> {
    stream: &'a mut St,
}
```

Future for the [`next`](super::StreamExt::next) method.

#### Implementations

- <span id="next-new"></span>`fn new(stream: &'a mut St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug + ?Sized> Debug for Next<'a, St>`

- <span id="next-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: ?Sized + FusedStream + Unpin> FusedFuture for Next<'_, St>`

- <span id="next-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: ?Sized + Stream + Unpin> Future for Next<'_, St>`

- <span id="next-future-type-output"></span>`type Output = Option<<St as Stream>::Item>`

- <span id="next-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for Next<'a, St>`

##### `impl IntoFuture for Next<'a, St>`

- <span id="next-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="next-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="next-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: ?Sized + Unpin> Unpin for Next<'_, St>`

