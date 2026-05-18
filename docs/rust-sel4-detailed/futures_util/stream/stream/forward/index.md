*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [forward](index.md)*

---

# Module `forward`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Forward`](#forward) | struct | Future for the [`forward`](super::StreamExt::forward) method. |

## Structs

### `Forward<St, Si, Item>`

```rust
struct Forward<St, Si, Item> {
    sink: Option<Si>,
    stream: crate::stream::Fuse<St>,
    buffered_item: Option<Item>,
}
```

Future for the [`forward`](super::StreamExt::forward) method.

#### Implementations

- <span id="forward-new"></span>`fn new(stream: St, sink: Si) -> Self`

#### Trait Implementations

##### `impl<St: fmt::Debug, Si: fmt::Debug, Item: fmt::Debug> Debug for Forward<St, Si, Item>`

- <span id="forward-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Si, Item> FusedFuture for Forward<St, Si, Item>`

- <span id="forward-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, Si, Item> Future for Forward<St, Si, Item>`

- <span id="forward-future-type-output"></span>`type Output = Result<(), E>`

- <span id="forward-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for Forward<St, Si, Item>`

##### `impl IntoFuture for Forward<St, Si, Item>`

- <span id="forward-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="forward-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="forward-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Forward<St, Si, Item>`

- <span id="forward-tryfuture-type-ok"></span>`type Ok = T`

- <span id="forward-tryfuture-type-error"></span>`type Error = E`

- <span id="forward-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl TryFutureExt for Forward<St, Si, Item>`

##### `impl<St, Si, Item> Unpin for Forward<St, Si, Item>`

