*[futures_util](../index.md) / [abortable](index.md)*

---

# Module `abortable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Abortable`](#abortable) | struct | A future/stream which can be remotely short-circuited using an `AbortHandle`. |
| [`AbortRegistration`](#abortregistration) | struct | A registration handle for an `Abortable` task. |
| [`AbortHandle`](#aborthandle) | struct | A handle to an `Abortable` task. |
| [`AbortInner`](#abortinner) | struct |  |
| [`Aborted`](#aborted) | struct | Indicator that the `Abortable` task was aborted. |

## Structs

### `Abortable<T>`

```rust
struct Abortable<T> {
    task: T,
    inner: alloc::sync::Arc<AbortInner>,
}
```

A future/stream which can be remotely short-circuited using an `AbortHandle`.

#### Implementations

- <span id="abortable-new"></span>`fn new(task: T, reg: AbortRegistration) -> Self` — [`AbortRegistration`](#abortregistration)

  Creates a new `Abortable` future/stream using an existing `AbortRegistration`.

  `AbortRegistration`s can be acquired through `AbortHandle::new`.

  

  When `abort` is called on the handle tied to `reg` or if `abort` has

  already been called, the future/stream will complete immediately without making

  any further progress.

  

  # Examples:

  

  Usage with futures:

  

  ```rust

  futures::executor::block_on(async {

  use futures::future::{Abortable, AbortHandle, Aborted};

  

  let (abort_handle, abort_registration) = AbortHandle::new_pair();

  let future = Abortable::new(async { 2 }, abort_registration);

  abort_handle.abort();

  assert_eq!(future.await, Err(Aborted));

  });

  ```

  

  Usage with streams:

  

  ```rust

  futures::executor::block_on(async {

  use futures::future::{Abortable, AbortHandle};

  use futures::stream::{self, StreamExt};

  

  let (abort_handle, abort_registration) = AbortHandle::new_pair();

  let mut stream = Abortable::new(stream::iter(vec![1, 2, 3]), abort_registration);

  abort_handle.abort();

  assert_eq!(stream.next().await, None);

  });

  ```

- <span id="abortable-is-aborted"></span>`fn is_aborted(&self) -> bool`

  Checks whether the task has been aborted. Note that all this

  method indicates is whether `AbortHandle::abort` was *called*.

  This means that it will return `true` even if:

  * `abort` was called after the task had completed.

  * `abort` was called while the task was being polled - the task may still be running and

    will not be stopped until `poll` returns.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Abortable<T>`

- <span id="abortable-clone"></span>`fn clone(&self) -> Abortable<T>` — [`Abortable`](#abortable)

##### `impl<T: fmt::Debug> Debug for Abortable<T>`

- <span id="abortable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fut> Future for Abortable<Fut>`

- <span id="abortable-future-type-output"></span>`type Output = Result<<Fut as Future>::Output, Aborted>`

- <span id="abortable-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl<T> FutureExt for Abortable<T>`

##### `impl IntoFuture for Abortable<T>`

- <span id="abortable-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="abortable-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="abortable-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

##### `impl<St> Stream for Abortable<St>`

- <span id="abortable-stream-type-item"></span>`type Item = <St as Stream>::Item`

- <span id="abortable-stream-poll-next"></span>`fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<<Self as >::Item>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Stream`](../stream/index.md#stream)

##### `impl<T> StreamExt for Abortable<T>`

##### `impl<T> TryFuture for Abortable<T>`

- <span id="abortable-tryfuture-type-ok"></span>`type Ok = T`

- <span id="abortable-tryfuture-type-error"></span>`type Error = E`

- <span id="abortable-tryfuture-try-poll"></span>`fn try_poll(self: Pin<&mut F>, cx: &mut Context<'_>) -> Poll<<F as Future>::Output>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`Future`](../future/index.md#future)

##### `impl TryFutureExt for Abortable<T>`

##### `impl<T> TryStream for Abortable<T>`

- <span id="abortable-trystream-type-ok"></span>`type Ok = T`

- <span id="abortable-trystream-type-error"></span>`type Error = E`

- <span id="abortable-trystream-try-poll-next"></span>`fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<'_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>>` — [`Context`](../task/index.md#context), [`Poll`](../task/index.md#poll), [`TryStream`](../stream/index.md#trystream)

##### `impl TryStreamExt for Abortable<T>`

##### `impl<T> Unpin for Abortable<T>`

### `AbortRegistration`

```rust
struct AbortRegistration {
    inner: alloc::sync::Arc<AbortInner>,
}
```

A registration handle for an `Abortable` task.
Values of this type can be acquired from `AbortHandle::new` and are used
in calls to `Abortable::new`.

#### Implementations

- <span id="abortregistration-handle"></span>`fn handle(&self) -> AbortHandle` — [`AbortHandle`](#aborthandle)

  Create an [`AbortHandle`](#aborthandle) from the given [`AbortRegistration`](#abortregistration).

  

  The created [`AbortHandle`](#aborthandle) is functionally the same as any other

  [`AbortHandle`](#aborthandle)s that are associated with the same [`AbortRegistration`](#abortregistration),

  such as the one created by `AbortHandle::new_pair`.

#### Trait Implementations

##### `impl Debug for AbortRegistration`

- <span id="abortregistration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AbortHandle`

```rust
struct AbortHandle {
    inner: alloc::sync::Arc<AbortInner>,
}
```

A handle to an `Abortable` task.

#### Implementations

- <span id="aborthandle-new-pair"></span>`fn new_pair() -> (Self, AbortRegistration)` — [`AbortRegistration`](#abortregistration)

  Creates an (`AbortHandle`, `AbortRegistration`) pair which can be used

  to abort a running future or stream.

  

  This function is usually paired with a call to `Abortable::new`.

#### Trait Implementations

##### `impl Clone for AbortHandle`

- <span id="aborthandle-clone"></span>`fn clone(&self) -> AbortHandle` — [`AbortHandle`](#aborthandle)

##### `impl Debug for AbortHandle`

- <span id="aborthandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AbortInner`

```rust
struct AbortInner {
    waker: crate::task::AtomicWaker,
    aborted: core::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl Debug for AbortInner`

- <span id="abortinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Aborted`

```rust
struct Aborted;
```

Indicator that the `Abortable` task was aborted.

#### Trait Implementations

##### `impl Clone for Aborted`

- <span id="aborted-clone"></span>`fn clone(&self) -> Aborted` — [`Aborted`](#aborted)

##### `impl Copy for Aborted`

##### `impl Debug for Aborted`

- <span id="aborted-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Aborted`

- <span id="aborted-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Aborted`

##### `impl PartialEq for Aborted`

- <span id="aborted-partialeq-eq"></span>`fn eq(&self, other: &Aborted) -> bool` — [`Aborted`](#aborted)

##### `impl StructuralPartialEq for Aborted`

##### `impl ToString for Aborted`

- <span id="aborted-tostring-to-string"></span>`fn to_string(&self) -> String`

