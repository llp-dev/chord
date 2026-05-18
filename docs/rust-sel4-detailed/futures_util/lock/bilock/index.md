*[futures_util](../../index.md) / [lock](../index.md) / [bilock](index.md)*

---

# Module `bilock`

Futures-powered synchronization primitives.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BiLock`](#bilock) | struct | A type of futures-powered synchronization primitive which is a mutex between two possible owners. |
| [`Inner`](#inner) | struct |  |
| [`ReuniteError`](#reuniteerror) | struct | Error indicating two `BiLock<T>`s were not two halves of a whole, and thus could not be `reunite`d. |
| [`BiLockGuard`](#bilockguard) | struct | Returned RAII guard from the `poll_lock` method. |
| [`invalid_ptr`](#invalid-ptr) | fn |  |

## Structs

### `BiLock<T>`

```rust
struct BiLock<T> {
    arc: alloc::sync::Arc<Inner<T>>,
}
```

A type of futures-powered synchronization primitive which is a mutex between
two possible owners.

This primitive is not as generic as a full-blown mutex but is sufficient for
many use cases where there are only two possible owners of a resource. The
implementation of `BiLock` can be more optimized for just the two possible
owners.

Note that it's possible to use this lock through a poll-style interface with
the `poll_lock` method but you can also use it as a future with the `lock`
method that consumes a `BiLock` and returns a future that will resolve when
it's locked.

A `BiLock` is typically used for "split" operations where data which serves
two purposes wants to be split into two to be worked with separately. For
example a TCP stream could be both a reader and a writer or a framing layer
could be both a stream and a sink for messages. A `BiLock` enables splitting
these two and then using each independently in a futures-powered fashion.

This type is only available when the `bilock` feature of this
library is activated.

#### Implementations

- <span id="bilock-new"></span>`fn new(t: T) -> (Self, Self)`

  Creates a new `BiLock` protecting the provided data.

  

  Two handles to the lock are returned, and these are the only two handles

  that will ever be available to the lock. These can then be sent to separate

  tasks to be managed there.

  

  The data behind the bilock is considered to be pinned, which allows `Pin`

  references to locked data. However, this means that the locked value

  will only be available through `Pin<&mut T>` (not `&mut T`) unless `T` is `Unpin`.

  Similarly, reuniting the lock and extracting the inner value is only

  possible when `T` is `Unpin`.

- <span id="bilock-poll-lock"></span>`fn poll_lock(&self, cx: &mut Context<'_>) -> Poll<BiLockGuard<'_, T>>` — [`Context`](../../task/index.md#context), [`Poll`](../../task/index.md#poll), [`BiLockGuard`](#bilockguard)

  Attempt to acquire this lock, returning `Pending` if it can't be

  acquired.

  

  This function will acquire the lock in a nonblocking fashion, returning

  immediately if the lock is already held. If the lock is successfully

  acquired then `Poll::Ready` is returned with a value that represents

  the locked value (and can be used to access the protected data). The

  lock is unlocked when the returned `BiLockGuard` is dropped.

  

  If the lock is already held then this function will return

  `Poll::Pending`. In this case the current task will also be scheduled

  to receive a notification when the lock would otherwise become

  available.

  

  # Panics

  

  This function will panic if called outside the context of a future's

  task.

- <span id="bilock-is-pair-of"></span>`fn is_pair_of(&self, other: &Self) -> bool`

  Returns `true` only if the other `BiLock<T>` originated from the same call to `BiLock::new`.

- <span id="bilock-reunite"></span>`fn reunite(self, other: Self) -> Result<T, ReuniteError<T>>` — [`ReuniteError`](#reuniteerror)

  Attempts to put the two "halves" of a `BiLock<T>` back together and

  recover the original value. Succeeds only if the two `BiLock<T>`s

  originated from the same call to `BiLock::new`.

- <span id="bilock-unlock"></span>`fn unlock(&self)`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for BiLock<T>`

- <span id="bilock-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Inner<T>`

```rust
struct Inner<T> {
    state: core::sync::atomic::AtomicPtr<futures_core::task::Waker>,
    value: Option<core::cell::UnsafeCell<T>>,
}
```

#### Implementations

- <span id="inner-into-value"></span>`unsafe fn into_value(self) -> T`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Inner<T>`

- <span id="inner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Inner<T>`

- <span id="inner-drop"></span>`fn drop(&mut self)`

##### `impl<T: Send> Send for Inner<T>`

##### `impl<T: Send> Sync for Inner<T>`

### `ReuniteError<T>`

```rust
struct ReuniteError<T>(BiLock<T>, BiLock<T>);
```

Error indicating two `BiLock<T>`s were not two halves of a whole, and
thus could not be `reunite`d.

#### Trait Implementations

##### `impl<T> Debug for ReuniteError<T>`

- <span id="reuniteerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Display for ReuniteError<T>`

- <span id="reuniteerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for ReuniteError<T>`

- <span id="reuniteerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `BiLockGuard<'a, T>`

```rust
struct BiLockGuard<'a, T> {
    bilock: &'a BiLock<T>,
}
```

Returned RAII guard from the `poll_lock` method.

This structure acts as a sentinel to the data in the `BiLock<T>` itself,
implementing `Deref` and `DerefMut` to `T`. When dropped, the lock will be
unlocked.

#### Implementations

- <span id="bilockguard-as-pin-mut"></span>`fn as_pin_mut(&mut self) -> Pin<&mut T>`

  Get a mutable pinned reference to the locked value.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for BiLockGuard<'a, T>`

- <span id="bilockguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Deref for BiLockGuard<'_, T>`

- <span id="bilockguard-deref-type-target"></span>`type Target = T`

- <span id="bilockguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: Unpin> DerefMut for BiLockGuard<'_, T>`

- <span id="bilockguard-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T> Drop for BiLockGuard<'_, T>`

- <span id="bilockguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for BiLockGuard<'a, T>`

- <span id="bilockguard-receiver-type-target"></span>`type Target = T`

##### `impl<T: Send + Sync> Sync for BiLockGuard<'_, T>`

## Functions

### `invalid_ptr`

```rust
fn invalid_ptr<T>(addr: usize) -> *mut T
```

