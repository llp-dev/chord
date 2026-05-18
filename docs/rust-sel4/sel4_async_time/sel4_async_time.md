**sel4_async_time**

# Module: sel4_async_time

## Contents

**Structs**

- [`Elapsed`](#elapsed) - Error returned by `Timeout`.
- [`Sleep`](#sleep)
- [`Timeout`](#timeout)
- [`TimerManager`](#timermanager)

---

## sel4_async_time::Elapsed

*Struct*

Error returned by `Timeout`.

This error is returned when a timeout expires before the function was able to finish.

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Elapsed) -> bool`



## sel4_async_time::Sleep

*Struct*

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## sel4_async_time::Timeout

*Struct*

**Generic Parameters:**
- F

**Trait Implementations:**

- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`



## sel4_async_time::TimerManager

*Struct*

**Methods:**

- `fn new() -> Self`
- `fn poll(self: &Self, timestamp: Instant) -> bool`
- `fn poll_at(self: &Self) -> Option<Instant>`
- `fn sleep_until(self: &Self, absolute_expiry: Instant) -> Sleep`
- `fn timeout_at<F>(self: &Self, absolute_deadline: Instant, future: F) -> Timeout<F>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TimerManager`



