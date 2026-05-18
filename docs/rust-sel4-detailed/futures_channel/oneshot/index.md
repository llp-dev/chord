*[futures_channel](../index.md) / [oneshot](index.md)*

---

# Module `oneshot`

A channel for sending a single message between asynchronous tasks.

This is a single-producer, single-consumer channel.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Receiver`](#receiver) | struct | A future for a value that will be provided by another asynchronous task. |
| [`Sender`](#sender) | struct | A means of transmitting a single value to another task. |
| [`Inner`](#inner) | struct | Internal state of the `Receiver`/`Sender` pair above. |
| [`Cancellation`](#cancellation) | struct | A future that resolves when the receiving end of a channel has hung up. |
| [`Canceled`](#canceled) | struct | Error returned from a [`Receiver`] when the corresponding [`Sender`] is dropped. |
| [`channel`](#channel) | fn | Creates a new one-shot channel for sending a single value across asynchronous tasks. |

## Structs

### `Receiver<T>`

```rust
struct Receiver<T> {
    inner: alloc::sync::Arc<Inner<T>>,
}
```

A future for a value that will be provided by another asynchronous task.

This is created by the [`channel`](#channel) function.

#### Implementations

- <span id="receiver-close"></span>`fn close(&mut self)`

  Gracefully close this receiver, preventing any subsequent attempts to

  send to it.

  

  Any `send` operation which happens after this method returns is

  guaranteed to fail. After calling this method, you can use

  [`Receiver::poll`](core::future::Future::poll) to determine whether a

  message had previously been sent.

- <span id="receiver-try-recv"></span>`fn try_recv(&mut self) -> Result<Option<T>, Canceled>` — [`Canceled`](#canceled)

  Attempts to receive a message outside of the context of a task.

  

  Does not schedule a task wakeup or have any other side effects.

  

  A return value of `None` must be considered immediately stale (out of

  date) unless [`close`](Receiver::close) has been called first.

  

  Returns an error if the sender was dropped.

#### Trait Implementations

##### `impl<T> Debug for Receiver<T>`

- <span id="receiver-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Receiver<T>`

- <span id="receiver-drop"></span>`fn drop(&mut self)`

##### `impl<T> FusedFuture for Receiver<T>`

- <span id="receiver-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Future for Receiver<T>`

- <span id="receiver-future-type-output"></span>`type Output = Result<T, Canceled>`

- <span id="receiver-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<T, Canceled>>` — [`Canceled`](#canceled)

##### `impl IntoFuture for Receiver<T>`

- <span id="receiver-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="receiver-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="receiver-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> TryFuture for Receiver<T>`

- <span id="receiver-tryfuture-type-ok"></span>`type Ok = T`

- <span id="receiver-tryfuture-type-error"></span>`type Error = E`

- <span id="receiver-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>`

##### `impl<T> Unpin for Receiver<T>`

### `Sender<T>`

```rust
struct Sender<T> {
    inner: alloc::sync::Arc<Inner<T>>,
}
```

A means of transmitting a single value to another task.

This is created by the [`channel`](#channel) function.

#### Implementations

- <span id="sender-send"></span>`fn send(self, t: T) -> Result<(), T>`

  Completes this oneshot with a successful result.

  

  This function will consume `self` and indicate to the other end, the

  [`Receiver`](#receiver), that the value provided is the result of the computation

  this represents.

  

  If the value is successfully enqueued for the remote end to receive,

  then `Ok(())` is returned. If the receiving end was dropped before

  this function was called, however, then `Err(t)` is returned.

- <span id="sender-poll-canceled"></span>`fn poll_canceled(&mut self, cx: &mut Context<'_>) -> Poll<()>`

  Polls this `Sender` half to detect whether its associated

  [`Receiver`](#receiver) has been dropped.

  

  # Return values

  

  If `Ready(())` is returned then the associated `Receiver` has been

  dropped, which means any work required for sending should be canceled.

  

  If `Pending` is returned then the associated `Receiver` is still

  alive and may be able to receive a message if sent. The current task,

  however, is scheduled to receive a notification if the corresponding

  `Receiver` goes away.

- <span id="sender-cancellation"></span>`fn cancellation(&mut self) -> Cancellation<'_, T>` — [`Cancellation`](#cancellation)

  Creates a future that resolves when this `Sender`'s corresponding

  [`Receiver`](#receiver) half has hung up.

  

  This is a utility wrapping [`poll_canceled`](Sender::poll_canceled)

  to expose a [`Future`](../../futures/prelude/index.md).

- <span id="sender-is-canceled"></span>`fn is_canceled(&self) -> bool`

  Tests to see whether this `Sender`'s corresponding `Receiver`

  has been dropped.

  

  Unlike [`poll_canceled`](Sender::poll_canceled), this function does not

  enqueue a task for wakeup upon cancellation, but merely reports the

  current state, which may be subject to concurrent modification.

- <span id="sender-is-connected-to"></span>`fn is_connected_to(&self, receiver: &Receiver<T>) -> bool` — [`Receiver`](#receiver)

  Tests to see whether this `Sender` is connected to the given `Receiver`. That is, whether

  they were created by the same call to `channel`.

#### Trait Implementations

##### `impl<T> Debug for Sender<T>`

- <span id="sender-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Sender<T>`

- <span id="sender-drop"></span>`fn drop(&mut self)`

##### `impl<T> Unpin for Sender<T>`

### `Inner<T>`

```rust
struct Inner<T> {
    complete: core::sync::atomic::AtomicBool,
    data: crate::lock::Lock<Option<T>>,
    rx_task: crate::lock::Lock<Option<futures_core::task::Waker>>,
    tx_task: crate::lock::Lock<Option<futures_core::task::Waker>>,
}
```

Internal state of the `Receiver`/`Sender` pair above. This is all used as
the internal synchronization between the two for send/recv operations.

#### Fields

- **`complete`**: `core::sync::atomic::AtomicBool`

  Indicates whether this oneshot is complete yet. This is filled in both
  by `Sender::drop` and by `Receiver::drop`, and both sides interpret it
  appropriately.
  
  For `Receiver`, if this is `true`, then it's guaranteed that `data` is
  unlocked and ready to be inspected.
  
  For `Sender` if this is `true` then the oneshot has gone away and it
  can return ready from `poll_canceled`.

- **`data`**: `crate::lock::Lock<Option<T>>`

  The actual data being transferred as part of this `Receiver`. This is
  filled in by `Sender::complete` and read by `Receiver::poll`.
  
  Note that this is protected by `Lock`, but it is in theory safe to
  replace with an `UnsafeCell` as it's actually protected by `complete`
  above. I wouldn't recommend doing this, however, unless someone is
  supremely confident in the various atomic orderings here and there.

- **`rx_task`**: `crate::lock::Lock<Option<futures_core::task::Waker>>`

  Field to store the task which is blocked in `Receiver::poll`.
  
  This is filled in when a oneshot is polled but not ready yet. Note that
  the `Lock` here, unlike in `data` above, is important to resolve races.
  Both the `Receiver` and the `Sender` halves understand that if they
  can't acquire the lock then some important interference is happening.

- **`tx_task`**: `crate::lock::Lock<Option<futures_core::task::Waker>>`

  Like `rx_task` above, except for the task blocked in
  `Sender::poll_canceled`. Additionally, `Lock` cannot be `UnsafeCell`.

#### Implementations

- <span id="inner-new"></span>`fn new() -> Self`

- <span id="inner-send"></span>`fn send(&self, t: T) -> Result<(), T>`

- <span id="inner-poll-canceled"></span>`fn poll_canceled(&self, cx: &mut Context<'_>) -> Poll<()>`

- <span id="inner-is-canceled"></span>`fn is_canceled(&self) -> bool`

- <span id="inner-drop-tx"></span>`fn drop_tx(&self)`

- <span id="inner-close-rx"></span>`fn close_rx(&self)`

- <span id="inner-try-recv"></span>`fn try_recv(&self) -> Result<Option<T>, Canceled>` — [`Canceled`](#canceled)

- <span id="inner-recv"></span>`fn recv(&self, cx: &mut Context<'_>) -> Poll<Result<T, Canceled>>` — [`Canceled`](#canceled)

- <span id="inner-drop-rx"></span>`fn drop_rx(&self)`

### `Cancellation<'a, T>`

```rust
struct Cancellation<'a, T> {
    inner: &'a mut Sender<T>,
}
```

A future that resolves when the receiving end of a channel has hung up.

This is an `.await`-friendly interface around [`poll_canceled`](Sender::poll_canceled).

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Cancellation<'a, T>`

- <span id="cancellation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Future for Cancellation<'_, T>`

- <span id="cancellation-future-type-output"></span>`type Output = ()`

- <span id="cancellation-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>`

##### `impl IntoFuture for Cancellation<'a, T>`

- <span id="cancellation-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="cancellation-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="cancellation-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

### `Canceled`

```rust
struct Canceled;
```

Error returned from a [`Receiver`](#receiver) when the corresponding [`Sender`](#sender) is
dropped.

#### Trait Implementations

##### `impl Clone for Canceled`

- <span id="canceled-clone"></span>`fn clone(&self) -> Canceled` — [`Canceled`](#canceled)

##### `impl Copy for Canceled`

##### `impl Debug for Canceled`

- <span id="canceled-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Canceled`

- <span id="canceled-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Canceled`

##### `impl PartialEq for Canceled`

- <span id="canceled-partialeq-eq"></span>`fn eq(&self, other: &Canceled) -> bool` — [`Canceled`](#canceled)

##### `impl StructuralPartialEq for Canceled`

##### `impl ToString for Canceled`

- <span id="canceled-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `channel`

```rust
fn channel<T>() -> (Sender<T>, Receiver<T>)
```

Creates a new one-shot channel for sending a single value across asynchronous tasks.

The channel works for a spsc (single-producer, single-consumer) scheme.

This function is similar to Rust's channel constructor found in the standard
library. Two halves are returned, the first of which is a `Sender` handle,
used to signal the end of a computation and provide its value. The second
half is a `Receiver` which implements the `Future` trait, resolving to the
value that was given to the `Sender` handle.

Each half can be separately owned and sent across tasks.

# Examples

```rust
use futures::channel::oneshot;
use std::{thread, time::Duration};

let (sender, receiver) = oneshot::channel::<i32>();

thread::spawn(|| {
    println!("THREAD: sleeping zzz...");
    thread::sleep(Duration::from_millis(1000));
    println!("THREAD: i'm awake! sending.");
    sender.send(3).unwrap();
});

println!("MAIN: doing some useful stuff");

futures::executor::block_on(async {
    println!("MAIN: waiting for msg...");
    println!("MAIN: got: {:?}", receiver.await)
});
```

