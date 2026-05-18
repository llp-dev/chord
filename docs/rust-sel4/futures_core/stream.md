**futures_core > stream**

# Module: stream

## Contents

**Traits**

- [`FusedStream`](#fusedstream) - A stream which tracks whether or not the underlying stream
- [`Stream`](#stream) - A stream of values produced asynchronously.
- [`TryStream`](#trystream) - A convenience for streams that return `Result` values that includes

**Type Aliases**

- [`BoxStream`](#boxstream) - An owned dynamically typed [`Stream`] for use in cases where you can't
- [`LocalBoxStream`](#localboxstream) - `BoxStream`, but without the `Send` requirement.

---

## futures_core::stream::BoxStream

*Type Alias*: `core::pin::Pin<alloc::boxed::Box<dyn Stream>>`

An owned dynamically typed [`Stream`] for use in cases where you can't
statically type your result or need to add some indirection.

This type is often created by the [`boxed`] method on [`StreamExt`]. See its documentation for more.

[`boxed`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed
[`StreamExt`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html



## futures_core::stream::FusedStream

*Trait*

A stream which tracks whether or not the underlying stream
should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled.
Usually, this state occurs after `poll_next` (or `try_poll_next`) returned
`Poll::Ready(None)`. However, `is_terminated` may also return `true` if a
stream has become inactive and can no longer make progress and should be
ignored or dropped rather than being polled again.

**Methods:**

- `is_terminated`: Returns `true` if the stream should no longer be polled.



## futures_core::stream::LocalBoxStream

*Type Alias*: `core::pin::Pin<alloc::boxed::Box<dyn Stream>>`

`BoxStream`, but without the `Send` requirement.

This type is often created by the [`boxed_local`] method on [`StreamExt`]. See its documentation for more.

[`boxed_local`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed_local
[`StreamExt`]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html



## futures_core::stream::Stream

*Trait*

A stream of values produced asynchronously.

If `Future<Output = T>` is an asynchronous version of `T`, then `Stream<Item
= T>` is an asynchronous version of `Iterator<Item = T>`. A stream
represents a sequence of value-producing events that occur asynchronously to
the caller.

The trait is modeled after `Future`, but allows `poll_next` to be called
even after a value has been produced, yielding `None` once the stream has
been fully exhausted.

**Methods:**

- `Item`: Values yielded by the stream.
- `poll_next`: Attempt to pull out the next value of this stream, registering the
- `size_hint`: Returns the bounds on the remaining length of the stream.



## futures_core::stream::TryStream

*Trait*

A convenience for streams that return `Result` values that includes
a variety of adapters tailored to such futures.

**Methods:**

- `Ok`: The type of successful values yielded by this future
- `Error`: The type of failures yielded by this future
- `try_poll_next`: Poll this `TryStream` as if it were a `Stream`.



