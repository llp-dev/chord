*[chrono](../index.md) / [format](index.md)*

---

# Module `format`

Formatting (and parsing) utilities for date and time.

This module provides the common types and routines to implement,
for example, [`DateTime::format`](../struct.DateTime.html#method.format) or
[`DateTime::parse_from_str`](../struct.DateTime.html#method.parse_from_str) methods.
For most cases you should use these high-level interfaces.

Internally the formatting and parsing shares the same abstract **formatting items**,
which are just an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) of
the [`Item`](./enum.Item.html) type.
They are generated from more readable **format strings**;
currently Chrono supports a built-in syntax closely resembling
C's `strftime` format. The available options can be found [here](./strftime/index.html).

# Example
```rust
#[cfg(feature = "alloc")] {
use chrono::{NaiveDateTime, TimeZone, Utc};

let date_time = Utc.with_ymd_and_hms(2020, 11, 10, 0, 1, 32).unwrap();

let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
assert_eq!(formatted, "2020-11-10 00:01:32");

let parsed = NaiveDateTime::parse_from_str(&formatted, "%Y-%m-%d %H:%M:%S")?.and_utc();
assert_eq!(parsed, date_time);
}
Ok::<(), chrono::ParseError>(())
```

## Contents

- [Modules](#modules)
  - [`formatting`](#formatting)
  - [`parsed`](#parsed)
  - [`parse`](#parse)
  - [`scan`](#scan)
  - [`strftime`](#strftime)
  - [`locales`](#locales)
- [Structs](#structs)
  - [`Parsed`](#parsed)
  - [`StrftimeItems`](#strftimeitems)
  - [`InternalNumeric`](#internalnumeric)
  - [`InternalFixed`](#internalfixed)
  - [`OffsetFormat`](#offsetformat)
  - [`ParseError`](#parseerror)
- [Enums](#enums)
  - [`SecondsFormat`](#secondsformat)
  - [`Void`](#void)
  - [`Pad`](#pad)
  - [`Numeric`](#numeric)
  - [`Fixed`](#fixed)
  - [`InternalInternal`](#internalinternal)
  - [`OffsetPrecision`](#offsetprecision)
  - [`Colons`](#colons)
  - [`Item`](#item)
  - [`ParseErrorKind`](#parseerrorkind)
- [Functions](#functions)
  - [`parse`](#parse)
  - [`parse_and_remainder`](#parse-and-remainder)
  - [`num`](#num)
  - [`num0`](#num0)
  - [`nums`](#nums)
  - [`fixed`](#fixed)
  - [`internal_fixed`](#internal-fixed)
- [Type Aliases](#type-aliases)
  - [`ParseResult`](#parseresult)
- [Constants](#constants)
  - [`OUT_OF_RANGE`](#out-of-range)
  - [`IMPOSSIBLE`](#impossible)
  - [`NOT_ENOUGH`](#not-enough)
  - [`INVALID`](#invalid)
  - [`TOO_SHORT`](#too-short)
  - [`TOO_LONG`](#too-long)
  - [`BAD_FORMAT`](#bad-format)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`formatting`](#formatting) | mod | Date and time formatting routines. |
| [`parsed`](#parsed) | mod | A collection of parsed date and time items. |
| [`parse`](#parse) | mod | Date and time parsing routines. |
| [`scan`](#scan) | mod | Various scanning routines for the parser. |
| [`strftime`](#strftime) | mod | `strftime`/`strptime`-inspired date and time formatting syntax. |
| [`locales`](#locales) | mod |  |
| [`Parsed`](#parsed) | struct |  |
| [`StrftimeItems`](#strftimeitems) | struct |  |
| [`InternalNumeric`](#internalnumeric) | struct | An opaque type representing numeric item types for internal uses only. |
| [`InternalFixed`](#internalfixed) | struct | An opaque type representing fixed-format item types for internal uses only. |
| [`OffsetFormat`](#offsetformat) | struct | Type for specifying the format of UTC offsets. |
| [`ParseError`](#parseerror) | struct | An error from the `parse` function. |
| [`SecondsFormat`](#secondsformat) | enum |  |
| [`Void`](#void) | enum | An uninhabited type used for `InternalNumeric` and `InternalFixed` below. |
| [`Pad`](#pad) | enum | Padding characters for numeric items. |
| [`Numeric`](#numeric) | enum | Numeric item types. |
| [`Fixed`](#fixed) | enum | Fixed-format item types. |
| [`InternalInternal`](#internalinternal) | enum |  |
| [`OffsetPrecision`](#offsetprecision) | enum | The precision of an offset from UTC formatting item. |
| [`Colons`](#colons) | enum | The separator between hours and minutes in an offset. |
| [`Item`](#item) | enum | A single formatting item. |
| [`ParseErrorKind`](#parseerrorkind) | enum | The category of parse error |
| [`parse`](#parse) | fn |  |
| [`parse_and_remainder`](#parse-and-remainder) | fn |  |
| [`num`](#num) | fn |  |
| [`num0`](#num0) | fn |  |
| [`nums`](#nums) | fn |  |
| [`fixed`](#fixed) | fn |  |
| [`internal_fixed`](#internal-fixed) | fn |  |
| [`ParseResult`](#parseresult) | type | Same as `Result<T, ParseError>`. |
| [`OUT_OF_RANGE`](#out-of-range) | const |  |
| [`IMPOSSIBLE`](#impossible) | const |  |
| [`NOT_ENOUGH`](#not-enough) | const |  |
| [`INVALID`](#invalid) | const |  |
| [`TOO_SHORT`](#too-short) | const |  |
| [`TOO_LONG`](#too-long) | const |  |
| [`BAD_FORMAT`](#bad-format) | const |  |

## Modules

- [`formatting`](formatting/index.md) — Date and time formatting routines.
- [`parsed`](parsed/index.md) — A collection of parsed date and time items.
- [`parse`](parse/index.md) — Date and time parsing routines.
- [`scan`](scan/index.md) — Various scanning routines for the parser.
- [`strftime`](strftime/index.md) — `strftime`/`strptime`-inspired date and time formatting syntax.
- [`locales`](locales/index.md)

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

- <span id="parsed-new"></span>`fn new() -> Parsed` — [`Parsed`](parsed/index.md#parsed)

  Returns the initial value of parsed parts.

- <span id="parsed-set-year"></span>`fn set_year(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`year`](Parsed::year) field to the given value.

  

  The value can be negative, unlike the [`year_div_100`](Parsed::year_div_100) and

  [`year_mod_100`](Parsed::year_mod_100) fields.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is outside the range of an `i32`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-year-div-100"></span>`fn set_year_div_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`year_div_100`](Parsed::year_div_100) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is negative or if it is greater than `i32::MAX`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-year-mod-100"></span>`fn set_year_mod_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`year_mod_100`](Parsed::year_mod_100) field to the given value.

  

  When set it implies that the year is not negative.

  

  If this field is set while the [`year_div_100`](Parsed::year_div_100) field is missing (and

  the full [`year`](Parsed::year) field is also not set), it assumes a default value for the

  [`year_div_100`](Parsed::year_div_100) field.

  The default is 19 when `year_mod_100 >= 70` and 20 otherwise.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is negative or if it is greater than 99.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoyear"></span>`fn set_isoyear(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`isoyear`](Parsed::isoyear) field, that is part of an [ISO 8601 week date], to the

  given value.

  

  The value can be negative, unlike the [`isoyear_div_100`](Parsed::isoyear_div_100) and

  [`isoyear_mod_100`](Parsed::isoyear_mod_100) fields.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is outside the range of an `i32`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoyear-div-100"></span>`fn set_isoyear_div_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`isoyear_div_100`](Parsed::isoyear_div_100) field, that is part of an

  [ISO 8601 week date], to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is negative or if it is greater than `i32::MAX`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoyear-mod-100"></span>`fn set_isoyear_mod_100(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

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

- <span id="parsed-set-quarter"></span>`fn set_quarter(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`quarter`](Parsed::quarter) field to the given value.

  

  Quarter 1 starts in January.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-4.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-month"></span>`fn set_month(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`month`](Parsed::month) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-12.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-week-from-sun"></span>`fn set_week_from_sun(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`week_from_sun`](Parsed::week_from_sun) week number field to the given value.

  

  Week 1 starts at the first Sunday of January.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-53.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-week-from-mon"></span>`fn set_week_from_mon(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`week_from_mon`](Parsed::week_from_mon) week number field to the given value.

  Set the 'week number starting with Monday' field to the given value.

  

  Week 1 starts at the first Monday of January.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-53.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-isoweek"></span>`fn set_isoweek(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`isoweek`](Parsed::isoweek) field for an [ISO 8601 week date] to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-53.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-weekday"></span>`fn set_weekday(&mut self, value: Weekday) -> ParseResult<()>` — [`Weekday`](../weekday/index.md#weekday), [`ParseResult`](#parseresult)

  Set the [`weekday`](Parsed::weekday) field to the given value.

  

  # Errors

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-ordinal"></span>`fn set_ordinal(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`ordinal`](Parsed::ordinal) (day of the year) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-366.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-day"></span>`fn set_day(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`day`](Parsed::day) of the month field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-31.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-ampm"></span>`fn set_ampm(&mut self, value: bool) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`hour_div_12`](Parsed::hour_div_12) am/pm field to the given value.

  

  `false` indicates AM and `true` indicates PM.

  

  # Errors

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-hour12"></span>`fn set_hour12(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`hour_mod_12`](Parsed::hour_mod_12) field, for the hour number in 12-hour clocks,

  to the given value.

  

  Value must be in the canonical range of 1-12.

  It will internally be stored as 0-11 (`value % 12`).

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 1-12.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-hour"></span>`fn set_hour(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`hour_div_12`](Parsed::hour_div_12) and [`hour_mod_12`](Parsed::hour_mod_12)

  fields to the given value for a 24-hour clock.

  

  # Errors

  

  May return `OUT_OF_RANGE` if `value` is not in the range 0-23.

  Currently only checks the value is not out of range for a `u32`.

  

  Returns `IMPOSSIBLE` one of the fields was already set to a different value.

- <span id="parsed-set-minute"></span>`fn set_minute(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`minute`](Parsed::minute) field to the given value.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-59.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-second"></span>`fn set_second(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`second`](Parsed::second) field to the given value.

  

  The value can be 60 in the case of a leap second.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-60.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-nanosecond"></span>`fn set_nanosecond(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`nanosecond`](Parsed::nanosecond) field to the given value.

  

  This is the number of nanoseconds since the whole second.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is not in the range 0-999,999,999.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-timestamp"></span>`fn set_timestamp(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`timestamp`](Parsed::timestamp) field to the given value.

  

  A Unix timestamp is defined as the number of non-leap seconds since midnight UTC on

  January 1, 1970.

  

  # Errors

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-set-offset"></span>`fn set_offset(&mut self, value: i64) -> ParseResult<()>` — [`ParseResult`](#parseresult)

  Set the [`offset`](Parsed::offset) field to the given value.

  

  The offset is in seconds from local time to UTC.

  

  # Errors

  

  Returns `OUT_OF_RANGE` if `value` is outside the range of an `i32`.

  

  Returns `IMPOSSIBLE` if this field was already set to a different value.

- <span id="parsed-to-naive-date"></span>`fn to_naive_date(&self) -> ParseResult<NaiveDate>` — [`ParseResult`](#parseresult), [`NaiveDate`](../naive/date/index.md#naivedate)

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

    - if the value would be outside the range of a [`NaiveDate`](../naive/date/index.md).

    - if the date does not exist.

- <span id="parsed-to-naive-time"></span>`fn to_naive_time(&self) -> ParseResult<NaiveTime>` — [`ParseResult`](#parseresult), [`NaiveTime`](../naive/time/index.md#naivetime)

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

- <span id="parsed-to-naive-datetime-with-offset"></span>`fn to_naive_datetime_with_offset(&self, offset: i32) -> ParseResult<NaiveDateTime>` — [`ParseResult`](#parseresult), [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime)

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

    - if the value would be outside the range of a [`NaiveDateTime`](../naive/datetime/index.md).

    - if the date does not exist.

- <span id="parsed-to-fixed-offset"></span>`fn to_fixed_offset(&self) -> ParseResult<FixedOffset>` — [`ParseResult`](#parseresult), [`FixedOffset`](../offset/fixed/index.md#fixedoffset)

  Returns a parsed fixed time zone offset out of given fields.

  

  # Errors

  

  This method returns:

  - `OUT_OF_RANGE` if the offset is out of range for a `FixedOffset`.

  - `NOT_ENOUGH` if the offset field is not set.

- <span id="parsed-to-datetime"></span>`fn to_datetime(&self) -> ParseResult<DateTime<FixedOffset>>` — [`ParseResult`](#parseresult), [`DateTime`](../datetime/index.md#datetime), [`FixedOffset`](../offset/fixed/index.md#fixedoffset)

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

    - if the value would be outside the range of a [`NaiveDateTime`](../naive/datetime/index.md) or [`FixedOffset`](../offset/fixed/index.md).

    - if the date does not exist.

- <span id="parsed-to-datetime-with-timezone"></span>`fn to_datetime_with_timezone<Tz: TimeZone>(&self, tz: &Tz) -> ParseResult<DateTime<Tz>>` — [`ParseResult`](#parseresult), [`DateTime`](../datetime/index.md#datetime)

  Returns a parsed timezone-aware date and time out of given fields,

  with an additional [`TimeZone`](../offset/index.md) used to interpret and validate the local date.

  

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

    - if the value would be outside the range of a [`NaiveDateTime`](../naive/datetime/index.md) or [`FixedOffset`](../offset/fixed/index.md).

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

- <span id="parsed-weekday"></span>`fn weekday(&self) -> Option<Weekday>` — [`Weekday`](../weekday/index.md#weekday)

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

- <span id="parsed-clone"></span>`fn clone(&self) -> Parsed` — [`Parsed`](parsed/index.md#parsed)

##### `impl Debug for Parsed`

- <span id="parsed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parsed`

- <span id="parsed-default"></span>`fn default() -> Parsed` — [`Parsed`](parsed/index.md#parsed)

##### `impl Eq for Parsed`

##### `impl Hash for Parsed`

- <span id="parsed-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Parsed`

- <span id="parsed-partialeq-eq"></span>`fn eq(&self, other: &Parsed) -> bool` — [`Parsed`](parsed/index.md#parsed)

##### `impl StructuralPartialEq for Parsed`

### `StrftimeItems<'a>`

```rust
struct StrftimeItems<'a> {
    remainder: &'a str,
    queue: &'static [super::Item<'static>],
    lenient: bool,
}
```

Parsing iterator for `strftime`-like format strings.

See the [`format::strftime` module](crate::format::strftime) for supported formatting
specifiers.

`StrftimeItems` is used in combination with more low-level methods such as `format::parse()`
or `format_with_items`.

If formatting or parsing date and time values is not performance-critical, the methods
`parse_from_str` and [`format`](#format) on types such as [`DateTime`](crate::DateTime) are easier to
use.






#### Fields

- **`remainder`**: `&'a str`

  Remaining portion of the string.

- **`queue`**: `&'static [super::Item<'static>]`

  If the current specifier is composed of multiple formatting items (e.g. `%+`),
  `queue` stores a slice of `Item`s that have to be returned one by one.

#### Implementations

- <span id="strftimeitems-new"></span>`const fn new(s: &'a str) -> StrftimeItems<'a>` — [`StrftimeItems`](strftime/index.md#strftimeitems)

  Creates a new parsing iterator from a `strftime`-like format string.

  

  # Errors

  

  While iterating [`Item::Error`](../index.md) will be returned if the format string contains an invalid

  or unrecognized formatting specifier.

  

  # Example

  

  ```rust

  use chrono::format::*;

  

  let strftime_parser = StrftimeItems::new("%F"); // %F: year-month-day (ISO 8601)

  

  const ISO8601_YMD_ITEMS: &[Item<'static>] = &[

      Item::Numeric(Numeric::Year, Pad::Zero),

      Item::Literal("-"),

      Item::Numeric(Numeric::Month, Pad::Zero),

      Item::Literal("-"),

      Item::Numeric(Numeric::Day, Pad::Zero),

  ];

  assert!(strftime_parser.eq(ISO8601_YMD_ITEMS.iter().cloned()));

  ```

- <span id="strftimeitems-new-lenient"></span>`const fn new_lenient(s: &'a str) -> StrftimeItems<'a>` — [`StrftimeItems`](strftime/index.md#strftimeitems)

  The same as `StrftimeItems::new`, but returns [`Item::Literal`](../index.md) instead of [`Item::Error`](../index.md).

  

  Useful for formatting according to potentially invalid format strings.

  

  # Example

  

  ```rust

  use chrono::format::*;

  

  let strftime_parser = StrftimeItems::new_lenient("%Y-%Q"); // %Y: year, %Q: invalid

  

  const ITEMS: &[Item<'static>] = &[

      Item::Numeric(Numeric::Year, Pad::Zero),

      Item::Literal("-"),

      Item::Literal("%Q"),

  ];

  println!("{:?}", strftime_parser.clone().collect::<Vec<_>>());

  assert!(strftime_parser.eq(ITEMS.iter().cloned()));

  ```

- <span id="strftimeitems-parse-next-item"></span>`fn parse_next_item(&mut self, remainder: &'a str) -> Option<(&'a str, Item<'a>)>` — [`Item`](#item)

- <span id="strftimeitems-error"></span>`fn error<'b>(&mut self, original: &'b str, remainder: &'b str) -> Item<'b>` — [`Item`](#item)

#### Trait Implementations

##### `impl Clone for StrftimeItems<'a>`

- <span id="strftimeitems-clone"></span>`fn clone(&self) -> StrftimeItems<'a>` — [`StrftimeItems`](strftime/index.md#strftimeitems)

##### `impl Debug for StrftimeItems<'a>`

- <span id="strftimeitems-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for StrftimeItems<'a>`

- <span id="strftimeitems-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strftimeitems-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strftimeitems-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrftimeItems<'a>`

- <span id="strftimeitems-iterator-type-item"></span>`type Item = Item<'a>`

- <span id="strftimeitems-iterator-next"></span>`fn next(&mut self) -> Option<Item<'a>>` — [`Item`](#item)

### `InternalNumeric`

```rust
struct InternalNumeric {
    _dummy: Void,
}
```

An opaque type representing numeric item types for internal uses only.

#### Trait Implementations

##### `impl Clone for InternalNumeric`

- <span id="internalnumeric-clone"></span>`fn clone(&self) -> InternalNumeric` — [`InternalNumeric`](#internalnumeric)

##### `impl Debug for InternalNumeric`

- <span id="internalnumeric-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InternalNumeric`

##### `impl Hash for InternalNumeric`

- <span id="internalnumeric-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for InternalNumeric`

- <span id="internalnumeric-partialeq-eq"></span>`fn eq(&self, other: &InternalNumeric) -> bool` — [`InternalNumeric`](#internalnumeric)

##### `impl StructuralPartialEq for InternalNumeric`

### `InternalFixed`

```rust
struct InternalFixed {
    val: InternalInternal,
}
```

An opaque type representing fixed-format item types for internal uses only.

#### Trait Implementations

##### `impl Clone for InternalFixed`

- <span id="internalfixed-clone"></span>`fn clone(&self) -> InternalFixed` — [`InternalFixed`](#internalfixed)

##### `impl Debug for InternalFixed`

- <span id="internalfixed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InternalFixed`

##### `impl Hash for InternalFixed`

- <span id="internalfixed-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for InternalFixed`

- <span id="internalfixed-partialeq-eq"></span>`fn eq(&self, other: &InternalFixed) -> bool` — [`InternalFixed`](#internalfixed)

##### `impl StructuralPartialEq for InternalFixed`

### `OffsetFormat`

```rust
struct OffsetFormat {
    pub precision: OffsetPrecision,
    pub colons: Colons,
    pub allow_zulu: bool,
    pub padding: Pad,
}
```

Type for specifying the format of UTC offsets.

#### Fields

- **`precision`**: `OffsetPrecision`

  See `OffsetPrecision`.

- **`colons`**: `Colons`

  Separator between hours, minutes and seconds.

- **`allow_zulu`**: `bool`

  Represent `+00:00` as `Z`.

- **`padding`**: `Pad`

  Pad the hour value to two digits.

#### Implementations

- <span id="superoffsetformat-format"></span>`fn format(&self, w: &mut impl Write + ?Sized, off: FixedOffset) -> fmt::Result` — [`FixedOffset`](../offset/fixed/index.md#fixedoffset)

  Writes an offset from UTC with the format defined by `self`.

#### Trait Implementations

##### `impl Clone for OffsetFormat`

- <span id="offsetformat-clone"></span>`fn clone(&self) -> OffsetFormat` — [`OffsetFormat`](#offsetformat)

##### `impl Copy for OffsetFormat`

##### `impl Debug for OffsetFormat`

- <span id="offsetformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OffsetFormat`

##### `impl Hash for OffsetFormat`

- <span id="offsetformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for OffsetFormat`

- <span id="offsetformat-partialeq-eq"></span>`fn eq(&self, other: &OffsetFormat) -> bool` — [`OffsetFormat`](#offsetformat)

##### `impl StructuralPartialEq for OffsetFormat`

### `ParseError`

```rust
struct ParseError(ParseErrorKind);
```

An error from the `parse` function.

#### Implementations

- <span id="parseerror-kind"></span>`const fn kind(&self) -> ParseErrorKind` — [`ParseErrorKind`](#parseerrorkind)

  The category of parse error

#### Trait Implementations

##### `impl Clone for ParseError`

- <span id="parseerror-clone"></span>`fn clone(&self) -> ParseError` — [`ParseError`](#parseerror)

##### `impl Copy for ParseError`

##### `impl Debug for ParseError`

- <span id="parseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseError`

- <span id="parseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseError`

##### `impl Hash for ParseError`

- <span id="parseerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ParseError`

- <span id="parseerror-partialeq-eq"></span>`fn eq(&self, other: &ParseError) -> bool` — [`ParseError`](#parseerror)

##### `impl StructuralPartialEq for ParseError`

##### `impl ToString for ParseError`

- <span id="parseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `SecondsFormat`

```rust
enum SecondsFormat {
    Secs,
    Millis,
    Micros,
    Nanos,
    AutoSi,
}
```

Specific formatting options for seconds. This may be extended in the
future, so exhaustive matching in external code is not recommended.

See the `TimeZone::to_rfc3339_opts` function for usage.

#### Variants

- **`Secs`**

  Format whole seconds only, with no decimal point nor subseconds.

- **`Millis`**

  Use fixed 3 subsecond digits. This corresponds to [Fixed::Nanosecond3].

- **`Micros`**

  Use fixed 6 subsecond digits. This corresponds to [Fixed::Nanosecond6].

- **`Nanos`**

  Use fixed 9 subsecond digits. This corresponds to [Fixed::Nanosecond9].

- **`AutoSi`**

  Automatically select one of `Secs`, `Millis`, `Micros`, or `Nanos` to display all available
  non-zero sub-second digits.  This corresponds to [Fixed::Nanosecond].

#### Trait Implementations

##### `impl Clone for SecondsFormat`

- <span id="secondsformat-clone"></span>`fn clone(&self) -> SecondsFormat` — [`SecondsFormat`](formatting/index.md#secondsformat)

##### `impl Copy for SecondsFormat`

##### `impl Debug for SecondsFormat`

- <span id="secondsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SecondsFormat`

##### `impl Hash for SecondsFormat`

- <span id="secondsformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SecondsFormat`

- <span id="secondsformat-partialeq-eq"></span>`fn eq(&self, other: &SecondsFormat) -> bool` — [`SecondsFormat`](formatting/index.md#secondsformat)

##### `impl StructuralPartialEq for SecondsFormat`

### `Void`

```rust
enum Void {
}
```

An uninhabited type used for `InternalNumeric` and `InternalFixed` below.

#### Trait Implementations

##### `impl Clone for Void`

- <span id="void-clone"></span>`fn clone(&self) -> Void` — [`Void`](#void)

##### `impl Eq for Void`

##### `impl Hash for Void`

- <span id="void-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Void`

- <span id="void-partialeq-eq"></span>`fn eq(&self, other: &Void) -> bool` — [`Void`](#void)

##### `impl StructuralPartialEq for Void`

### `Pad`

```rust
enum Pad {
    None,
    Zero,
    Space,
}
```

Padding characters for numeric items.

#### Variants

- **`None`**

  No padding.

- **`Zero`**

  Zero (`0`) padding.

- **`Space`**

  Space padding.

#### Trait Implementations

##### `impl Clone for Pad`

- <span id="pad-clone"></span>`fn clone(&self) -> Pad` — [`Pad`](#pad)

##### `impl Copy for Pad`

##### `impl Debug for Pad`

- <span id="pad-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Pad`

##### `impl Hash for Pad`

- <span id="pad-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Pad`

- <span id="pad-partialeq-eq"></span>`fn eq(&self, other: &Pad) -> bool` — [`Pad`](#pad)

##### `impl StructuralPartialEq for Pad`

### `Numeric`

```rust
enum Numeric {
    Year,
    YearDiv100,
    YearMod100,
    IsoYear,
    IsoYearDiv100,
    IsoYearMod100,
    Quarter,
    Month,
    Day,
    WeekFromSun,
    WeekFromMon,
    IsoWeek,
    NumDaysFromSun,
    WeekdayFromMon,
    Ordinal,
    Hour,
    Hour12,
    Minute,
    Second,
    Nanosecond,
    Timestamp,
    Internal(InternalNumeric),
}
```

Numeric item types.
They have associated formatting width (FW) and parsing width (PW).

The **formatting width** is the minimal width to be formatted.
If the number is too short, and the padding is not [`Pad::None`](./enum.Pad.html#variant.None),
then it is left-padded.
If the number is too long or (in some cases) negative, it is printed as is.

The **parsing width** is the maximal width to be scanned.
The parser only tries to consume from one to given number of digits (greedily).
It also trims the preceding whitespace if any.
It cannot parse the negative number, so some date and time cannot be formatted then
parsed with the same formatting items.

#### Variants

- **`Year`**

  Full Gregorian year (FW=4, PW=∞).
  May accept years before 1 BCE or after 9999 CE, given an initial sign (+/-).

- **`YearDiv100`**

  Gregorian year divided by 100 (century number; FW=PW=2). Implies the non-negative year.

- **`YearMod100`**

  Gregorian year modulo 100 (FW=PW=2). Cannot be negative.

- **`IsoYear`**

  Year in the ISO week date (FW=4, PW=∞).
  May accept years before 1 BCE or after 9999 CE, given an initial sign.

- **`IsoYearDiv100`**

  Year in the ISO week date, divided by 100 (FW=PW=2). Implies the non-negative year.

- **`IsoYearMod100`**

  Year in the ISO week date, modulo 100 (FW=PW=2). Cannot be negative.

- **`Quarter`**

  Quarter (FW=PW=1).

- **`Month`**

  Month (FW=PW=2).

- **`Day`**

  Day of the month (FW=PW=2).

- **`WeekFromSun`**

  Week number, where the week 1 starts at the first Sunday of January (FW=PW=2).

- **`WeekFromMon`**

  Week number, where the week 1 starts at the first Monday of January (FW=PW=2).

- **`IsoWeek`**

  Week number in the ISO week date (FW=PW=2).

- **`NumDaysFromSun`**

  Day of the week, where Sunday = 0 and Saturday = 6 (FW=PW=1).

- **`WeekdayFromMon`**

  Day of the week, where Monday = 1 and Sunday = 7 (FW=PW=1).

- **`Ordinal`**

  Day of the year (FW=PW=3).

- **`Hour`**

  Hour number in the 24-hour clocks (FW=PW=2).

- **`Hour12`**

  Hour number in the 12-hour clocks (FW=PW=2).

- **`Minute`**

  The number of minutes since the last whole hour (FW=PW=2).

- **`Second`**

  The number of seconds since the last whole minute (FW=PW=2).

- **`Nanosecond`**

  The number of nanoseconds since the last whole second (FW=PW=9).
  Note that this is *not* left-aligned;
  see also [`Fixed::Nanosecond`](./enum.Fixed.html#variant.Nanosecond).

- **`Timestamp`**

  The number of non-leap seconds since the midnight UTC on January 1, 1970 (FW=1, PW=∞).
  For formatting, it assumes UTC upon the absence of time zone offset.

- **`Internal`**

  Internal uses only.
  
  This item exists so that one can add additional internal-only formatting
  without breaking major compatibility (as enum variants cannot be selectively private).

#### Trait Implementations

##### `impl Clone for Numeric`

- <span id="numeric-clone"></span>`fn clone(&self) -> Numeric` — [`Numeric`](#numeric)

##### `impl Debug for Numeric`

- <span id="numeric-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Numeric`

##### `impl Hash for Numeric`

- <span id="numeric-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Numeric`

- <span id="numeric-partialeq-eq"></span>`fn eq(&self, other: &Numeric) -> bool` — [`Numeric`](#numeric)

##### `impl StructuralPartialEq for Numeric`

### `Fixed`

```rust
enum Fixed {
    ShortMonthName,
    LongMonthName,
    ShortWeekdayName,
    LongWeekdayName,
    LowerAmPm,
    UpperAmPm,
    Nanosecond,
    Nanosecond3,
    Nanosecond6,
    Nanosecond9,
    TimezoneName,
    TimezoneOffsetColon,
    TimezoneOffsetDoubleColon,
    TimezoneOffsetTripleColon,
    TimezoneOffsetColonZ,
    TimezoneOffset,
    TimezoneOffsetZ,
    RFC2822,
    RFC3339,
    Internal(InternalFixed),
}
```

Fixed-format item types.

They have their own rules of formatting and parsing.
Otherwise noted, they print in the specified cases but parse case-insensitively.

#### Variants

- **`ShortMonthName`**

  Abbreviated month names.
  
  Prints a three-letter-long name in the title case, reads the same name in any case.

- **`LongMonthName`**

  Full month names.
  
  Prints a full name in the title case, reads either a short or full name in any case.

- **`ShortWeekdayName`**

  Abbreviated day of the week names.
  
  Prints a three-letter-long name in the title case, reads the same name in any case.

- **`LongWeekdayName`**

  Full day of the week names.
  
  Prints a full name in the title case, reads either a short or full name in any case.

- **`LowerAmPm`**

  AM/PM.
  
  Prints in lower case, reads in any case.

- **`UpperAmPm`**

  AM/PM.
  
  Prints in upper case, reads in any case.

- **`Nanosecond`**

  An optional dot plus one or more digits for left-aligned nanoseconds.
  May print nothing, 3, 6 or 9 digits according to the available accuracy.
  See also [`Numeric::Nanosecond`](./enum.Numeric.html#variant.Nanosecond).

- **`Nanosecond3`**

  Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3.

- **`Nanosecond6`**

  Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6.

- **`Nanosecond9`**

  Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9.

- **`TimezoneName`**

  Timezone name.
  
  It does not support parsing, its use in the parser is an immediate failure.

- **`TimezoneOffsetColon`**

  Offset from the local time to UTC (`+09:00` or `-04:00` or `+00:00`).
  
  In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
  The offset is limited from `-24:00` to `+24:00`,
  which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

- **`TimezoneOffsetDoubleColon`**

  Offset from the local time to UTC with seconds (`+09:00:00` or `-04:00:00` or `+00:00:00`).
  
  In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
  The offset is limited from `-24:00:00` to `+24:00:00`,
  which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

- **`TimezoneOffsetTripleColon`**

  Offset from the local time to UTC without minutes (`+09` or `-04` or `+00`).
  
  In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
  The offset is limited from `-24` to `+24`,
  which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

- **`TimezoneOffsetColonZ`**

  Offset from the local time to UTC (`+09:00` or `-04:00` or `Z`).
  
  In the parser, the colon can be omitted and/or surrounded with any amount of whitespace,
  and `Z` can be either in upper case or in lower case.
  The offset is limited from `-24:00` to `+24:00`,
  which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.

- **`TimezoneOffset`**

  Same as [`TimezoneOffsetColon`](#variant.TimezoneOffsetColon) but prints no colon.
  Parsing allows an optional colon.

- **`TimezoneOffsetZ`**

  Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ) but prints no colon.
  Parsing allows an optional colon.

- **`RFC2822`**

  RFC 2822 date and time syntax. Commonly used for email and MIME date and time.

- **`RFC3339`**

  RFC 3339 & ISO 8601 date and time syntax.

- **`Internal`**

  Internal uses only.
  
  This item exists so that one can add additional internal-only formatting
  without breaking major compatibility (as enum variants cannot be selectively private).

#### Trait Implementations

##### `impl Clone for Fixed`

- <span id="fixed-clone"></span>`fn clone(&self) -> Fixed` — [`Fixed`](#fixed)

##### `impl Debug for Fixed`

- <span id="fixed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Fixed`

##### `impl Hash for Fixed`

- <span id="fixed-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Fixed`

- <span id="fixed-partialeq-eq"></span>`fn eq(&self, other: &Fixed) -> bool` — [`Fixed`](#fixed)

##### `impl StructuralPartialEq for Fixed`

### `InternalInternal`

```rust
enum InternalInternal {
    TimezoneOffsetPermissive,
    Nanosecond3NoDot,
    Nanosecond6NoDot,
    Nanosecond9NoDot,
}
```

#### Variants

- **`TimezoneOffsetPermissive`**

  Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ), but
  allows missing minutes (per [ISO 8601][iso8601]).
  
  # Panics
  
  If you try to use this for printing.
  

- **`Nanosecond3NoDot`**

  Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3 and there is no leading dot.

- **`Nanosecond6NoDot`**

  Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6 and there is no leading dot.

- **`Nanosecond9NoDot`**

  Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9 and there is no leading dot.

#### Trait Implementations

##### `impl Clone for InternalInternal`

- <span id="internalinternal-clone"></span>`fn clone(&self) -> InternalInternal` — [`InternalInternal`](#internalinternal)

##### `impl Debug for InternalInternal`

- <span id="internalinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InternalInternal`

##### `impl Hash for InternalInternal`

- <span id="internalinternal-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for InternalInternal`

- <span id="internalinternal-partialeq-eq"></span>`fn eq(&self, other: &InternalInternal) -> bool` — [`InternalInternal`](#internalinternal)

##### `impl StructuralPartialEq for InternalInternal`

### `OffsetPrecision`

```rust
enum OffsetPrecision {
    Hours,
    Minutes,
    Seconds,
    OptionalMinutes,
    OptionalSeconds,
    OptionalMinutesAndSeconds,
}
```

The precision of an offset from UTC formatting item.

#### Variants

- **`Hours`**

  Format offset from UTC as only hours. Not recommended, it is not uncommon for timezones to
  have an offset of 30 minutes, 15 minutes, etc.
  Any minutes and seconds get truncated.

- **`Minutes`**

  Format offset from UTC as hours and minutes.
  Any seconds will be rounded to the nearest minute.

- **`Seconds`**

  Format offset from UTC as hours, minutes and seconds.

- **`OptionalMinutes`**

  Format offset from UTC as hours, and optionally with minutes.
  Any seconds will be rounded to the nearest minute.

- **`OptionalSeconds`**

  Format offset from UTC as hours and minutes, and optionally seconds.

- **`OptionalMinutesAndSeconds`**

  Format offset from UTC as hours and optionally minutes and seconds.

#### Trait Implementations

##### `impl Clone for OffsetPrecision`

- <span id="offsetprecision-clone"></span>`fn clone(&self) -> OffsetPrecision` — [`OffsetPrecision`](#offsetprecision)

##### `impl Copy for OffsetPrecision`

##### `impl Debug for OffsetPrecision`

- <span id="offsetprecision-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OffsetPrecision`

##### `impl Hash for OffsetPrecision`

- <span id="offsetprecision-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for OffsetPrecision`

- <span id="offsetprecision-partialeq-eq"></span>`fn eq(&self, other: &OffsetPrecision) -> bool` — [`OffsetPrecision`](#offsetprecision)

##### `impl StructuralPartialEq for OffsetPrecision`

### `Colons`

```rust
enum Colons {
    None,
    Colon,
    Maybe,
}
```

The separator between hours and minutes in an offset.

#### Variants

- **`None`**

  No separator

- **`Colon`**

  Colon (`:`) as separator

- **`Maybe`**

  No separator when formatting, colon allowed when parsing.

#### Trait Implementations

##### `impl Clone for Colons`

- <span id="colons-clone"></span>`fn clone(&self) -> Colons` — [`Colons`](#colons)

##### `impl Copy for Colons`

##### `impl Debug for Colons`

- <span id="colons-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Colons`

##### `impl Hash for Colons`

- <span id="colons-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Colons`

- <span id="colons-partialeq-eq"></span>`fn eq(&self, other: &Colons) -> bool` — [`Colons`](#colons)

##### `impl StructuralPartialEq for Colons`

### `Item<'a>`

```rust
enum Item<'a> {
    Literal(&'a str),
    Space(&'a str),
    Numeric(Numeric, Pad),
    Fixed(Fixed),
    Error,
}
```

A single formatting item. This is used for both formatting and parsing.

#### Variants

- **`Literal`**

  A literally printed and parsed text.

- **`Space`**

  Whitespace. Prints literally but reads zero or more whitespace.

- **`Numeric`**

  Numeric item. Can be optionally padded to the maximal length (if any) when formatting;
  the parser simply ignores any padded whitespace and zeroes.

- **`Fixed`**

  Fixed-format item.

- **`Error`**

  Issues a formatting error. Used to signal an invalid format string.

#### Trait Implementations

##### `impl Clone for Item<'a>`

- <span id="item-clone"></span>`fn clone(&self) -> Item<'a>` — [`Item`](#item)

##### `impl Debug for Item<'a>`

- <span id="item-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Item<'a>`

##### `impl Hash for Item<'a>`

- <span id="item-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Item<'a>`

- <span id="item-partialeq-eq"></span>`fn eq(&self, other: &Item<'a>) -> bool` — [`Item`](#item)

##### `impl StructuralPartialEq for Item<'a>`

### `ParseErrorKind`

```rust
enum ParseErrorKind {
    OutOfRange,
    Impossible,
    NotEnough,
    Invalid,
    TooShort,
    TooLong,
    BadFormat,
}
```

The category of parse error

#### Variants

- **`OutOfRange`**

  Given field is out of permitted range.

- **`Impossible`**

  There is no possible date and time value with given set of fields.
  
  This does not include the out-of-range conditions, which are trivially invalid.
  It includes the case that there are one or more fields that are inconsistent to each other.

- **`NotEnough`**

  Given set of fields is not enough to make a requested date and time value.
  
  Note that there *may* be a case that given fields constrain the possible values so much
  that there is a unique possible value. Chrono only tries to be correct for
  most useful sets of fields however, as such constraint solving can be expensive.

- **`Invalid`**

  The input string has some invalid character sequence for given formatting items.

- **`TooShort`**

  The input string has been prematurely ended.

- **`TooLong`**

  All formatting items have been read but there is a remaining input.

- **`BadFormat`**

  There was an error on the formatting string, or there were non-supported formatting items.

#### Trait Implementations

##### `impl Clone for ParseErrorKind`

- <span id="parseerrorkind-clone"></span>`fn clone(&self) -> ParseErrorKind` — [`ParseErrorKind`](#parseerrorkind)

##### `impl Copy for ParseErrorKind`

##### `impl Debug for ParseErrorKind`

- <span id="parseerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseErrorKind`

##### `impl Hash for ParseErrorKind`

- <span id="parseerrorkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ParseErrorKind`

- <span id="parseerrorkind-partialeq-eq"></span>`fn eq(&self, other: &ParseErrorKind) -> bool` — [`ParseErrorKind`](#parseerrorkind)

##### `impl StructuralPartialEq for ParseErrorKind`

## Functions

### `parse`

```rust
fn parse<'a, I, B>(parsed: &mut super::Parsed, s: &str, items: I) -> super::ParseResult<()>
where
    I: Iterator<Item = B>,
    B: Borrow<super::Item<'a>>
```

Tries to parse given string into `parsed` with given formatting items.
Returns `Ok` when the entire string has been parsed (otherwise `parsed` should not be used).
There should be no trailing string after parsing;
use a stray [`Item::Space`](./enum.Item.html#variant.Space) to trim whitespaces.

This particular date and time parser is:

- Greedy. It will consume the longest possible prefix.
  For example, `April` is always consumed entirely when the long month name is requested;
  it equally accepts `Apr`, but prefers the longer prefix in this case.

- Padding-agnostic (for numeric items).
  The [`Pad`](./enum.Pad.html) field is completely ignored,
  so one can prepend any number of whitespace then any number of zeroes before numbers.

- (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`.

### `parse_and_remainder`

```rust
fn parse_and_remainder<'a, 'b, I, B>(parsed: &mut super::Parsed, s: &'b str, items: I) -> super::ParseResult<&'b str>
where
    I: Iterator<Item = B>,
    B: Borrow<super::Item<'a>>
```

Tries to parse given string into `parsed` with given formatting items.
Returns `Ok` with a slice of the unparsed remainder.

This particular date and time parser is:

- Greedy. It will consume the longest possible prefix.
  For example, `April` is always consumed entirely when the long month name is requested;
  it equally accepts `Apr`, but prefers the longer prefix in this case.

- Padding-agnostic (for numeric items).
  The [`Pad`](./enum.Pad.html) field is completely ignored,
  so one can prepend any number of zeroes before numbers.

- (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`.

### `num`

```rust
const fn num(numeric: Numeric) -> Item<'static>
```

### `num0`

```rust
const fn num0(numeric: Numeric) -> Item<'static>
```

### `nums`

```rust
const fn nums(numeric: Numeric) -> Item<'static>
```

### `fixed`

```rust
const fn fixed(fixed: Fixed) -> Item<'static>
```

### `internal_fixed`

```rust
const fn internal_fixed(val: InternalInternal) -> Item<'static>
```

## Type Aliases

### `ParseResult<T>`

```rust
type ParseResult<T> = Result<T, ParseError>;
```

Same as `Result<T, ParseError>`.

## Constants

### `OUT_OF_RANGE`
```rust
const OUT_OF_RANGE: ParseError;
```

### `IMPOSSIBLE`
```rust
const IMPOSSIBLE: ParseError;
```

### `NOT_ENOUGH`
```rust
const NOT_ENOUGH: ParseError;
```

### `INVALID`
```rust
const INVALID: ParseError;
```

### `TOO_SHORT`
```rust
const TOO_SHORT: ParseError;
```

### `TOO_LONG`
```rust
const TOO_LONG: ParseError;
```

### `BAD_FORMAT`
```rust
const BAD_FORMAT: ParseError;
```

