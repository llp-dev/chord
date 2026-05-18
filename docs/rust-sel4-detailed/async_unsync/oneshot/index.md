*[async_unsync](../index.md) / [oneshot](index.md)*

---

# Module `oneshot`

An unsync **oneshot** channel implementation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RecvError`](#recverror) | struct | An error which can occur when receiving on a closed channel. |
| [`OneshotChannel`](#oneshotchannel) | struct | An unsynchronized (`!Sync`), asynchronous oneshot channel. |
| [`SenderRef`](#senderref) | struct | A borrowing handle for sending an element through a split [`OneshotChannel`]. |
| [`ReceiverRef`](#receiverref) | struct | A borrowing handle for receiving elements through a split [`OneshotChannel`]. |
| [`Slot`](#slot) | struct | A shared underlying data structure for the internal state of a [`OneshotChannel`]. |
| [`channel`](#channel) | fn | Creates a new oneshot channel. |

## Structs

### `RecvError`

```rust
struct RecvError;
```

An error which can occur when receiving on a closed channel.

#### Trait Implementations

##### `impl Clone for RecvError`

- <span id="recverror-clone"></span>`fn clone(&self) -> RecvError` — [`RecvError`](#recverror)

##### `impl Copy for RecvError`

##### `impl Debug for RecvError`

- <span id="recverror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RecvError`

##### `impl PartialEq for RecvError`

- <span id="recverror-partialeq-eq"></span>`fn eq(&self, other: &RecvError) -> bool` — [`RecvError`](#recverror)

##### `impl StructuralPartialEq for RecvError`

### `OneshotChannel<T>`

```rust
struct OneshotChannel<T>(core::cell::UnsafeCell<Slot<T>>);
```

An unsynchronized (`!Sync`), asynchronous oneshot channel.

This is useful for asynchronously handing a single value from one future to
another.

#### Implementations

- <span id="oneshotchannel-split"></span>`fn split(&mut self) -> (SenderRef<'_, T>, ReceiverRef<'_, T>)` — [`SenderRef`](#senderref), [`ReceiverRef`](#receiverref)

  Splits the channel into borrowing [`SenderRef`](#senderref) and [`ReceiverRef`](#receiverref)

  handles.

### `SenderRef<'a, T>`

```rust
struct SenderRef<'a, T> {
    slot: &'a core::cell::UnsafeCell<Slot<T>>,
}
```

A borrowing handle for sending an element through a split
[`OneshotChannel`](#oneshotchannel).

#### Implementations

- <span id="senderref-is-closed"></span>`fn is_closed(&self) -> bool`

  Returns `true` if the channel has been closed.

- <span id="senderref-poll-closed"></span>`fn poll_closed(&mut self, cx: &mut Context<'_>) -> Poll<()>`

  Polls the channel, resolving if the channel has been closed.

- <span id="senderref-closed"></span>`async fn closed(&mut self)`

  Resolves when the channel is closed.

- <span id="senderref-send"></span>`fn send(self, value: T) -> Result<(), SendError<T>>` — [`SendError`](../index.md#senderror)

  Sends a value through the channel.

  

  # Errors

  

  Fails, if the channel is closed.

#### Trait Implementations

##### `impl<T> Debug for SenderRef<'_, T>`

- <span id="senderref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for SenderRef<'_, T>`

- <span id="senderref-drop"></span>`fn drop(&mut self)`

### `ReceiverRef<'a, T>`

```rust
struct ReceiverRef<'a, T> {
    slot: &'a core::cell::UnsafeCell<Slot<T>>,
}
```

A borrowing handle for receiving elements through a split
[`OneshotChannel`](#oneshotchannel).

# Note



This receiver implements [`Future`](../../futures/prelude/index.md) and can be awaited directly:

```rust
async fn example_receiver_ref() {
let mut chan = async_unsync::oneshot::channel();
let (tx, rx) = chan.split();
tx.send(()).unwrap();
let _ = rx.await;
}
```

#### Implementations

- <span id="receiverref-is-closed"></span>`fn is_closed(&self) -> bool`

  Returns `true` if the channel has been closed.

- <span id="receiverref-close"></span>`fn close(&mut self)`

  Closes the channel, causing any [`closed`](SenderRef::closed) or

  subsequent [`poll_closed`](SenderRef::poll_closed) calls to resolve and

  any subsequent [`send`s](SenderRef::send) to fail on the corresponding

  [`SenderRef`](#senderref).

- <span id="receiverref-try-recv"></span>`fn try_recv(&mut self) -> Result<T, TryRecvError>` — [`TryRecvError`](../index.md#tryrecverror)

  Receives an element through the channel.

  

  # Errors

  

  Fails, if the channel is [empty](TryRecvError::Empty) or

  [disconnected](TryRecvError::Disconnected).

#### Trait Implementations

##### `impl<T> Debug for ReceiverRef<'_, T>`

- <span id="receiverref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for ReceiverRef<'_, T>`

- <span id="receiverref-drop"></span>`fn drop(&mut self)`

##### `impl<T> Future for ReceiverRef<'_, T>`

- <span id="receiverref-future-type-output"></span>`type Output = Result<T, RecvError>`

- <span id="receiverref-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl IntoFuture for ReceiverRef<'a, T>`

- <span id="receiverref-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="receiverref-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="receiverref-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

### `Slot<T>`

```rust
struct Slot<T> {
    value: Option<T>,
    recv_waker: Option<core::task::Waker>,
    close_waker: Option<core::task::Waker>,
    closed: bool,
}
```

A shared underlying data structure for the internal state of a
[`OneshotChannel`](#oneshotchannel).

#### Implementations

- <span id="slot-send"></span>`fn send(&mut self, value: T) -> Result<(), SendError<T>>` — [`SendError`](../index.md#senderror)

- <span id="slot-close-and-wake"></span>`fn close_and_wake(&mut self)`

- <span id="slot-poll-closed"></span>`fn poll_closed(&mut self, cx: &mut Context<'_>) -> Poll<()>`

- <span id="slot-try-recv"></span>`fn try_recv(&mut self) -> Result<T, TryRecvError>` — [`TryRecvError`](../index.md#tryrecverror)

- <span id="slot-poll-recv"></span>`fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Result<T, RecvError>>` — [`RecvError`](#recverror)

## Functions

### `channel`

```rust
const fn channel<T>() -> OneshotChannel<T>
```

Creates a new oneshot channel.

