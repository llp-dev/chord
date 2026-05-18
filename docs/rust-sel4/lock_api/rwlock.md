**lock_api > rwlock**

# Module: rwlock

## Contents

**Structs**

- [`MappedRwLockReadGuard`](#mappedrwlockreadguard) - An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a
- [`MappedRwLockWriteGuard`](#mappedrwlockwriteguard) - An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a
- [`RwLock`](#rwlock) - A reader-writer lock
- [`RwLockReadGuard`](#rwlockreadguard) - RAII structure used to release the shared read access of a lock when
- [`RwLockUpgradableReadGuard`](#rwlockupgradablereadguard) - RAII structure used to release the upgradable read access of a lock when
- [`RwLockWriteGuard`](#rwlockwriteguard) - RAII structure used to release the exclusive write access of a lock when

**Traits**

- [`RawRwLock`](#rawrwlock) - Basic operations for a reader-writer lock.
- [`RawRwLockDowngrade`](#rawrwlockdowngrade) - Additional methods for `RwLock`s which support atomically downgrading an
- [`RawRwLockFair`](#rawrwlockfair) - Additional methods for `RwLock`s which support fair unlocking.
- [`RawRwLockRecursive`](#rawrwlockrecursive) - Additional methods for `RwLock`s which support recursive read locks.
- [`RawRwLockRecursiveTimed`](#rawrwlockrecursivetimed) - Additional methods for `RwLock`s which support recursive read locks and timeouts.
- [`RawRwLockTimed`](#rawrwlocktimed) - Additional methods for `RwLock`s which support locking with timeouts.
- [`RawRwLockUpgrade`](#rawrwlockupgrade) - Additional methods for `RwLock`s which support atomically upgrading a shared
- [`RawRwLockUpgradeDowngrade`](#rawrwlockupgradedowngrade) - Additional methods for `RwLock`s which support upgradable locks and lock
- [`RawRwLockUpgradeFair`](#rawrwlockupgradefair) - Additional methods for `RwLock`s which support upgradable locks and fair
- [`RawRwLockUpgradeTimed`](#rawrwlockupgradetimed) - Additional methods for `RwLock`s which support upgradable locks and locking

---

## lock_api::rwlock::MappedRwLockReadGuard

*Struct*

An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedRwLockReadGuard` and `RwLockReadGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the `RwLock` using a fair unlock protocol.
- `fn map<U, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, R, U>` - Make a new `MappedRwLockReadGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, Self>` - Attempts to make  a new `MappedRwLockReadGuard` for a component of the
- `fn try_map_or_else<U, F, E>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, (Self, E)>` - Attempts to make  a new `MappedRwLockReadGuard` for a component of the

**Traits:** Sync, Send

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &T`



## lock_api::rwlock::MappedRwLockWriteGuard

*Struct*

An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a
subfield of the protected data.

The main difference between `MappedRwLockWriteGuard` and `RwLockWriteGuard` is that the
former doesn't support temporarily unlocking and re-locking, since that
could introduce soundness issues if the locked object is modified by another
thread.

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the `RwLock` using a fair unlock protocol.
- `fn map<U, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, R, U>` - Make a new `MappedRwLockWriteGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, Self>` - Attempts to make  a new `MappedRwLockWriteGuard` for a component of the
- `fn try_map_or_err<U, F, E>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, (Self, E)>` - Attempts to make  a new `MappedRwLockWriteGuard` for a component of the

**Traits:** Sync, Send

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`



## lock_api::rwlock::RawRwLock

*Trait*

Basic operations for a reader-writer lock.

Types implementing this trait can be used by `RwLock` to form a safe and
fully-functioning `RwLock` type.

# Safety

Implementations of this trait must ensure that the `RwLock` is actually
exclusive: an exclusive lock can't be acquired while an exclusive or shared
lock exists, and a shared lock can't be acquire while an exclusive lock
exists.

**Methods:**

- `INIT`: Initial value for an unlocked `RwLock`.
- `GuardMarker`: Marker type which determines whether a lock guard should be `Send`. Use
- `lock_shared`: Acquires a shared lock, blocking the current thread until it is able to do so.
- `try_lock_shared`: Attempts to acquire a shared lock without blocking.
- `unlock_shared`: Releases a shared lock.
- `lock_exclusive`: Acquires an exclusive lock, blocking the current thread until it is able to do so.
- `try_lock_exclusive`: Attempts to acquire an exclusive lock without blocking.
- `unlock_exclusive`: Releases an exclusive lock.
- `is_locked`: Checks if this `RwLock` is currently locked in any way.
- `is_locked_exclusive`: Check if this `RwLock` is currently exclusively locked.



## lock_api::rwlock::RawRwLockDowngrade

*Trait*

Additional methods for `RwLock`s which support atomically downgrading an
exclusive lock to a shared lock.

**Methods:**

- `downgrade`: Atomically downgrades an exclusive lock into a shared lock without



## lock_api::rwlock::RawRwLockFair

*Trait*

Additional methods for `RwLock`s which support fair unlocking.

Fair unlocking means that a lock is handed directly over to the next waiting
thread if there is one, without giving other threads the opportunity to
"steal" the lock in the meantime. This is typically slower than unfair
unlocking, but may be necessary in certain circumstances.

**Methods:**

- `unlock_shared_fair`: Releases a shared lock using a fair unlock protocol.
- `unlock_exclusive_fair`: Releases an exclusive lock using a fair unlock protocol.
- `bump_shared`: Temporarily yields a shared lock to a waiting thread if there is one.
- `bump_exclusive`: Temporarily yields an exclusive lock to a waiting thread if there is one.



## lock_api::rwlock::RawRwLockRecursive

*Trait*

Additional methods for `RwLock`s which support recursive read locks.

These are guaranteed to succeed without blocking if
another read lock is held at the time of the call. This allows a thread
to recursively lock a `RwLock`. However using this method can cause
writers to starve since readers no longer block if a writer is waiting
for the lock.

**Methods:**

- `lock_shared_recursive`: Acquires a shared lock without deadlocking in case of a recursive lock.
- `try_lock_shared_recursive`: Attempts to acquire a shared lock without deadlocking in case of a recursive lock.



## lock_api::rwlock::RawRwLockRecursiveTimed

*Trait*

Additional methods for `RwLock`s which support recursive read locks and timeouts.

**Methods:**

- `try_lock_shared_recursive_for`: Attempts to acquire a shared lock until a timeout is reached, without
- `try_lock_shared_recursive_until`: Attempts to acquire a shared lock until a timeout is reached, without



## lock_api::rwlock::RawRwLockTimed

*Trait*

Additional methods for `RwLock`s which support locking with timeouts.

The `Duration` and `Instant` types are specified as associated types so that
this trait is usable even in `no_std` environments.

**Methods:**

- `Duration`: Duration type used for `try_lock_for`.
- `Instant`: Instant type used for `try_lock_until`.
- `try_lock_shared_for`: Attempts to acquire a shared lock until a timeout is reached.
- `try_lock_shared_until`: Attempts to acquire a shared lock until a timeout is reached.
- `try_lock_exclusive_for`: Attempts to acquire an exclusive lock until a timeout is reached.
- `try_lock_exclusive_until`: Attempts to acquire an exclusive lock until a timeout is reached.



## lock_api::rwlock::RawRwLockUpgrade

*Trait*

Additional methods for `RwLock`s which support atomically upgrading a shared
lock to an exclusive lock.

This requires acquiring a special "upgradable read lock" instead of a
normal shared lock. There may only be one upgradable lock at any time,
otherwise deadlocks could occur when upgrading.

**Methods:**

- `lock_upgradable`: Acquires an upgradable lock, blocking the current thread until it is able to do so.
- `try_lock_upgradable`: Attempts to acquire an upgradable lock without blocking.
- `unlock_upgradable`: Releases an upgradable lock.
- `upgrade`: Upgrades an upgradable lock to an exclusive lock.
- `try_upgrade`: Attempts to upgrade an upgradable lock to an exclusive lock without



## lock_api::rwlock::RawRwLockUpgradeDowngrade

*Trait*

Additional methods for `RwLock`s which support upgradable locks and lock
downgrading.

**Methods:**

- `downgrade_upgradable`: Downgrades an upgradable lock to a shared lock.
- `downgrade_to_upgradable`: Downgrades an exclusive lock to an upgradable lock.



## lock_api::rwlock::RawRwLockUpgradeFair

*Trait*

Additional methods for `RwLock`s which support upgradable locks and fair
unlocking.

**Methods:**

- `unlock_upgradable_fair`: Releases an upgradable lock using a fair unlock protocol.
- `bump_upgradable`: Temporarily yields an upgradable lock to a waiting thread if there is one.



## lock_api::rwlock::RawRwLockUpgradeTimed

*Trait*

Additional methods for `RwLock`s which support upgradable locks and locking
with timeouts.

**Methods:**

- `try_lock_upgradable_for`: Attempts to acquire an upgradable lock until a timeout is reached.
- `try_lock_upgradable_until`: Attempts to acquire an upgradable lock until a timeout is reached.
- `try_upgrade_for`: Attempts to upgrade an upgradable lock to an exclusive lock until a
- `try_upgrade_until`: Attempts to upgrade an upgradable lock to an exclusive lock until a



## lock_api::rwlock::RwLock

*Struct*

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

**Generic Parameters:**
- R
- T

**Methods:**

- `fn try_upgradable_read_for(self: &Self, timeout: <R as >::Duration) -> Option<RwLockUpgradableReadGuard<R, T>>` - Attempts to acquire this `RwLock` with upgradable read access until a timeout
- `fn try_upgradable_read_until(self: &Self, timeout: <R as >::Instant) -> Option<RwLockUpgradableReadGuard<R, T>>` - Attempts to acquire this `RwLock` with upgradable read access until a timeout
- `fn try_read_recursive_for(self: &Self, timeout: <R as >::Duration) -> Option<RwLockReadGuard<R, T>>` - Attempts to acquire this `RwLock` with shared read access until a timeout
- `fn try_read_recursive_until(self: &Self, timeout: <R as >::Instant) -> Option<RwLockReadGuard<R, T>>` - Attempts to acquire this `RwLock` with shared read access until a timeout
- `fn try_read_for(self: &Self, timeout: <R as >::Duration) -> Option<RwLockReadGuard<R, T>>` - Attempts to acquire this `RwLock` with shared read access until a timeout
- `fn try_read_until(self: &Self, timeout: <R as >::Instant) -> Option<RwLockReadGuard<R, T>>` - Attempts to acquire this `RwLock` with shared read access until a timeout
- `fn try_write_for(self: &Self, timeout: <R as >::Duration) -> Option<RwLockWriteGuard<R, T>>` - Attempts to acquire this `RwLock` with exclusive write access until a
- `fn try_write_until(self: &Self, timeout: <R as >::Instant) -> Option<RwLockWriteGuard<R, T>>` - Attempts to acquire this `RwLock` with exclusive write access until a
- `fn new(val: T) -> RwLock<R, T>` - Creates a new instance of an `RwLock<T>` which is unlocked.
- `fn into_inner(self: Self) -> T` - Consumes this `RwLock`, returning the underlying data.
- `fn make_read_guard_unchecked(self: &Self) -> RwLockReadGuard<R, T>` - Creates a new `RwLockReadGuard` without checking if the lock is held.
- `fn make_write_guard_unchecked(self: &Self) -> RwLockWriteGuard<R, T>` - Creates a new `RwLockReadGuard` without checking if the lock is held.
- `fn read(self: &Self) -> RwLockReadGuard<R, T>` - Locks this `RwLock` with shared read access, blocking the current thread
- `fn try_read(self: &Self) -> Option<RwLockReadGuard<R, T>>` - Attempts to acquire this `RwLock` with shared read access.
- `fn write(self: &Self) -> RwLockWriteGuard<R, T>` - Locks this `RwLock` with exclusive write access, blocking the current
- `fn try_write(self: &Self) -> Option<RwLockWriteGuard<R, T>>` - Attempts to lock this `RwLock` with exclusive write access.
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.
- `fn is_locked(self: &Self) -> bool` - Checks whether this `RwLock` is currently locked in any way.
- `fn is_locked_exclusive(self: &Self) -> bool` - Check if this `RwLock` is currently exclusively locked.
- `fn force_unlock_read(self: &Self)` - Forcibly unlocks a read lock.
- `fn force_unlock_write(self: &Self)` - Forcibly unlocks a write lock.
- `fn raw(self: &Self) -> &R` - Returns the underlying raw reader-writer lock object.
- `fn data_ptr(self: &Self) -> *mut T` - Returns a raw pointer to the underlying data.
- `fn make_upgradable_guard_unchecked(self: &Self) -> RwLockUpgradableReadGuard<R, T>` - Creates a new `RwLockUpgradableReadGuard` without checking if the lock is held.
- `fn upgradable_read(self: &Self) -> RwLockUpgradableReadGuard<R, T>` - Locks this `RwLock` with upgradable read access, blocking the current thread
- `fn try_upgradable_read(self: &Self) -> Option<RwLockUpgradableReadGuard<R, T>>` - Attempts to acquire this `RwLock` with upgradable read access.
- `fn read_recursive(self: &Self) -> RwLockReadGuard<R, T>` - Locks this `RwLock` with shared read access, blocking the current thread
- `fn try_read_recursive(self: &Self) -> Option<RwLockReadGuard<R, T>>` - Attempts to acquire this `RwLock` with shared read access.
- `fn from_raw(raw_rwlock: R, val: T) -> RwLock<R, T>` - Creates a new new instance of an `RwLock<T>` based on a pre-existing
- `fn const_new(raw_rwlock: R, val: T) -> RwLock<R, T>` - Creates a new new instance of an `RwLock<T>` based on a pre-existing
- `fn force_unlock_read_fair(self: &Self)` - Forcibly unlocks a read lock using a fair unlock protocol.
- `fn force_unlock_write_fair(self: &Self)` - Forcibly unlocks a write lock using a fair unlock protocol.

**Traits:** Sync, Send

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(t: T) -> RwLock<R, T>`
- **Default**
  - `fn default() -> RwLock<R, T>`



## lock_api::rwlock::RwLockReadGuard

*Struct*

RAII structure used to release the shared read access of a lock when
dropped.

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the `RwLock` using a fair unlock protocol.
- `fn unlocked_fair<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the `RwLock` to execute the given function.
- `fn bump(s: & mut Self)` - Temporarily yields the `RwLock` to a waiting thread if there is one.
- `fn rwlock(s: &Self) -> &'a RwLock<R, T>` - Returns a reference to the original reader-writer lock object.
- `fn map<U, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, R, U>` - Make a new `MappedRwLockReadGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, Self>` - Attempts to make  a new `MappedRwLockReadGuard` for a component of the
- `fn try_map_or_err<U, F, E>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, R, U>, (Self, E)>` - Attempts to make  a new `MappedRwLockReadGuard` for a component of the
- `fn unlocked<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the `RwLock` to execute the given function.

**Traits:** Sync

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## lock_api::rwlock::RwLockUpgradableReadGuard

*Struct*

RAII structure used to release the upgradable read access of a lock when
dropped.

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn try_upgrade_for(s: Self, timeout: <R as >::Duration) -> Result<RwLockWriteGuard<'a, R, T>, Self>` - Tries to atomically upgrade an upgradable read lock into an exclusive
- `fn try_upgrade_until(s: Self, timeout: <R as >::Instant) -> Result<RwLockWriteGuard<'a, R, T>, Self>` - Tries to atomically upgrade an upgradable read lock into an exclusive
- `fn try_with_upgraded_for<Ret, F>(self: & mut Self, timeout: <R as >::Duration, f: F) -> Option<Ret>` - Tries to atomically upgrade an upgradable read lock into an exclusive
- `fn try_with_upgraded_until<Ret, F>(self: & mut Self, timeout: <R as >::Instant, f: F) -> Option<Ret>` - Tries to atomically upgrade an upgradable read lock into an exclusive
- `fn downgrade(s: Self) -> RwLockReadGuard<'a, R, T>` - Atomically downgrades an upgradable read lock lock into a shared read lock
- `fn with_upgraded<Ret, F>(self: & mut Self, f: F) -> Ret` - First, atomically upgrades an upgradable read lock lock into an exclusive write lock,
- `fn try_with_upgraded<Ret, F>(self: & mut Self, f: F) -> Option<Ret>` - First, tries to atomically upgrade an upgradable read lock into an exclusive write lock.
- `fn unlock_fair(s: Self)` - Unlocks the `RwLock` using a fair unlock protocol.
- `fn unlocked_fair<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the `RwLock` to execute the given function.
- `fn bump(s: & mut Self)` - Temporarily yields the `RwLock` to a waiting thread if there is one.
- `fn rwlock(s: &Self) -> &'a RwLock<R, T>` - Returns a reference to the original reader-writer lock object.
- `fn unlocked<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the `RwLock` to execute the given function.
- `fn upgrade(s: Self) -> RwLockWriteGuard<'a, R, T>` - Atomically upgrades an upgradable read lock lock into an exclusive write lock,
- `fn try_upgrade(s: Self) -> Result<RwLockWriteGuard<'a, R, T>, Self>` - Tries to atomically upgrade an upgradable read lock into an exclusive write lock.

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



## lock_api::rwlock::RwLockWriteGuard

*Struct*

RAII structure used to release the exclusive write access of a lock when
dropped.

**Generic Parameters:**
- 'a
- R
- T

**Methods:**

- `fn unlock_fair(s: Self)` - Unlocks the `RwLock` using a fair unlock protocol.
- `fn unlocked_fair<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the `RwLock` to execute the given function.
- `fn bump(s: & mut Self)` - Temporarily yields the `RwLock` to a waiting thread if there is one.
- `fn downgrade_to_upgradable(s: Self) -> RwLockUpgradableReadGuard<'a, R, T>` - Atomically downgrades a write lock into an upgradable read lock without allowing any
- `fn downgrade(s: Self) -> RwLockReadGuard<'a, R, T>` - Atomically downgrades a write lock into a read lock without allowing any
- `fn rwlock(s: &Self) -> &'a RwLock<R, T>` - Returns a reference to the original reader-writer lock object.
- `fn map<U, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, R, U>` - Make a new `MappedRwLockWriteGuard` for a component of the locked data.
- `fn try_map<U, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, Self>` - Attempts to make  a new `MappedRwLockWriteGuard` for a component of the
- `fn try_map_or_err<U, F, E>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, R, U>, (Self, E)>` - Attempts to make  a new `MappedRwLockWriteGuard` for a component of the
- `fn unlocked<F, U>(s: & mut Self, f: F) -> U` - Temporarily unlocks the `RwLock` to execute the given function.

**Traits:** Sync

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



