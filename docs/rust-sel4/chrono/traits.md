**chrono > traits**

# Module: traits

## Contents

**Traits**

- [`Datelike`](#datelike) - The common set of methods for date component.
- [`Timelike`](#timelike) - The common set of methods for time component.

---

## chrono::traits::Datelike

*Trait*

The common set of methods for date component.

Methods such as [`year`], [`month`], [`day`] and [`weekday`] can be used to get basic
information about the date.

The `with_*` methods can change the date.

# Warning

The `with_*` methods can be convenient to change a single component of a date, but they must be
used with some care. Examples to watch out for:

- [`with_year`] changes the year component of a year-month-day value. Don't use this method if
  you want the ordinal to stay the same after changing the year, of if you want the week and
  weekday values to stay the same.
- Don't combine two `with_*` methods to change two components of the date. For example to
  change both the year and month components of a date. This could fail because an intermediate
  value does not exist, while the final date would be valid.

For more complex changes to a date, it is best to use the methods on [`NaiveDate`] to create a
new value instead of altering an existing date.

[`year`]: Datelike::year
[`month`]: Datelike::month
[`day`]: Datelike::day
[`weekday`]: Datelike::weekday
[`with_year`]: Datelike::with_year
[`NaiveDate`]: crate::NaiveDate

**Methods:**

- `year`: Returns the year number in the [calendar date](./naive/struct.NaiveDate.html#calendar-date).
- `year_ce`: Returns the absolute year number starting from 1 with a boolean flag,
- `quarter`: Returns the quarter number starting from 1.
- `month`: Returns the month number starting from 1.
- `month0`: Returns the month number starting from 0.
- `day`: Returns the day of month starting from 1.
- `day0`: Returns the day of month starting from 0.
- `ordinal`: Returns the day of year starting from 1.
- `ordinal0`: Returns the day of year starting from 0.
- `weekday`: Returns the day of week.
- `iso_week`: Returns the ISO week.
- `with_year`: Makes a new value with the year number changed, while keeping the same month and day.
- `with_month`: Makes a new value with the month number (starting from 1) changed.
- `with_month0`: Makes a new value with the month number (starting from 0) changed.
- `with_day`: Makes a new value with the day of month (starting from 1) changed.
- `with_day0`: Makes a new value with the day of month (starting from 0) changed.
- `with_ordinal`: Makes a new value with the day of year (starting from 1) changed.
- `with_ordinal0`: Makes a new value with the day of year (starting from 0) changed.
- `num_days_from_ce`: Counts the days in the proleptic Gregorian calendar, with January 1, Year 1 (CE) as day 1.
- `num_days_in_month`: Get the length in days of the month



## chrono::traits::Timelike

*Trait*

The common set of methods for time component.

**Methods:**

- `hour`: Returns the hour number from 0 to 23.
- `hour12`: Returns the hour number from 1 to 12 with a boolean flag,
- `minute`: Returns the minute number from 0 to 59.
- `second`: Returns the second number from 0 to 59.
- `nanosecond`: Returns the number of nanoseconds since the whole non-leap second.
- `with_hour`: Makes a new value with the hour number changed.
- `with_minute`: Makes a new value with the minute number changed.
- `with_second`: Makes a new value with the second number changed.
- `with_nanosecond`: Makes a new value with nanoseconds since the whole non-leap second changed.
- `num_seconds_from_midnight`: Returns the number of non-leap seconds past the last midnight.



