**spin > mutex**

# Module: mutex

## Contents

**Modules**

- [`spin`](#spin) - A naïve spinning mutex.

**Structs**

- [`Mutex`](#mutex) - A spin-based lock providing mutually exclusive access to data.
- [`MutexGuard`](#mutexguard) - A generic guard that will protect some data access and

**Type Aliases**

- [`InnerMutex`](#innermutex)
- [`InnerMutexGuard`](#innermutexguard)

---

## spin::mutex::InnerMutex

*Type Alias*: `self::spin::SpinMutex<T, R>`



## spin::mutex::InnerMutexGuard

*Type Alias*: `self::spin::SpinMutexGuard<'a, T>`



## spin::mutex::Mutex

*Struct*

A spin-based lock providing mutually exclusive access to data.

The implementation uses either a ticket mutex or a regular spin mutex depending on whether the `spin_mutex` or
`ticket_mutex` feature flag is enabled.

# Example

```
use spin;

let lock = spin::Mutex::new(0);

// Modify the data
*lock.lock() = 2;

// Read the data
let answer = *lock.lock();
assert_eq!(answer, 2);
```

# Thread safety example

```
use spin;
use std::sync::{Arc, Barrier};

let thread_count = 1000;
let spin_mutex = Arc::new(spin::Mutex::new(0));

// We use a barrier to ensure the readout happens after all writing
let barrier = Arc::new(Barrier::new(thread_count + 1));

# let mut ts = Vec::new();
for _ in (0..thread_count) {
    let my_barrier = barrier.clone();
    let my_lock = spin_mutex.clone();
# let t =
    std::thread::spawn(move || {
        let mut guard = my_lock.lock();
        *guard += 1;

        // Release the lock to prevent a deadlock
        drop(guard);
        my_barrier.wait();
    });
# ts.push(t);
}

barrier.wait();

let answer = { *spin_mutex.lock() };
assert_eq!(answer, thread_count);

# for t in ts {
#     t.join().unwrap();
# }
```

**Generic Parameters:**
- T
- R

**Fields:**
- `inner: self::spin::SpinMutex<T, R>`

**Methods:**

- `fn lock(self: &Self) -> MutexGuard<T>` - Locks the [`Mutex`] and returns a guard that permits access to the inner data.
- `fn is_locked(self: &Self) -> bool` - Returns `true` if the lock is currently held.
- `fn force_unlock(self: &Self)` - Force unlock this [`Mutex`].
- `fn try_lock(self: &Self) -> Option<MutexGuard<T>>` - Try to lock this [`Mutex`], returning a lock guard if successful.
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.
- `fn new(value: T) -> Self` - Creates a new [`Mutex`] wrapping the supplied data.
- `fn into_inner(self: Self) -> T` - Consumes this [`Mutex`] and unwraps the underlying data.

**Traits:** Sync, Send

**Trait Implementations:**

- **From**
  - `fn from(data: T) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **RawMutex**
  - `fn lock(self: &Self)`
  - `fn try_lock(self: &Self) -> bool`
  - `fn unlock(self: &Self)`
  - `fn is_locked(self: &Self) -> bool`



## spin::mutex::MutexGuard

*Struct*

A generic guard that will protect some data access and
uses either a ticket lock or a normal spin mutex.

For more info see [`TicketMutexGuard`] or [`SpinMutexGuard`].

[`TicketMutexGuard`]: ./struct.TicketMutexGuard.html
[`SpinMutexGuard`]: ./struct.SpinMutexGuard.html

**Generic Parameters:**
- 'a
- T

**Fields:**
- `inner: self::spin::SpinMutexGuard<'a, T>`

**Methods:**

- `fn leak(this: Self) -> &'a  mut T` - Leak the lock guard, yielding a mutable reference to the underlying data.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Deref**
  - `fn deref(self: &Self) -> &T`



## Module: spin

A naïve spinning mutex.

Waiting threads hammer an atomic variable until it becomes available. Best-case latency is low, but worst-case
latency is theoretically infinite.



