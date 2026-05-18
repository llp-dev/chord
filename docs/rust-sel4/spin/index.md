# spin

This crate provides [spin-based](https://en.wikipedia.org/wiki/Spinlock) versions of the
primitives in `std::sync` and `std::lazy`. Because synchronization is done through spinning,
the primitives are suitable for use in `no_std` environments.

# Features

- `Mutex`, `RwLock`, `Once`/`SyncOnceCell`, and `SyncLazy` equivalents

- Support for `no_std` environments

- [`lock_api`](https://crates.io/crates/lock_api) compatibility

- Upgradeable `RwLock` guards

- Guards can be sent and shared between threads

- Guard leaking

- Ticket locks

- Different strategies for dealing with contention

# Relationship with `std::sync`

While `spin` is not a drop-in replacement for `std::sync` (and
[should not be considered as such](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html))
an effort is made to keep this crate reasonably consistent with `std::sync`.

Many of the types defined in this crate have 'additional capabilities' when compared to `std::sync`:

- Because spinning does not depend on the thread-driven model of `std::sync`, guards ([`MutexGuard`],
  [`RwLockReadGuard`], [`RwLockWriteGuard`], etc.) may be sent and shared between threads.

- [`RwLockUpgradableGuard`] supports being upgraded into a [`RwLockWriteGuard`].

- Guards support [leaking](https://doc.rust-lang.org/nomicon/leaking.html).

- [`Once`] owns the value returned by its `call_once` initializer.

- [`RwLock`] supports counting readers and writers.

Conversely, the types in this crate do not have some of the features `std::sync` has:

- Locks do not track [panic poisoning](https://doc.rust-lang.org/nomicon/poisoning.html).

## Feature flags

The crate comes with a few feature flags that you may wish to use.

- `lock_api` enables support for [`lock_api`](https://crates.io/crates/lock_api)

- `ticket_mutex` uses a ticket lock for the implementation of `Mutex`

- `fair_mutex` enables a fairer implementation of `Mutex` that uses eventual fairness to avoid
  starvation

- `std` enables support for thread yielding instead of spinning

## Modules

### [`spin`](spin.md)

*7 modules, 7 type aliases*

### [`barrier`](barrier.md)

*3 structs*

### [`lazy`](lazy.md)

*1 struct*

### [`lock_api`](lock_api.md)

*6 type aliases*

### [`mutex`](mutex.md)

*1 module, 2 structs, 2 type aliases*

### [`mutex::spin`](mutex/spin.md)

*2 structs*

### [`once`](once.md)

*1 module, 2 structs*

### [`once::status`](once/status.md)

*1 enum, 1 struct*

### [`relax`](relax.md)

*1 trait, 2 structs*

### [`rwlock`](rwlock.md)

*1 function, 3 constants, 4 structs*

