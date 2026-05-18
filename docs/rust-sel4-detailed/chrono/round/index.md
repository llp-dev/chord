*[chrono](../index.md) / [round](index.md)*

---

# Module `round`

Functionality for rounding or truncating a `DateTime` by a `TimeDelta`.

## Contents

- [Enums](#enums)
  - [`RoundingError`](#roundingerror)
- [Traits](#traits)
  - [`SubsecRound`](#subsecround)
  - [`DurationRound`](#durationround)
- [Functions](#functions)
  - [`span_for_digits`](#span-for-digits)
  - [`duration_round`](#duration-round)
  - [`duration_trunc`](#duration-trunc)
  - [`duration_round_up`](#duration-round-up)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RoundingError`](#roundingerror) | enum | An error from rounding by `TimeDelta` |
| [`SubsecRound`](#subsecround) | trait | Extension trait for subsecond rounding or truncation to a maximum number of digits. |
| [`DurationRound`](#durationround) | trait | Extension trait for rounding or truncating a DateTime by a TimeDelta. |
| [`span_for_digits`](#span-for-digits) | fn |  |
| [`duration_round`](#duration-round) | fn |  |
| [`duration_trunc`](#duration-trunc) | fn |  |
| [`duration_round_up`](#duration-round-up) | fn |  |

## Enums

### `RoundingError`

```rust
enum RoundingError {
    DurationExceedsTimestamp,
    DurationExceedsLimit,
    TimestampExceedsLimit,
}
```

An error from rounding by `TimeDelta`

See: [`DurationRound`](#durationround)

#### Variants

- **`DurationExceedsTimestamp`**

  Error when the TimeDelta exceeds the TimeDelta from or until the Unix epoch.
  
  Note: this error is not produced anymore.

- **`DurationExceedsLimit`**

  Error when `TimeDelta.num_nanoseconds` exceeds the limit.
  
  ``` rust
  use chrono::{DurationRound, TimeDelta, RoundingError, NaiveDate};
  let dt = NaiveDate::from_ymd_opt(2260, 12, 31)
      .unwrap()
      .and_hms_nano_opt(23, 59, 59, 1_75_500_000)
      .unwrap()
      .and_utc();
  
  assert_eq!(
      dt.duration_round(TimeDelta::try_days(300 * 365).unwrap()),
      Err(RoundingError::DurationExceedsLimit)
  );
  ```

- **`TimestampExceedsLimit`**

  Error when `DateTime.timestamp_nanos` exceeds the limit.
  
  ``` rust
  use chrono::{DurationRound, TimeDelta, RoundingError, TimeZone, Utc};
  let dt = Utc.with_ymd_and_hms(2300, 12, 12, 0, 0, 0).unwrap();
  
  assert_eq!(
      dt.duration_round(TimeDelta::try_days(1).unwrap()),
      Err(RoundingError::TimestampExceedsLimit)
  );
  ```

#### Trait Implementations

##### `impl Clone for RoundingError`

- <span id="roundingerror-clone"></span>`fn clone(&self) -> RoundingError` — [`RoundingError`](#roundingerror)

##### `impl Copy for RoundingError`

##### `impl Debug for RoundingError`

- <span id="roundingerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RoundingError`

- <span id="roundingerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RoundingError`

##### `impl PartialEq for RoundingError`

- <span id="roundingerror-partialeq-eq"></span>`fn eq(&self, other: &RoundingError) -> bool` — [`RoundingError`](#roundingerror)

##### `impl StructuralPartialEq for RoundingError`

##### `impl ToString for RoundingError`

- <span id="roundingerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `SubsecRound`

```rust
trait SubsecRound { ... }
```

Extension trait for subsecond rounding or truncation to a maximum number
of digits. Rounding can be used to decrease the error variance when
serializing/persisting to lower precision. Truncation is the default
behavior in Chrono display formatting.  Either can be used to guarantee
equality (e.g. for testing) when round-tripping through a lower precision
format.

#### Required Methods

- `fn round_subsecs(self, digits: u16) -> Self`

  Return a copy rounded to the specified number of subsecond digits. With

- `fn trunc_subsecs(self, digits: u16) -> Self`

  Return a copy truncated to the specified number of subsecond

#### Implementors

- `T`

### `DurationRound`

```rust
trait DurationRound: Sized { ... }
```

Extension trait for rounding or truncating a DateTime by a TimeDelta.

# Limitations
Both rounding and truncating are done via `TimeDelta::num_nanoseconds` and
`DateTime::timestamp_nanos_opt`. This means that they will fail if either the
`TimeDelta` or the `DateTime` are too big to represented as nanoseconds. They
will also fail if the `TimeDelta` is bigger than the timestamp, negative or zero.

#### Associated Types

- `type Err: 2`

#### Required Methods

- `fn duration_round(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`

  Return a copy rounded by TimeDelta.

- `fn duration_trunc(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`

  Return a copy truncated by TimeDelta.

- `fn duration_round_up(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`

  Return a copy rounded **up** by TimeDelta.

#### Implementors

- [`DateTime`](../datetime/index.md#datetime)
- [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime)

## Functions

### `span_for_digits`

```rust
const fn span_for_digits(digits: u16) -> u32
```

### `duration_round`

```rust
fn duration_round<T>(naive: crate::NaiveDateTime, original: T, duration: crate::TimeDelta) -> Result<T, RoundingError>
where
    T: Timelike + Add<crate::TimeDelta, Output = T> + Sub<crate::TimeDelta, Output = T>
```

### `duration_trunc`

```rust
fn duration_trunc<T>(naive: crate::NaiveDateTime, original: T, duration: crate::TimeDelta) -> Result<T, RoundingError>
where
    T: Timelike + Add<crate::TimeDelta, Output = T> + Sub<crate::TimeDelta, Output = T>
```

### `duration_round_up`

```rust
fn duration_round_up<T>(naive: crate::NaiveDateTime, original: T, duration: crate::TimeDelta) -> Result<T, RoundingError>
where
    T: Timelike + Add<crate::TimeDelta, Output = T> + Sub<crate::TimeDelta, Output = T>
```

