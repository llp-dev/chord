*[one_shot_mutex](../../index.md) / [unsync](../index.md) / [mutex](index.md)*

---

# Module `mutex`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RawOneShotMutex`](#rawoneshotmutex) | struct | A one-shot mutex that panics instead of (dead)locking on contention. |
| [`OneShotMutex`](#oneshotmutex) | type | A [`lock_api::Mutex`] based on [`RawOneShotMutex`]. |
| [`OneShotMutexGuard`](#oneshotmutexguard) | type | A [`lock_api::MutexGuard`] based on [`RawOneShotMutex`]. |

## Structs

### `RawOneShotMutex`

```rust
struct RawOneShotMutex {
    lock: core::cell::Cell<bool>,
}
```

A one-shot mutex that panics instead of (dead)locking on contention.

This mutex allows no contention and panics instead of blocking on [`lock`](#lock) if it is already locked.
This is useful in situations where contention would be a bug,
such as in single-threaded programs that would deadlock on contention.

This mutex does not implement `Sync`, which permits a slightly more efficient implementation.
For a variant that does implement `Sync`, see [`sync::RawOneShotMutex`](crate::sync::RawOneShotMutex).

This mutex should be used through [`OneShotMutex`](#oneshotmutex).

# Examples

```rust
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

## Type Aliases

### `OneShotMutex<T>`

```rust
type OneShotMutex<T> = lock_api::Mutex<RawOneShotMutex, T>;
```

A [`lock_api::Mutex`](../../../lock_api/index.md) based on [`RawOneShotMutex`](#rawoneshotmutex).

### `OneShotMutexGuard<'a, T>`

```rust
type OneShotMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawOneShotMutex, T>;
```

A [`lock_api::MutexGuard`](../../../lock_api/index.md) based on [`RawOneShotMutex`](#rawoneshotmutex).

