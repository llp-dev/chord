**spin**

# Module: spin

## Contents

**Modules**

- [`barrier`](#barrier) - Synchronization primitive allowing multiple threads to synchronize the
- [`lazy`](#lazy) - Synchronization primitives for lazy evaluation.
- [`lock_api`](#lock_api) - Spin synchronisation primitives, but compatible with [`lock_api`](https://crates.io/crates/lock_api).
- [`mutex`](#mutex) - Locks that have the same behaviour as a mutex.
- [`once`](#once) - Synchronization primitives for one-time evaluation.
- [`relax`](#relax) - Strategies that determine the behaviour of locks when encountering contention.
- [`rwlock`](#rwlock) - A lock that provides data access to either one writer or many readers.

**Type Aliases**

- [`Barrier`](#barrier) - A primitive that synchronizes the execution of multiple threads. See [`barrier::Barrier`] for documentation.
- [`Lazy`](#lazy) - A value which is initialized on the first access. See [`lazy::Lazy`] for documentation.
- [`Mutex`](#mutex) - A primitive that synchronizes the execution of multiple threads. See [`mutex::Mutex`] for documentation.
- [`Once`](#once) - A primitive that provides lazy one-time initialization. See [`once::Once`] for documentation.
- [`RwLock`](#rwlock) - A lock that provides data access to either one writer or many readers. See [`rwlock::RwLock`] for documentation.
- [`RwLockUpgradableGuard`](#rwlockupgradableguard) - A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`]. See
- [`RwLockWriteGuard`](#rwlockwriteguard) - A guard that provides mutable data access. See [`rwlock::RwLockWriteGuard`] for documentation.

---

## spin::Barrier

*Type Alias*: `crate::barrier::Barrier`

A primitive that synchronizes the execution of multiple threads. See [`barrier::Barrier`] for documentation.

A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.



## spin::Lazy

*Type Alias*: `crate::lazy::Lazy<T, F>`

A value which is initialized on the first access. See [`lazy::Lazy`] for documentation.

A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.



## spin::Mutex

*Type Alias*: `crate::mutex::Mutex<T>`

A primitive that synchronizes the execution of multiple threads. See [`mutex::Mutex`] for documentation.

A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.



## spin::Once

*Type Alias*: `crate::once::Once<T>`

A primitive that provides lazy one-time initialization. See [`once::Once`] for documentation.

A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.



## spin::RwLock

*Type Alias*: `crate::rwlock::RwLock<T>`

A lock that provides data access to either one writer or many readers. See [`rwlock::RwLock`] for documentation.

A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.



## spin::RwLockUpgradableGuard

*Type Alias*: `crate::rwlock::RwLockUpgradableGuard<'a, T>`

A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`]. See
[`rwlock::RwLockUpgradableGuard`] for documentation.

A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.



## spin::RwLockWriteGuard

*Type Alias*: `crate::rwlock::RwLockWriteGuard<'a, T>`

A guard that provides mutable data access. See [`rwlock::RwLockWriteGuard`] for documentation.

A note for advanced users: this alias exists to avoid subtle type inference errors due to the default relax
strategy type parameter. If you need a non-default relax strategy, use the fully-qualified path.



## Module: barrier

Synchronization primitive allowing multiple threads to synchronize the
beginning of some computation.

Implementation adapted from the 'Barrier' type of the standard library. See:
<https://doc.rust-lang.org/std/sync/struct.Barrier.html>

Copyright 2014 The Rust Project Developers. See the COPYRIGHT
file at the top-level directory of this distribution and at
<http://rust-lang.org/COPYRIGHT>.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
<http://www.apache.org/licenses/LICENSE-2.0>> or the MIT license
<LICENSE-MIT or <http://opensource.org/licenses/MIT>>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.



## Module: lazy

Synchronization primitives for lazy evaluation.

Implementation adapted from the `SyncLazy` type of the standard library. See:
<https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html>



## Module: lock_api

Spin synchronisation primitives, but compatible with [`lock_api`](https://crates.io/crates/lock_api).



## Module: mutex

Locks that have the same behaviour as a mutex.

The [`Mutex`] in the root of the crate, can be configured using the `ticket_mutex` feature.
If it's enabled, [`TicketMutex`] and [`TicketMutexGuard`] will be re-exported as [`Mutex`]
and [`MutexGuard`], otherwise the [`SpinMutex`] and guard will be re-exported.

`ticket_mutex` is disabled by default.

[`Mutex`]: ../struct.Mutex.html
[`MutexGuard`]: ../struct.MutexGuard.html
[`TicketMutex`]: ./struct.TicketMutex.html
[`TicketMutexGuard`]: ./struct.TicketMutexGuard.html
[`SpinMutex`]: ./struct.SpinMutex.html
[`SpinMutexGuard`]: ./struct.SpinMutexGuard.html



## Module: once

Synchronization primitives for one-time evaluation.



## Module: relax

Strategies that determine the behaviour of locks when encountering contention.



## Module: rwlock

A lock that provides data access to either one writer or many readers.



