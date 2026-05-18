**rtcc**

# Module: rtcc

## Contents

**Enums**

- [`Hours`](#hours) - Hours in either 12-hour (AM/PM) or 24-hour format

**Traits**

- [`DateTimeAccess`](#datetimeaccess) - Real-Time Clock / Calendar DateTimeAccess trait to read/write a complete
- [`Rtcc`](#rtcc) - Real-Time Clock / Calendar trait

---

## rtcc::DateTimeAccess

*Trait*

Real-Time Clock / Calendar DateTimeAccess trait to read/write a complete
date/time.

Prefer to use only these methods rather than the individual methods from the
`Rtcc` trait to avoid situations where the passing of time makes the results
of the method calls inconsistent if you combine the results of several methods.

For example, this can happen at certain timepoints:
1. The time is `01:59:59`
2. A call to `hours()` returns 1.
3. The time is increased to `02:00:00`.
4. A call to `minutes()` returns 0.
5. A call to `seconds()` returns 0.
6. Your system thinks it is `01:00:00`.

The same applies to the date as well, as well as when calling setter methods.

**Methods:**

- `Error`: Error type
- `datetime`: Read the date and time.
- `set_datetime`: Set the date and time.



## rtcc::Hours

*Enum*

Hours in either 12-hour (AM/PM) or 24-hour format

**Variants:**
- `AM(u8)` - AM [1-12]
- `PM(u8)` - PM [1-12]
- `H24(u8)` - 24H format [0-23]

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Hours`
- **PartialEq**
  - `fn eq(self: &Self, other: &Hours) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rtcc::Rtcc

*Trait*

Real-Time Clock / Calendar trait

If you want to combine calls to these methods, prefer to use only
the `DateTimeAccess` trait to avoid situations where the passing of time makes the results
of the method calls inconsistent.

For example, this can happen at certain timepoints:
1. The time is `01:59:59`
2. A call to `hours()` returns 1.
3. The time is increased to `02:00:00`.
4. A call to `minutes()` returns 0.
5. A call to `seconds()` returns 0.
6. Your system thinks it is `01:00:00`.

The same applies to the date, as well as when calling setter methods.

**Methods:**

- `seconds`: Read the seconds.
- `minutes`: Read the minutes.
- `hours`: Read the hours.
- `time`: Read the time.
- `weekday`: Read the day of the week [1-7].
- `day`: Read the day of the month [1-31].
- `month`: Read the month [1-12].
- `year`: Read the year (e.g. 2000).
- `date`: Read the date.
- `set_seconds`: Set the seconds [0-59].
- `set_minutes`: Set the minutes [0-59].
- `set_hours`: Set the hours.
- `set_time`: Set the time.
- `set_weekday`: Set the day of week [1-7].
- `set_day`: Set the day of month [1-31].
- `set_month`: Set the month [1-12].
- `set_year`: Set the year. (e.g. 2000)
- `set_date`: Set the date.



