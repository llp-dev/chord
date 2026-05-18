*[futures_util](../../index.md) / [stream](../index.md) / [futures_ordered](index.md)*

---

# Module `futures_ordered`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OrderWrapper`](#orderwrapper) | struct |  |
| [`FuturesOrdered`](#futuresordered) | struct | An unbounded queue of futures. |

## Structs

### `OrderWrapper<T>`

```rust
struct OrderWrapper<T> {
    data: T,
    index: i64,
}
```

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for OrderWrapper<T>`

- <span id="orderwrapper-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Eq for OrderWrapper<T>`

##### `impl<T> Future for OrderWrapper<T>`

- <span id="orderwrapper-future-type-output"></span>`type Output = OrderWrapper<<T as Future>::Output>`

- <span id="orderwrapper-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../../future/index.md#future)

##### `impl<T> FutureExt for OrderWrapper<T>`

##### `impl IntoFuture for OrderWrapper<T>`

- <span id="orderwrapper-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="orderwrapper-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="orderwrapper-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> Ord for OrderWrapper<T>`

- <span id="orderwrapper-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T> PartialEq for OrderWrapper<T>`

- <span id="orderwrapper-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> PartialOrd for OrderWrapper<T>`

- <span id="orderwrapper-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Unpin for OrderWrapper<T>`

### `FuturesOrdered<T: Future>`

```rust
struct FuturesOrdered<T: Future> {
    in_progress_queue: crate::stream::FuturesUnordered<OrderWrapper<T>>,
    queued_outputs: alloc::collections::binary_heap::BinaryHeap<OrderWrapper<<T as >::Output>>,
    next_incoming_index: i64,
    next_outgoing_index: i64,
}
```

An unbounded queue of futures.

This "combinator" is similar to [`FuturesUnordered`](../futures_unordered/index.md), but it imposes a FIFO
order on top of the set of futures. While futures in the set will race to
completion in parallel, results will only be returned in the order their
originating futures were added to the queue.

Futures are pushed into this queue and their realized values are yielded in
order. This structure is optimized to manage a large number of futures.
Futures managed by [`FuturesOrdered`](#futuresordered) will only be polled when they generate
notifications. This reduces the required amount of work needed to coordinate
large numbers of futures.

When a [`FuturesOrdered`](#futuresordered) is first created, it does not contain any futures.
Calling [`poll_next`](FuturesOrdered::poll_next) in this state will result
in [`Poll::Ready(None)`](Poll::Ready) to be returned. Futures are submitted
to the queue using [`push_back`](FuturesOrdered::push_back) (or
[`push_front`](FuturesOrdered::push_front)); however, the future will
**not** be polled at this point. [`FuturesOrdered`](#futuresordered) will only poll managed
futures when `FuturesOrdered::poll_next` is called. As such, it
is important to call [`poll_next`](FuturesOrdered::poll_next) after pushing
new futures.

If `FuturesOrdered::poll_next` returns [`Poll::Ready(None)`](Poll::Ready)
this means that the queue is currently not managing any futures. A future
may be submitted to the queue at a later time. At that point, a call to
`FuturesOrdered::poll_next` will either return the future's resolved value
**or** `Poll::Pending` if the future has not yet completed. When
multiple futures are submitted to the queue, `FuturesOrdered::poll_next`
will return `Poll::Pending` until the first future completes, even if
some of the later futures have already completed.

Note that you can create a ready-made [`FuturesOrdered`](#futuresordered) via the
[`collect`](Iterator::collect) method, or you can start with an empty queue
with the `FuturesOrdered::new` constructor.

This type is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

#### Implementations

- <span id="futuresordered-new"></span>`fn new() -> Self`

  Constructs a new, empty `FuturesOrdered`

  

  The returned [`FuturesOrdered`](#futuresordered) does not contain any futures and, in

  this state, `FuturesOrdered::poll_next` will return

  [`Poll::Ready(None)`](Poll::Ready).

- <span id="futuresordered-len"></span>`fn len(&self) -> usize`

  Returns the number of futures contained in the queue.

  

  This represents the total number of in-flight futures, both

  those currently processing and those that have completed but

  which are waiting for earlier futures to complete.

- <span id="futuresordered-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the queue contains no futures

- <span id="futuresordered-push"></span>`fn push(&mut self, future: Fut)`

  Push a future into the queue.

  

  This function submits the given future to the internal set for managing.

  This function will not call [`poll`](Future::poll) on the submitted

  future. The caller must ensure that `FuturesOrdered::poll_next` is

  called in order to receive task notifications.

- <span id="futuresordered-push-back"></span>`fn push_back(&mut self, future: Fut)`

  Pushes a future to the back of the queue.

  

  This function submits the given future to the internal set for managing.

  This function will not call [`poll`](Future::poll) on the submitted

  future. The caller must ensure that `FuturesOrdered::poll_next` is

  called in order to receive task notifications.

- <span id="futuresordered-push-front"></span>`fn push_front(&mut self, future: Fut)`

  Pushes a future to the front of the queue.

  

  This function submits the given future to the internal set for managing.

  This function will not call [`poll`](Future::poll) on the submitted

  future. The caller must ensure that `FuturesOrdered::poll_next` is

  called in order to receive task notifications. This future will be

  the next future to be returned complete.

- <span id="futuresordered-clear"></span>`fn clear(&mut self)`

  Clear the whole `FuturesOrdered` queue.

  

  This function clears the pending futures and the queued outputs

  to make it fully empty.

#### Trait Implementations

##### `impl<Fut: Future> Debug for FuturesOrdered<Fut>`

- <span id="futuresordered-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut: Future> Default for FuturesOrdered<Fut>`

- <span id="futuresordered-default"></span>`fn default() -> Self`

##### `impl<Fut: Future> Extend for FuturesOrdered<Fut>`

- <span id="futuresordered-extend"></span>`fn extend<I>(&mut self, iter: I)`

##### `impl<Fut: Future> FromIterator for FuturesOrdered<Fut>`

- <span id="futuresordered-fromiterator-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl<Fut: Future> FusedStream for FuturesOrdered<Fut>`

- <span id="futuresordered-fusedstream-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<Fut: Future> Stream for FuturesOrdered<Fut>`

- <span id="futuresordered-stream-type-item"></span>`type Item = <Fut as Future>::Output`

- <span id="futuresordered-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Stream`](../index.md#stream)

- <span id="futuresordered-stream-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> StreamExt for FuturesOrdered<T>`

##### `impl<T> TryStream for FuturesOrdered<T>`

- <span id="futuresordered-trystream-type-ok"></span>`type Ok = T`

- <span id="futuresordered-trystream-type-error"></span>`type Error = E`

- <span id="futuresordered-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`TryStream`](../index.md#trystream)

##### `impl TryStreamExt for FuturesOrdered<T>`

##### `impl<T: Future> Unpin for FuturesOrdered<T>`

