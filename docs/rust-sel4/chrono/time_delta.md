**chrono > time_delta**

# Module: time_delta

## Contents

**Structs**

- [`OutOfRangeError`](#outofrangeerror) - Represents error when converting `TimeDelta` to/from a standard library
- [`TimeDelta`](#timedelta) - Time duration with nanosecond precision.

---

## chrono::time_delta::OutOfRangeError

*Struct*

Represents error when converting `TimeDelta` to/from a standard library
implementation

The `std::time::Duration` supports a range from zero to `u64::MAX`
*seconds*, while this module supports signed range of up to
`i64::MAX` of *milliseconds*.

**Tuple Struct**: `()`



## chrono::time_delta::TimeDelta

*Struct*

Time duration with nanosecond precision.

This also allows for negative durations; see individual methods for details.

A `TimeDelta` is represented internally as a complement of seconds and
nanoseconds. The range is restricted to that of `i64` milliseconds, with the
minimum value notably being set to `-i64::MAX` rather than allowing the full
range of `i64::MIN`. This is to allow easy flipping of sign, so that for
instance `abs()` can be called without any checks.

**Methods:**

- `fn new(secs: i64, nanos: u32) -> Option<TimeDelta>` - Makes a new `TimeDelta` with given number of seconds and nanoseconds.
- `fn weeks(weeks: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of weeks.
- `fn try_weeks(weeks: i64) -> Option<TimeDelta>` - Makes a new `TimeDelta` with the given number of weeks.
- `fn days(days: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of days.
- `fn try_days(days: i64) -> Option<TimeDelta>` - Makes a new `TimeDelta` with the given number of days.
- `fn hours(hours: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of hours.
- `fn try_hours(hours: i64) -> Option<TimeDelta>` - Makes a new `TimeDelta` with the given number of hours.
- `fn minutes(minutes: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of minutes.
- `fn try_minutes(minutes: i64) -> Option<TimeDelta>` - Makes a new `TimeDelta` with the given number of minutes.
- `fn seconds(seconds: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of seconds.
- `fn try_seconds(seconds: i64) -> Option<TimeDelta>` - Makes a new `TimeDelta` with the given number of seconds.
- `fn milliseconds(milliseconds: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of milliseconds.
- `fn try_milliseconds(milliseconds: i64) -> Option<TimeDelta>` - Makes a new `TimeDelta` with the given number of milliseconds.
- `fn microseconds(microseconds: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of microseconds.
- `fn nanoseconds(nanos: i64) -> TimeDelta` - Makes a new `TimeDelta` with the given number of nanoseconds.
- `fn num_weeks(self: &Self) -> i64` - Returns the total number of whole weeks in the `TimeDelta`.
- `fn num_days(self: &Self) -> i64` - Returns the total number of whole days in the `TimeDelta`.
- `fn num_hours(self: &Self) -> i64` - Returns the total number of whole hours in the `TimeDelta`.
- `fn num_minutes(self: &Self) -> i64` - Returns the total number of whole minutes in the `TimeDelta`.
- `fn num_seconds(self: &Self) -> i64` - Returns the total number of whole seconds in the `TimeDelta`.
- `fn as_seconds_f64(self: Self) -> f64` - Returns the fractional number of seconds in the `TimeDelta`.
- `fn as_seconds_f32(self: Self) -> f32` - Returns the fractional number of seconds in the `TimeDelta`.
- `fn num_milliseconds(self: &Self) -> i64` - Returns the total number of whole milliseconds in the `TimeDelta`.
- `fn subsec_millis(self: &Self) -> i32` - Returns the number of milliseconds in the fractional part of the duration.
- `fn num_microseconds(self: &Self) -> Option<i64>` - Returns the total number of whole microseconds in the `TimeDelta`,
- `fn subsec_micros(self: &Self) -> i32` - Returns the number of microseconds in the fractional part of the duration.
- `fn num_nanoseconds(self: &Self) -> Option<i64>` - Returns the total number of whole nanoseconds in the `TimeDelta`,
- `fn subsec_nanos(self: &Self) -> i32` - Returns the number of nanoseconds in the fractional part of the duration.
- `fn checked_add(self: &Self, rhs: &TimeDelta) -> Option<TimeDelta>` - Add two `TimeDelta`s, returning `None` if overflow occurred.
- `fn checked_sub(self: &Self, rhs: &TimeDelta) -> Option<TimeDelta>` - Subtract two `TimeDelta`s, returning `None` if overflow occurred.
- `fn checked_mul(self: &Self, rhs: i32) -> Option<TimeDelta>` - Multiply a `TimeDelta` with a i32, returning `None` if overflow occurred.
- `fn checked_div(self: &Self, rhs: i32) -> Option<TimeDelta>` - Divide a `TimeDelta` with a i32, returning `None` if dividing by 0.
- `fn abs(self: &Self) -> TimeDelta` - Returns the `TimeDelta` as an absolute (non-negative) value.
- `fn min_value() -> TimeDelta` - The minimum possible `TimeDelta`: `-i64::MAX` milliseconds.
- `fn max_value() -> TimeDelta` - The maximum possible `TimeDelta`: `i64::MAX` milliseconds.
- `fn zero() -> TimeDelta` - A `TimeDelta` where the stored seconds and nanoseconds are equal to zero.
- `fn is_zero(self: &Self) -> bool` - Returns `true` if the `TimeDelta` equals `TimeDelta::zero()`.
- `fn from_std(duration: Duration) -> Result<TimeDelta, OutOfRangeError>` - Creates a `TimeDelta` object from `std::time::Duration`
- `fn to_std(self: &Self) -> Result<Duration, OutOfRangeError>` - Creates a `std::time::Duration` object from a `TimeDelta`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **Sum**
  - `fn sum<I>(iter: I) -> TimeDelta`
- **Mul**
  - `fn mul(self: Self, rhs: i32) -> TimeDelta`
- **Sum**
  - `fn sum<I>(iter: I) -> TimeDelta`
- **PartialEq**
  - `fn eq(self: &Self, other: &TimeDelta) -> bool`
- **Sub**
  - `fn sub(self: Self, rhs: TimeDelta) -> TimeDelta`
- **Neg**
  - `fn neg(self: Self) -> TimeDelta`
- **Default**
  - `fn default() -> TimeDelta`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: TimeDelta)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &TimeDelta) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Div**
  - `fn div(self: Self, rhs: i32) -> TimeDelta`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: TimeDelta)`
- **Ord**
  - `fn cmp(self: &Self, other: &TimeDelta) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Format a `TimeDelta` using the [ISO 8601] format
- **Add**
  - `fn add(self: Self, rhs: TimeDelta) -> TimeDelta`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> TimeDelta`



