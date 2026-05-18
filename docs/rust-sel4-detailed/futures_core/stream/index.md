*[futures_core](../index.md) / [stream](index.md)*

---

# Module `stream`

Asynchronous streams.

## Contents

- [Modules](#modules)
  - [`private_try_stream`](#private-try-stream)
  - [`if_alloc`](#if-alloc)
- [Traits](#traits)
  - [`Stream`](#stream)
  - [`FusedStream`](#fusedstream)
  - [`TryStream`](#trystream)
- [Type Aliases](#type-aliases)
  - [`BoxStream`](#boxstream)
  - [`LocalBoxStream`](#localboxstream)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private_try_stream`](#private-try-stream) | mod |  |
| [`if_alloc`](#if-alloc) | mod |  |
| [`Stream`](#stream) | trait | A stream of values produced asynchronously. |
| [`FusedStream`](#fusedstream) | trait | A stream which tracks whether or not the underlying stream should no longer be polled. |
| [`TryStream`](#trystream) | trait | A convenience for streams that return `Result` values that includes a variety of adapters tailored to such futures. |
| [`BoxStream`](#boxstream) | type | An owned dynamically typed [`Stream`] for use in cases where you can't statically type your result or need to add some indirection. |
| [`LocalBoxStream`](#localboxstream) | type | `BoxStream`, but without the `Send` requirement. |

## Modules

- [`private_try_stream`](private_try_stream/index.md)
- [`if_alloc`](if_alloc/index.md)

## Traits

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

## Type Aliases

### `BoxStream<'a, T>`

```rust
type BoxStream<'a, T> = core::pin::Pin<alloc::boxed::Box<dyn Stream<Item = T> + Send>>;
```

An owned dynamically typed [`Stream`](#stream) for use in cases where you can't
statically type your result or need to add some indirection.

This type is often created by the [`boxed`](#boxed) method on `StreamExt`. See its documentation for more.



### `LocalBoxStream<'a, T>`

```rust
type LocalBoxStream<'a, T> = core::pin::Pin<alloc::boxed::Box<dyn Stream<Item = T>>>;
```

`BoxStream`, but without the `Send` requirement.

This type is often created by the `boxed_local` method on `StreamExt`. See its documentation for more.



