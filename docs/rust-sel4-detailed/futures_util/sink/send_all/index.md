*[futures_util](../../index.md) / [sink](../index.md) / [send_all](index.md)*

---

# Module `send_all`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SendAll`](#sendall) | struct | Future for the [`send_all`](super::SinkExt::send_all) method. |

## Structs

### `SendAll<'a, Si, St>`

```rust
struct SendAll<'a, Si, St>
where
    Si: ?Sized,
    St: ?Sized + TryStream {
    sink: &'a mut Si,
    stream: crate::stream::Fuse<&'a mut St>,
    buffered: Option<<St as >::Ok>,
}
```

Future for the [`send_all`](super::SinkExt::send_all) method.

#### Implementations

- <span id="sendall-new"></span>`fn new(sink: &'a mut Si, stream: &'a mut St) -> Self`

- <span id="sendall-try-start-send"></span>`fn try_start_send(&mut self, cx: &mut Context<'_>, item: <St as >::Ok) -> Poll<Result<(), <Si as >::Error>>` — [`Context`](../../task/index.md#context), [`TryStream`](../../stream/index.md#trystream), [`Poll`](../../task/index.md#poll)

#### Trait Implementations

##### `impl<Si, St> Debug for SendAll<'_, Si, St>`

- <span id="sendall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Si, St> Future for SendAll<'_, Si, St>`

- <span id="sendall-future-type-output"></span>`type Output = Result<(), Error>`

- <span id="sendall-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl FutureExt for SendAll<'a, Si, St>`

##### `impl IntoFuture for SendAll<'a, Si, St>`

- <span id="sendall-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="sendall-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="sendall-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl TryFuture for SendAll<'a, Si, St>`

- <span id="sendall-tryfuture-type-ok"></span>`type Ok = T`

- <span id="sendall-tryfuture-type-error"></span>`type Error = E`

- <span id="sendall-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl TryFutureExt for SendAll<'a, Si, St>`

##### `impl<Si, St> Unpin for SendAll<'_, Si, St>`

