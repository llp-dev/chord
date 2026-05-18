**chrono > offset > fixed**

# Module: offset::fixed

## Contents

**Structs**

- [`FixedOffset`](#fixedoffset) - The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59.

---

## chrono::offset::fixed::FixedOffset

*Struct*

The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59.

Using the [`TimeZone`](./trait.TimeZone.html) methods
on a `FixedOffset` struct is the preferred way to construct
`DateTime<FixedOffset>` instances. See the [`east_opt`](#method.east_opt) and
[`west_opt`](#method.west_opt) methods for examples.

**Methods:**

- `fn east(secs: i32) -> FixedOffset` - Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.
- `fn east_opt(secs: i32) -> Option<FixedOffset>` - Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.
- `fn west(secs: i32) -> FixedOffset` - Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.
- `fn west_opt(secs: i32) -> Option<FixedOffset>` - Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.
- `fn local_minus_utc(self: &Self) -> i32` - Returns the number of seconds to add to convert from UTC to the local time.
- `fn utc_minus_local(self: &Self) -> i32` - Returns the number of seconds to add to convert from the local time to UTC.

**Traits:** Eq, Copy

**Trait Implementations:**

- **TimeZone**
  - `fn from_offset(offset: &FixedOffset) -> FixedOffset`
  - `fn offset_from_local_date(self: &Self, _local: &NaiveDate) -> MappedLocalTime<FixedOffset>`
  - `fn offset_from_local_datetime(self: &Self, _local: &NaiveDateTime) -> MappedLocalTime<FixedOffset>`
  - `fn offset_from_utc_date(self: &Self, _utc: &NaiveDate) -> FixedOffset`
  - `fn offset_from_utc_datetime(self: &Self, _utc: &NaiveDateTime) -> FixedOffset`
- **PartialEq**
  - `fn eq(self: &Self, other: &FixedOffset) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FixedOffset`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Offset**
  - `fn fix(self: &Self) -> FixedOffset`



