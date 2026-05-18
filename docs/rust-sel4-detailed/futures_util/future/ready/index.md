*[futures_util](../../index.md) / [future](../index.md) / [ready](index.md)*

---

# Module `ready`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ready`](#ready) | struct | Future for the [`ready`](ready()) function. |
| [`ready`](#ready) | fn | Creates a future that is immediately ready with a value. |
| [`ok`](#ok) | fn | Create a future that is immediately ready with a success value. |
| [`err`](#err) | fn | Create a future that is immediately ready with an error value. |

## Structs

### `Ready<T>`

```rust
struct Ready<T>(Option<T>);
```

Future for the [`ready`](ready()) function.

#### Implementations

- <span id="ready-into-inner"></span>`fn into_inner(self) -> T`

  Unwraps the value from this immediately ready future.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Ready<T>`

- <span id="ready-clone"></span>`fn clone(&self) -> Ready<T>` — [`Ready`](#ready)

##### `impl<T: fmt::Debug> Debug for Ready<T>`

- <span id="ready-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> FusedFuture for Ready<T>`

- <span id="ready-fusedfuture-is-terminated"></span>`fn is_terminated(&self) -> bool`

##### `impl<T> Future for Ready<T>`

- <span id="ready-future-type-output"></span>`type Output = T`

- <span id="ready-future-poll"></span>`fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll)

##### `impl<T> FutureExt for Ready<T>`

##### `impl IntoFuture for Ready<T>`

- <span id="ready-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="ready-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="ready-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<T> TryFuture for Ready<T>`

- <span id="ready-tryfuture-type-ok"></span>`type Ok = T`

- <span id="ready-tryfuture-type-error"></span>`type Error = E`

- <span id="ready-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`Future`](../index.md#future)

##### `impl TryFutureExt for Ready<T>`

##### `impl<T> Unpin for Ready<T>`

## Functions

### `ready`

```rust
fn ready<T>(t: T) -> Ready<T>
```

Creates a future that is immediately ready with a value.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ready(1);
assert_eq!(a.await, 1);
});
```

### `ok`

```rust
fn ok<T, E>(t: T) -> Ready<Result<T, E>>
```

Create a future that is immediately ready with a success value.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::ok::<i32, i32>(1);
assert_eq!(a.await, Ok(1));
});
```

### `err`

```rust
fn err<T, E>(err: E) -> Ready<Result<T, E>>
```

Create a future that is immediately ready with an error value.

# Examples

```rust
futures::executor::block_on(async {
use futures::future;

let a = future::err::<i32, i32>(1);
assert_eq!(a.await, Err(1));
});
```

