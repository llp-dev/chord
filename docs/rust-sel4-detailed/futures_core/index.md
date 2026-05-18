# Crate `futures_core`

Core traits and types for asynchronous operations in Rust.

## Contents

- [Modules](#modules)
  - [`future`](#future)
  - [`stream`](#stream)
  - [`task`](#task)
- [Traits](#traits)
  - [`FusedFuture`](#fusedfuture)
  - [`TryFuture`](#tryfuture)
  - [`FusedStream`](#fusedstream)
  - [`Stream`](#stream)
  - [`TryStream`](#trystream)
- [Functions](#functions)
  - [`Future`](#future)
- [Macros](#macros)
  - [`ready!`](#ready)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`future`](#future) | mod | Futures. |
| [`stream`](#stream) | mod | Asynchronous streams. |
| [`task`](#task) | mod | Task notification. |
| [`FusedFuture`](#fusedfuture) | trait |  |
| [`TryFuture`](#tryfuture) | trait |  |
| [`FusedStream`](#fusedstream) | trait |  |
| [`Stream`](#stream) | trait |  |
| [`TryStream`](#trystream) | trait |  |
| [`Future`](#future) | fn |  |
| [`ready!`](#ready) | macro | Extracts the successful type of `Poll<T>`. |

## Modules

- [`future`](future/index.md) — Futures.
- [`stream`](stream/index.md) — Asynchronous streams.
- [`task`](task/index.md) — Task notification.

## Traits

### `FusedFuture`

```rust
trait FusedFuture: Future { ... }
```

A future which tracks whether or not the underlying future
should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled.
Usually, this state occurs after `poll` (or `try_poll`) returned
`Poll::Ready`. However, `is_terminated` may also return `true` if a future
has become inactive and can no longer make progress and should be ignored
or dropped rather than being `poll`ed again.

#### Required Methods

- `fn is_terminated(&self) -> bool`

  Returns `true` if the underlying future should no longer be polled.

#### Implementors

- `&mut F`
- `alloc::boxed::Box<F>`
- `core::pin::Pin<P>`

### `TryFuture`

```rust
trait TryFuture: Future + private_try_future::Sealed { ... }
```

A convenience for futures that return `Result` values that includes
a variety of adapters tailored to such futures.

#### Associated Types

- `type Ok`

- `type Error`

#### Required Methods

- `fn try_poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<<Self as >::Ok, <Self as >::Error>>`

  Poll this `TryFuture` as if it were a `Future`.

#### Implementors

- `F`

### `FusedStream`

```rust
trait FusedStream: Stream { ... }
```

A stream which tracks whether or not the underlying stream
should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled.
Usually, this state occurs after `poll_next` (or `try_poll_next`) returned
`Poll::Ready(None)`. However, `is_terminated` may also return `true` if a
stream has become inactive and can no longer make progress and should be
ignored or dropped rather than being polled again.

#### Required Methods

- `fn is_terminated(&self) -> bool`

  Returns `true` if the stream should no longer be polled.

#### Implementors

- `&mut F`
- `alloc::boxed::Box<S>`
- `core::pin::Pin<P>`

### `Stream`

```rust
trait Stream { ... }
```

A stream of values produced asynchronously.

If `Future<Output = T>` is an asynchronous version of `T`, then `Stream<Item
= T>` is an asynchronous version of `Iterator<Item = T>`. A stream
represents a sequence of value-producing events that occur asynchronously to
the caller.

The trait is modeled after `Future`, but allows `poll_next` to be called
even after a value has been produced, yielding `None` once the stream has
been fully exhausted.

#### Associated Types

- `type Item`

#### Required Methods

- `fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>`

  Attempt to pull out the next value of this stream, registering the

#### Provided Methods

- `fn size_hint(&self) -> (usize, Option<usize>)`

  Returns the bounds on the remaining length of the stream.

#### Implementors

- `&mut S`
- `alloc::boxed::Box<S>`
- `core::pin::Pin<P>`

### `TryStream`

```rust
trait TryStream: Stream + private_try_stream::Sealed { ... }
```

A convenience for streams that return `Result` values that includes
a variety of adapters tailored to such futures.

#### Associated Types

- `type Ok`

- `type Error`

#### Required Methods

- `fn try_poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<<Self as >::Ok, <Self as >::Error>>>`

  Poll this `TryStream` as if it were a `Stream`.

#### Implementors

- `S`

## Functions

### `Future`

```rust
fn Future(bytes: &[u8]) -> Self
```

## Macros

### `ready!`

Extracts the successful type of `Poll<T>`.

This macro bakes in propagation of `Pending` signals by returning early.

**Note:** Since Rust 1.64, this macro is soft-deprecated in favor of
[`ready!`](core::task::ready) macro in the standard library.

