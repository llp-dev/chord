*[futures_util](../../index.md) / [sink](../index.md) / [close](index.md)*

---

# Module `close`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Close`](#close) | struct | Future for the [`close`](super::SinkExt::close) method. |

## Structs

### `Close<'a, Si: ?Sized, Item>`

```rust
struct Close<'a, Si: ?Sized, Item> {
    sink: &'a mut Si,
    _phantom: core::marker::PhantomData<fn(Item)>,
}
```

Future for the [`close`](super::SinkExt::close) method.

#### Implementations

- <span id="close-new"></span>`fn new(sink: &'a mut Si) -> Self`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Close<'a, Si, Item>`

- <span id="close-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Close<'_, Si, Item>`

- <span id="close-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="close-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for Close<'a, Si, Item>`

##### `impl IntoFuture for Close<'a, Si, Item>`

- <span id="close-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="close-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="close-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Close<'a, Si, Item>`

- <span id="close-tryfuture-type-ok"></span>`type Ok = T`

- <span id="close-tryfuture-type-error"></span>`type Error = E`

- <span id="close-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for Close<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Close<'_, Si, Item>`

