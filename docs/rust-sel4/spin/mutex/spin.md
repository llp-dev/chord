**spin > mutex > spin**

# Module: mutex::spin

## Contents

**Structs**

- [`SpinMutex`](#spinmutex) - A [spin lock](https://en.m.wikipedia.org/wiki/Spinlock) providing mutually exclusive access to data.
- [`SpinMutexGuard`](#spinmutexguard) - A guard that provides mutable data access.

---

## spin::mutex::spin::SpinMutex

*Struct*

A [spin lock](https://en.m.wikipedia.org/wiki/Spinlock) providing mutually exclusive access to data.

# Example

```
use spin;

let lock = spin::mutex::SpinMutex::<_>::new(0);

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
let spin_mutex = Arc::new(spin::mutex::SpinMutex::<_>::new(0));

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
- `phantom: core::marker::PhantomData<R>`
- `lock: crate::atomic::AtomicBool`
- `data: core::cell::UnsafeCell<T>`

**Methods:**

- `fn lock(self: &Self) -> SpinMutexGuard<T>` - Locks the [`SpinMutex`] and returns a guard that permits access to the inner data.
- `fn is_locked(self: &Self) -> bool` - Returns `true` if the lock is currently held.
- `fn force_unlock(self: &Self)` - Force unlock this [`SpinMutex`].
- `fn try_lock(self: &Self) -> Option<SpinMutexGuard<T>>` - Try to lock this [`SpinMutex`], returning a lock guard if successful.
- `fn get_mut(self: & mut Self) -> & mut T` - Returns a mutable reference to the underlying data.
- `fn new(data: T) -> Self` - Creates a new [`SpinMutex`] wrapping the supplied data.
- `fn into_inner(self: Self) -> T` - Consumes this [`SpinMutex`] and unwraps the underlying data.
- `fn as_mut_ptr(self: &Self) -> *mut T` - Returns a mutable pointer to the underlying data.

**Traits:** Send, Sync

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



## spin::mutex::spin::SpinMutexGuard

*Struct*

A guard that provides mutable data access.

When the guard falls out of scope it will release the lock.

**Generic Parameters:**
- 'a
- T

**Fields:**
- `lock: &'a crate::atomic::AtomicBool`
- `data: *mut T`

**Methods:**

- `fn leak(this: Self) -> &'a  mut T` - Leak the lock guard, yielding a mutable reference to the underlying data.

**Traits:** Sync, Send

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)` - The dropping of the MutexGuard will release the lock it was created from.



