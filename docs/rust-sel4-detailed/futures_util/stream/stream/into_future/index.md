*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [into_future](index.md)*

---

# Module `into_future`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StreamFuture`](#streamfuture) | struct | Future for the [`into_future`](super::StreamExt::into_future) method. |

## Structs

### `StreamFuture<St>`

```rust
struct StreamFuture<St> {
    stream: Option<St>,
}
```

Future for the [`into_future`](super::StreamExt::into_future) method.

#### Implementations

- <span id="streamfuture-new"></span>`fn new(stream: St) -> Self`

- <span id="streamfuture-get-ref"></span>`fn get_ref(&self) -> Option<&St>`

  Acquires a reference to the underlying stream that this combinator is

  pulling from.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

- <span id="streamfuture-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut St>`

  Acquires a mutable reference to the underlying stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

- <span id="streamfuture-get-pin-mut"></span>`fn get_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut St>>`

  Acquires a pinned mutable reference to the underlying stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  stream which may otherwise confuse this combinator.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

- <span id="streamfuture-into-inner"></span>`fn into_inner(self) -> Option<St>`

  Consumes this combinator, returning the underlying stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

  

  This method returns an `Option` to account for the fact that `StreamFuture`'s

  implementation of `Future::poll` consumes the underlying stream during polling

  in order to return it to the caller of `Future::poll` if the stream yielded

  an element.

#### Trait Implementations

##### `impl<St: fmt::Debug> Debug for StreamFuture<St>`

- <span id="streamfuture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream + Unpin> FusedFuture for StreamFuture<St>`

- <span id="streamfuture-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St: Stream + Unpin> Future for StreamFuture<St>`

- <span id="streamfuture-future-type-output"></span>`type Output = (Option<<St as Stream>::Item>, St)`

- <span id="streamfuture-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for StreamFuture<St>`

##### `impl IntoFuture for StreamFuture<St>`

- <span id="streamfuture-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="streamfuture-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="streamfuture-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

