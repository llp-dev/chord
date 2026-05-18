**chrono > offset**

# Module: offset

## Contents

**Enums**

- [`LocalResult`](#localresult) - Old name of [`MappedLocalTime`]. See that type for more documentation.

**Traits**

- [`Offset`](#offset) - The offset from the local time to UTC.
- [`TimeZone`](#timezone) - The time zone.

**Type Aliases**

- [`MappedLocalTime`](#mappedlocaltime) - The result of mapping a local time to a concrete instant in a given time zone.

---

## chrono::offset::LocalResult

*Enum*

Old name of [`MappedLocalTime`]. See that type for more documentation.

**Generic Parameters:**
- T

**Variants:**
- `Single(T)` - The local time maps to a single unique result.
- `Ambiguous(T, T)` - The local time is _ambiguous_ because there is a _fold_ in the local time.
- `None` - The local time does not exist because there is a _gap_ in the local time.

**Methods:**

- `fn unwrap(self: Self) -> T` - Returns a single unique conversion result or panics.
- `fn single(self: Self) -> Option<T>` - Returns `Some` if the time zone mapping has a single result.
- `fn earliest(self: Self) -> Option<T>` - Returns the earliest possible result of the time zone mapping.
- `fn latest(self: Self) -> Option<T>` - Returns the latest possible result of the time zone mapping.
- `fn map<U, F>(self: Self, f: F) -> MappedLocalTime<U>` - Maps a `MappedLocalTime<T>` into `MappedLocalTime<U>` with given function.
- `fn and_time(self: Self, time: NaiveTime) -> MappedLocalTime<DateTime<Tz>>` - Makes a new `DateTime` from the current date and given `NaiveTime`.
- `fn and_hms_opt(self: Self, hour: u32, min: u32, sec: u32) -> MappedLocalTime<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute and second.
- `fn and_hms_milli_opt(self: Self, hour: u32, min: u32, sec: u32, milli: u32) -> MappedLocalTime<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute, second and millisecond.
- `fn and_hms_micro_opt(self: Self, hour: u32, min: u32, sec: u32, micro: u32) -> MappedLocalTime<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute, second and microsecond.
- `fn and_hms_nano_opt(self: Self, hour: u32, min: u32, sec: u32, nano: u32) -> MappedLocalTime<DateTime<Tz>>` - Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> LocalResult<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &LocalResult<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## chrono::offset::MappedLocalTime

*Type Alias*: `LocalResult<T>`

The result of mapping a local time to a concrete instant in a given time zone.

The calculation to go from a local time (wall clock time) to an instant in UTC can end up in
three cases:
* A single, simple result.
* An ambiguous result when the clock is turned backwards during a transition due to for example
  DST.
* No result when the clock is turned forwards during a transition due to for example DST.

<div class="warning">

In wasm, when using [`Local`], only the [`LocalResult::Single`] variant is returned.
Specifically:

* When the clock is turned backwards, where `Ambiguous(earliest, latest)` would be expected,
  `Single(earliest)` is returned instead.
* When the clock is turned forwards, where `None` would be expected, `Single(t)` is returned,
  with `t` being the requested local time represented as though there is no transition on that
  day (i.e. still "summer time")

This is caused because of limitations in the JavaScript
[`Date`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
API, which always parses a local time as a single, valid time - even for an
input which describes a nonexistent or ambiguous time.

See further discussion and workarounds in <https://github.com/chronotope/chrono/issues/1701>.

</div>

When the clock is turned backwards it creates a _fold_ in local time, during which the local
time is _ambiguous_. When the clock is turned forwards it creates a _gap_ in local time, during
which the local time is _missing_, or does not exist.

Chrono does not return a default choice or invalid data during time zone transitions, but has
the `MappedLocalTime` type to help deal with the result correctly.

The type of `T` is usually a [`DateTime`] but may also be only an offset.



## chrono::offset::Offset

*Trait*

The offset from the local time to UTC.

**Methods:**

- `fix`: Returns the fixed offset from UTC to the local time stored.



## chrono::offset::TimeZone

*Trait*

The time zone.

The methods here are the primary constructors for the [`DateTime`] type.

**Methods:**

- `Offset`: An associated offset type.
- `with_ymd_and_hms`: Make a new `DateTime` from year, month, day, time components and current time zone.
- `ymd`: Makes a new `Date` from year, month, day and the current time zone.
- `ymd_opt`: Makes a new `Date` from year, month, day and the current time zone.
- `yo`: Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.
- `yo_opt`: Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.
- `isoywd`: Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and
- `isoywd_opt`: Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and
- `timestamp`: Makes a new `DateTime` from the number of non-leap seconds
- `timestamp_opt`: Makes a new `DateTime` from the number of non-leap seconds
- `timestamp_millis`: Makes a new `DateTime` from the number of non-leap milliseconds
- `timestamp_millis_opt`: Makes a new `DateTime` from the number of non-leap milliseconds
- `timestamp_nanos`: Makes a new `DateTime` from the number of non-leap nanoseconds
- `timestamp_micros`: Makes a new `DateTime` from the number of non-leap microseconds
- `datetime_from_str`: Parses a string with the specified format string and returns a
- `from_offset`: Reconstructs the time zone from the offset.
- `offset_from_local_date`: Creates the offset(s) for given local `NaiveDate` if possible.
- `offset_from_local_datetime`: Creates the offset(s) for given local `NaiveDateTime` if possible.
- `from_local_date`: Converts the local `NaiveDate` to the timezone-aware `Date` if possible.
- `from_local_datetime`: Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.
- `offset_from_utc_date`: Creates the offset for given UTC `NaiveDate`. This cannot fail.
- `offset_from_utc_datetime`: Creates the offset for given UTC `NaiveDateTime`. This cannot fail.
- `from_utc_date`: Converts the UTC `NaiveDate` to the local time.
- `from_utc_datetime`: Converts the UTC `NaiveDateTime` to the local time.



