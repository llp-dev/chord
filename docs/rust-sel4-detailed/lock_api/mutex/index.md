*[lock_api](../index.md) / [mutex](index.md)*

---

# Module `mutex`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Mutex`](#mutex) | struct | A mutual exclusion primitive useful for protecting shared data |
| [`MutexGuard`](#mutexguard) | struct | An RAII implementation of a "scoped lock" of a mutex. |
| [`MappedMutexGuard`](#mappedmutexguard) | struct | An RAII mutex guard returned by `MutexGuard::map`, which can point to a subfield of the protected data. |
| [`RawMutex`](#rawmutex) | trait | Basic operations for a mutex. |
| [`RawMutexFair`](#rawmutexfair) | trait | Additional methods for mutexes which support fair unlocking. |
| [`RawMutexTimed`](#rawmutextimed) | trait | Additional methods for mutexes which support locking with timeouts. |

## Structs

### `Mutex<R, T: ?Sized>`

```rust
struct Mutex<R, T: ?Sized> {
    raw: R,
    data: core::cell::UnsafeCell<T>,
}
```

A mutual exclusion primitive useful for protecting shared data

This mutex will block threads waiting for the lock to become available. The
mutex can also be statically initialized or created via a `new`
constructor. Each mutex has a type parameter which represents the data that
it is protecting. The data can only be accessed through the RAII guards
returned from `lock` and `try_lock`, which guarantees that the data is only
ever accessed when the mutex is locked.

#### Implementations

- <span id="mutex-new"></span>`const fn new(val: T) -> Mutex<R, T>` — [`Mutex`](../index.md#mutex)

  Creates a new mutex in an unlocked state ready for use.

- <span id="mutex-into-inner"></span>`fn into_inner(self) -> T`

  Consumes this mutex, returning the underlying data.

#### Trait Implementations

##### `impl<R: RawMutex, T: ?Sized + fmt::Debug> Debug for Mutex<R, T>`

- <span id="mutex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex, T: ?Sized + Default> Default for Mutex<R, T>`

- <span id="mutex-default"></span>`fn default() -> Mutex<R, T>` — [`Mutex`](../index.md#mutex)

##### `impl<R: RawMutex + Send, T: ?Sized + Send> Send for Mutex<R, T>`

##### `impl<R: RawMutex + Sync, T: ?Sized + Send> Sync for Mutex<R, T>`

### `MutexGuard<'a, R: RawMutex, T: ?Sized>`

```rust
struct MutexGuard<'a, R: RawMutex, T: ?Sized> {
    mutex: &'a Mutex<R, T>,
    marker: core::marker::PhantomData<(&'a mut T, <R as >::GuardMarker)>,
}
```

An RAII implementation of a "scoped lock" of a mutex. When this structure is
dropped (falls out of scope), the lock will be unlocked.

The data protected by the mutex can be accessed through this guard via its
`Deref` and `DerefMut` implementations.

#### Implementations

- <span id="mutexguard-mutex"></span>`fn mutex(s: &Self) -> &'a Mutex<R, T>` — [`Mutex`](../index.md#mutex)

  Returns a reference to the original `Mutex` object.

- <span id="mutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedMutexGuard<'a, R, U>` — [`MappedMutexGuard`](../index.md#mappedmutexguard)

  Makes a new `MappedMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `MutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, Self>` — [`MappedMutexGuard`](../index.md#mappedmutexguard)

  Attempts to make a new `MappedMutexGuard` for a component of the

  locked data. The original guard is returned if the closure returns `None`.

  

  This operation cannot fail as the `MutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, (Self, E)>` — [`MappedMutexGuard`](../index.md#mappedmutexguard)

  Attempts to make a new `MappedMutexGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `MutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MutexGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mutexguard-unlocked"></span>`fn unlocked<F, U>(s: &mut Self, f: F) -> U`

  Temporarily unlocks the mutex to execute the given function.

  

  This is safe because `&mut` guarantees that there exist no other

  references to the data protected by the mutex.

- <span id="mutexguard-leak"></span>`fn leak(s: Self) -> &'a mut T`

  Leaks the mutex guard and returns a mutable reference to the data

  protected by the mutex.

  

  This will leave the `Mutex` in a locked state.

#### Trait Implementations

##### `impl<R: RawMutex + 'a, T: fmt::Debug + ?Sized + 'a> Debug for MutexGuard<'a, R, T>`

- <span id="mutexguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, T: ?Sized + 'a> Deref for MutexGuard<'a, R, T>`

- <span id="mutexguard-deref-type-target"></span>`type Target = T`

- <span id="mutexguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawMutex + 'a, T: ?Sized + 'a> DerefMut for MutexGuard<'a, R, T>`

- <span id="mutexguard-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<R: RawMutex + 'a, T: fmt::Display + ?Sized + 'a> Display for MutexGuard<'a, R, T>`

- <span id="mutexguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, T: ?Sized + 'a> Drop for MutexGuard<'a, R, T>`

- <span id="mutexguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for MutexGuard<'a, R, T>`

- <span id="mutexguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawMutex + Sync + 'a, T: ?Sized + Sync + 'a> Sync for MutexGuard<'a, R, T>`

### `MappedMutexGuard<'a, R: RawMutex, T: ?Sized>`

```rust
struct MappedMutexGuard<'a, R: RawMutex, T: ?Sized> {
    raw: &'a R,
    data: *mut T,
    marker: core::marker::PhantomData<&'a mut T>,
}
```

An RAII mutex guard returned by `MutexGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedMutexGuard` and `MutexGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

#### Implementations

- <span id="mappedmutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedMutexGuard<'a, R, U>` — [`MappedMutexGuard`](../index.md#mappedmutexguard)

  Makes a new `MappedMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedMutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedmutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, Self>` — [`MappedMutexGuard`](../index.md#mappedmutexguard)

  Attempts to make a new `MappedMutexGuard` for a component of the

  locked data. The original guard is returned if the closure returns `None`.

  

  This operation cannot fail as the `MappedMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedMutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedmutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, (Self, E)>` — [`MappedMutexGuard`](../index.md#mappedmutexguard)

  Attempts to make a new `MappedMutexGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `MappedMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedMutexGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

#### Trait Implementations

##### `impl<R: RawMutex + 'a, T: fmt::Debug + ?Sized + 'a> Debug for MappedMutexGuard<'a, R, T>`

- <span id="mappedmutexguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, T: ?Sized + 'a> Deref for MappedMutexGuard<'a, R, T>`

- <span id="mappedmutexguard-deref-type-target"></span>`type Target = T`

- <span id="mappedmutexguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawMutex + 'a, T: ?Sized + 'a> DerefMut for MappedMutexGuard<'a, R, T>`

- <span id="mappedmutexguard-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<R: RawMutex + 'a, T: fmt::Display + ?Sized + 'a> Display for MappedMutexGuard<'a, R, T>`

- <span id="mappedmutexguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex + 'a, T: ?Sized + 'a> Drop for MappedMutexGuard<'a, R, T>`

- <span id="mappedmutexguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for MappedMutexGuard<'a, R, T>`

- <span id="mappedmutexguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawMutex + 'a, T: ?Sized + Send + 'a> Send for MappedMutexGuard<'a, R, T>`

##### `impl<R: RawMutex + Sync + 'a, T: ?Sized + Sync + 'a> Sync for MappedMutexGuard<'a, R, T>`

## Traits

### `RawMutex`

```rust
trait RawMutex { ... }
```

Basic operations for a mutex.

Types implementing this trait can be used by `Mutex` to form a safe and
fully-functioning mutex type.

# Safety

Implementations of this trait must ensure that the mutex is actually
exclusive: a lock can't be acquired while the mutex is already locked.

#### Associated Types

- `type GuardMarker`

#### Associated Constants

- `const INIT: Self`

#### Required Methods

- `fn lock(&self)`

  Acquires this mutex, blocking the current thread until it is able to do so.

- `fn try_lock(&self) -> bool`

  Attempts to acquire this mutex without blocking. Returns `true`

- `fn unlock(&self)`

  Unlocks this mutex.

#### Provided Methods

- `fn is_locked(&self) -> bool`

  Checks whether the mutex is currently locked.

### `RawMutexFair`

```rust
trait RawMutexFair: RawMutex { ... }
```

Additional methods for mutexes which support fair unlocking.

Fair unlocking means that a lock is handed directly over to the next waiting
thread if there is one, without giving other threads the opportunity to
"steal" the lock in the meantime. This is typically slower than unfair
unlocking, but may be necessary in certain circumstances.

#### Required Methods

- `fn unlock_fair(&self)`

  Unlocks this mutex using a fair unlock protocol.

#### Provided Methods

- `fn bump(&self)`

  Temporarily yields the mutex to a waiting thread if there is one.

### `RawMutexTimed`

```rust
trait RawMutexTimed: RawMutex { ... }
```

Additional methods for mutexes which support locking with timeouts.

The `Duration` and `Instant` types are specified as associated types so that
this trait is usable even in `no_std` environments.

#### Associated Types

- `type Duration`

- `type Instant`

#### Required Methods

- `fn try_lock_for(&self, timeout: <Self as >::Duration) -> bool`

  Attempts to acquire this lock until a timeout is reached.

- `fn try_lock_until(&self, timeout: <Self as >::Instant) -> bool`

  Attempts to acquire this lock until a timeout is reached.

