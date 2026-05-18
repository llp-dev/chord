*[async_unsync](../index.md) / [semaphore](index.md)*

---

# Module `semaphore`

A simple asynchronous semaphore for limiting and sequencing access
to arbitrary shared resources.

## Contents

- [Structs](#structs)
  - [`Semaphore`](#semaphore)
  - [`Permit`](#permit)
  - [`AcquireError`](#acquireerror)
  - [`Acquire`](#acquire)
  - [`Shared`](#shared)
  - [`WaiterQueue`](#waiterqueue)
  - [`Waiter`](#waiter)
  - [`LateInitWaker`](#lateinitwaker)
- [Enums](#enums)
  - [`TryAcquireError`](#tryacquireerror)
  - [`WaiterState`](#waiterstate)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Semaphore`](#semaphore) | struct | An unsynchronized (`!Sync`), simple semaphore for asynchronous permit acquisition. |
| [`Permit`](#permit) | struct | A permit representing access to the [`Semaphore`]'s guarded resource. |
| [`AcquireError`](#acquireerror) | struct | An error which can occur when a [`Semaphore`] has been closed. |
| [`Acquire`](#acquire) | struct | The [`Future`] returned by [`acquire`](Semaphore::acquire), which resolves when the required number of permits becomes available. |
| [`Shared`](#shared) | struct | The shared [`Semaphore`] accounting state. |
| [`WaiterQueue`](#waiterqueue) | struct |  |
| [`Waiter`](#waiter) | struct | A queue-able waiter that will be notified, when its requested number of semaphore permits has been granted. |
| [`LateInitWaker`](#lateinitwaker) | struct | The `Waker` in an `Acquire` future is only used in case it gets enqueued in the `waiters` list or not at all. |
| [`TryAcquireError`](#tryacquireerror) | enum | An error which can occur when a [`Semaphore`] has been closed or has no available permits. |
| [`WaiterState`](#waiterstate) | enum | The current state of a [`Waiter`]. |

## Structs

### `Semaphore`

```rust
struct Semaphore {
    shared: core::cell::UnsafeCell<Shared>,
}
```

An unsynchronized (`!Sync`), simple semaphore for asynchronous permit
acquisition.

#### Implementations

- <span id="semaphore-new"></span>`const fn new(permits: usize) -> Self`

  Creates a new semaphore with the initial number of permits.

- <span id="semaphore-close"></span>`fn close(&self) -> usize`

  Closes the semaphore and returns the number of notified pending waiters.

  

  This prevents the semaphore from issuing new permits and notifies all

  pending waiters.

- <span id="semaphore-is-closed"></span>`fn is_closed(&self) -> bool`

  Returns `true` if the semaphore has been closed

- <span id="semaphore-waiters"></span>`fn waiters(&self) -> usize`

  Returns the number of currently registered [`Future`](../../futures/prelude/index.md)s waiting for a

  [`Permit`](#permit).

- <span id="semaphore-available-permits"></span>`fn available_permits(&self) -> usize`

  Returns the current number of available permits.

- <span id="semaphore-add-permits"></span>`fn add_permits(&self, n: usize)`

  Adds `n` new permits to the semaphore.

- <span id="semaphore-remove-permits"></span>`fn remove_permits(&self, n: usize)`

  Permanently reduces the number of available permits by `n`.

- <span id="semaphore-try-acquire"></span>`fn try_acquire(&self) -> Result<Permit<'_>, TryAcquireError>` — [`Permit`](#permit), [`TryAcquireError`](#tryacquireerror)

  Acquires a single [`Permit`](#permit) or returns an [error](TryAcquireError), if

  there are no available permits.

  

  # Errors

  

  Fails, if the semaphore has been closed or has no available permits.

- <span id="semaphore-try-acquire-many"></span>`fn try_acquire_many(&self, n: usize) -> Result<Permit<'_>, TryAcquireError>` — [`Permit`](#permit), [`TryAcquireError`](#tryacquireerror)

  Acquires `n` [`Permit`](#permit)s or returns an [error](TryAcquireError), if

  there are not enough available permits.

  

  # Errors

  

  Fails, if the semaphore has been closed or has not enough available

  permits.

- <span id="semaphore-acquire"></span>`fn acquire(&self) -> Acquire<'_>` — [`Acquire`](#acquire)

  Acquires a single [`Permit`](#permit), potentially blocking until one becomes

  available.

  

  # Errors

  

  Awaiting the [`Future`](../../futures/prelude/index.md) fails, if the semaphore has been closed.

- <span id="semaphore-acquire-many"></span>`fn acquire_many(&self, n: usize) -> Acquire<'_>` — [`Acquire`](#acquire)

  Acquires `n` [`Permit`](#permit)s, potentially blocking until they become

  available.

  

  # Errors

  

  Awaiting the [`Future`](../../futures/prelude/index.md) fails, if the semaphore has been closed.

- <span id="semaphore-build-acquire"></span>`fn build_acquire(&self, wants: usize) -> Acquire<'_>` — [`Acquire`](#acquire)

  Returns an correctly initialized [`Acquire`](#acquire) future instance for

  acquiring `wants` permits.

#### Trait Implementations

##### `impl Debug for Semaphore`

- <span id="semaphore-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Permit<'a>`

```rust
struct Permit<'a> {
    shared: &'a core::cell::UnsafeCell<Shared>,
    count: usize,
}
```

A permit representing access to the [`Semaphore`](#semaphore)'s guarded resource.

#### Implementations

- <span id="permit-new"></span>`fn new(shared: &'a UnsafeCell<Shared>, count: usize) -> Self` — [`Shared`](#shared)

  Returns a new [`Permit`](#permit) without actually acquiring it.

  

  NOTE: Only use this to "revive" a Permit that has been explicitly

  [forgotten](Permit::forget)!

- <span id="permit-forget"></span>`fn forget(self)`

  Drops the permit without returning it to the [`Semaphore`](#semaphore).

  

  This permanently reduces the number of available permits.

#### Trait Implementations

##### `impl Debug for Permit<'_>`

- <span id="permit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Permit<'_>`

- <span id="permit-drop"></span>`fn drop(&mut self)`

### `AcquireError`

```rust
struct AcquireError(());
```

An error which can occur when a [`Semaphore`](#semaphore) has been closed.

#### Trait Implementations

##### `impl Clone for AcquireError`

- <span id="acquireerror-clone"></span>`fn clone(&self) -> AcquireError` — [`AcquireError`](#acquireerror)

##### `impl Copy for AcquireError`

##### `impl Debug for AcquireError`

- <span id="acquireerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for AcquireError`

- <span id="acquireerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for AcquireError`

- <span id="acquireerror-partialeq-eq"></span>`fn eq(&self, other: &AcquireError) -> bool` — [`AcquireError`](#acquireerror)

##### `impl PartialOrd for AcquireError`

- <span id="acquireerror-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &AcquireError) -> option::Option<cmp::Ordering>` — [`AcquireError`](#acquireerror)

##### `impl StructuralPartialEq for AcquireError`

### `Acquire<'a>`

```rust
struct Acquire<'a> {
    shared: &'a core::cell::UnsafeCell<Shared>,
    waiter: Waiter,
}
```

The [`Future`](../../futures/prelude/index.md) returned by [`acquire`](Semaphore::acquire), which
resolves when the required number of permits becomes available.

#### Fields

- **`shared`**: `&'a core::cell::UnsafeCell<Shared>`

  The shared [`Semaphore`](#semaphore) state.

- **`waiter`**: `Waiter`

  The state for waiting and resolving the future.

#### Trait Implementations

##### `impl Drop for Acquire<'_>`

- <span id="acquire-drop"></span>`fn drop(&mut self)`

##### `impl Future for Acquire<'a>`

- <span id="acquire-future-type-output"></span>`type Output = Result<Permit<'a>, AcquireError>`

- <span id="acquire-future-poll"></span>`fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as >::Output>`

##### `impl IntoFuture for Acquire<'a>`

- <span id="acquire-intofuture-type-output"></span>`type Output = <F as Future>::Output`

- <span id="acquire-intofuture-type-intofuture"></span>`type IntoFuture = F`

- <span id="acquire-intofuture-into-future"></span>`fn into_future(self) -> <F as IntoFuture>::IntoFuture`

### `Shared`

```rust
struct Shared {
    waiters: WaiterQueue,
    permits: usize,
    closed: bool,
}
```

The shared [`Semaphore`](#semaphore) accounting state.

#### Fields

- **`waiters`**: `WaiterQueue`

  The queue of registered `Waker`s.

- **`permits`**: `usize`

  The number of currently available permits.

- **`closed`**: `bool`

  The flag indicating if the semaphore has been closed.

#### Implementations

- <span id="shared-close"></span>`fn close(&mut self) -> usize`

  Closes the semaphore and notifies all remaining waiters.

- <span id="shared-is-closed"></span>`fn is_closed(&self) -> bool`

  Returns `true` if the semaphore has been closed.

- <span id="shared-add-permits"></span>`fn add_permits(&mut self, n: usize)`

  Adds `n` permits and wakes all waiters whose requests can now be

  completed.

- <span id="shared-try-acquire"></span>`fn try_acquire<const STRICT: bool>(&mut self, n: usize) -> Result<usize, TryAcquireError>` — [`TryAcquireError`](#tryacquireerror)

  Attempts to reduce available permits by up to `n` or returns an error,

  if the semaphore has been closed or has no available permits.

- <span id="shared-poll-acquire"></span>`fn poll_acquire(&mut self, waiter: Pin<&Waiter>, cx: &mut Context<'_>) -> Poll<Result<(), AcquireError>>` — [`Waiter`](#waiter), [`AcquireError`](#acquireerror)

- <span id="shared-poll-acquire-initial"></span>`fn poll_acquire_initial(&mut self, waiter: Pin<&Waiter>, cx: &mut Context<'_>) -> Poll<Result<(), AcquireError>>` — [`Waiter`](#waiter), [`AcquireError`](#acquireerror)

### `WaiterQueue`

```rust
struct WaiterQueue {
    head: *const Waiter,
    tail: *const Waiter,
}
```

#### Implementations

- <span id="waiterqueue-new"></span>`const fn new() -> Self`

  Returns a new empty queue.

- <span id="waiterqueue-front"></span>`fn front(&self) -> Option<NonNull<Waiter>>` — [`Waiter`](#waiter)

  Returns the first `Waiter` of `null`, if the queue is empty.

- <span id="waiterqueue-len"></span>`unsafe fn len(&self) -> usize`

  Returns the number of currently enqueued `Waiter`s.

  

  # Safety

  

  All pointers must reference valid, live and non-aliased `Waiter`s.

- <span id="waiterqueue-push-back"></span>`unsafe fn push_back(&mut self, waiter: &Waiter)` — [`Waiter`](#waiter)

  Enqueues `waiter` at the back of the queue.

  

  # Safety

  

  All pointers must reference valid, live and non-aliased `Waiter`s.

- <span id="waiterqueue-try-remove"></span>`unsafe fn try_remove(&mut self, waiter: &Waiter)` — [`Waiter`](#waiter)

  Searches for `waiter` in the queue and removes it if found.

  

  # Safety

  

  All pointers must reference valid, live and non-aliased `Waiter`s.

- <span id="waiterqueue-pop-front"></span>`unsafe fn pop_front(&mut self, head: &Waiter)` — [`Waiter`](#waiter)

  Removes `head` from the front of the queue.

  

  # Safety

  

  All pointers must reference valid, live and non-aliased `Waiter`s and

  `head` must be the current queue head.

- <span id="waiterqueue-wake-all"></span>`unsafe fn wake_all(&mut self) -> usize`

### `Waiter`

```rust
struct Waiter {
    wants: usize,
    waker: LateInitWaker,
    state: core::cell::Cell<WaiterState>,
    permits: core::cell::Cell<usize>,
    next: core::cell::Cell<*const Self>,
    prev: core::cell::Cell<*const Self>,
    _marker: core::marker::PhantomPinned,
}
```

A queue-able waiter that will be notified, when its requested number of
semaphore permits has been granted.

#### Fields

- **`wants`**: `usize`

  The number of requested permits.

- **`waker`**: `LateInitWaker`

  The waker to be woken if the future is enqueued as waiting.
  
  This field is **never** used, if the waiter does not get enqueued,
  because its request can be fulfilled immediately.

- **`state`**: `core::cell::Cell<WaiterState>`

  The flag indicating the waiter's state.

- **`permits`**: `core::cell::Cell<usize>`

  The counter of already collected permits.

- **`next`**: `core::cell::Cell<*const Self>`

  The pointer to the next enqueued waiter

- **`prev`**: `core::cell::Cell<*const Self>`

  The pointer to the previous enqueued waiter

#### Implementations

- <span id="waiter-new"></span>`const fn new(wants: usize) -> Self`

### `LateInitWaker`

```rust
struct LateInitWaker(core::cell::UnsafeCell<Option<core::task::Waker>>);
```

The `Waker` in an `Acquire` future is only used in case it gets enqueued
in the `waiters` list or not at all.

`get` is only called during traversal of that list, so it is guaranteed to
have been initialized

#### Implementations

- <span id="lateinitwaker-new"></span>`const fn new() -> Self`

- <span id="lateinitwaker-set"></span>`fn set(&self, waker: Waker)`

- <span id="lateinitwaker-get"></span>`unsafe fn get(&self) -> &Waker`

## Enums

### `TryAcquireError`

```rust
enum TryAcquireError {
    Closed,
    NoPermits,
}
```

An error which can occur when a [`Semaphore`](#semaphore) has been closed or has no
available permits.

#### Variants

- **`Closed`**

  The semaphore has been [closed](Semaphore::close) and can not issue new
  permits.

- **`NoPermits`**

  The semaphore has no available permits.

#### Trait Implementations

##### `impl Clone for TryAcquireError`

- <span id="tryacquireerror-clone"></span>`fn clone(&self) -> TryAcquireError` — [`TryAcquireError`](#tryacquireerror)

##### `impl Copy for TryAcquireError`

##### `impl Debug for TryAcquireError`

- <span id="tryacquireerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryAcquireError`

- <span id="tryacquireerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for TryAcquireError`

- <span id="tryacquireerror-partialeq-eq"></span>`fn eq(&self, other: &TryAcquireError) -> bool` — [`TryAcquireError`](#tryacquireerror)

##### `impl PartialOrd for TryAcquireError`

- <span id="tryacquireerror-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &TryAcquireError) -> option::Option<cmp::Ordering>` — [`TryAcquireError`](#tryacquireerror)

##### `impl StructuralPartialEq for TryAcquireError`

### `WaiterState`

```rust
enum WaiterState {
    Inert,
    Waiting,
    Woken,
}
```

The current state of a [`Waiter`](#waiter).

#### Variants

- **`Inert`**

  The waiter is inert and its future has not yet been polled.

- **`Waiting`**

  The waiter's future has been polled and the waiter was enqueued.

- **`Woken`**

  The waiter's future has been polled to completion.
  
  If the waiter had been queued it is now no longer queued.

#### Trait Implementations

##### `impl Clone for WaiterState`

- <span id="waiterstate-clone"></span>`fn clone(&self) -> WaiterState` — [`WaiterState`](#waiterstate)

##### `impl Copy for WaiterState`

