**lock_api > remutex**

# Module: remutex

## Contents

**Structs**

- [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard) - An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can point to a
- [`RawReentrantMutex`](#rawreentrantmutex) - A raw mutex type that wraps another raw mutex to provide reentrancy.
- [`ReentrantMutex`](#reentrantmutex) - A mutex which can be recursively locked by a single thread.
- [`ReentrantMutexGuard`](#reentrantmutexguard) - An RAII implementation of a "scoped lock" of a reentrant mutex. When this structure

**Traits**

- [`GetThreadId`](#getthreadid) - Helper trait which returns a non-zero thread ID.

---

## lock_api::remutex::GetThreadId

*Trait*

Helper trait which returns a non-zero thread ID.

The simplest way to implement this trait is to return the address of a
thread-local variable.

# Safety

Implementations of this trait must ensure that no two active threads share
the same thread ID. However the ID of a thread that has exited can be
re-used since that thread is no longer active.

**Methods:**

- `INIT`: Initial value.
- `nonzero_thread_id`: Returns a non-zero thread ID which identifies the current thread of



## lock_api::remutex::MappedReentrantMutexGuard

*Struct*

An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedReentrantMutexGuard` and `ReentrantMutexGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

**Generic Parameters:**
- 'a
- R
- G
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the mutex using a fair unlock protocol.
- `fn map<U, F>(s: Self, f: F) -> MappedReentrantMutexGuard<'a, R, G, U>` - Makes a new `MappedReentrantMutexGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, Self>` - Attempts to make  a new `MappedReentrantMutexGuard` for a component of the
- `fn try_map_or_err<U, F, E>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, (Self, E)>` - Attempts to make  a new `MappedReentrantMutexGuard` for a component of the

**Traits:** Sync

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &T`



## lock_api::remutex::RawReentrantMutex

*Struct*

A raw mutex type that wraps another raw mutex to provide reentrancy.

Although this has the same methods as the [`RawMutex`] trait, it does
not implement it, and should not be used in the same way, since this
mutex can successfully acquire a lock multiple times in the same thread.
Only use this when you know you want a raw mutex that can be locked
reentrantly; you probably want [`ReentrantMutex`] instead.

**Generic Parameters:**
- R
- G

**Methods:**

- `fn try_lock_until(self: &Self, timeout: <R as >::Instant) -> bool` - Attempts to acquire this lock until a timeout is reached.
- `fn try_lock_for(self: &Self, timeout: <R as >::Duration) -> bool` - Attempts to acquire this lock until a timeout is reached.
- `fn lock(self: &Self)` - Acquires this mutex, blocking if it's held by another thread.
- `fn try_lock(self: &Self) -> bool` - Attempts to acquire this mutex without blocking. Returns `true`
- `fn unlock(self: &Self)` - Unlocks this mutex. The inner mutex may not be unlocked if
- `fn is_locked(self: &Self) -> bool` - Checks whether the mutex is currently locked.
- `fn is_owned_by_current_thread(self: &Self) -> bool` - Checks whether the mutex is currently held by the current thread.
- `fn unlock_fair(self: &Self)` - Unlocks this mutex using a fair unlock protocol. The inner mutex
- `fn bump(self: &Self)` - Temporarily yields the mutex to a waiting thread if there is one.

**Traits:** Send, Sync



## lock_api::remutex::ReentrantMutex

*Struct*

A mutex which can be recursively locked by a single thread.

This type is identical to `Mutex` except for the following points:

- Locking multiple times from the same thread will work correctly instead of
  deadlocking.
- `ReentrantMutexGuard` does not give mutable references to the locked data.
  Use a `RefCell` if you need this.

See [`Mutex`](crate::Mutex) for more details about the underlying mutex
primitive.

**Generic Parameters:**
- R
- G
- T

**Methods:**

- `fn new(val: T) -> ReentrantMutex<R, G, T>` - Creates a new reentrant mutex in an unlocked state ready for use.
- `fn into_inner(self: Self) -> T` - Consumes this mutex, returning the underlying data.
- `fn try_lock_for(self: &Self, timeout: <R as >::Duration) -> Option<ReentrantMutexGuard<R, G, T>>` - Attempts to acquire this lock until a timeout is reached.
- `fn try_lock_until(self: &Self, timeout: <R as >::Instant) -> Option<ReentrantMutexGuard<R, G, T>>` - Attempts to acquire this lock until a timeout is reached.
- `fn from_raw(raw_mutex: R, get_thread_id: G, val: T) -> ReentrantMutex<R, G, T>` - Creates a new reentrant mutex based on a pre-existing raw mutex and a
- `fn const_new(raw_mutex: R, get_thread_id: G, val: T) -> ReentrantMutex<R, G, T>` - Creates a new reentrant mutex based on a pre-existing raw mutex and a
- `fn force_unlock_fair(self: &Self)` - Forcibly unlocks the mutex using a fair unlock protocol.
- `fn make_guard_unchecked(self: &Self) -> ReentrantMutexGuard<R, G, T>` - Creates a new `ReentrantMutexGuard` without checking if the lock is held.
- `fn lock(self: &Self) -> ReentrantMutexGuard<R, G, T>` - Acquires a reentrant mutex, blocking the current thread until it is able
- `fn try_lock(self: &Self) -> Option<ReentrantMutexGuard<R, G, T>>` - Attempts to acquire this lock.
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.
- `fn is_locked(self: &Self) -> bool` - Checks whether the mutex is currently locked.
- `fn is_owned_by_current_thread(self: &Self) -> bool` - Checks whether the mutex is currently held by the current thread.
- `fn force_unlock(self: &Self)` - Forcibly unlocks the mutex.
- `fn raw(self: &Self) -> &R` - Returns the underlying raw mutex object.
- `fn data_ptr(self: &Self) -> *mut T` - Returns a raw pointer to the underlying data.

**Traits:** Send, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(t: T) -> ReentrantMutex<R, G, T>`
- **Default**
  - `fn default() -> ReentrantMutex<R, G, T>`



## lock_api::remutex::ReentrantMutexGuard

*Struct*

An RAII implementation of a "scoped lock" of a reentrant mutex. When this structure
is dropped (falls out of scope), the lock will be unlocked.

The data protected by the mutex can be accessed through this guard via its
`Deref` implementation.

**Generic Parameters:**
- 'a
- R
- G
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the mutex using a fair unlock protocol.
- `fn unlocked_fair<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the mutex to execute the given function.
- `fn bump(s: & mut Self)` - Temporarily yields the mutex to a waiting thread if there is one.
- `fn remutex(s: &Self) -> &'a ReentrantMutex<R, G, T>` - Returns a reference to the original `ReentrantMutex` object.
- `fn map<U, F>(s: Self, f: F) -> MappedReentrantMutexGuard<'a, R, G, U>` - Makes a new `MappedReentrantMutexGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, Self>` - Attempts to make  a new `MappedReentrantMutexGuard` for a component of the
- `fn try_map_or_err<U, F, E>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, (Self, E)>` - Attempts to make  a new `MappedReentrantMutexGuard` for a component of the
- `fn unlocked<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the mutex to execute the given function.

**Traits:** Sync

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &T`



