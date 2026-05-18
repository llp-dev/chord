# Crate `futures_sink`

Asynchronous sinks

This crate contains the `Sink` trait which allows values to be sent
asynchronously.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`if_alloc`](#if-alloc) | mod |  |
| [`Sink`](#sink) | trait | A `Sink` is a value into which other values can be sent, asynchronously. |

## Modules

- [`if_alloc`](if_alloc/index.md)

## Traits

### `Sink<Item>`

```rust
trait Sink<Item> { ... }
```

A `Sink` is a value into which other values can be sent, asynchronously.

Basic examples of sinks include the sending side of:

- Channels
- Sockets
- Pipes

In addition to such "primitive" sinks, it's typical to layer additional
functionality, such as buffering, on top of an existing sink.

Sending to a sink is "asynchronous" in the sense that the value may not be
sent in its entirety immediately. Instead, values are sent in a two-phase
way: first by initiating a send, and then by polling for completion. This
two-phase setup is analogous to buffered writing in synchronous code, where
writes often succeed immediately, but internally are buffered and are
*actually* written only upon flushing.

In addition, the `Sink` may be *full*, in which case it is not even possible
to start the sending process.

As with `Future` and `Stream`, the `Sink` trait is built from a few core
required methods, and a host of default methods for working in a
higher-level way. The `Sink::send_all` combinator is of particular
importance: you can use it to send an entire stream to a sink, which is
the simplest way to ultimately consume a stream.

#### Associated Types

- `type Error`

#### Required Methods

- `fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>`

  Attempts to prepare the `Sink` to receive a value.

- `fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), <Self as >::Error>`

  Begin the process of sending a value to the sink.

- `fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>`

  Flush any remaining output from this sink.

- `fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), <Self as >::Error>>`

  Flush any remaining output and close this sink, if necessary.

#### Implementors

- `&mut S`
- `alloc::boxed::Box<S>`
- `alloc::collections::VecDeque<T>`
- `alloc::vec::Vec<T>`
- `core::pin::Pin<P>`

