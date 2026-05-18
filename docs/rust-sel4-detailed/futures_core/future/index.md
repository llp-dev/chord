*[futures_core](../index.md) / [future](index.md)*

---

# Module `future`

Futures.

## Contents

- [Modules](#modules)
  - [`private_try_future`](#private-try-future)
  - [`if_alloc`](#if-alloc)
- [Traits](#traits)
  - [`FusedFuture`](#fusedfuture)
  - [`TryFuture`](#tryfuture)
- [Functions](#functions)
  - [`Future`](#future)
- [Type Aliases](#type-aliases)
  - [`BoxFuture`](#boxfuture)
  - [`LocalBoxFuture`](#localboxfuture)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private_try_future`](#private-try-future) | mod |  |
| [`if_alloc`](#if-alloc) | mod |  |
| [`FusedFuture`](#fusedfuture) | trait | A future which tracks whether or not the underlying future should no longer be polled. |
| [`TryFuture`](#tryfuture) | trait | A convenience for futures that return `Result` values that includes a variety of adapters tailored to such futures. |
| [`Future`](#future) | fn |  |
| [`BoxFuture`](#boxfuture) | type | An owned dynamically typed [`Future`] for use in cases where you can't statically type your result or need to add some indirection. |
| [`LocalBoxFuture`](#localboxfuture) | type | `BoxFuture`, but without the `Send` requirement. |

## Modules

- [`private_try_future`](private_try_future/index.md)
- [`if_alloc`](if_alloc/index.md)

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

## Functions

### `Future`

```rust
fn Future(bytes: &[u8]) -> Self
```

## Type Aliases

### `BoxFuture<'a, T>`

```rust
type BoxFuture<'a, T> = core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T> + Send>>;
```

An owned dynamically typed [`Future`](#future) for use in cases where you can't
statically type your result or need to add some indirection.

This type is often created by the [`boxed`](#boxed) method on [`FutureExt`](#futureext). See its documentation for more.



### `LocalBoxFuture<'a, T>`

```rust
type LocalBoxFuture<'a, T> = core::pin::Pin<alloc::boxed::Box<dyn Future<Output = T>>>;
```

`BoxFuture`, but without the `Send` requirement.

This type is often created by the `boxed_local` method on [`FutureExt`](#futureext). See its documentation for more.



