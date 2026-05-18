**chrono > datetime**

# Module: datetime

## Contents

**Structs**

- [`DateTime`](#datetime) - ISO 8601 combined date and time with time zone.

**Constants**

- [`MAX_DATETIME`](#max_datetime) - The maximum possible `DateTime<Utc>`.
- [`MIN_DATETIME`](#min_datetime) - The minimum possible `DateTime<Utc>`.

---

## chrono::datetime::DateTime

*Struct*

ISO 8601 combined date and time with time zone.

There are some constructors implemented here (the `from_*` methods), but
the general-purpose constructors are all via the methods on the
[`TimeZone`](./offset/trait.TimeZone.html) implementations.

**Generic Parameters:**
- Tz

**Methods:**

- `fn from_naive_utc_and_offset(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` - Makes a new `DateTime` from its components: a `NaiveDateTime` in UTC and an `Offset`.
- `fn from_utc(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` - Makes a new `DateTime` from its components: a `NaiveDateTime` in UTC and an `Offset`.
- `fn from_local(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` - Makes a new `DateTime` from a `NaiveDateTime` in *local* time and an `Offset`.
- `fn date(self: &Self) -> Date<Tz>` - Retrieves the date component with an associated timezone.
- `fn date_naive(self: &Self) -> NaiveDate` - Retrieves the date component.
- `fn time(self: &Self) -> NaiveTime` - Retrieves the time component.
- `fn timestamp(self: &Self) -> i64` - Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC
- `fn timestamp_millis(self: &Self) -> i64` - Returns the number of non-leap-milliseconds since January 1, 1970 UTC.
- `fn timestamp_micros(self: &Self) -> i64` - Returns the number of non-leap-microseconds since January 1, 1970 UTC.
- `fn timestamp_nanos(self: &Self) -> i64` - Returns the number of non-leap-nanoseconds since January 1, 1970 UTC.
- `fn timestamp_nanos_opt(self: &Self) -> Option<i64>` - Returns the number of non-leap-nanoseconds since January 1, 1970 UTC.
- `fn timestamp_subsec_millis(self: &Self) -> u32` - Returns the number of milliseconds since the last second boundary.
- `fn timestamp_subsec_micros(self: &Self) -> u32` - Returns the number of microseconds since the last second boundary.
- `fn timestamp_subsec_nanos(self: &Self) -> u32` - Returns the number of nanoseconds since the last second boundary
- `fn offset(self: &Self) -> &<Tz as >::Offset` - Retrieves an associated offset from UTC.
- `fn timezone(self: &Self) -> Tz` - Retrieves an associated time zone.
- `fn with_timezone<Tz2>(self: &Self, tz: &Tz2) -> DateTime<Tz2>` - Changes the associated time zone.
- `fn fixed_offset(self: &Self) -> DateTime<FixedOffset>` - Fix the offset from UTC to its current value, dropping the associated timezone information.
- `fn to_utc(self: &Self) -> DateTime<Utc>` - Turn this `DateTime` into a `DateTime<Utc>`, dropping the offset and associated timezone
- `fn checked_add_signed(self: Self, rhs: TimeDelta) -> Option<DateTime<Tz>>` - Adds given `TimeDelta` to the current date and time.
- `fn checked_add_months(self: Self, months: Months) -> Option<DateTime<Tz>>` - Adds given `Months` to the current date and time.
- `fn checked_sub_signed(self: Self, rhs: TimeDelta) -> Option<DateTime<Tz>>` - Subtracts given `TimeDelta` from the current date and time.
- `fn checked_sub_months(self: Self, months: Months) -> Option<DateTime<Tz>>` - Subtracts given `Months` from the current date and time.
- `fn checked_add_days(self: Self, days: Days) -> Option<Self>` - Add a duration in [`Days`] to the date part of the `DateTime`.
- `fn checked_sub_days(self: Self, days: Days) -> Option<Self>` - Subtract a duration in [`Days`] from the date part of the `DateTime`.
- `fn signed_duration_since<Tz2, impl Borrow<DateTime<Tz2>>>(self: Self, rhs: impl Trait) -> TimeDelta` - Subtracts another `DateTime` from the current date and time.
- `fn naive_utc(self: &Self) -> NaiveDateTime` - Returns a view to the naive UTC datetime.
- `fn naive_local(self: &Self) -> NaiveDateTime` - Returns a view to the naive local datetime.
- `fn years_since(self: &Self, base: Self) -> Option<u32>` - Retrieve the elapsed years from now to the given [`DateTime`].
- `fn with_time(self: &Self, time: NaiveTime) -> LocalResult<Self>` - Set the time to a new fixed time on the existing date.
- `fn parse_from_rfc2822(s: &str) -> ParseResult<DateTime<FixedOffset>>` - Parses an RFC 2822 date-and-time string into a `DateTime<FixedOffset>` value.
- `fn parse_from_rfc3339(s: &str) -> ParseResult<DateTime<FixedOffset>>` - Parses an RFC 3339 date-and-time string into a `DateTime<FixedOffset>` value.
- `fn parse_from_str(s: &str, fmt: &str) -> ParseResult<DateTime<FixedOffset>>` - Parses a string from a user-specified format into a `DateTime<FixedOffset>` value.
- `fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(DateTime<FixedOffset>, &'a str)>` - Parses a string from a user-specified format into a `DateTime<FixedOffset>` value, and a
- `fn from_timestamp_secs(secs: i64) -> Option<Self>` - Makes a new `DateTime<Utc>` from the number of non-leap seconds
- `fn from_timestamp(secs: i64, nsecs: u32) -> Option<Self>` - Makes a new `DateTime<Utc>` from the number of non-leap seconds
- `fn from_timestamp_millis(millis: i64) -> Option<Self>` - Makes a new `DateTime<Utc>` from the number of non-leap milliseconds
- `fn from_timestamp_micros(micros: i64) -> Option<Self>` - Creates a new `DateTime<Utc>` from the number of non-leap microseconds
- `fn from_timestamp_nanos(nanos: i64) -> Self` - Creates a new [`DateTime<Utc>`] from the number of non-leap nanoseconds

**Traits:** Copy, Eq

**Trait Implementations:**

- **Sub**
  - `fn sub(self: Self, rhs: TimeDelta) -> DateTime<Tz>`
- **Add**
  - `fn add(self: Self, rhs: Duration) -> DateTime<Tz>`
- **Datelike**
  - `fn year(self: &Self) -> i32`
  - `fn month(self: &Self) -> u32`
  - `fn month0(self: &Self) -> u32`
  - `fn day(self: &Self) -> u32`
  - `fn day0(self: &Self) -> u32`
  - `fn ordinal(self: &Self) -> u32`
  - `fn ordinal0(self: &Self) -> u32`
  - `fn weekday(self: &Self) -> Weekday`
  - `fn iso_week(self: &Self) -> IsoWeek`
  - `fn with_year(self: &Self, year: i32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the year number changed, while keeping the same month and day.
  - `fn with_month(self: &Self, month: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the month number (starting from 1) changed.
  - `fn with_month0(self: &Self, month0: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the month number (starting from 0) changed.
  - `fn with_day(self: &Self, day: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the day of month (starting from 1) changed.
  - `fn with_day0(self: &Self, day0: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the day of month (starting from 0) changed.
  - `fn with_ordinal(self: &Self, ordinal: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the day of year (starting from 1) changed.
  - `fn with_ordinal0(self: &Self, ordinal0: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the day of year (starting from 0) changed.
- **Sub**
  - `fn sub(self: Self, rhs: Months) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, rhs: Duration) -> DateTime<Tz>`
- **From**
  - `fn from(src: DateTime<FixedOffset>) -> Self` - Convert this `DateTime<FixedOffset>` instance into a `DateTime<Utc>` instance.
- **Sub**
  - `fn sub(self: Self, rhs: DateTime<Tz>) -> TimeDelta`
- **Ord**
  - `fn cmp(self: &Self, other: &DateTime<Tz>) -> Ordering`
- **Clone**
  - `fn clone(self: &Self) -> DateTime<Tz>`
- **Timelike**
  - `fn hour(self: &Self) -> u32`
  - `fn minute(self: &Self) -> u32`
  - `fn second(self: &Self) -> u32`
  - `fn nanosecond(self: &Self) -> u32`
  - `fn with_hour(self: &Self, hour: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the hour number changed.
  - `fn with_minute(self: &Self, min: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the minute number changed.
  - `fn with_second(self: &Self, sec: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with the second number changed.
  - `fn with_nanosecond(self: &Self, nano: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` with nanoseconds since the whole non-leap second changed.
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Sub**
  - `fn sub(self: Self, rhs: &DateTime<Tz>) -> TimeDelta`
- **PartialEq**
  - `fn eq(self: &Self, other: &DateTime<Tz2>) -> bool`
- **Add**
  - `fn add(self: Self, days: Days) -> <Self as >::Output`
- **From**
  - `fn from(src: DateTime<Utc>) -> Self` - Convert this `DateTime<Utc>` instance into a `DateTime<FixedOffset>` instance.
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DateTime<Tz2>) -> Option<Ordering>` - Compare two DateTimes based on their true time, ignoring time zones
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> ParseResult<DateTime<Utc>>`
- **Sub**
  - `fn sub(self: Self, days: Days) -> <Self as >::Output`
- **DurationRound**
  - `fn duration_round(self: Self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`
  - `fn duration_trunc(self: Self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`
  - `fn duration_round_up(self: Self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: TimeDelta)`
- **Default**
  - `fn default() -> Self`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: TimeDelta)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: Duration)`
- **Add**
  - `fn add(self: Self, rhs: TimeDelta) -> DateTime<Tz>`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: Duration)`
- **Add**
  - `fn add(self: Self, rhs: FixedOffset) -> DateTime<Tz>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Default**
  - `fn default() -> Self`
- **FromStr**
  - `fn from_str(s: &str) -> ParseResult<DateTime<FixedOffset>>`
- **Sub**
  - `fn sub(self: Self, rhs: FixedOffset) -> DateTime<Tz>`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Add**
  - `fn add(self: Self, rhs: Months) -> <Self as >::Output`



## chrono::datetime::MAX_DATETIME

*Constant*: `DateTime<crate::offset::Utc>`

The maximum possible `DateTime<Utc>`.



## chrono::datetime::MIN_DATETIME

*Constant*: `DateTime<crate::offset::Utc>`

The minimum possible `DateTime<Utc>`.



