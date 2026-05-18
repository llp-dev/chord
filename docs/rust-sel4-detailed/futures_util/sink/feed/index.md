*[futures_util](../../index.md) / [sink](../index.md) / [feed](index.md)*

---

# Module `feed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Feed`](#feed) | struct | Future for the [`feed`](super::SinkExt::feed) method. |

## Structs

### `Feed<'a, Si: ?Sized, Item>`

```rust
struct Feed<'a, Si: ?Sized, Item> {
    sink: &'a mut Si,
    item: Option<Item>,
}
```

Future for the [`feed`](super::SinkExt::feed) method.

#### Implementations

- <span id="feed-new"></span>`fn new(sink: &'a mut Si, item: Item) -> Self`

- <span id="feed-sink-pin-mut"></span>`fn sink_pin_mut(&mut self) -> Pin<&mut Si>`

- <span id="feed-is-item-pending"></span>`fn is_item_pending(&self) -> bool`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Feed<'a, Si, Item>`

- <span id="feed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Feed<'_, Si, Item>`

- <span id="feed-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="feed-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for Feed<'a, Si, Item>`

##### `impl IntoFuture for Feed<'a, Si, Item>`

- <span id="feed-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="feed-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="feed-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Feed<'a, Si, Item>`

- <span id="feed-tryfuture-type-ok"></span>`type Ok = T`

- <span id="feed-tryfuture-type-error"></span>`type Error = E`

- <span id="feed-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for Feed<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Feed<'_, Si, Item>`

