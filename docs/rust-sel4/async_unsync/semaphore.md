**async_unsync > semaphore**

# Module: semaphore

## Contents

**Structs**

- [`Acquire`](#acquire) - The [`Future`] returned by [`acquire`](Semaphore::acquire), which
- [`AcquireError`](#acquireerror) - An error which can occur when a [`Semaphore`] has been closed.
- [`Permit`](#permit) - A permit representing access to the [`Semaphore`]'s guarded resource.
- [`Semaphore`](#semaphore) - An unsynchronized (`!Sync`), simple semaphore for asynchronous permit

**Enums**

- [`TryAcquireError`](#tryacquireerror) - An error which can occur when a [`Semaphore`] has been closed or has no

---

## async_unsync::semaphore::Acquire

*Struct*

The [`Future`] returned by [`acquire`](Semaphore::acquire), which
resolves when the required number of permits becomes available.

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## async_unsync::semaphore::AcquireError

*Struct*

An error which can occur when a [`Semaphore`] has been closed.

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AcquireError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &AcquireError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &AcquireError) -> bool`



## async_unsync::semaphore::Permit

*Struct*

A permit representing access to the [`Semaphore`]'s guarded resource.

**Generic Parameters:**
- 'a

**Methods:**

- `fn forget(self: Self)` - Drops the permit without returning it to the [`Semaphore`].

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## async_unsync::semaphore::Semaphore

*Struct*

An unsynchronized (`!Sync`), simple semaphore for asynchronous permit
acquisition.

**Methods:**

- `fn new(permits: usize) -> Self` - Creates a new semaphore with the initial number of permits.
- `fn close(self: &Self) -> usize` - Closes the semaphore and returns the number of notified pending waiters.
- `fn is_closed(self: &Self) -> bool` - Returns `true` if the semaphore has been closed
- `fn waiters(self: &Self) -> usize` - Returns the number of currently registered [`Future`]s waiting for a
- `fn available_permits(self: &Self) -> usize` - Returns the current number of available permits.
- `fn add_permits(self: &Self, n: usize)` - Adds `n` new permits to the semaphore.
- `fn remove_permits(self: &Self, n: usize)` - Permanently reduces the number of available permits by `n`.
- `fn try_acquire(self: &Self) -> Result<Permit, TryAcquireError>` - Acquires a single [`Permit`] or returns an [error](TryAcquireError), if
- `fn try_acquire_many(self: &Self, n: usize) -> Result<Permit, TryAcquireError>` - Acquires `n` [`Permit`]s or returns an [error](TryAcquireError), if
- `fn acquire(self: &Self) -> Acquire` - Acquires a single [`Permit`], potentially blocking until one becomes
- `fn acquire_many(self: &Self, n: usize) -> Acquire` - Acquires `n` [`Permit`]s, potentially blocking until they become

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## async_unsync::semaphore::TryAcquireError

*Enum*

An error which can occur when a [`Semaphore`] has been closed or has no
available permits.

**Variants:**
- `Closed` - The semaphore has been [closed](Semaphore::close) and can not issue new
- `NoPermits` - The semaphore has no available permits.

**Traits:** Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &TryAcquireError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &TryAcquireError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TryAcquireError`



