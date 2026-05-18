*[futures_util](../../../index.md) / [stream](../../index.md) / [stream](../index.md) / [peek](index.md)*

---

# Module `peek`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Peekable`](#peekable) | struct | A `Stream` that implements a `peek` method. |
| [`Peek`](#peek) | struct | Future for the [`Peekable::peek`](self::Peekable::peek) method. |
| [`PeekMut`](#peekmut) | struct | Future for the [`Peekable::peek_mut`](self::Peekable::peek_mut) method. |
| [`NextIf`](#nextif) | struct | Future for the [`Peekable::next_if`](self::Peekable::next_if) method. |
| [`NextIfEq`](#nextifeq) | struct | Future for the [`Peekable::next_if_eq`](self::Peekable::next_if_eq) method. |
| [`NextIfEqFn`](#nextifeqfn) | struct |  |

## Structs

### `Peekable<St: Stream>`

```rust
struct Peekable<St: Stream> {
    stream: crate::stream::Fuse<St>,
    peeked: Option<<St as >::Item>,
}
```

A `Stream` that implements a `peek` method.

The `peek` method can be used to retrieve a reference
to the next `Stream::Item` if available. A subsequent
call to `poll` will return the owned item.

#### Implementations

- <span id="peekable-new"></span>`fn new(stream: St) -> Self`

- <span id="peekable-get-ref"></span>`fn get_ref(&self) -> &St`

  Acquires a reference to the underlying sink or stream that this combinator is

  pulling from.

- <span id="peekable-get-mut"></span>`fn get_mut(&mut self) -> &mut St`

  Acquires a mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="peekable-get-pin-mut"></span>`fn get_pin_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut St>`

  Acquires a pinned mutable reference to the underlying sink or stream that this

  combinator is pulling from.

  

  Note that care must be taken to avoid tampering with the state of the

  sink or stream which may otherwise confuse this combinator.

- <span id="peekable-into-inner"></span>`fn into_inner(self) -> St`

  Consumes this combinator, returning the underlying sink or stream.

  

  Note that this may discard intermediate state of this combinator, so

  care should be taken to avoid losing resources when this is called.

- <span id="peekable-peek"></span>`fn peek(self: Pin<&mut Self>) -> Peek<'_, St>` — [`Peek`](#peek)

  Produces a future which retrieves a reference to the next item

  in the stream, or `None` if the underlying stream terminates.

- <span id="peekable-poll-peek"></span>`fn poll_peek(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&<St as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

  Peek retrieves a reference to the next item in the stream.

  

  This method polls the underlying stream and return either a reference

  to the next item if the stream is ready or passes through any errors.

- <span id="peekable-peek-mut"></span>`fn peek_mut(self: Pin<&mut Self>) -> PeekMut<'_, St>` — [`PeekMut`](#peekmut)

  Produces a future which retrieves a mutable reference to the next item

  in the stream, or `None` if the underlying stream terminates.

  

  # Examples

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(vec![1, 2, 3]).peekable();

  let mut stream = pin!(stream);

  

  assert_eq!(stream.as_mut().peek_mut().await, Some(&mut 1));

  assert_eq!(stream.as_mut().next().await, Some(1));

  

  // Peek into the stream and modify the value which will be returned next

  if let Some(p) = stream.as_mut().peek_mut().await {

      if *p == 2 {

          *p = 5;

      }

  }

  

  assert_eq!(stream.collect::<Vec<_>>().await, vec![5, 3]);

  });

  ```

- <span id="peekable-poll-peek-mut"></span>`fn poll_peek_mut(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<&mut <St as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

  Peek retrieves a mutable reference to the next item in the stream.

- <span id="peekable-next-if"></span>`fn next_if<F>(self: Pin<&mut Self>, func: F) -> NextIf<'_, St, F>` — [`NextIf`](#nextif)

  Creates a future which will consume and return the next value of this

  stream if a condition is true.

  

  If `func` returns `true` for the next value of this stream, consume and

  return it. Otherwise, return `None`.

  

  # Examples

  

  Consume a number if it's equal to 0.

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(0..5).peekable();

  let mut stream = pin!(stream);

  // The first item of the stream is 0; consume it.

  assert_eq!(stream.as_mut().next_if(|&x| x == 0).await, Some(0));

  // The next item returned is now 1, so `consume` will return `false`.

  assert_eq!(stream.as_mut().next_if(|&x| x == 0).await, None);

  // `next_if` saves the value of the next item if it was not equal to `expected`.

  assert_eq!(stream.next().await, Some(1));

  });

  ```

  

  Consume any number less than 10.

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(1..20).peekable();

  let mut stream = pin!(stream);

  // Consume all numbers less than 10

  while stream.as_mut().next_if(|&x| x < 10).await.is_some() {}

  // The next value returned will be 10

  assert_eq!(stream.next().await, Some(10));

  });

  ```

- <span id="peekable-next-if-eq"></span>`fn next_if_eq<'a, T>(self: Pin<&'a mut Self>, expected: &'a T) -> NextIfEq<'a, St, T>` — [`NextIfEq`](#nextifeq)

  Creates a future which will consume and return the next item if it is

  equal to `expected`.

  

  # Example

  

  Consume a number if it's equal to 0.

  

  ```rust

  futures::executor::block_on(async {

  use core::pin::pin;

  

  use futures::stream;

  use futures::stream::StreamExt;

  

  let stream = stream::iter(0..5).peekable();

  let mut stream = pin!(stream);

  // The first item of the stream is 0; consume it.

  assert_eq!(stream.as_mut().next_if_eq(&0).await, Some(0));

  // The next item returned is now 1, so `consume` will return `false`.

  assert_eq!(stream.as_mut().next_if_eq(&0).await, None);

  // `next_if_eq` saves the value of the next item if it was not equal to `expected`.

  assert_eq!(stream.next().await, Some(1));

  });

  ```

#### Trait Implementations

##### `impl<St: fmt::Debug + Stream> Debug for Peekable<St>`

- <span id="peekable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream> FusedStream for Peekable<St>`

- <span id="peekable-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<S, Item> Sink for Peekable<S>`

- <span id="peekable-sink-type-error"></span>`type Error = <S as Sink>::Error`

- <span id="peekable-sink-poll-ready"></span>`fn poll_ready(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="peekable-sink-start-send"></span>`fn start_send(self: core::pin::Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>` — [`Sink`](../../../sink/index.md#sink)

- <span id="peekable-sink-poll-flush"></span>`fn poll_flush(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

- <span id="peekable-sink-poll-close"></span>`fn poll_close(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), <Self as >::Error>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Sink`](../../../sink/index.md#sink)

##### `impl<Item> SinkExt for Peekable<St>`

##### `impl<S: Stream> Stream for Peekable<S>`

- <span id="peekable-stream-type-item"></span>`type Item = <S as Stream>::Item`

- <span id="peekable-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Stream`](../../index.md#stream)

- <span id="peekable-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl StreamExt for Peekable<St>`

##### `impl TryStream for Peekable<St>`

- <span id="peekable-trystream-type-ok"></span>`type Ok = T`

- <span id="peekable-trystream-type-error"></span>`type Error = E`

- <span id="peekable-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`TryStream`](../../index.md#trystream)

##### `impl TryStreamExt for Peekable<St>`

##### `impl<St: Stream> Unpin for Peekable<St>`

### `Peek<'a, St: Stream>`

```rust
struct Peek<'a, St: Stream> {
    inner: Option<core::pin::Pin<&'a mut Peekable<St>>>,
}
```

Future for the [`Peekable::peek`](self::Peekable::peek) method.

#### Trait Implementations

##### `impl<St> Debug for Peek<'_, St>`

- <span id="peek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream> FusedFuture for Peek<'_, St>`

- <span id="peek-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Future for Peek<'a, St>`

- <span id="peek-future-type-output"></span>`type Output = Option<&'a <St as Stream>::Item>`

- <span id="peek-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for Peek<'a, St>`

##### `impl IntoFuture for Peek<'a, St>`

- <span id="peek-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="peek-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="peek-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream> Unpin for Peek<'a, St>`

### `PeekMut<'a, St: Stream>`

```rust
struct PeekMut<'a, St: Stream> {
    inner: Option<core::pin::Pin<&'a mut Peekable<St>>>,
}
```

Future for the [`Peekable::peek_mut`](self::Peekable::peek_mut) method.

#### Trait Implementations

##### `impl<St> Debug for PeekMut<'_, St>`

- <span id="peekmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St: Stream> FusedFuture for PeekMut<'_, St>`

- <span id="peekmut-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St> Future for PeekMut<'a, St>`

- <span id="peekmut-future-type-output"></span>`type Output = Option<&'a mut <St as Stream>::Item>`

- <span id="peekmut-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for PeekMut<'a, St>`

##### `impl IntoFuture for PeekMut<'a, St>`

- <span id="peekmut-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="peekmut-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="peekmut-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream> Unpin for PeekMut<'a, St>`

### `NextIf<'a, St: Stream, F>`

```rust
struct NextIf<'a, St: Stream, F> {
    inner: Option<(core::pin::Pin<&'a mut Peekable<St>>, F)>,
}
```

Future for the [`Peekable::next_if`](self::Peekable::next_if) method.

#### Trait Implementations

##### `impl<St, F> Debug for NextIf<'_, St, F>`

- <span id="nextif-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, F> FusedFuture for NextIf<'_, St, F>`

- <span id="nextif-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, F> Future for NextIf<'_, St, F>`

- <span id="nextif-future-type-output"></span>`type Output = Option<<St as Stream>::Item>`

- <span id="nextif-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl FutureExt for NextIf<'a, St, F>`

##### `impl<F> IntoFuture for NextIf<'a, St, F>`

- <span id="nextif-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="nextif-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="nextif-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream, F> Unpin for NextIf<'a, St, F>`

### `NextIfEq<'a, St: Stream, T: ?Sized>`

```rust
struct NextIfEq<'a, St: Stream, T: ?Sized> {
    inner: NextIf<'a, St, NextIfEqFn<'a, T, <St as >::Item>>,
}
```

Future for the [`Peekable::next_if_eq`](self::Peekable::next_if_eq) method.

#### Trait Implementations

##### `impl<St, T> Debug for NextIfEq<'_, St, T>`

- <span id="nextifeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<St, T> FusedFuture for NextIfEq<'_, St, T>`

- <span id="nextifeq-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<St, T> Future for NextIfEq<'_, St, T>`

- <span id="nextifeq-future-type-output"></span>`type Output = Option<<St as Stream>::Item>`

- <span id="nextifeq-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../../task/index.md#context), [`Poll`](../../../task/index.md#poll), [`Future`](../../../future/index.md#future)

##### `impl<T> FutureExt for NextIfEq<'a, St, T>`

##### `impl IntoFuture for NextIfEq<'a, St, T>`

- <span id="nextifeq-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="nextifeq-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="nextifeq-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St: Stream, T: ?Sized> Unpin for NextIfEq<'a, St, T>`

### `NextIfEqFn<'a, T: ?Sized, Item>`

```rust
struct NextIfEqFn<'a, T: ?Sized, Item> {
    expected: &'a T,
    _next: core::marker::PhantomData<Item>,
}
```

#### Trait Implementations

##### `impl<T, Item> FnOnce1 for NextIfEqFn<'_, T, Item>`

- <span id="nextifeqfn-fnonce1-type-output"></span>`type Output = bool`

- <span id="nextifeqfn-fnonce1-call-once"></span>`fn call_once(self, next: &Item) -> <Self as >::Output` — [`FnOnce1`](../../../fns/index.md#fnonce1)

