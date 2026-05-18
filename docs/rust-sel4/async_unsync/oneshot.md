**async_unsync > oneshot**

# Module: oneshot

## Contents

**Structs**

- [`OneshotChannel`](#oneshotchannel) - An unsynchronized (`!Sync`), asynchronous oneshot channel.
- [`ReceiverRef`](#receiverref) - A borrowing handle for receiving elements through a split
- [`RecvError`](#recverror) - An error which can occur when receiving on a closed channel.
- [`SenderRef`](#senderref) - A borrowing handle for sending an element through a split

**Functions**

- [`channel`](#channel) - Creates a new oneshot channel.

---

## async_unsync::oneshot::OneshotChannel

*Struct*

An unsynchronized (`!Sync`), asynchronous oneshot channel.

This is useful for asynchronously handing a single value from one future to
another.

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn split(self: & mut Self) -> (SenderRef<T>, ReceiverRef<T>)` - Splits the channel into borrowing [`SenderRef`] and [`ReceiverRef`]



## async_unsync::oneshot::ReceiverRef

*Struct*

A borrowing handle for receiving elements through a split
[`OneshotChannel`].

# Note



This receiver implements [`Future`] and can be awaited directly:

```
# async fn example_receiver_ref() {
let mut chan = async_unsync::oneshot::channel();
let (tx, rx) = chan.split();
tx.send(()).unwrap();
let _ = rx.await;
# }
```

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn is_closed(self: &Self) -> bool` - Returns `true` if the channel has been closed.
- `fn close(self: & mut Self)` - Closes the channel, causing any [`closed`](SenderRef::closed) or
- `fn try_recv(self: & mut Self) -> Result<T, TryRecvError>` - Receives an element through the channel.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## async_unsync::oneshot::RecvError

*Struct*

An error which can occur when receiving on a closed channel.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RecvError`
- **PartialEq**
  - `fn eq(self: &Self, other: &RecvError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## async_unsync::oneshot::SenderRef

*Struct*

A borrowing handle for sending an element through a split
[`OneshotChannel`].

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn is_closed(self: &Self) -> bool` - Returns `true` if the channel has been closed.
- `fn poll_closed(self: & mut Self, cx: & mut Context) -> Poll<()>` - Polls the channel, resolving if the channel has been closed.
- `fn closed(self: & mut Self)` - Resolves when the channel is closed.
- `fn send(self: Self, value: T) -> Result<(), SendError<T>>` - Sends a value through the channel.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## async_unsync::oneshot::channel

*Function*

Creates a new oneshot channel.

```rust
fn channel<T>() -> OneshotChannel<T>
```



