**futures_core > future**

# Module: future

## Contents

**Traits**

- [`FusedFuture`](#fusedfuture) - A future which tracks whether or not the underlying future
- [`TryFuture`](#tryfuture) - A convenience for futures that return `Result` values that includes

**Type Aliases**

- [`BoxFuture`](#boxfuture) - An owned dynamically typed [`Future`] for use in cases where you can't
- [`LocalBoxFuture`](#localboxfuture) - `BoxFuture`, but without the `Send` requirement.

---

## futures_core::future::BoxFuture

*Type Alias*: `core::pin::Pin<alloc::boxed::Box<dyn Future>>`

An owned dynamically typed [`Future`] for use in cases where you can't
statically type your result or need to add some indirection.

This type is often created by the [`boxed`] method on [`FutureExt`]. See its documentation for more.

[`boxed`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed
[`FutureExt`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html



## futures_core::future::FusedFuture

*Trait*

A future which tracks whether or not the underlying future
should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled.
Usually, this state occurs after `poll` (or `try_poll`) returned
`Poll::Ready`. However, `is_terminated` may also return `true` if a future
has become inactive and can no longer make progress and should be ignored
or dropped rather than being `poll`ed again.

**Methods:**

- `is_terminated`: Returns `true` if the underlying future should no longer be polled.



## futures_core::future::LocalBoxFuture

*Type Alias*: `core::pin::Pin<alloc::boxed::Box<dyn Future>>`

`BoxFuture`, but without the `Send` requirement.

This type is often created by the [`boxed_local`] method on [`FutureExt`]. See its documentation for more.

[`boxed_local`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed_local
[`FutureExt`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html



## futures_core::future::TryFuture

*Trait*

A convenience for futures that return `Result` values that includes
a variety of adapters tailored to such futures.

**Methods:**

- `Ok`: The type of successful values yielded by this future
- `Error`: The type of failures yielded by this future
- `try_poll`: Poll this `TryFuture` as if it were a `Future`.



