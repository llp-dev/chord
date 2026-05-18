*[chrono](../index.md) / [offset](index.md)*

---

# Module `offset`

The time zone, which calculates offsets from the local time to UTC.

There are four operations provided by the `TimeZone` trait:

1. Converting the local `NaiveDateTime` to `DateTime<Tz>`
2. Converting the UTC `NaiveDateTime` to `DateTime<Tz>`
3. Converting `DateTime<Tz>` to the local `NaiveDateTime`
4. Constructing `DateTime<Tz>` objects from various offsets

1 is used for constructors. 2 is used for the `with_timezone` method of date and time types.
3 is used for other methods, e.g. `year()` or `format()`, and provided by an associated type
which implements `Offset` (which then passed to `TimeZone` for actual implementations).
Technically speaking `TimeZone` has a total knowledge about given timescale,
but `Offset` is used as a cache to avoid the repeated conversion
and provides implementations for 1 and 3.
An `TimeZone` instance can be reconstructed from the corresponding `Offset` instance.

## Contents

- [Modules](#modules)
  - [`fixed`](#fixed)
  - [`utc`](#utc)
- [Structs](#structs)
  - [`FixedOffset`](#fixedoffset)
  - [`Utc`](#utc)
- [Enums](#enums)
  - [`LocalResult`](#localresult)
- [Traits](#traits)
  - [`Offset`](#offset)
  - [`TimeZone`](#timezone)
- [Type Aliases](#type-aliases)
  - [`MappedLocalTime`](#mappedlocaltime)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fixed`](#fixed) | mod | The time zone which has a fixed offset from UTC. |
| [`utc`](#utc) | mod | The UTC (Coordinated Universal Time) time zone. |
| [`FixedOffset`](#fixedoffset) | struct |  |
| [`Utc`](#utc) | struct |  |
| [`LocalResult`](#localresult) | enum | Old name of [`MappedLocalTime`]. |
| [`Offset`](#offset) | trait | The offset from the local time to UTC. |
| [`TimeZone`](#timezone) | trait | The time zone. |
| [`MappedLocalTime`](#mappedlocaltime) | type | The result of mapping a local time to a concrete instant in a given time zone. |

## Modules

- [`fixed`](fixed/index.md) — The time zone which has a fixed offset from UTC.
- [`utc`](utc/index.md) — The UTC (Coordinated Universal Time) time zone.

## Structs

### `FixedOffset`

```rust
struct FixedOffset {
    local_minus_utc: i32,
}
```

The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59.

Using the [`TimeZone`](./trait.TimeZone.html) methods
on a `FixedOffset` struct is the preferred way to construct
`DateTime<FixedOffset>` instances. See the [`east_opt`](#method.east_opt) and
[`west_opt`](#method.west_opt) methods for examples.

#### Implementations

- <span id="fixedoffset-east"></span>`fn east(secs: i32) -> FixedOffset` — [`FixedOffset`](fixed/index.md#fixedoffset)

  Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.

  The negative `secs` means the Western Hemisphere.

  

  Panics on the out-of-bound `secs`.

- <span id="fixedoffset-east-opt"></span>`const fn east_opt(secs: i32) -> Option<FixedOffset>` — [`FixedOffset`](fixed/index.md#fixedoffset)

  Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.

  The negative `secs` means the Western Hemisphere.

  

  Returns `None` on the out-of-bound `secs`.

  

  # Example

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{FixedOffset, TimeZone};

  let hour = 3600;

  let datetime =

      FixedOffset::east_opt(5 * hour).unwrap().with_ymd_and_hms(2016, 11, 08, 0, 0, 0).unwrap();

  assert_eq!(&datetime.to_rfc3339(), "2016-11-08T00:00:00+05:00")

  }

  ```

- <span id="fixedoffset-west"></span>`fn west(secs: i32) -> FixedOffset` — [`FixedOffset`](fixed/index.md#fixedoffset)

  Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.

  The negative `secs` means the Eastern Hemisphere.

  

  Panics on the out-of-bound `secs`.

- <span id="fixedoffset-west-opt"></span>`const fn west_opt(secs: i32) -> Option<FixedOffset>` — [`FixedOffset`](fixed/index.md#fixedoffset)

  Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.

  The negative `secs` means the Eastern Hemisphere.

  

  Returns `None` on the out-of-bound `secs`.

  

  # Example

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{FixedOffset, TimeZone};

  let hour = 3600;

  let datetime =

      FixedOffset::west_opt(5 * hour).unwrap().with_ymd_and_hms(2016, 11, 08, 0, 0, 0).unwrap();

  assert_eq!(&datetime.to_rfc3339(), "2016-11-08T00:00:00-05:00")

  }

  ```

- <span id="fixedoffset-local-minus-utc"></span>`const fn local_minus_utc(&self) -> i32`

  Returns the number of seconds to add to convert from UTC to the local time.

- <span id="fixedoffset-utc-minus-local"></span>`const fn utc_minus_local(&self) -> i32`

  Returns the number of seconds to add to convert from the local time to UTC.

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, rhs: FixedOffset) -> DateTime<Tz>` — [`FixedOffset`](fixed/index.md#fixedoffset), [`DateTime`](../datetime/index.md#datetime)

##### `impl Clone for FixedOffset`

- <span id="fixedoffset-clone"></span>`fn clone(&self) -> FixedOffset` — [`FixedOffset`](fixed/index.md#fixedoffset)

##### `impl Copy for FixedOffset`

##### `impl Debug for FixedOffset`

- <span id="fixedoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FixedOffset`

- <span id="fixedoffset-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FixedOffset`

##### `impl FromStr for FixedOffset`

- <span id="fixedoffset-fromstr-type-err"></span>`type Err = ParseError`

- <span id="fixedoffset-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for FixedOffset`

- <span id="fixedoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Offset for FixedOffset`

- <span id="fixedoffset-offset-fix"></span>`fn fix(&self) -> FixedOffset` — [`FixedOffset`](fixed/index.md#fixedoffset)

##### `impl PartialEq for FixedOffset`

- <span id="fixedoffset-partialeq-eq"></span>`fn eq(&self, other: &FixedOffset) -> bool` — [`FixedOffset`](fixed/index.md#fixedoffset)

##### `impl StructuralPartialEq for FixedOffset`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, rhs: FixedOffset) -> DateTime<Tz>` — [`FixedOffset`](fixed/index.md#fixedoffset), [`DateTime`](../datetime/index.md#datetime)

##### `impl TimeZone for FixedOffset`

- <span id="fixedoffset-timezone-type-offset"></span>`type Offset = FixedOffset`

- <span id="fixedoffset-timezone-from-offset"></span>`fn from_offset(offset: &FixedOffset) -> FixedOffset` — [`FixedOffset`](fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-local-date"></span>`fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<FixedOffset>` — [`NaiveDate`](../naive/date/index.md#naivedate), [`MappedLocalTime`](#mappedlocaltime), [`FixedOffset`](fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-local-datetime"></span>`fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> MappedLocalTime<FixedOffset>` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime), [`MappedLocalTime`](#mappedlocaltime), [`FixedOffset`](fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-utc-date"></span>`fn offset_from_utc_date(&self, _utc: &NaiveDate) -> FixedOffset` — [`NaiveDate`](../naive/date/index.md#naivedate), [`FixedOffset`](fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-utc-datetime"></span>`fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> FixedOffset` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime), [`FixedOffset`](fixed/index.md#fixedoffset)

##### `impl ToString for FixedOffset`

- <span id="fixedoffset-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Utc`

```rust
struct Utc;
```

The UTC time zone. This is the most efficient time zone when you don't need the local time.
It is also used as an offset (which is also a dummy type).

Using the [`TimeZone`](./trait.TimeZone.html) methods
on the UTC struct is the preferred way to construct `DateTime<Utc>`
instances.

# Example

```rust
use chrono::{DateTime, TimeZone, Utc};

let dt = DateTime::from_timestamp(61, 0).unwrap();

assert_eq!(Utc.timestamp_opt(61, 0).unwrap(), dt);
assert_eq!(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap(), dt);
```

#### Trait Implementations

##### `impl Clone for Utc`

- <span id="utc-clone"></span>`fn clone(&self) -> Utc` — [`Utc`](utc/index.md#utc)

##### `impl Copy for Utc`

##### `impl Debug for Utc`

- <span id="utc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utc`

- <span id="utc-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utc`

##### `impl Hash for Utc`

- <span id="utc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Offset for Utc`

- <span id="utc-offset-fix"></span>`fn fix(&self) -> FixedOffset` — [`FixedOffset`](fixed/index.md#fixedoffset)

##### `impl PartialEq for Utc`

- <span id="utc-partialeq-eq"></span>`fn eq(&self, other: &Utc) -> bool` — [`Utc`](utc/index.md#utc)

##### `impl StructuralPartialEq for Utc`

##### `impl TimeZone for Utc`

- <span id="utc-timezone-type-offset"></span>`type Offset = Utc`

- <span id="utc-timezone-from-offset"></span>`fn from_offset(_state: &Utc) -> Utc` — [`Utc`](utc/index.md#utc)

- <span id="utc-timezone-offset-from-local-date"></span>`fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<Utc>` — [`NaiveDate`](../naive/date/index.md#naivedate), [`MappedLocalTime`](#mappedlocaltime), [`Utc`](utc/index.md#utc)

- <span id="utc-timezone-offset-from-local-datetime"></span>`fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> MappedLocalTime<Utc>` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime), [`MappedLocalTime`](#mappedlocaltime), [`Utc`](utc/index.md#utc)

- <span id="utc-timezone-offset-from-utc-date"></span>`fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Utc` — [`NaiveDate`](../naive/date/index.md#naivedate), [`Utc`](utc/index.md#utc)

- <span id="utc-timezone-offset-from-utc-datetime"></span>`fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Utc` — [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime), [`Utc`](utc/index.md#utc)

##### `impl ToString for Utc`

- <span id="utc-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `LocalResult<T>`

```rust
enum LocalResult<T> {
    Single(T),
    Ambiguous(T, T),
    None,
}
```

Old name of [`MappedLocalTime`](#mappedlocaltime). See that type for more documentation.

#### Variants

- **`Single`**

  The local time maps to a single unique result.

- **`Ambiguous`**

  The local time is _ambiguous_ because there is a _fold_ in the local time.
  
  This variant contains the two possible results, in the order `(earliest, latest)`.

- **`None`**

  The local time does not exist because there is a _gap_ in the local time.
  
  This variant may also be returned if there was an error while resolving the local time,
  caused by for example missing time zone data files, an error in an OS API, or overflow.

#### Implementations

- <span id="localresult-single"></span>`fn single(self) -> Option<T>`

  Returns `Some` if the time zone mapping has a single result.

  

  # Errors

  

  Returns `None` if local time falls in a _fold_ or _gap_ in the local time, or if there was

  an error.

- <span id="localresult-earliest"></span>`fn earliest(self) -> Option<T>`

  Returns the earliest possible result of the time zone mapping.

  

  # Errors

  

  Returns `None` if local time falls in a _gap_ in the local time, or if there was an error.

- <span id="localresult-latest"></span>`fn latest(self) -> Option<T>`

  Returns the latest possible result of the time zone mapping.

  

  # Errors

  

  Returns `None` if local time falls in a _gap_ in the local time, or if there was an error.

- <span id="localresult-map"></span>`fn map<U, F: FnMut(T) -> U>(self, f: F) -> MappedLocalTime<U>` — [`MappedLocalTime`](#mappedlocaltime)

  Maps a `MappedLocalTime<T>` into `MappedLocalTime<U>` with given function.

- <span id="localresult-and-then"></span>`fn and_then<U, F: FnMut(T) -> Option<U>>(self, f: F) -> MappedLocalTime<U>` — [`MappedLocalTime`](#mappedlocaltime)

  Maps a `MappedLocalTime<T>` into `MappedLocalTime<U>` with given function.

  

  Returns `MappedLocalTime::None` if the function returns `None`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for LocalResult<T>`

- <span id="localresult-clone"></span>`fn clone(&self) -> LocalResult<T>` — [`LocalResult`](#localresult)

##### `impl<T: marker::Copy> Copy for LocalResult<T>`

##### `impl<T: fmt::Debug> Debug for LocalResult<T>`

- <span id="localresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for LocalResult<T>`

##### `impl<T: hash::Hash> Hash for LocalResult<T>`

- <span id="localresult-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for LocalResult<T>`

- <span id="localresult-partialeq-eq"></span>`fn eq(&self, other: &LocalResult<T>) -> bool` — [`LocalResult`](#localresult)

##### `impl<T> StructuralPartialEq for LocalResult<T>`

## Traits

### `Offset`

```rust
trait Offset: Sized + Clone + fmt::Debug { ... }
```

The offset from the local time to UTC.

#### Required Methods

- `fn fix(&self) -> FixedOffset`

  Returns the fixed offset from UTC to the local time stored.

#### Implementors

- [`FixedOffset`](fixed/index.md#fixedoffset)
- [`Utc`](utc/index.md#utc)

### `TimeZone`

```rust
trait TimeZone: Sized + Clone { ... }
```

The time zone.

The methods here are the primary constructors for the [`DateTime`](../datetime/index.md) type.

#### Associated Types

- `type Offset: 1`

#### Required Methods

- `fn from_offset(offset: &<Self as >::Offset) -> Self`

  Reconstructs the time zone from the offset.

- `fn offset_from_local_date(&self, local: &NaiveDate) -> MappedLocalTime<<Self as >::Offset>`

  Creates the offset(s) for given local `NaiveDate` if possible.

- `fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> MappedLocalTime<<Self as >::Offset>`

  Creates the offset(s) for given local `NaiveDateTime` if possible.

- `fn offset_from_utc_date(&self, utc: &NaiveDate) -> <Self as >::Offset`

  Creates the offset for given UTC `NaiveDate`. This cannot fail.

- `fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> <Self as >::Offset`

  Creates the offset for given UTC `NaiveDateTime`. This cannot fail.

#### Provided Methods

- `fn with_ymd_and_hms(&self, year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> MappedLocalTime<DateTime<Self>>`

  Make a new `DateTime` from year, month, day, time components and current time zone.

- `fn ymd(&self, year: i32, month: u32, day: u32) -> Date<Self>`

  Makes a new `Date` from year, month, day and the current time zone.

- `fn ymd_opt(&self, year: i32, month: u32, day: u32) -> MappedLocalTime<Date<Self>>`

  Makes a new `Date` from year, month, day and the current time zone.

- `fn yo(&self, year: i32, ordinal: u32) -> Date<Self>`

  Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.

- `fn yo_opt(&self, year: i32, ordinal: u32) -> MappedLocalTime<Date<Self>>`

  Makes a new `Date` from year, day of year (DOY or "ordinal") and the current time zone.

- `fn isoywd(&self, year: i32, week: u32, weekday: Weekday) -> Date<Self>`

  Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and

- `fn isoywd_opt(&self, year: i32, week: u32, weekday: Weekday) -> MappedLocalTime<Date<Self>>`

  Makes a new `Date` from ISO week date (year and week number), day of the week (DOW) and

- `fn timestamp(&self, secs: i64, nsecs: u32) -> DateTime<Self>`

  Makes a new `DateTime` from the number of non-leap seconds

- `fn timestamp_opt(&self, secs: i64, nsecs: u32) -> MappedLocalTime<DateTime<Self>>`

  Makes a new `DateTime` from the number of non-leap seconds

- `fn timestamp_millis(&self, millis: i64) -> DateTime<Self>`

  Makes a new `DateTime` from the number of non-leap milliseconds

- `fn timestamp_millis_opt(&self, millis: i64) -> MappedLocalTime<DateTime<Self>>`

  Makes a new `DateTime` from the number of non-leap milliseconds

- `fn timestamp_nanos(&self, nanos: i64) -> DateTime<Self>`

  Makes a new `DateTime` from the number of non-leap nanoseconds

- `fn timestamp_micros(&self, micros: i64) -> MappedLocalTime<DateTime<Self>>`

  Makes a new `DateTime` from the number of non-leap microseconds

- `fn datetime_from_str(&self, s: &str, fmt: &str) -> ParseResult<DateTime<Self>>`

  Parses a string with the specified format string and returns a

- `fn from_local_date(&self, local: &NaiveDate) -> MappedLocalTime<Date<Self>>`

  Converts the local `NaiveDate` to the timezone-aware `Date` if possible.

- `fn from_local_datetime(&self, local: &NaiveDateTime) -> MappedLocalTime<DateTime<Self>>`

  Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.

- `fn from_utc_date(&self, utc: &NaiveDate) -> Date<Self>`

  Converts the UTC `NaiveDate` to the local time.

- `fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Self>`

  Converts the UTC `NaiveDateTime` to the local time.

#### Implementors

- [`FixedOffset`](fixed/index.md#fixedoffset)
- [`Utc`](utc/index.md#utc)

## Type Aliases

### `MappedLocalTime<T>`

```rust
type MappedLocalTime<T> = LocalResult<T>;
```

The result of mapping a local time to a concrete instant in a given time zone.

The calculation to go from a local time (wall clock time) to an instant in UTC can end up in
three cases:
* A single, simple result.
* An ambiguous result when the clock is turned backwards during a transition due to for example
  DST.
* No result when the clock is turned forwards during a transition due to for example DST.

<div class="warning">

In wasm, when using `Local`, only the [`LocalResult::Single`](../index.md) variant is returned.
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

The type of `T` is usually a [`DateTime`](../datetime/index.md) but may also be only an offset.

