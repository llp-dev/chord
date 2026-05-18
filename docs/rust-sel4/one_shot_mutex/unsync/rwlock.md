**one_shot_mutex > unsync > rwlock**

# Module: unsync::rwlock

## Contents

**Structs**

- [`RawOneShotRwLock`](#rawoneshotrwlock) - A one-shot readers-writer lock that panics instead of (dead)locking on contention.

**Type Aliases**

- [`OneShotRwLock`](#oneshotrwlock) - A [`lock_api::RwLock`] based on [`RawOneShotRwLock`].
- [`OneShotRwLockReadGuard`](#oneshotrwlockreadguard) - A [`lock_api::RwLockReadGuard`] based on [`RawOneShotRwLock`].
- [`OneShotRwLockUpgradableReadGuard`](#oneshotrwlockupgradablereadguard) - A [`lock_api::RwLockUpgradableReadGuard`] based on [`RawOneShotRwLock`].
- [`OneShotRwLockWriteGuard`](#oneshotrwlockwriteguard) - A [`lock_api::RwLockWriteGuard`] based on [`RawOneShotRwLock`].

---

## one_shot_mutex::unsync::rwlock::OneShotRwLock

*Type Alias*: `lock_api::RwLock<RawOneShotRwLock, T>`

A [`lock_api::RwLock`] based on [`RawOneShotRwLock`].



## one_shot_mutex::unsync::rwlock::OneShotRwLockReadGuard

*Type Alias*: `lock_api::RwLockReadGuard<'a, RawOneShotRwLock, T>`

A [`lock_api::RwLockReadGuard`] based on [`RawOneShotRwLock`].



## one_shot_mutex::unsync::rwlock::OneShotRwLockUpgradableReadGuard

*Type Alias*: `lock_api::RwLockUpgradableReadGuard<'a, RawOneShotRwLock, T>`

A [`lock_api::RwLockUpgradableReadGuard`] based on [`RawOneShotRwLock`].



## one_shot_mutex::unsync::rwlock::OneShotRwLockWriteGuard

*Type Alias*: `lock_api::RwLockWriteGuard<'a, RawOneShotRwLock, T>`

A [`lock_api::RwLockWriteGuard`] based on [`RawOneShotRwLock`].



## one_shot_mutex::unsync::rwlock::RawOneShotRwLock

*Struct*

A one-shot readers-writer lock that panics instead of (dead)locking on contention.

This lock allows no contention and panics on [`lock_shared`], [`lock_exclusive`], [`lock_upgradable`], and [`upgrade`] if it is already locked conflictingly.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.

This lock does not implement `Sync`, which permits a slightly more efficient implementation.
For a variant that does implement `Sync`, see [`sync::RawOneShotRwLock`](crate::sync::RawOneShotRwLock).

[`lock_shared`]: RawOneShotRwLock::lock_shared
[`lock_exclusive`]: RawOneShotRwLock::lock_exclusive
[`lock_upgradable`]: RawOneShotRwLock::lock_upgradable
[`upgrade`]: RawOneShotRwLock::upgrade

# Examples

```
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

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **RawRwLock**
  - `fn lock_shared(self: &Self)`
  - `fn try_lock_shared(self: &Self) -> bool`
  - `fn unlock_shared(self: &Self)`
  - `fn lock_exclusive(self: &Self)`
  - `fn try_lock_exclusive(self: &Self) -> bool`
  - `fn unlock_exclusive(self: &Self)`
  - `fn is_locked(self: &Self) -> bool`
  - `fn is_locked_exclusive(self: &Self) -> bool`
- **RawRwLockUpgradeDowngrade**
  - `fn downgrade_upgradable(self: &Self)`
  - `fn downgrade_to_upgradable(self: &Self)`
- **RawRwLockUpgrade**
  - `fn lock_upgradable(self: &Self)`
  - `fn try_lock_upgradable(self: &Self) -> bool`
  - `fn unlock_upgradable(self: &Self)`
  - `fn upgrade(self: &Self)`
  - `fn try_upgrade(self: &Self) -> bool`
- **RawRwLockDowngrade**
  - `fn downgrade(self: &Self)`
- **Default**
  - `fn default() -> Self`
- **RawRwLockRecursive**
  - `fn lock_shared_recursive(self: &Self)`
  - `fn try_lock_shared_recursive(self: &Self) -> bool`



