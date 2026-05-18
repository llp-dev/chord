**chrono > naive > isoweek**

# Module: naive::isoweek

## Contents

**Structs**

- [`IsoWeek`](#isoweek) - ISO 8601 week.

---

## chrono::naive::isoweek::IsoWeek

*Struct*

ISO 8601 week.

This type, combined with [`Weekday`](../enum.Weekday.html),
constitutes the ISO 8601 [week date](./struct.NaiveDate.html#week-date).
One can retrieve this type from the existing [`Datelike`](../trait.Datelike.html) types
via the [`Datelike::iso_week`](../trait.Datelike.html#tymethod.iso_week) method.

**Methods:**

- `fn year(self: &Self) -> i32` - Returns the year number for this ISO week.
- `fn week(self: &Self) -> u32` - Returns the ISO week number starting from 1.
- `fn week0(self: &Self) -> u32` - Returns the ISO week number starting from 0.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &IsoWeek) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> IsoWeek`
- **PartialEq**
  - `fn eq(self: &Self, other: &IsoWeek) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IsoWeek) -> $crate::option::Option<$crate::cmp::Ordering>`



