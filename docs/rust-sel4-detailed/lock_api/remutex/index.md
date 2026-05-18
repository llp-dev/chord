*[lock_api](../index.md) / [remutex](index.md)*

---

# Module `remutex`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RawReentrantMutex`](#rawreentrantmutex) | struct | A raw mutex type that wraps another raw mutex to provide reentrancy. |
| [`ReentrantMutex`](#reentrantmutex) | struct | A mutex which can be recursively locked by a single thread. |
| [`ReentrantMutexGuard`](#reentrantmutexguard) | struct | An RAII implementation of a "scoped lock" of a reentrant mutex. |
| [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard) | struct | An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can point to a subfield of the protected data. |
| [`GetThreadId`](#getthreadid) | trait | Helper trait which returns a non-zero thread ID. |

## Structs

### `RawReentrantMutex<R, G>`

```rust
struct RawReentrantMutex<R, G> {
    owner: core::sync::atomic::AtomicUsize,
    lock_count: core::cell::Cell<usize>,
    mutex: R,
    get_thread_id: G,
}
```

A raw mutex type that wraps another raw mutex to provide reentrancy.

Although this has the same methods as the [`RawMutex`](../index.md) trait, it does
not implement it, and should not be used in the same way, since this
mutex can successfully acquire a lock multiple times in the same thread.
Only use this when you know you want a raw mutex that can be locked
reentrantly; you probably want [`ReentrantMutex`](../index.md) instead.

#### Implementations

- <span id="rawreentrantmutex-const-init"></span>`const INIT: Self`

- <span id="rawreentrantmutex-lock-internal"></span>`fn lock_internal<F: FnOnce() -> bool>(&self, try_lock: F) -> bool`

- <span id="rawreentrantmutex-lock"></span>`fn lock(&self)`

  Acquires this mutex, blocking if it's held by another thread.

- <span id="rawreentrantmutex-try-lock"></span>`fn try_lock(&self) -> bool`

  Attempts to acquire this mutex without blocking. Returns `true`

  if the lock was successfully acquired and `false` otherwise.

- <span id="rawreentrantmutex-unlock"></span>`unsafe fn unlock(&self)`

  Unlocks this mutex. The inner mutex may not be unlocked if

  this mutex was acquired previously in the current thread.

  

  # Safety

  

  This method may only be called if the mutex is held by the current thread.

- <span id="rawreentrantmutex-is-locked"></span>`fn is_locked(&self) -> bool`

  Checks whether the mutex is currently locked.

- <span id="rawreentrantmutex-is-owned-by-current-thread"></span>`fn is_owned_by_current_thread(&self) -> bool`

  Checks whether the mutex is currently held by the current thread.

#### Trait Implementations

##### `impl<R: RawMutex + Send, G: GetThreadId + Send> Send for RawReentrantMutex<R, G>`

##### `impl<R: RawMutex + Sync, G: GetThreadId + Sync> Sync for RawReentrantMutex<R, G>`

### `ReentrantMutex<R, G, T: ?Sized>`

```rust
struct ReentrantMutex<R, G, T: ?Sized> {
    raw: RawReentrantMutex<R, G>,
    data: core::cell::UnsafeCell<T>,
}
```

A mutex which can be recursively locked by a single thread.

This type is identical to `Mutex` except for the following points:

- Locking multiple times from the same thread will work correctly instead of
  deadlocking.
- `ReentrantMutexGuard` does not give mutable references to the locked data.
  Use a `RefCell` if you need this.

See [`Mutex`](crate::Mutex) for more details about the underlying mutex
primitive.

#### Implementations

- <span id="reentrantmutex-new"></span>`const fn new(val: T) -> ReentrantMutex<R, G, T>` — [`ReentrantMutex`](../index.md#reentrantmutex)

  Creates a new reentrant mutex in an unlocked state ready for use.

- <span id="reentrantmutex-into-inner"></span>`fn into_inner(self) -> T`

  Consumes this mutex, returning the underlying data.

#### Trait Implementations

##### `impl<R: RawMutex, G: GetThreadId, T: ?Sized + fmt::Debug> Debug for ReentrantMutex<R, G, T>`

- <span id="reentrantmutex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex, G: GetThreadId, T: ?Sized + Default> Default for ReentrantMutex<R, G, T>`

- <span id="reentrantmutex-default"></span>`fn default() -> ReentrantMutex<R, G, T>` — [`ReentrantMutex`](../index.md#reentrantmutex)

##### `impl<R: RawMutex + Send, G: GetThreadId + Send, T: ?Sized + Send> Send for ReentrantMutex<R, G, T>`

##### `impl<R: RawMutex + Sync, G: GetThreadId + Sync, T: ?Sized + Send> Sync for ReentrantMutex<R, G, T>`

### `ReentrantMutexGuard<'a, R: RawMutex, G: GetThreadId, T: ?Sized>`

```rust
struct ReentrantMutexGuard<'a, R: RawMutex, G: GetThreadId, T: ?Sized> {
    remutex: &'a ReentrantMutex<R, G, T>,
    marker: core::marker::PhantomData<(&'a T, crate::GuardNoSend)>,
}
```

An RAII implementation of a "scoped lock" of a reentrant mutex. When this structure
is dropped (falls out of scope), the lock will be unlocked.

The data protected by the mutex can be accessed through this guard via its
`Deref` implementation.

#### Implementations

- <span id="reentrantmutexguard-remutex"></span>`fn remutex(s: &Self) -> &'a ReentrantMutex<R, G, T>` — [`ReentrantMutex`](../index.md#reentrantmutex)

  Returns a reference to the original `ReentrantMutex` object.

- <span id="reentrantmutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedReentrantMutexGuard<'a, R, G, U>` — [`MappedReentrantMutexGuard`](../index.md#mappedreentrantmutexguard)

  Makes a new `MappedReentrantMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `ReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `ReentrantMutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="reentrantmutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, Self>` — [`MappedReentrantMutexGuard`](../index.md#mappedreentrantmutexguard)

  Attempts to make  a new `MappedReentrantMutexGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `ReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `ReentrantMutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="reentrantmutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, (Self, E)>` — [`MappedReentrantMutexGuard`](../index.md#mappedreentrantmutexguard)

  Attempts to make  a new `MappedReentrantMutexGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `ReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `ReentrantMutexGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="reentrantmutexguard-unlocked"></span>`fn unlocked<F, U>(s: &mut Self, f: F) -> U`

  Temporarily unlocks the mutex to execute the given function.

  

  This is safe because `&mut` guarantees that there exist no other

  references to the data protected by the mutex.

#### Trait Implementations

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: fmt::Debug + ?Sized + 'a> Debug for ReentrantMutexGuard<'a, R, G, T>`

- <span id="reentrantmutexguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: ?Sized + 'a> Deref for ReentrantMutexGuard<'a, R, G, T>`

- <span id="reentrantmutexguard-deref-type-target"></span>`type Target = T`

- <span id="reentrantmutexguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: fmt::Display + ?Sized + 'a> Display for ReentrantMutexGuard<'a, R, G, T>`

- <span id="reentrantmutexguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: ?Sized + 'a> Drop for ReentrantMutexGuard<'a, R, G, T>`

- <span id="reentrantmutexguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for ReentrantMutexGuard<'a, R, G, T>`

- <span id="reentrantmutexguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawMutex + Sync + 'a, G: GetThreadId + Sync + 'a, T: ?Sized + Sync + 'a> Sync for ReentrantMutexGuard<'a, R, G, T>`

### `MappedReentrantMutexGuard<'a, R: RawMutex, G: GetThreadId, T: ?Sized>`

```rust
struct MappedReentrantMutexGuard<'a, R: RawMutex, G: GetThreadId, T: ?Sized> {
    raw: &'a RawReentrantMutex<R, G>,
    data: *const T,
    marker: core::marker::PhantomData<&'a T>,
}
```

An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedReentrantMutexGuard` and `ReentrantMutexGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

#### Implementations

- <span id="mappedreentrantmutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedReentrantMutexGuard<'a, R, G, U>` — [`MappedReentrantMutexGuard`](../index.md#mappedreentrantmutexguard)

  Makes a new `MappedReentrantMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedReentrantMutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedreentrantmutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, Self>` — [`MappedReentrantMutexGuard`](../index.md#mappedreentrantmutexguard)

  Attempts to make  a new `MappedReentrantMutexGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `MappedReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedReentrantMutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedreentrantmutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, (Self, E)>` — [`MappedReentrantMutexGuard`](../index.md#mappedreentrantmutexguard)

  Attempts to make  a new `MappedReentrantMutexGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `MappedReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedReentrantMutexGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

#### Trait Implementations

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: fmt::Debug + ?Sized + 'a> Debug for MappedReentrantMutexGuard<'a, R, G, T>`

- <span id="mappedreentrantmutexguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: ?Sized + 'a> Deref for MappedReentrantMutexGuard<'a, R, G, T>`

- <span id="mappedreentrantmutexguard-deref-type-target"></span>`type Target = T`

- <span id="mappedreentrantmutexguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: fmt::Display + ?Sized + 'a> Display for MappedReentrantMutexGuard<'a, R, G, T>`

- <span id="mappedreentrantmutexguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, G: GetThreadId + 'a, T: ?Sized + 'a> Drop for MappedReentrantMutexGuard<'a, R, G, T>`

- <span id="mappedreentrantmutexguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for MappedReentrantMutexGuard<'a, R, G, T>`

- <span id="mappedreentrantmutexguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawMutex + Sync + 'a, G: GetThreadId + Sync + 'a, T: ?Sized + Sync + 'a> Sync for MappedReentrantMutexGuard<'a, R, G, T>`

## Traits

### `GetThreadId`

```rust
trait GetThreadId { ... }
```

Helper trait which returns a non-zero thread ID.

The simplest way to implement this trait is to return the address of a
thread-local variable.

# Safety

Implementations of this trait must ensure that no two active threads share
the same thread ID. However the ID of a thread that has exited can be
re-used since that thread is no longer active.

#### Associated Constants

- `const INIT: Self`

#### Required Methods

- `fn nonzero_thread_id(&self) -> NonZeroUsize`

  Returns a non-zero thread ID which identifies the current thread of

