**chrono > naive**

# Module: naive

## Contents

**Modules**

- [`serde`](#serde) - Serialization/Deserialization of `NaiveDateTime` in alternate formats

**Structs**

- [`Days`](#days) - A duration in calendar days.
- [`NaiveWeek`](#naiveweek) - A week represented by a [`NaiveDate`] and a [`Weekday`] which is the first

---

## chrono::naive::Days

*Struct*

A duration in calendar days.

This is useful because when using `TimeDelta` it is possible that adding `TimeDelta::days(1)`
doesn't increment the day value as expected due to it being a fixed number of seconds. This
difference applies only when dealing with `DateTime<TimeZone>` data types and in other cases
`TimeDelta::days(n)` and `Days::new(n)` are equivalent.

**Tuple Struct**: `()`

**Methods:**

- `fn new(num: u64) -> Self` - Construct a new `Days` from a number of days

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Days) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Days) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Days`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Days) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## chrono::naive::NaiveWeek

*Struct*

A week represented by a [`NaiveDate`] and a [`Weekday`] which is the first
day of the week.

**Methods:**

- `fn first_day(self: &Self) -> NaiveDate` - Returns a date representing the first day of the week.
- `fn checked_first_day(self: &Self) -> Option<NaiveDate>` - Returns a date representing the first day of the week or
- `fn last_day(self: &Self) -> NaiveDate` - Returns a date representing the last day of the week.
- `fn checked_last_day(self: &Self) -> Option<NaiveDate>` - Returns a date representing the last day of the week or
- `fn days(self: &Self) -> RangeInclusive<NaiveDate>` - Returns a [`RangeInclusive<T>`] representing the whole week bounded by
- `fn checked_days(self: &Self) -> Option<RangeInclusive<NaiveDate>>` - Returns an [`Option<RangeInclusive<T>>`] representing the whole week bounded by

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Clone**
  - `fn clone(self: &Self) -> NaiveWeek`



## Module: serde

Serialization/Deserialization of `NaiveDateTime` in alternate formats

The various modules in here are intended to be used with serde's [`with` annotation] to
serialize as something other than the default ISO 8601 format.

[`with` annotation]: https://serde.rs/field-attrs.html#with



