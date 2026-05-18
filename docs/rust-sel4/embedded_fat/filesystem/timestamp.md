**embedded_fat > filesystem > timestamp**

# Module: filesystem::timestamp

## Contents

**Structs**

- [`Timestamp`](#timestamp) - Represents an instant in time, in the local time zone. TODO: Consider

**Traits**

- [`TimeSource`](#timesource) - Things that impl this can tell you the current time.

---

## embedded_fat::filesystem::timestamp::TimeSource

*Trait*

Things that impl this can tell you the current time.

**Methods:**

- `get_timestamp`: Returns the current time



## embedded_fat::filesystem::timestamp::Timestamp

*Struct*

Represents an instant in time, in the local time zone. TODO: Consider
replacing this with POSIX time as a `u32`, which would save two bytes at
the expense of some maths.

**Fields:**
- `year_since_1970: u8` - Add 1970 to this file to get the calendar year
- `zero_indexed_month: u8` - Add one to this value to get the calendar month
- `zero_indexed_day: u8` - Add one to this value to get the calendar day
- `hours: u8` - The number of hours past midnight
- `minutes: u8` - The number of minutes past the hour
- `seconds: u8` - The number of seconds past the minute

**Methods:**

- `fn from_fat(date: u16, time: u16) -> Timestamp` - Create a `Timestamp` from the 16-bit FAT date and time fields.
- `fn serialize_to_fat(self: Self) -> [u8; 4]` - Serialize a `Timestamp` to FAT format
- `fn from_calendar(year: u16, month: u8, day: u8, hours: u8, minutes: u8, seconds: u8) -> Result<Timestamp, &'static str>` - Create a `Timestamp` from year/month/day/hour/minute/second.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Timestamp) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Timestamp) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Timestamp) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Timestamp`



