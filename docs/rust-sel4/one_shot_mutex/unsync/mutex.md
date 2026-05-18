**one_shot_mutex > unsync > mutex**

# Module: unsync::mutex

## Contents

**Structs**

- [`RawOneShotMutex`](#rawoneshotmutex) - A one-shot mutex that panics instead of (dead)locking on contention.

**Type Aliases**

- [`OneShotMutex`](#oneshotmutex) - A [`lock_api::Mutex`] based on [`RawOneShotMutex`].
- [`OneShotMutexGuard`](#oneshotmutexguard) - A [`lock_api::MutexGuard`] based on [`RawOneShotMutex`].

---

## one_shot_mutex::unsync::mutex::OneShotMutex

*Type Alias*: `lock_api::Mutex<RawOneShotMutex, T>`

A [`lock_api::Mutex`] based on [`RawOneShotMutex`].



## one_shot_mutex::unsync::mutex::OneShotMutexGuard

*Type Alias*: `lock_api::MutexGuard<'a, RawOneShotMutex, T>`

A [`lock_api::MutexGuard`] based on [`RawOneShotMutex`].



## one_shot_mutex::unsync::mutex::RawOneShotMutex

*Struct*

A one-shot mutex that panics instead of (dead)locking on contention.

This mutex allows no contention and panics instead of blocking on [`lock`] if it is already locked.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.

This mutex does not implement `Sync`, which permits a slightly more efficient implementation.
For a variant that does implement `Sync`, see [`sync::RawOneShotMutex`](crate::sync::RawOneShotMutex).

This mutex should be used through [`OneShotMutex`].

[`lock`]: Self::lock

# Examples

```
use one_shot_mutex::unsync::OneShotMutex;

let m: OneShotMutex<i32> = OneShotMutex::new(42);

// This is equivalent to `X.try_lock().unwrap()`.
let x = m.lock();

// This panics instead of deadlocking.
// let x2 = m.lock();

// Once we unlock the mutex, we can lock it again.
drop(x);
let x = m.lock();
```

**Methods:**

- `fn new() -> Self`

**Trait Implementations:**

- **RawMutex**
  - `fn lock(self: &Self)`
  - `fn try_lock(self: &Self) -> bool`
  - `fn unlock(self: &Self)`
  - `fn is_locked(self: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **RawMutexFair**
  - `fn unlock_fair(self: &Self)`
  - `fn bump(self: &Self)`



