*[chrono](../index.md) / [datetime](index.md)*

---

# Module `datetime`

ISO 8601 date and time with time zone.

## Contents

- [Modules](#modules)
  - [`serde`](#serde)
- [Structs](#structs)
  - [`DateTime`](#datetime)
- [Functions](#functions)
  - [`map_local`](#map-local)
- [Constants](#constants)
  - [`MIN_DATETIME`](#min-datetime)
  - [`MAX_DATETIME`](#max-datetime)
  - [`UNIX_EPOCH_DAY`](#unix-epoch-day)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serde`](#serde) | mod | documented at re-export site |
| [`DateTime`](#datetime) | struct | ISO 8601 combined date and time with time zone. |
| [`map_local`](#map-local) | fn | Maps the local datetime to other datetime with given conversion function. |
| [`MIN_DATETIME`](#min-datetime) | const | The minimum possible `DateTime<Utc>`. |
| [`MAX_DATETIME`](#max-datetime) | const | The maximum possible `DateTime<Utc>`. |
| [`UNIX_EPOCH_DAY`](#unix-epoch-day) | const | Number of days between January 1, 1970 and December 31, 1 BCE which we define to be day 0. 4 full leap year cycles until December 31, 1600     4 * 146097 = 584388 1 day until January 1, 1601                                           1 369 years until January 1, 1970                      369 * 365 = 134685 of which floor(369 / 4) are leap years          floor(369 / 4) =     92 except for 1700, 1800 and 1900                                       -3 + -------- 719163 |

## Modules

- [`serde`](serde/index.md) — documented at re-export site

## Structs

### `DateTime<Tz: TimeZone>`

```rust
struct DateTime<Tz: TimeZone> {
    datetime: crate::naive::NaiveDateTime,
    offset: <Tz as >::Offset,
}
```

ISO 8601 combined date and time with time zone.

There are some constructors implemented here (the `from_*` methods), but
the general-purpose constructors are all via the methods on the
[`TimeZone`](./offset/trait.TimeZone.html) implementations.

#### Implementations

- <span id="datetime-from-naive-utc-and-offset"></span>`const fn from_naive_utc_and_offset(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime), [`TimeZone`](../offset/index.md#timezone), [`DateTime`](#datetime)

  Makes a new `DateTime` from its components: a `NaiveDateTime` in UTC and an `Offset`.

  

  This is a low-level method, intended for use cases such as deserializing a `DateTime` or

  passing it through FFI.

  

  For regular use you will probably want to use a method such as

  `TimeZone::from_local_datetime` or `NaiveDateTime::and_local_timezone` instead.

  

  # Example

  

  ```rust

  #[cfg(feature = "clock")] {

  use chrono::{DateTime, Local};

  

  let dt = Local::now();

  // Get components

  let naive_utc = dt.naive_utc();

  let offset = dt.offset().clone();

  // Serialize, pass through FFI... and recreate the `DateTime`:

  let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);

  assert_eq!(dt, dt_new);

  }

  ```

- <span id="datetime-from-utc"></span>`fn from_utc(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime), [`TimeZone`](../offset/index.md#timezone), [`DateTime`](#datetime)

  Makes a new `DateTime` from its components: a `NaiveDateTime` in UTC and an `Offset`.

- <span id="datetime-from-local"></span>`fn from_local(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime), [`TimeZone`](../offset/index.md#timezone), [`DateTime`](#datetime)

  Makes a new `DateTime` from a `NaiveDateTime` in *local* time and an `Offset`.

  

  # Panics

  

  Panics if the local datetime can't be converted to UTC because it would be out of range.

  

  This can happen if `datetime` is near the end of the representable range of `NaiveDateTime`,

  and the offset from UTC pushes it beyond that.

- <span id="datetime-date"></span>`fn date(&self) -> Date<Tz>` — [`Date`](../date/index.md#date)

  Retrieves the date component with an associated timezone.

  

  Unless you are immediately planning on turning this into a `DateTime`

  with the same timezone you should use the [`date_naive`](DateTime::date_naive) method.

  

  [`NaiveDate`](../naive/date/index.md) is a more well-defined type, and has more traits implemented on it,

  so should be preferred to [`Date`](../date/index.md) any time you truly want to operate on dates.

  

  # Panics

  

  [`DateTime`](#datetime) internally stores the date and time in UTC with a [`NaiveDateTime`](../naive/datetime/index.md). This

  method will panic if the offset from UTC would push the local date outside of the

  representable range of a [`Date`](../date/index.md).

- <span id="datetime-date-naive"></span>`fn date_naive(&self) -> NaiveDate` — [`NaiveDate`](../naive/date/index.md#naivedate)

  Retrieves the date component.

  

  # Panics

  

  [`DateTime`](#datetime) internally stores the date and time in UTC with a [`NaiveDateTime`](../naive/datetime/index.md). This

  method will panic if the offset from UTC would push the local date outside of the

  representable range of a [`NaiveDate`](../naive/date/index.md).

  

  # Example

  

  ```rust

  use chrono::prelude::*;

  

  let date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

  let other: DateTime<FixedOffset> =

      FixedOffset::east_opt(23).unwrap().with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

  assert_eq!(date.date_naive(), other.date_naive());

  ```

- <span id="datetime-time"></span>`fn time(&self) -> NaiveTime` — [`NaiveTime`](../naive/time/index.md#naivetime)

  Retrieves the time component.

- <span id="datetime-timestamp"></span>`const fn timestamp(&self) -> i64`

  Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC

  (aka "UNIX timestamp").

  

  The reverse operation of creating a [`DateTime`](#datetime) from a timestamp can be performed

  using [`from_timestamp`](DateTime::from_timestamp) or `TimeZone::timestamp_opt`.

  

  ```rust

  use chrono::{DateTime, TimeZone, Utc};

  

  let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2015, 5, 15, 0, 0, 0).unwrap();

  assert_eq!(dt.timestamp(), 1431648000);

  

  assert_eq!(DateTime::from_timestamp(dt.timestamp(), dt.timestamp_subsec_nanos()).unwrap(), dt);

  ```

- <span id="datetime-timestamp-millis"></span>`const fn timestamp_millis(&self) -> i64`

  Returns the number of non-leap-milliseconds since January 1, 1970 UTC.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  

  let dt = NaiveDate::from_ymd_opt(1970, 1, 1)

      .unwrap()

      .and_hms_milli_opt(0, 0, 1, 444)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_millis(), 1_444);

  

  let dt = NaiveDate::from_ymd_opt(2001, 9, 9)

      .unwrap()

      .and_hms_milli_opt(1, 46, 40, 555)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_millis(), 1_000_000_000_555);

  ```

- <span id="datetime-timestamp-micros"></span>`const fn timestamp_micros(&self) -> i64`

  Returns the number of non-leap-microseconds since January 1, 1970 UTC.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  

  let dt = NaiveDate::from_ymd_opt(1970, 1, 1)

      .unwrap()

      .and_hms_micro_opt(0, 0, 1, 444)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_micros(), 1_000_444);

  

  let dt = NaiveDate::from_ymd_opt(2001, 9, 9)

      .unwrap()

      .and_hms_micro_opt(1, 46, 40, 555)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_micros(), 1_000_000_000_000_555);

  ```

- <span id="datetime-timestamp-nanos"></span>`const fn timestamp_nanos(&self) -> i64`

  Returns the number of non-leap-nanoseconds since January 1, 1970 UTC.

  

  # Panics

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function panics on

  an out of range `DateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

- <span id="datetime-timestamp-nanos-opt"></span>`const fn timestamp_nanos_opt(&self) -> Option<i64>`

  Returns the number of non-leap-nanoseconds since January 1, 1970 UTC.

  

  # Errors

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function returns

  `None` on an out of range `DateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  

  let dt = NaiveDate::from_ymd_opt(1970, 1, 1)

      .unwrap()

      .and_hms_nano_opt(0, 0, 1, 444)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(1_000_000_444));

  

  let dt = NaiveDate::from_ymd_opt(2001, 9, 9)

      .unwrap()

      .and_hms_nano_opt(1, 46, 40, 555)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(1_000_000_000_000_000_555));

  

  let dt = NaiveDate::from_ymd_opt(1677, 9, 21)

      .unwrap()

      .and_hms_nano_opt(0, 12, 43, 145_224_192)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(-9_223_372_036_854_775_808));

  

  let dt = NaiveDate::from_ymd_opt(2262, 4, 11)

      .unwrap()

      .and_hms_nano_opt(23, 47, 16, 854_775_807)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(9_223_372_036_854_775_807));

  

  let dt = NaiveDate::from_ymd_opt(1677, 9, 21)

      .unwrap()

      .and_hms_nano_opt(0, 12, 43, 145_224_191)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), None);

  

  let dt = NaiveDate::from_ymd_opt(2262, 4, 11)

      .unwrap()

      .and_hms_nano_opt(23, 47, 16, 854_775_808)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), None);

  ```

- <span id="datetime-timestamp-subsec-millis"></span>`const fn timestamp_subsec_millis(&self) -> u32`

  Returns the number of milliseconds since the last second boundary.

  

  In event of a leap second this may exceed 999.

- <span id="datetime-timestamp-subsec-micros"></span>`const fn timestamp_subsec_micros(&self) -> u32`

  Returns the number of microseconds since the last second boundary.

  

  In event of a leap second this may exceed 999,999.

- <span id="datetime-timestamp-subsec-nanos"></span>`const fn timestamp_subsec_nanos(&self) -> u32`

  Returns the number of nanoseconds since the last second boundary

  

  In event of a leap second this may exceed 999,999,999.

- <span id="datetime-offset"></span>`const fn offset(&self) -> &<Tz as >::Offset` — [`TimeZone`](../offset/index.md#timezone)

  Retrieves an associated offset from UTC.

- <span id="datetime-timezone"></span>`fn timezone(&self) -> Tz`

  Retrieves an associated time zone.

- <span id="datetime-with-timezone"></span>`fn with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> DateTime<Tz2>` — [`DateTime`](#datetime)

  Changes the associated time zone.

  The returned `DateTime` references the same instant of time from the perspective of the

  provided time zone.

- <span id="datetime-fixed-offset"></span>`fn fixed_offset(&self) -> DateTime<FixedOffset>` — [`DateTime`](#datetime), [`FixedOffset`](../offset/fixed/index.md#fixedoffset)

  Fix the offset from UTC to its current value, dropping the associated timezone information.

  This is useful for converting a generic `DateTime<Tz: Timezone>` to `DateTime<FixedOffset>`.

- <span id="datetime-to-utc"></span>`const fn to_utc(&self) -> DateTime<Utc>` — [`DateTime`](#datetime), [`Utc`](../offset/utc/index.md#utc)

  Turn this `DateTime` into a `DateTime<Utc>`, dropping the offset and associated timezone

  information.

- <span id="datetime-checked-add-signed"></span>`fn checked_add_signed(self, rhs: TimeDelta) -> Option<DateTime<Tz>>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DateTime`](#datetime)

  Adds given `TimeDelta` to the current date and time.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

- <span id="datetime-checked-add-months"></span>`fn checked_add_months(self, months: Months) -> Option<DateTime<Tz>>` — [`Months`](../month/index.md#months), [`DateTime`](#datetime)

  Adds given `Months` to the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  See `NaiveDate::checked_add_months` for more details on behavior.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `months` is zero).

- <span id="datetime-checked-sub-signed"></span>`fn checked_sub_signed(self, rhs: TimeDelta) -> Option<DateTime<Tz>>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DateTime`](#datetime)

  Subtracts given `TimeDelta` from the current date and time.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

- <span id="datetime-checked-sub-months"></span>`fn checked_sub_months(self, months: Months) -> Option<DateTime<Tz>>` — [`Months`](../month/index.md#months), [`DateTime`](#datetime)

  Subtracts given `Months` from the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  See `NaiveDate::checked_sub_months` for more details on behavior.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `months` is zero).

- <span id="datetime-checked-add-days"></span>`fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](../naive/index.md#days)

  Add a duration in [`Days`](../naive/index.md) to the date part of the `DateTime`.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `days` is zero).

- <span id="datetime-checked-sub-days"></span>`fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](../naive/index.md#days)

  Subtract a duration in [`Days`](../naive/index.md) from the date part of the `DateTime`.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `days` is zero).

- <span id="datetime-signed-duration-since"></span>`fn signed_duration_since<Tz2: TimeZone>(self, rhs: impl Borrow<DateTime<Tz2>>) -> TimeDelta` — [`DateTime`](#datetime), [`TimeDelta`](../time_delta/index.md#timedelta)

  Subtracts another `DateTime` from the current date and time.

  This does not overflow or underflow at all.

- <span id="datetime-naive-utc"></span>`const fn naive_utc(&self) -> NaiveDateTime` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime)

  Returns a view to the naive UTC datetime.

- <span id="datetime-naive-local"></span>`fn naive_local(&self) -> NaiveDateTime` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime)

  Returns a view to the naive local datetime.

  

  # Panics

  

  [`DateTime`](#datetime) internally stores the date and time in UTC with a [`NaiveDateTime`](../naive/datetime/index.md). This

  method will panic if the offset from UTC would push the local datetime outside of the

  representable range of a [`NaiveDateTime`](../naive/datetime/index.md).

- <span id="datetime-overflowing-naive-local"></span>`fn overflowing_naive_local(&self) -> NaiveDateTime` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime)

  Returns the naive local datetime.

  

  This makes use of the buffer space outside of the representable range of values of

  `NaiveDateTime`. The result can be used as intermediate value, but should never be exposed

  outside chrono.

- <span id="datetime-years-since"></span>`fn years_since(&self, base: Self) -> Option<u32>`

  Retrieve the elapsed years from now to the given [`DateTime`](#datetime).

  

  # Errors

  

  Returns `None` if `base > self`.

- <span id="datetime-with-time"></span>`fn with_time(&self, time: NaiveTime) -> LocalResult<Self>` — [`NaiveTime`](../naive/time/index.md#naivetime), [`LocalResult`](../offset/index.md#localresult)

  Set the time to a new fixed time on the existing date.

  

  # Errors

  

  Returns `LocalResult::None` if the datetime is at the edge of the representable range for a

  `DateTime`, and `with_time` would push the value in UTC out of range.

  

  # Example

  

  ```rust

  #[cfg(feature = "clock")] {

  use chrono::{Local, NaiveTime};

  

  let noon = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

  let today_noon = Local::now().with_time(noon);

  let today_midnight = Local::now().with_time(NaiveTime::MIN);

  

  assert_eq!(today_noon.single().unwrap().time(), noon);

  assert_eq!(today_midnight.single().unwrap().time(), NaiveTime::MIN);

  }

  ```

- <span id="datetime-const-min-utc"></span>`const MIN_UTC: DateTime<Utc>`

- <span id="datetime-const-max-utc"></span>`const MAX_UTC: DateTime<Utc>`

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, rhs: TimeDelta) -> DateTime<Tz>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DateTime`](#datetime)

##### `impl<Tz: TimeZone> AddAssign for DateTime<Tz>`

- <span id="datetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl<Tz: clone::Clone + TimeZone> Clone for DateTime<Tz>`

- <span id="datetime-clone"></span>`fn clone(&self) -> DateTime<Tz>` — [`DateTime`](#datetime)

##### `impl<Tz: TimeZone> Copy for DateTime<Tz>`

##### `impl<Tz: TimeZone> Datelike for DateTime<Tz>`

- <span id="datetime-datelike-year"></span>`fn year(&self) -> i32`

- <span id="datetime-datelike-month"></span>`fn month(&self) -> u32`

- <span id="datetime-datelike-month0"></span>`fn month0(&self) -> u32`

- <span id="datetime-datelike-day"></span>`fn day(&self) -> u32`

- <span id="datetime-datelike-day0"></span>`fn day0(&self) -> u32`

- <span id="datetime-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

- <span id="datetime-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

- <span id="datetime-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](../weekday/index.md#weekday)

- <span id="datetime-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](../naive/isoweek/index.md#isoweek)

- <span id="datetime-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the year number changed, while keeping the same month and day.

  

  See also the `NaiveDate::with_year` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (February 29 in a non-leap year).

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless the year remains the same).

- <span id="datetime-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the month number (starting from 1) changed.

  

  Don't combine multiple `Datelike::with_*` methods. The intermediate value may not exist.

  

  See also the `NaiveDate::with_month` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month(4)` when day of the month is 31).

  - The value for `month` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the month number (starting from 0) changed.

  

  See also the `NaiveDate::with_month0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month0(3)` when day of the month is 31).

  - The value for `month0` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the day of month (starting from 1) changed.

  

  See also the `NaiveDate::with_day` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(31)` in April).

  - The value for `day` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the day of month (starting from 0) changed.

  

  See also the `NaiveDate::with_day0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(30)` in April).

  - The value for `day0` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the day of year (starting from 1) changed.

  

  See also the `NaiveDate::with_ordinal` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal(366)` in a non-leap year).

  - The value for `ordinal` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the day of year (starting from 0) changed.

  

  See also the `NaiveDate::with_ordinal0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal0(365)` in a non-leap year).

  - The value for `ordinal0` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

##### `impl<Tz: TimeZone> Debug for DateTime<Tz>`

- <span id="datetime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DateTime<crate::offset::Utc>`

- <span id="datetime-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::DateTime<crate::offset::FixedOffset>`

- <span id="superdatetime-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for DateTime<Tz>`

##### `impl<Tz: TimeZone> Display for DateTime<Tz>`

- <span id="datetime-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Tz: TimeZone> DurationRound for crate::DateTime<Tz>`

- <span id="cratedatetime-durationround-type-err"></span>`type Err = RoundingError`

- <span id="cratedatetime-durationround-duration-round"></span>`fn duration_round(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DurationRound`](../round/index.md#durationround)

- <span id="cratedatetime-durationround-duration-trunc"></span>`fn duration_trunc(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DurationRound`](../round/index.md#durationround)

- <span id="cratedatetime-durationround-duration-round-up"></span>`fn duration_round_up(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DurationRound`](../round/index.md#durationround)

##### `impl<Tz: TimeZone> Eq for DateTime<Tz>`

##### `impl FromStr for DateTime<crate::offset::Utc>`

- <span id="datetime-fromstr-type-err"></span>`type Err = ParseError`

- <span id="datetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<DateTime<Utc>>` — [`ParseResult`](../format/index.md#parseresult), [`DateTime`](#datetime), [`Utc`](../offset/utc/index.md#utc)

##### `impl<Tz: TimeZone> Hash for DateTime<Tz>`

- <span id="datetime-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<Tz: TimeZone> Ord for DateTime<Tz>`

- <span id="datetime-ord-cmp"></span>`fn cmp(&self, other: &DateTime<Tz>) -> Ordering` — [`DateTime`](#datetime)

##### `impl<Tz: TimeZone, Tz2: TimeZone> PartialEq for DateTime<Tz>`

- <span id="datetime-partialeq-eq"></span>`fn eq(&self, other: &DateTime<Tz2>) -> bool` — [`DateTime`](#datetime)

##### `impl<Tz: TimeZone, Tz2: TimeZone> PartialOrd for DateTime<Tz>`

- <span id="datetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DateTime<Tz2>) -> Option<Ordering>` — [`DateTime`](#datetime)

  Compare two DateTimes based on their true time, ignoring time zones

  

  # Example

  

  ```rust

  use chrono::prelude::*;

  

  let earlier = Utc

      .with_ymd_and_hms(2015, 5, 15, 2, 0, 0)

      .unwrap()

      .with_timezone(&FixedOffset::west_opt(1 * 3600).unwrap());

  let later = Utc

      .with_ymd_and_hms(2015, 5, 15, 3, 0, 0)

      .unwrap()

      .with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());

  

  assert_eq!(earlier.to_string(), "2015-05-15 01:00:00 -01:00");

  assert_eq!(later.to_string(), "2015-05-14 22:00:00 -05:00");

  

  assert!(later > earlier);

  ```

##### `impl<Tz: TimeZone> Serialize for super::DateTime<Tz>`

- <span id="superdatetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> DateTime<Tz>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DateTime`](#datetime)

##### `impl<Tz: TimeZone> SubAssign for DateTime<Tz>`

- <span id="datetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl SubsecRound for DateTime<Tz>`

- <span id="datetime-subsecround-round-subsecs"></span>`fn round_subsecs(self, digits: u16) -> T`

- <span id="datetime-subsecround-trunc-subsecs"></span>`fn trunc_subsecs(self, digits: u16) -> T`

##### `impl<Tz: TimeZone> Timelike for DateTime<Tz>`

- <span id="datetime-timelike-hour"></span>`fn hour(&self) -> u32`

- <span id="datetime-timelike-minute"></span>`fn minute(&self) -> u32`

- <span id="datetime-timelike-second"></span>`fn second(&self) -> u32`

- <span id="datetime-timelike-nanosecond"></span>`fn nanosecond(&self) -> u32`

- <span id="datetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the hour number changed.

  

  See also the `NaiveTime::with_hour` method.

  

  # Errors

  

  Returns `None` if:

  - The value for `hour` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the minute number changed.

  

  See also the `NaiveTime::with_minute` method.

  

  # Errors

  

  - The value for `minute` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with the second number changed.

  

  As with the [`second`](#method.second) method,

  the input range is restricted to 0 through 59.

  

  See also the `NaiveTime::with_second` method.

  

  # Errors

  

  Returns `None` if:

  - The value for `second` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<DateTime<Tz>>` — [`DateTime`](#datetime)

  Makes a new `DateTime` with nanoseconds since the whole non-leap second changed.

  

  Returns `None` when the resulting `NaiveDateTime` would be invalid.

  As with the `NaiveDateTime::nanosecond` method,

  the input range can exceed 1,000,000,000 for leap seconds.

  

  See also the `NaiveTime::with_nanosecond` method.

  

  # Errors

  

  Returns `None` if `nanosecond >= 2,000,000,000`.

##### `impl ToString for DateTime<Tz>`

- <span id="datetime-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `map_local`

```rust
fn map_local<Tz: TimeZone, F>(dt: &DateTime<Tz>, f: F) -> Option<DateTime<Tz>>
where
    F: FnMut(crate::naive::NaiveDateTime) -> Option<crate::naive::NaiveDateTime>
```

Maps the local datetime to other datetime with given conversion function.

## Constants

### `MIN_DATETIME`
```rust
const MIN_DATETIME: DateTime<crate::offset::Utc>;
```

The minimum possible `DateTime<Utc>`.

### `MAX_DATETIME`
```rust
const MAX_DATETIME: DateTime<crate::offset::Utc>;
```

The maximum possible `DateTime<Utc>`.

### `UNIX_EPOCH_DAY`
```rust
const UNIX_EPOCH_DAY: i64 = 719_163i64;
```

Number of days between January 1, 1970 and December 31, 1 BCE which we define to be day 0.
4 full leap year cycles until December 31, 1600     4 * 146097 = 584388
1 day until January 1, 1601                                           1
369 years until January 1, 1970                      369 * 365 = 134685
of which floor(369 / 4) are leap years          floor(369 / 4) =     92
except for 1700, 1800 and 1900                                       -3 +
                                                                 --------
                                                                 719163

