*[chrono](../index.md) / [naive](index.md)*

---

# Module `naive`

Date and time types unconcerned with timezones.

They are primarily building blocks for other types
(e.g. [`TimeZone`](../offset/trait.TimeZone.html)),
but can be also used for the simpler date and time handling.

## Contents

- [Modules](#modules)
  - [`date`](#date)
  - [`datetime`](#datetime)
  - [`internals`](#internals)
  - [`isoweek`](#isoweek)
  - [`time`](#time)
  - [`serde`](#serde)
- [Structs](#structs)
  - [`NaiveDate`](#naivedate)
  - [`NaiveDateDaysIterator`](#naivedatedaysiterator)
  - [`NaiveDateWeeksIterator`](#naivedateweeksiterator)
  - [`NaiveDateTime`](#naivedatetime)
  - [`IsoWeek`](#isoweek)
  - [`NaiveTime`](#naivetime)
  - [`NaiveWeek`](#naiveweek)
  - [`Days`](#days)
- [Constants](#constants)
  - [`MAX_DATE`](#max-date)
  - [`MIN_DATE`](#min-date)
  - [`MAX_DATETIME`](#max-datetime)
  - [`MIN_DATETIME`](#min-datetime)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`date`](#date) | mod | ISO 8601 calendar date without timezone. |
| [`datetime`](#datetime) | mod | ISO 8601 date and time without timezone. |
| [`internals`](#internals) | mod | Internal helper types for working with dates. |
| [`isoweek`](#isoweek) | mod | ISO 8601 week. |
| [`time`](#time) | mod | ISO 8601 time without timezone. |
| [`serde`](#serde) | mod | Serialization/Deserialization of `NaiveDateTime` in alternate formats |
| [`NaiveDate`](#naivedate) | struct |  |
| [`NaiveDateDaysIterator`](#naivedatedaysiterator) | struct |  |
| [`NaiveDateWeeksIterator`](#naivedateweeksiterator) | struct |  |
| [`NaiveDateTime`](#naivedatetime) | struct |  |
| [`IsoWeek`](#isoweek) | struct |  |
| [`NaiveTime`](#naivetime) | struct |  |
| [`NaiveWeek`](#naiveweek) | struct | A week represented by a [`NaiveDate`] and a [`Weekday`] which is the first day of the week. |
| [`Days`](#days) | struct | A duration in calendar days. |
| [`MAX_DATE`](#max-date) | const |  |
| [`MIN_DATE`](#min-date) | const |  |
| [`MAX_DATETIME`](#max-datetime) | const |  |
| [`MIN_DATETIME`](#min-datetime) | const |  |

## Modules

- [`date`](date/index.md) — ISO 8601 calendar date without timezone.
- [`datetime`](datetime/index.md) — ISO 8601 date and time without timezone.
- [`internals`](internals/index.md) — Internal helper types for working with dates.
- [`isoweek`](isoweek/index.md) — ISO 8601 week.
- [`time`](time/index.md) — ISO 8601 time without timezone.
- [`serde`](serde/index.md) — Serialization/Deserialization of `NaiveDateTime` in alternate formats

## Structs

### `NaiveDate`

```rust
struct NaiveDate {
    yof: core::num::NonZeroI32,
}
```

ISO 8601 calendar date without timezone.
Allows for every [proleptic Gregorian date] from Jan 1, 262145 BCE to Dec 31, 262143 CE.
Also supports the conversion from ISO 8601 ordinal and week date.

# Calendar Date

The ISO 8601 **calendar date** follows the proleptic Gregorian calendar.
It is like a normal civil calendar but note some slight differences:

* Dates before the Gregorian calendar's inception in 1582 are defined via the extrapolation.
  Be careful, as historical dates are often noted in the Julian calendar and others
  and the transition to Gregorian may differ across countries (as late as early 20C).

  (Some example: Both Shakespeare from Britain and Cervantes from Spain seemingly died
  on the same calendar date---April 23, 1616---but in the different calendar.
  Britain used the Julian calendar at that time, so Shakespeare's death is later.)

* ISO 8601 calendars have the year 0, which is 1 BCE (a year before 1 CE).
  If you need a typical BCE/BC and CE/AD notation for year numbers,
  use the `Datelike::year_ce` method.

# Week Date

The ISO 8601 **week date** is a triple of year number, week number
and [day of the week](Weekday) with the following rules:

* A week consists of Monday through Sunday, and is always numbered within some year.
  The week number ranges from 1 to 52 or 53 depending on the year.

* The week 1 of given year is defined as the first week containing January 4 of that year,
  or equivalently, the first week containing four or more days in that year.

* The year number in the week date may *not* correspond to the actual Gregorian year.
  For example, January 3, 2016 (Sunday) was on the last (53rd) week of 2015.

Chrono's date types default to the ISO 8601 [calendar date](#calendar-date), but
`Datelike::iso_week` and `Datelike::weekday` methods can be used to get the corresponding
week date.

# Ordinal Date

The ISO 8601 **ordinal date** is a pair of year number and day of the year ("ordinal").
The ordinal number ranges from 1 to 365 or 366 depending on the year.
The year number is the same as that of the [calendar date](#calendar-date).

This is currently the internal format of Chrono's date types.


#### Implementations

- <span id="naivedate-weeks-from"></span>`fn weeks_from(&self, day: Weekday) -> i32` — [`Weekday`](../weekday/index.md#weekday)

- <span id="naivedate-from-ordinal-and-flags"></span>`const fn from_ordinal_and_flags(year: i32, ordinal: u32, flags: YearFlags) -> Option<NaiveDate>` — [`YearFlags`](internals/index.md#yearflags), [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from year, ordinal and flags.

  Does not check whether the flags are correct for the provided year.

- <span id="naivedate-from-mdf"></span>`const fn from_mdf(year: i32, mdf: Mdf) -> Option<NaiveDate>` — [`Mdf`](internals/index.md#mdf), [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from year and packed month-day-flags.

  Does not check whether the flags are correct for the provided year.

- <span id="naivedate-from-ymd"></span>`const fn from_ymd(year: i32, month: u32, day: u32) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from the [calendar date](#calendar-date)

  (year, month and day).

  

  # Panics

  

  Panics if the specified calendar day does not exist, on invalid values for `month` or `day`,

  or if `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-ymd-opt"></span>`const fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from the [calendar date](#calendar-date)

  (year, month and day).

  

  # Errors

  

  Returns `None` if:

  - The specified calendar day does not exist (for example 2023-04-31).

  - The value for `month` or `day` is invalid.

  - `year` is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ymd_opt = NaiveDate::from_ymd_opt;

  

  assert!(from_ymd_opt(2015, 3, 14).is_some());

  assert!(from_ymd_opt(2015, 0, 14).is_none());

  assert!(from_ymd_opt(2015, 2, 29).is_none());

  assert!(from_ymd_opt(-4, 2, 29).is_some()); // 5 BCE is a leap year

  assert!(from_ymd_opt(400000, 1, 1).is_none());

  assert!(from_ymd_opt(-400000, 1, 1).is_none());

  ```

- <span id="naivedate-from-yo"></span>`const fn from_yo(year: i32, ordinal: u32) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)

  (year and day of the year).

  

  # Panics

  

  Panics if the specified ordinal day does not exist, on invalid values for `ordinal`, or if

  `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-yo-opt"></span>`const fn from_yo_opt(year: i32, ordinal: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)

  (year and day of the year).

  

  # Errors

  

  Returns `None` if:

  - The specified ordinal day does not exist (for example 2023-366).

  - The value for `ordinal` is invalid (for example: `0`, `400`).

  - `year` is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_yo_opt = NaiveDate::from_yo_opt;

  

  assert!(from_yo_opt(2015, 100).is_some());

  assert!(from_yo_opt(2015, 0).is_none());

  assert!(from_yo_opt(2015, 365).is_some());

  assert!(from_yo_opt(2015, 366).is_none());

  assert!(from_yo_opt(-4, 366).is_some()); // 5 BCE is a leap year

  assert!(from_yo_opt(400000, 1).is_none());

  assert!(from_yo_opt(-400000, 1).is_none());

  ```

- <span id="naivedate-from-isoywd"></span>`const fn from_isoywd(year: i32, week: u32, weekday: Weekday) -> NaiveDate` — [`Weekday`](../weekday/index.md#weekday), [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ISO week date](#week-date)

  (year, week number and day of the week).

  The resulting `NaiveDate` may have a different year from the input year.

  

  # Panics

  

  Panics if the specified week does not exist in that year, on invalid values for `week`, or

  if the resulting date is out of range for `NaiveDate`.

- <span id="naivedate-from-isoywd-opt"></span>`const fn from_isoywd_opt(year: i32, week: u32, weekday: Weekday) -> Option<NaiveDate>` — [`Weekday`](../weekday/index.md#weekday), [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ISO week date](#week-date)

  (year, week number and day of the week).

  The resulting `NaiveDate` may have a different year from the input year.

  

  # Errors

  

  Returns `None` if:

  - The specified week does not exist in that year (for example 2023 week 53).

  - The value for `week` is invalid (for example: `0`, `60`).

  - If the resulting date is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let from_isoywd_opt = NaiveDate::from_isoywd_opt;

  

  assert_eq!(from_isoywd_opt(2015, 0, Weekday::Sun), None);

  assert_eq!(from_isoywd_opt(2015, 10, Weekday::Sun), Some(from_ymd(2015, 3, 8)));

  assert_eq!(from_isoywd_opt(2015, 30, Weekday::Mon), Some(from_ymd(2015, 7, 20)));

  assert_eq!(from_isoywd_opt(2015, 60, Weekday::Mon), None);

  

  assert_eq!(from_isoywd_opt(400000, 10, Weekday::Fri), None);

  assert_eq!(from_isoywd_opt(-400000, 10, Weekday::Sat), None);

  ```

  

  The year number of ISO week date may differ from that of the calendar date.

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let from_isoywd_opt = NaiveDate::from_isoywd_opt;

  //           Mo Tu We Th Fr Sa Su

  // 2014-W52  22 23 24 25 26 27 28    has 4+ days of new year,

  // 2015-W01  29 30 31  1  2  3  4 <- so this is the first week

  assert_eq!(from_isoywd_opt(2014, 52, Weekday::Sun), Some(from_ymd(2014, 12, 28)));

  assert_eq!(from_isoywd_opt(2014, 53, Weekday::Mon), None);

  assert_eq!(from_isoywd_opt(2015, 1, Weekday::Mon), Some(from_ymd(2014, 12, 29)));

  

  // 2015-W52  21 22 23 24 25 26 27    has 4+ days of old year,

  // 2015-W53  28 29 30 31  1  2  3 <- so this is the last week

  // 2016-W01   4  5  6  7  8  9 10

  assert_eq!(from_isoywd_opt(2015, 52, Weekday::Sun), Some(from_ymd(2015, 12, 27)));

  assert_eq!(from_isoywd_opt(2015, 53, Weekday::Sun), Some(from_ymd(2016, 1, 3)));

  assert_eq!(from_isoywd_opt(2015, 54, Weekday::Mon), None);

  assert_eq!(from_isoywd_opt(2016, 1, Weekday::Mon), Some(from_ymd(2016, 1, 4)));

  ```

- <span id="naivedate-from-num-days-from-ce"></span>`const fn from_num_days_from_ce(days: i32) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with

  January 1, 1 being day 1.

  

  # Panics

  

  Panics if the date is out of range.

- <span id="naivedate-from-num-days-from-ce-opt"></span>`const fn from_num_days_from_ce_opt(days: i32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with

  January 1, 1 being day 1.

  

  # Errors

  

  Returns `None` if the date is out of range.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ndays_opt = NaiveDate::from_num_days_from_ce_opt;

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  assert_eq!(from_ndays_opt(730_000), Some(from_ymd(1999, 9, 3)));

  assert_eq!(from_ndays_opt(1), Some(from_ymd(1, 1, 1)));

  assert_eq!(from_ndays_opt(0), Some(from_ymd(0, 12, 31)));

  assert_eq!(from_ndays_opt(-1), Some(from_ymd(0, 12, 30)));

  assert_eq!(from_ndays_opt(100_000_000), None);

  assert_eq!(from_ndays_opt(-100_000_000), None);

  ```

- <span id="naivedate-from-epoch-days"></span>`const fn from_epoch_days(days: i32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with

  January 1, 1970 being day 0.

  

  # Errors

  

  Returns `None` if the date is out of range.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ndays_opt = NaiveDate::from_epoch_days;

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  assert_eq!(from_ndays_opt(-719_162), Some(from_ymd(1, 1, 1)));

  assert_eq!(from_ndays_opt(1), Some(from_ymd(1970, 1, 2)));

  assert_eq!(from_ndays_opt(0), Some(from_ymd(1970, 1, 1)));

  assert_eq!(from_ndays_opt(-1), Some(from_ymd(1969, 12, 31)));

  assert_eq!(from_ndays_opt(13036), Some(from_ymd(2005, 9, 10)));

  assert_eq!(from_ndays_opt(100_000_000), None);

  assert_eq!(from_ndays_opt(-100_000_000), None);

  ```

- <span id="naivedate-from-weekday-of-month"></span>`const fn from_weekday_of_month(year: i32, month: u32, weekday: Weekday, n: u8) -> NaiveDate` — [`Weekday`](../weekday/index.md#weekday), [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week

  since the beginning of the given month. For instance, if you want the 2nd Friday of March

  2017, you would use `NaiveDate::from_weekday_of_month(2017, 3, Weekday::Fri, 2)`.

  

  `n` is 1-indexed.

  

  # Panics

  

  Panics if the specified day does not exist in that month, on invalid values for `month` or

  `n`, or if `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-weekday-of-month-opt"></span>`const fn from_weekday_of_month_opt(year: i32, month: u32, weekday: Weekday, n: u8) -> Option<NaiveDate>` — [`Weekday`](../weekday/index.md#weekday), [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week

  since the beginning of the given month. For instance, if you want the 2nd Friday of March

  2017, you would use `NaiveDate::from_weekday_of_month(2017, 3, Weekday::Fri, 2)`.

  

  `n` is 1-indexed.

  

  # Errors

  

  Returns `None` if:

  - The specified day does not exist in that month (for example the 5th Monday of Apr. 2023).

  - The value for `month` or `n` is invalid.

  - `year` is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  assert_eq!(

      NaiveDate::from_weekday_of_month_opt(2017, 3, Weekday::Fri, 2),

      NaiveDate::from_ymd_opt(2017, 3, 10)

  )

  ```

- <span id="naivedate-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDate>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveDate`](date/index.md#naivedate)

  Parses a string with the specified format string and returns a new `NaiveDate`.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let parse_from_str = NaiveDate::parse_from_str;

  

  assert_eq!(

      parse_from_str("2015-09-05", "%Y-%m-%d"),

      Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap())

  );

  assert_eq!(

      parse_from_str("5sep2015", "%d%b%Y"),

      Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap())

  );

  ```

  

  Time and offset is ignored for the purpose of parsing.

  

  ```rust

  use chrono::NaiveDate;

  let parse_from_str = NaiveDate::parse_from_str;

  assert_eq!(

      parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),

      Ok(NaiveDate::from_ymd_opt(2014, 5, 17).unwrap())

  );

  ```

  

  Out-of-bound dates or insufficient fields are errors.

  

  ```rust

  use chrono::NaiveDate;

  let parse_from_str = NaiveDate::parse_from_str;

  assert!(parse_from_str("2015/9", "%Y/%m").is_err());

  assert!(parse_from_str("2015/9/31", "%Y/%m/%d").is_err());

  ```

  

  All parsed fields should be consistent to each other, otherwise it's an error.

  

  ```rust

  use chrono::NaiveDate;

  let parse_from_str = NaiveDate::parse_from_str;

  assert!(parse_from_str("Sat, 09 Aug 2013", "%a, %d %b %Y").is_err());

  ```

- <span id="naivedate-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDate, &'a str)>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveDate`](date/index.md#naivedate)

  Parses a string from a user-specified format into a new `NaiveDate` value, and a slice with

  the remaining portion of the string.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  Similar to [`parse_from_str`](#method.parse_from_str).

  

  # Example

  

  ```rust

  use chrono::{NaiveDate};

  let (date, remainder) =

      NaiveDate::parse_and_remainder("2015-02-18 trailing text", "%Y-%m-%d").unwrap();

  assert_eq!(date, NaiveDate::from_ymd_opt(2015, 2, 18).unwrap());

  assert_eq!(remainder, " trailing text");

  ```

- <span id="naivedate-checked-add-months"></span>`const fn checked_add_months(self, months: Months) -> Option<Self>` — [`Months`](../month/index.md#months)

  Add a duration in [`Months`](../month/index.md) to the date

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Months};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_add_months(Months::new(6)),

      Some(NaiveDate::from_ymd_opt(2022, 8, 20).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_months(Months::new(2)),

      Some(NaiveDate::from_ymd_opt(2022, 9, 30).unwrap())

  );

  ```

- <span id="naivedate-checked-sub-months"></span>`const fn checked_sub_months(self, months: Months) -> Option<Self>` — [`Months`](../month/index.md#months)

  Subtract a duration in [`Months`](../month/index.md) from the date

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Months};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_months(Months::new(6)),

      Some(NaiveDate::from_ymd_opt(2021, 8, 20).unwrap())

  );

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),

      None

  );

  ```

- <span id="naivedate-diff-months"></span>`const fn diff_months(self, months: i32) -> Option<Self>`

- <span id="naivedate-checked-add-days"></span>`const fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](#days)

  Add a duration in [`Days`](#days) to the date

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Days};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_add_days(Days::new(9)),

      Some(NaiveDate::from_ymd_opt(2022, 3, 1).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_days(Days::new(2)),

      Some(NaiveDate::from_ymd_opt(2022, 8, 2).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_days(Days::new(1000000000000)),

      None

  );

  ```

- <span id="naivedate-checked-sub-days"></span>`const fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](#days)

  Subtract a duration in [`Days`](#days) from the date

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Days};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_days(Days::new(6)),

      Some(NaiveDate::from_ymd_opt(2022, 2, 14).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_days(Days::new(1000000000000)),

      None

  );

  ```

- <span id="naivedate-add-days"></span>`const fn add_days(self, days: i32) -> Option<Self>`

  Add a duration of `i32` days to the date.

- <span id="naivedate-and-time"></span>`const fn and_time(&self, time: NaiveTime) -> NaiveDateTime` — [`NaiveTime`](time/index.md#naivetime), [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date and given `NaiveTime`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

  

  let dt: NaiveDateTime = d.and_time(t);

  assert_eq!(dt.date(), d);

  assert_eq!(dt.time(), t);

  ```

- <span id="naivedate-and-hms"></span>`const fn and_hms(&self, hour: u32, min: u32, sec: u32) -> NaiveDateTime` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute and second.

  

  No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;

  use `NaiveDate::and_hms_*` methods with a subsecond parameter instead.

  

  # Panics

  

  Panics on invalid hour, minute and/or second.

- <span id="naivedate-and-hms-opt"></span>`const fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute and second.

  

  No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;

  use `NaiveDate::and_hms_*_opt` methods with a subsecond parameter instead.

  

  # Errors

  

  Returns `None` on invalid hour, minute and/or second.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_opt(12, 34, 56).is_some());

  assert!(d.and_hms_opt(12, 34, 60).is_none()); // use `and_hms_milli_opt` instead

  assert!(d.and_hms_opt(12, 60, 56).is_none());

  assert!(d.and_hms_opt(24, 34, 56).is_none());

  ```

- <span id="naivedate-and-hms-milli"></span>`const fn and_hms_milli(&self, hour: u32, min: u32, sec: u32, milli: u32) -> NaiveDateTime` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.

  

  The millisecond part is allowed to exceed 1,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="naivedate-and-hms-milli-opt"></span>`const fn and_hms_milli_opt(&self, hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.

  

  The millisecond part is allowed to exceed 1,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or millisecond.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_milli_opt(12, 34, 56, 789).is_some());

  assert!(d.and_hms_milli_opt(12, 34, 59, 1_789).is_some()); // leap second

  assert!(d.and_hms_milli_opt(12, 34, 59, 2_789).is_none());

  assert!(d.and_hms_milli_opt(12, 34, 60, 789).is_none());

  assert!(d.and_hms_milli_opt(12, 60, 56, 789).is_none());

  assert!(d.and_hms_milli_opt(24, 34, 56, 789).is_none());

  ```

- <span id="naivedate-and-hms-micro"></span>`const fn and_hms_micro(&self, hour: u32, min: u32, sec: u32, micro: u32) -> NaiveDateTime` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or microsecond.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Weekday};

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  

  let dt: NaiveDateTime = d.and_hms_micro_opt(12, 34, 56, 789_012).unwrap();

  assert_eq!(dt.year(), 2015);

  assert_eq!(dt.weekday(), Weekday::Wed);

  assert_eq!(dt.second(), 56);

  assert_eq!(dt.nanosecond(), 789_012_000);

  ```

- <span id="naivedate-and-hms-micro-opt"></span>`const fn and_hms_micro_opt(&self, hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or microsecond.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_micro_opt(12, 34, 56, 789_012).is_some());

  assert!(d.and_hms_micro_opt(12, 34, 59, 1_789_012).is_some()); // leap second

  assert!(d.and_hms_micro_opt(12, 34, 59, 2_789_012).is_none());

  assert!(d.and_hms_micro_opt(12, 34, 60, 789_012).is_none());

  assert!(d.and_hms_micro_opt(12, 60, 56, 789_012).is_none());

  assert!(d.and_hms_micro_opt(24, 34, 56, 789_012).is_none());

  ```

- <span id="naivedate-and-hms-nano"></span>`const fn and_hms_nano(&self, hour: u32, min: u32, sec: u32, nano: u32) -> NaiveDateTime` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="naivedate-and-hms-nano-opt"></span>`const fn and_hms_nano_opt(&self, hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or nanosecond.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_nano_opt(12, 34, 56, 789_012_345).is_some());

  assert!(d.and_hms_nano_opt(12, 34, 59, 1_789_012_345).is_some()); // leap second

  assert!(d.and_hms_nano_opt(12, 34, 59, 2_789_012_345).is_none());

  assert!(d.and_hms_nano_opt(12, 34, 60, 789_012_345).is_none());

  assert!(d.and_hms_nano_opt(12, 60, 56, 789_012_345).is_none());

  assert!(d.and_hms_nano_opt(24, 34, 56, 789_012_345).is_none());

  ```

- <span id="naivedate-mdf"></span>`const fn mdf(&self) -> Mdf` — [`Mdf`](internals/index.md#mdf)

  Returns the packed month-day-flags.

- <span id="naivedate-with-mdf"></span>`const fn with_mdf(&self, mdf: Mdf) -> Option<NaiveDate>` — [`Mdf`](internals/index.md#mdf), [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the packed month-day-flags changed.

  

  Returns `None` when the resulting `NaiveDate` would be invalid.

- <span id="naivedate-succ"></span>`const fn succ(&self) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` for the next calendar date.

  

  # Panics

  

  Panics when `self` is the last representable date.

- <span id="naivedate-succ-opt"></span>`const fn succ_opt(&self) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` for the next calendar date.

  

  # Errors

  

  Returns `None` when `self` is the last representable date.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 6, 3).unwrap().succ_opt(),

      Some(NaiveDate::from_ymd_opt(2015, 6, 4).unwrap())

  );

  assert_eq!(NaiveDate::MAX.succ_opt(), None);

  ```

- <span id="naivedate-pred"></span>`const fn pred(&self) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` for the previous calendar date.

  

  # Panics

  

  Panics when `self` is the first representable date.

- <span id="naivedate-pred-opt"></span>`const fn pred_opt(&self) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` for the previous calendar date.

  

  # Errors

  

  Returns `None` when `self` is the first representable date.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 6, 3).unwrap().pred_opt(),

      Some(NaiveDate::from_ymd_opt(2015, 6, 2).unwrap())

  );

  assert_eq!(NaiveDate::MIN.pred_opt(), None);

  ```

- <span id="naivedate-checked-add-signed"></span>`const fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDate>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveDate`](date/index.md#naivedate)

  Adds the number of whole days in the given `TimeDelta` to the current date.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

  assert_eq!(

      d.checked_add_signed(TimeDelta::try_days(40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 10, 15).unwrap())

  );

  assert_eq!(

      d.checked_add_signed(TimeDelta::try_days(-40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 7, 27).unwrap())

  );

  assert_eq!(d.checked_add_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  assert_eq!(d.checked_add_signed(TimeDelta::try_days(-1_000_000_000).unwrap()), None);

  assert_eq!(NaiveDate::MAX.checked_add_signed(TimeDelta::try_days(1).unwrap()), None);

  ```

- <span id="naivedate-checked-sub-signed"></span>`const fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDate>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveDate`](date/index.md#naivedate)

  Subtracts the number of whole days in the given `TimeDelta` from the current date.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

  assert_eq!(

      d.checked_sub_signed(TimeDelta::try_days(40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 7, 27).unwrap())

  );

  assert_eq!(

      d.checked_sub_signed(TimeDelta::try_days(-40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 10, 15).unwrap())

  );

  assert_eq!(d.checked_sub_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  assert_eq!(d.checked_sub_signed(TimeDelta::try_days(-1_000_000_000).unwrap()), None);

  assert_eq!(NaiveDate::MIN.checked_sub_signed(TimeDelta::try_days(1).unwrap()), None);

  ```

- <span id="naivedate-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: Self) -> TimeDelta` — [`TimeDelta`](../time_delta/index.md#timedelta)

  Subtracts another `NaiveDate` from the current date.

  Returns a `TimeDelta` of integral numbers.

  

  This does not overflow or underflow at all,

  as all possible output fits in the range of `TimeDelta`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let since = NaiveDate::signed_duration_since;

  

  assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2014, 1, 1)), TimeDelta::zero());

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2013, 12, 31)),

      TimeDelta::try_days(1).unwrap()

  );

  assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2014, 1, 2)), TimeDelta::try_days(-1).unwrap());

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2013, 9, 23)),

      TimeDelta::try_days(100).unwrap()

  );

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2013, 1, 1)),

      TimeDelta::try_days(365).unwrap()

  );

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2010, 1, 1)),

      TimeDelta::try_days(365 * 4 + 1).unwrap()

  );

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(1614, 1, 1)),

      TimeDelta::try_days(365 * 400 + 97).unwrap()

  );

  ```

- <span id="naivedate-abs-diff"></span>`const fn abs_diff(self, rhs: Self) -> Days` — [`Days`](#days)

  Returns the absolute difference between two `NaiveDate`s measured as the number of days.

  

  This is always an integer, non-negative number, similar to `abs_diff` in `std`.

  

  # Example

  

  ```rust

  use chrono::{Days, NaiveDate};

  

  let date1: NaiveDate = "2020-01-01".parse().unwrap();

  let date2: NaiveDate = "2020-01-31".parse().unwrap();

  assert_eq!(date2.abs_diff(date1), Days::new(30));

  assert_eq!(date1.abs_diff(date2), Days::new(30));

  ```

- <span id="naivedate-years-since"></span>`const fn years_since(&self, base: Self) -> Option<u32>`

  Returns the number of whole years from the given `base` until `self`.

  

  # Errors

  

  Returns `None` if `base > self`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate};

  

  let base: NaiveDate = "2025-01-01".parse().unwrap();

  let date: NaiveDate = "2030-01-01".parse().unwrap();

  

  assert_eq!(date.years_since(base), Some(5))

  ```

- <span id="naivedate-iter-days"></span>`const fn iter_days(&self) -> NaiveDateDaysIterator` — [`NaiveDateDaysIterator`](date/index.md#naivedatedaysiterator)

  Returns an iterator that steps by days across all representable dates.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let expected = [

      NaiveDate::from_ymd_opt(2016, 2, 27).unwrap(),

      NaiveDate::from_ymd_opt(2016, 2, 28).unwrap(),

      NaiveDate::from_ymd_opt(2016, 2, 29).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 1).unwrap(),

  ];

  

  let mut count = 0;

  for (idx, d) in NaiveDate::from_ymd_opt(2016, 2, 27).unwrap().iter_days().take(4).enumerate() {

      assert_eq!(d, expected[idx]);

      count += 1;

  }

  assert_eq!(count, 4);

  

  for d in NaiveDate::from_ymd_opt(2016, 3, 1).unwrap().iter_days().rev().take(4) {

      count -= 1;

      assert_eq!(d, expected[count]);

  }

  ```

- <span id="naivedate-iter-weeks"></span>`const fn iter_weeks(&self) -> NaiveDateWeeksIterator` — [`NaiveDateWeeksIterator`](date/index.md#naivedateweeksiterator)

  Returns an iterator that steps by weeks across all representable dates.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let expected = [

      NaiveDate::from_ymd_opt(2016, 2, 27).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 5).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 12).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 19).unwrap(),

  ];

  

  let mut count = 0;

  for (idx, d) in NaiveDate::from_ymd_opt(2016, 2, 27).unwrap().iter_weeks().take(4).enumerate() {

      assert_eq!(d, expected[idx]);

      count += 1;

  }

  assert_eq!(count, 4);

  

  for d in NaiveDate::from_ymd_opt(2016, 3, 19).unwrap().iter_weeks().rev().take(4) {

      count -= 1;

      assert_eq!(d, expected[count]);

  }

  ```

- <span id="naivedate-week"></span>`const fn week(&self, start: Weekday) -> NaiveWeek` — [`Weekday`](../weekday/index.md#weekday), [`NaiveWeek`](#naiveweek)

  Returns the [`NaiveWeek`](#naiveweek) that the date belongs to, starting with the [`Weekday`](../weekday/index.md)

  specified.

- <span id="naivedate-leap-year"></span>`const fn leap_year(&self) -> bool`

  Returns `true` if this is a leap year.

  

  ```rust

  use chrono::NaiveDate;

  assert_eq!(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap().leap_year(), true);

  assert_eq!(NaiveDate::from_ymd_opt(2001, 1, 1).unwrap().leap_year(), false);

  assert_eq!(NaiveDate::from_ymd_opt(2002, 1, 1).unwrap().leap_year(), false);

  assert_eq!(NaiveDate::from_ymd_opt(2003, 1, 1).unwrap().leap_year(), false);

  assert_eq!(NaiveDate::from_ymd_opt(2004, 1, 1).unwrap().leap_year(), true);

  assert_eq!(NaiveDate::from_ymd_opt(2100, 1, 1).unwrap().leap_year(), false);

  ```

- <span id="naivedate-year"></span>`const fn year(&self) -> i32`

- <span id="naivedate-ordinal"></span>`const fn ordinal(&self) -> u32`

  Returns the day of year starting from 1.

- <span id="naivedate-month"></span>`const fn month(&self) -> u32`

- <span id="naivedate-day"></span>`const fn day(&self) -> u32`

- <span id="naivedate-weekday"></span>`const fn weekday(&self) -> Weekday` — [`Weekday`](../weekday/index.md#weekday)

  Returns the day of week.

- <span id="naivedate-year-flags"></span>`const fn year_flags(&self) -> YearFlags` — [`YearFlags`](internals/index.md#yearflags)

- <span id="naivedate-num-days-from-ce"></span>`const fn num_days_from_ce(&self) -> i32`

  Counts the days in the proleptic Gregorian calendar, with January 1, Year 1 (CE) as day 1.

- <span id="naivedate-to-epoch-days"></span>`const fn to_epoch_days(&self) -> i32`

  Counts the days in the proleptic Gregorian calendar, with January 1, Year 1970 as day 0.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  assert_eq!(from_ymd(1, 1, 1).to_epoch_days(), -719162);

  assert_eq!(from_ymd(1970, 1, 1).to_epoch_days(), 0);

  assert_eq!(from_ymd(2005, 9, 10).to_epoch_days(), 13036);

  ```

- <span id="naivedate-from-yof"></span>`const fn from_yof(yof: i32) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Create a new `NaiveDate` from a raw year-ordinal-flags `i32`.

  

  In a valid value an ordinal is never `0`, and neither are the year flags. This method

  doesn't do any validation in release builds.

- <span id="naivedate-yof"></span>`const fn yof(&self) -> i32`

  Get the raw year-ordinal-flags `i32`.

- <span id="naivedate-const-min"></span>`const MIN: NaiveDate`

- <span id="naivedate-const-max"></span>`const MAX: NaiveDate`

- <span id="naivedate-const-before-min"></span>`const BEFORE_MIN: NaiveDate`

- <span id="naivedate-const-after-max"></span>`const AFTER_MAX: NaiveDate`

#### Trait Implementations

##### `impl Add for NaiveDate`

- <span id="naivedate-add-type-output"></span>`type Output = NaiveDate`

- <span id="naivedate-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveDate` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveDate`](date/index.md#naivedate)

##### `impl AddAssign for NaiveDate`

- <span id="naivedate-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl Clone for NaiveDate`

- <span id="naivedate-clone"></span>`fn clone(&self) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

##### `impl Copy for NaiveDate`

##### `impl Datelike for NaiveDate`

- <span id="naivedate-datelike-year"></span>`fn year(&self) -> i32`

  Returns the year number in the [calendar date](#calendar-date).

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().year(), 2015);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().year(), -308); // 309 BCE

  ```

- <span id="naivedate-datelike-month"></span>`fn month(&self) -> u32`

  Returns the month number starting from 1.

  

  The return value ranges from 1 to 12.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().month(), 9);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().month(), 3);

  ```

- <span id="naivedate-datelike-month0"></span>`fn month0(&self) -> u32`

  Returns the month number starting from 0.

  

  The return value ranges from 0 to 11.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().month0(), 8);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().month0(), 2);

  ```

- <span id="naivedate-datelike-day"></span>`fn day(&self) -> u32`

  Returns the day of month starting from 1.

  

  The return value ranges from 1 to 31. (The last day of month differs by months.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().day(), 8);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().day(), 14);

  ```

  

  Combined with [`NaiveDate::pred_opt`](#method.pred_opt),

  one can determine the number of days in a particular month.

  (Note that this panics when `year` is out of range.)

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  fn ndays_in_month(year: i32, month: u32) -> u32 {

      // the first day of the next month...

      let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };

      let d = NaiveDate::from_ymd_opt(y, m, 1).unwrap();

  

      // ...is preceded by the last day of the original month

      d.pred_opt().unwrap().day()

  }

  

  assert_eq!(ndays_in_month(2015, 8), 31);

  assert_eq!(ndays_in_month(2015, 9), 30);

  assert_eq!(ndays_in_month(2015, 12), 31);

  assert_eq!(ndays_in_month(2016, 2), 29);

  assert_eq!(ndays_in_month(2017, 2), 28);

  ```

- <span id="naivedate-datelike-day0"></span>`fn day0(&self) -> u32`

  Returns the day of month starting from 0.

  

  The return value ranges from 0 to 30. (The last day of month differs by months.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().day0(), 7);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().day0(), 13);

  ```

- <span id="naivedate-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

  Returns the day of year starting from 1.

  

  The return value ranges from 1 to 366. (The last day of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().ordinal(), 251);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().ordinal(), 74);

  ```

  

  Combined with [`NaiveDate::pred_opt`](#method.pred_opt),

  one can determine the number of days in a particular year.

  (Note that this panics when `year` is out of range.)

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  fn ndays_in_year(year: i32) -> u32 {

      // the first day of the next year...

      let d = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();

  

      // ...is preceded by the last day of the original year

      d.pred_opt().unwrap().ordinal()

  }

  

  assert_eq!(ndays_in_year(2015), 365);

  assert_eq!(ndays_in_year(2016), 366);

  assert_eq!(ndays_in_year(2017), 365);

  assert_eq!(ndays_in_year(2000), 366);

  assert_eq!(ndays_in_year(2100), 365);

  ```

- <span id="naivedate-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

  Returns the day of year starting from 0.

  

  The return value ranges from 0 to 365. (The last day of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().ordinal0(), 250);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().ordinal0(), 73);

  ```

- <span id="naivedate-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](../weekday/index.md#weekday)

  Returns the day of week.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().weekday(), Weekday::Tue);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().weekday(), Weekday::Fri);

  ```

- <span id="naivedate-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](isoweek/index.md#isoweek)

- <span id="naivedate-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the year number changed, while keeping the same month and day.

  

  This method assumes you want to work on the date as a year-month-day value. Don't use it if

  you want the ordinal to stay the same after changing the year, of if you want the week and

  weekday values to stay the same.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (February 29 in a non-leap year).

  - The year is out of range for a `NaiveDate`.

  

  # Examples

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_year(2016),

      Some(NaiveDate::from_ymd_opt(2016, 9, 8).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_year(-308),

      Some(NaiveDate::from_ymd_opt(-308, 9, 8).unwrap())

  );

  ```

  

  A leap day (February 29) is a case where this method can return `None`.

  

  ```rust

  use chrono::{NaiveDate, Datelike};

  assert!(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().with_year(2015).is_none());

  assert!(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().with_year(2020).is_some());

  ```

  

  Don't use `with_year` if you want the ordinal date to stay the same:

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  assert_ne!(

      NaiveDate::from_yo_opt(2020, 100).unwrap().with_year(2023).unwrap(),

      NaiveDate::from_yo_opt(2023, 100).unwrap() // result is 2023-101

  );

  ```

- <span id="naivedate-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the month number (starting from 1) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month(4)` when day of the month is 31).

  - The value for `month` is invalid.

  

  # Examples

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month(10),

      Some(NaiveDate::from_ymd_opt(2015, 10, 8).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month(13), None); // No month 13

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().with_month(2), None); // No Feb 30

  ```

  

  Don't combine multiple `Datelike::with_*` methods. The intermediate value may not exist.

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  fn with_year_month(date: NaiveDate, year: i32, month: u32) -> Option<NaiveDate> {

      date.with_year(year)?.with_month(month)

  }

  let d = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();

  assert!(with_year_month(d, 2019, 1).is_none()); // fails because of invalid intermediate value

  

  // Correct version:

  fn with_year_month_fixed(date: NaiveDate, year: i32, month: u32) -> Option<NaiveDate> {

      NaiveDate::from_ymd_opt(year, month, date.day())

  }

  let d = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();

  assert_eq!(with_year_month_fixed(d, 2019, 1), NaiveDate::from_ymd_opt(2019, 1, 29));

  ```

- <span id="naivedate-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the month number (starting from 0) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month0(3)` when day of the month is 31).

  - The value for `month0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month0(9),

      Some(NaiveDate::from_ymd_opt(2015, 10, 8).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month0(12), None); // No month 12

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().with_month0(1), None); // No Feb 30

  ```

- <span id="naivedate-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of month (starting from 1) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(31)` in April).

  - The value for `day` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day(30),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day(31), None);

  // no September 31

  ```

- <span id="naivedate-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of month (starting from 0) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(30)` in April).

  - The value for `day0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day0(29),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day0(30), None);

  // no September 31

  ```

- <span id="naivedate-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of year (starting from 1) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal(366)` in a non-leap year).

  - The value for `ordinal` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Datelike};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal(60),

             Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal(366),

             None); // 2015 had only 365 days

  

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal(60),

             Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal(366),

             Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap()));

  ```

- <span id="naivedate-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of year (starting from 0) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal0(365)` in a non-leap year).

  - The value for `ordinal0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Datelike};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal0(59),

             Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal0(365),

             None); // 2015 had only 365 days

  

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal0(59),

             Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal0(365),

             Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap()));

  ```

##### `impl Debug for NaiveDate`

- <span id="naivedate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NaiveDate`

- <span id="naivedate-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::NaiveDate`

- <span id="supernaivedate-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for NaiveDate`

##### `impl Display for NaiveDate`

- <span id="naivedate-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NaiveDate`

##### `impl FromStr for NaiveDate`

- <span id="naivedate-fromstr-type-err"></span>`type Err = ParseError`

- <span id="naivedate-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveDate>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveDate`](date/index.md#naivedate)

##### `impl Hash for NaiveDate`

- <span id="naivedate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveDate`

- <span id="naivedate-ord-cmp"></span>`fn cmp(&self, other: &NaiveDate) -> cmp::Ordering` — [`NaiveDate`](date/index.md#naivedate)

##### `impl PartialEq for NaiveDate`

- <span id="naivedate-partialeq-eq"></span>`fn eq(&self, other: &NaiveDate) -> bool` — [`NaiveDate`](date/index.md#naivedate)

##### `impl PartialOrd for NaiveDate`

- <span id="naivedate-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDate) -> option::Option<cmp::Ordering>` — [`NaiveDate`](date/index.md#naivedate)

##### `impl Serialize for super::NaiveDate`

- <span id="supernaivedate-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveDate`

##### `impl Sub for NaiveDate`

- <span id="naivedate-sub-type-output"></span>`type Output = NaiveDate`

- <span id="naivedate-sub"></span>`fn sub(self, months: Months) -> <Self as >::Output` — [`Months`](../month/index.md#months)

##### `impl SubAssign for NaiveDate`

- <span id="naivedate-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl ToString for NaiveDate`

- <span id="naivedate-tostring-to-string"></span>`fn to_string(&self) -> String`

### `NaiveDateDaysIterator`

```rust
struct NaiveDateDaysIterator {
    value: NaiveDate,
}
```

Iterator over `NaiveDate` with a step size of one day.

#### Trait Implementations

##### `impl Clone for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-clone"></span>`fn clone(&self) -> NaiveDateDaysIterator` — [`NaiveDateDaysIterator`](date/index.md#naivedatedaysiterator)

##### `impl Copy for NaiveDateDaysIterator`

##### `impl Debug for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl Eq for NaiveDateDaysIterator`

##### `impl ExactSizeIterator for NaiveDateDaysIterator`

##### `impl FusedIterator for NaiveDateDaysIterator`

##### `impl Hash for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="naivedatedaysiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="naivedatedaysiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-iterator-type-item"></span>`type Item = NaiveDate`

- <span id="naivedatedaysiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="naivedatedaysiterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Ord for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-ord-cmp"></span>`fn cmp(&self, other: &NaiveDateDaysIterator) -> cmp::Ordering` — [`NaiveDateDaysIterator`](date/index.md#naivedatedaysiterator)

##### `impl PartialEq for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-partialeq-eq"></span>`fn eq(&self, other: &NaiveDateDaysIterator) -> bool` — [`NaiveDateDaysIterator`](date/index.md#naivedatedaysiterator)

##### `impl PartialOrd for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDateDaysIterator) -> option::Option<cmp::Ordering>` — [`NaiveDateDaysIterator`](date/index.md#naivedatedaysiterator)

##### `impl StructuralPartialEq for NaiveDateDaysIterator`

### `NaiveDateWeeksIterator`

```rust
struct NaiveDateWeeksIterator {
    value: NaiveDate,
}
```

Iterator over `NaiveDate` with a step size of one week.

#### Trait Implementations

##### `impl Clone for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-clone"></span>`fn clone(&self) -> NaiveDateWeeksIterator` — [`NaiveDateWeeksIterator`](date/index.md#naivedateweeksiterator)

##### `impl Copy for NaiveDateWeeksIterator`

##### `impl Debug for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl Eq for NaiveDateWeeksIterator`

##### `impl ExactSizeIterator for NaiveDateWeeksIterator`

##### `impl FusedIterator for NaiveDateWeeksIterator`

##### `impl Hash for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="naivedateweeksiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="naivedateweeksiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-iterator-type-item"></span>`type Item = NaiveDate`

- <span id="naivedateweeksiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="naivedateweeksiterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Ord for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-ord-cmp"></span>`fn cmp(&self, other: &NaiveDateWeeksIterator) -> cmp::Ordering` — [`NaiveDateWeeksIterator`](date/index.md#naivedateweeksiterator)

##### `impl PartialEq for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-partialeq-eq"></span>`fn eq(&self, other: &NaiveDateWeeksIterator) -> bool` — [`NaiveDateWeeksIterator`](date/index.md#naivedateweeksiterator)

##### `impl PartialOrd for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDateWeeksIterator) -> option::Option<cmp::Ordering>` — [`NaiveDateWeeksIterator`](date/index.md#naivedateweeksiterator)

##### `impl StructuralPartialEq for NaiveDateWeeksIterator`

### `NaiveDateTime`

```rust
struct NaiveDateTime {
    date: crate::naive::NaiveDate,
    time: crate::naive::NaiveTime,
}
```

ISO 8601 combined date and time without timezone.

# Example

`NaiveDateTime` is commonly created from [`NaiveDate`](date/index.md).

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

- <span id="naivedatetime-new"></span>`const fn new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime` — [`NaiveDate`](date/index.md#naivedate), [`NaiveTime`](time/index.md#naivetime), [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-from-timestamp"></span>`const fn from_timestamp(secs: i64, nsecs: u32) -> NaiveDateTime` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-from-timestamp-millis"></span>`const fn from_timestamp_millis(millis: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Creates a new [NaiveDateTime] from milliseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of milliseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-micros"></span>`const fn from_timestamp_micros(micros: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Creates a new [NaiveDateTime] from microseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of microseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-nanos"></span>`const fn from_timestamp_nanos(nanos: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Creates a new [NaiveDateTime] from nanoseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of nanoseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-opt"></span>`const fn from_timestamp_opt(secs: i64, nsecs: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDateTime>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDateTime, &'a str)>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-date"></span>`const fn date(&self) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Retrieves a date component.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();

  assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2016, 7, 8).unwrap());

  ```

- <span id="naivedatetime-time"></span>`const fn time(&self) -> NaiveTime` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivedatetime-checked-add-signed"></span>`const fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-checked-add-months"></span>`const fn checked_add_months(self, rhs: Months) -> Option<NaiveDateTime>` — [`Months`](../month/index.md#months), [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-checked-add-offset"></span>`const fn checked_add_offset(self, rhs: FixedOffset) -> Option<NaiveDateTime>` — [`FixedOffset`](../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Adds given `FixedOffset` to the current datetime.

  Returns `None` if the result would be outside the valid range for [`NaiveDateTime`](datetime/index.md).

  

  This method is similar to [`checked_add_signed`](#method.checked_add_offset), but preserves

  leap seconds.

- <span id="naivedatetime-checked-sub-offset"></span>`const fn checked_sub_offset(self, rhs: FixedOffset) -> Option<NaiveDateTime>` — [`FixedOffset`](../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Subtracts given `FixedOffset` from the current datetime.

  Returns `None` if the result would be outside the valid range for [`NaiveDateTime`](datetime/index.md).

  

  This method is similar to [`checked_sub_signed`](#method.checked_sub_signed), but preserves

  leap seconds.

- <span id="naivedatetime-overflowing-add-offset"></span>`fn overflowing_add_offset(self, rhs: FixedOffset) -> NaiveDateTime` — [`FixedOffset`](../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Adds given `FixedOffset` to the current datetime.

  The resulting value may be outside the valid range of [`NaiveDateTime`](datetime/index.md).

  

  This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`

  should not be exposed to library users.

- <span id="naivedatetime-overflowing-sub-offset"></span>`fn overflowing_sub_offset(self, rhs: FixedOffset) -> NaiveDateTime` — [`FixedOffset`](../offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](datetime/index.md#naivedatetime)

  Subtracts given `FixedOffset` from the current datetime.

  The resulting value may be outside the valid range of [`NaiveDateTime`](datetime/index.md).

  

  This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`

  should not be exposed to library users.

- <span id="naivedatetime-checked-sub-signed"></span>`const fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-checked-sub-months"></span>`const fn checked_sub_months(self, rhs: Months) -> Option<NaiveDateTime>` — [`Months`](../month/index.md#months), [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-checked-add-days"></span>`const fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](#days)

  Add a duration in [`Days`](#days) to the date part of the `NaiveDateTime`

  

  Returns `None` if the resulting date would be out of range.

- <span id="naivedatetime-checked-sub-days"></span>`const fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](#days)

  Subtract a duration in [`Days`](#days) from the date part of the `NaiveDateTime`

  

  Returns `None` if the resulting date would be out of range.

- <span id="naivedatetime-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: NaiveDateTime) -> TimeDelta` — [`NaiveDateTime`](datetime/index.md#naivedatetime), [`TimeDelta`](../time_delta/index.md#timedelta)

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

- <span id="naivedatetime-and-local-timezone"></span>`fn and_local_timezone<Tz: TimeZone>(&self, tz: Tz) -> MappedLocalTime<DateTime<Tz>>` — [`MappedLocalTime`](../offset/index.md#mappedlocaltime), [`DateTime`](../datetime/index.md#datetime)

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

- <span id="naivedatetime-and-utc"></span>`const fn and_utc(&self) -> DateTime<Utc>` — [`DateTime`](../datetime/index.md#datetime), [`Utc`](../offset/utc/index.md#utc)

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

- <span id="naivedatetime-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveDateTime` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveDateTime`](datetime/index.md#naivedatetime)

##### `impl AddAssign for NaiveDateTime`

- <span id="naivedatetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl Clone for NaiveDateTime`

- <span id="naivedatetime-clone"></span>`fn clone(&self) -> NaiveDateTime` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](../weekday/index.md#weekday)

  Returns the day of week.

  

  See also the [`NaiveDate::weekday`](./struct.NaiveDate.html#method.weekday) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime, Weekday};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.weekday(), Weekday::Fri);

  ```

- <span id="naivedatetime-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](isoweek/index.md#isoweek)

- <span id="naivedatetime-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="cratenaivedatetime-durationround-duration-round"></span>`fn duration_round(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DurationRound`](../round/index.md#durationround)

- <span id="cratenaivedatetime-durationround-duration-trunc"></span>`fn duration_trunc(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DurationRound`](../round/index.md#durationround)

- <span id="cratenaivedatetime-durationround-duration-round-up"></span>`fn duration_round_up(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`DurationRound`](../round/index.md#durationround)

##### `impl Eq for NaiveDateTime`

##### `impl FromStr for NaiveDateTime`

- <span id="naivedatetime-fromstr-type-err"></span>`type Err = ParseError`

- <span id="naivedatetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveDateTime>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveDateTime`](datetime/index.md#naivedatetime)

##### `impl Hash for NaiveDateTime`

- <span id="naivedatetime-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveDateTime`

- <span id="naivedatetime-ord-cmp"></span>`fn cmp(&self, other: &NaiveDateTime) -> cmp::Ordering` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

##### `impl PartialEq for NaiveDateTime`

- <span id="naivedatetime-partialeq-eq"></span>`fn eq(&self, other: &NaiveDateTime) -> bool` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

##### `impl PartialOrd for NaiveDateTime`

- <span id="naivedatetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDateTime) -> option::Option<cmp::Ordering>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

##### `impl Serialize for super::NaiveDateTime`

- <span id="supernaivedatetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveDateTime`

##### `impl Sub for NaiveDateTime`

- <span id="naivedatetime-sub-type-output"></span>`type Output = NaiveDateTime`

- <span id="naivedatetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> NaiveDateTime` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveDateTime`](datetime/index.md#naivedatetime)

##### `impl SubAssign for NaiveDateTime`

- <span id="naivedatetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

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

  

  See also the [`NaiveTime#method.nanosecond`](time/index.md) method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.nanosecond(), 789_000_000);

  ```

- <span id="naivedatetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

- <span id="naivedatetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](datetime/index.md#naivedatetime)

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

### `IsoWeek`

```rust
struct IsoWeek {
    ywf: i32,
}
```

ISO 8601 week.

This type, combined with [`Weekday`](../enum.Weekday.html),
constitutes the ISO 8601 [week date](./struct.NaiveDate.html#week-date).
One can retrieve this type from the existing [`Datelike`](../trait.Datelike.html) types
via the [`Datelike::iso_week`](../trait.Datelike.html#tymethod.iso_week) method.

#### Implementations

- <span id="isoweek-from-yof"></span>`fn from_yof(year: i32, ordinal: u32, year_flags: YearFlags) -> Self` — [`YearFlags`](internals/index.md#yearflags)

  Returns the corresponding `IsoWeek` from the year and the `Of` internal value.

- <span id="isoweek-year"></span>`const fn year(&self) -> i32`

  Returns the year number for this ISO week.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  let d = NaiveDate::from_isoywd_opt(2015, 1, Weekday::Mon).unwrap();

  assert_eq!(d.iso_week().year(), 2015);

  ```

  

  This year number might not match the calendar year number.

  Continuing the example...

  

  ```rust

  use chrono::{NaiveDate, Datelike, Weekday};

  let d = NaiveDate::from_isoywd_opt(2015, 1, Weekday::Mon).unwrap();

  assert_eq!(d.year(), 2014);

  assert_eq!(d, NaiveDate::from_ymd_opt(2014, 12, 29).unwrap());

  ```

- <span id="isoweek-week"></span>`const fn week(&self) -> u32`

  Returns the ISO week number starting from 1.

  

  The return value ranges from 1 to 53. (The last week of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  let d = NaiveDate::from_isoywd_opt(2015, 15, Weekday::Mon).unwrap();

  assert_eq!(d.iso_week().week(), 15);

  ```

- <span id="isoweek-week0"></span>`const fn week0(&self) -> u32`

  Returns the ISO week number starting from 0.

  

  The return value ranges from 0 to 52. (The last week of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  let d = NaiveDate::from_isoywd_opt(2015, 15, Weekday::Mon).unwrap();

  assert_eq!(d.iso_week().week0(), 14);

  ```

#### Trait Implementations

##### `impl Clone for IsoWeek`

- <span id="isoweek-clone"></span>`fn clone(&self) -> IsoWeek` — [`IsoWeek`](isoweek/index.md#isoweek)

##### `impl Copy for IsoWeek`

##### `impl Debug for IsoWeek`

- <span id="isoweek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IsoWeek`

##### `impl Hash for IsoWeek`

- <span id="isoweek-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for IsoWeek`

- <span id="isoweek-ord-cmp"></span>`fn cmp(&self, other: &IsoWeek) -> cmp::Ordering` — [`IsoWeek`](isoweek/index.md#isoweek)

##### `impl PartialEq for IsoWeek`

- <span id="isoweek-partialeq-eq"></span>`fn eq(&self, other: &IsoWeek) -> bool` — [`IsoWeek`](isoweek/index.md#isoweek)

##### `impl PartialOrd for IsoWeek`

- <span id="isoweek-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &IsoWeek) -> option::Option<cmp::Ordering>` — [`IsoWeek`](isoweek/index.md#isoweek)

##### `impl StructuralPartialEq for IsoWeek`

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

- <span id="naivetime-from-hms"></span>`const fn from_hms(hour: u32, min: u32, sec: u32) -> NaiveTime` — [`NaiveTime`](time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute and second.

  

  No [leap second](#leap-second-handling) is allowed here;

  use `NaiveTime::from_hms_*` methods with a subsecond parameter instead.

  

  # Panics

  

  Panics on invalid hour, minute and/or second.

- <span id="naivetime-from-hms-opt"></span>`const fn from_hms_opt(hour: u32, min: u32, sec: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-from-hms-milli"></span>`const fn from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> NaiveTime` — [`NaiveTime`](time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and millisecond.

  

  The millisecond part can exceed 1,000

  in order to represent the [leap second](#leap-second-handling).

  

  # Panics

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="naivetime-from-hms-milli-opt"></span>`const fn from_hms_milli_opt(hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-from-hms-micro"></span>`const fn from_hms_micro(hour: u32, min: u32, sec: u32, micro: u32) -> NaiveTime` — [`NaiveTime`](time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or microsecond.

- <span id="naivetime-from-hms-micro-opt"></span>`const fn from_hms_micro_opt(hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-from-hms-nano"></span>`const fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> NaiveTime` — [`NaiveTime`](time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="naivetime-from-hms-nano-opt"></span>`const fn from_hms_nano_opt(hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-from-num-seconds-from-midnight"></span>`const fn from_num_seconds_from_midnight(secs: u32, nano: u32) -> NaiveTime` — [`NaiveTime`](time/index.md#naivetime)

  Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `secs % 60 == 59`.

  

  # Panics

  

  Panics on invalid number of seconds and/or nanosecond.

- <span id="naivetime-from-num-seconds-from-midnight-opt"></span>`const fn from_num_seconds_from_midnight_opt(secs: u32, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveTime>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveTime, &'a str)>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-overflowing-add-signed"></span>`const fn overflowing_add_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64)` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-overflowing-sub-signed"></span>`const fn overflowing_sub_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64)` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: NaiveTime) -> TimeDelta` — [`NaiveTime`](time/index.md#naivetime), [`TimeDelta`](../time_delta/index.md#timedelta)

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

- <span id="naivetime-overflowing-add-offset"></span>`const fn overflowing_add_offset(&self, offset: FixedOffset) -> (NaiveTime, i32)` — [`FixedOffset`](../offset/fixed/index.md#fixedoffset), [`NaiveTime`](time/index.md#naivetime)

  Adds given `FixedOffset` to the current time, and returns the number of days that should be

  added to a date as a result of the offset (either `-1`, `0`, or `1` because the offset is

  always less than 24h).

  

  This method is similar to [`overflowing_add_signed`](#method.overflowing_add_signed), but

  preserves leap seconds.

- <span id="naivetime-overflowing-sub-offset"></span>`const fn overflowing_sub_offset(&self, offset: FixedOffset) -> (NaiveTime, i32)` — [`FixedOffset`](../offset/fixed/index.md#fixedoffset), [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveTime` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveTime`](time/index.md#naivetime)

##### `impl AddAssign for NaiveTime`

- <span id="naivetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl Clone for NaiveTime`

- <span id="naivetime-clone"></span>`fn clone(&self) -> NaiveTime` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveTime>` — [`ParseResult`](../format/index.md#parseresult), [`NaiveTime`](time/index.md#naivetime)

##### `impl Hash for NaiveTime`

- <span id="naivetime-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveTime`

- <span id="naivetime-ord-cmp"></span>`fn cmp(&self, other: &NaiveTime) -> cmp::Ordering` — [`NaiveTime`](time/index.md#naivetime)

##### `impl PartialEq for NaiveTime`

- <span id="naivetime-partialeq-eq"></span>`fn eq(&self, other: &NaiveTime) -> bool` — [`NaiveTime`](time/index.md#naivetime)

##### `impl PartialOrd for NaiveTime`

- <span id="naivetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveTime) -> option::Option<cmp::Ordering>` — [`NaiveTime`](time/index.md#naivetime)

##### `impl Serialize for super::NaiveTime`

- <span id="supernaivetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveTime`

##### `impl Sub for NaiveTime`

- <span id="naivetime-sub-type-output"></span>`type Output = NaiveTime`

- <span id="naivetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> NaiveTime` — [`TimeDelta`](../time_delta/index.md#timedelta), [`NaiveTime`](time/index.md#naivetime)

##### `impl SubAssign for NaiveTime`

- <span id="naivetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

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

- <span id="naivetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

- <span id="naivetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](time/index.md#naivetime)

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

### `NaiveWeek`

```rust
struct NaiveWeek {
    date: NaiveDate,
    start: crate::Weekday,
}
```

A week represented by a [`NaiveDate`](date/index.md) and a [`Weekday`](../weekday/index.md) which is the first
day of the week.

#### Implementations

- <span id="naiveweek-new"></span>`const fn new(date: NaiveDate, start: Weekday) -> Self` — [`NaiveDate`](date/index.md#naivedate), [`Weekday`](../weekday/index.md#weekday)

  Create a new `NaiveWeek`

- <span id="naiveweek-first-day"></span>`const fn first_day(&self) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Returns a date representing the first day of the week.

  

  # Panics

  

  Panics if the first day of the week happens to fall just out of range of `NaiveDate`

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();

  let week = date.week(Weekday::Mon);

  assert!(week.first_day() <= date);

  ```

- <span id="naiveweek-checked-first-day"></span>`const fn checked_first_day(&self) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Returns a date representing the first day of the week or

  `None` if the date is out of `NaiveDate`'s range

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::MIN;

  let week = date.week(Weekday::Mon);

  if let Some(first_day) = week.checked_first_day() {

      assert!(first_day == date);

  } else {

      // error handling code

      return;

  };

  ```

- <span id="naiveweek-last-day"></span>`const fn last_day(&self) -> NaiveDate` — [`NaiveDate`](date/index.md#naivedate)

  Returns a date representing the last day of the week.

  

  # Panics

  

  Panics if the last day of the week happens to fall just out of range of `NaiveDate`

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();

  let week = date.week(Weekday::Mon);

  assert!(week.last_day() >= date);

  ```

- <span id="naiveweek-checked-last-day"></span>`const fn checked_last_day(&self) -> Option<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Returns a date representing the last day of the week or

  `None` if the date is out of `NaiveDate`'s range

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::MAX;

  let week = date.week(Weekday::Mon);

  if let Some(last_day) = week.checked_last_day() {

      assert!(last_day == date);

  } else {

      // error handling code

      return;

  };

  ```

- <span id="naiveweek-days"></span>`const fn days(&self) -> RangeInclusive<NaiveDate>` — [`NaiveDate`](date/index.md#naivedate)

  Returns a [`RangeInclusive<T>`](../../num_iter/index.md) representing the whole week bounded by

  [first_day](NaiveWeek::first_day) and [last_day](NaiveWeek::last_day) functions.

  

  # Panics

  

  Panics if the either the first or last day of the week happens to fall just out of range of

  `NaiveDate` (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();

  let week = date.week(Weekday::Mon);

  let days = week.days();

  assert!(days.contains(&date));

  ```

- <span id="naiveweek-checked-days"></span>`const fn checked_days(&self) -> Option<RangeInclusive<NaiveDate>>` — [`NaiveDate`](date/index.md#naivedate)

  Returns an [`Option<RangeInclusive<T>>`](../../serde_core/index.md) representing the whole week bounded by

  [checked_first_day](NaiveWeek::checked_first_day) and

  [checked_last_day](NaiveWeek::checked_last_day) functions.

  

  Returns `None` if either of the boundaries are out of `NaiveDate`'s range

  (more than ca. 262,000 years away from common era).

  

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::MAX;

  let week = date.week(Weekday::Mon);

  let _days = match week.checked_days() {

      Some(d) => d,

      None => {

          // error handling code

          return;

      }

  };

  ```

#### Trait Implementations

##### `impl Clone for NaiveWeek`

- <span id="naiveweek-clone"></span>`fn clone(&self) -> NaiveWeek` — [`NaiveWeek`](#naiveweek)

##### `impl Copy for NaiveWeek`

##### `impl Debug for NaiveWeek`

- <span id="naiveweek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NaiveWeek`

##### `impl Hash for NaiveWeek`

- <span id="naiveweek-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for NaiveWeek`

- <span id="naiveweek-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `Days`

```rust
struct Days(u64);
```

A duration in calendar days.

This is useful because when using `TimeDelta` it is possible that adding `TimeDelta::days(1)`
doesn't increment the day value as expected due to it being a fixed number of seconds. This
difference applies only when dealing with `DateTime<TimeZone>` data types and in other cases
`TimeDelta::days(n)` and `Days::new(n)` are equivalent.

#### Implementations

- <span id="days-new"></span>`const fn new(num: u64) -> Self`

  Construct a new `Days` from a number of days

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, days: Days) -> <Self as >::Output` — [`Days`](#days)

##### `impl Clone for Days`

- <span id="days-clone"></span>`fn clone(&self) -> Days` — [`Days`](#days)

##### `impl Copy for Days`

##### `impl Debug for Days`

- <span id="days-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Days`

##### `impl Hash for Days`

- <span id="days-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Days`

- <span id="days-ord-cmp"></span>`fn cmp(&self, other: &Days) -> cmp::Ordering` — [`Days`](#days)

##### `impl PartialEq for Days`

- <span id="days-partialeq-eq"></span>`fn eq(&self, other: &Days) -> bool` — [`Days`](#days)

##### `impl PartialOrd for Days`

- <span id="days-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Days) -> option::Option<cmp::Ordering>` — [`Days`](#days)

##### `impl StructuralPartialEq for Days`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, days: Days) -> <Self as >::Output` — [`Days`](#days)

## Constants

### `MAX_DATE`
```rust
const MAX_DATE: NaiveDate;
```

The maximum possible `NaiveDate` (December 31, 262143 CE).

### `MIN_DATE`
```rust
const MIN_DATE: NaiveDate;
```

The minimum possible `NaiveDate` (January 1, 262145 BCE).

### `MAX_DATETIME`
```rust
const MAX_DATETIME: NaiveDateTime;
```

The maximum possible `NaiveDateTime`.

### `MIN_DATETIME`
```rust
const MIN_DATETIME: NaiveDateTime;
```

The minimum possible `NaiveDateTime`.

