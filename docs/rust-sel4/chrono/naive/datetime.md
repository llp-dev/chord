**chrono > naive > datetime**

# Module: naive::datetime

## Contents

**Structs**

- [`NaiveDateTime`](#naivedatetime) - ISO 8601 combined date and time without timezone.

**Constants**

- [`MAX_DATETIME`](#max_datetime) - The maximum possible `NaiveDateTime`.
- [`MIN_DATETIME`](#min_datetime) - The minimum possible `NaiveDateTime`.

---

## chrono::naive::datetime::MAX_DATETIME

*Constant*: `NaiveDateTime`

The maximum possible `NaiveDateTime`.



## chrono::naive::datetime::MIN_DATETIME

*Constant*: `NaiveDateTime`

The minimum possible `NaiveDateTime`.



## chrono::naive::datetime::NaiveDateTime

*Struct*

ISO 8601 combined date and time without timezone.

# Example

`NaiveDateTime` is commonly created from [`NaiveDate`].

```
use chrono::{NaiveDate, NaiveDateTime};

let dt: NaiveDateTime =
    NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
# let _ = dt;
```

You can use typical [date-like](Datelike) and [time-like](Timelike) methods,
provided that relevant traits are in the scope.

```
# use chrono::{NaiveDate, NaiveDateTime};
# let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
use chrono::{Datelike, Timelike, Weekday};

assert_eq!(dt.weekday(), Weekday::Fri);
assert_eq!(dt.num_seconds_from_midnight(), 33011);
```

**Methods:**

- `fn new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime` - Makes a new `NaiveDateTime` from date and time components.
- `fn from_timestamp(secs: i64, nsecs: u32) -> NaiveDateTime` - Makes a new `NaiveDateTime` corresponding to a UTC date and time,
- `fn from_timestamp_millis(millis: i64) -> Option<NaiveDateTime>` - Creates a new [NaiveDateTime] from milliseconds since the UNIX epoch.
- `fn from_timestamp_micros(micros: i64) -> Option<NaiveDateTime>` - Creates a new [NaiveDateTime] from microseconds since the UNIX epoch.
- `fn from_timestamp_nanos(nanos: i64) -> Option<NaiveDateTime>` - Creates a new [NaiveDateTime] from nanoseconds since the UNIX epoch.
- `fn from_timestamp_opt(secs: i64, nsecs: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` corresponding to a UTC date and time,
- `fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDateTime>` - Parses a string with the specified format string and returns a new `NaiveDateTime`.
- `fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDateTime, &'a str)>` - Parses a string with the specified format string and returns a new `NaiveDateTime`, and a
- `fn date(self: &Self) -> NaiveDate` - Retrieves a date component.
- `fn time(self: &Self) -> NaiveTime` - Retrieves a time component.
- `fn timestamp(self: &Self) -> i64` - Returns the number of non-leap seconds since the midnight on January 1, 1970.
- `fn timestamp_millis(self: &Self) -> i64` - Returns the number of non-leap *milliseconds* since midnight on January 1, 1970.
- `fn timestamp_micros(self: &Self) -> i64` - Returns the number of non-leap *microseconds* since midnight on January 1, 1970.
- `fn timestamp_nanos(self: &Self) -> i64` - Returns the number of non-leap *nanoseconds* since midnight on January 1, 1970.
- `fn timestamp_nanos_opt(self: &Self) -> Option<i64>` - Returns the number of non-leap *nanoseconds* since midnight on January 1, 1970.
- `fn timestamp_subsec_millis(self: &Self) -> u32` - Returns the number of milliseconds since the last whole non-leap second.
- `fn timestamp_subsec_micros(self: &Self) -> u32` - Returns the number of microseconds since the last whole non-leap second.
- `fn timestamp_subsec_nanos(self: &Self) -> u32` - Returns the number of nanoseconds since the last whole non-leap second.
- `fn checked_add_signed(self: Self, rhs: TimeDelta) -> Option<NaiveDateTime>` - Adds given `TimeDelta` to the current date and time.
- `fn checked_add_months(self: Self, rhs: Months) -> Option<NaiveDateTime>` - Adds given `Months` to the current date and time.
- `fn checked_add_offset(self: Self, rhs: FixedOffset) -> Option<NaiveDateTime>` - Adds given `FixedOffset` to the current datetime.
- `fn checked_sub_offset(self: Self, rhs: FixedOffset) -> Option<NaiveDateTime>` - Subtracts given `FixedOffset` from the current datetime.
- `fn checked_sub_signed(self: Self, rhs: TimeDelta) -> Option<NaiveDateTime>` - Subtracts given `TimeDelta` from the current date and time.
- `fn checked_sub_months(self: Self, rhs: Months) -> Option<NaiveDateTime>` - Subtracts given `Months` from the current date and time.
- `fn checked_add_days(self: Self, days: Days) -> Option<Self>` - Add a duration in [`Days`] to the date part of the `NaiveDateTime`
- `fn checked_sub_days(self: Self, days: Days) -> Option<Self>` - Subtract a duration in [`Days`] from the date part of the `NaiveDateTime`
- `fn signed_duration_since(self: Self, rhs: NaiveDateTime) -> TimeDelta` - Subtracts another `NaiveDateTime` from the current date and time.
- `fn and_local_timezone<Tz>(self: &Self, tz: Tz) -> MappedLocalTime<DateTime<Tz>>` - Converts the `NaiveDateTime` into a timezone-aware `DateTime<Tz>` with the provided
- `fn and_utc(self: &Self) -> DateTime<Utc>` - Converts the `NaiveDateTime` into the timezone-aware `DateTime<Utc>`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &NaiveDateTime) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> NaiveDateTime`
- **DurationRound**
  - `fn duration_round(self: Self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`
  - `fn duration_trunc(self: Self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`
  - `fn duration_round_up(self: Self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NaiveDateTime) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> Self`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Duration)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: Duration)`
- **Datelike**
  - `fn year(self: &Self) -> i32` - Returns the year number in the [calendar date](./struct.NaiveDate.html#calendar-date).
  - `fn month(self: &Self) -> u32` - Returns the month number starting from 1.
  - `fn month0(self: &Self) -> u32` - Returns the month number starting from 0.
  - `fn day(self: &Self) -> u32` - Returns the day of month starting from 1.
  - `fn day0(self: &Self) -> u32` - Returns the day of month starting from 0.
  - `fn ordinal(self: &Self) -> u32` - Returns the day of year starting from 1.
  - `fn ordinal0(self: &Self) -> u32` - Returns the day of year starting from 0.
  - `fn weekday(self: &Self) -> Weekday` - Returns the day of week.
  - `fn iso_week(self: &Self) -> IsoWeek`
  - `fn with_year(self: &Self, year: i32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the year number changed, while keeping the same month and
  - `fn with_month(self: &Self, month: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the month number (starting from 1) changed.
  - `fn with_month0(self: &Self, month0: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the month number (starting from 0) changed.
  - `fn with_day(self: &Self, day: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the day of month (starting from 1) changed.
  - `fn with_day0(self: &Self, day0: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the day of month (starting from 0) changed.
  - `fn with_ordinal(self: &Self, ordinal: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the day of year (starting from 1) changed.
  - `fn with_ordinal0(self: &Self, ordinal0: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the day of year (starting from 0) changed.
- **PartialEq**
  - `fn eq(self: &Self, other: &NaiveDateTime) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Add**
  - `fn add(self: Self, rhs: FixedOffset) -> NaiveDateTime`
- **Timelike**
  - `fn hour(self: &Self) -> u32` - Returns the hour number from 0 to 23.
  - `fn minute(self: &Self) -> u32` - Returns the minute number from 0 to 59.
  - `fn second(self: &Self) -> u32` - Returns the second number from 0 to 59.
  - `fn nanosecond(self: &Self) -> u32` - Returns the number of nanoseconds since the whole non-leap second.
  - `fn with_hour(self: &Self, hour: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the hour number changed.
  - `fn with_minute(self: &Self, min: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the minute number changed.
  - `fn with_second(self: &Self, sec: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with the second number changed.
  - `fn with_nanosecond(self: &Self, nano: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` with nanoseconds since the whole non-leap second changed.
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: TimeDelta)`
- **Sub**
  - `fn sub(self: Self, rhs: FixedOffset) -> NaiveDateTime`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **FromStr**
  - `fn from_str(s: &str) -> ParseResult<NaiveDateTime>`
- **Add**
  - `fn add(self: Self, rhs: Months) -> <Self as >::Output`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **Add**
  - `fn add(self: Self, rhs: Duration) -> NaiveDateTime`
- **Sub**
  - `fn sub(self: Self, rhs: Months) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, rhs: Duration) -> NaiveDateTime`
- **From**
  - `fn from(date: NaiveDate) -> Self` - Converts a `NaiveDate` to a `NaiveDateTime` of the same date but at midnight.
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: TimeDelta)`
- **Sub**
  - `fn sub(self: Self, rhs: NaiveDateTime) -> TimeDelta`
- **Add**
  - `fn add(self: Self, rhs: TimeDelta) -> NaiveDateTime`
- **Add**
  - `fn add(self: Self, days: Days) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, days: Days) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, rhs: TimeDelta) -> NaiveDateTime`



