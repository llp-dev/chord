**chrono > naive > time**

# Module: naive::time

## Contents

**Structs**

- [`NaiveTime`](#naivetime) - ISO 8601 time without timezone.

---

## chrono::naive::time::NaiveTime

*Struct*

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

```
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
# let _ = (t, dt1, dt2);
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

```
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
[`Timelike::with_nanosecond()`].

```
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

**Methods:**

- `fn from_hms(hour: u32, min: u32, sec: u32) -> NaiveTime` - Makes a new `NaiveTime` from hour, minute and second.
- `fn from_hms_opt(hour: u32, min: u32, sec: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` from hour, minute and second.
- `fn from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> NaiveTime` - Makes a new `NaiveTime` from hour, minute, second and millisecond.
- `fn from_hms_milli_opt(hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` from hour, minute, second and millisecond.
- `fn from_hms_micro(hour: u32, min: u32, sec: u32, micro: u32) -> NaiveTime` - Makes a new `NaiveTime` from hour, minute, second and microsecond.
- `fn from_hms_micro_opt(hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` from hour, minute, second and microsecond.
- `fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> NaiveTime` - Makes a new `NaiveTime` from hour, minute, second and nanosecond.
- `fn from_hms_nano_opt(hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` from hour, minute, second and nanosecond.
- `fn from_num_seconds_from_midnight(secs: u32, nano: u32) -> NaiveTime` - Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.
- `fn from_num_seconds_from_midnight_opt(secs: u32, nano: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.
- `fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveTime>` - Parses a string with the specified format string and returns a new `NaiveTime`.
- `fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveTime, &'a str)>` - Parses a string from a user-specified format into a new `NaiveTime` value, and a slice with
- `fn overflowing_add_signed(self: &Self, rhs: TimeDelta) -> (NaiveTime, i64)` - Adds given `TimeDelta` to the current time, and also returns the number of *seconds*
- `fn overflowing_sub_signed(self: &Self, rhs: TimeDelta) -> (NaiveTime, i64)` - Subtracts given `TimeDelta` from the current time, and also returns the number of *seconds*
- `fn signed_duration_since(self: Self, rhs: NaiveTime) -> TimeDelta` - Subtracts another `NaiveTime` from the current time.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **Default**
  - `fn default() -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FromStr**
  - `fn from_str(s: &str) -> ParseResult<NaiveTime>`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: TimeDelta)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &NaiveTime) -> $crate::cmp::Ordering`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: TimeDelta)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: Duration)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Duration)`
- **Clone**
  - `fn clone(self: &Self) -> NaiveTime`
- **Sub**
  - `fn sub(self: Self, rhs: TimeDelta) -> NaiveTime`
- **Sub**
  - `fn sub(self: Self, rhs: FixedOffset) -> NaiveTime`
- **Add**
  - `fn add(self: Self, rhs: FixedOffset) -> NaiveTime`
- **Timelike**
  - `fn hour(self: &Self) -> u32` - Returns the hour number from 0 to 23.
  - `fn minute(self: &Self) -> u32` - Returns the minute number from 0 to 59.
  - `fn second(self: &Self) -> u32` - Returns the second number from 0 to 59.
  - `fn nanosecond(self: &Self) -> u32` - Returns the number of nanoseconds since the whole non-leap second.
  - `fn with_hour(self: &Self, hour: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` with the hour number changed.
  - `fn with_minute(self: &Self, min: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` with the minute number changed.
  - `fn with_second(self: &Self, sec: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` with the second number changed.
  - `fn with_nanosecond(self: &Self, nano: u32) -> Option<NaiveTime>` - Makes a new `NaiveTime` with nanoseconds since the whole non-leap second changed.
  - `fn num_seconds_from_midnight(self: &Self) -> u32` - Returns the number of non-leap seconds past the last midnight.
- **Sub**
  - `fn sub(self: Self, rhs: NaiveTime) -> TimeDelta`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NaiveTime) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Add**
  - `fn add(self: Self, rhs: TimeDelta) -> NaiveTime`
- **Sub**
  - `fn sub(self: Self, rhs: Duration) -> NaiveTime`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **Add**
  - `fn add(self: Self, rhs: Duration) -> NaiveTime`
- **PartialEq**
  - `fn eq(self: &Self, other: &NaiveTime) -> bool`



