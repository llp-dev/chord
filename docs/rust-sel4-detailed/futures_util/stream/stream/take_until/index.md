*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [take_until](index.md)*

---

# Module `take_until`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TakeUntil`](#takeuntil) | struct | Stream for the [`take_until`](super::StreamExt::take_until) method. |

## Structs

### `TakeUntil<St: Stream, Fut: Future>`

```rust
struct TakeUntil<St: Stream, Fut: Future> {
    stream: St,
    fut: Option<Fut>,
    fut_result: Option<<Fut as >::Output>,
    free: bool,
}
```

Stream for the [`take_until`](super::StreamExt::take_until) method.

#### Implementations

- <span id="takeuntil-new"></span>`fn new(stream: St, fut: Fut) -> Self`

- <span id="takeuntil-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="takeuntil-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takeuntil-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="takeuntil-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="takeuntil-take-future"></span>`fn take_future(&mut self) -> Option<Fut>`

  Extract the stopping future out of the combinator.

  The future is returned only if it isn't resolved yet, ie. if the stream isn't stopped yet.

  Taking out the future means the combinator will be yielding

  elements from the wrapped stream without ever stopping it.

- <span id="takeuntil-take-result"></span>`fn take_result(&mut self) -> Option<<Fut as >::Output>` — [`Future`](../../../future/index.md#future)

  Once the stopping future is resolved, this method can be used

  to extract the value returned by the stopping future.

  

  This may be used to retrieve arbitrary data from the stopping

  future, for example a reason why the stream was stopped.

  

  This method will return `None` if the future isn't resolved yet,

  or if the result was already taken out.

  

  # Examples

  

  ```rust

  futures::executor::block_on(async {

  use futures::future;

  use futures::stream::{self, StreamExt};

  use futures::task::Poll;

  

  let stream = stream::iter(1..=10);

  

  let mut i = 0;

  let stop_fut = future::poll_fn(|_cx| {

      i += 1;

      if i <= 5 {

          Poll::Pending

      } else {

          Poll::Ready("reason")

      }

  });

  

  let mut stream = stream.take_until(stop_fut);

  let _ = stream.by_ref().collect::<Vec<_>>().await;

  

  let result = stream.take_result().unwrap();

  assert_eq!(result, "reason");

  });

  ```

- <span id="takeuntil-is-stopped"></span>`fn is_stopped(&self) -> bool`

  Whether the stream was stopped yet by the stopping future

  being resolved.

#### Trait Implementations

##### `impl<St, Fut> Debug for TakeUntil<St, Fut>`

- <span id="takeuntil-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, Fut> FusedStream for TakeUntil<St, Fut>`

- <span id="takeuntil-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Fut, Item> Sink for TakeUntil<S, Fut>`

- <span id="takeuntil-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="takeuntil-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="takeuntil-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="takeuntil-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="takeuntil-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for TakeUntil<St, Fut>`

##### `impl<St, Fut> Stream for TakeUntil<St, Fut>`

- <span id="takeuntil-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="takeuntil-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<St as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="takeuntil-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for TakeUntil<St, Fut>`

##### `impl TryStream for TakeUntil<St, Fut>`

- <span id="takeuntil-trystream-type-ok"></span>`type Ok = T`

- <span id="takeuntil-trystream-type-error"></span>`type Error = E`

- <span id="takeuntil-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for TakeUntil<St, Fut>`

##### `impl<St: Stream, Fut: Future> Unpin for TakeUntil<St, Fut>`

