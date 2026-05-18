*[one_shot_mutex](../index.md) / [sync](index.md)*

---

# Module `sync`

One-shot lock variants that implement `Sync`.

## Contents

- [Modules](#modules)
  - [`mutex`](#mutex)
  - [`rwlock`](#rwlock)
- [Structs](#structs)
  - [`RawOneShotMutex`](#rawoneshotmutex)
  - [`RawOneShotRwLock`](#rawoneshotrwlock)
- [Type Aliases](#type-aliases)
  - [`OneShotMutex`](#oneshotmutex)
  - [`OneShotMutexGuard`](#oneshotmutexguard)
  - [`OneShotRwLock`](#oneshotrwlock)
  - [`OneShotRwLockReadGuard`](#oneshotrwlockreadguard)
  - [`OneShotRwLockUpgradableReadGuard`](#oneshotrwlockupgradablereadguard)
  - [`OneShotRwLockWriteGuard`](#oneshotrwlockwriteguard)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`mutex`](#mutex) | mod |  |
| [`rwlock`](#rwlock) | mod |  |
| [`RawOneShotMutex`](#rawoneshotmutex) | struct |  |
| [`RawOneShotRwLock`](#rawoneshotrwlock) | struct |  |
| [`OneShotMutex`](#oneshotmutex) | type |  |
| [`OneShotMutexGuard`](#oneshotmutexguard) | type |  |
| [`OneShotRwLock`](#oneshotrwlock) | type |  |
| [`OneShotRwLockReadGuard`](#oneshotrwlockreadguard) | type |  |
| [`OneShotRwLockUpgradableReadGuard`](#oneshotrwlockupgradablereadguard) | type |  |
| [`OneShotRwLockWriteGuard`](#oneshotrwlockwriteguard) | type |  |

## Modules

- [`mutex`](mutex/index.md)
- [`rwlock`](rwlock/index.md)

## Structs

### `RawOneShotMutex`

```rust
struct RawOneShotMutex {
    lock: core::sync::atomic::AtomicBool,
}
```

A one-shot mutex that panics instead of (dead)locking on contention.

This mutex allows no contention and panics instead of blocking on [`lock`](#lock) if it is already locked.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.

This mutex should be used through [`OneShotMutex`](mutex/index.md).

# Examples

```rust
use one_shot_mutex::sync::OneShotMutex;

static X: OneShotMutex<i32> = OneShotMutex::new(42);

// This is equivalent to `X.try_lock().unwrap()`.
let x = X.lock();

// This panics instead of deadlocking.
// let x2 = X.lock();

// Once we unlock the mutex, we can lock it again.
drop(x);
let x = X.lock();
```

#### Implementations

- <span id="rawoneshotmutex-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Default for RawOneShotMutex`

- <span id="rawoneshotmutex-default"></span>`fn default() -> Self`

##### `impl RawMutex for RawOneShotMutex`

- <span id="rawoneshotmutex-rawmutex-const-init"></span>`const INIT: Self`

- <span id="rawoneshotmutex-rawmutex-type-guardmarker"></span>`type GuardMarker = GuardSend`

- <span id="rawoneshotmutex-rawmutex-lock"></span>`fn lock(&self)`

- <span id="rawoneshotmutex-rawmutex-try-lock"></span>`fn try_lock(&self) -> bool`

- <span id="rawoneshotmutex-rawmutex-unlock"></span>`unsafe fn unlock(&self)`

- <span id="rawoneshotmutex-rawmutex-is-locked"></span>`fn is_locked(&self) -> bool`

##### `impl RawMutexFair for RawOneShotMutex`

- <span id="rawoneshotmutex-rawmutexfair-unlock-fair"></span>`unsafe fn unlock_fair(&self)`

- <span id="rawoneshotmutex-rawmutexfair-bump"></span>`unsafe fn bump(&self)`

### `RawOneShotRwLock`

```rust
struct RawOneShotRwLock {
    lock: core::sync::atomic::AtomicUsize,
}
```

A one-shot readers-writer lock that panics instead of (dead)locking on contention.

This lock allows no contention and panics on `lock_shared`, `lock_exclusive`, `lock_upgradable`, and `upgrade` if it is already locked conflictingly.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.




# Examples

```rust
use one_shot_mutex::sync::OneShotRwLock;

static X: OneShotRwLock<i32> = OneShotRwLock::new(42);

// This is equivalent to `X.try_write().unwrap()`.
let x = X.write();

// This panics instead of deadlocking.
// let x2 = X.write();

// Once we unlock the mutex, we can lock it again.
drop(x);
let x = X.write();
```

#### Implementations

- <span id="rawoneshotrwlock-new"></span>`const fn new() -> Self`

- <span id="rawoneshotrwlock-is-locked-shared"></span>`fn is_locked_shared(&self) -> bool`

- <span id="rawoneshotrwlock-is-locked-upgradable"></span>`fn is_locked_upgradable(&self) -> bool`

- <span id="rawoneshotrwlock-acquire-shared"></span>`fn acquire_shared(&self) -> usize`

  Acquire a shared lock, returning the new lock value.

#### Trait Implementations

##### `impl Default for RawOneShotRwLock`

- <span id="rawoneshotrwlock-default"></span>`fn default() -> Self`

##### `impl RawRwLock for RawOneShotRwLock`

- <span id="rawoneshotrwlock-rawrwlock-const-init"></span>`const INIT: Self`

- <span id="rawoneshotrwlock-rawrwlock-type-guardmarker"></span>`type GuardMarker = GuardSend`

- <span id="rawoneshotrwlock-rawrwlock-lock-shared"></span>`fn lock_shared(&self)`

- <span id="rawoneshotrwlock-rawrwlock-try-lock-shared"></span>`fn try_lock_shared(&self) -> bool`

- <span id="rawoneshotrwlock-rawrwlock-unlock-shared"></span>`unsafe fn unlock_shared(&self)`

- <span id="rawoneshotrwlock-rawrwlock-lock-exclusive"></span>`fn lock_exclusive(&self)`

- <span id="rawoneshotrwlock-rawrwlock-try-lock-exclusive"></span>`fn try_lock_exclusive(&self) -> bool`

- <span id="rawoneshotrwlock-rawrwlock-unlock-exclusive"></span>`unsafe fn unlock_exclusive(&self)`

- <span id="rawoneshotrwlock-rawrwlock-is-locked"></span>`fn is_locked(&self) -> bool`

- <span id="rawoneshotrwlock-rawrwlock-is-locked-exclusive"></span>`fn is_locked_exclusive(&self) -> bool`

##### `impl RawRwLockDowngrade for RawOneShotRwLock`

- <span id="rawoneshotrwlock-rawrwlockdowngrade-downgrade"></span>`unsafe fn downgrade(&self)`

##### `impl RawRwLockRecursive for RawOneShotRwLock`

- <span id="rawoneshotrwlock-rawrwlockrecursive-lock-shared-recursive"></span>`fn lock_shared_recursive(&self)`

- <span id="rawoneshotrwlock-rawrwlockrecursive-try-lock-shared-recursive"></span>`fn try_lock_shared_recursive(&self) -> bool`

##### `impl RawRwLockUpgrade for RawOneShotRwLock`

- <span id="rawoneshotrwlock-rawrwlockupgrade-lock-upgradable"></span>`fn lock_upgradable(&self)`

- <span id="rawoneshotrwlock-rawrwlockupgrade-try-lock-upgradable"></span>`fn try_lock_upgradable(&self) -> bool`

- <span id="rawoneshotrwlock-rawrwlockupgrade-unlock-upgradable"></span>`unsafe fn unlock_upgradable(&self)`

- <span id="rawoneshotrwlock-rawrwlockupgrade-upgrade"></span>`unsafe fn upgrade(&self)`

- <span id="rawoneshotrwlock-rawrwlockupgrade-try-upgrade"></span>`unsafe fn try_upgrade(&self) -> bool`

##### `impl RawRwLockUpgradeDowngrade for RawOneShotRwLock`

- <span id="rawoneshotrwlock-rawrwlockupgradedowngrade-downgrade-upgradable"></span>`unsafe fn downgrade_upgradable(&self)`

- <span id="rawoneshotrwlock-rawrwlockupgradedowngrade-downgrade-to-upgradable"></span>`unsafe fn downgrade_to_upgradable(&self)`

## Type Aliases

### `OneShotMutex<T>`

```rust
type OneShotMutex<T> = lock_api::Mutex<RawOneShotMutex, T>;
```

A [`lock_api::Mutex`](../../lock_api/index.md) based on [`RawOneShotMutex`](mutex/index.md).

### `OneShotMutexGuard<'a, T>`

```rust
type OneShotMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawOneShotMutex, T>;
```

A [`lock_api::MutexGuard`](../../lock_api/index.md) based on [`RawOneShotMutex`](mutex/index.md).

### `OneShotRwLock<T>`

```rust
type OneShotRwLock<T> = lock_api::RwLock<RawOneShotRwLock, T>;
```

A [`lock_api::RwLock`](../../lock_api/index.md) based on [`RawOneShotRwLock`](rwlock/index.md).

### `OneShotRwLockReadGuard<'a, T>`

```rust
type OneShotRwLockReadGuard<'a, T> = lock_api::RwLockReadGuard<'a, RawOneShotRwLock, T>;
```

A [`lock_api::RwLockReadGuard`](../../lock_api/index.md) based on [`RawOneShotRwLock`](rwlock/index.md).

### `OneShotRwLockUpgradableReadGuard<'a, T>`

```rust
type OneShotRwLockUpgradableReadGuard<'a, T> = lock_api::RwLockUpgradableReadGuard<'a, RawOneShotRwLock, T>;
```

A [`lock_api::RwLockUpgradableReadGuard`](../../lock_api/index.md) based on [`RawOneShotRwLock`](rwlock/index.md).

### `OneShotRwLockWriteGuard<'a, T>`

```rust
type OneShotRwLockWriteGuard<'a, T> = lock_api::RwLockWriteGuard<'a, RawOneShotRwLock, T>;
```

A [`lock_api::RwLockWriteGuard`](../../lock_api/index.md) based on [`RawOneShotRwLock`](rwlock/index.md).

