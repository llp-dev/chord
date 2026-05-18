*[futures_channel](../index.md) / [lock](index.md)*

---

# Module `lock`

A "mutex" which only supports `try_lock`

As a futures library the eventual call to an event loop should be the only
thing that ever blocks, so this is assisted with a fast user-space
implementation of a lock that can only have a `try_lock` operation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Lock`](#lock) | struct | A "mutex" around a value, similar to `std::sync::Mutex<T>`. |
| [`TryLock`](#trylock) | struct | Sentinel representing an acquired lock through which the data can be accessed. |

## Structs

### `Lock<T>`

```rust
struct Lock<T> {
    locked: core::sync::atomic::AtomicBool,
    data: core::cell::UnsafeCell<T>,
}
```

A "mutex" around a value, similar to `std::sync::Mutex<T>`.

This lock only supports the `try_lock` operation, however, and does not
implement poisoning.

#### Implementations

- <span id="lock-new"></span>`fn new(t: T) -> Self`

  Creates a new lock around the given value.

- <span id="lock-try-lock"></span>`fn try_lock(&self) -> Option<TryLock<'_, T>>` — [`TryLock`](#trylock)

  Attempts to acquire this lock, returning whether the lock was acquired or

  not.

  

  If `Some` is returned then the data this lock protects can be accessed

  through the sentinel. This sentinel allows both mutable and immutable

  access.

  

  If `None` is returned then the lock is already locked, either elsewhere

  on this thread or on another thread.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Lock<T>`

- <span id="lock-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> Send for Lock<T>`

##### `impl<T: Send> Sync for Lock<T>`

### `TryLock<'a, T>`

```rust
struct TryLock<'a, T> {
    __ptr: &'a Lock<T>,
}
```

Sentinel representing an acquired lock through which the data can be
accessed.

#### Trait Implementations

##### `impl<T> Deref for TryLock<'_, T>`

- <span id="trylock-deref-type-target"></span>`type Target = T`

- <span id="trylock-deref"></span>`fn deref(&self) -> &T`

##### `impl<T> DerefMut for TryLock<'_, T>`

- <span id="trylock-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T> Drop for TryLock<'_, T>`

- <span id="trylock-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for TryLock<'a, T>`

- <span id="trylock-receiver-type-target"></span>`type Target = T`

