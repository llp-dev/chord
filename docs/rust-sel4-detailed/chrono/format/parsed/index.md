*[chrono](../../index.md) / [format](../index.md) / [parsed](index.md)*

---

# Module `parsed`

A collection of parsed date and time items.
They can be constructed incrementally while being checked for consistency.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parsed`](#parsed) | struct | A type to hold parsed fields of date and time that can check all fields are consistent. |
| [`set_if_consistent`](#set-if-consistent) | fn | Checks if `old` is either empty or has the same value as `new` (i.e. "consistent"), and if it is empty, set `old` to `new` as well. |
| [`resolve_week_date`](#resolve-week-date) | fn | Create a `NaiveDate` when given a year, week, weekday, and the definition at which day of the week a week starts. |

## Structs

### `Parsed`

```rust
struct Parsed {
    // [REDACTED: Private Fields]
}
```

A type to hold parsed fields of date and time that can check all fields are consistent.

There are three classes of methods:

- `set_*` methods to set fields you have available. They do a basic range check, and if the
  same field is set more than once it is checked for consistency.

- `to_*` methods try to make a concrete date and time value out of set fields.
  They fully check that all fields are consistent and whether the date/datetime exists.

- Methods to inspect the parsed fields.

`Parsed` is used internally by all parsing functions in chrono. It is a public type so that it
can be used to write custom parsers that reuse the resolving algorithm, or to inspect the
results of a string parsed with chrono without converting it to concrete types.

# Resolving algorithm

Resolving date/time parts is littered with lots of corner cases, which is why common date/time
parsers do not implement it correctly.

Chrono provides a complete resolution algorithm that checks all fields for consistency via the
`Parsed` type.

As an easy example, consider RFC 2822. The [RFC 2822 date and time format] has a day of the week
part, which should be consistent with the other date parts. But a `strptime`-based parse would
happily accept inconsistent input:

```python
>>> import time
>>> time.strptime('Wed, 31 Dec 2014 04:26:40 +0000',
                  '%a, %d %b %Y %H:%M:%S +0000')
time.struct_time(tm_year=2014, tm_mon=12, tm_mday=31,
                 tm_hour=4, tm_min=26, tm_sec=40,
                 tm_wday=2, tm_yday=365, tm_isdst=-1)
>>> time.strptime('Thu, 31 Dec 2014 04:26:40 +0000',
                  '%a, %d %b %Y %H:%M:%S +0000')
time.struct_time(tm_year=2014, tm_mon=12, tm_mday=31,
                 tm_hour=4, tm_min=26, tm_sec=40,
                 tm_wday=3, tm_yday=365, tm_isdst=-1)
```

# Example

Let's see how `Parsed` correctly detects the second RFC 2822 string from before is inconsistent.

```rust
#[cfg(feature = "alloc")] {
use chrono::format::{ParseErrorKind, Parsed};
use chrono::Weekday;

let mut parsed = Parsed::new();
parsed.set_weekday(Weekday::Wed)?;
parsed.set_day(31)?;
parsed.set_month(12)?;
parsed.set_year(2014)?;
parsed.set_hour(4)?;
parsed.set_minute(26)?;
parsed.set_second(40)?;
parsed.set_offset(0)?;
let dt = parsed.to_datetime()?;
assert_eq!(dt.to_rfc2822(), "Wed, 31 Dec 2014 04:26:40 +0000");

let mut parsed = Parsed::new();
parsed.set_weekday(Weekday::Thu)?; // changed to the wrong day
parsed.set_day(31)?;
parsed.set_month(12)?;
parsed.set_year(2014)?;
parsed.set_hour(4)?;
parsed.set_minute(26)?;
parsed.set_second(40)?;
parsed.set_offset(0)?;
let result = parsed.to_datetime();

assert!(result.is_err());
if let Err(error) = result {
    assert_eq!(error.kind(), ParseErrorKind::Impossible);
}
}
Ok::<(), chrono::ParseError>(())
```

The same using chrono's built-in parser for RFC 2822 (the [RFC2822 formatting item]) and
`format::parse()` showing how to inspect a field on failure.


```rust
#[cfg(feature = "alloc")] {
use chrono::format::{parse, Fixed, Item, Parsed};
use chrono::Weekday;

let rfc_2822 = [Item::Fixed(Fixed::RFC2822)];

let mut parsed = Parsed::new();
parse(&mut parsed, "Wed, 31 Dec 2014 04:26:40 +0000", rfc_2822.iter())?;
let dt = parsed.to_datetime()?;

assert_eq!(dt.to_rfc2822(), "Wed, 31 Dec 2014 04:26:40 +0000");

let mut parsed = Parsed::new();
parse(&mut parsed, "Thu, 31 Dec 2014 04:26:40 +0000", rfc_2822.iter())?;
let result = parsed.to_datetime();

assert!(result.is_err());
if result.is_err() {
    // What is the weekday?
    assert_eq!(parsed.weekday(), Some(Weekday::Thu));
}
}
Ok::<(), chrono::ParseError>(())
```

#### Implementations

- <span id="parsed-new"></span>`fn new() -> Parsed` — [`Parsed`](#parsed)

  Returns the initial value of parsed parts.

- <span id="parsed-set-year"></span>`fn set_year(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`year`](Parsed::year) field to the given value.

  

  The value can be negative, unlike the [`year_div_100`](Parsed::year_div_100) and

  [`year_mod_100`](Parsed::year_mod_100) fields.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is outside the range of an `i32`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-year-div-100"></span>`fn set_year_div_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`year_div_100`](Parsed::year_div_100) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is negative or if it is greater than `i32::MAX`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-year-mod-100"></span>`fn set_year_mod_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`year_mod_100`](Parsed::year_mod_100) field to the given value.

  

  When set it implies that the year is not negative.

  

  If this field is set while the [`year_div_100`](Parsed::year_div_100) field is missing (and

  the full [`year`](Parsed::year) field is also not set), it assumes a default value for the

  [`year_div_100`](Parsed::year_div_100) field.

  The default is 19 when `year_mod_100 >= 70` and 20 otherwise.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is negative or if it is greater than 99.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoyear"></span>`fn set_isoyear(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`isoyear`](Parsed::isoyear) field, that is part of an [ISO 8601 week date], to the

  given value.

  

  The value can be negative, unlike the [`isoyear_div_100`](Parsed::isoyear_div_100) and

  [`isoyear_mod_100`](Parsed::isoyear_mod_100) fields.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is outside the range of an `i32`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoyear-div-100"></span>`fn set_isoyear_div_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`isoyear_div_100`](Parsed::isoyear_div_100) field, that is part of an

  [ISO 8601 week date], to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is negative or if it is greater than `i32::MAX`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoyear-mod-100"></span>`fn set_isoyear_mod_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`isoyear_mod_100`](Parsed::isoyear_mod_100) field, that is part of an

  [ISO 8601 week date], to the given value.

  

  When set it implies that the year is not negative.

  

  If this field is set while the [`isoyear_div_100`](Parsed::isoyear_div_100) field is missing

  (and the full [`isoyear`](Parsed::isoyear) field is also not set), it assumes a default

  value for the [`isoyear_div_100`](Parsed::isoyear_div_100) field.

  The default is 19 when `year_mod_100 >= 70` and 20 otherwise.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is negative or if it is greater than 99.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-quarter"></span>`fn set_quarter(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`quarter`](Parsed::quarter) field to the given value.

  

  Quarter 1 starts in January.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-4.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-month"></span>`fn set_month(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`month`](Parsed::month) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-12.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-week-from-sun"></span>`fn set_week_from_sun(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`week_from_sun`](Parsed::week_from_sun) week number field to the given value.

  

  Week 1 starts at the first Sunday of January.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-53.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-week-from-mon"></span>`fn set_week_from_mon(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`week_from_mon`](Parsed::week_from_mon) week number field to the given value.

  Set the 'week number starting with Monday' field to the given value.

  

  Week 1 starts at the first Monday of January.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-53.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoweek"></span>`fn set_isoweek(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`isoweek`](Parsed::isoweek) field for an [ISO 8601 week date] to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-53.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-weekday"></span>`fn set_weekday(&mut self, value: Weekday) -> ParseResult<()>` — [`Weekday`](../../weekday/index.md#weekday), [`ParseResult`](../index.md#parseresult)

  Set the [`weekday`](Parsed::weekday) field to the given value.

  

  # Errors

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-ordinal"></span>`fn set_ordinal(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`ordinal`](Parsed::ordinal) (day of the year) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-366.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-day"></span>`fn set_day(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`day`](Parsed::day) of the month field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-31.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-ampm"></span>`fn set_ampm(&mut self, value: bool) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`hour_div_12`](Parsed::hour_div_12) am/pm field to the given value.

  

  `false` indicates AM and `true` indicates PM.

  

  # Errors

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-hour12"></span>`fn set_hour12(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`hour_mod_12`](Parsed::hour_mod_12) field, for the hour number in 12-hour clocks,

  to the given value.

  

  Value must be in the canonical range of 1-12.

  It will internally be stored as 0-11 (`value % 12`).

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-12.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-hour"></span>`fn set_hour(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`hour_div_12`](Parsed::hour_div_12) and [`hour_mod_12`](Parsed::hour_mod_12)

  fields to the given value for a 24-hour clock.

  

  # Errors

  

  May return `OUT_OF_RANGE` if `value` is not in the range 0-23.

  Currently only checks the value is not out of range for a `u32`.

  

  Returns `IMPOSSIBLE` one of the fields was already set to a different value.

- <span id="parsed-set-minute"></span>`fn set_minute(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`minute`](Parsed::minute) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-59.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-second"></span>`fn set_second(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`second`](Parsed::second) field to the given value.

  

  The value can be 60 in the case of a leap second.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-60.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-nanosecond"></span>`fn set_nanosecond(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`nanosecond`](Parsed::nanosecond) field to the given value.

  

  This is the number of nanoseconds since the whole second.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-999,999,999.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-timestamp"></span>`fn set_timestamp(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`timestamp`](Parsed::timestamp) field to the given value.

  

  A Unix timestamp is defined as the number of non-leap seconds since midnight UTC on

  January 1, 1970.

  

  # Errors

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-offset"></span>`fn set_offset(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](../index.md#parseresult)

  Set the [`offset`](Parsed::offset) field to the given value.

  

  The offset is in seconds from local time to UTC.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is outside the range of an `i32`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-to-naive-date"></span>`fn to_naive_date(&self) -> ParseResult<NaiveDate>` — [`ParseResult`](../index.md#parseresult), [`NaiveDate`](../../naive/date/index.md#naivedate)

  Returns a parsed naive date out of given fields.

  

  This method is able to determine the date from given subset of fields:

  

  - Year, month, day.

  - Year, day of the year (ordinal).

  - Year, week number counted from Sunday or Monday, day of the week.

  - ISO week date.

  

  Gregorian year and ISO week date year can have their century number (`*_div_100`) omitted,

  the two-digit year is used to guess the century number then.

  

  It checks all given date fields are consistent with each other.

  

  # Errors

  

  This method returns:

  - `IMPOSSIBLE` if any of the date fields conflict.

  - `NOT_ENOUGH` if there are not enough fields set in `Parsed` for a complete date.

  - `OUT_OF_RANGE`

    - if any of the date fields of `Parsed` are set to a value beyond their acceptable range.

    - if the value would be outside the range of a [`NaiveDate`](../../naive/date/index.md).

    - if the date does not exist.

- <span id="parsed-to-naive-time"></span>`fn to_naive_time(&self) -> ParseResult<NaiveTime>` — [`ParseResult`](../index.md#parseresult), [`NaiveTime`](../../naive/time/index.md#naivetime)

  Returns a parsed naive time out of given fields.

  

  This method is able to determine the time from given subset of fields:

  

  - Hour, minute. (second and nanosecond assumed to be 0)

  - Hour, minute, second. (nanosecond assumed to be 0)

  - Hour, minute, second, nanosecond.

  

  It is able to handle leap seconds when given second is 60.

  

  # Errors

  

  This method returns:

  - `OUT_OF_RANGE` if any of the time fields of `Parsed` are set to a value beyond

    their acceptable range.

  - `NOT_ENOUGH` if an hour field is missing, if AM/PM is missing in a 12-hour clock,

    if minutes are missing, or if seconds are missing while the nanosecond field is present.

- <span id="parsed-to-naive-datetime-with-offset"></span>`fn to_naive_datetime_with_offset(&self, offset: i32) -> ParseResult<NaiveDateTime>` — [`ParseResult`](../index.md#parseresult), [`NaiveDateTime`](../../naive/datetime/index.md#naivedatetime)

  Returns a parsed naive date and time out of given fields, except for the offset field.

  

  The offset is assumed to have a given value. It is not compared against the offset field set

  in the `Parsed` type, so it is allowed to be inconsistent.

  

  This method is able to determine the combined date and time from date and time fields or

  from a single timestamp field. It checks all fields are consistent with each other.

  

  # Errors

  

  This method returns:

  - `IMPOSSIBLE`  if any of the date fields conflict, or if a timestamp conflicts with any of

    the other fields.

  - `NOT_ENOUGH` if there are not enough fields set in `Parsed` for a complete datetime.

  - `OUT_OF_RANGE`

    - if any of the date or time fields of `Parsed` are set to a value beyond their acceptable

      range.

    - if the value would be outside the range of a [`NaiveDateTime`](../../naive/datetime/index.md).

    - if the date does not exist.

- <span id="parsed-to-fixed-offset"></span>`fn to_fixed_offset(&self) -> ParseResult<FixedOffset>` — [`ParseResult`](../index.md#parseresult), [`FixedOffset`](../../offset/fixed/index.md#fixedoffset)

  Returns a parsed fixed time zone offset out of given fields.

  

  # Errors

  

  This method returns:

  - `OUT_OF_RANGE` if the offset is out of range for a `FixedOffset`.

  - `NOT_ENOUGH` if the offset field is not set.

- <span id="parsed-to-datetime"></span>`fn to_datetime(&self) -> ParseResult<DateTime<FixedOffset>>` — [`ParseResult`](../index.md#parseresult), [`DateTime`](../../datetime/index.md#datetime), [`FixedOffset`](../../offset/fixed/index.md#fixedoffset)

  Returns a parsed timezone-aware date and time out of given fields.

  

  This method is able to determine the combined date and time from date, time and offset

  fields, and/or from a single timestamp field. It checks all fields are consistent with each

  other.

  

  # Errors

  

  This method returns:

  - `IMPOSSIBLE`  if any of the date fields conflict, or if a timestamp conflicts with any of

    the other fields.

  - `NOT_ENOUGH` if there are not enough fields set in `Parsed` for a complete datetime

    including offset from UTC.

  - `OUT_OF_RANGE`

    - if any of the fields of `Parsed` are set to a value beyond their acceptable

      range.

    - if the value would be outside the range of a [`NaiveDateTime`](../../naive/datetime/index.md) or [`FixedOffset`](../../offset/fixed/index.md).

    - if the date does not exist.

- <span id="parsed-to-datetime-with-timezone"></span>`fn to_datetime_with_timezone<Tz: TimeZone>(&self, tz: &Tz) -> ParseResult<DateTime<Tz>>` — [`ParseResult`](../index.md#parseresult), [`DateTime`](../../datetime/index.md#datetime)

  Returns a parsed timezone-aware date and time out of given fields,

  with an additional [`TimeZone`](../../offset/index.md) used to interpret and validate the local date.

  

  This method is able to determine the combined date and time from date and time, and/or from

  a single timestamp field. It checks all fields are consistent with each other.

  

  If the parsed fields include an UTC offset, it also has to be consistent with the offset in

  the provided `tz` time zone for that datetime.

  

  # Errors

  

  This method returns:

  - `IMPOSSIBLE`

    - if any of the date fields conflict, if a timestamp conflicts with any of the other

      fields, or if the offset field is set but differs from the offset at that time in the

      `tz` time zone.

    - if the local datetime does not exists in the provided time zone (because it falls in a

      transition due to for example DST).

  - `NOT_ENOUGH` if there are not enough fields set in `Parsed` for a complete datetime, or if

    the local time in the provided time zone is ambiguous (because it falls in a transition

    due to for example DST) while there is no offset field or timestamp field set.

  - `OUT_OF_RANGE`

    - if the value would be outside the range of a [`NaiveDateTime`](../../naive/datetime/index.md) or [`FixedOffset`](../../offset/fixed/index.md).

    - if any of the fields of `Parsed` are set to a value beyond their acceptable range.

    - if the date does not exist.

- <span id="parsed-year"></span>`fn year(&self) -> Option<i32>`

  Get the `year` field if set.

  

  See also [`set_year()`](Parsed::set_year).

- <span id="parsed-year-div-100"></span>`fn year_div_100(&self) -> Option<i32>`

  Get the `year_div_100` field if set.

  

  See also [`set_year_div_100()`](Parsed::set_year_div_100).

- <span id="parsed-year-mod-100"></span>`fn year_mod_100(&self) -> Option<i32>`

  Get the `year_mod_100` field if set.

  

  See also [`set_year_mod_100()`](Parsed::set_year_mod_100).

- <span id="parsed-isoyear"></span>`fn isoyear(&self) -> Option<i32>`

  Get the `isoyear` field that is part of an [ISO 8601 week date] if set.

  

  See also [`set_isoyear()`](Parsed::set_isoyear).

- <span id="parsed-isoyear-div-100"></span>`fn isoyear_div_100(&self) -> Option<i32>`

  Get the `isoyear_div_100` field that is part of an [ISO 8601 week date] if set.

  

  See also [`set_isoyear_div_100()`](Parsed::set_isoyear_div_100).

- <span id="parsed-isoyear-mod-100"></span>`fn isoyear_mod_100(&self) -> Option<i32>`

  Get the `isoyear_mod_100` field that is part of an [ISO 8601 week date] if set.

  

  See also [`set_isoyear_mod_100()`](Parsed::set_isoyear_mod_100).

- <span id="parsed-quarter"></span>`fn quarter(&self) -> Option<u32>`

  Get the `quarter` field if set.

  

  See also [`set_quarter()`](Parsed::set_quarter).

- <span id="parsed-month"></span>`fn month(&self) -> Option<u32>`

  Get the `month` field if set.

  

  See also [`set_month()`](Parsed::set_month).

- <span id="parsed-week-from-sun"></span>`fn week_from_sun(&self) -> Option<u32>`

  Get the `week_from_sun` field if set.

  

  See also [`set_week_from_sun()`](Parsed::set_week_from_sun).

- <span id="parsed-week-from-mon"></span>`fn week_from_mon(&self) -> Option<u32>`

  Get the `week_from_mon` field if set.

  

  See also [`set_week_from_mon()`](Parsed::set_week_from_mon).

- <span id="parsed-isoweek"></span>`fn isoweek(&self) -> Option<u32>`

  Get the `isoweek` field that is part of an [ISO 8601 week date] if set.

  

  See also [`set_isoweek()`](Parsed::set_isoweek).

- <span id="parsed-weekday"></span>`fn weekday(&self) -> Option<Weekday>` — [`Weekday`](../../weekday/index.md#weekday)

  Get the `weekday` field if set.

  

  See also [`set_weekday()`](Parsed::set_weekday).

- <span id="parsed-ordinal"></span>`fn ordinal(&self) -> Option<u32>`

  Get the `ordinal` (day of the year) field if set.

  

  See also [`set_ordinal()`](Parsed::set_ordinal).

- <span id="parsed-day"></span>`fn day(&self) -> Option<u32>`

  Get the `day` of the month field if set.

  

  See also [`set_day()`](Parsed::set_day).

- <span id="parsed-hour-div-12"></span>`fn hour_div_12(&self) -> Option<u32>`

  Get the `hour_div_12` field (am/pm) if set.

  

  0 indicates AM and 1 indicates PM.

  

  See also [`set_ampm()`](Parsed::set_ampm) and [`set_hour()`](Parsed::set_hour).

- <span id="parsed-hour-mod-12"></span>`fn hour_mod_12(&self) -> Option<u32>`

  Get the `hour_mod_12` field if set.

  

  See also [`set_hour12()`](Parsed::set_hour12) and [`set_hour()`](Parsed::set_hour).

- <span id="parsed-minute"></span>`fn minute(&self) -> Option<u32>`

  Get the `minute` field if set.

  

  See also [`set_minute()`](Parsed::set_minute).

- <span id="parsed-second"></span>`fn second(&self) -> Option<u32>`

  Get the `second` field if set.

  

  See also [`set_second()`](Parsed::set_second).

- <span id="parsed-nanosecond"></span>`fn nanosecond(&self) -> Option<u32>`

  Get the `nanosecond` field if set.

  

  See also [`set_nanosecond()`](Parsed::set_nanosecond).

- <span id="parsed-timestamp"></span>`fn timestamp(&self) -> Option<i64>`

  Get the `timestamp` field if set.

  

  See also [`set_timestamp()`](Parsed::set_timestamp).

- <span id="parsed-offset"></span>`fn offset(&self) -> Option<i32>`

  Get the `offset` field if set.

  

  See also [`set_offset()`](Parsed::set_offset).

#### Trait Implementations

##### `impl Clone for Parsed`

- <span id="parsed-clone"></span>`fn clone(&self) -> Parsed` — [`Parsed`](#parsed)

##### `impl Debug for Parsed`

- <span id="parsed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parsed`

- <span id="parsed-default"></span>`fn default() -> Parsed` — [`Parsed`](#parsed)

##### `impl Eq for Parsed`

##### `impl Hash for Parsed`

- <span id="parsed-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Parsed`

- <span id="parsed-partialeq-eq"></span>`fn eq(&self, other: &Parsed) -> bool` — [`Parsed`](#parsed)

##### `impl StructuralPartialEq for Parsed`

## Functions

### `set_if_consistent`

```rust
fn set_if_consistent<T: PartialEq>(old: &mut Option<T>, new: T) -> super::ParseResult<()>
```

Checks if `old` is either empty or has the same value as `new` (i.e. "consistent"),
and if it is empty, set `old` to `new` as well.

### `resolve_week_date`

```rust
fn resolve_week_date(year: i32, week: u32, weekday: crate::Weekday, week_start_day: crate::Weekday) -> super::ParseResult<crate::naive::NaiveDate>
```

Create a `NaiveDate` when given a year, week, weekday, and the definition at which day of the
week a week starts.

Returns `IMPOSSIBLE` if `week` is `0` or `53` and the `weekday` falls outside the year.

