*[chrono](../index.md) / [date](index.md)*

---

# Module `date`

ISO 8601 calendar date with time zone.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Date`](#date) | struct | ISO 8601 calendar date with time zone. |
| [`map_local`](#map-local) | fn | Maps the local date to other date with given conversion function. |
| [`MIN_DATE`](#min-date) | const | The minimum possible `Date`. |
| [`MAX_DATE`](#max-date) | const | The maximum possible `Date`. |

## Structs

### `Date<Tz: TimeZone>`

```rust
struct Date<Tz: TimeZone> {
    date: crate::naive::NaiveDate,
    offset: <Tz as >::Offset,
}
```

ISO 8601 calendar date with time zone.

You almost certainly want to be using a [`NaiveDate`](../naive/date/index.md) instead of this type.

This type primarily exists to aid in the construction of DateTimes that
have a timezone by way of the [`TimeZone`](../offset/index.md) datelike constructors (e.g.
`TimeZone::ymd`).

This type should be considered ambiguous at best, due to the inherent lack
of precision required for the time zone resolution.

There are some guarantees on the usage of `Date<Tz>`:

- If properly constructed via `TimeZone::ymd` and others without an error,
  the corresponding local date should exist for at least a moment.
  (It may still have a gap from the offset changes.)

- The `TimeZone` is free to assign *any* [`Offset`](crate::offset::Offset) to the
  local date, as long as that offset did occur in given day.

  For example, if `2015-03-08T01:59-08:00` is followed by `2015-03-08T03:00-07:00`,
  it may produce either `2015-03-08-08:00` or `2015-03-08-07:00`
  but *not* `2015-03-08+00:00` and others.

- Once constructed as a full `DateTime`, `DateTime::date` and other associated
  methods should return those for the original `Date`. For example, if `dt =
  tz.ymd_opt(y,m,d).unwrap().hms(h,n,s)` were valid, `dt.date() == tz.ymd_opt(y,m,d).unwrap()`.

- The date is timezone-agnostic up to one day (i.e. practically always),
  so the local date and UTC date should be equal for most cases
  even though the raw calculation between `NaiveDate` and `TimeDelta` may not.

#### Implementations

- <span id="date-from-utc"></span>`fn from_utc(date: NaiveDate, offset: <Tz as >::Offset) -> Date<Tz>` — [`NaiveDate`](../naive/date/index.md#naivedate), [`TimeZone`](../offset/index.md#timezone), [`Date`](#date)

  Makes a new `Date` with given *UTC* date and offset.

  The local date should be constructed via the `TimeZone` trait.

- <span id="date-and-time"></span>`fn and_time(&self, time: NaiveTime) -> Option<DateTime<Tz>>` — [`NaiveTime`](../naive/time/index.md#naivetime), [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date and given `NaiveTime`.

  The offset in the current date is preserved.

  

  Returns `None` on invalid datetime.

- <span id="date-and-hms"></span>`fn and_hms(&self, hour: u32, min: u32, sec: u32) -> DateTime<Tz>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute and second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute and/or second.

- <span id="date-and-hms-opt"></span>`fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<DateTime<Tz>>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute and second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute and/or second.

- <span id="date-and-hms-milli"></span>`fn and_hms_milli(&self, hour: u32, min: u32, sec: u32, milli: u32) -> DateTime<Tz>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and millisecond.

  The millisecond part can exceed 1,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="date-and-hms-milli-opt"></span>`fn and_hms_milli_opt(&self, hour: u32, min: u32, sec: u32, milli: u32) -> Option<DateTime<Tz>>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and millisecond.

  The millisecond part can exceed 1,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute, second and/or millisecond.

- <span id="date-and-hms-micro"></span>`fn and_hms_micro(&self, hour: u32, min: u32, sec: u32, micro: u32) -> DateTime<Tz>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and microsecond.

  The microsecond part can exceed 1,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute, second and/or microsecond.

- <span id="date-and-hms-micro-opt"></span>`fn and_hms_micro_opt(&self, hour: u32, min: u32, sec: u32, micro: u32) -> Option<DateTime<Tz>>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and microsecond.

  The microsecond part can exceed 1,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute, second and/or microsecond.

- <span id="date-and-hms-nano"></span>`fn and_hms_nano(&self, hour: u32, min: u32, sec: u32, nano: u32) -> DateTime<Tz>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.

  The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="date-and-hms-nano-opt"></span>`fn and_hms_nano_opt(&self, hour: u32, min: u32, sec: u32, nano: u32) -> Option<DateTime<Tz>>` — [`DateTime`](../datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.

  The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute, second and/or nanosecond.

- <span id="date-succ"></span>`fn succ(&self) -> Date<Tz>` — [`Date`](#date)

  Makes a new `Date` for the next date.

  

  Panics when `self` is the last representable date.

- <span id="date-succ-opt"></span>`fn succ_opt(&self) -> Option<Date<Tz>>` — [`Date`](#date)

  Makes a new `Date` for the next date.

  

  Returns `None` when `self` is the last representable date.

- <span id="date-pred"></span>`fn pred(&self) -> Date<Tz>` — [`Date`](#date)

  Makes a new `Date` for the prior date.

  

  Panics when `self` is the first representable date.

- <span id="date-pred-opt"></span>`fn pred_opt(&self) -> Option<Date<Tz>>` — [`Date`](#date)

  Makes a new `Date` for the prior date.

  

  Returns `None` when `self` is the first representable date.

- <span id="date-offset"></span>`fn offset(&self) -> &<Tz as >::Offset` — [`TimeZone`](../offset/index.md#timezone)

  Retrieves an associated offset from UTC.

- <span id="date-timezone"></span>`fn timezone(&self) -> Tz`

  Retrieves an associated time zone.

- <span id="date-with-timezone"></span>`fn with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> Date<Tz2>` — [`Date`](#date)

  Changes the associated time zone.

  This does not change the actual `Date` (but will change the string representation).

- <span id="date-checked-add-signed"></span>`fn checked_add_signed(self, rhs: TimeDelta) -> Option<Date<Tz>>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`Date`](#date)

  Adds given `TimeDelta` to the current date.

  

  Returns `None` when it will result in overflow.

- <span id="date-checked-sub-signed"></span>`fn checked_sub_signed(self, rhs: TimeDelta) -> Option<Date<Tz>>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`Date`](#date)

  Subtracts given `TimeDelta` from the current date.

  

  Returns `None` when it will result in overflow.

- <span id="date-signed-duration-since"></span>`fn signed_duration_since<Tz2: TimeZone>(self, rhs: Date<Tz2>) -> TimeDelta` — [`Date`](#date), [`TimeDelta`](../time_delta/index.md#timedelta)

  Subtracts another `Date` from the current date.

  Returns a `TimeDelta` of integral numbers.

  

  This does not overflow or underflow at all,

  as all possible output fits in the range of `TimeDelta`.

- <span id="date-naive-utc"></span>`fn naive_utc(&self) -> NaiveDate` — [`NaiveDate`](../naive/date/index.md#naivedate)

  Returns a view to the naive UTC date.

- <span id="date-naive-local"></span>`fn naive_local(&self) -> NaiveDate` — [`NaiveDate`](../naive/date/index.md#naivedate)

  Returns a view to the naive local date.

  

  This is technically the same as [`naive_utc`](#method.naive_utc)

  because the offset is restricted to never exceed one day,

  but provided for the consistency.

- <span id="date-years-since"></span>`fn years_since(&self, base: Self) -> Option<u32>`

  Returns the number of whole years from the given `base` until `self`.

- <span id="date-const-min-utc"></span>`const MIN_UTC: Date<Utc>`

- <span id="date-const-max-utc"></span>`const MAX_UTC: Date<Utc>`

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for Date<Tz>`

- <span id="date-add-type-output"></span>`type Output = Date<Tz>`

- <span id="date-add"></span>`fn add(self, rhs: TimeDelta) -> Date<Tz>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`Date`](#date)

##### `impl<Tz: TimeZone> AddAssign for Date<Tz>`

- <span id="date-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl<Tz: clone::Clone + TimeZone> Clone for Date<Tz>`

- <span id="date-clone"></span>`fn clone(&self) -> Date<Tz>` — [`Date`](#date)

##### `impl<Tz: TimeZone> Copy for Date<Tz>`

##### `impl<Tz: TimeZone> Datelike for Date<Tz>`

- <span id="date-datelike-year"></span>`fn year(&self) -> i32`

- <span id="date-datelike-month"></span>`fn month(&self) -> u32`

- <span id="date-datelike-month0"></span>`fn month0(&self) -> u32`

- <span id="date-datelike-day"></span>`fn day(&self) -> u32`

- <span id="date-datelike-day0"></span>`fn day0(&self) -> u32`

- <span id="date-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

- <span id="date-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

- <span id="date-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](../weekday/index.md#weekday)

- <span id="date-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](../naive/isoweek/index.md#isoweek)

- <span id="date-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<Date<Tz>>` — [`Date`](#date)

- <span id="date-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<Date<Tz>>` — [`Date`](#date)

- <span id="date-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<Date<Tz>>` — [`Date`](#date)

- <span id="date-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<Date<Tz>>` — [`Date`](#date)

- <span id="date-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<Date<Tz>>` — [`Date`](#date)

- <span id="date-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<Date<Tz>>` — [`Date`](#date)

- <span id="date-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<Date<Tz>>` — [`Date`](#date)

##### `impl<Tz: TimeZone> Debug for Date<Tz>`

- <span id="date-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Tz: TimeZone> Display for Date<Tz>`

- <span id="date-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Tz: TimeZone> Eq for Date<Tz>`

##### `impl<Tz: TimeZone> Hash for Date<Tz>`

- <span id="date-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<Tz: TimeZone> Ord for Date<Tz>`

- <span id="date-ord-cmp"></span>`fn cmp(&self, other: &Date<Tz>) -> Ordering` — [`Date`](#date)

##### `impl<Tz: TimeZone, Tz2: TimeZone> PartialEq for Date<Tz>`

- <span id="date-partialeq-eq"></span>`fn eq(&self, other: &Date<Tz2>) -> bool` — [`Date`](#date)

##### `impl<Tz: TimeZone> PartialOrd for Date<Tz>`

- <span id="date-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Date<Tz>) -> Option<Ordering>` — [`Date`](#date)

##### `impl<Tz: TimeZone> Send for Date<Tz>`

##### `impl<Tz: TimeZone> Sub for Date<Tz>`

- <span id="date-sub-type-output"></span>`type Output = Date<Tz>`

- <span id="date-sub"></span>`fn sub(self, rhs: TimeDelta) -> Date<Tz>` — [`TimeDelta`](../time_delta/index.md#timedelta), [`Date`](#date)

##### `impl<Tz: TimeZone> SubAssign for Date<Tz>`

- <span id="date-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](../time_delta/index.md#timedelta)

##### `impl ToString for Date<Tz>`

- <span id="date-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `map_local`

```rust
fn map_local<Tz: TimeZone, F>(d: &Date<Tz>, f: F) -> Option<Date<Tz>>
where
    F: FnMut(crate::naive::NaiveDate) -> Option<crate::naive::NaiveDate>
```

Maps the local date to other date with given conversion function.

## Constants

### `MIN_DATE`
```rust
const MIN_DATE: Date<crate::offset::Utc>;
```

The minimum possible `Date`.

### `MAX_DATE`
```rust
const MAX_DATE: Date<crate::offset::Utc>;
```

The maximum possible `Date`.

