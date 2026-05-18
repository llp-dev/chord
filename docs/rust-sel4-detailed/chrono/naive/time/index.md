*[chrono](../../index.md) / [naive](../index.md) / [time](index.md)*

---

# Module `time`

ISO 8601 time without timezone.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serde`](#serde) | mod |  |
| [`NaiveTime`](#naivetime) | struct | ISO 8601 time without timezone. |

## Modules

- [`serde`](serde/index.md)

## Structs

### `NaiveTime`

```rust
struct NaiveTime {
    secs: u32,
    frac: u32,
}
```

ISO 8601 time without timezone.
Allows for the nanosecond precision and optional leap second representation.

# Leap Second Handling

Since 1960s, the manmade atomic clock has been so accurate that
it is much more accurate than Earth's own motion.
It became desirable to define the civil time in terms of the atomic clock,
but that risks the desynchronization of the civil time from Earth.
To account for this, the designers of the Coordinated Universal Time (UTC)
made that the UTC should be kept within 0.9 seconds of the observed Earth-bound time.
When the mean solar day is longer than the ideal (86,400 seconds),
the error slowly accumulates and it is necessary to add a **leap second**
to slow the UTC down a bit.
(We may also remove a second to speed the UTC up a bit, but it never happened.)
The leap second, if any, follows 23:59:59 of June 30 or December 31 in the UTC.

Fast forward to the 21st century,
we have seen 26 leap seconds from January 1972 to December 2015.
Yes, 26 seconds. Probably you can read this paragraph within 26 seconds.
But those 26 seconds, and possibly more in the future, are never predictable,
and whether to add a leap second or not is known only before 6 months.
Internet-based clocks (via NTP) do account for known leap seconds,
but the system API normally doesn't (and often can't, with no network connection)
and there is no reliable way to retrieve leap second information.

Chrono does not try to accurately implement leap seconds; it is impossible.
Rather, **it allows for leap seconds but behaves as if there are *no other* leap seconds.**
Various operations will ignore any possible leap second(s)
except when any of the operands were actually leap seconds.

If you cannot tolerate this behavior,
you must use a separate `TimeZone` for the International Atomic Time (TAI).
TAI is like UTC but has no leap seconds, and thus slightly differs from UTC.
Chrono does not yet provide such implementation, but it is planned.

## Representing Leap Seconds

The leap second is indicated via fractional seconds more than 1 second.
This makes possible to treat a leap second as the prior non-leap second
if you don't care about sub-second accuracy.
You should use the proper formatting to get the raw leap second.

All methods accepting fractional seconds will accept such values.

```rust
use chrono::{NaiveDate, NaiveTime};

let t = NaiveTime::from_hms_milli_opt(8, 59, 59, 1_000).unwrap();

let dt1 = NaiveDate::from_ymd_opt(2015, 7, 1)
    .unwrap()
    .and_hms_micro_opt(8, 59, 59, 1_000_000)
    .unwrap();

let dt2 = NaiveDate::from_ymd_opt(2015, 6, 30)
    .unwrap()
    .and_hms_nano_opt(23, 59, 59, 1_000_000_000)
    .unwrap()
    .and_utc();
let _ = (t, dt1, dt2);
```

Note that the leap second can happen anytime given an appropriate time zone;
2015-07-01 01:23:60 would be a proper leap second if UTC+01:24 had existed.
Practically speaking, though, by the time of the first leap second on 1972-06-30,
every time zone offset around the world has standardized to the 5-minute alignment.

## Date And Time Arithmetic

As a concrete example, let's assume that `03:00:60` and `04:00:60` are leap seconds.
In reality, of course, leap seconds are separated by at least 6 months.
We will also use some intuitive concise notations for the explanation.

`Time + TimeDelta`
(short for [`NaiveTime::overflowing_add_signed`](#method.overflowing_add_signed)):

- `03:00:00 + 1s = 03:00:01`.
- `03:00:59 + 60s = 03:01:59`.
- `03:00:59 + 61s = 03:02:00`.
- `03:00:59 + 1s = 03:01:00`.
- `03:00:60 + 1s = 03:01:00`.
  Note that the sum is identical to the previous.
- `03:00:60 + 60s = 03:01:59`.
- `03:00:60 + 61s = 03:02:00`.
- `03:00:60.1 + 0.8s = 03:00:60.9`.

`Time - TimeDelta`
(short for [`NaiveTime::overflowing_sub_signed`](#method.overflowing_sub_signed)):

- `03:00:00 - 1s = 02:59:59`.
- `03:01:00 - 1s = 03:00:59`.
- `03:01:00 - 60s = 03:00:00`.
- `03:00:60 - 60s = 03:00:00`.
  Note that the result is identical to the previous.
- `03:00:60.7 - 0.4s = 03:00:60.3`.
- `03:00:60.7 - 0.9s = 03:00:59.8`.

`Time - Time`
(short for [`NaiveTime::signed_duration_since`](#method.signed_duration_since)):

- `04:00:00 - 03:00:00 = 3600s`.
- `03:01:00 - 03:00:00 = 60s`.
- `03:00:60 - 03:00:00 = 60s`.
  Note that the difference is identical to the previous.
- `03:00:60.6 - 03:00:59.4 = 1.2s`.
- `03:01:00 - 03:00:59.8 = 0.2s`.
- `03:01:00 - 03:00:60.5 = 0.5s`.
  Note that the difference is larger than the previous,
  even though the leap second clearly follows the previous whole second.
- `04:00:60.9 - 03:00:60.1 =
  (04:00:60.9 - 04:00:00) + (04:00:00 - 03:01:00) + (03:01:00 - 03:00:60.1) =
  60.9s + 3540s + 0.9s = 3601.8s`.

In general,

- `Time + TimeDelta` unconditionally equals to `TimeDelta + Time`.

- `Time - TimeDelta` unconditionally equals to `Time + (-TimeDelta)`.

- `Time1 - Time2` unconditionally equals to `-(Time2 - Time1)`.

- Associativity does not generally hold, because
  `(Time + TimeDelta1) - TimeDelta2` no longer equals to `Time + (TimeDelta1 - TimeDelta2)`
  for two positive durations.

    - As a special case, `(Time + TimeDelta) - TimeDelta` also does not equal to `Time`.

    - If you can assume that all durations have the same sign, however,
      then the associativity holds:
      `(Time + TimeDelta1) + TimeDelta2` equals to `Time + (TimeDelta1 + TimeDelta2)`
      for two positive durations.

## Reading And Writing Leap Seconds

The "typical" leap seconds on the minute boundary are
correctly handled both in the formatting and parsing.
The leap second in the human-readable representation
will be represented as the second part being 60, as required by ISO 8601.

```rust
use chrono::NaiveDate;

let dt = NaiveDate::from_ymd_opt(2015, 6, 30)
    .unwrap()
    .and_hms_milli_opt(23, 59, 59, 1_000)
    .unwrap()
    .and_utc();
assert_eq!(format!("{:?}", dt), "2015-06-30T23:59:60Z");
```

There are hypothetical leap seconds not on the minute boundary nevertheless supported by Chrono.
They are allowed for the sake of completeness and consistency; there were several "exotic" time
zone offsets with fractional minutes prior to UTC after all.
For such cases the human-readable representation is ambiguous and would be read back to the next
non-leap second.

A `NaiveTime` with a leap second that is not on a minute boundary can only be created from a
[`DateTime`](crate::DateTime) with fractional minutes as offset, or using
`Timelike::with_nanosecond()`.

```rust
use chrono::{FixedOffset, NaiveDate, TimeZone};

let paramaribo_pre1945 = FixedOffset::east_opt(-13236).unwrap(); // -03:40:36
let leap_sec_2015 =
    NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_milli_opt(23, 59, 59, 1_000).unwrap();
let dt1 = paramaribo_pre1945.from_utc_datetime(&leap_sec_2015);
assert_eq!(format!("{:?}", dt1), "2015-06-30T20:19:24-03:40:36");
assert_eq!(format!("{:?}", dt1.time()), "20:19:24");

let next_sec = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
let dt2 = paramaribo_pre1945.from_utc_datetime(&next_sec);
assert_eq!(format!("{:?}", dt2), "2015-06-30T20:19:24-03:40:36");
assert_eq!(format!("{:?}", dt2.time()), "20:19:24");

assert!(dt1.time() != dt2.time());
assert!(dt1.time().to_string() == dt2.time().to_string());
```

Since Chrono alone cannot determine any existence of leap seconds,
**there is absolutely no guarantee that the leap second read has actually happened**.

#### Implementations

- <span id="naivetime-from-hms"></span>`const fn from_hms(hour: u32, min: u32, sec: u32) -> NaiveTime` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute and second.

  

  No [leap second](#leap-second-handling) is allowed here;

  use `NaiveTime::from_hms_*` methods with a subsecond parameter instead.

  

  # Panics

  

  Panics on invalid hour, minute and/or second.

- <span id="naivetime-from-hms-opt"></span>`const fn from_hms_opt(hour: u32, min: u32, sec: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute and second.

  

  The millisecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute and/or second.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hms_opt = NaiveTime::from_hms_opt;

  

  assert!(from_hms_opt(0, 0, 0).is_some());

  assert!(from_hms_opt(23, 59, 59).is_some());

  assert!(from_hms_opt(24, 0, 0).is_none());

  assert!(from_hms_opt(23, 60, 0).is_none());

  assert!(from_hms_opt(23, 59, 60).is_none());

  ```

- <span id="naivetime-from-hms-milli"></span>`const fn from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> NaiveTime` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and millisecond.

  

  The millisecond part can exceed 1,000

  in order to represent the [leap second](#leap-second-handling).

  

  # Panics

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="naivetime-from-hms-milli-opt"></span>`const fn from_hms_milli_opt(hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and millisecond.

  

  The millisecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or millisecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hmsm_opt = NaiveTime::from_hms_milli_opt;

  

  assert!(from_hmsm_opt(0, 0, 0, 0).is_some());

  assert!(from_hmsm_opt(23, 59, 59, 999).is_some());

  assert!(from_hmsm_opt(23, 59, 59, 1_999).is_some()); // a leap second after 23:59:59

  assert!(from_hmsm_opt(24, 0, 0, 0).is_none());

  assert!(from_hmsm_opt(23, 60, 0, 0).is_none());

  assert!(from_hmsm_opt(23, 59, 60, 0).is_none());

  assert!(from_hmsm_opt(23, 59, 59, 2_000).is_none());

  ```

- <span id="naivetime-from-hms-micro"></span>`const fn from_hms_micro(hour: u32, min: u32, sec: u32, micro: u32) -> NaiveTime` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or microsecond.

- <span id="naivetime-from-hms-micro-opt"></span>`const fn from_hms_micro_opt(hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or microsecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hmsu_opt = NaiveTime::from_hms_micro_opt;

  

  assert!(from_hmsu_opt(0, 0, 0, 0).is_some());

  assert!(from_hmsu_opt(23, 59, 59, 999_999).is_some());

  assert!(from_hmsu_opt(23, 59, 59, 1_999_999).is_some()); // a leap second after 23:59:59

  assert!(from_hmsu_opt(24, 0, 0, 0).is_none());

  assert!(from_hmsu_opt(23, 60, 0, 0).is_none());

  assert!(from_hmsu_opt(23, 59, 60, 0).is_none());

  assert!(from_hmsu_opt(23, 59, 59, 2_000_000).is_none());

  ```

- <span id="naivetime-from-hms-nano"></span>`const fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> NaiveTime` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="naivetime-from-hms-nano-opt"></span>`const fn from_hms_nano_opt(hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or nanosecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hmsn_opt = NaiveTime::from_hms_nano_opt;

  

  assert!(from_hmsn_opt(0, 0, 0, 0).is_some());

  assert!(from_hmsn_opt(23, 59, 59, 999_999_999).is_some());

  assert!(from_hmsn_opt(23, 59, 59, 1_999_999_999).is_some()); // a leap second after 23:59:59

  assert!(from_hmsn_opt(24, 0, 0, 0).is_none());

  assert!(from_hmsn_opt(23, 60, 0, 0).is_none());

  assert!(from_hmsn_opt(23, 59, 60, 0).is_none());

  assert!(from_hmsn_opt(23, 59, 59, 2_000_000_000).is_none());

  ```

- <span id="naivetime-from-num-seconds-from-midnight"></span>`const fn from_num_seconds_from_midnight(secs: u32, nano: u32) -> NaiveTime` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `secs % 60 == 59`.

  

  # Panics

  

  Panics on invalid number of seconds and/or nanosecond.

- <span id="naivetime-from-num-seconds-from-midnight-opt"></span>`const fn from_num_seconds_from_midnight_opt(secs: u32, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `secs % 60 == 59`.

  

  # Errors

  

  Returns `None` on invalid number of seconds and/or nanosecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_nsecs_opt = NaiveTime::from_num_seconds_from_midnight_opt;

  

  assert!(from_nsecs_opt(0, 0).is_some());

  assert!(from_nsecs_opt(86399, 999_999_999).is_some());

  assert!(from_nsecs_opt(86399, 1_999_999_999).is_some()); // a leap second after 23:59:59

  assert!(from_nsecs_opt(86_400, 0).is_none());

  assert!(from_nsecs_opt(86399, 2_000_000_000).is_none());

  ```

- <span id="naivetime-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveTime>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveTime`](#naivetime)

  Parses a string with the specified format string and returns a new `NaiveTime`.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let parse_from_str = NaiveTime::parse_from_str;

  

  assert_eq!(

      parse_from_str("23:56:04", "%H:%M:%S"),

      Ok(NaiveTime::from_hms_opt(23, 56, 4).unwrap())

  );

  assert_eq!(

      parse_from_str("pm012345.6789", "%p%I%M%S%.f"),

      Ok(NaiveTime::from_hms_micro_opt(13, 23, 45, 678_900).unwrap())

  );

  ```

  

  Date and offset is ignored for the purpose of parsing.

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert_eq!(

      parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),

      Ok(NaiveTime::from_hms_opt(12, 34, 56).unwrap())

  );

  ```

  

  [Leap seconds](#leap-second-handling) are correctly handled by

  treating any time of the form `hh:mm:60` as a leap second.

  (This equally applies to the formatting, so the round trip is possible.)

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert_eq!(

      parse_from_str("08:59:60.123", "%H:%M:%S%.f"),

      Ok(NaiveTime::from_hms_milli_opt(8, 59, 59, 1_123).unwrap())

  );

  ```

  

  Missing seconds are assumed to be zero,

  but out-of-bound times or insufficient fields are errors otherwise.

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert_eq!(parse_from_str("7:15", "%H:%M"), Ok(NaiveTime::from_hms_opt(7, 15, 0).unwrap()));

  

  assert!(parse_from_str("04m33s", "%Mm%Ss").is_err());

  assert!(parse_from_str("12", "%H").is_err());

  assert!(parse_from_str("17:60", "%H:%M").is_err());

  assert!(parse_from_str("24:00:00", "%H:%M:%S").is_err());

  ```

  

  All parsed fields should be consistent to each other, otherwise it's an error.

  Here `%H` is for 24-hour clocks, unlike `%I`,

  and thus can be independently determined without AM/PM.

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert!(parse_from_str("13:07 AM", "%H:%M %p").is_err());

  ```

- <span id="naivetime-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveTime, &'a str)>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveTime`](#naivetime)

  Parses a string from a user-specified format into a new `NaiveTime` value, and a slice with

  the remaining portion of the string.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  Similar to [`parse_from_str`](#method.parse_from_str).

  

  # Example

  

  ```rust

  use chrono::{NaiveTime};

  let (time, remainder) =

      NaiveTime::parse_and_remainder("3h4m33s trailing text", "%-Hh%-Mm%-Ss").unwrap();

  assert_eq!(time, NaiveTime::from_hms_opt(3, 4, 33).unwrap());

  assert_eq!(remainder, " trailing text");

  ```

- <span id="naivetime-overflowing-add-signed"></span>`const fn overflowing_add_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64)` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveTime`](#naivetime)

  Adds given `TimeDelta` to the current time, and also returns the number of *seconds*

  in the integral number of days ignored from the addition.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, TimeDelta};

  

  let from_hms = |h, m, s| NaiveTime::from_hms_opt(h, m, s).unwrap();

  

  assert_eq!(

      from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::try_hours(11).unwrap()),

      (from_hms(14, 4, 5), 0)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::try_hours(23).unwrap()),

      (from_hms(2, 4, 5), 86_400)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::try_hours(-7).unwrap()),

      (from_hms(20, 4, 5), -86_400)

  );

  ```

- <span id="naivetime-overflowing-sub-signed"></span>`const fn overflowing_sub_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64)` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveTime`](#naivetime)

  Subtracts given `TimeDelta` from the current time, and also returns the number of *seconds*

  in the integral number of days ignored from the subtraction.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, TimeDelta};

  

  let from_hms = |h, m, s| NaiveTime::from_hms_opt(h, m, s).unwrap();

  

  assert_eq!(

      from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::try_hours(2).unwrap()),

      (from_hms(1, 4, 5), 0)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::try_hours(17).unwrap()),

      (from_hms(10, 4, 5), 86_400)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::try_hours(-22).unwrap()),

      (from_hms(1, 4, 5), -86_400)

  );

  ```

- <span id="naivetime-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: NaiveTime) -> TimeDelta` — [`NaiveTime`](#naivetime), [`TimeDelta`](../../time_delta/index.md#timedelta)

  Subtracts another `NaiveTime` from the current time.

  Returns a `TimeDelta` within +/- 1 day.

  This does not overflow or underflow at all.

  

  As a part of Chrono's [leap second handling](#leap-second-handling),

  the subtraction assumes that **there is no leap second ever**,

  except when any of the `NaiveTime`s themselves represents a leap second

  in which case the assumption becomes that

  **there are exactly one (or two) leap second(s) ever**.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, TimeDelta};

  

  let from_hmsm = |h, m, s, milli| NaiveTime::from_hms_milli_opt(h, m, s, milli).unwrap();

  let since = NaiveTime::signed_duration_since;

  

  assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 7, 900)), TimeDelta::zero());

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 7, 875)),

      TimeDelta::try_milliseconds(25).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 6, 925)),

      TimeDelta::try_milliseconds(975).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 0, 900)),

      TimeDelta::try_seconds(7).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 0, 7, 900)),

      TimeDelta::try_seconds(5 * 60).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(0, 5, 7, 900)),

      TimeDelta::try_seconds(3 * 3600).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(4, 5, 7, 900)),

      TimeDelta::try_seconds(-3600).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(2, 4, 6, 800)),

      TimeDelta::try_seconds(3600 + 60 + 1).unwrap() + TimeDelta::try_milliseconds(100).unwrap()

  );

  ```

  

  Leap seconds are handled, but the subtraction assumes that

  there were no other leap seconds happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveTime};

  let from_hmsm = |h, m, s, milli| { NaiveTime::from_hms_milli_opt(h, m, s, milli).unwrap() };

  let since = NaiveTime::signed_duration_since;

  assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(3, 0, 59, 0)),

             TimeDelta::try_seconds(1).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 59, 1_500), from_hmsm(3, 0, 59, 0)),

             TimeDelta::try_milliseconds(1500).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(3, 0, 0, 0)),

             TimeDelta::try_seconds(60).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 0, 0), from_hmsm(2, 59, 59, 1_000)),

             TimeDelta::try_seconds(1).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(2, 59, 59, 1_000)),

             TimeDelta::try_seconds(61).unwrap());

  ```

- <span id="naivetime-overflowing-add-offset"></span>`const fn overflowing_add_offset(&self, offset: FixedOffset) -> (NaiveTime, i32)` — [`FixedOffset`](../../offset/fixed/index.md#fixedoffset), [`NaiveTime`](#naivetime)

  Adds given `FixedOffset` to the current time, and returns the number of days that should be

  added to a date as a result of the offset (either `-1`, `0`, or `1` because the offset is

  always less than 24h).

  

  This method is similar to [`overflowing_add_signed`](#method.overflowing_add_signed), but

  preserves leap seconds.

- <span id="naivetime-overflowing-sub-offset"></span>`const fn overflowing_sub_offset(&self, offset: FixedOffset) -> (NaiveTime, i32)` — [`FixedOffset`](../../offset/fixed/index.md#fixedoffset), [`NaiveTime`](#naivetime)

  Subtracts given `FixedOffset` from the current time, and returns the number of days that

  should be added to a date as a result of the offset (either `-1`, `0`, or `1` because the

  offset is always less than 24h).

  

  This method is similar to [`overflowing_sub_signed`](#method.overflowing_sub_signed), but

  preserves leap seconds.

- <span id="naivetime-hms"></span>`fn hms(&self) -> (u32, u32, u32)`

  Returns a triple of the hour, minute and second numbers.

- <span id="naivetime-num-seconds-from-midnight"></span>`const fn num_seconds_from_midnight(&self) -> u32`

  Returns the number of non-leap seconds past the last midnight.

- <span id="naivetime-nanosecond"></span>`const fn nanosecond(&self) -> u32`

  Returns the number of nanoseconds since the whole non-leap second.

- <span id="naivetime-const-min"></span>`const MIN: Self`

- <span id="naivetime-const-max"></span>`const MAX: Self`

#### Trait Implementations

##### `impl Add for NaiveTime`

- <span id="naivetime-add-type-output"></span>`type Output = NaiveTime`

- <span id="naivetime-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveTime` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveTime`](#naivetime)

##### `impl AddAssign for NaiveTime`

- <span id="naivetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../../time_delta/index.md#timedelta)

##### `impl Clone for NaiveTime`

- <span id="naivetime-clone"></span>`fn clone(&self) -> NaiveTime` — [`NaiveTime`](#naivetime)

##### `impl Copy for NaiveTime`

##### `impl Debug for NaiveTime`

- <span id="naivetime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NaiveTime`

- <span id="naivetime-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::NaiveTime`

- <span id="supernaivetime-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for NaiveTime`

##### `impl Display for NaiveTime`

- <span id="naivetime-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NaiveTime`

##### `impl FromStr for NaiveTime`

- <span id="naivetime-fromstr-type-err"></span>`type Err = ParseError`

- <span id="naivetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveTime>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveTime`](#naivetime)

##### `impl Hash for NaiveTime`

- <span id="naivetime-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveTime`

- <span id="naivetime-ord-cmp"></span>`fn cmp(&self, other: &NaiveTime) -> cmp::Ordering` — [`NaiveTime`](#naivetime)

##### `impl PartialEq for NaiveTime`

- <span id="naivetime-partialeq-eq"></span>`fn eq(&self, other: &NaiveTime) -> bool` — [`NaiveTime`](#naivetime)

##### `impl PartialOrd for NaiveTime`

- <span id="naivetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveTime) -> option::Option<cmp::Ordering>` — [`NaiveTime`](#naivetime)

##### `impl Serialize for super::NaiveTime`

- <span id="supernaivetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveTime`

##### `impl Sub for NaiveTime`

- <span id="naivetime-sub-type-output"></span>`type Output = NaiveTime`

- <span id="naivetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> NaiveTime` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveTime`](#naivetime)

##### `impl SubAssign for NaiveTime`

- <span id="naivetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../../time_delta/index.md#timedelta)

##### `impl SubsecRound for NaiveTime`

- <span id="naivetime-subsecround-round-subsecs"></span>`fn round_subsecs(self, digits: u16) -> T`

- <span id="naivetime-subsecround-trunc-subsecs"></span>`fn trunc_subsecs(self, digits: u16) -> T`

##### `impl Timelike for NaiveTime`

- <span id="naivetime-timelike-hour"></span>`fn hour(&self) -> u32`

  Returns the hour number from 0 to 23.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().hour(), 0);

  assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().hour(), 23);

  ```

- <span id="naivetime-timelike-minute"></span>`fn minute(&self) -> u32`

  Returns the minute number from 0 to 59.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().minute(), 0);

  assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().minute(), 56);

  ```

- <span id="naivetime-timelike-second"></span>`fn second(&self) -> u32`

  Returns the second number from 0 to 59.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().second(), 0);

  assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().second(), 4);

  ```

  

  This method never returns 60 even when it is a leap second.

  ([Why?](#leap-second-handling))

  Use the proper [formatting method](#method.format) to get a human-readable representation.

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{NaiveTime, Timelike};

  let leap = NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap();

  assert_eq!(leap.second(), 59);

  assert_eq!(leap.format("%H:%M:%S").to_string(), "23:59:60");

  }

  ```

- <span id="naivetime-timelike-nanosecond"></span>`fn nanosecond(&self) -> u32`

  Returns the number of nanoseconds since the whole non-leap second.

  The range from 1,000,000,000 to 1,999,999,999 represents

  the [leap second](#leap-second-handling).

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().nanosecond(), 0);

  assert_eq!(

      NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().nanosecond(),

      12_345_678

  );

  ```

  

  Leap seconds may have seemingly out-of-range return values.

  You can reduce the range with `time.nanosecond() % 1_000_000_000`, or

  use the proper [formatting method](#method.format) to get a human-readable representation.

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{NaiveTime, Timelike};

  let leap = NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap();

  assert_eq!(leap.nanosecond(), 1_000_000_000);

  assert_eq!(leap.format("%H:%M:%S%.9f").to_string(), "23:59:60.000000000");

  }

  ```

- <span id="naivetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` with the hour number changed.

  

  # Errors

  

  Returns `None` if the value for `hour` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(dt.with_hour(7), Some(NaiveTime::from_hms_nano_opt(7, 56, 4, 12_345_678).unwrap()));

  assert_eq!(dt.with_hour(24), None);

  ```

- <span id="naivetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` with the minute number changed.

  

  # Errors

  

  Returns `None` if the value for `minute` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(

      dt.with_minute(45),

      Some(NaiveTime::from_hms_nano_opt(23, 45, 4, 12_345_678).unwrap())

  );

  assert_eq!(dt.with_minute(60), None);

  ```

- <span id="naivetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` with the second number changed.

  

  As with the [`second`](#method.second) method,

  the input range is restricted to 0 through 59.

  

  # Errors

  

  Returns `None` if the value for `second` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(

      dt.with_second(17),

      Some(NaiveTime::from_hms_nano_opt(23, 56, 17, 12_345_678).unwrap())

  );

  assert_eq!(dt.with_second(60), None);

  ```

- <span id="naivetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](#naivetime)

  Makes a new `NaiveTime` with nanoseconds since the whole non-leap second changed.

  

  As with the [`nanosecond`](#method.nanosecond) method,

  the input range can exceed 1,000,000,000 for leap seconds.

  

  # Errors

  

  Returns `None` if `nanosecond >= 2,000,000,000`.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(

      dt.with_nanosecond(333_333_333),

      Some(NaiveTime::from_hms_nano_opt(23, 56, 4, 333_333_333).unwrap())

  );

  assert_eq!(dt.with_nanosecond(2_000_000_000), None);

  ```

  

  Leap seconds can theoretically follow *any* whole second.

  The following would be a proper leap second at the time zone offset of UTC-00:03:57

  (there are several historical examples comparable to this "non-sense" offset),

  and therefore is allowed.

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  let strange_leap_second = dt.with_nanosecond(1_333_333_333).unwrap();

  assert_eq!(strange_leap_second.nanosecond(), 1_333_333_333);

  ```

- <span id="naivetime-timelike-num-seconds-from-midnight"></span>`fn num_seconds_from_midnight(&self) -> u32`

  Returns the number of non-leap seconds past the last midnight.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(1, 2, 3).unwrap().num_seconds_from_midnight(), 3723);

  assert_eq!(

      NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().num_seconds_from_midnight(),

      86164

  );

  assert_eq!(

      NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap().num_seconds_from_midnight(),

      86399

  );

  ```

##### `impl ToString for NaiveTime`

- <span id="naivetime-tostring-to-string"></span>`fn to_string(&self) -> String`

