*[chrono](../../index.md) / [naive](../index.md) / [datetime](index.md)*

---

# Module `datetime`

ISO 8601 date and time without timezone.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serde`](#serde) | mod | Tools to help serializing/deserializing `NaiveDateTime`s |
| [`NaiveDateTime`](#naivedatetime) | struct | ISO 8601 combined date and time without timezone. |
| [`MIN_DATETIME`](#min-datetime) | const | The minimum possible `NaiveDateTime`. |
| [`MAX_DATETIME`](#max-datetime) | const | The maximum possible `NaiveDateTime`. |

## Modules

- [`serde`](serde/index.md) — Tools to help serializing/deserializing `NaiveDateTime`s

## Structs

### `NaiveDateTime`

```rust
struct NaiveDateTime {
    date: crate::naive::NaiveDate,
    time: crate::naive::NaiveTime,
}
```

ISO 8601 combined date and time without timezone.

# Example

`NaiveDateTime` is commonly created from [`NaiveDate`](../date/index.md).

```rust
use chrono::{NaiveDate, NaiveDateTime};

let dt: NaiveDateTime =
    NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
let _ = dt;
```

You can use typical [date-like](Datelike) and [time-like](Timelike) methods,
provided that relevant traits are in the scope.

```rust
use chrono::{NaiveDate, NaiveDateTime};
let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
use chrono::{Datelike, Timelike, Weekday};

assert_eq!(dt.weekday(), Weekday::Fri);
assert_eq!(dt.num_seconds_from_midnight(), 33011);
```

#### Implementations

- <span id="naivedatetime-new"></span>`const fn new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime` — [`NaiveDate`](../date/index.md#naivedate), [`NaiveTime`](../time/index.md#naivetime), [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` from date and time components.

  Equivalent to [`date.and_time(time)`](./struct.NaiveDate.html#method.and_time)

  and many other helper constructors on `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

  

  let dt = NaiveDateTime::new(d, t);

  assert_eq!(dt.date(), d);

  assert_eq!(dt.time(), t);

  ```

- <span id="naivedatetime-from-timestamp"></span>`const fn from_timestamp(secs: i64, nsecs: u32) -> NaiveDateTime` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` corresponding to a UTC date and time,

  from the number of non-leap seconds

  since the midnight UTC on January 1, 1970 (aka "UNIX timestamp")

  and the number of nanoseconds since the last whole non-leap second.

  

  For a non-naive version of this function see `TimeZone::timestamp`.

  

  The nanosecond part can exceed 1,000,000,000 in order to represent a

  [leap second](NaiveTime#leap-second-handling), but only when `secs % 60 == 59`.

  (The true "UNIX timestamp" cannot represent a leap second unambiguously.)

  

  # Panics

  

  Panics if the number of seconds would be out of range for a `NaiveDateTime` (more than

  ca. 262,000 years away from common era), and panics on an invalid nanosecond (2 seconds or

  more).

- <span id="naivedatetime-from-timestamp-millis"></span>`const fn from_timestamp_millis(millis: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Creates a new [NaiveDateTime] from milliseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of milliseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-micros"></span>`const fn from_timestamp_micros(micros: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Creates a new [NaiveDateTime] from microseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of microseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-nanos"></span>`const fn from_timestamp_nanos(nanos: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Creates a new [NaiveDateTime] from nanoseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of nanoseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-opt"></span>`const fn from_timestamp_opt(secs: i64, nsecs: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` corresponding to a UTC date and time,

  from the number of non-leap seconds

  since the midnight UTC on January 1, 1970 (aka "UNIX timestamp")

  and the number of nanoseconds since the last whole non-leap second.

  

  The nanosecond part can exceed 1,000,000,000 in order to represent a

  [leap second](NaiveTime#leap-second-handling), but only when `secs % 60 == 59`.

  (The true "UNIX timestamp" cannot represent a leap second unambiguously.)

  

  # Errors

  

  Returns `None` if the number of seconds would be out of range for a `NaiveDateTime` (more

  than ca. 262,000 years away from common era), and panics on an invalid nanosecond

  (2 seconds or more).

- <span id="naivedatetime-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDateTime>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveDateTime`](#naivedatetime)

   Parses a string with the specified format string and returns a new `NaiveDateTime`.

   See the [`format::strftime` module](crate::format::strftime)

   on the supported escape sequences.

  

   # Example

  

   ```rust

   use chrono::{NaiveDate, NaiveDateTime};

  

   let parse_from_str = NaiveDateTime::parse_from_str;

  

   assert_eq!(

       parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S"),

       Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap())

   );

   assert_eq!(

       parse_from_str("5sep2015pm012345.6789", "%d%b%Y%p%I%M%S%.f"),

       Ok(NaiveDate::from_ymd_opt(2015, 9, 5)

           .unwrap()

           .and_hms_micro_opt(13, 23, 45, 678_900)

           .unwrap())

   );

   ```

  

   Offset is ignored for the purpose of parsing.

  

   ```rust

   use chrono::{NaiveDateTime, NaiveDate};

   let parse_from_str = NaiveDateTime::parse_from_str;

   assert_eq!(

       parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),

       Ok(NaiveDate::from_ymd_opt(2014, 5, 17).unwrap().and_hms_opt(12, 34, 56).unwrap())

   );

   ```

  

   [Leap seconds](./struct.NaiveTime.html#leap-second-handling) are correctly handled by

   treating any time of the form `hh:mm:60` as a leap second.

   (This equally applies to the formatting, so the round trip is possible.)

  

   ```rust

   use chrono::{NaiveDateTime, NaiveDate};

   let parse_from_str = NaiveDateTime::parse_from_str;

   assert_eq!(

       parse_from_str("2015-07-01 08:59:60.123", "%Y-%m-%d %H:%M:%S%.f"),

       Ok(NaiveDate::from_ymd_opt(2015, 7, 1)

           .unwrap()

           .and_hms_milli_opt(8, 59, 59, 1_123)

           .unwrap())

   );

   ```

  

   Missing seconds are assumed to be zero,

   but out-of-bound times or insufficient fields are errors otherwise.

  

   ```rust

   use chrono::{NaiveDateTime, NaiveDate};

   let parse_from_str = NaiveDateTime::parse_from_str;

   assert_eq!(

       parse_from_str("94/9/4 7:15", "%y/%m/%d %H:%M"),

       Ok(NaiveDate::from_ymd_opt(1994, 9, 4).unwrap().and_hms_opt(7, 15, 0).unwrap())

   );

  

   assert!(parse_from_str("04m33s", "%Mm%Ss").is_err());

   assert!(parse_from_str("94/9/4 12", "%y/%m/%d %H").is_err());

   assert!(parse_from_str("94/9/4 17:60", "%y/%m/%d %H:%M").is_err());

   assert!(parse_from_str("94/9/4 24:00:00", "%y/%m/%d %H:%M:%S").is_err());

   ```

  

   All parsed fields should be consistent to each other, otherwise it's an error.

  

   ```rust

   use chrono::NaiveDateTime;

   let parse_from_str = NaiveDateTime::parse_from_str;

   let fmt = "%Y-%m-%d %H:%M:%S = UNIX timestamp %s";

   assert!(parse_from_str("2001-09-09 01:46:39 = UNIX timestamp 999999999", fmt).is_ok());

   assert!(parse_from_str("1970-01-01 00:00:00 = UNIX timestamp 1", fmt).is_err());

   ```

  

   Years before 1 BCE or after 9999 CE, require an initial sign

  

  ```rust

   use chrono::NaiveDateTime;

   let parse_from_str = NaiveDateTime::parse_from_str;

   let fmt = "%Y-%m-%d %H:%M:%S";

   assert!(parse_from_str("10000-09-09 01:46:39", fmt).is_err());

   assert!(parse_from_str("+10000-09-09 01:46:39", fmt).is_ok());

   ```

- <span id="naivedatetime-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDateTime, &'a str)>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveDateTime`](#naivedatetime)

  Parses a string with the specified format string and returns a new `NaiveDateTime`, and a

  slice with the remaining portion of the string.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  Similar to [`parse_from_str`](#method.parse_from_str).

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime};

  let (datetime, remainder) = NaiveDateTime::parse_and_remainder(

      "2015-02-18 23:16:09 trailing text",

      "%Y-%m-%d %H:%M:%S",

  )

  .unwrap();

  assert_eq!(

      datetime,

      NaiveDate::from_ymd_opt(2015, 2, 18).unwrap().and_hms_opt(23, 16, 9).unwrap()

  );

  assert_eq!(remainder, " trailing text");

  ```

- <span id="naivedatetime-date"></span>`const fn date(&self) -> NaiveDate` — [`NaiveDate`](../date/index.md#naivedate)

  Retrieves a date component.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();

  assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2016, 7, 8).unwrap());

  ```

- <span id="naivedatetime-time"></span>`const fn time(&self) -> NaiveTime` — [`NaiveTime`](../time/index.md#naivetime)

  Retrieves a time component.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveTime};

  

  let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();

  assert_eq!(dt.time(), NaiveTime::from_hms_opt(9, 10, 11).unwrap());

  ```

- <span id="naivedatetime-timestamp"></span>`const fn timestamp(&self) -> i64`

  Returns the number of non-leap seconds since the midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

- <span id="naivedatetime-timestamp-millis"></span>`const fn timestamp_millis(&self) -> i64`

  Returns the number of non-leap *milliseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

- <span id="naivedatetime-timestamp-micros"></span>`const fn timestamp_micros(&self) -> i64`

  Returns the number of non-leap *microseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

- <span id="naivedatetime-timestamp-nanos"></span>`const fn timestamp_nanos(&self) -> i64`

  Returns the number of non-leap *nanoseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

  

  # Panics

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function panics on

  an out of range `NaiveDateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

- <span id="naivedatetime-timestamp-nanos-opt"></span>`const fn timestamp_nanos_opt(&self) -> Option<i64>`

  Returns the number of non-leap *nanoseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

  

  # Errors

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function returns

  `None` on an out of range `NaiveDateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

- <span id="naivedatetime-timestamp-subsec-millis"></span>`const fn timestamp_subsec_millis(&self) -> u32`

  Returns the number of milliseconds since the last whole non-leap second.

  

  The return value ranges from 0 to 999,

  or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999.

- <span id="naivedatetime-timestamp-subsec-micros"></span>`const fn timestamp_subsec_micros(&self) -> u32`

  Returns the number of microseconds since the last whole non-leap second.

  

  The return value ranges from 0 to 999,999,

  or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999,999.

- <span id="naivedatetime-timestamp-subsec-nanos"></span>`const fn timestamp_subsec_nanos(&self) -> u32`

  Returns the number of nanoseconds since the last whole non-leap second.

  

  The return value ranges from 0 to 999,999,999,

  or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999,999,999.

- <span id="naivedatetime-checked-add-signed"></span>`const fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime>` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveDateTime`](#naivedatetime)

  Adds given `TimeDelta` to the current date and time.

  

  As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),

  the addition assumes that **there is no leap second ever**,

  except when the `NaiveDateTime` itself represents a leap second

  in which case the assumption becomes that **there is exactly a single leap second ever**.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  let d = from_ymd(2016, 7, 8);

  let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::zero()), Some(hms(3, 5, 7)));

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(1).unwrap()),

      Some(hms(3, 5, 8))

  );

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(-1).unwrap()),

      Some(hms(3, 5, 6))

  );

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(3600 + 60).unwrap()),

      Some(hms(4, 6, 7))

  );

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(86_400).unwrap()),

      Some(from_ymd(2016, 7, 9).and_hms_opt(3, 5, 7).unwrap())

  );

  

  let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();

  assert_eq!(

      hmsm(3, 5, 7, 980).checked_add_signed(TimeDelta::try_milliseconds(450).unwrap()),

      Some(hmsm(3, 5, 8, 430))

  );

  ```

  

  Overflow returns `None`.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let hms = |h, m, s| NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  ```

  

  Leap seconds are handled,

  but the addition assumes that it is the only leap second happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();

  let leap = hmsm(3, 5, 59, 1_300);

  assert_eq!(leap.checked_add_signed(TimeDelta::zero()),

             Some(hmsm(3, 5, 59, 1_300)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_milliseconds(-500).unwrap()),

             Some(hmsm(3, 5, 59, 800)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_milliseconds(500).unwrap()),

             Some(hmsm(3, 5, 59, 1_800)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_milliseconds(800).unwrap()),

             Some(hmsm(3, 6, 0, 100)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_seconds(10).unwrap()),

             Some(hmsm(3, 6, 9, 300)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_seconds(-10).unwrap()),

             Some(hmsm(3, 5, 50, 300)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_days(1).unwrap()),

             Some(from_ymd(2016, 7, 9).and_hms_milli_opt(3, 5, 59, 300).unwrap()));

  ```

- <span id="naivedatetime-checked-add-months"></span>`const fn checked_add_months(self, rhs: Months) -> Option<NaiveDateTime>` — [`Months`](../../month/index.md#months), [`NaiveDateTime`](#naivedatetime)

  Adds given `Months` to the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{Months, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_add_months(Months::new(1)),

      Some(NaiveDate::from_ymd_opt(2014, 2, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())

  );

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_add_months(Months::new(core::i32::MAX as u32 + 1)),

      None

  );

  ```

- <span id="naivedatetime-checked-add-offset"></span>`const fn checked_add_offset(self, rhs: FixedOffset) -> Option<NaiveDateTime>` — [`FixedOffset`](../../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](#naivedatetime)

  Adds given `FixedOffset` to the current datetime.

  Returns `None` if the result would be outside the valid range for [`NaiveDateTime`](#naivedatetime).

  

  This method is similar to [`checked_add_signed`](#method.checked_add_offset), but preserves

  leap seconds.

- <span id="naivedatetime-checked-sub-offset"></span>`const fn checked_sub_offset(self, rhs: FixedOffset) -> Option<NaiveDateTime>` — [`FixedOffset`](../../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](#naivedatetime)

  Subtracts given `FixedOffset` from the current datetime.

  Returns `None` if the result would be outside the valid range for [`NaiveDateTime`](#naivedatetime).

  

  This method is similar to [`checked_sub_signed`](#method.checked_sub_signed), but preserves

  leap seconds.

- <span id="naivedatetime-overflowing-add-offset"></span>`fn overflowing_add_offset(self, rhs: FixedOffset) -> NaiveDateTime` — [`FixedOffset`](../../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](#naivedatetime)

  Adds given `FixedOffset` to the current datetime.

  The resulting value may be outside the valid range of [`NaiveDateTime`](#naivedatetime).

  

  This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`

  should not be exposed to library users.

- <span id="naivedatetime-overflowing-sub-offset"></span>`fn overflowing_sub_offset(self, rhs: FixedOffset) -> NaiveDateTime` — [`FixedOffset`](../../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](#naivedatetime)

  Subtracts given `FixedOffset` from the current datetime.

  The resulting value may be outside the valid range of [`NaiveDateTime`](#naivedatetime).

  

  This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`

  should not be exposed to library users.

- <span id="naivedatetime-checked-sub-signed"></span>`const fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime>` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveDateTime`](#naivedatetime)

  Subtracts given `TimeDelta` from the current date and time.

  

  As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),

  the subtraction assumes that **there is no leap second ever**,

  except when the `NaiveDateTime` itself represents a leap second

  in which case the assumption becomes that **there is exactly a single leap second ever**.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  let d = from_ymd(2016, 7, 8);

  let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::zero()), Some(hms(3, 5, 7)));

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(1).unwrap()),

      Some(hms(3, 5, 6))

  );

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(-1).unwrap()),

      Some(hms(3, 5, 8))

  );

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(3600 + 60).unwrap()),

      Some(hms(2, 4, 7))

  );

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(86_400).unwrap()),

      Some(from_ymd(2016, 7, 7).and_hms_opt(3, 5, 7).unwrap())

  );

  

  let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();

  assert_eq!(

      hmsm(3, 5, 7, 450).checked_sub_signed(TimeDelta::try_milliseconds(670).unwrap()),

      Some(hmsm(3, 5, 6, 780))

  );

  ```

  

  Overflow returns `None`.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let hms = |h, m, s| NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  ```

  

  Leap seconds are handled,

  but the subtraction assumes that it is the only leap second happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();

  let leap = hmsm(3, 5, 59, 1_300);

  assert_eq!(leap.checked_sub_signed(TimeDelta::zero()),

             Some(hmsm(3, 5, 59, 1_300)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_milliseconds(200).unwrap()),

             Some(hmsm(3, 5, 59, 1_100)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_milliseconds(500).unwrap()),

             Some(hmsm(3, 5, 59, 800)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_seconds(60).unwrap()),

             Some(hmsm(3, 5, 0, 300)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_days(1).unwrap()),

             Some(from_ymd(2016, 7, 7).and_hms_milli_opt(3, 6, 0, 300).unwrap()));

  ```

- <span id="naivedatetime-checked-sub-months"></span>`const fn checked_sub_months(self, rhs: Months) -> Option<NaiveDateTime>` — [`Months`](../../month/index.md#months), [`NaiveDateTime`](#naivedatetime)

  Subtracts given `Months` from the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{Months, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_sub_months(Months::new(1)),

      Some(NaiveDate::from_ymd_opt(2013, 12, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())

  );

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),

      None

  );

  ```

- <span id="naivedatetime-checked-add-days"></span>`const fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](../index.md#days)

  Add a duration in [`Days`](../index.md) to the date part of the `NaiveDateTime`

  

  Returns `None` if the resulting date would be out of range.

- <span id="naivedatetime-checked-sub-days"></span>`const fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](../index.md#days)

  Subtract a duration in [`Days`](../index.md) from the date part of the `NaiveDateTime`

  

  Returns `None` if the resulting date would be out of range.

- <span id="naivedatetime-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: NaiveDateTime) -> TimeDelta` — [`NaiveDateTime`](#naivedatetime), [`TimeDelta`](../../time_delta/index.md#timedelta)

  Subtracts another `NaiveDateTime` from the current date and time.

  This does not overflow or underflow at all.

  

  As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),

  the subtraction assumes that **there is no leap second ever**,

  except when any of the `NaiveDateTime`s themselves represents a leap second

  in which case the assumption becomes that

  **there are exactly one (or two) leap second(s) ever**.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  let d = from_ymd(2016, 7, 8);

  assert_eq!(

      d.and_hms_opt(3, 5, 7).unwrap().signed_duration_since(d.and_hms_opt(2, 4, 6).unwrap()),

      TimeDelta::try_seconds(3600 + 60 + 1).unwrap()

  );

  

  // July 8 is 190th day in the year 2016

  let d0 = from_ymd(2016, 1, 1);

  assert_eq!(

      d.and_hms_milli_opt(0, 7, 6, 500)

          .unwrap()

          .signed_duration_since(d0.and_hms_opt(0, 0, 0).unwrap()),

      TimeDelta::try_seconds(189 * 86_400 + 7 * 60 + 6).unwrap()

          + TimeDelta::try_milliseconds(500).unwrap()

  );

  ```

  

  Leap seconds are handled, but the subtraction assumes that

  there were no other leap seconds happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let leap = from_ymd(2015, 6, 30).and_hms_milli_opt(23, 59, 59, 1_500).unwrap();

  assert_eq!(

      leap.signed_duration_since(from_ymd(2015, 6, 30).and_hms_opt(23, 0, 0).unwrap()),

      TimeDelta::try_seconds(3600).unwrap() + TimeDelta::try_milliseconds(500).unwrap()

  );

  assert_eq!(

      from_ymd(2015, 7, 1).and_hms_opt(1, 0, 0).unwrap().signed_duration_since(leap),

      TimeDelta::try_seconds(3600).unwrap() - TimeDelta::try_milliseconds(500).unwrap()

  );

  ```

- <span id="naivedatetime-and-local-timezone"></span>`fn and_local_timezone<Tz: TimeZone>(&self, tz: Tz) -> MappedLocalTime<DateTime<Tz>>` — [`MappedLocalTime`](../../offset/index.md#mappedlocaltime), [`DateTime`](../../datetime/index.md#datetime)

  Converts the `NaiveDateTime` into a timezone-aware `DateTime<Tz>` with the provided

  time zone.

  

  # Example

  

  ```rust

  use chrono::{FixedOffset, NaiveDate};

  let hour = 3600;

  let tz = FixedOffset::east_opt(5 * hour).unwrap();

  let dt = NaiveDate::from_ymd_opt(2015, 9, 5)

      .unwrap()

      .and_hms_opt(23, 56, 4)

      .unwrap()

      .and_local_timezone(tz)

      .unwrap();

  assert_eq!(dt.timezone(), tz);

  ```

- <span id="naivedatetime-and-utc"></span>`const fn and_utc(&self) -> DateTime<Utc>` — [`DateTime`](../../datetime/index.md#datetime), [`Utc`](../../offset/utc/index.md#utc)

  Converts the `NaiveDateTime` into the timezone-aware `DateTime<Utc>`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  let dt =

      NaiveDate::from_ymd_opt(2023, 1, 30).unwrap().and_hms_opt(19, 32, 33).unwrap().and_utc();

  assert_eq!(dt.timezone(), Utc);

  ```

- <span id="naivedatetime-const-min"></span>`const MIN: Self`

- <span id="naivedatetime-const-max"></span>`const MAX: Self`

- <span id="naivedatetime-const-unix-epoch"></span>`const UNIX_EPOCH: Self`

#### Trait Implementations

##### `impl Add for NaiveDateTime`

- <span id="naivedatetime-add-type-output"></span>`type Output = NaiveDateTime`

- <span id="naivedatetime-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveDateTime` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveDateTime`](#naivedatetime)

##### `impl AddAssign for NaiveDateTime`

- <span id="naivedatetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../../time_delta/index.md#timedelta)

##### `impl Clone for NaiveDateTime`

- <span id="naivedatetime-clone"></span>`fn clone(&self) -> NaiveDateTime` — [`NaiveDateTime`](#naivedatetime)

##### `impl Copy for NaiveDateTime`

##### `impl Datelike for NaiveDateTime`

- <span id="naivedatetime-datelike-year"></span>`fn year(&self) -> i32`

  Returns the year number in the [calendar date](./struct.NaiveDate.html#calendar-date).

  

  See also the [`NaiveDate::year`](./struct.NaiveDate.html#method.year) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.year(), 2015);

  ```

- <span id="naivedatetime-datelike-month"></span>`fn month(&self) -> u32`

  Returns the month number starting from 1.

  

  The return value ranges from 1 to 12.

  

  See also the [`NaiveDate::month`](./struct.NaiveDate.html#method.month) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.month(), 9);

  ```

- <span id="naivedatetime-datelike-month0"></span>`fn month0(&self) -> u32`

  Returns the month number starting from 0.

  

  The return value ranges from 0 to 11.

  

  See also the `NaiveDate::month0` method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.month0(), 8);

  ```

- <span id="naivedatetime-datelike-day"></span>`fn day(&self) -> u32`

  Returns the day of month starting from 1.

  

  The return value ranges from 1 to 31. (The last day of month differs by months.)

  

  See also the [`NaiveDate::day`](./struct.NaiveDate.html#method.day) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.day(), 25);

  ```

- <span id="naivedatetime-datelike-day0"></span>`fn day0(&self) -> u32`

  Returns the day of month starting from 0.

  

  The return value ranges from 0 to 30. (The last day of month differs by months.)

  

  See also the `NaiveDate::day0` method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.day0(), 24);

  ```

- <span id="naivedatetime-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

  Returns the day of year starting from 1.

  

  The return value ranges from 1 to 366. (The last day of year differs by years.)

  

  See also the [`NaiveDate::ordinal`](./struct.NaiveDate.html#method.ordinal) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.ordinal(), 268);

  ```

- <span id="naivedatetime-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

  Returns the day of year starting from 0.

  

  The return value ranges from 0 to 365. (The last day of year differs by years.)

  

  See also the `NaiveDate::ordinal0` method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.ordinal0(), 267);

  ```

- <span id="naivedatetime-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](../../weekday/index.md#weekday)

  Returns the day of week.

  

  See also the [`NaiveDate::weekday`](./struct.NaiveDate.html#method.weekday) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime, Weekday};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.weekday(), Weekday::Fri);

  ```

- <span id="naivedatetime-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](../isoweek/index.md#isoweek)

- <span id="naivedatetime-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the year number changed, while keeping the same month and

  day.

  

  See also the `NaiveDate::with_year` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (February 29 in a non-leap year).

  - The year is out of range for a `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_year(2016),

      Some(NaiveDate::from_ymd_opt(2016, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(

      dt.with_year(-308),

      Some(NaiveDate::from_ymd_opt(-308, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  ```

- <span id="naivedatetime-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the month number (starting from 1) changed.

  

  Don't combine multiple `Datelike::with_*` methods. The intermediate value may not exist.

  

  See also the `NaiveDate::with_month` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month(4)` when day of the month is 31).

  - The value for `month` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_month(10),

      Some(NaiveDate::from_ymd_opt(2015, 10, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_month(13), None); // No month 13

  assert_eq!(dt.with_month(2), None); // No February 30

  ```

- <span id="naivedatetime-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the month number (starting from 0) changed.

  

  See also the `NaiveDate::with_month0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month0(3)` when day of the month is 31).

  - The value for `month0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_month0(9),

      Some(NaiveDate::from_ymd_opt(2015, 10, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_month0(12), None); // No month 13

  assert_eq!(dt.with_month0(1), None); // No February 30

  ```

- <span id="naivedatetime-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the day of month (starting from 1) changed.

  

  See also the `NaiveDate::with_day` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(31)` in April).

  - The value for `day` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_day(30),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_day(31), None); // no September 31

  ```

- <span id="naivedatetime-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the day of month (starting from 0) changed.

  

  See also the `NaiveDate::with_day0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(30)` in April).

  - The value for `day0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_day0(29),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_day0(30), None); // no September 31

  ```

- <span id="naivedatetime-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the day of year (starting from 1) changed.

  

  See also the `NaiveDate::with_ordinal` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal(366)` in a non-leap year).

  - The value for `ordinal` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal(60),

      Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_ordinal(366), None); // 2015 had only 365 days

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2016, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal(60),

      Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(

      dt.with_ordinal(366),

      Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  ```

- <span id="naivedatetime-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the day of year (starting from 0) changed.

  

  See also the `NaiveDate::with_ordinal0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal0(365)` in a non-leap year).

  - The value for `ordinal0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal0(59),

      Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_ordinal0(365), None); // 2015 had only 365 days

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2016, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal0(59),

      Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(

      dt.with_ordinal0(365),

      Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  ```

##### `impl Debug for NaiveDateTime`

- <span id="naivedatetime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NaiveDateTime`

- <span id="naivedatetime-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::NaiveDateTime`

- <span id="supernaivedatetime-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for NaiveDateTime`

##### `impl Display for NaiveDateTime`

- <span id="naivedatetime-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DurationRound for crate::NaiveDateTime`

- <span id="cratenaivedatetime-durationround-type-err"></span>`type Err = RoundingError`

- <span id="cratenaivedatetime-durationround-duration-round"></span>`fn duration_round(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`DurationRound`](../../round/index.md#durationround)

- <span id="cratenaivedatetime-durationround-duration-trunc"></span>`fn duration_trunc(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`DurationRound`](../../round/index.md#durationround)

- <span id="cratenaivedatetime-durationround-duration-round-up"></span>`fn duration_round_up(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`DurationRound`](../../round/index.md#durationround)

##### `impl Eq for NaiveDateTime`

##### `impl FromStr for NaiveDateTime`

- <span id="naivedatetime-fromstr-type-err"></span>`type Err = ParseError`

- <span id="naivedatetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveDateTime>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveDateTime`](#naivedatetime)

##### `impl Hash for NaiveDateTime`

- <span id="naivedatetime-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveDateTime`

- <span id="naivedatetime-ord-cmp"></span>`fn cmp(&self, other: &NaiveDateTime) -> cmp::Ordering` — [`NaiveDateTime`](#naivedatetime)

##### `impl PartialEq for NaiveDateTime`

- <span id="naivedatetime-partialeq-eq"></span>`fn eq(&self, other: &NaiveDateTime) -> bool` — [`NaiveDateTime`](#naivedatetime)

##### `impl PartialOrd for NaiveDateTime`

- <span id="naivedatetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDateTime) -> option::Option<cmp::Ordering>` — [`NaiveDateTime`](#naivedatetime)

##### `impl Serialize for super::NaiveDateTime`

- <span id="supernaivedatetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveDateTime`

##### `impl Sub for NaiveDateTime`

- <span id="naivedatetime-sub-type-output"></span>`type Output = NaiveDateTime`

- <span id="naivedatetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> NaiveDateTime` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveDateTime`](#naivedatetime)

##### `impl SubAssign for NaiveDateTime`

- <span id="naivedatetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../../time_delta/index.md#timedelta)

##### `impl SubsecRound for NaiveDateTime`

- <span id="naivedatetime-subsecround-round-subsecs"></span>`fn round_subsecs(self, digits: u16) -> T`

- <span id="naivedatetime-subsecround-trunc-subsecs"></span>`fn trunc_subsecs(self, digits: u16) -> T`

##### `impl Timelike for NaiveDateTime`

- <span id="naivedatetime-timelike-hour"></span>`fn hour(&self) -> u32`

  Returns the hour number from 0 to 23.

  

  See also the `NaiveTime::hour` method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.hour(), 12);

  ```

- <span id="naivedatetime-timelike-minute"></span>`fn minute(&self) -> u32`

  Returns the minute number from 0 to 59.

  

  See also the `NaiveTime::minute` method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.minute(), 34);

  ```

- <span id="naivedatetime-timelike-second"></span>`fn second(&self) -> u32`

  Returns the second number from 0 to 59.

  

  See also the `NaiveTime::second` method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.second(), 56);

  ```

- <span id="naivedatetime-timelike-nanosecond"></span>`fn nanosecond(&self) -> u32`

  Returns the number of nanoseconds since the whole non-leap second.

  The range from 1,000,000,000 to 1,999,999,999 represents

  the [leap second](./struct.NaiveTime.html#leap-second-handling).

  

  See also the [`NaiveTime#method.nanosecond`](../time/index.md) method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.nanosecond(), 789_000_000);

  ```

- <span id="naivedatetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the hour number changed.

  

  See also the `NaiveTime::with_hour` method.

  

  # Errors

  

  Returns `None` if the value for `hour` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(

      dt.with_hour(7),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(7, 34, 56, 789).unwrap()

      )

  );

  assert_eq!(dt.with_hour(24), None);

  ```

- <span id="naivedatetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the minute number changed.

  

  See also the `NaiveTime::with_minute` method.

  

  # Errors

  

  Returns `None` if the value for `minute` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(

      dt.with_minute(45),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_milli_opt(12, 45, 56, 789)

              .unwrap()

      )

  );

  assert_eq!(dt.with_minute(60), None);

  ```

- <span id="naivedatetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with the second number changed.

  

  As with the [`second`](#method.second) method,

  the input range is restricted to 0 through 59.

  

  See also the `NaiveTime::with_second` method.

  

  # Errors

  

  Returns `None` if the value for `second` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(

      dt.with_second(17),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_milli_opt(12, 34, 17, 789)

              .unwrap()

      )

  );

  assert_eq!(dt.with_second(60), None);

  ```

- <span id="naivedatetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](#naivedatetime)

  Makes a new `NaiveDateTime` with nanoseconds since the whole non-leap second changed.

  

  Returns `None` when the resulting `NaiveDateTime` would be invalid.

  As with the `NaiveDateTime::nanosecond` method,

  the input range can exceed 1,000,000,000 for leap seconds.

  

  See also the `NaiveTime::with_nanosecond` method.

  

  # Errors

  

  Returns `None` if `nanosecond >= 2,000,000,000`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 59, 789).unwrap();

  assert_eq!(

      dt.with_nanosecond(333_333_333),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_nano_opt(12, 34, 59, 333_333_333)

              .unwrap()

      )

  );

  assert_eq!(

      dt.with_nanosecond(1_333_333_333), // leap second

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_nano_opt(12, 34, 59, 1_333_333_333)

              .unwrap()

      )

  );

  assert_eq!(dt.with_nanosecond(2_000_000_000), None);

  ```

##### `impl ToString for NaiveDateTime`

- <span id="naivedatetime-tostring-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `MIN_DATETIME`
```rust
const MIN_DATETIME: NaiveDateTime;
```

The minimum possible `NaiveDateTime`.

### `MAX_DATETIME`
```rust
const MAX_DATETIME: NaiveDateTime;
```

The maximum possible `NaiveDateTime`.

