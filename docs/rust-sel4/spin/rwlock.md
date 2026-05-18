**spin > rwlock**

# Module: rwlock

## Contents

**Structs**

- [`RwLock`](#rwlock) - A lock that provides data access to either one writer or many readers.
- [`RwLockReadGuard`](#rwlockreadguard) - A guard that provides immutable data access.
- [`RwLockUpgradableGuard`](#rwlockupgradableguard) - A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`].
- [`RwLockWriteGuard`](#rwlockwriteguard) - A guard that provides mutable data access.

**Functions**

- [`compare_exchange`](#compare_exchange)

**Constants**

- [`READER`](#reader)
- [`UPGRADED`](#upgraded)
- [`WRITER`](#writer)

---

## spin::rwlock::READER

*Constant*: `usize`



## spin::rwlock::RwLock

*Struct*

A lock that provides data access to either one writer or many readers.

This lock behaves in a similar manner to its namesake `std::sync::RwLock` but uses
spinning for synchronisation instead. Unlike its namespace, this lock does not
track lock poisoning.

This type of lock allows a number of readers or at most one writer at any
point in time. The write portion of this lock typically allows modification
of the underlying data (exclusive access) and the read portion of this lock
typically allows for read-only access (shared access).

The type parameter `T` represents the data that this lock protects. It is
required that `T` satisfies `Send` to be shared across tasks and `Sync` to
allow concurrent access through readers. The RAII guards returned from the
locking methods implement `Deref` (and `DerefMut` for the `write` methods)
to allow access to the contained of the lock.

An [`RwLockUpgradableGuard`](RwLockUpgradableGuard) can be upgraded to a
writable guard through the [`RwLockUpgradableGuard::upgrade`](RwLockUpgradableGuard::upgrade)
[`RwLockUpgradableGuard::try_upgrade`](RwLockUpgradableGuard::try_upgrade) functions.
Writable or upgradeable guards can be downgraded through their respective `downgrade`
functions.

Based on Facebook's
[`folly/RWSpinLock.h`](https://github.com/facebook/folly/blob/a0394d84f2d5c3e50ebfd0566f9d3acb52cfab5a/folly/synchronization/RWSpinLock.h).
This implementation is unfair to writers - if the lock always has readers, then no writers will
ever get a chance. Using an upgradeable lock guard can *somewhat* alleviate this issue as no
new readers are allowed when an upgradeable guard is held, but upgradeable guards can be taken
when there are existing readers. However if the lock is that highly contended and writes are
crucial then this implementation may be a poor choice.

# Examples

```
use spin;

let lock = spin::RwLock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read();
    let r2 = lock.read();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write();
    *w += 1;
    assert_eq!(*w, 6);
} // write lock is dropped here
```

**Generic Parameters:**
- T
- R

**Fields:**
- `phantom: core::marker::PhantomData<R>`
- `lock: crate::atomic::AtomicUsize`
- `data: core::cell::UnsafeCell<T>`

**Methods:**

- `fn read(self: &Self) -> RwLockReadGuard<T>` - Locks this rwlock with shared read access, blocking the current thread
- `fn write(self: &Self) -> RwLockWriteGuard<T, R>` - Lock this rwlock with exclusive write access, blocking the current
- `fn upgradeable_read(self: &Self) -> RwLockUpgradableGuard<T, R>` - Obtain a readable lock guard that can later be upgraded to a writable lock guard.
- `fn new(data: T) -> Self` - Creates a new spinlock wrapping the supplied data.
- `fn into_inner(self: Self) -> T` - Consumes this `RwLock`, returning the underlying data.
- `fn as_mut_ptr(self: &Self) -> *mut T` - Returns a mutable pointer to the underying data.
- `fn acquire_reader(self: &Self) -> usize`
- `fn try_read(self: &Self) -> Option<RwLockReadGuard<T>>` - Attempt to acquire this lock with shared read access.
- `fn reader_count(self: &Self) -> usize` - Return the number of readers that currently hold the lock (including upgradable readers).
- `fn writer_count(self: &Self) -> usize` - Return the number of writers that currently hold the lock.
- `fn force_read_decrement(self: &Self)` - Force decrement the reader count.
- `fn force_write_unlock(self: &Self)` - Force unlock exclusive write access.
- `fn try_write_internal(self: &Self, strong: bool) -> Option<RwLockWriteGuard<T, R>>`
- `fn try_write(self: &Self) -> Option<RwLockWriteGuard<T, R>>` - Attempt to lock this rwlock with exclusive write access.
- `fn try_upgradeable_read(self: &Self) -> Option<RwLockUpgradableGuard<T, R>>` - Tries to obtain an upgradeable lock guard.
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.

**Traits:** Send, Sync

**Trait Implementations:**

- **RawRwLockDowngrade**
  - `fn downgrade(self: &Self)`
- **RawRwLock**
  - `fn lock_exclusive(self: &Self)`
  - `fn try_lock_exclusive(self: &Self) -> bool`
  - `fn unlock_exclusive(self: &Self)`
  - `fn lock_shared(self: &Self)`
  - `fn try_lock_shared(self: &Self) -> bool`
  - `fn unlock_shared(self: &Self)`
  - `fn is_locked(self: &Self) -> bool`
- **From**
  - `fn from(data: T) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **RawRwLockUpgrade**
  - `fn lock_upgradable(self: &Self)`
  - `fn try_lock_upgradable(self: &Self) -> bool`
  - `fn unlock_upgradable(self: &Self)`
  - `fn upgrade(self: &Self)`
  - `fn try_upgrade(self: &Self) -> bool`



## spin::rwlock::RwLockReadGuard

*Struct*

A guard that provides immutable data access.

When the guard falls out of scope it will decrement the read count,
potentially releasing the lock.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `lock: &'a crate::atomic::AtomicUsize`
- `data: *const T`

**Methods:**

- `fn leak(this: Self) -> &'rwlock T` - Leak the lock guard, yielding a reference to the underlying data.

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



## spin::rwlock::RwLockUpgradableGuard

*Struct*

A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`].

No writers or other upgradeable guards can exist while this is in scope. New reader
creation is prevented (to alleviate writer starvation) but there may be existing readers
when the lock is acquired.

When the guard falls out of scope it will release the lock.

**Generic Parameters:**
- 'a
- T
- R

**Fields:**
- `phantom: core::marker::PhantomData<R>`
- `inner: &'a RwLock<T, R>`
- `data: *const T`

**Methods:**

- `fn try_upgrade_internal(self: Self, strong: bool) -> Result<RwLockWriteGuard<'rwlock, T, R>, Self>`
- `fn try_upgrade(self: Self) -> Result<RwLockWriteGuard<'rwlock, T, R>, Self>` - Tries to upgrade an upgradeable lock guard to a writable lock guard.
- `fn downgrade(self: Self) -> RwLockReadGuard<'rwlock, T>` - Downgrades the upgradeable lock guard to a readable, shared lock guard. Cannot fail and is guaranteed not to spin.
- `fn leak(this: Self) -> &'rwlock T` - Leak the lock guard, yielding a reference to the underlying data.
- `fn upgrade(self: Self) -> RwLockWriteGuard<'rwlock, T, R>` - Upgrades an upgradeable lock guard to a writable lock guard.

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



## spin::rwlock::RwLockWriteGuard

*Struct*

A guard that provides mutable data access.

When the guard falls out of scope it will release the lock.

**Generic Parameters:**
- 'a
- T
- R

**Fields:**
- `phantom: core::marker::PhantomData<R>`
- `inner: &'a RwLock<T, R>`
- `data: *mut T`

**Methods:**

- `fn downgrade(self: Self) -> RwLockReadGuard<'rwlock, T>` - Downgrades the writable lock guard to a readable, shared lock guard. Cannot fail and is guaranteed not to spin.
- `fn downgrade_to_upgradeable(self: Self) -> RwLockUpgradableGuard<'rwlock, T, R>` - Downgrades the writable lock guard to an upgradable, shared lock guard. Cannot fail and is guaranteed not to spin.
- `fn leak(this: Self) -> &'rwlock  mut T` - Leak the lock guard, yielding a mutable reference to the underlying data.

**Traits:** Sync, Send

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## spin::rwlock::UPGRADED

*Constant*: `usize`



## spin::rwlock::WRITER

*Constant*: `usize`



## spin::rwlock::compare_exchange

*Function*

```rust
fn compare_exchange(atomic: &crate::atomic::AtomicUsize, current: usize, new: usize, success: crate::atomic::Ordering, failure: crate::atomic::Ordering, strong: bool) -> Result<usize, usize>
```



