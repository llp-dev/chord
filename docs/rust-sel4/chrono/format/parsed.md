**chrono > format > parsed**

# Module: format::parsed

## Contents

**Structs**

- [`Parsed`](#parsed) - A type to hold parsed fields of date and time that can check all fields are consistent.

---

## chrono::format::parsed::Parsed

*Struct*

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

[RFC 2822 date and time format]: https://tools.ietf.org/html/rfc2822#section-3.3

# Example

Let's see how `Parsed` correctly detects the second RFC 2822 string from before is inconsistent.

```
# #[cfg(feature = "alloc")] {
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
# }
# Ok::<(), chrono::ParseError>(())
```

The same using chrono's built-in parser for RFC 2822 (the [RFC2822 formatting item]) and
[`format::parse()`] showing how to inspect a field on failure.

[RFC2822 formatting item]: crate::format::Fixed::RFC2822
[`format::parse()`]: crate::format::parse()

```
# #[cfg(feature = "alloc")] {
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
# }
# Ok::<(), chrono::ParseError>(())
```

**Methods:**

- `fn new() -> Parsed` - Returns the initial value of parsed parts.
- `fn set_year(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`year`](Parsed::year) field to the given value.
- `fn set_year_div_100(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`year_div_100`](Parsed::year_div_100) field to the given value.
- `fn set_year_mod_100(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`year_mod_100`](Parsed::year_mod_100) field to the given value.
- `fn set_isoyear(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`isoyear`](Parsed::isoyear) field, that is part of an [ISO 8601 week date], to the
- `fn set_isoyear_div_100(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`isoyear_div_100`](Parsed::isoyear_div_100) field, that is part of an
- `fn set_isoyear_mod_100(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`isoyear_mod_100`](Parsed::isoyear_mod_100) field, that is part of an
- `fn set_quarter(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`quarter`](Parsed::quarter) field to the given value.
- `fn set_month(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`month`](Parsed::month) field to the given value.
- `fn set_week_from_sun(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`week_from_sun`](Parsed::week_from_sun) week number field to the given value.
- `fn set_week_from_mon(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`week_from_mon`](Parsed::week_from_mon) week number field to the given value.
- `fn set_isoweek(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`isoweek`](Parsed::isoweek) field for an [ISO 8601 week date] to the given value.
- `fn set_weekday(self: & mut Self, value: Weekday) -> ParseResult<()>` - Set the [`weekday`](Parsed::weekday) field to the given value.
- `fn set_ordinal(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`ordinal`](Parsed::ordinal) (day of the year) field to the given value.
- `fn set_day(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`day`](Parsed::day) of the month field to the given value.
- `fn set_ampm(self: & mut Self, value: bool) -> ParseResult<()>` - Set the [`hour_div_12`](Parsed::hour_div_12) am/pm field to the given value.
- `fn set_hour12(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`hour_mod_12`](Parsed::hour_mod_12) field, for the hour number in 12-hour clocks,
- `fn set_hour(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`hour_div_12`](Parsed::hour_div_12) and [`hour_mod_12`](Parsed::hour_mod_12)
- `fn set_minute(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`minute`](Parsed::minute) field to the given value.
- `fn set_second(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`second`](Parsed::second) field to the given value.
- `fn set_nanosecond(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`nanosecond`](Parsed::nanosecond) field to the given value.
- `fn set_timestamp(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`timestamp`](Parsed::timestamp) field to the given value.
- `fn set_offset(self: & mut Self, value: i64) -> ParseResult<()>` - Set the [`offset`](Parsed::offset) field to the given value.
- `fn to_naive_date(self: &Self) -> ParseResult<NaiveDate>` - Returns a parsed naive date out of given fields.
- `fn to_naive_time(self: &Self) -> ParseResult<NaiveTime>` - Returns a parsed naive time out of given fields.
- `fn to_naive_datetime_with_offset(self: &Self, offset: i32) -> ParseResult<NaiveDateTime>` - Returns a parsed naive date and time out of given fields, except for the offset field.
- `fn to_fixed_offset(self: &Self) -> ParseResult<FixedOffset>` - Returns a parsed fixed time zone offset out of given fields.
- `fn to_datetime(self: &Self) -> ParseResult<DateTime<FixedOffset>>` - Returns a parsed timezone-aware date and time out of given fields.
- `fn to_datetime_with_timezone<Tz>(self: &Self, tz: &Tz) -> ParseResult<DateTime<Tz>>` - Returns a parsed timezone-aware date and time out of given fields,
- `fn year(self: &Self) -> Option<i32>` - Get the `year` field if set.
- `fn year_div_100(self: &Self) -> Option<i32>` - Get the `year_div_100` field if set.
- `fn year_mod_100(self: &Self) -> Option<i32>` - Get the `year_mod_100` field if set.
- `fn isoyear(self: &Self) -> Option<i32>` - Get the `isoyear` field that is part of an [ISO 8601 week date] if set.
- `fn isoyear_div_100(self: &Self) -> Option<i32>` - Get the `isoyear_div_100` field that is part of an [ISO 8601 week date] if set.
- `fn isoyear_mod_100(self: &Self) -> Option<i32>` - Get the `isoyear_mod_100` field that is part of an [ISO 8601 week date] if set.
- `fn quarter(self: &Self) -> Option<u32>` - Get the `quarter` field if set.
- `fn month(self: &Self) -> Option<u32>` - Get the `month` field if set.
- `fn week_from_sun(self: &Self) -> Option<u32>` - Get the `week_from_sun` field if set.
- `fn week_from_mon(self: &Self) -> Option<u32>` - Get the `week_from_mon` field if set.
- `fn isoweek(self: &Self) -> Option<u32>` - Get the `isoweek` field that is part of an [ISO 8601 week date] if set.
- `fn weekday(self: &Self) -> Option<Weekday>` - Get the `weekday` field if set.
- `fn ordinal(self: &Self) -> Option<u32>` - Get the `ordinal` (day of the year) field if set.
- `fn day(self: &Self) -> Option<u32>` - Get the `day` of the month field if set.
- `fn hour_div_12(self: &Self) -> Option<u32>` - Get the `hour_div_12` field (am/pm) if set.
- `fn hour_mod_12(self: &Self) -> Option<u32>` - Get the `hour_mod_12` field if set.
- `fn minute(self: &Self) -> Option<u32>` - Get the `minute` field if set.
- `fn second(self: &Self) -> Option<u32>` - Get the `second` field if set.
- `fn nanosecond(self: &Self) -> Option<u32>` - Get the `nanosecond` field if set.
- `fn timestamp(self: &Self) -> Option<i64>` - Get the `timestamp` field if set.
- `fn offset(self: &Self) -> Option<i32>` - Get the `offset` field if set.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Parsed) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Parsed`
- **Clone**
  - `fn clone(self: &Self) -> Parsed`



