**lock_api > mutex**

# Module: mutex

## Contents

**Structs**

- [`MappedMutexGuard`](#mappedmutexguard) - An RAII mutex guard returned by `MutexGuard::map`, which can point to a
- [`Mutex`](#mutex) - A mutual exclusion primitive useful for protecting shared data
- [`MutexGuard`](#mutexguard) - An RAII implementation of a "scoped lock" of a mutex. When this structure is

**Traits**

- [`RawMutex`](#rawmutex) - Basic operations for a mutex.
- [`RawMutexFair`](#rawmutexfair) - Additional methods for mutexes which support fair unlocking.
- [`RawMutexTimed`](#rawmutextimed) - Additional methods for mutexes which support locking with timeouts.

---

## lock_api::mutex::MappedMutexGuard

*Struct*

An RAII mutex guard returned by `MutexGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedMutexGuard` and `MutexGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the mutex using a fair unlock protocol.
- `fn map<U, F>(s: Self, f: F) -> MappedMutexGuard<'a, R, U>` - Makes a new `MappedMutexGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, Self>` - Attempts to make a new `MappedMutexGuard` for a component of the
- `fn try_map_or_err<U, F, E>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, (Self, E)>` - Attempts to make a new `MappedMutexGuard` for a component of the

**Traits:** Send, Sync

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Deref**
  - `fn deref(self: &Self) -> &T`



## lock_api::mutex::Mutex

*Struct*

A mutual exclusion primitive useful for protecting shared data

This mutex will block threads waiting for the lock to become available. The
mutex can also be statically initialized or created via a `new`
constructor. Each mutex has a type parameter which represents the data that
it is protecting. The data can only be accessed through the RAII guards
returned from `lock` and `try_lock`, which guarantees that the data is only
ever accessed when the mutex is locked.

**Generic Parameters:**
- R
- T

**Methods:**

- `fn new(val: T) -> Mutex<R, T>` - Creates a new mutex in an unlocked state ready for use.
- `fn into_inner(self: Self) -> T` - Consumes this mutex, returning the underlying data.
- `fn try_lock_for(self: &Self, timeout: <R as >::Duration) -> Option<MutexGuard<R, T>>` - Attempts to acquire this lock until a timeout is reached.
- `fn try_lock_until(self: &Self, timeout: <R as >::Instant) -> Option<MutexGuard<R, T>>` - Attempts to acquire this lock until a timeout is reached.
- `fn from_raw(raw_mutex: R, val: T) -> Mutex<R, T>` - Creates a new mutex based on a pre-existing raw mutex.
- `fn const_new(raw_mutex: R, val: T) -> Mutex<R, T>` - Creates a new mutex based on a pre-existing raw mutex.
- `fn force_unlock_fair(self: &Self)` - Forcibly unlocks the mutex using a fair unlock protocol.
- `fn make_guard_unchecked(self: &Self) -> MutexGuard<R, T>` - Creates a new `MutexGuard` without checking if the mutex is locked.
- `fn lock(self: &Self) -> MutexGuard<R, T>` - Acquires a mutex, blocking the current thread until it is able to do so.
- `fn try_lock(self: &Self) -> Option<MutexGuard<R, T>>` - Attempts to acquire this lock.
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.
- `fn is_locked(self: &Self) -> bool` - Checks whether the mutex is currently locked.
- `fn force_unlock(self: &Self)` - Forcibly unlocks the mutex.
- `fn raw(self: &Self) -> &R` - Returns the underlying raw mutex object.
- `fn data_ptr(self: &Self) -> *mut T` - Returns a raw pointer to the underlying data.

**Traits:** Sync, Send

**Trait Implementations:**

- **Default**
  - `fn default() -> Mutex<R, T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(t: T) -> Mutex<R, T>`



## lock_api::mutex::MutexGuard

*Struct*

An RAII implementation of a "scoped lock" of a mutex. When this structure is
dropped (falls out of scope), the lock will be unlocked.

The data protected by the mutex can be accessed through this guard via its
`Deref` and `DerefMut` implementations.

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the mutex using a fair unlock protocol.
- `fn unlocked_fair<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the mutex to execute the given function.
- `fn bump(s: & mut Self)` - Temporarily yields the mutex to a waiting thread if there is one.
- `fn mutex(s: &Self) -> &'a Mutex<R, T>` - Returns a reference to the original `Mutex` object.
- `fn map<U, F>(s: Self, f: F) -> MappedMutexGuard<'a, R, U>` - Makes a new `MappedMutexGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, Self>` - Attempts to make a new `MappedMutexGuard` for a component of the
- `fn try_map_or_err<U, F, E>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, (Self, E)>` - Attempts to make a new `MappedMutexGuard` for a component of the
- `fn unlocked<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the mutex to execute the given function.
- `fn leak(s: Self) -> &'a  mut T` - Leaks the mutex guard and returns a mutable reference to the data

**Traits:** Sync

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Drop**
  - `fn drop(self: & mut Self)`



## lock_api::mutex::RawMutex

*Trait*

Basic operations for a mutex.

Types implementing this trait can be used by `Mutex` to form a safe and
fully-functioning mutex type.

# Safety

Implementations of this trait must ensure that the mutex is actually
exclusive: a lock can't be acquired while the mutex is already locked.

**Methods:**

- `INIT`: Initial value for an unlocked mutex.
- `GuardMarker`: Marker type which determines whether a lock guard should be `Send`. Use
- `lock`: Acquires this mutex, blocking the current thread until it is able to do so.
- `try_lock`: Attempts to acquire this mutex without blocking. Returns `true`
- `unlock`: Unlocks this mutex.
- `is_locked`: Checks whether the mutex is currently locked.



## lock_api::mutex::RawMutexFair

*Trait*

Additional methods for mutexes which support fair unlocking.

Fair unlocking means that a lock is handed directly over to the next waiting
thread if there is one, without giving other threads the opportunity to
"steal" the lock in the meantime. This is typically slower than unfair
unlocking, but may be necessary in certain circumstances.

**Methods:**

- `unlock_fair`: Unlocks this mutex using a fair unlock protocol.
- `bump`: Temporarily yields the mutex to a waiting thread if there is one.



## lock_api::mutex::RawMutexTimed

*Trait*

Additional methods for mutexes which support locking with timeouts.

The `Duration` and `Instant` types are specified as associated types so that
this trait is usable even in `no_std` environments.

**Methods:**

- `Duration`: Duration type used for `try_lock_for`.
- `Instant`: Instant type used for `try_lock_until`.
- `try_lock_for`: Attempts to acquire this lock until a timeout is reached.
- `try_lock_until`: Attempts to acquire this lock until a timeout is reached.



