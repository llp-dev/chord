**smoltcp > time**

# Module: time

## Contents

**Structs**

- [`Duration`](#duration) - A relative amount of time.
- [`Instant`](#instant) - A representation of an absolute time value.

---

## smoltcp::time::Duration

*Struct*

A relative amount of time.

**Methods:**

- `fn from_micros(micros: u64) -> Duration` - Create a new `Duration` from a number of microseconds.
- `fn from_millis(millis: u64) -> Duration` - Create a new `Duration` from a number of milliseconds.
- `fn from_secs(secs: u64) -> Duration` - Create a new `Duration` from a number of seconds.
- `fn millis(self: &Self) -> u64` - The fractional number of milliseconds in this `Duration`.
- `fn micros(self: &Self) -> u64` - The fractional number of milliseconds in this `Duration`.
- `fn secs(self: &Self) -> u64` - The number of whole seconds in this `Duration`.
- `fn total_millis(self: &Self) -> u64` - The total number of milliseconds in this `Duration`.
- `fn total_micros(self: &Self) -> u64` - The total number of microseconds in this `Duration`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Shr**
  - `fn shr(self: Self, rhs: u32) -> Duration`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, rhs: u32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Duration)`
- **Sub**
  - `fn sub(self: Self, rhs: Duration) -> Duration`
- **Shl**
  - `fn shl(self: Self, rhs: u32) -> Duration`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, rhs: u32)`
- **Ord**
  - `fn cmp(self: &Self, other: &Duration) -> $crate::cmp::Ordering`
- **Div**
  - `fn div(self: Self, rhs: u32) -> Duration`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DivAssign**
  - `fn div_assign(self: & mut Self, rhs: u32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: Duration)`
- **Mul**
  - `fn mul(self: Self, rhs: u32) -> Duration`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, rhs: u32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Add**
  - `fn add(self: Self, rhs: Duration) -> Duration`
- **From**
  - `fn from(other: ::core::time::Duration) -> Duration`
- **Default**
  - `fn default() -> Duration`
- **Clone**
  - `fn clone(self: &Self) -> Duration`
- **PartialEq**
  - `fn eq(self: &Self, other: &Duration) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Duration) -> $crate::option::Option<$crate::cmp::Ordering>`



## smoltcp::time::Instant

*Struct*

A representation of an absolute time value.

The `Instant` type is a wrapper around a `i64` value that
represents a number of microseconds, monotonically increasing
since an arbitrary moment in time, such as system startup.

* A value of `0` is inherently arbitrary.
* A value less than `0` indicates a time before the starting
  point.

**Methods:**

- `fn from_micros<T>(micros: T) -> Instant` - Create a new `Instant` from a number of microseconds.
- `fn from_micros_const(micros: i64) -> Instant`
- `fn from_millis<T>(millis: T) -> Instant` - Create a new `Instant` from a number of milliseconds.
- `fn from_millis_const(millis: i64) -> Instant` - Create a new `Instant` from a number of milliseconds.
- `fn from_secs<T>(secs: T) -> Instant` - Create a new `Instant` from a number of seconds.
- `fn millis(self: &Self) -> i64` - The fractional number of milliseconds that have passed
- `fn micros(self: &Self) -> i64` - The fractional number of microseconds that have passed
- `fn secs(self: &Self) -> i64` - The number of whole seconds that have passed since the
- `fn total_millis(self: &Self) -> i64` - The total number of milliseconds that have passed since
- `fn total_micros(self: &Self) -> i64` - The total number of milliseconds that have passed since

**Traits:** Copy, Eq

**Trait Implementations:**

- **Sub**
  - `fn sub(self: Self, rhs: Duration) -> Instant`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: Duration)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Add**
  - `fn add(self: Self, rhs: Duration) -> Instant`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Duration)`
- **Clone**
  - `fn clone(self: &Self) -> Instant`
- **PartialEq**
  - `fn eq(self: &Self, other: &Instant) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Instant) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Instant) -> $crate::cmp::Ordering`
- **Sub**
  - `fn sub(self: Self, rhs: Instant) -> Duration`



