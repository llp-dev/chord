**chrono > round**

# Module: round

## Contents

**Enums**

- [`RoundingError`](#roundingerror) - An error from rounding by `TimeDelta`

**Traits**

- [`DurationRound`](#durationround) - Extension trait for rounding or truncating a DateTime by a TimeDelta.
- [`SubsecRound`](#subsecround) - Extension trait for subsecond rounding or truncation to a maximum number

---

## chrono::round::DurationRound

*Trait*

Extension trait for rounding or truncating a DateTime by a TimeDelta.

# Limitations
Both rounding and truncating are done via [`TimeDelta::num_nanoseconds`] and
[`DateTime::timestamp_nanos_opt`]. This means that they will fail if either the
`TimeDelta` or the `DateTime` are too big to represented as nanoseconds. They
will also fail if the `TimeDelta` is bigger than the timestamp, negative or zero.

**Methods:**

- `Err`: Error that can occur in rounding or truncating
- `duration_round`: Return a copy rounded by TimeDelta.
- `duration_trunc`: Return a copy truncated by TimeDelta.
- `duration_round_up`: Return a copy rounded **up** by TimeDelta.



## chrono::round::RoundingError

*Enum*

An error from rounding by `TimeDelta`

See: [`DurationRound`]

**Variants:**
- `DurationExceedsTimestamp` - Error when the TimeDelta exceeds the TimeDelta from or until the Unix epoch.
- `DurationExceedsLimit` - Error when `TimeDelta.num_nanoseconds` exceeds the limit.
- `TimestampExceedsLimit` - Error when `DateTime.timestamp_nanos` exceeds the limit.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RoundingError`
- **PartialEq**
  - `fn eq(self: &Self, other: &RoundingError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## chrono::round::SubsecRound

*Trait*

Extension trait for subsecond rounding or truncation to a maximum number
of digits. Rounding can be used to decrease the error variance when
serializing/persisting to lower precision. Truncation is the default
behavior in Chrono display formatting.  Either can be used to guarantee
equality (e.g. for testing) when round-tripping through a lower precision
format.

**Methods:**

- `round_subsecs`: Return a copy rounded to the specified number of subsecond digits. With
- `trunc_subsecs`: Return a copy truncated to the specified number of subsecond



