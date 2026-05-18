# Crate `lock_api`

This library provides type-safe and fully-featured [`Mutex`](#mutex) and [`RwLock`](#rwlock)
types which wrap a simple raw mutex or rwlock type. This has several
benefits: not only does it eliminate a large portion of the work in
implementing custom lock types, it also allows users to write code which is
generic with regards to different lock implementations.

Basic usage of this crate is very straightforward:

1. Create a raw lock type. This should only contain the lock state, not any
   data protected by the lock.
2. Implement the `RawMutex` trait for your custom lock type.
3. Export your mutex as a type alias for `lock_api::Mutex`, and
   your mutex guard as a type alias for `lock_api::MutexGuard`.
   See the [example](#example) below for details.

This process is similar for [`RwLock`](#rwlock)s, except that two guards need to be
exported instead of one. (Or 3 guards if your type supports upgradable read
locks, see [extension traits](#extension-traits) below for details)

# Example

```rust
use lock_api::{RawMutex, Mutex, GuardSend};
use std::sync::atomic::{AtomicBool, Ordering};

// 1. Define our raw lock type
pub struct RawSpinlock(AtomicBool);

// 2. Implement RawMutex for this type
unsafe impl RawMutex for RawSpinlock {
    const INIT: RawSpinlock = RawSpinlock(AtomicBool::new(false));

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {
        // Note: This isn't the best way of implementing a spinlock, but it
        // suffices for the sake of this example.
        while !self.try_lock() {}
    }

    fn try_lock(&self) -> bool {
        self.0
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        self.0.store(false, Ordering::Release);
    }
}

// 3. Export the wrappers. This are the types that your users will actually use.
pub type Spinlock<T> = lock_api::Mutex<RawSpinlock, T>;
pub type SpinlockGuard<'a, T> = lock_api::MutexGuard<'a, RawSpinlock, T>;
```

# Extension traits

In addition to basic locking & unlocking functionality, you have the option
of exposing additional functionality in your lock types by implementing
additional traits for it. Examples of extension features include:

- Fair unlocking (`RawMutexFair`, `RawRwLockFair`)
- Lock timeouts (`RawMutexTimed`, `RawRwLockTimed`)
- Downgradable write locks (`RawRwLockDowngradable`)
- Recursive read locks (`RawRwLockRecursive`)
- Upgradable read locks (`RawRwLockUpgrade`)

The `Mutex` and `RwLock` wrappers will automatically expose this additional
functionality if the raw lock type implements these extension traits.

# Cargo features

This crate supports three cargo features:

- `owning_ref`: Allows your lock types to be used with the `owning_ref` crate.
- `arc_lock`: Enables locking from an `Arc`. This enables types such as `ArcMutexGuard`. Note that this
  requires the `alloc` crate to be present.

## Contents

- [Modules](#modules)
  - [`mutex`](#mutex)
  - [`remutex`](#remutex)
  - [`rwlock`](#rwlock)
- [Structs](#structs)
  - [`GuardSend`](#guardsend)
  - [`GuardNoSend`](#guardnosend)
  - [`Mutex`](#mutex)
  - [`MutexGuard`](#mutexguard)
  - [`MappedMutexGuard`](#mappedmutexguard)
  - [`RawReentrantMutex`](#rawreentrantmutex)
  - [`ReentrantMutex`](#reentrantmutex)
  - [`ReentrantMutexGuard`](#reentrantmutexguard)
  - [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard)
  - [`RwLock`](#rwlock)
  - [`RwLockReadGuard`](#rwlockreadguard)
  - [`RwLockWriteGuard`](#rwlockwriteguard)
  - [`RwLockUpgradableReadGuard`](#rwlockupgradablereadguard)
  - [`MappedRwLockReadGuard`](#mappedrwlockreadguard)
  - [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)
- [Traits](#traits)
  - [`RawMutex`](#rawmutex)
  - [`RawMutexFair`](#rawmutexfair)
  - [`RawMutexTimed`](#rawmutextimed)
  - [`GetThreadId`](#getthreadid)
  - [`RawRwLock`](#rawrwlock)
  - [`RawRwLockFair`](#rawrwlockfair)
  - [`RawRwLockDowngrade`](#rawrwlockdowngrade)
  - [`RawRwLockTimed`](#rawrwlocktimed)
  - [`RawRwLockRecursive`](#rawrwlockrecursive)
  - [`RawRwLockRecursiveTimed`](#rawrwlockrecursivetimed)
  - [`RawRwLockUpgrade`](#rawrwlockupgrade)
  - [`RawRwLockUpgradeFair`](#rawrwlockupgradefair)
  - [`RawRwLockUpgradeDowngrade`](#rawrwlockupgradedowngrade)
  - [`RawRwLockUpgradeTimed`](#rawrwlockupgradetimed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`mutex`](#mutex) | mod |  |
| [`remutex`](#remutex) | mod |  |
| [`rwlock`](#rwlock) | mod |  |
| [`GuardSend`](#guardsend) | struct | Marker type which indicates that the Guard type for a lock is `Send`. |
| [`GuardNoSend`](#guardnosend) | struct | Marker type which indicates that the Guard type for a lock is not `Send`. |
| [`Mutex`](#mutex) | struct | A mutual exclusion primitive useful for protecting shared data |
| [`MutexGuard`](#mutexguard) | struct | An RAII implementation of a "scoped lock" of a mutex. |
| [`MappedMutexGuard`](#mappedmutexguard) | struct | An RAII mutex guard returned by `MutexGuard::map`, which can point to a subfield of the protected data. |
| [`RawReentrantMutex`](#rawreentrantmutex) | struct | A raw mutex type that wraps another raw mutex to provide reentrancy. |
| [`ReentrantMutex`](#reentrantmutex) | struct | A mutex which can be recursively locked by a single thread. |
| [`ReentrantMutexGuard`](#reentrantmutexguard) | struct | An RAII implementation of a "scoped lock" of a reentrant mutex. |
| [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard) | struct | An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can point to a subfield of the protected data. |
| [`RwLock`](#rwlock) | struct | A reader-writer lock |
| [`RwLockReadGuard`](#rwlockreadguard) | struct | RAII structure used to release the shared read access of a lock when dropped. |
| [`RwLockWriteGuard`](#rwlockwriteguard) | struct | RAII structure used to release the exclusive write access of a lock when dropped. |
| [`RwLockUpgradableReadGuard`](#rwlockupgradablereadguard) | struct | RAII structure used to release the upgradable read access of a lock when dropped. |
| [`MappedRwLockReadGuard`](#mappedrwlockreadguard) | struct | An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a subfield of the protected data. |
| [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard) | struct | An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a subfield of the protected data. |
| [`RawMutex`](#rawmutex) | trait | Basic operations for a mutex. |
| [`RawMutexFair`](#rawmutexfair) | trait | Additional methods for mutexes which support fair unlocking. |
| [`RawMutexTimed`](#rawmutextimed) | trait | Additional methods for mutexes which support locking with timeouts. |
| [`GetThreadId`](#getthreadid) | trait | Helper trait which returns a non-zero thread ID. |
| [`RawRwLock`](#rawrwlock) | trait | Basic operations for a reader-writer lock. |
| [`RawRwLockFair`](#rawrwlockfair) | trait | Additional methods for `RwLock`s which support fair unlocking. |
| [`RawRwLockDowngrade`](#rawrwlockdowngrade) | trait | Additional methods for `RwLock`s which support atomically downgrading an exclusive lock to a shared lock. |
| [`RawRwLockTimed`](#rawrwlocktimed) | trait | Additional methods for `RwLock`s which support locking with timeouts. |
| [`RawRwLockRecursive`](#rawrwlockrecursive) | trait | Additional methods for `RwLock`s which support recursive read locks. |
| [`RawRwLockRecursiveTimed`](#rawrwlockrecursivetimed) | trait | Additional methods for `RwLock`s which support recursive read locks and timeouts. |
| [`RawRwLockUpgrade`](#rawrwlockupgrade) | trait | Additional methods for `RwLock`s which support atomically upgrading a shared lock to an exclusive lock. |
| [`RawRwLockUpgradeFair`](#rawrwlockupgradefair) | trait | Additional methods for `RwLock`s which support upgradable locks and fair unlocking. |
| [`RawRwLockUpgradeDowngrade`](#rawrwlockupgradedowngrade) | trait | Additional methods for `RwLock`s which support upgradable locks and lock downgrading. |
| [`RawRwLockUpgradeTimed`](#rawrwlockupgradetimed) | trait | Additional methods for `RwLock`s which support upgradable locks and locking with timeouts. |

## Modules

- [`mutex`](mutex/index.md)
- [`remutex`](remutex/index.md)
- [`rwlock`](rwlock/index.md)

## Structs

### `GuardSend`

```rust
struct GuardSend(());
```

Marker type which indicates that the Guard type for a lock is `Send`.

### `GuardNoSend`

```rust
struct GuardNoSend(*mut ());
```

Marker type which indicates that the Guard type for a lock is not `Send`.

#### Trait Implementations

##### `impl Sync for GuardNoSend`

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

- <span id="mutex-new"></span>`const fn new(val: T) -> Mutex<R, T>` — [`Mutex`](#mutex)

  Creates a new mutex in an unlocked state ready for use.

- <span id="mutex-into-inner"></span>`fn into_inner(self) -> T`

  Consumes this mutex, returning the underlying data.

#### Trait Implementations

##### `impl<R: RawMutex, T: ?Sized + fmt::Debug> Debug for Mutex<R, T>`

- <span id="mutex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex, T: ?Sized + Default> Default for Mutex<R, T>`

- <span id="mutex-default"></span>`fn default() -> Mutex<R, T>` — [`Mutex`](#mutex)

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

- <span id="mutexguard-mutex"></span>`fn mutex(s: &Self) -> &'a Mutex<R, T>` — [`Mutex`](#mutex)

  Returns a reference to the original `Mutex` object.

- <span id="mutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedMutexGuard<'a, R, U>` — [`MappedMutexGuard`](#mappedmutexguard)

  Makes a new `MappedMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `MutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, Self>` — [`MappedMutexGuard`](#mappedmutexguard)

  Attempts to make a new `MappedMutexGuard` for a component of the

  locked data. The original guard is returned if the closure returns `None`.

  

  This operation cannot fail as the `MutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, (Self, E)>` — [`MappedMutexGuard`](#mappedmutexguard)

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

- <span id="mappedmutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedMutexGuard<'a, R, U>` — [`MappedMutexGuard`](#mappedmutexguard)

  Makes a new `MappedMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedMutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedmutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, Self>` — [`MappedMutexGuard`](#mappedmutexguard)

  Attempts to make a new `MappedMutexGuard` for a component of the

  locked data. The original guard is returned if the closure returns `None`.

  

  This operation cannot fail as the `MappedMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedMutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedmutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedMutexGuard<'a, R, U>, (Self, E)>` — [`MappedMutexGuard`](#mappedmutexguard)

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

Although this has the same methods as the [`RawMutex`](#rawmutex) trait, it does
not implement it, and should not be used in the same way, since this
mutex can successfully acquire a lock multiple times in the same thread.
Only use this when you know you want a raw mutex that can be locked
reentrantly; you probably want [`ReentrantMutex`](#reentrantmutex) instead.

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

- <span id="reentrantmutex-new"></span>`const fn new(val: T) -> ReentrantMutex<R, G, T>` — [`ReentrantMutex`](#reentrantmutex)

  Creates a new reentrant mutex in an unlocked state ready for use.

- <span id="reentrantmutex-into-inner"></span>`fn into_inner(self) -> T`

  Consumes this mutex, returning the underlying data.

#### Trait Implementations

##### `impl<R: RawMutex, G: GetThreadId, T: ?Sized + fmt::Debug> Debug for ReentrantMutex<R, G, T>`

- <span id="reentrantmutex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawMutex, G: GetThreadId, T: ?Sized + Default> Default for ReentrantMutex<R, G, T>`

- <span id="reentrantmutex-default"></span>`fn default() -> ReentrantMutex<R, G, T>` — [`ReentrantMutex`](#reentrantmutex)

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

- <span id="reentrantmutexguard-remutex"></span>`fn remutex(s: &Self) -> &'a ReentrantMutex<R, G, T>` — [`ReentrantMutex`](#reentrantmutex)

  Returns a reference to the original `ReentrantMutex` object.

- <span id="reentrantmutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedReentrantMutexGuard<'a, R, G, U>` — [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard)

  Makes a new `MappedReentrantMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `ReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `ReentrantMutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="reentrantmutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, Self>` — [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard)

  Attempts to make  a new `MappedReentrantMutexGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `ReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `ReentrantMutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="reentrantmutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, (Self, E)>` — [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard)

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

- <span id="mappedreentrantmutexguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedReentrantMutexGuard<'a, R, G, U>` — [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard)

  Makes a new `MappedReentrantMutexGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedReentrantMutexGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedreentrantmutexguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, Self>` — [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard)

  Attempts to make  a new `MappedReentrantMutexGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `MappedReentrantMutexGuard` passed

  in already locked the mutex.

  

  This is an associated function that needs to be

  used as `MappedReentrantMutexGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedreentrantmutexguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedReentrantMutexGuard<'a, R, G, U>, (Self, E)>` — [`MappedReentrantMutexGuard`](#mappedreentrantmutexguard)

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

### `RwLock<R, T: ?Sized>`

```rust
struct RwLock<R, T: ?Sized> {
    raw: R,
    data: core::cell::UnsafeCell<T>,
}
```

A reader-writer lock

This type of lock allows a number of readers or at most one writer at any
point in time. The write portion of this lock typically allows modification
of the underlying data (exclusive access) and the read portion of this lock
typically allows for read-only access (shared access).

The type parameter `T` represents the data that this lock protects. It is
required that `T` satisfies `Send` to be shared across threads and `Sync` to
allow concurrent access through readers. The RAII guards returned from the
locking methods implement `Deref` (and `DerefMut` for the `write` methods)
to allow access to the contained of the lock.

#### Implementations

- <span id="rwlock-new"></span>`const fn new(val: T) -> RwLock<R, T>` — [`RwLock`](#rwlock)

  Creates a new instance of an `RwLock<T>` which is unlocked.

- <span id="rwlock-into-inner"></span>`fn into_inner(self) -> T`

  Consumes this `RwLock`, returning the underlying data.

#### Trait Implementations

##### `impl<R: RawRwLock, T: ?Sized + fmt::Debug> Debug for RwLock<R, T>`

- <span id="rwlock-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock, T: ?Sized + Default> Default for RwLock<R, T>`

- <span id="rwlock-default"></span>`fn default() -> RwLock<R, T>` — [`RwLock`](#rwlock)

##### `impl<R: RawRwLock + Send, T: ?Sized + Send> Send for RwLock<R, T>`

##### `impl<R: RawRwLock + Sync, T: ?Sized + Send + Sync> Sync for RwLock<R, T>`

### `RwLockReadGuard<'a, R: RawRwLock, T: ?Sized>`

```rust
struct RwLockReadGuard<'a, R: RawRwLock, T: ?Sized> {
    rwlock: &'a RwLock<R, T>,
    marker: core::marker::PhantomData<(&'a T, <R as >::GuardMarker)>,
}
```

RAII structure used to release the shared read access of a lock when
dropped.

#### Implementations

- <span id="rwlockreadguard-rwlock"></span>`fn rwlock(s: &Self) -> &'a RwLock<R, T>` — [`RwLock`](#rwlock)

  Returns a reference to the original reader-writer lock object.

- <span id="rwlockreadguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, R, U>` — [`MappedRwLockReadGuard`](#mappedrwlockreadguard)

  Make a new `MappedRwLockReadGuard` for a component of the locked data.

  

  This operation cannot fail as the `RwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockReadGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockreadguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, Self>` — [`MappedRwLockReadGuard`](#mappedrwlockreadguard)

  Attempts to make  a new `MappedRwLockReadGuard` for a component of the

  locked data. Returns the original guard if the closure returns `None`.

  

  This operation cannot fail as the `RwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockReadGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockreadguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockReadGuard`](#mappedrwlockreadguard)

  Attempts to make  a new `MappedRwLockReadGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `RwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockReadGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockreadguard-unlocked"></span>`fn unlocked<F, U>(s: &mut Self, f: F) -> U`

  Temporarily unlocks the `RwLock` to execute the given function.

  

  This is safe because `&mut` guarantees that there exist no other

  references to the data protected by the `RwLock`.

#### Trait Implementations

##### `impl<R: RawRwLock + 'a, T: fmt::Debug + ?Sized + 'a> Debug for RwLockReadGuard<'a, R, T>`

- <span id="rwlockreadguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Deref for RwLockReadGuard<'a, R, T>`

- <span id="rwlockreadguard-deref-type-target"></span>`type Target = T`

- <span id="rwlockreadguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawRwLock + 'a, T: fmt::Display + ?Sized + 'a> Display for RwLockReadGuard<'a, R, T>`

- <span id="rwlockreadguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Drop for RwLockReadGuard<'a, R, T>`

- <span id="rwlockreadguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for RwLockReadGuard<'a, R, T>`

- <span id="rwlockreadguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawRwLock + Sync, T: Sync + ?Sized> Sync for RwLockReadGuard<'_, R, T>`

### `RwLockWriteGuard<'a, R: RawRwLock, T: ?Sized>`

```rust
struct RwLockWriteGuard<'a, R: RawRwLock, T: ?Sized> {
    rwlock: &'a RwLock<R, T>,
    marker: core::marker::PhantomData<(&'a mut T, <R as >::GuardMarker)>,
}
```

RAII structure used to release the exclusive write access of a lock when
dropped.

#### Implementations

- <span id="rwlockwriteguard-rwlock"></span>`fn rwlock(s: &Self) -> &'a RwLock<R, T>` — [`RwLock`](#rwlock)

  Returns a reference to the original reader-writer lock object.

- <span id="rwlockwriteguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, R, U>` — [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)

  Make a new `MappedRwLockWriteGuard` for a component of the locked data.

  

  This operation cannot fail as the `RwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockWriteGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockwriteguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, Self>` — [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)

  Attempts to make  a new `MappedRwLockWriteGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `RwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockWriteGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockwriteguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)

  Attempts to make  a new `MappedRwLockWriteGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `RwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockWriteGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockwriteguard-unlocked"></span>`fn unlocked<F, U>(s: &mut Self, f: F) -> U`

  Temporarily unlocks the `RwLock` to execute the given function.

  

  This is safe because `&mut` guarantees that there exist no other

  references to the data protected by the `RwLock`.

#### Trait Implementations

##### `impl<R: RawRwLock + 'a, T: fmt::Debug + ?Sized + 'a> Debug for RwLockWriteGuard<'a, R, T>`

- <span id="rwlockwriteguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Deref for RwLockWriteGuard<'a, R, T>`

- <span id="rwlockwriteguard-deref-type-target"></span>`type Target = T`

- <span id="rwlockwriteguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> DerefMut for RwLockWriteGuard<'a, R, T>`

- <span id="rwlockwriteguard-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<R: RawRwLock + 'a, T: fmt::Display + ?Sized + 'a> Display for RwLockWriteGuard<'a, R, T>`

- <span id="rwlockwriteguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Drop for RwLockWriteGuard<'a, R, T>`

- <span id="rwlockwriteguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for RwLockWriteGuard<'a, R, T>`

- <span id="rwlockwriteguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawRwLock + Sync, T: Sync + ?Sized> Sync for RwLockWriteGuard<'_, R, T>`

### `RwLockUpgradableReadGuard<'a, R: RawRwLockUpgrade, T: ?Sized>`

```rust
struct RwLockUpgradableReadGuard<'a, R: RawRwLockUpgrade, T: ?Sized> {
    rwlock: &'a RwLock<R, T>,
    marker: core::marker::PhantomData<(&'a T, <R as >::GuardMarker)>,
}
```

RAII structure used to release the upgradable read access of a lock when
dropped.

#### Implementations

- <span id="rwlockupgradablereadguard-rwlock"></span>`fn rwlock(s: &Self) -> &'a RwLock<R, T>` — [`RwLock`](#rwlock)

  Returns a reference to the original reader-writer lock object.

- <span id="rwlockupgradablereadguard-unlocked"></span>`fn unlocked<F, U>(s: &mut Self, f: F) -> U`

  Temporarily unlocks the `RwLock` to execute the given function.

  

  This is safe because `&mut` guarantees that there exist no other

  references to the data protected by the `RwLock`.

- <span id="rwlockupgradablereadguard-upgrade"></span>`fn upgrade(s: Self) -> RwLockWriteGuard<'a, R, T>` — [`RwLockWriteGuard`](#rwlockwriteguard)

  Atomically upgrades an upgradable read lock lock into an exclusive write lock,

  blocking the current thread until it can be acquired.

- <span id="rwlockupgradablereadguard-try-upgrade"></span>`fn try_upgrade(s: Self) -> Result<RwLockWriteGuard<'a, R, T>, Self>` — [`RwLockWriteGuard`](#rwlockwriteguard)

  Tries to atomically upgrade an upgradable read lock into an exclusive write lock.

  

  If the access could not be granted at this time, then the current guard is returned.

#### Trait Implementations

##### `impl<R: RawRwLockUpgrade + 'a, T: fmt::Debug + ?Sized + 'a> Debug for RwLockUpgradableReadGuard<'a, R, T>`

- <span id="rwlockupgradablereadguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLockUpgrade + 'a, T: ?Sized + 'a> Deref for RwLockUpgradableReadGuard<'a, R, T>`

- <span id="rwlockupgradablereadguard-deref-type-target"></span>`type Target = T`

- <span id="rwlockupgradablereadguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawRwLockUpgrade + 'a, T: fmt::Display + ?Sized + 'a> Display for RwLockUpgradableReadGuard<'a, R, T>`

- <span id="rwlockupgradablereadguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLockUpgrade + 'a, T: ?Sized + 'a> Drop for RwLockUpgradableReadGuard<'a, R, T>`

- <span id="rwlockupgradablereadguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for RwLockUpgradableReadGuard<'a, R, T>`

- <span id="rwlockupgradablereadguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawRwLockUpgrade + 'a, T: ?Sized + Sync + 'a> Sync for RwLockUpgradableReadGuard<'a, R, T>`

### `MappedRwLockReadGuard<'a, R: RawRwLock, T: ?Sized>`

```rust
struct MappedRwLockReadGuard<'a, R: RawRwLock, T: ?Sized> {
    raw: &'a R,
    data: *const T,
    marker: core::marker::PhantomData<&'a T>,
}
```

An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedRwLockReadGuard` and `RwLockReadGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

#### Implementations

- <span id="mappedrwlockreadguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, R, U>` — [`MappedRwLockReadGuard`](#mappedrwlockreadguard)

  Make a new `MappedRwLockReadGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedRwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockReadGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockreadguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, Self>` — [`MappedRwLockReadGuard`](#mappedrwlockreadguard)

  Attempts to make  a new `MappedRwLockReadGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `MappedRwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockReadGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockreadguard-try-map-or-else"></span>`fn try_map_or_else<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockReadGuard`](#mappedrwlockreadguard)

  Attempts to make  a new `MappedRwLockReadGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `MappedRwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockReadGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

#### Trait Implementations

##### `impl<R: RawRwLock + 'a, T: fmt::Debug + ?Sized + 'a> Debug for MappedRwLockReadGuard<'a, R, T>`

- <span id="mappedrwlockreadguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Deref for MappedRwLockReadGuard<'a, R, T>`

- <span id="mappedrwlockreadguard-deref-type-target"></span>`type Target = T`

- <span id="mappedrwlockreadguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawRwLock + 'a, T: fmt::Display + ?Sized + 'a> Display for MappedRwLockReadGuard<'a, R, T>`

- <span id="mappedrwlockreadguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Drop for MappedRwLockReadGuard<'a, R, T>`

- <span id="mappedrwlockreadguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for MappedRwLockReadGuard<'a, R, T>`

- <span id="mappedrwlockreadguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawRwLock + 'a, T: ?Sized + Sync + 'a> Send for MappedRwLockReadGuard<'a, R, T>`

##### `impl<R: RawRwLock + 'a, T: ?Sized + Sync + 'a> Sync for MappedRwLockReadGuard<'a, R, T>`

### `MappedRwLockWriteGuard<'a, R: RawRwLock, T: ?Sized>`

```rust
struct MappedRwLockWriteGuard<'a, R: RawRwLock, T: ?Sized> {
    raw: &'a R,
    data: *mut T,
    marker: core::marker::PhantomData<&'a mut T>,
}
```

An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedRwLockWriteGuard` and `RwLockWriteGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

#### Implementations

- <span id="mappedrwlockwriteguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, R, U>` — [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)

  Make a new `MappedRwLockWriteGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedRwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockWriteGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockwriteguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, Self>` — [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)

  Attempts to make  a new `MappedRwLockWriteGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `MappedRwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockWriteGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockwriteguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)

  Attempts to make  a new `MappedRwLockWriteGuard` for a component of the

  locked data. The original guard is returned alongside arbitrary user data

  if the closure returns `Err`.

  

  This operation cannot fail as the `MappedRwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockWriteGuard::try_map_or_err(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

#### Trait Implementations

##### `impl<R: RawRwLock + 'a, T: fmt::Debug + ?Sized + 'a> Debug for MappedRwLockWriteGuard<'a, R, T>`

- <span id="mappedrwlockwriteguard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Deref for MappedRwLockWriteGuard<'a, R, T>`

- <span id="mappedrwlockwriteguard-deref-type-target"></span>`type Target = T`

- <span id="mappedrwlockwriteguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> DerefMut for MappedRwLockWriteGuard<'a, R, T>`

- <span id="mappedrwlockwriteguard-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<R: RawRwLock + 'a, T: fmt::Display + ?Sized + 'a> Display for MappedRwLockWriteGuard<'a, R, T>`

- <span id="mappedrwlockwriteguard-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock + 'a, T: ?Sized + 'a> Drop for MappedRwLockWriteGuard<'a, R, T>`

- <span id="mappedrwlockwriteguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for MappedRwLockWriteGuard<'a, R, T>`

- <span id="mappedrwlockwriteguard-receiver-type-target"></span>`type Target = T`

##### `impl<R: RawRwLock + 'a, T: ?Sized + Send + 'a> Send for MappedRwLockWriteGuard<'a, R, T>`

##### `impl<R: RawRwLock + 'a, T: ?Sized + Sync + 'a> Sync for MappedRwLockWriteGuard<'a, R, T>`

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

### `RawRwLock`

```rust
trait RawRwLock { ... }
```

Basic operations for a reader-writer lock.

Types implementing this trait can be used by `RwLock` to form a safe and
fully-functioning `RwLock` type.

# Safety

Implementations of this trait must ensure that the `RwLock` is actually
exclusive: an exclusive lock can't be acquired while an exclusive or shared
lock exists, and a shared lock can't be acquire while an exclusive lock
exists.

#### Associated Types

- `type GuardMarker`

#### Associated Constants

- `const INIT: Self`

#### Required Methods

- `fn lock_shared(&self)`

  Acquires a shared lock, blocking the current thread until it is able to do so.

- `fn try_lock_shared(&self) -> bool`

  Attempts to acquire a shared lock without blocking.

- `fn unlock_shared(&self)`

  Releases a shared lock.

- `fn lock_exclusive(&self)`

  Acquires an exclusive lock, blocking the current thread until it is able to do so.

- `fn try_lock_exclusive(&self) -> bool`

  Attempts to acquire an exclusive lock without blocking.

- `fn unlock_exclusive(&self)`

  Releases an exclusive lock.

#### Provided Methods

- `fn is_locked(&self) -> bool`

  Checks if this `RwLock` is currently locked in any way.

- `fn is_locked_exclusive(&self) -> bool`

  Check if this `RwLock` is currently exclusively locked.

### `RawRwLockFair`

```rust
trait RawRwLockFair: RawRwLock { ... }
```

Additional methods for `RwLock`s which support fair unlocking.

Fair unlocking means that a lock is handed directly over to the next waiting
thread if there is one, without giving other threads the opportunity to
"steal" the lock in the meantime. This is typically slower than unfair
unlocking, but may be necessary in certain circumstances.

#### Required Methods

- `fn unlock_shared_fair(&self)`

  Releases a shared lock using a fair unlock protocol.

- `fn unlock_exclusive_fair(&self)`

  Releases an exclusive lock using a fair unlock protocol.

#### Provided Methods

- `fn bump_shared(&self)`

  Temporarily yields a shared lock to a waiting thread if there is one.

- `fn bump_exclusive(&self)`

  Temporarily yields an exclusive lock to a waiting thread if there is one.

### `RawRwLockDowngrade`

```rust
trait RawRwLockDowngrade: RawRwLock { ... }
```

Additional methods for `RwLock`s which support atomically downgrading an
exclusive lock to a shared lock.

#### Required Methods

- `fn downgrade(&self)`

  Atomically downgrades an exclusive lock into a shared lock without

### `RawRwLockTimed`

```rust
trait RawRwLockTimed: RawRwLock { ... }
```

Additional methods for `RwLock`s which support locking with timeouts.

The `Duration` and `Instant` types are specified as associated types so that
this trait is usable even in `no_std` environments.

#### Associated Types

- `type Duration`

- `type Instant`

#### Required Methods

- `fn try_lock_shared_for(&self, timeout: <Self as >::Duration) -> bool`

  Attempts to acquire a shared lock until a timeout is reached.

- `fn try_lock_shared_until(&self, timeout: <Self as >::Instant) -> bool`

  Attempts to acquire a shared lock until a timeout is reached.

- `fn try_lock_exclusive_for(&self, timeout: <Self as >::Duration) -> bool`

  Attempts to acquire an exclusive lock until a timeout is reached.

- `fn try_lock_exclusive_until(&self, timeout: <Self as >::Instant) -> bool`

  Attempts to acquire an exclusive lock until a timeout is reached.

### `RawRwLockRecursive`

```rust
trait RawRwLockRecursive: RawRwLock { ... }
```

Additional methods for `RwLock`s which support recursive read locks.

These are guaranteed to succeed without blocking if
another read lock is held at the time of the call. This allows a thread
to recursively lock a `RwLock`. However using this method can cause
writers to starve since readers no longer block if a writer is waiting
for the lock.

#### Required Methods

- `fn lock_shared_recursive(&self)`

  Acquires a shared lock without deadlocking in case of a recursive lock.

- `fn try_lock_shared_recursive(&self) -> bool`

  Attempts to acquire a shared lock without deadlocking in case of a recursive lock.

### `RawRwLockRecursiveTimed`

```rust
trait RawRwLockRecursiveTimed: RawRwLockRecursive + RawRwLockTimed { ... }
```

Additional methods for `RwLock`s which support recursive read locks and timeouts.

#### Required Methods

- `fn try_lock_shared_recursive_for(&self, timeout: <Self as >::Duration) -> bool`

  Attempts to acquire a shared lock until a timeout is reached, without

- `fn try_lock_shared_recursive_until(&self, timeout: <Self as >::Instant) -> bool`

  Attempts to acquire a shared lock until a timeout is reached, without

### `RawRwLockUpgrade`

```rust
trait RawRwLockUpgrade: RawRwLock { ... }
```

Additional methods for `RwLock`s which support atomically upgrading a shared
lock to an exclusive lock.

This requires acquiring a special "upgradable read lock" instead of a
normal shared lock. There may only be one upgradable lock at any time,
otherwise deadlocks could occur when upgrading.

#### Required Methods

- `fn lock_upgradable(&self)`

  Acquires an upgradable lock, blocking the current thread until it is able to do so.

- `fn try_lock_upgradable(&self) -> bool`

  Attempts to acquire an upgradable lock without blocking.

- `fn unlock_upgradable(&self)`

  Releases an upgradable lock.

- `fn upgrade(&self)`

  Upgrades an upgradable lock to an exclusive lock.

- `fn try_upgrade(&self) -> bool`

  Attempts to upgrade an upgradable lock to an exclusive lock without

### `RawRwLockUpgradeFair`

```rust
trait RawRwLockUpgradeFair: RawRwLockUpgrade + RawRwLockFair { ... }
```

Additional methods for `RwLock`s which support upgradable locks and fair
unlocking.

#### Required Methods

- `fn unlock_upgradable_fair(&self)`

  Releases an upgradable lock using a fair unlock protocol.

#### Provided Methods

- `fn bump_upgradable(&self)`

  Temporarily yields an upgradable lock to a waiting thread if there is one.

### `RawRwLockUpgradeDowngrade`

```rust
trait RawRwLockUpgradeDowngrade: RawRwLockUpgrade + RawRwLockDowngrade { ... }
```

Additional methods for `RwLock`s which support upgradable locks and lock
downgrading.

#### Required Methods

- `fn downgrade_upgradable(&self)`

  Downgrades an upgradable lock to a shared lock.

- `fn downgrade_to_upgradable(&self)`

  Downgrades an exclusive lock to an upgradable lock.

### `RawRwLockUpgradeTimed`

```rust
trait RawRwLockUpgradeTimed: RawRwLockUpgrade + RawRwLockTimed { ... }
```

Additional methods for `RwLock`s which support upgradable locks and locking
with timeouts.

#### Required Methods

- `fn try_lock_upgradable_for(&self, timeout: <Self as >::Duration) -> bool`

  Attempts to acquire an upgradable lock until a timeout is reached.

- `fn try_lock_upgradable_until(&self, timeout: <Self as >::Instant) -> bool`

  Attempts to acquire an upgradable lock until a timeout is reached.

- `fn try_upgrade_for(&self, timeout: <Self as >::Duration) -> bool`

  Attempts to upgrade an upgradable lock to an exclusive lock until a

- `fn try_upgrade_until(&self, timeout: <Self as >::Instant) -> bool`

  Attempts to upgrade an upgradable lock to an exclusive lock until a

