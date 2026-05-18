**chrono > offset > utc**

# Module: offset::utc

## Contents

**Structs**

- [`Utc`](#utc) - The UTC time zone. This is the most efficient time zone when you don't need the local time.

---

## chrono::offset::utc::Utc

*Struct*

The UTC time zone. This is the most efficient time zone when you don't need the local time.
It is also used as an offset (which is also a dummy type).

Using the [`TimeZone`](./trait.TimeZone.html) methods
on the UTC struct is the preferred way to construct `DateTime<Utc>`
instances.

# Example

```
use chrono::{DateTime, TimeZone, Utc};

let dt = DateTime::from_timestamp(61, 0).unwrap();

assert_eq!(Utc.timestamp_opt(61, 0).unwrap(), dt);
assert_eq!(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap(), dt);
```

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Utc`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Utc) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Offset**
  - `fn fix(self: &Self) -> FixedOffset`
- **TimeZone**
  - `fn from_offset(_state: &Utc) -> Utc`
  - `fn offset_from_local_date(self: &Self, _local: &NaiveDate) -> MappedLocalTime<Utc>`
  - `fn offset_from_local_datetime(self: &Self, _local: &NaiveDateTime) -> MappedLocalTime<Utc>`
  - `fn offset_from_utc_date(self: &Self, _utc: &NaiveDate) -> Utc`
  - `fn offset_from_utc_datetime(self: &Self, _utc: &NaiveDateTime) -> Utc`



