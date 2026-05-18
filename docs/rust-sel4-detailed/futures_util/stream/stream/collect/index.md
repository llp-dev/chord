*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [collect](index.md)*

---

# Module `collect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Collect`](#collect) | struct | Future for the [`collect`](super::StreamExt::collect) method. |

## Structs

### `Collect<St, C>`

```rust
struct Collect<St, C> {
    stream: St,
    collection: C,
}
```

Future for the [`collect`](super::StreamExt::collect) method.

#### Implementations

- <span id="collect-finish"></span>`fn finish(self: Pin<&mut Self>) -> C`

- <span id="collect-new"></span>`fn new(stream: St) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, C: fmt::Debug> Debug for Collect<St, C>`

- <span id="collect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, C> FusedFuture for Collect<St, C>`

- <span id="collect-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, C> Future for Collect<St, C>`

- <span id="collect-future-type-output"></span>`type Output = C`

- <span id="collect-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<C>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll)

##### `impl FutureExt for Collect<St, C>`

##### `impl IntoFuture for Collect<St, C>`

- <span id="collect-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="collect-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="collect-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St, C> Unpin for Collect<St, C>`

