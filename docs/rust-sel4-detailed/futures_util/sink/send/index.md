*[futures_util](../../index.md) / [sink](../index.md) / [send](index.md)*

---

# Module `send`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Send`](#send) | struct | Future for the [`send`](super::SinkExt::send) method. |

## Structs

### `Send<'a, Si: ?Sized, Item>`

```rust
struct Send<'a, Si: ?Sized, Item> {
    feed: super::Feed<'a, Si, Item>,
}
```

Future for the [`send`](super::SinkExt::send) method.

#### Implementations

- <span id="send-new"></span>`fn new(sink: &'a mut Si, item: Item) -> Self`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Send<'a, Si, Item>`

- <span id="send-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Send<'_, Si, Item>`

- <span id="send-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="send-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for Send<'a, Si, Item>`

##### `impl IntoFuture for Send<'a, Si, Item>`

- <span id="send-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="send-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="send-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Send<'a, Si, Item>`

- <span id="send-tryfuture-type-ok"></span>`type Ok = T`

- <span id="send-tryfuture-type-error"></span>`type Error = E`

- <span id="send-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for Send<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Send<'_, Si, Item>`

