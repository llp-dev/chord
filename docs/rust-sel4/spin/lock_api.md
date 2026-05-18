**spin > lock_api**

# Module: lock_api

## Contents

**Type Aliases**

- [`Mutex`](#mutex) - A lock that provides mutually exclusive data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
- [`MutexGuard`](#mutexguard) - A guard that provides mutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
- [`RwLock`](#rwlock) - A lock that provides data access to either one writer or many readers (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
- [`RwLockReadGuard`](#rwlockreadguard) - A guard that provides immutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
- [`RwLockUpgradableReadGuard`](#rwlockupgradablereadguard) - A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`] (compatible with [`lock_api`](https://crates.io/crates/lock_api)).
- [`RwLockWriteGuard`](#rwlockwriteguard) - A guard that provides mutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).

---

## spin::lock_api::Mutex

*Type Alias*: `lock_api_crate::Mutex<crate::Mutex<()>, T>`

A lock that provides mutually exclusive data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).



## spin::lock_api::MutexGuard

*Type Alias*: `lock_api_crate::MutexGuard<'a, crate::Mutex<()>, T>`

A guard that provides mutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).



## spin::lock_api::RwLock

*Type Alias*: `lock_api_crate::RwLock<crate::RwLock<()>, T>`

A lock that provides data access to either one writer or many readers (compatible with [`lock_api`](https://crates.io/crates/lock_api)).



## spin::lock_api::RwLockReadGuard

*Type Alias*: `lock_api_crate::RwLockReadGuard<'a, crate::RwLock<()>, T>`

A guard that provides immutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).



## spin::lock_api::RwLockUpgradableReadGuard

*Type Alias*: `lock_api_crate::RwLockUpgradableReadGuard<'a, crate::RwLock<()>, T>`

A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`] (compatible with [`lock_api`](https://crates.io/crates/lock_api)).



## spin::lock_api::RwLockWriteGuard

*Type Alias*: `lock_api_crate::RwLockWriteGuard<'a, crate::RwLock<()>, T>`

A guard that provides mutable data access (compatible with [`lock_api`](https://crates.io/crates/lock_api)).



