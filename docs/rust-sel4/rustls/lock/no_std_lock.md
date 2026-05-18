**rustls > lock > no_std_lock**

# Module: lock::no_std_lock

## Contents

**Structs**

- [`Mutex`](#mutex) - A no-std compatible wrapper around [`Lock`].
- [`Poisoned`](#poisoned) - A marker type used to indicate `Lock::lock` failed due to a poisoned lock.

**Traits**

- [`Lock`](#lock) - A lock protecting shared data.
- [`MakeMutex`](#makemutex) - A lock builder.

**Type Aliases**

- [`MutexGuard`](#mutexguard) - A no-std compatible mutex guard.

---

## rustls::lock::no_std_lock::Lock

*Trait*

A lock protecting shared data.

**Methods:**

- `lock`: Acquire the lock.



## rustls::lock::no_std_lock::MakeMutex

*Trait*

A lock builder.

**Methods:**

- `make_mutex`: Create a new mutex.



## rustls::lock::no_std_lock::Mutex

*Struct*

A no-std compatible wrapper around [`Lock`].

**Generic Parameters:**
- T

**Methods:**

- `fn new<M>(val: T) -> Self` - Creates a new mutex in an unlocked state ready for use.
- `fn lock(self: &Self) -> Option<MutexGuard<T>>` - Acquires the mutex, blocking the current thread until it is able to do so.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::lock::no_std_lock::MutexGuard

*Type Alias*: `alloc::boxed::Box<dyn DerefMut>`

A no-std compatible mutex guard.



## rustls::lock::no_std_lock::Poisoned

*Struct*

A marker type used to indicate `Lock::lock` failed due to a poisoned lock.

**Unit Struct**



