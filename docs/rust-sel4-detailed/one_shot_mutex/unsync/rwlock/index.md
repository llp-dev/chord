*[one_shot_mutex](../../index.md) / [unsync](../index.md) / [rwlock](index.md)*

---

# Module `rwlock`

## Contents

- [Structs](#structs)
  - [`RawOneShotRwLock`](#rawoneshotrwlock)
- [Type Aliases](#type-aliases)
  - [`OneShotRwLock`](#oneshotrwlock)
  - [`OneShotRwLockReadGuard`](#oneshotrwlockreadguard)
  - [`OneShotRwLockUpgradableReadGuard`](#oneshotrwlockupgradablereadguard)
  - [`OneShotRwLockWriteGuard`](#oneshotrwlockwriteguard)
- [Constants](#constants)
  - [`SHARED`](#shared)
  - [`UPGRADABLE`](#upgradable)
  - [`EXCLUSIVE`](#exclusive)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RawOneShotRwLock`](#rawoneshotrwlock) | struct | A one-shot readers-writer lock that panics instead of (dead)locking on contention. |
| [`OneShotRwLock`](#oneshotrwlock) | type | A [`lock_api::RwLock`] based on [`RawOneShotRwLock`]. |
| [`OneShotRwLockReadGuard`](#oneshotrwlockreadguard) | type | A [`lock_api::RwLockReadGuard`] based on [`RawOneShotRwLock`]. |
| [`OneShotRwLockUpgradableReadGuard`](#oneshotrwlockupgradablereadguard) | type | A [`lock_api::RwLockUpgradableReadGuard`] based on [`RawOneShotRwLock`]. |
| [`OneShotRwLockWriteGuard`](#oneshotrwlockwriteguard) | type | A [`lock_api::RwLockWriteGuard`] based on [`RawOneShotRwLock`]. |
| [`SHARED`](#shared) | const | Normal shared lock counter |
| [`UPGRADABLE`](#upgradable) | const | Special upgradable shared lock flag |
| [`EXCLUSIVE`](#exclusive) | const | Exclusive lock flag |

## Structs

### `RawOneShotRwLock`

```rust
struct RawOneShotRwLock {
    lock: core::cell::Cell<usize>,
}
```

A one-shot readers-writer lock that panics instead of (dead)locking on contention.

This lock allows no contention and panics on `lock_shared`, `lock_exclusive`, `lock_upgradable`, and `upgrade` if it is already locked conflictingly.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.

This lock does not implement `Sync`, which permits a slightly more efficient implementation.
For a variant that does implement `Sync`, see [`sync::RawOneShotRwLock`](crate::sync::RawOneShotRwLock).




# Examples

```rust
use one_shot_mutex::unsync::OneShotRwLock;

let m: OneShotRwLock<i32> = OneShotRwLock::new(42);

// This is equivalent to `X.try_write().unwrap()`.
let x = m.write();

// This panics instead of deadlocking.
// let x2 = m.write();

// Once we unlock the mutex, we can lock it again.
drop(x);
let x = m.write();
```

#### Implementations

- <span id="rawoneshotrwlock-new"></span>`const fn new() -> Self`

- <span id="rawoneshotrwlock-over-state"></span>`fn over_state(&self, f: impl FnOnce(usize) -> usize) -> usize`

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

### `OneShotRwLock<T>`

```rust
type OneShotRwLock<T> = lock_api::RwLock<RawOneShotRwLock, T>;
```

A [`lock_api::RwLock`](../../../lock_api/index.md) based on [`RawOneShotRwLock`](#rawoneshotrwlock).

### `OneShotRwLockReadGuard<'a, T>`

```rust
type OneShotRwLockReadGuard<'a, T> = lock_api::RwLockReadGuard<'a, RawOneShotRwLock, T>;
```

A [`lock_api::RwLockReadGuard`](../../../lock_api/index.md) based on [`RawOneShotRwLock`](#rawoneshotrwlock).

### `OneShotRwLockUpgradableReadGuard<'a, T>`

```rust
type OneShotRwLockUpgradableReadGuard<'a, T> = lock_api::RwLockUpgradableReadGuard<'a, RawOneShotRwLock, T>;
```

A [`lock_api::RwLockUpgradableReadGuard`](../../../lock_api/index.md) based on [`RawOneShotRwLock`](#rawoneshotrwlock).

### `OneShotRwLockWriteGuard<'a, T>`

```rust
type OneShotRwLockWriteGuard<'a, T> = lock_api::RwLockWriteGuard<'a, RawOneShotRwLock, T>;
```

A [`lock_api::RwLockWriteGuard`](../../../lock_api/index.md) based on [`RawOneShotRwLock`](#rawoneshotrwlock).

## Constants

### `SHARED`
```rust
const SHARED: usize = 4usize;
```

Normal shared lock counter

### `UPGRADABLE`
```rust
const UPGRADABLE: usize = 2usize;
```

Special upgradable shared lock flag

### `EXCLUSIVE`
```rust
const EXCLUSIVE: usize = 1usize;
```

Exclusive lock flag

