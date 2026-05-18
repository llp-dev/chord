*[chrono](../index.md) / [traits](index.md)*

---

# Module `traits`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Datelike`](#datelike) | trait | The common set of methods for date component. |
| [`Timelike`](#timelike) | trait | The common set of methods for time component. |

## Traits

### `Datelike`

```rust
trait Datelike: Sized { ... }
```

The common set of methods for date component.

Methods such as `year`, [`month`](../month/index.md), `day` and [`weekday`](../weekday/index.md) can be used to get basic
information about the date.

The `with_*` methods can change the date.

# Warning

The `with_*` methods can be convenient to change a single component of a date, but they must be
used with some care. Examples to watch out for:

- `with_year` changes the year component of a year-month-day value. Don't use this method if
  you want the ordinal to stay the same after changing the year, of if you want the week and
  weekday values to stay the same.
- Don't combine two `with_*` methods to change two components of the date. For example to
  change both the year and month components of a date. This could fail because an intermediate
  value does not exist, while the final date would be valid.

For more complex changes to a date, it is best to use the methods on [`NaiveDate`](../naive/date/index.md) to create a
new value instead of altering an existing date.







#### Required Methods

- `fn year(&self) -> i32`

  Returns the year number in the [calendar date](./naive/struct.NaiveDate.html#calendar-date).

- `fn month(&self) -> u32`

  Returns the month number starting from 1.

- `fn month0(&self) -> u32`

  Returns the month number starting from 0.

- `fn day(&self) -> u32`

  Returns the day of month starting from 1.

- `fn day0(&self) -> u32`

  Returns the day of month starting from 0.

- `fn ordinal(&self) -> u32`

  Returns the day of year starting from 1.

- `fn ordinal0(&self) -> u32`

  Returns the day of year starting from 0.

- `fn weekday(&self) -> Weekday`

  Returns the day of week.

- `fn iso_week(&self) -> IsoWeek`

  Returns the ISO week.

- `fn with_year(&self, year: i32) -> Option<Self>`

  Makes a new value with the year number changed, while keeping the same month and day.

- `fn with_month(&self, month: u32) -> Option<Self>`

  Makes a new value with the month number (starting from 1) changed.

- `fn with_month0(&self, month0: u32) -> Option<Self>`

  Makes a new value with the month number (starting from 0) changed.

- `fn with_day(&self, day: u32) -> Option<Self>`

  Makes a new value with the day of month (starting from 1) changed.

- `fn with_day0(&self, day0: u32) -> Option<Self>`

  Makes a new value with the day of month (starting from 0) changed.

- `fn with_ordinal(&self, ordinal: u32) -> Option<Self>`

  Makes a new value with the day of year (starting from 1) changed.

- `fn with_ordinal0(&self, ordinal0: u32) -> Option<Self>`

  Makes a new value with the day of year (starting from 0) changed.

#### Provided Methods

- `fn year_ce(&self) -> (bool, u32)`

  Returns the absolute year number starting from 1 with a boolean flag,

- `fn quarter(&self) -> u32`

  Returns the quarter number starting from 1.

- `fn num_days_from_ce(&self) -> i32`

  Counts the days in the proleptic Gregorian calendar, with January 1, Year 1 (CE) as day 1.

- `fn num_days_in_month(&self) -> u8`

  Get the length in days of the month

#### Implementors

- [`DateTime`](../datetime/index.md#datetime)
- [`Date`](../date/index.md#date)
- [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime)
- [`NaiveDate`](../naive/date/index.md#naivedate)

### `Timelike`

```rust
trait Timelike: Sized { ... }
```

The common set of methods for time component.

#### Required Methods

- `fn hour(&self) -> u32`

  Returns the hour number from 0 to 23.

- `fn minute(&self) -> u32`

  Returns the minute number from 0 to 59.

- `fn second(&self) -> u32`

  Returns the second number from 0 to 59.

- `fn nanosecond(&self) -> u32`

  Returns the number of nanoseconds since the whole non-leap second.

- `fn with_hour(&self, hour: u32) -> Option<Self>`

  Makes a new value with the hour number changed.

- `fn with_minute(&self, min: u32) -> Option<Self>`

  Makes a new value with the minute number changed.

- `fn with_second(&self, sec: u32) -> Option<Self>`

  Makes a new value with the second number changed.

- `fn with_nanosecond(&self, nano: u32) -> Option<Self>`

  Makes a new value with nanoseconds since the whole non-leap second changed.

#### Provided Methods

- `fn hour12(&self) -> (bool, u32)`

  Returns the hour number from 1 to 12 with a boolean flag,

- `fn num_seconds_from_midnight(&self) -> u32`

  Returns the number of non-leap seconds past the last midnight.

#### Implementors

- [`DateTime`](../datetime/index.md#datetime)
- [`NaiveDateTime`](../naive/datetime/index.md#naivedatetime)
- [`NaiveTime`](../naive/time/index.md#naivetime)

