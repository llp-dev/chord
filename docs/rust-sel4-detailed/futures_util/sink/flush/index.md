*[futures_util](../../index.md) / [sink](../index.md) / [flush](index.md)*

---

# Module `flush`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flush`](#flush) | struct | Future for the [`flush`](super::SinkExt::flush) method. |

## Structs

### `Flush<'a, Si: ?Sized, Item>`

```rust
struct Flush<'a, Si: ?Sized, Item> {
    sink: &'a mut Si,
    _phantom: core::marker::PhantomData<fn(Item)>,
}
```

Future for the [`flush`](super::SinkExt::flush) method.

#### Implementations

- <span id="flush-new"></span>`fn new(sink: &'a mut Si) -> Self`

#### Trait Implementations

##### `impl<Si: fmt::Debug + ?Sized, Item: fmt::Debug> Debug for Flush<'a, Si, Item>`

- <span id="flush-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si: Sink<Item> + Unpin + ?Sized, Item> Future for Flush<'_, Si, Item>`

- <span id="flush-future-type-output"></span>`type Output = Result<(), <Si as Sink>::Error>`

- <span id="flush-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for Flush<'a, Si, Item>`

##### `impl IntoFuture for Flush<'a, Si, Item>`

- <span id="flush-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="flush-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="flush-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for Flush<'a, Si, Item>`

- <span id="flush-tryfuture-type-ok"></span>`type Ok = T`

- <span id="flush-tryfuture-type-error"></span>`type Error = E`

- <span id="flush-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for Flush<'a, Si, Item>`

##### `impl<Si: Unpin + ?Sized, Item> Unpin for Flush<'_, Si, Item>`

