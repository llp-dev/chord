**chrono > date**

# Module: date

## Contents

**Structs**

- [`Date`](#date) - ISO 8601 calendar date with time zone.

**Constants**

- [`MAX_DATE`](#max_date) - The maximum possible `Date`.
- [`MIN_DATE`](#min_date) - The minimum possible `Date`.

---

## chrono::date::Date

*Struct*

ISO 8601 calendar date with time zone.

You almost certainly want to be using a [`NaiveDate`] instead of this type.

This type primarily exists to aid in the construction of DateTimes that
have a timezone by way of the [`TimeZone`] datelike constructors (e.g.
[`TimeZone::ymd`]).

This type should be considered ambiguous at best, due to the inherent lack
of precision required for the time zone resolution.

There are some guarantees on the usage of `Date<Tz>`:

- If properly constructed via [`TimeZone::ymd`] and others without an error,
  the corresponding local date should exist for at least a moment.
  (It may still have a gap from the offset changes.)

- The `TimeZone` is free to assign *any* [`Offset`](crate::offset::Offset) to the
  local date, as long as that offset did occur in given day.

  For example, if `2015-03-08T01:59-08:00` is followed by `2015-03-08T03:00-07:00`,
  it may produce either `2015-03-08-08:00` or `2015-03-08-07:00`
  but *not* `2015-03-08+00:00` and others.

- Once constructed as a full `DateTime`, [`DateTime::date`] and other associated
  methods should return those for the original `Date`. For example, if `dt =
  tz.ymd_opt(y,m,d).unwrap().hms(h,n,s)` were valid, `dt.date() == tz.ymd_opt(y,m,d).unwrap()`.

- The date is timezone-agnostic up to one day (i.e. practically always),
  so the local date and UTC date should be equal for most cases
  even though the raw calculation between `NaiveDate` and `TimeDelta` may not.

**Generic Parameters:**
- Tz

**Methods:**

- `fn from_utc(date: NaiveDate, offset: <Tz as >::Offset) -> Date<Tz>` - Makes a new `Date` with given *UTC* date and offset.
- `fn and_time(self: &Self, time: NaiveTime) -> Option<DateTime<Tz>>` - Makes a new `DateTime` from the current date and given `NaiveTime`.
- `fn and_hms(self: &Self, hour: u32, min: u32, sec: u32) -> DateTime<Tz>` - Makes a new `DateTime` from the current date, hour, minute and second.
- `fn and_hms_opt(self: &Self, hour: u32, min: u32, sec: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute and second.
- `fn and_hms_milli(self: &Self, hour: u32, min: u32, sec: u32, milli: u32) -> DateTime<Tz>` - Makes a new `DateTime` from the current date, hour, minute, second and millisecond.
- `fn and_hms_milli_opt(self: &Self, hour: u32, min: u32, sec: u32, milli: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute, second and millisecond.
- `fn and_hms_micro(self: &Self, hour: u32, min: u32, sec: u32, micro: u32) -> DateTime<Tz>` - Makes a new `DateTime` from the current date, hour, minute, second and microsecond.
- `fn and_hms_micro_opt(self: &Self, hour: u32, min: u32, sec: u32, micro: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute, second and microsecond.
- `fn and_hms_nano(self: &Self, hour: u32, min: u32, sec: u32, nano: u32) -> DateTime<Tz>` - Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.
- `fn and_hms_nano_opt(self: &Self, hour: u32, min: u32, sec: u32, nano: u32) -> Option<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.
- `fn succ(self: &Self) -> Date<Tz>` - Makes a new `Date` for the next date.
- `fn succ_opt(self: &Self) -> Option<Date<Tz>>` - Makes a new `Date` for the next date.
- `fn pred(self: &Self) -> Date<Tz>` - Makes a new `Date` for the prior date.
- `fn pred_opt(self: &Self) -> Option<Date<Tz>>` - Makes a new `Date` for the prior date.
- `fn offset(self: &Self) -> &<Tz as >::Offset` - Retrieves an associated offset from UTC.
- `fn timezone(self: &Self) -> Tz` - Retrieves an associated time zone.
- `fn with_timezone<Tz2>(self: &Self, tz: &Tz2) -> Date<Tz2>` - Changes the associated time zone.
- `fn checked_add_signed(self: Self, rhs: TimeDelta) -> Option<Date<Tz>>` - Adds given `TimeDelta` to the current date.
- `fn checked_sub_signed(self: Self, rhs: TimeDelta) -> Option<Date<Tz>>` - Subtracts given `TimeDelta` from the current date.
- `fn signed_duration_since<Tz2>(self: Self, rhs: Date<Tz2>) -> TimeDelta` - Subtracts another `Date` from the current date.
- `fn naive_utc(self: &Self) -> NaiveDate` - Returns a view to the naive UTC date.
- `fn naive_local(self: &Self) -> NaiveDate` - Returns a view to the naive local date.
- `fn years_since(self: &Self, base: Self) -> Option<u32>` - Returns the number of whole years from the given `base` until `self`.

**Traits:** Copy, Send, Eq

**Trait Implementations:**

- **Sub**
  - `fn sub(self: Self, rhs: Date<Tz>) -> TimeDelta`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: TimeDelta)`
- **Ord**
  - `fn cmp(self: &Self, other: &Date<Tz>) -> Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Date<Tz>`
- **Sub**
  - `fn sub(self: Self, rhs: TimeDelta) -> Date<Tz>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Date<Tz2>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Date<Tz>) -> Option<Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
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
  - `fn with_year(self: &Self, year: i32) -> Option<Date<Tz>>`
  - `fn with_month(self: &Self, month: u32) -> Option<Date<Tz>>`
  - `fn with_month0(self: &Self, month0: u32) -> Option<Date<Tz>>`
  - `fn with_day(self: &Self, day: u32) -> Option<Date<Tz>>`
  - `fn with_day0(self: &Self, day0: u32) -> Option<Date<Tz>>`
  - `fn with_ordinal(self: &Self, ordinal: u32) -> Option<Date<Tz>>`
  - `fn with_ordinal0(self: &Self, ordinal0: u32) -> Option<Date<Tz>>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: TimeDelta)`
- **Add**
  - `fn add(self: Self, rhs: TimeDelta) -> Date<Tz>`



## chrono::date::MAX_DATE

*Constant*: `Date<crate::offset::Utc>`

The maximum possible `Date`.



## chrono::date::MIN_DATE

*Constant*: `Date<crate::offset::Utc>`

The minimum possible `Date`.



