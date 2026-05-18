**futures_channel > oneshot**

# Module: oneshot

## Contents

**Structs**

- [`Canceled`](#canceled) - Error returned from a [`Receiver`] when the corresponding [`Sender`] is
- [`Cancellation`](#cancellation) - A future that resolves when the receiving end of a channel has hung up.
- [`Receiver`](#receiver) - A future for a value that will be provided by another asynchronous task.
- [`Sender`](#sender) - A means of transmitting a single value to another task.

**Functions**

- [`channel`](#channel) - Creates a new one-shot channel for sending a single value across asynchronous tasks.

---

## futures_channel::oneshot::Canceled

*Struct*

Error returned from a [`Receiver`] when the corresponding [`Sender`] is
dropped.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Canceled) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Canceled`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## futures_channel::oneshot::Cancellation

*Struct*

A future that resolves when the receiving end of a channel has hung up.

This is an `.await`-friendly interface around [`poll_canceled`](Sender::poll_canceled).

**Generic Parameters:**
- 'a
- T

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<()>`



## futures_channel::oneshot::Receiver

*Struct*

A future for a value that will be provided by another asynchronous task.

This is created by the [`channel`] function.

**Generic Parameters:**
- T

**Methods:**

- `fn close(self: & mut Self)` - Gracefully close this receiver, preventing any subsequent attempts to
- `fn try_recv(self: & mut Self) -> Result<Option<T>, Canceled>` - Attempts to receive a message outside of the context of a task.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **FusedFuture**
  - `fn is_terminated(self: &Self) -> bool`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<Result<T, Canceled>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## futures_channel::oneshot::Sender

*Struct*

A means of transmitting a single value to another task.

This is created by the [`channel`] function.

**Generic Parameters:**
- T

**Methods:**

- `fn send(self: Self, t: T) -> Result<(), T>` - Completes this oneshot with a successful result.
- `fn poll_canceled(self: & mut Self, cx: & mut Context) -> Poll<()>` - Polls this `Sender` half to detect whether its associated
- `fn cancellation(self: & mut Self) -> Cancellation<T>` - Creates a future that resolves when this `Sender`'s corresponding
- `fn is_canceled(self: &Self) -> bool` - Tests to see whether this `Sender`'s corresponding `Receiver`
- `fn is_connected_to(self: &Self, receiver: &Receiver<T>) -> bool` - Tests to see whether this `Sender` is connected to the given `Receiver`. That is, whether

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## futures_channel::oneshot::channel

*Function*

Creates a new one-shot channel for sending a single value across asynchronous tasks.

The channel works for a spsc (single-producer, single-consumer) scheme.

This function is similar to Rust's channel constructor found in the standard
library. Two halves are returned, the first of which is a `Sender` handle,
used to signal the end of a computation and provide its value. The second
half is a `Receiver` which implements the `Future` trait, resolving to the
value that was given to the `Sender` handle.

Each half can be separately owned and sent across tasks.

# Examples

```
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

```rust
fn channel<T>() -> (Sender<T>, Receiver<T>)
```



