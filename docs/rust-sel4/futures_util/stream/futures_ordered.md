**futures_util > stream > futures_ordered**

# Module: stream::futures_ordered

## Contents

**Structs**

- [`FuturesOrdered`](#futuresordered) - An unbounded queue of futures.

---

## futures_util::stream::futures_ordered::FuturesOrdered

*Struct*

An unbounded queue of futures.

This "combinator" is similar to [`FuturesUnordered`], but it imposes a FIFO
order on top of the set of futures. While futures in the set will race to
completion in parallel, results will only be returned in the order their
originating futures were added to the queue.

Futures are pushed into this queue and their realized values are yielded in
order. This structure is optimized to manage a large number of futures.
Futures managed by [`FuturesOrdered`] will only be polled when they generate
notifications. This reduces the required amount of work needed to coordinate
large numbers of futures.

When a [`FuturesOrdered`] is first created, it does not contain any futures.
Calling [`poll_next`](FuturesOrdered::poll_next) in this state will result
in [`Poll::Ready(None)`](Poll::Ready) to be returned. Futures are submitted
to the queue using [`push_back`](FuturesOrdered::push_back) (or
[`push_front`](FuturesOrdered::push_front)); however, the future will
**not** be polled at this point. [`FuturesOrdered`] will only poll managed
futures when [`FuturesOrdered::poll_next`] is called. As such, it
is important to call [`poll_next`](FuturesOrdered::poll_next) after pushing
new futures.

If [`FuturesOrdered::poll_next`] returns [`Poll::Ready(None)`](Poll::Ready)
this means that the queue is currently not managing any futures. A future
may be submitted to the queue at a later time. At that point, a call to
[`FuturesOrdered::poll_next`] will either return the future's resolved value
**or** [`Poll::Pending`] if the future has not yet completed. When
multiple futures are submitted to the queue, [`FuturesOrdered::poll_next`]
will return [`Poll::Pending`] until the first future completes, even if
some of the later futures have already completed.

Note that you can create a ready-made [`FuturesOrdered`] via the
[`collect`](Iterator::collect) method, or you can start with an empty queue
with the [`FuturesOrdered::new`] constructor.

This type is only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Constructs a new, empty `FuturesOrdered`
- `fn len(self: &Self) -> usize` - Returns the number of futures contained in the queue.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the queue contains no futures
- `fn push(self: & mut Self, future: Fut)` - Push a future into the queue.
- `fn push_back(self: & mut Self, future: Fut)` - Pushes a future to the back of the queue.
- `fn push_front(self: & mut Self, future: Fut)` - Pushes a future to the front of the queue.
- `fn clear(self: & mut Self)` - Clear the whole `FuturesOrdered` queue.

**Trait Implementations:**

- **Stream**
  - `fn poll_next(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Option<<Self as >::Item>>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **FusedStream**
  - `fn is_terminated(self: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **Default**
  - `fn default() -> Self`



