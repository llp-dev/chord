*[chrono](../../index.md) / [naive](../index.md) / [isoweek](index.md)*

---

# Module `isoweek`

ISO 8601 week.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IsoWeek`](#isoweek) | struct | ISO 8601 week. |

## Structs

### `IsoWeek`

```rust
struct IsoWeek {
    ywf: i32,
}
```

ISO 8601 week.

This type, combined with [`Weekday`](../enum.Weekday.html),
constitutes the ISO 8601 [week date](./struct.NaiveDate.html#week-date).
One can retrieve this type from the existing [`Datelike`](../trait.Datelike.html) types
via the [`Datelike::iso_week`](../trait.Datelike.html#tymethod.iso_week) method.

#### Implementations

- <span id="isoweek-from-yof"></span>`fn from_yof(year: i32, ordinal: u32, year_flags: YearFlags) -> Self` — [`YearFlags`](../internals/index.md#yearflags)

  Returns the corresponding `IsoWeek` from the year and the `Of` internal value.

- <span id="isoweek-year"></span>`const fn year(&self) -> i32`

  Returns the year number for this ISO week.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  let d = NaiveDate::from_isoywd_opt(2015, 1, Weekday::Mon).unwrap();

  assert_eq!(d.iso_week().year(), 2015);

  ```

  

  This year number might not match the calendar year number.

  Continuing the example...

  

  ```rust

  use chrono::{NaiveDate, Datelike, Weekday};

  let d = NaiveDate::from_isoywd_opt(2015, 1, Weekday::Mon).unwrap();

  assert_eq!(d.year(), 2014);

  assert_eq!(d, NaiveDate::from_ymd_opt(2014, 12, 29).unwrap());

  ```

- <span id="isoweek-week"></span>`const fn week(&self) -> u32`

  Returns the ISO week number starting from 1.

  

  The return value ranges from 1 to 53. (The last week of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  let d = NaiveDate::from_isoywd_opt(2015, 15, Weekday::Mon).unwrap();

  assert_eq!(d.iso_week().week(), 15);

  ```

- <span id="isoweek-week0"></span>`const fn week0(&self) -> u32`

  Returns the ISO week number starting from 0.

  

  The return value ranges from 0 to 52. (The last week of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  let d = NaiveDate::from_isoywd_opt(2015, 15, Weekday::Mon).unwrap();

  assert_eq!(d.iso_week().week0(), 14);

  ```

#### Trait Implementations

##### `impl Clone for IsoWeek`

- <span id="isoweek-clone"></span>`fn clone(&self) -> IsoWeek` — [`IsoWeek`](#isoweek)

##### `impl Copy for IsoWeek`

##### `impl Debug for IsoWeek`

- <span id="isoweek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IsoWeek`

##### `impl Hash for IsoWeek`

- <span id="isoweek-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for IsoWeek`

- <span id="isoweek-ord-cmp"></span>`fn cmp(&self, other: &IsoWeek) -> cmp::Ordering` — [`IsoWeek`](#isoweek)

##### `impl PartialEq for IsoWeek`

- <span id="isoweek-partialeq-eq"></span>`fn eq(&self, other: &IsoWeek) -> bool` — [`IsoWeek`](#isoweek)

##### `impl PartialOrd for IsoWeek`

- <span id="isoweek-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &IsoWeek) -> option::Option<cmp::Ordering>` — [`IsoWeek`](#isoweek)

##### `impl StructuralPartialEq for IsoWeek`

