# Crate `async_unsync`

**async-unsync** - Asynchronous channels for single-threaded use.

This crate provides asynchronous but unsynchronized (`!Sync`) alternatives
to [`tokio::sync::mpsc`][1] channel types with almost identical APIs.

Using synchronized data-structures in context that are statically known to
always execute on a single thread has non-trivial overhead.
The specialized (and much simpler) implementations in this library are
primarily intended for use in high-performance async code utilizing thread
local tasks and `!Send` futures.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`oneshot`](#oneshot) | mod | An unsync **oneshot** channel implementation. |
| [`semaphore`](#semaphore) | mod | A simple asynchronous semaphore for limiting and sequencing access to arbitrary shared resources. |
| [`error`](#error) | mod | Error types for fallible channel operations. |
| [`SendError`](#senderror) | struct | An error which can occur when sending a value through a closed channel. |
| [`TryRecvError`](#tryrecverror) | enum | An error which can occur when receiving on a closed or empty channel. |
| [`TrySendError`](#trysenderror) | enum | An error which can occur when sending a value through a closed or full channel. |

## Modules

- [`oneshot`](oneshot/index.md) — An unsync **oneshot** channel implementation.
- [`semaphore`](semaphore/index.md) — A simple asynchronous semaphore for limiting and sequencing access
- [`error`](error/index.md) — Error types for fallible channel operations.

## Structs

### `SendError<T>`

```rust
struct SendError<T>(T);
```

An error which can occur when sending a value through a closed channel.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for SendError<T>`

- <span id="senderror-clone"></span>`fn clone(&self) -> SendError<T>` — [`SendError`](#senderror)

##### `impl<T: marker::Copy> Copy for SendError<T>`

##### `impl<T> Debug for SendError<T>`

- <span id="senderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Display for SendError<T>`

- <span id="senderror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for SendError<T>`

##### `impl<T: cmp::PartialEq> PartialEq for SendError<T>`

- <span id="senderror-partialeq-eq"></span>`fn eq(&self, other: &SendError<T>) -> bool` — [`SendError`](#senderror)

##### `impl<T> StructuralPartialEq for SendError<T>`

## Enums

### `TryRecvError`

```rust
enum TryRecvError {
    Empty,
    Disconnected,
}
```

An error which can occur when receiving on a closed or empty channel.

#### Variants

- **`Empty`**

  This **channel** is currently empty, but the **Sender**(s) have not yet
  disconnected, so data may yet become available.

- **`Disconnected`**

  The **channel**’s sending half has become disconnected, and there will
  never be any more data received on it.

#### Implementations

- <span id="tryrecverror-is-empty"></span>`fn is_empty(self) -> bool`

  Returns `true` if the error is [`TryRecvError::Empty`](#tryrecverrorempty).

- <span id="tryrecverror-is-disconnected"></span>`fn is_disconnected(self) -> bool`

  Returns `true` if the error is [`TryRecvError::Disconnected`](#tryrecverrordisconnected).

#### Trait Implementations

##### `impl Clone for TryRecvError`

- <span id="tryrecverror-clone"></span>`fn clone(&self) -> TryRecvError` — [`TryRecvError`](#tryrecverror)

##### `impl Copy for TryRecvError`

##### `impl Debug for TryRecvError`

- <span id="tryrecverror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryRecvError`

- <span id="tryrecverror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryRecvError`

##### `impl PartialEq for TryRecvError`

- <span id="tryrecverror-partialeq-eq"></span>`fn eq(&self, other: &TryRecvError) -> bool` — [`TryRecvError`](#tryrecverror)

##### `impl StructuralPartialEq for TryRecvError`

### `TrySendError<T>`

```rust
enum TrySendError<T> {
    Full(T),
    Closed(T),
}
```

An error which can occur when sending a value through a closed or full
channel.

#### Variants

- **`Full`**

  This **channel** is currently full.

- **`Closed`**

  This **channel**'s receiving half has been explicitly disconnected or
  dropped and the channel was closed.

#### Implementations

- <span id="trysenderror-is-full"></span>`fn is_full(&self) -> bool`

  Returns `true` if the error is [`TrySendError::Full`](#trysenderrorfull).

- <span id="trysenderror-is-closed"></span>`fn is_closed(&self) -> bool`

  Returns `true` if the error is [`TrySendError::Closed`](#trysenderrorclosed).

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for TrySendError<T>`

- <span id="trysenderror-clone"></span>`fn clone(&self) -> TrySendError<T>` — [`TrySendError`](#trysenderror)

##### `impl<T: marker::Copy> Copy for TrySendError<T>`

##### `impl<T> Debug for TrySendError<T>`

- <span id="trysenderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Display for TrySendError<T>`

- <span id="trysenderror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for TrySendError<T>`

##### `impl<T: cmp::PartialEq> PartialEq for TrySendError<T>`

- <span id="trysenderror-partialeq-eq"></span>`fn eq(&self, other: &TrySendError<T>) -> bool` — [`TrySendError`](#trysenderror)

##### `impl<T> StructuralPartialEq for TrySendError<T>`

