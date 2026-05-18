*[chrono](../index.md) / [time_delta](index.md)*

---

# Module `time_delta`

Temporal quantification

## Contents

- [Modules](#modules)
  - [`serde`](#serde)
- [Structs](#structs)
  - [`TimeDelta`](#timedelta)
  - [`OutOfRangeError`](#outofrangeerror)
- [Functions](#functions)
  - [`div_mod_floor_64`](#div-mod-floor-64)
- [Constants](#constants)
  - [`NANOS_PER_MICRO`](#nanos-per-micro)
  - [`NANOS_PER_MILLI`](#nanos-per-milli)
  - [`NANOS_PER_SEC`](#nanos-per-sec)
  - [`MICROS_PER_SEC`](#micros-per-sec)
  - [`MILLIS_PER_SEC`](#millis-per-sec)
  - [`SECS_PER_MINUTE`](#secs-per-minute)
  - [`SECS_PER_HOUR`](#secs-per-hour)
  - [`SECS_PER_DAY`](#secs-per-day)
  - [`SECS_PER_WEEK`](#secs-per-week)
  - [`MIN`](#min)
  - [`MAX`](#max)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serde`](#serde) | mod |  |
| [`TimeDelta`](#timedelta) | struct | Time duration with nanosecond precision. |
| [`OutOfRangeError`](#outofrangeerror) | struct | Represents error when converting `TimeDelta` to/from a standard library implementation |
| [`div_mod_floor_64`](#div-mod-floor-64) | fn |  |
| [`NANOS_PER_MICRO`](#nanos-per-micro) | const | The number of nanoseconds in a microsecond. |
| [`NANOS_PER_MILLI`](#nanos-per-milli) | const | The number of nanoseconds in a millisecond. |
| [`NANOS_PER_SEC`](#nanos-per-sec) | const | The number of nanoseconds in seconds. |
| [`MICROS_PER_SEC`](#micros-per-sec) | const | The number of microseconds per second. |
| [`MILLIS_PER_SEC`](#millis-per-sec) | const | The number of milliseconds per second. |
| [`SECS_PER_MINUTE`](#secs-per-minute) | const | The number of seconds in a minute. |
| [`SECS_PER_HOUR`](#secs-per-hour) | const | The number of seconds in an hour. |
| [`SECS_PER_DAY`](#secs-per-day) | const | The number of (non-leap) seconds in days. |
| [`SECS_PER_WEEK`](#secs-per-week) | const | The number of (non-leap) seconds in a week. |
| [`MIN`](#min) | const | The minimum possible `TimeDelta`: `-i64::MAX` milliseconds. |
| [`MAX`](#max) | const | The maximum possible `TimeDelta`: `i64::MAX` milliseconds. |

## Modules

- [`serde`](serde/index.md)

## Structs

### `TimeDelta`

```rust
struct TimeDelta {
    secs: i64,
    nanos: i32,
}
```

Time duration with nanosecond precision.

This also allows for negative durations; see individual methods for details.

A `TimeDelta` is represented internally as a complement of seconds and
nanoseconds. The range is restricted to that of `i64` milliseconds, with the
minimum value notably being set to `-i64::MAX` rather than allowing the full
range of `i64::MIN`. This is to allow easy flipping of sign, so that for
instance `abs()` can be called without any checks.

#### Implementations

- <span id="timedelta-new"></span>`const fn new(secs: i64, nanos: u32) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with given number of seconds and nanoseconds.

  

  # Errors

  

  Returns `None` when the duration is out of bounds, or if `nanos` ≥ 1,000,000,000.

- <span id="timedelta-weeks"></span>`const fn weeks(weeks: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of weeks.

  

  Equivalent to `TimeDelta::seconds(weeks * 7 * 24 * 60 * 60)` with

  overflow checks.

  

  # Panics

  

  Panics when the duration is out of bounds.

- <span id="timedelta-try-weeks"></span>`const fn try_weeks(weeks: i64) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of weeks.

  

  Equivalent to `TimeDelta::try_seconds(weeks * 7 * 24 * 60 * 60)` with

  overflow checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-days"></span>`const fn days(days: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of days.

  

  Equivalent to `TimeDelta::seconds(days * 24 * 60 * 60)` with overflow

  checks.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds.

- <span id="timedelta-try-days"></span>`const fn try_days(days: i64) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of days.

  

  Equivalent to `TimeDelta::try_seconds(days * 24 * 60 * 60)` with overflow

  checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-hours"></span>`const fn hours(hours: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of hours.

  

  Equivalent to `TimeDelta::seconds(hours * 60 * 60)` with overflow checks.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds.

- <span id="timedelta-try-hours"></span>`const fn try_hours(hours: i64) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of hours.

  

  Equivalent to `TimeDelta::try_seconds(hours * 60 * 60)` with overflow checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-minutes"></span>`const fn minutes(minutes: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of minutes.

  

  Equivalent to `TimeDelta::seconds(minutes * 60)` with overflow checks.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds.

- <span id="timedelta-try-minutes"></span>`const fn try_minutes(minutes: i64) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of minutes.

  

  Equivalent to `TimeDelta::try_seconds(minutes * 60)` with overflow checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-seconds"></span>`const fn seconds(seconds: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of seconds.

  

  # Panics

  

  Panics when `seconds` is more than `i64::MAX / 1_000` or less than `-i64::MAX / 1_000`

  (in this context, this is the same as `i64::MIN / 1_000` due to rounding).

- <span id="timedelta-try-seconds"></span>`const fn try_seconds(seconds: i64) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of seconds.

  

  # Errors

  

  Returns `None` when `seconds` is more than `i64::MAX / 1_000` or less than

  `-i64::MAX / 1_000` (in this context, this is the same as `i64::MIN / 1_000` due to

  rounding).

- <span id="timedelta-milliseconds"></span>`const fn milliseconds(milliseconds: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of milliseconds.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds, i.e. when `milliseconds` is more than

  `i64::MAX` or less than `-i64::MAX`. Notably, this is not the same as `i64::MIN`.

- <span id="timedelta-try-milliseconds"></span>`const fn try_milliseconds(milliseconds: i64) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of milliseconds.

  

  # Errors

  

  Returns `None` the `TimeDelta` would be out of bounds, i.e. when `milliseconds` is more

  than `i64::MAX` or less than `-i64::MAX`. Notably, this is not the same as `i64::MIN`.

- <span id="timedelta-microseconds"></span>`const fn microseconds(microseconds: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of microseconds.

  

  The number of microseconds acceptable by this constructor is less than

  the total number that can actually be stored in a `TimeDelta`, so it is

  not possible to specify a value that would be out of bounds. This

  function is therefore infallible.

- <span id="timedelta-nanoseconds"></span>`const fn nanoseconds(nanos: i64) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Makes a new `TimeDelta` with the given number of nanoseconds.

  

  The number of nanoseconds acceptable by this constructor is less than

  the total number that can actually be stored in a `TimeDelta`, so it is

  not possible to specify a value that would be out of bounds. This

  function is therefore infallible.

- <span id="timedelta-num-weeks"></span>`const fn num_weeks(&self) -> i64`

  Returns the total number of whole weeks in the `TimeDelta`.

- <span id="timedelta-num-days"></span>`const fn num_days(&self) -> i64`

  Returns the total number of whole days in the `TimeDelta`.

- <span id="timedelta-num-hours"></span>`const fn num_hours(&self) -> i64`

  Returns the total number of whole hours in the `TimeDelta`.

- <span id="timedelta-num-minutes"></span>`const fn num_minutes(&self) -> i64`

  Returns the total number of whole minutes in the `TimeDelta`.

- <span id="timedelta-num-seconds"></span>`const fn num_seconds(&self) -> i64`

  Returns the total number of whole seconds in the `TimeDelta`.

- <span id="timedelta-as-seconds-f64"></span>`fn as_seconds_f64(self) -> f64`

  Returns the fractional number of seconds in the `TimeDelta`.

- <span id="timedelta-as-seconds-f32"></span>`fn as_seconds_f32(self) -> f32`

  Returns the fractional number of seconds in the `TimeDelta`.

- <span id="timedelta-num-milliseconds"></span>`const fn num_milliseconds(&self) -> i64`

  Returns the total number of whole milliseconds in the `TimeDelta`.

- <span id="timedelta-subsec-millis"></span>`const fn subsec_millis(&self) -> i32`

  Returns the number of milliseconds in the fractional part of the duration.

  

  This is the number of milliseconds such that

  `subsec_millis() + num_seconds() * 1_000` is the truncated number of

  milliseconds in the duration.

- <span id="timedelta-num-microseconds"></span>`const fn num_microseconds(&self) -> Option<i64>`

  Returns the total number of whole microseconds in the `TimeDelta`,

  or `None` on overflow (exceeding 2^63 microseconds in either direction).

- <span id="timedelta-subsec-micros"></span>`const fn subsec_micros(&self) -> i32`

  Returns the number of microseconds in the fractional part of the duration.

  

  This is the number of microseconds such that

  `subsec_micros() + num_seconds() * 1_000_000` is the truncated number of

  microseconds in the duration.

- <span id="timedelta-num-nanoseconds"></span>`const fn num_nanoseconds(&self) -> Option<i64>`

  Returns the total number of whole nanoseconds in the `TimeDelta`,

  or `None` on overflow (exceeding 2^63 nanoseconds in either direction).

- <span id="timedelta-subsec-nanos"></span>`const fn subsec_nanos(&self) -> i32`

  Returns the number of nanoseconds in the fractional part of the duration.

  

  This is the number of nanoseconds such that

  `subsec_nanos() + num_seconds() * 1_000_000_000` is the total number of

  nanoseconds in the `TimeDelta`.

- <span id="timedelta-checked-add"></span>`const fn checked_add(&self, rhs: &TimeDelta) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Add two `TimeDelta`s, returning `None` if overflow occurred.

- <span id="timedelta-checked-sub"></span>`const fn checked_sub(&self, rhs: &TimeDelta) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Subtract two `TimeDelta`s, returning `None` if overflow occurred.

- <span id="timedelta-checked-mul"></span>`const fn checked_mul(&self, rhs: i32) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Multiply a `TimeDelta` with a i32, returning `None` if overflow occurred.

- <span id="timedelta-checked-div"></span>`const fn checked_div(&self, rhs: i32) -> Option<TimeDelta>` — [`TimeDelta`](#timedelta)

  Divide a `TimeDelta` with a i32, returning `None` if dividing by 0.

- <span id="timedelta-abs"></span>`const fn abs(&self) -> TimeDelta` — [`TimeDelta`](#timedelta)

  Returns the `TimeDelta` as an absolute (non-negative) value.

- <span id="timedelta-min-value"></span>`const fn min_value() -> TimeDelta` — [`TimeDelta`](#timedelta)

  The minimum possible `TimeDelta`: `-i64::MAX` milliseconds.

- <span id="timedelta-max-value"></span>`const fn max_value() -> TimeDelta` — [`TimeDelta`](#timedelta)

  The maximum possible `TimeDelta`: `i64::MAX` milliseconds.

- <span id="timedelta-zero"></span>`const fn zero() -> TimeDelta` — [`TimeDelta`](#timedelta)

  A `TimeDelta` where the stored seconds and nanoseconds are equal to zero.

- <span id="timedelta-is-zero"></span>`const fn is_zero(&self) -> bool`

  Returns `true` if the `TimeDelta` equals `TimeDelta::zero()`.

- <span id="timedelta-from-std"></span>`const fn from_std(duration: Duration) -> Result<TimeDelta, OutOfRangeError>` — [`TimeDelta`](#timedelta), [`OutOfRangeError`](#outofrangeerror)

  Creates a `TimeDelta` object from `std::time::Duration`

  

  This function errors when original duration is larger than the maximum

  value supported for this type.

- <span id="timedelta-to-std"></span>`const fn to_std(&self) -> Result<Duration, OutOfRangeError>` — [`OutOfRangeError`](#outofrangeerror)

  Creates a `std::time::Duration` object from a `TimeDelta`.

  

  This function errors when duration is less than zero. As standard

  library implementation is limited to non-negative values.

- <span id="timedelta-neg"></span>`const fn neg(self) -> TimeDelta` — [`TimeDelta`](#timedelta)

  This duplicates `Neg::neg` because trait methods can't be const yet.

- <span id="timedelta-const-min"></span>`const MIN: Self`

- <span id="timedelta-const-max"></span>`const MAX: Self`

#### Trait Implementations

##### `impl Add for TimeDelta`

- <span id="timedelta-add-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-add"></span>`fn add(self, rhs: TimeDelta) -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl AddAssign for TimeDelta`

- <span id="timedelta-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](#timedelta)

##### `impl Clone for TimeDelta`

- <span id="timedelta-clone"></span>`fn clone(&self) -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl Copy for TimeDelta`

##### `impl Debug for TimeDelta`

- <span id="timedelta-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TimeDelta`

- <span id="timedelta-default"></span>`fn default() -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl Deserialize for super::TimeDelta`

- <span id="supertimedelta-deserialize"></span>`fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for TimeDelta`

##### `impl Display for TimeDelta`

- <span id="timedelta-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format a `TimeDelta` using the [ISO 8601] format

##### `impl Div for TimeDelta`

- <span id="timedelta-div-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-div"></span>`fn div(self, rhs: i32) -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl Eq for TimeDelta`

##### `impl Hash for TimeDelta`

- <span id="timedelta-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Mul for TimeDelta`

- <span id="timedelta-mul-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-mul"></span>`fn mul(self, rhs: i32) -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl Neg for TimeDelta`

- <span id="timedelta-neg-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-neg"></span>`fn neg(self) -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl Ord for TimeDelta`

- <span id="timedelta-ord-cmp"></span>`fn cmp(&self, other: &TimeDelta) -> cmp::Ordering` — [`TimeDelta`](#timedelta)

##### `impl PartialEq for TimeDelta`

- <span id="timedelta-partialeq-eq"></span>`fn eq(&self, other: &TimeDelta) -> bool` — [`TimeDelta`](#timedelta)

##### `impl PartialOrd for TimeDelta`

- <span id="timedelta-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &TimeDelta) -> option::Option<cmp::Ordering>` — [`TimeDelta`](#timedelta)

##### `impl Serialize for super::TimeDelta`

- <span id="supertimedelta-serialize"></span>`fn serialize<S: Serializer>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for TimeDelta`

##### `impl Sub for TimeDelta`

- <span id="timedelta-sub-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-sub"></span>`fn sub(self, rhs: TimeDelta) -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl SubAssign for TimeDelta`

- <span id="timedelta-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](#timedelta)

##### `impl Sum for TimeDelta`

- <span id="timedelta-sum"></span>`fn sum<I: Iterator<Item = &'a TimeDelta>>(iter: I) -> TimeDelta` — [`TimeDelta`](#timedelta)

##### `impl ToString for TimeDelta`

- <span id="timedelta-tostring-to-string"></span>`fn to_string(&self) -> String`

### `OutOfRangeError`

```rust
struct OutOfRangeError(());
```

Represents error when converting `TimeDelta` to/from a standard library
implementation

The `std::time::Duration` supports a range from zero to `u64::MAX`
*seconds*, while this module supports signed range of up to
`i64::MAX` of *milliseconds*.

#### Trait Implementations

##### `impl Clone for OutOfRangeError`

- <span id="outofrangeerror-clone"></span>`fn clone(&self) -> OutOfRangeError` — [`OutOfRangeError`](#outofrangeerror)

##### `impl Copy for OutOfRangeError`

##### `impl Debug for OutOfRangeError`

- <span id="outofrangeerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for OutOfRangeError`

- <span id="outofrangeerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OutOfRangeError`

##### `impl PartialEq for OutOfRangeError`

- <span id="outofrangeerror-partialeq-eq"></span>`fn eq(&self, other: &OutOfRangeError) -> bool` — [`OutOfRangeError`](#outofrangeerror)

##### `impl StructuralPartialEq for OutOfRangeError`

##### `impl ToString for OutOfRangeError`

- <span id="outofrangeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `div_mod_floor_64`

```rust
const fn div_mod_floor_64(this: i64, other: i64) -> (i64, i64)
```

## Constants

### `NANOS_PER_MICRO`
```rust
const NANOS_PER_MICRO: i32 = 1_000i32;
```

The number of nanoseconds in a microsecond.

### `NANOS_PER_MILLI`
```rust
const NANOS_PER_MILLI: i32 = 1_000_000i32;
```

The number of nanoseconds in a millisecond.

### `NANOS_PER_SEC`
```rust
const NANOS_PER_SEC: i32 = 1_000_000_000i32;
```

The number of nanoseconds in seconds.

### `MICROS_PER_SEC`
```rust
const MICROS_PER_SEC: i64 = 1_000_000i64;
```

The number of microseconds per second.

### `MILLIS_PER_SEC`
```rust
const MILLIS_PER_SEC: i64 = 1_000i64;
```

The number of milliseconds per second.

### `SECS_PER_MINUTE`
```rust
const SECS_PER_MINUTE: i64 = 60i64;
```

The number of seconds in a minute.

### `SECS_PER_HOUR`
```rust
const SECS_PER_HOUR: i64 = 3_600i64;
```

The number of seconds in an hour.

### `SECS_PER_DAY`
```rust
const SECS_PER_DAY: i64 = 86_400i64;
```

The number of (non-leap) seconds in days.

### `SECS_PER_WEEK`
```rust
const SECS_PER_WEEK: i64 = 604_800i64;
```

The number of (non-leap) seconds in a week.

### `MIN`
```rust
const MIN: TimeDelta;
```

The minimum possible `TimeDelta`: `-i64::MAX` milliseconds.

### `MAX`
```rust
const MAX: TimeDelta;
```

The maximum possible `TimeDelta`: `i64::MAX` milliseconds.

