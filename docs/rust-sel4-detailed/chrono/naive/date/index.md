*[chrono](../../index.md) / [naive](../index.md) / [date](index.md)*

---

# Module `date`

ISO 8601 calendar date without timezone.

The implementation is optimized for determining year, month, day and day of week.

Format of `NaiveDate`:
`YYYY_YYYY_YYYY_YYYY_YYYO_OOOO_OOOO_LWWW`
`Y`: Year
`O`: Ordinal
`L`: leap year flag (1 = common year, 0 is leap year)
`W`: weekday before the first day of the year
`LWWW`: will also be referred to as the year flags (`F`)

## Contents

- [Modules](#modules)
  - [`serde`](#serde)
- [Structs](#structs)
  - [`NaiveDate`](#naivedate)
  - [`NaiveDateDaysIterator`](#naivedatedaysiterator)
  - [`NaiveDateWeeksIterator`](#naivedateweeksiterator)
- [Functions](#functions)
  - [`cycle_to_yo`](#cycle-to-yo)
  - [`yo_to_cycle`](#yo-to-cycle)
  - [`div_mod_floor`](#div-mod-floor)
- [Constants](#constants)
  - [`MIN_DATE`](#min-date)
  - [`MAX_DATE`](#max-date)
  - [`MAX_YEAR`](#max-year)
  - [`MIN_YEAR`](#min-year)
  - [`ORDINAL_MASK`](#ordinal-mask)
  - [`LEAP_YEAR_MASK`](#leap-year-mask)
  - [`OL_MASK`](#ol-mask)
  - [`MAX_OL`](#max-ol)
  - [`WEEKDAY_FLAGS_MASK`](#weekday-flags-mask)
  - [`YEAR_FLAGS_MASK`](#year-flags-mask)
  - [`YEAR_DELTAS`](#year-deltas)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serde`](#serde) | mod |  |
| [`NaiveDate`](#naivedate) | struct | ISO 8601 calendar date without timezone. |
| [`NaiveDateDaysIterator`](#naivedatedaysiterator) | struct | Iterator over `NaiveDate` with a step size of one day. |
| [`NaiveDateWeeksIterator`](#naivedateweeksiterator) | struct | Iterator over `NaiveDate` with a step size of one week. |
| [`cycle_to_yo`](#cycle-to-yo) | fn |  |
| [`yo_to_cycle`](#yo-to-cycle) | fn |  |
| [`div_mod_floor`](#div-mod-floor) | fn |  |
| [`MIN_DATE`](#min-date) | const | The minimum possible `NaiveDate` (January 1, 262145 BCE). |
| [`MAX_DATE`](#max-date) | const | The maximum possible `NaiveDate` (December 31, 262143 CE). |
| [`MAX_YEAR`](#max-year) | const | MAX_YEAR is one year less than the type is capable of representing. |
| [`MIN_YEAR`](#min-year) | const | MIN_YEAR is one year more than the type is capable of representing. |
| [`ORDINAL_MASK`](#ordinal-mask) | const |  |
| [`LEAP_YEAR_MASK`](#leap-year-mask) | const |  |
| [`OL_MASK`](#ol-mask) | const |  |
| [`MAX_OL`](#max-ol) | const |  |
| [`WEEKDAY_FLAGS_MASK`](#weekday-flags-mask) | const |  |
| [`YEAR_FLAGS_MASK`](#year-flags-mask) | const |  |
| [`YEAR_DELTAS`](#year-deltas) | const |  |

## Modules

- [`serde`](serde/index.md)

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

- <span id="naivedate-weeks-from"></span>`fn weeks_from(&self, day: Weekday) -> i32` — [`Weekday`](../../weekday/index.md#weekday)

- <span id="naivedate-from-ordinal-and-flags"></span>`const fn from_ordinal_and_flags(year: i32, ordinal: u32, flags: YearFlags) -> Option<NaiveDate>` — [`YearFlags`](../internals/index.md#yearflags), [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` from year, ordinal and flags.

  Does not check whether the flags are correct for the provided year.

- <span id="naivedate-from-mdf"></span>`const fn from_mdf(year: i32, mdf: Mdf) -> Option<NaiveDate>` — [`Mdf`](../internals/index.md#mdf), [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` from year and packed month-day-flags.

  Does not check whether the flags are correct for the provided year.

- <span id="naivedate-from-ymd"></span>`const fn from_ymd(year: i32, month: u32, day: u32) -> NaiveDate` — [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` from the [calendar date](#calendar-date)

  (year, month and day).

  

  # Panics

  

  Panics if the specified calendar day does not exist, on invalid values for `month` or `day`,

  or if `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-ymd-opt"></span>`const fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-from-yo"></span>`const fn from_yo(year: i32, ordinal: u32) -> NaiveDate` — [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)

  (year and day of the year).

  

  # Panics

  

  Panics if the specified ordinal day does not exist, on invalid values for `ordinal`, or if

  `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-yo-opt"></span>`const fn from_yo_opt(year: i32, ordinal: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-from-isoywd"></span>`const fn from_isoywd(year: i32, week: u32, weekday: Weekday) -> NaiveDate` — [`Weekday`](../../weekday/index.md#weekday), [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` from the [ISO week date](#week-date)

  (year, week number and day of the week).

  The resulting `NaiveDate` may have a different year from the input year.

  

  # Panics

  

  Panics if the specified week does not exist in that year, on invalid values for `week`, or

  if the resulting date is out of range for `NaiveDate`.

- <span id="naivedate-from-isoywd-opt"></span>`const fn from_isoywd_opt(year: i32, week: u32, weekday: Weekday) -> Option<NaiveDate>` — [`Weekday`](../../weekday/index.md#weekday), [`NaiveDate`](#naivedate)

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

- <span id="naivedate-from-num-days-from-ce"></span>`const fn from_num_days_from_ce(days: i32) -> NaiveDate` — [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with

  January 1, 1 being day 1.

  

  # Panics

  

  Panics if the date is out of range.

- <span id="naivedate-from-num-days-from-ce-opt"></span>`const fn from_num_days_from_ce_opt(days: i32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-from-epoch-days"></span>`const fn from_epoch_days(days: i32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-from-weekday-of-month"></span>`const fn from_weekday_of_month(year: i32, month: u32, weekday: Weekday, n: u8) -> NaiveDate` — [`Weekday`](../../weekday/index.md#weekday), [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week

  since the beginning of the given month. For instance, if you want the 2nd Friday of March

  2017, you would use `NaiveDate::from_weekday_of_month(2017, 3, Weekday::Fri, 2)`.

  

  `n` is 1-indexed.

  

  # Panics

  

  Panics if the specified day does not exist in that month, on invalid values for `month` or

  `n`, or if `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-weekday-of-month-opt"></span>`const fn from_weekday_of_month_opt(year: i32, month: u32, weekday: Weekday, n: u8) -> Option<NaiveDate>` — [`Weekday`](../../weekday/index.md#weekday), [`NaiveDate`](#naivedate)

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

- <span id="naivedate-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDate>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveDate`](#naivedate)

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

- <span id="naivedate-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDate, &'a str)>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveDate`](#naivedate)

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

- <span id="naivedate-checked-add-months"></span>`const fn checked_add_months(self, months: Months) -> Option<Self>` — [`Months`](../../month/index.md#months)

  Add a duration in [`Months`](../../month/index.md) to the date

  

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

- <span id="naivedate-checked-sub-months"></span>`const fn checked_sub_months(self, months: Months) -> Option<Self>` — [`Months`](../../month/index.md#months)

  Subtract a duration in [`Months`](../../month/index.md) from the date

  

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

- <span id="naivedate-checked-add-days"></span>`const fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](../index.md#days)

  Add a duration in [`Days`](../index.md) to the date

  

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

- <span id="naivedate-checked-sub-days"></span>`const fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](../index.md#days)

  Subtract a duration in [`Days`](../index.md) from the date

  

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

- <span id="naivedate-and-time"></span>`const fn and_time(&self, time: NaiveTime) -> NaiveDateTime` — [`NaiveTime`](../time/index.md#naivetime), [`NaiveDateTime`](../datetime/index.md#naivedatetime)

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

- <span id="naivedate-and-hms"></span>`const fn and_hms(&self, hour: u32, min: u32, sec: u32) -> NaiveDateTime` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute and second.

  

  No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;

  use `NaiveDate::and_hms_*` methods with a subsecond parameter instead.

  

  # Panics

  

  Panics on invalid hour, minute and/or second.

- <span id="naivedate-and-hms-opt"></span>`const fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

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

- <span id="naivedate-and-hms-milli"></span>`const fn and_hms_milli(&self, hour: u32, min: u32, sec: u32, milli: u32) -> NaiveDateTime` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.

  

  The millisecond part is allowed to exceed 1,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="naivedate-and-hms-milli-opt"></span>`const fn and_hms_milli_opt(&self, hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

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

- <span id="naivedate-and-hms-micro"></span>`const fn and_hms_micro(&self, hour: u32, min: u32, sec: u32, micro: u32) -> NaiveDateTime` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

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

- <span id="naivedate-and-hms-micro-opt"></span>`const fn and_hms_micro_opt(&self, hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

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

- <span id="naivedate-and-hms-nano"></span>`const fn and_hms_nano(&self, hour: u32, min: u32, sec: u32, nano: u32) -> NaiveDateTime` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="naivedate-and-hms-nano-opt"></span>`const fn and_hms_nano_opt(&self, hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](../datetime/index.md#naivedatetime)

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

- <span id="naivedate-mdf"></span>`const fn mdf(&self) -> Mdf` — [`Mdf`](../internals/index.md#mdf)

  Returns the packed month-day-flags.

- <span id="naivedate-with-mdf"></span>`const fn with_mdf(&self, mdf: Mdf) -> Option<NaiveDate>` — [`Mdf`](../internals/index.md#mdf), [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` with the packed month-day-flags changed.

  

  Returns `None` when the resulting `NaiveDate` would be invalid.

- <span id="naivedate-succ"></span>`const fn succ(&self) -> NaiveDate` — [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` for the next calendar date.

  

  # Panics

  

  Panics when `self` is the last representable date.

- <span id="naivedate-succ-opt"></span>`const fn succ_opt(&self) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-pred"></span>`const fn pred(&self) -> NaiveDate` — [`NaiveDate`](#naivedate)

  Makes a new `NaiveDate` for the previous calendar date.

  

  # Panics

  

  Panics when `self` is the first representable date.

- <span id="naivedate-pred-opt"></span>`const fn pred_opt(&self) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-checked-add-signed"></span>`const fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDate>` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveDate`](#naivedate)

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

- <span id="naivedate-checked-sub-signed"></span>`const fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDate>` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveDate`](#naivedate)

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

- <span id="naivedate-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: Self) -> TimeDelta` — [`TimeDelta`](../../time_delta/index.md#timedelta)

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

- <span id="naivedate-abs-diff"></span>`const fn abs_diff(self, rhs: Self) -> Days` — [`Days`](../index.md#days)

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

- <span id="naivedate-iter-days"></span>`const fn iter_days(&self) -> NaiveDateDaysIterator` — [`NaiveDateDaysIterator`](#naivedatedaysiterator)

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

- <span id="naivedate-iter-weeks"></span>`const fn iter_weeks(&self) -> NaiveDateWeeksIterator` — [`NaiveDateWeeksIterator`](#naivedateweeksiterator)

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

- <span id="naivedate-week"></span>`const fn week(&self, start: Weekday) -> NaiveWeek` — [`Weekday`](../../weekday/index.md#weekday), [`NaiveWeek`](../index.md#naiveweek)

  Returns the [`NaiveWeek`](../index.md) that the date belongs to, starting with the [`Weekday`](../../weekday/index.md)

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

- <span id="naivedate-weekday"></span>`const fn weekday(&self) -> Weekday` — [`Weekday`](../../weekday/index.md#weekday)

  Returns the day of week.

- <span id="naivedate-year-flags"></span>`const fn year_flags(&self) -> YearFlags` — [`YearFlags`](../internals/index.md#yearflags)

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

- <span id="naivedate-from-yof"></span>`const fn from_yof(yof: i32) -> NaiveDate` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveDate` — [`TimeDelta`](../../time_delta/index.md#timedelta), [`NaiveDate`](#naivedate)

##### `impl AddAssign for NaiveDate`

- <span id="naivedate-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../../time_delta/index.md#timedelta)

##### `impl Clone for NaiveDate`

- <span id="naivedate-clone"></span>`fn clone(&self) -> NaiveDate` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](../../weekday/index.md#weekday)

  Returns the day of week.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().weekday(), Weekday::Tue);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().weekday(), Weekday::Fri);

  ```

- <span id="naivedate-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](../isoweek/index.md#isoweek)

- <span id="naivedate-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDate>` — [`NaiveDate`](#naivedate)

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

- <span id="naivedate-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveDate>` — [`ParseResult`](../../format/index.md#parseresult), [`NaiveDate`](#naivedate)

##### `impl Hash for NaiveDate`

- <span id="naivedate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveDate`

- <span id="naivedate-ord-cmp"></span>`fn cmp(&self, other: &NaiveDate) -> cmp::Ordering` — [`NaiveDate`](#naivedate)

##### `impl PartialEq for NaiveDate`

- <span id="naivedate-partialeq-eq"></span>`fn eq(&self, other: &NaiveDate) -> bool` — [`NaiveDate`](#naivedate)

##### `impl PartialOrd for NaiveDate`

- <span id="naivedate-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDate) -> option::Option<cmp::Ordering>` — [`NaiveDate`](#naivedate)

##### `impl Serialize for super::NaiveDate`

- <span id="supernaivedate-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveDate`

##### `impl Sub for NaiveDate`

- <span id="naivedate-sub-type-output"></span>`type Output = NaiveDate`

- <span id="naivedate-sub"></span>`fn sub(self, months: Months) -> <Self as >::Output` — [`Months`](../../month/index.md#months)

##### `impl SubAssign for NaiveDate`

- <span id="naivedate-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../../time_delta/index.md#timedelta)

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

- <span id="naivedatedaysiterator-clone"></span>`fn clone(&self) -> NaiveDateDaysIterator` — [`NaiveDateDaysIterator`](#naivedatedaysiterator)

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

- <span id="naivedatedaysiterator-ord-cmp"></span>`fn cmp(&self, other: &NaiveDateDaysIterator) -> cmp::Ordering` — [`NaiveDateDaysIterator`](#naivedatedaysiterator)

##### `impl PartialEq for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-partialeq-eq"></span>`fn eq(&self, other: &NaiveDateDaysIterator) -> bool` — [`NaiveDateDaysIterator`](#naivedatedaysiterator)

##### `impl PartialOrd for NaiveDateDaysIterator`

- <span id="naivedatedaysiterator-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDateDaysIterator) -> option::Option<cmp::Ordering>` — [`NaiveDateDaysIterator`](#naivedatedaysiterator)

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

- <span id="naivedateweeksiterator-clone"></span>`fn clone(&self) -> NaiveDateWeeksIterator` — [`NaiveDateWeeksIterator`](#naivedateweeksiterator)

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

- <span id="naivedateweeksiterator-ord-cmp"></span>`fn cmp(&self, other: &NaiveDateWeeksIterator) -> cmp::Ordering` — [`NaiveDateWeeksIterator`](#naivedateweeksiterator)

##### `impl PartialEq for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-partialeq-eq"></span>`fn eq(&self, other: &NaiveDateWeeksIterator) -> bool` — [`NaiveDateWeeksIterator`](#naivedateweeksiterator)

##### `impl PartialOrd for NaiveDateWeeksIterator`

- <span id="naivedateweeksiterator-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDateWeeksIterator) -> option::Option<cmp::Ordering>` — [`NaiveDateWeeksIterator`](#naivedateweeksiterator)

##### `impl StructuralPartialEq for NaiveDateWeeksIterator`

## Functions

### `cycle_to_yo`

```rust
const fn cycle_to_yo(cycle: u32) -> (u32, u32)
```

### `yo_to_cycle`

```rust
const fn yo_to_cycle(year_mod_400: u32, ordinal: u32) -> u32
```

### `div_mod_floor`

```rust
const fn div_mod_floor(val: i32, div: i32) -> (i32, i32)
```

## Constants

### `MIN_DATE`
```rust
const MIN_DATE: NaiveDate;
```

The minimum possible `NaiveDate` (January 1, 262145 BCE).

### `MAX_DATE`
```rust
const MAX_DATE: NaiveDate;
```

The maximum possible `NaiveDate` (December 31, 262143 CE).

### `MAX_YEAR`
```rust
const MAX_YEAR: i32 = 262_142i32;
```

MAX_YEAR is one year less than the type is capable of representing. Internally we may sometimes
use the headroom, notably to handle cases where the offset of a `DateTime` constructed with
`NaiveDate::MAX` pushes it beyond the valid, representable range.

### `MIN_YEAR`
```rust
const MIN_YEAR: i32 = -262_143i32;
```

MIN_YEAR is one year more than the type is capable of representing. Internally we may sometimes
use the headroom, notably to handle cases where the offset of a `DateTime` constructed with
`NaiveDate::MIN` pushes it beyond the valid, representable range.

### `ORDINAL_MASK`
```rust
const ORDINAL_MASK: i32 = 8_176i32;
```

### `LEAP_YEAR_MASK`
```rust
const LEAP_YEAR_MASK: i32 = 8i32;
```

### `OL_MASK`
```rust
const OL_MASK: i32 = 8_184i32;
```

### `MAX_OL`
```rust
const MAX_OL: i32 = 5_856i32;
```

### `WEEKDAY_FLAGS_MASK`
```rust
const WEEKDAY_FLAGS_MASK: i32 = 7i32;
```

### `YEAR_FLAGS_MASK`
```rust
const YEAR_FLAGS_MASK: i32 = 15i32;
```

### `YEAR_DELTAS`
```rust
const YEAR_DELTAS: &[u8; 401];
```

