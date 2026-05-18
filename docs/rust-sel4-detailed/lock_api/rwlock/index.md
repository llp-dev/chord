*[lock_api](../index.md) / [rwlock](index.md)*

---

# Module `rwlock`

## Contents

- [Structs](#structs)
  - [`RwLock`](#rwlock)
  - [`RwLockReadGuard`](#rwlockreadguard)
  - [`RwLockWriteGuard`](#rwlockwriteguard)
  - [`RwLockUpgradableReadGuard`](#rwlockupgradablereadguard)
  - [`MappedRwLockReadGuard`](#mappedrwlockreadguard)
  - [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard)
- [Traits](#traits)
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
| [`RwLock`](#rwlock) | struct | A reader-writer lock |
| [`RwLockReadGuard`](#rwlockreadguard) | struct | RAII structure used to release the shared read access of a lock when dropped. |
| [`RwLockWriteGuard`](#rwlockwriteguard) | struct | RAII structure used to release the exclusive write access of a lock when dropped. |
| [`RwLockUpgradableReadGuard`](#rwlockupgradablereadguard) | struct | RAII structure used to release the upgradable read access of a lock when dropped. |
| [`MappedRwLockReadGuard`](#mappedrwlockreadguard) | struct | An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a subfield of the protected data. |
| [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard) | struct | An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a subfield of the protected data. |
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

## Structs

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

- <span id="rwlock-new"></span>`const fn new(val: T) -> RwLock<R, T>` — [`RwLock`](../index.md#rwlock)

  Creates a new instance of an `RwLock<T>` which is unlocked.

- <span id="rwlock-into-inner"></span>`fn into_inner(self) -> T`

  Consumes this `RwLock`, returning the underlying data.

#### Trait Implementations

##### `impl<R: RawRwLock, T: ?Sized + fmt::Debug> Debug for RwLock<R, T>`

- <span id="rwlock-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: RawRwLock, T: ?Sized + Default> Default for RwLock<R, T>`

- <span id="rwlock-default"></span>`fn default() -> RwLock<R, T>` — [`RwLock`](../index.md#rwlock)

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

- <span id="rwlockreadguard-rwlock"></span>`fn rwlock(s: &Self) -> &'a RwLock<R, T>` — [`RwLock`](../index.md#rwlock)

  Returns a reference to the original reader-writer lock object.

- <span id="rwlockreadguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, R, U>` — [`MappedRwLockReadGuard`](../index.md#mappedrwlockreadguard)

  Make a new `MappedRwLockReadGuard` for a component of the locked data.

  

  This operation cannot fail as the `RwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockReadGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockreadguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, Self>` — [`MappedRwLockReadGuard`](../index.md#mappedrwlockreadguard)

  Attempts to make  a new `MappedRwLockReadGuard` for a component of the

  locked data. Returns the original guard if the closure returns `None`.

  

  This operation cannot fail as the `RwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockReadGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockreadguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockReadGuard`](../index.md#mappedrwlockreadguard)

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

- <span id="rwlockwriteguard-rwlock"></span>`fn rwlock(s: &Self) -> &'a RwLock<R, T>` — [`RwLock`](../index.md#rwlock)

  Returns a reference to the original reader-writer lock object.

- <span id="rwlockwriteguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, R, U>` — [`MappedRwLockWriteGuard`](../index.md#mappedrwlockwriteguard)

  Make a new `MappedRwLockWriteGuard` for a component of the locked data.

  

  This operation cannot fail as the `RwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockWriteGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockwriteguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, Self>` — [`MappedRwLockWriteGuard`](../index.md#mappedrwlockwriteguard)

  Attempts to make  a new `MappedRwLockWriteGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `RwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `RwLockWriteGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="rwlockwriteguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockWriteGuard`](../index.md#mappedrwlockwriteguard)

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

- <span id="rwlockupgradablereadguard-rwlock"></span>`fn rwlock(s: &Self) -> &'a RwLock<R, T>` — [`RwLock`](../index.md#rwlock)

  Returns a reference to the original reader-writer lock object.

- <span id="rwlockupgradablereadguard-unlocked"></span>`fn unlocked<F, U>(s: &mut Self, f: F) -> U`

  Temporarily unlocks the `RwLock` to execute the given function.

  

  This is safe because `&mut` guarantees that there exist no other

  references to the data protected by the `RwLock`.

- <span id="rwlockupgradablereadguard-upgrade"></span>`fn upgrade(s: Self) -> RwLockWriteGuard<'a, R, T>` — [`RwLockWriteGuard`](../index.md#rwlockwriteguard)

  Atomically upgrades an upgradable read lock lock into an exclusive write lock,

  blocking the current thread until it can be acquired.

- <span id="rwlockupgradablereadguard-try-upgrade"></span>`fn try_upgrade(s: Self) -> Result<RwLockWriteGuard<'a, R, T>, Self>` — [`RwLockWriteGuard`](../index.md#rwlockwriteguard)

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

- <span id="mappedrwlockreadguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, R, U>` — [`MappedRwLockReadGuard`](../index.md#mappedrwlockreadguard)

  Make a new `MappedRwLockReadGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedRwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockReadGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockreadguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, Self>` — [`MappedRwLockReadGuard`](../index.md#mappedrwlockreadguard)

  Attempts to make  a new `MappedRwLockReadGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `MappedRwLockReadGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockReadGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockreadguard-try-map-or-else"></span>`fn try_map_or_else<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockReadGuard`](../index.md#mappedrwlockreadguard)

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

- <span id="mappedrwlockwriteguard-map"></span>`fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, R, U>` — [`MappedRwLockWriteGuard`](../index.md#mappedrwlockwriteguard)

  Make a new `MappedRwLockWriteGuard` for a component of the locked data.

  

  This operation cannot fail as the `MappedRwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockWriteGuard::map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockwriteguard-try-map"></span>`fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, Self>` — [`MappedRwLockWriteGuard`](../index.md#mappedrwlockwriteguard)

  Attempts to make  a new `MappedRwLockWriteGuard` for a component of the

  locked data. The original guard is return if the closure returns `None`.

  

  This operation cannot fail as the `MappedRwLockWriteGuard` passed

  in already locked the data.

  

  This is an associated function that needs to be

  used as `MappedRwLockWriteGuard::try_map(...)`. A method would interfere with methods of

  the same name on the contents of the locked data.

- <span id="mappedrwlockwriteguard-try-map-or-err"></span>`fn try_map_or_err<U: ?Sized, F, E>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, (Self, E)>` — [`MappedRwLockWriteGuard`](../index.md#mappedrwlockwriteguard)

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

