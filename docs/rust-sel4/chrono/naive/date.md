**chrono > naive > date**

# Module: naive::date

## Contents

**Structs**

- [`NaiveDate`](#naivedate) - ISO 8601 calendar date without timezone.
- [`NaiveDateDaysIterator`](#naivedatedaysiterator) - Iterator over `NaiveDate` with a step size of one day.
- [`NaiveDateWeeksIterator`](#naivedateweeksiterator) - Iterator over `NaiveDate` with a step size of one week.

**Constants**

- [`MAX_DATE`](#max_date) - The maximum possible `NaiveDate` (December 31, 262143 CE).
- [`MIN_DATE`](#min_date) - The minimum possible `NaiveDate` (January 1, 262145 BCE).

---

## chrono::naive::date::MAX_DATE

*Constant*: `NaiveDate`

The maximum possible `NaiveDate` (December 31, 262143 CE).



## chrono::naive::date::MIN_DATE

*Constant*: `NaiveDate`

The minimum possible `NaiveDate` (January 1, 262145 BCE).



## chrono::naive::date::NaiveDate

*Struct*

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
  use the [`Datelike::year_ce`] method.

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
[`Datelike::iso_week`] and [`Datelike::weekday`] methods can be used to get the corresponding
week date.

# Ordinal Date

The ISO 8601 **ordinal date** is a pair of year number and day of the year ("ordinal").
The ordinal number ranges from 1 to 365 or 366 depending on the year.
The year number is the same as that of the [calendar date](#calendar-date).

This is currently the internal format of Chrono's date types.

[proleptic Gregorian date]: crate::NaiveDate#calendar-date

**Methods:**

- `fn from_ymd(year: i32, month: u32, day: u32) -> NaiveDate` - Makes a new `NaiveDate` from the [calendar date](#calendar-date)
- `fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` from the [calendar date](#calendar-date)
- `fn from_yo(year: i32, ordinal: u32) -> NaiveDate` - Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)
- `fn from_yo_opt(year: i32, ordinal: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)
- `fn from_isoywd(year: i32, week: u32, weekday: Weekday) -> NaiveDate` - Makes a new `NaiveDate` from the [ISO week date](#week-date)
- `fn from_isoywd_opt(year: i32, week: u32, weekday: Weekday) -> Option<NaiveDate>` - Makes a new `NaiveDate` from the [ISO week date](#week-date)
- `fn from_num_days_from_ce(days: i32) -> NaiveDate` - Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with
- `fn from_num_days_from_ce_opt(days: i32) -> Option<NaiveDate>` - Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with
- `fn from_epoch_days(days: i32) -> Option<NaiveDate>` - Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with
- `fn from_weekday_of_month(year: i32, month: u32, weekday: Weekday, n: u8) -> NaiveDate` - Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week
- `fn from_weekday_of_month_opt(year: i32, month: u32, weekday: Weekday, n: u8) -> Option<NaiveDate>` - Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week
- `fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDate>` - Parses a string with the specified format string and returns a new `NaiveDate`.
- `fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDate, &'a str)>` - Parses a string from a user-specified format into a new `NaiveDate` value, and a slice with
- `fn checked_add_months(self: Self, months: Months) -> Option<Self>` - Add a duration in [`Months`] to the date
- `fn checked_sub_months(self: Self, months: Months) -> Option<Self>` - Subtract a duration in [`Months`] from the date
- `fn checked_add_days(self: Self, days: Days) -> Option<Self>` - Add a duration in [`Days`] to the date
- `fn checked_sub_days(self: Self, days: Days) -> Option<Self>` - Subtract a duration in [`Days`] from the date
- `fn and_time(self: &Self, time: NaiveTime) -> NaiveDateTime` - Makes a new `NaiveDateTime` from the current date and given `NaiveTime`.
- `fn and_hms(self: &Self, hour: u32, min: u32, sec: u32) -> NaiveDateTime` - Makes a new `NaiveDateTime` from the current date, hour, minute and second.
- `fn and_hms_opt(self: &Self, hour: u32, min: u32, sec: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` from the current date, hour, minute and second.
- `fn and_hms_milli(self: &Self, hour: u32, min: u32, sec: u32, milli: u32) -> NaiveDateTime` - Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.
- `fn and_hms_milli_opt(self: &Self, hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.
- `fn and_hms_micro(self: &Self, hour: u32, min: u32, sec: u32, micro: u32) -> NaiveDateTime` - Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.
- `fn and_hms_micro_opt(self: &Self, hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.
- `fn and_hms_nano(self: &Self, hour: u32, min: u32, sec: u32, nano: u32) -> NaiveDateTime` - Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.
- `fn and_hms_nano_opt(self: &Self, hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveDateTime>` - Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.
- `fn succ(self: &Self) -> NaiveDate` - Makes a new `NaiveDate` for the next calendar date.
- `fn succ_opt(self: &Self) -> Option<NaiveDate>` - Makes a new `NaiveDate` for the next calendar date.
- `fn pred(self: &Self) -> NaiveDate` - Makes a new `NaiveDate` for the previous calendar date.
- `fn pred_opt(self: &Self) -> Option<NaiveDate>` - Makes a new `NaiveDate` for the previous calendar date.
- `fn checked_add_signed(self: Self, rhs: TimeDelta) -> Option<NaiveDate>` - Adds the number of whole days in the given `TimeDelta` to the current date.
- `fn checked_sub_signed(self: Self, rhs: TimeDelta) -> Option<NaiveDate>` - Subtracts the number of whole days in the given `TimeDelta` from the current date.
- `fn signed_duration_since(self: Self, rhs: Self) -> TimeDelta` - Subtracts another `NaiveDate` from the current date.
- `fn abs_diff(self: Self, rhs: Self) -> Days` - Returns the absolute difference between two `NaiveDate`s measured as the number of days.
- `fn years_since(self: &Self, base: Self) -> Option<u32>` - Returns the number of whole years from the given `base` until `self`.
- `fn iter_days(self: &Self) -> NaiveDateDaysIterator` - Returns an iterator that steps by days across all representable dates.
- `fn iter_weeks(self: &Self) -> NaiveDateWeeksIterator` - Returns an iterator that steps by weeks across all representable dates.
- `fn week(self: &Self, start: Weekday) -> NaiveWeek` - Returns the [`NaiveWeek`] that the date belongs to, starting with the [`Weekday`]
- `fn leap_year(self: &Self) -> bool` - Returns `true` if this is a leap year.
- `fn to_epoch_days(self: &Self) -> i32` - Counts the days in the proleptic Gregorian calendar, with January 1, Year 1970 as day 0.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &NaiveDate) -> $crate::cmp::Ordering`
- **Sub**
  - `fn sub(self: Self, months: Months) -> <Self as >::Output`
- **From**
  - `fn from(naive_datetime: NaiveDateTime) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> NaiveDate`
- **Add**
  - `fn add(self: Self, days: Days) -> <Self as >::Output`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **Sub**
  - `fn sub(self: Self, days: Days) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NaiveDate) -> $crate::option::Option<$crate::cmp::Ordering>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: TimeDelta)`
- **Sub**
  - `fn sub(self: Self, rhs: NaiveDate) -> TimeDelta`
- **PartialEq**
  - `fn eq(self: &Self, other: &NaiveDate) -> bool`
- **Default**
  - `fn default() -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: TimeDelta)`
- **Add**
  - `fn add(self: Self, rhs: TimeDelta) -> NaiveDate`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FromStr**
  - `fn from_str(s: &str) -> ParseResult<NaiveDate>`
- **Sub**
  - `fn sub(self: Self, rhs: TimeDelta) -> NaiveDate`
- **Datelike**
  - `fn year(self: &Self) -> i32` - Returns the year number in the [calendar date](#calendar-date).
  - `fn month(self: &Self) -> u32` - Returns the month number starting from 1.
  - `fn month0(self: &Self) -> u32` - Returns the month number starting from 0.
  - `fn day(self: &Self) -> u32` - Returns the day of month starting from 1.
  - `fn day0(self: &Self) -> u32` - Returns the day of month starting from 0.
  - `fn ordinal(self: &Self) -> u32` - Returns the day of year starting from 1.
  - `fn ordinal0(self: &Self) -> u32` - Returns the day of year starting from 0.
  - `fn weekday(self: &Self) -> Weekday` - Returns the day of week.
  - `fn iso_week(self: &Self) -> IsoWeek`
  - `fn with_year(self: &Self, year: i32) -> Option<NaiveDate>` - Makes a new `NaiveDate` with the year number changed, while keeping the same month and day.
  - `fn with_month(self: &Self, month: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` with the month number (starting from 1) changed.
  - `fn with_month0(self: &Self, month0: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` with the month number (starting from 0) changed.
  - `fn with_day(self: &Self, day: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` with the day of month (starting from 1) changed.
  - `fn with_day0(self: &Self, day0: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` with the day of month (starting from 0) changed.
  - `fn with_ordinal(self: &Self, ordinal: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` with the day of year (starting from 1) changed.
  - `fn with_ordinal0(self: &Self, ordinal0: u32) -> Option<NaiveDate>` - Makes a new `NaiveDate` with the day of year (starting from 0) changed.
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **Add**
  - `fn add(self: Self, months: Months) -> <Self as >::Output`



## chrono::naive::date::NaiveDateDaysIterator

*Struct*

Iterator over `NaiveDate` with a step size of one day.

**Traits:** Copy, FusedIterator, ExactSizeIterator, Eq

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NaiveDateDaysIterator) -> $crate::option::Option<$crate::cmp::Ordering>`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Ord**
  - `fn cmp(self: &Self, other: &NaiveDateDaysIterator) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &NaiveDateDaysIterator) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NaiveDateDaysIterator`



## chrono::naive::date::NaiveDateWeeksIterator

*Struct*

Iterator over `NaiveDate` with a step size of one week.

**Traits:** Eq, Copy, ExactSizeIterator, FusedIterator

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &NaiveDateWeeksIterator) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NaiveDateWeeksIterator`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NaiveDateWeeksIterator) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &NaiveDateWeeksIterator) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



