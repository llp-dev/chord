# Crate `chrono`

# Chrono: Date and Time for Rust

Chrono aims to provide all functionality needed to do correct operations on dates and times in
the [proleptic Gregorian calendar]:

* The [`DateTime`](datetime/index.md) type is timezone-aware by default, with separate timezone-naive types.
* Operations that may produce an invalid or ambiguous date and time return `Option` or
  [`MappedLocalTime`](offset/index.md).
* Configurable parsing and formatting with a `strftime` inspired date and time formatting
  syntax.
* The `Local` timezone works with the current timezone of the OS.
* Types and operations are implemented to be reasonably efficient.

Timezone data is not shipped with chrono by default to limit binary sizes. Use the companion
crate [Chrono-TZ] or `tzfile` for full timezone support.



### Features

Chrono supports various runtime environments and operating systems, and has several features
that may be enabled or disabled.

Default features:

- `alloc`: Enable features that depend on allocation (primarily string formatting).
- `std`: Enables functionality that depends on the standard library. This is a superset of
  `alloc` and adds interoperation with standard library types and traits.
- `clock`: Enables reading the local timezone (`Local`). This is a superset of `now`.
- `now`: Enables reading the system time (`now`).
- `wasmbind`: Interface with the JS Date API for the `wasm32` target.

Optional features:

- `serde`: Enable serialization/deserialization via [`serde`](time_delta/serde/index.md).
- `rkyv`: Deprecated, use the `rkyv-*` features.
- `rkyv-16`: Enable serialization/deserialization via [rkyv],
  using 16-bit integers for integral `*size` types.
- `rkyv-32`: Enable serialization/deserialization via [rkyv],
  using 32-bit integers for integral `*size` types.
- `rkyv-64`: Enable serialization/deserialization via [rkyv],
  using 64-bit integers for integral `*size` types.
- `rkyv-validation`: Enable rkyv validation support using `bytecheck`.
- `arbitrary`: Construct arbitrary instances of a type with the Arbitrary crate.
- `unstable-locales`: Enable localization. This adds various methods with a `_localized` suffix.
  The implementation and API may change or even be removed in a patch release. Feedback welcome.
- `oldtime`: This feature no longer has any effect; it used to offer compatibility with the
  `time` 0.1 crate.

Note: The `rkyv{,-16,-32,-64}` features are mutually exclusive.

See the [cargo docs] for examples of specifying features.



## Overview

### Time delta / Duration

Chrono has a [`TimeDelta`](time_delta/index.md) type to represent the magnitude of a time span. This is an "accurate"
duration represented as seconds and nanoseconds, and does not represent "nominal" components
such as days or months.

The [`TimeDelta`](time_delta/index.md) type was previously named `Duration` (and is still available as a type alias
with that name). A notable difference with the similar [`core::time::Duration`](#coretimeduration) is that it is a
signed value instead of unsigned.

Chrono currently only supports a small number of operations with [`core::time::Duration`](#coretimeduration).
You can convert between both types with the `TimeDelta::from_std` and `TimeDelta::to_std`
methods.

### Date and Time

Chrono provides a [`DateTime`](datetime/index.md) type to represent a date and a time in a timezone.

For more abstract moment-in-time tracking such as internal timekeeping that is unconcerned with
timezones, consider `std::time::SystemTime`, which tracks your system clock, or
`std::time::Instant`, which is an opaque but monotonically-increasing representation of a
moment in time.

[`DateTime`](datetime/index.md) is timezone-aware and must be constructed from a [`TimeZone`](offset/index.md) object, which defines
how the local date is converted to and back from the UTC date.
There are three well-known [`TimeZone`](offset/index.md) implementations:

* [`Utc`](offset/utc/index.md) specifies the UTC time zone. It is most efficient.

* `Local` specifies the system local time zone.

* [`FixedOffset`](offset/fixed/index.md) specifies an arbitrary, fixed time zone such as UTC+09:00 or UTC-10:30.
  This often results from the parsed textual date and time. Since it stores the most information
  and does not depend on the system environment, you would want to normalize other `TimeZone`s
  into this type.

[`DateTime`](datetime/index.md)s with different [`TimeZone`](offset/index.md) types are distinct and do not mix, but can be
converted to each other using the `DateTime::with_timezone` method.

You can get the current date and time in the UTC time zone (`Utc::now()`) or in the local time
zone (`Local::now()`).

```rust
#[cfg(feature = "now")] {
use chrono::prelude::*;

let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
let _ = utc;
}
```

```rust
#[cfg(feature = "clock")] {
use chrono::prelude::*;

let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
let _ = local;
}
```

Alternatively, you can create your own date and time. This is a bit verbose due to Rust's lack
of function and method overloading, but in turn we get a rich combination of initialization
methods.

```rust
use chrono::offset::MappedLocalTime;
use chrono::prelude::*;

fn doctest() -> Option<()> {

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_opt(9, 10, 11)?
        .and_utc()
);

// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, NaiveDate::from_yo_opt(2014, 189)?.and_hms_opt(9, 10, 11)?.and_utc());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(
    dt,
    NaiveDate::from_isoywd_opt(2014, 28, Weekday::Tue)?.and_hms_opt(9, 10, 11)?.and_utc()
);

let dt = NaiveDate::from_ymd_opt(2014, 7, 8)?
    .and_hms_milli_opt(9, 10, 11, 12)?
    .and_utc(); // `2014-07-08T09:10:11.012Z`
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_micro_opt(9, 10, 11, 12_000)?
        .and_utc()
);
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_nano_opt(9, 10, 11, 12_000_000)?
        .and_utc()
);

// dynamic verification
assert_eq!(
    Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33),
    MappedLocalTime::Single(
        NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_opt(21, 15, 33)?.and_utc()
    )
);
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 8, 80, 15, 33), MappedLocalTime::None);
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 38, 21, 15, 33), MappedLocalTime::None);

#[cfg(feature = "clock")] {
// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap(),
    )
    .unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600)
    .unwrap()
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 7, 8)
            .unwrap()
            .and_hms_milli_opt(18, 10, 11, 12)
            .unwrap(),
    )
    .unwrap();
assert_eq!(dt, fixed_dt);
let _ = local_dt;
}
Some(())
}
doctest().unwrap();
```

Various properties are available to the date and time, and can be altered individually. Most of
them are defined in the traits [`Datelike`](traits/index.md) and [`Timelike`](traits/index.md) which you should `use` before.
Addition and subtraction is also supported.
The following illustrates most supported operations to the date and time:

```rust
use chrono::prelude::*;
use chrono::TimeDelta;

// assume this returned `2014-11-28T21:45:59.324310806+09:00`:
let dt = FixedOffset::east_opt(9 * 3600)
    .unwrap()
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 11, 28)
            .unwrap()
            .and_hms_nano_opt(21, 45, 59, 324310806)
            .unwrap(),
    )
    .unwrap();

// property accessors
assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
assert_eq!((dt.month0(), dt.day0()), (10, 27)); // for unfortunate souls
assert_eq!((dt.hour(), dt.minute(), dt.second()), (21, 45, 59));
assert_eq!(dt.weekday(), Weekday::Fri);
assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7
assert_eq!(dt.ordinal(), 332); // the day of year
assert_eq!(dt.num_days_from_ce(), 735565); // the number of days from and including Jan 1, 1

// time zone accessor and manipulation
assert_eq!(dt.offset().fix().local_minus_utc(), 9 * 3600);
assert_eq!(dt.timezone(), FixedOffset::east_opt(9 * 3600).unwrap());
assert_eq!(
    dt.with_timezone(&Utc),
    NaiveDate::from_ymd_opt(2014, 11, 28)
        .unwrap()
        .and_hms_nano_opt(12, 45, 59, 324310806)
        .unwrap()
        .and_utc()
);

// a sample of property manipulations (validates dynamically)
assert_eq!(dt.with_day(29).unwrap().weekday(), Weekday::Sat); // 2014-11-29 is Saturday
assert_eq!(dt.with_day(32), None);
assert_eq!(dt.with_year(-300).unwrap().num_days_from_ce(), -109606); // November 29, 301 BCE

// arithmetic operations
let dt1 = Utc.with_ymd_and_hms(2014, 11, 14, 8, 9, 10).unwrap();
let dt2 = Utc.with_ymd_and_hms(2014, 11, 14, 10, 9, 8).unwrap();
assert_eq!(dt1.signed_duration_since(dt2), TimeDelta::try_seconds(-2 * 3600 + 2).unwrap());
assert_eq!(dt2.signed_duration_since(dt1), TimeDelta::try_seconds(2 * 3600 - 2).unwrap());
assert_eq!(
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
        + TimeDelta::try_seconds(1_000_000_000).unwrap(),
    Utc.with_ymd_and_hms(2001, 9, 9, 1, 46, 40).unwrap()
);
assert_eq!(
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
        - TimeDelta::try_seconds(1_000_000_000).unwrap(),
    Utc.with_ymd_and_hms(1938, 4, 24, 22, 13, 20).unwrap()
);
```

### Formatting and Parsing

Formatting is done via the [`format`](DateTime::format()) method, which format is equivalent to
the familiar `strftime` format.

See [`format::strftime`](format::strftime#specifiers) documentation for full syntax and list of
specifiers.

The default `to_string` method and `{:?}` specifier also give a reasonable representation.
Chrono also provides [`to_rfc2822`](DateTime::to_rfc2822) and
[`to_rfc3339`](DateTime::to_rfc3339) methods for well-known formats.

Chrono now also provides date formatting in almost any language without the help of an
additional C library. This functionality is under the feature `unstable-locales`:

```toml
chrono = { version = "0.4", features = ["unstable-locales"] }
```

The `unstable-locales` feature requires and implies at least the `alloc` feature.

```rust
#[allow(unused_imports)]
use chrono::prelude::*;

#[cfg(all(feature = "unstable-locales", feature = "alloc"))]
fn test() {
let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09");
assert_eq!(dt.format("%a %b %e %T %Y").to_string(), "Fri Nov 28 12:00:09 2014");
assert_eq!(
    dt.format_localized("%A %e %B %Y, %T", Locale::fr_BE).to_string(),
    "vendredi 28 novembre 2014, 12:00:09"
);

assert_eq!(dt.format("%a %b %e %T %Y").to_string(), dt.format("%c").to_string());
assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
assert_eq!(dt.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
assert_eq!(dt.to_rfc3339(), "2014-11-28T12:00:09+00:00");
assert_eq!(format!("{:?}", dt), "2014-11-28T12:00:09Z");

// Note that milli/nanoseconds are only printed if they are non-zero
let dt_nano = NaiveDate::from_ymd_opt(2014, 11, 28)
    .unwrap()
    .and_hms_nano_opt(12, 0, 9, 1)
    .unwrap()
    .and_utc();
assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
}
#[cfg(not(all(feature = "unstable-locales", feature = "alloc")))]
fn test() {}
if cfg!(all(feature = "unstable-locales", feature = "alloc")) {
   test();
}
```

Parsing can be done with two methods:

1. The standard [`FromStr`](std::str::FromStr) trait (and [`parse`](str::parse) method on a
   string) can be used for parsing `DateTime<FixedOffset>`, `DateTime<Utc>` and
   `DateTime<Local>` values. This parses what the `{:?}` (`std::fmt::Debug` format specifier
   prints, and requires the offset to be present.

2. `DateTime::parse_from_str` parses a date and time with offsets and returns
   `DateTime<FixedOffset>`. This should be used when the offset is a part of input and the
   caller cannot guess that. It *cannot* be used when the offset can be missing.
   `DateTime::parse_from_rfc2822` and `DateTime::parse_from_rfc3339` are similar but for
   well-known formats.

More detailed control over the parsing process is available via [`format`](mod@format) module.

```rust
use chrono::prelude::*;

let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
let fixed_dt = dt.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());

// method 1
assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

// method 2
assert_eq!(
    DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
    Ok(fixed_dt.clone())
);
assert_eq!(
    DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
    Ok(fixed_dt.clone())
);
assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

// oops, the year is missing!
assert!(DateTime::parse_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
// oops, the format string does not include the year at all!
assert!(DateTime::parse_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
// oops, the weekday is incorrect!
assert!(DateTime::parse_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
```

Again: See [`format::strftime`](format::strftime#specifiers) documentation for full syntax and
list of specifiers.

### Conversion from and to EPOCH timestamps

Use [`DateTime::from_timestamp(seconds, nanoseconds)`](DateTime::from_timestamp)
to construct a [`DateTime<Utc>`](datetime/index.md) from a UNIX timestamp
(seconds, nanoseconds that passed since January 1st 1970).

Use [`DateTime.timestamp`](DateTime::timestamp) to get the timestamp (in seconds)
from a [`DateTime`](datetime/index.md). Additionally, you can use
[`DateTime.timestamp_subsec_nanos`](DateTime::timestamp_subsec_nanos)
to get the number of additional number of nanoseconds.

```rust
#[cfg(feature = "alloc")] {
// We need the trait in scope to use Utc::timestamp().
use chrono::{DateTime, Utc};

// Construct a datetime from epoch:
let dt: DateTime<Utc> = DateTime::from_timestamp_secs(1_500_000_000).unwrap();
assert_eq!(dt.to_rfc2822(), "Fri, 14 Jul 2017 02:40:00 +0000");

// Get epoch value from a datetime:
let dt = DateTime::parse_from_rfc2822("Fri, 14 Jul 2017 02:40:00 +0000").unwrap();
assert_eq!(dt.timestamp(), 1_500_000_000);
}
```

### Naive date and time

Chrono provides naive counterparts to `Date`, (non-existent) `Time` and `DateTime` as
[`NaiveDate`](naive/date/index.md), [`NaiveTime`](naive/time/index.md) and [`NaiveDateTime`](naive/datetime/index.md) respectively.

They have almost equivalent interfaces as their timezone-aware twins, but are not associated to
time zones obviously and can be quite low-level. They are mostly useful for building blocks for
higher-level types.

Timezone-aware `DateTime` and `Date` types have two methods returning naive versions:
[`naive_local`](DateTime::naive_local) returns a view to the naive local time,
and [`naive_utc`](DateTime::naive_utc) returns a view to the naive UTC time.

## Limitations

* Only the proleptic Gregorian calendar (i.e. extended to support older dates) is supported.
* Date types are limited to about +/- 262,000 years from the common epoch.
* Time types are limited to nanosecond accuracy.
* Leap seconds can be represented, but Chrono does not fully support them.
  See [Leap Second Handling](NaiveTime#leap-second-handling).

## Rust version requirements

The Minimum Supported Rust Version (MSRV) is currently **Rust 1.61.0**.

The MSRV is explicitly tested in CI. It may be bumped in minor releases, but this is not done
lightly.

## Relation between chrono and time 0.1

Rust first had a `time` module added to `std` in its 0.7 release. It later moved to
`libextra`, and then to a `libtime` library shipped alongside the standard library. In 2014
work on chrono started in order to provide a full-featured date and time library in Rust.
Some improvements from chrono made it into the standard library; notably, `chrono::Duration`
was included as `std::time::Duration` ([rust#15934]) in 2014.

In preparation of Rust 1.0 at the end of 2014 `libtime` was moved out of the Rust distro and
into the `time` crate to eventually be redesigned ([rust#18832], [rust#18858]), like the
`num` and `rand` crates. Of course chrono kept its dependency on this `time` crate. `time`
started re-exporting `std::time::Duration` during this period. Later, the standard library was
changed to have a more limited unsigned `Duration` type ([rust#24920], [RFC 1040]), while the
`time` crate kept the full functionality with `time::Duration`. `time::Duration` had been a
part of chrono's public API.

By 2016 `time` 0.1 lived under the `rust-lang-deprecated` organisation and was not actively
maintained ([time#136]). chrono absorbed the platform functionality and `Duration` type of the
`time` crate in [chrono#478] (the work started in [chrono#286]). In order to preserve
compatibility with downstream crates depending on `time` and `chrono` sharing a `Duration`
type, chrono kept depending on time 0.1. chrono offered the option to opt out of the `time`
dependency by disabling the `oldtime` feature (swapping it out for an effectively similar
chrono type). In 2019, @jhpratt took over maintenance on the `time` crate and released what
amounts to a new crate as `time` 0.2.








## Security advisories

In November of 2020 [CVE-2020-26235] and [RUSTSEC-2020-0071] were opened against the `time` crate.
@quininer had found that calls to `localtime_r` may be unsound ([chrono#499]). Eventually, almost
a year later, this was also made into a security advisory against chrono as [RUSTSEC-2020-0159],
which had platform code similar to `time`.

On Unix-like systems a process is given a timezone id or description via the `TZ` environment
variable. We need this timezone data to calculate the current local time from a value that is
in UTC, such as the time from the system clock. `time` 0.1 and chrono used the POSIX function
`localtime_r` to do the conversion to local time, which reads the `TZ` variable.

Rust assumes the environment to be writable and uses locks to access it from multiple threads.
Some other programming languages and libraries use similar locking strategies, but these are
typically not shared across languages. More importantly, POSIX declares modifying the
environment in a multi-threaded process as unsafe, and `getenv` in libc can't be changed to
take a lock because it returns a pointer to the data (see [rust#27970] for more discussion).

Since version 4.20 chrono no longer uses `localtime_r`, instead using Rust code to query the
timezone (from the `TZ` variable or via `iana-time-zone` as a fallback) and work with data
from the system timezone database directly. The code for this was forked from the [tz-rs crate]
by @x-hgg-x. As such, chrono now respects the Rust lock when reading the `TZ` environment
variable. In general, code should avoid modifying the environment.







## Removing time 0.1

Because time 0.1 has been unmaintained for years, however, the security advisory mentioned
above has not been addressed. While chrono maintainers were careful not to break backwards
compatibility with the `time::Duration` type, there has been a long stream of issues from
users inquiring about the time 0.1 dependency with the vulnerability. We investigated the
potential breakage of removing the time 0.1 dependency in [chrono#1095] using a crater-like
experiment and determined that the potential for breaking (public) dependencies is very low.
We reached out to those few crates that did still depend on compatibility with time 0.1.

As such, for chrono 0.4.30 we have decided to swap out the time 0.1 `Duration` implementation
for a local one that will offer a strict superset of the existing API going forward. This
will prevent most downstream users from being affected by the security vulnerability in time
0.1 while minimizing the ecosystem impact of semver-incompatible version churn.


## Contents

- [Modules](#modules)
  - [`time_delta`](#time-delta)
  - [`prelude`](#prelude)
  - [`date`](#date)
  - [`datetime`](#datetime)
  - [`format`](#format)
  - [`naive`](#naive)
  - [`offset`](#offset)
  - [`round`](#round)
  - [`weekday`](#weekday)
  - [`weekday_set`](#weekday-set)
  - [`month`](#month)
  - [`traits`](#traits)
  - [`serde`](#serde)
- [Structs](#structs)
  - [`TimeDelta`](#timedelta)
  - [`Date`](#date)
  - [`DateTime`](#datetime)
  - [`ParseError`](#parseerror)
  - [`Days`](#days)
  - [`NaiveDate`](#naivedate)
  - [`NaiveDateTime`](#naivedatetime)
  - [`NaiveTime`](#naivetime)
  - [`IsoWeek`](#isoweek)
  - [`NaiveWeek`](#naiveweek)
  - [`FixedOffset`](#fixedoffset)
  - [`Utc`](#utc)
  - [`ParseWeekdayError`](#parseweekdayerror)
  - [`WeekdaySet`](#weekdayset)
  - [`ParseMonthError`](#parsemontherror)
  - [`Months`](#months)
  - [`OutOfRange`](#outofrange)
- [Enums](#enums)
  - [`SecondsFormat`](#secondsformat)
  - [`RoundingError`](#roundingerror)
  - [`Weekday`](#weekday)
  - [`Month`](#month)
- [Traits](#traits)
  - [`Offset`](#offset)
  - [`TimeZone`](#timezone)
  - [`DurationRound`](#durationround)
  - [`SubsecRound`](#subsecround)
  - [`Datelike`](#datelike)
  - [`Timelike`](#timelike)
- [Functions](#functions)
  - [`expect`](#expect)
- [Type Aliases](#type-aliases)
  - [`Duration`](#duration)
  - [`ParseResult`](#parseresult)
  - [`MappedLocalTime`](#mappedlocaltime)
- [Constants](#constants)
  - [`MAX_DATE`](#max-date)
  - [`MIN_DATE`](#min-date)
  - [`MAX_DATETIME`](#max-datetime)
  - [`MIN_DATETIME`](#min-datetime)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`time_delta`](#time-delta) | mod | Temporal quantification |
| [`prelude`](#prelude) | mod | A convenience module appropriate for glob imports (`use chrono::prelude::*;`). |
| [`date`](#date) | mod | ISO 8601 calendar date with time zone. |
| [`datetime`](#datetime) | mod | ISO 8601 date and time with time zone. |
| [`format`](#format) | mod | Formatting (and parsing) utilities for date and time. |
| [`naive`](#naive) | mod | Date and time types unconcerned with timezones. |
| [`offset`](#offset) | mod | The time zone, which calculates offsets from the local time to UTC. |
| [`round`](#round) | mod | Functionality for rounding or truncating a `DateTime` by a `TimeDelta`. |
| [`weekday`](#weekday) | mod |  |
| [`weekday_set`](#weekday-set) | mod |  |
| [`month`](#month) | mod |  |
| [`traits`](#traits) | mod |  |
| [`serde`](#serde) | mod | Serialization/Deserialization with serde |
| [`TimeDelta`](#timedelta) | struct |  |
| [`Date`](#date) | struct |  |
| [`DateTime`](#datetime) | struct |  |
| [`ParseError`](#parseerror) | struct |  |
| [`Days`](#days) | struct |  |
| [`NaiveDate`](#naivedate) | struct |  |
| [`NaiveDateTime`](#naivedatetime) | struct |  |
| [`NaiveTime`](#naivetime) | struct |  |
| [`IsoWeek`](#isoweek) | struct |  |
| [`NaiveWeek`](#naiveweek) | struct |  |
| [`FixedOffset`](#fixedoffset) | struct |  |
| [`Utc`](#utc) | struct |  |
| [`ParseWeekdayError`](#parseweekdayerror) | struct |  |
| [`WeekdaySet`](#weekdayset) | struct |  |
| [`ParseMonthError`](#parsemontherror) | struct |  |
| [`Months`](#months) | struct |  |
| [`OutOfRange`](#outofrange) | struct | Out of range error type used in various converting APIs |
| [`SecondsFormat`](#secondsformat) | enum |  |
| [`RoundingError`](#roundingerror) | enum |  |
| [`Weekday`](#weekday) | enum |  |
| [`Month`](#month) | enum |  |
| [`Offset`](#offset) | trait |  |
| [`TimeZone`](#timezone) | trait |  |
| [`DurationRound`](#durationround) | trait |  |
| [`SubsecRound`](#subsecround) | trait |  |
| [`Datelike`](#datelike) | trait |  |
| [`Timelike`](#timelike) | trait |  |
| [`expect`](#expect) | fn | Workaround because `.expect()` is not (yet) available in const context. |
| [`Duration`](#duration) | type | Alias of [`TimeDelta`]. |
| [`ParseResult`](#parseresult) | type |  |
| [`MappedLocalTime`](#mappedlocaltime) | type |  |
| [`MAX_DATE`](#max-date) | const |  |
| [`MIN_DATE`](#min-date) | const |  |
| [`MAX_DATETIME`](#max-datetime) | const |  |
| [`MIN_DATETIME`](#min-datetime) | const |  |

## Modules

- [`time_delta`](time_delta/index.md) — Temporal quantification
- [`prelude`](prelude/index.md) — A convenience module appropriate for glob imports (`use chrono::prelude::*;`).
- [`date`](date/index.md) — ISO 8601 calendar date with time zone.
- [`datetime`](datetime/index.md) — ISO 8601 date and time with time zone.
- [`format`](format/index.md) — Formatting (and parsing) utilities for date and time.
- [`naive`](naive/index.md) — Date and time types unconcerned with timezones.
- [`offset`](offset/index.md) — The time zone, which calculates offsets from the local time to UTC.
- [`round`](round/index.md) — Functionality for rounding or truncating a `DateTime` by a `TimeDelta`.
- [`weekday`](weekday/index.md)
- [`weekday_set`](weekday_set/index.md)
- [`month`](month/index.md)
- [`traits`](traits/index.md)
- [`serde`](serde/index.md) — Serialization/Deserialization with serde

## Structs

### `TimeDelta`

```rust
struct TimeDelta {
    secs: i64,
    nanos: i32,
}
```

Time duration with nanosecond precision.

This also allows for negative durations; see individual methods for details.

A `TimeDelta` is represented internally as a complement of seconds and
nanoseconds. The range is restricted to that of `i64` milliseconds, with the
minimum value notably being set to `-i64::MAX` rather than allowing the full
range of `i64::MIN`. This is to allow easy flipping of sign, so that for
instance `abs()` can be called without any checks.

#### Implementations

- <span id="timedelta-new"></span>`const fn new(secs: i64, nanos: u32) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with given number of seconds and nanoseconds.

  

  # Errors

  

  Returns `None` when the duration is out of bounds, or if `nanos` ≥ 1,000,000,000.

- <span id="timedelta-weeks"></span>`const fn weeks(weeks: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of weeks.

  

  Equivalent to `TimeDelta::seconds(weeks * 7 * 24 * 60 * 60)` with

  overflow checks.

  

  # Panics

  

  Panics when the duration is out of bounds.

- <span id="timedelta-try-weeks"></span>`const fn try_weeks(weeks: i64) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of weeks.

  

  Equivalent to `TimeDelta::try_seconds(weeks * 7 * 24 * 60 * 60)` with

  overflow checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-days"></span>`const fn days(days: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of days.

  

  Equivalent to `TimeDelta::seconds(days * 24 * 60 * 60)` with overflow

  checks.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds.

- <span id="timedelta-try-days"></span>`const fn try_days(days: i64) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of days.

  

  Equivalent to `TimeDelta::try_seconds(days * 24 * 60 * 60)` with overflow

  checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-hours"></span>`const fn hours(hours: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of hours.

  

  Equivalent to `TimeDelta::seconds(hours * 60 * 60)` with overflow checks.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds.

- <span id="timedelta-try-hours"></span>`const fn try_hours(hours: i64) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of hours.

  

  Equivalent to `TimeDelta::try_seconds(hours * 60 * 60)` with overflow checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-minutes"></span>`const fn minutes(minutes: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of minutes.

  

  Equivalent to `TimeDelta::seconds(minutes * 60)` with overflow checks.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds.

- <span id="timedelta-try-minutes"></span>`const fn try_minutes(minutes: i64) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of minutes.

  

  Equivalent to `TimeDelta::try_seconds(minutes * 60)` with overflow checks.

  

  # Errors

  

  Returns `None` when the `TimeDelta` would be out of bounds.

- <span id="timedelta-seconds"></span>`const fn seconds(seconds: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of seconds.

  

  # Panics

  

  Panics when `seconds` is more than `i64::MAX / 1_000` or less than `-i64::MAX / 1_000`

  (in this context, this is the same as `i64::MIN / 1_000` due to rounding).

- <span id="timedelta-try-seconds"></span>`const fn try_seconds(seconds: i64) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of seconds.

  

  # Errors

  

  Returns `None` when `seconds` is more than `i64::MAX / 1_000` or less than

  `-i64::MAX / 1_000` (in this context, this is the same as `i64::MIN / 1_000` due to

  rounding).

- <span id="timedelta-milliseconds"></span>`const fn milliseconds(milliseconds: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of milliseconds.

  

  # Panics

  

  Panics when the `TimeDelta` would be out of bounds, i.e. when `milliseconds` is more than

  `i64::MAX` or less than `-i64::MAX`. Notably, this is not the same as `i64::MIN`.

- <span id="timedelta-try-milliseconds"></span>`const fn try_milliseconds(milliseconds: i64) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of milliseconds.

  

  # Errors

  

  Returns `None` the `TimeDelta` would be out of bounds, i.e. when `milliseconds` is more

  than `i64::MAX` or less than `-i64::MAX`. Notably, this is not the same as `i64::MIN`.

- <span id="timedelta-microseconds"></span>`const fn microseconds(microseconds: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of microseconds.

  

  The number of microseconds acceptable by this constructor is less than

  the total number that can actually be stored in a `TimeDelta`, so it is

  not possible to specify a value that would be out of bounds. This

  function is therefore infallible.

- <span id="timedelta-nanoseconds"></span>`const fn nanoseconds(nanos: i64) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Makes a new `TimeDelta` with the given number of nanoseconds.

  

  The number of nanoseconds acceptable by this constructor is less than

  the total number that can actually be stored in a `TimeDelta`, so it is

  not possible to specify a value that would be out of bounds. This

  function is therefore infallible.

- <span id="timedelta-num-weeks"></span>`const fn num_weeks(&self) -> i64`

  Returns the total number of whole weeks in the `TimeDelta`.

- <span id="timedelta-num-days"></span>`const fn num_days(&self) -> i64`

  Returns the total number of whole days in the `TimeDelta`.

- <span id="timedelta-num-hours"></span>`const fn num_hours(&self) -> i64`

  Returns the total number of whole hours in the `TimeDelta`.

- <span id="timedelta-num-minutes"></span>`const fn num_minutes(&self) -> i64`

  Returns the total number of whole minutes in the `TimeDelta`.

- <span id="timedelta-num-seconds"></span>`const fn num_seconds(&self) -> i64`

  Returns the total number of whole seconds in the `TimeDelta`.

- <span id="timedelta-as-seconds-f64"></span>`fn as_seconds_f64(self) -> f64`

  Returns the fractional number of seconds in the `TimeDelta`.

- <span id="timedelta-as-seconds-f32"></span>`fn as_seconds_f32(self) -> f32`

  Returns the fractional number of seconds in the `TimeDelta`.

- <span id="timedelta-num-milliseconds"></span>`const fn num_milliseconds(&self) -> i64`

  Returns the total number of whole milliseconds in the `TimeDelta`.

- <span id="timedelta-subsec-millis"></span>`const fn subsec_millis(&self) -> i32`

  Returns the number of milliseconds in the fractional part of the duration.

  

  This is the number of milliseconds such that

  `subsec_millis() + num_seconds() * 1_000` is the truncated number of

  milliseconds in the duration.

- <span id="timedelta-num-microseconds"></span>`const fn num_microseconds(&self) -> Option<i64>`

  Returns the total number of whole microseconds in the `TimeDelta`,

  or `None` on overflow (exceeding 2^63 microseconds in either direction).

- <span id="timedelta-subsec-micros"></span>`const fn subsec_micros(&self) -> i32`

  Returns the number of microseconds in the fractional part of the duration.

  

  This is the number of microseconds such that

  `subsec_micros() + num_seconds() * 1_000_000` is the truncated number of

  microseconds in the duration.

- <span id="timedelta-num-nanoseconds"></span>`const fn num_nanoseconds(&self) -> Option<i64>`

  Returns the total number of whole nanoseconds in the `TimeDelta`,

  or `None` on overflow (exceeding 2^63 nanoseconds in either direction).

- <span id="timedelta-subsec-nanos"></span>`const fn subsec_nanos(&self) -> i32`

  Returns the number of nanoseconds in the fractional part of the duration.

  

  This is the number of nanoseconds such that

  `subsec_nanos() + num_seconds() * 1_000_000_000` is the total number of

  nanoseconds in the `TimeDelta`.

- <span id="timedelta-checked-add"></span>`const fn checked_add(&self, rhs: &TimeDelta) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Add two `TimeDelta`s, returning `None` if overflow occurred.

- <span id="timedelta-checked-sub"></span>`const fn checked_sub(&self, rhs: &TimeDelta) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Subtract two `TimeDelta`s, returning `None` if overflow occurred.

- <span id="timedelta-checked-mul"></span>`const fn checked_mul(&self, rhs: i32) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Multiply a `TimeDelta` with a i32, returning `None` if overflow occurred.

- <span id="timedelta-checked-div"></span>`const fn checked_div(&self, rhs: i32) -> Option<TimeDelta>` — [`TimeDelta`](time_delta/index.md#timedelta)

  Divide a `TimeDelta` with a i32, returning `None` if dividing by 0.

- <span id="timedelta-abs"></span>`const fn abs(&self) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Returns the `TimeDelta` as an absolute (non-negative) value.

- <span id="timedelta-min-value"></span>`const fn min_value() -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  The minimum possible `TimeDelta`: `-i64::MAX` milliseconds.

- <span id="timedelta-max-value"></span>`const fn max_value() -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  The maximum possible `TimeDelta`: `i64::MAX` milliseconds.

- <span id="timedelta-zero"></span>`const fn zero() -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  A `TimeDelta` where the stored seconds and nanoseconds are equal to zero.

- <span id="timedelta-is-zero"></span>`const fn is_zero(&self) -> bool`

  Returns `true` if the `TimeDelta` equals `TimeDelta::zero()`.

- <span id="timedelta-from-std"></span>`const fn from_std(duration: Duration) -> Result<TimeDelta, OutOfRangeError>` — [`TimeDelta`](time_delta/index.md#timedelta), [`OutOfRangeError`](time_delta/index.md#outofrangeerror)

  Creates a `TimeDelta` object from `std::time::Duration`

  

  This function errors when original duration is larger than the maximum

  value supported for this type.

- <span id="timedelta-to-std"></span>`const fn to_std(&self) -> Result<Duration, OutOfRangeError>` — [`OutOfRangeError`](time_delta/index.md#outofrangeerror)

  Creates a `std::time::Duration` object from a `TimeDelta`.

  

  This function errors when duration is less than zero. As standard

  library implementation is limited to non-negative values.

- <span id="timedelta-neg"></span>`const fn neg(self) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  This duplicates `Neg::neg` because trait methods can't be const yet.

- <span id="timedelta-const-min"></span>`const MIN: Self`

- <span id="timedelta-const-max"></span>`const MAX: Self`

#### Trait Implementations

##### `impl Add for TimeDelta`

- <span id="timedelta-add-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-add"></span>`fn add(self, rhs: TimeDelta) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl AddAssign for TimeDelta`

- <span id="timedelta-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Clone for TimeDelta`

- <span id="timedelta-clone"></span>`fn clone(&self) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Copy for TimeDelta`

##### `impl Debug for TimeDelta`

- <span id="timedelta-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TimeDelta`

- <span id="timedelta-default"></span>`fn default() -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Deserialize for super::TimeDelta`

- <span id="supertimedelta-deserialize"></span>`fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for TimeDelta`

##### `impl Display for TimeDelta`

- <span id="timedelta-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format a `TimeDelta` using the [ISO 8601] format

##### `impl Div for TimeDelta`

- <span id="timedelta-div-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-div"></span>`fn div(self, rhs: i32) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Eq for TimeDelta`

##### `impl Hash for TimeDelta`

- <span id="timedelta-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Mul for TimeDelta`

- <span id="timedelta-mul-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-mul"></span>`fn mul(self, rhs: i32) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Neg for TimeDelta`

- <span id="timedelta-neg-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-neg"></span>`fn neg(self) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Ord for TimeDelta`

- <span id="timedelta-ord-cmp"></span>`fn cmp(&self, other: &TimeDelta) -> cmp::Ordering` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl PartialEq for TimeDelta`

- <span id="timedelta-partialeq-eq"></span>`fn eq(&self, other: &TimeDelta) -> bool` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl PartialOrd for TimeDelta`

- <span id="timedelta-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &TimeDelta) -> option::Option<cmp::Ordering>` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Serialize for super::TimeDelta`

- <span id="supertimedelta-serialize"></span>`fn serialize<S: Serializer>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for TimeDelta`

##### `impl Sub for TimeDelta`

- <span id="timedelta-sub-type-output"></span>`type Output = TimeDelta`

- <span id="timedelta-sub"></span>`fn sub(self, rhs: TimeDelta) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl SubAssign for TimeDelta`

- <span id="timedelta-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Sum for TimeDelta`

- <span id="timedelta-sum"></span>`fn sum<I: Iterator<Item = &'a TimeDelta>>(iter: I) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl ToString for TimeDelta`

- <span id="timedelta-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Date<Tz: TimeZone>`

```rust
struct Date<Tz: TimeZone> {
    date: crate::naive::NaiveDate,
    offset: <Tz as >::Offset,
}
```

ISO 8601 calendar date with time zone.

You almost certainly want to be using a [`NaiveDate`](naive/date/index.md) instead of this type.

This type primarily exists to aid in the construction of DateTimes that
have a timezone by way of the [`TimeZone`](offset/index.md) datelike constructors (e.g.
`TimeZone::ymd`).

This type should be considered ambiguous at best, due to the inherent lack
of precision required for the time zone resolution.

There are some guarantees on the usage of `Date<Tz>`:

- If properly constructed via `TimeZone::ymd` and others without an error,
  the corresponding local date should exist for at least a moment.
  (It may still have a gap from the offset changes.)

- The `TimeZone` is free to assign *any* [`Offset`](crate::offset::Offset) to the
  local date, as long as that offset did occur in given day.

  For example, if `2015-03-08T01:59-08:00` is followed by `2015-03-08T03:00-07:00`,
  it may produce either `2015-03-08-08:00` or `2015-03-08-07:00`
  but *not* `2015-03-08+00:00` and others.

- Once constructed as a full `DateTime`, `DateTime::date` and other associated
  methods should return those for the original `Date`. For example, if `dt =
  tz.ymd_opt(y,m,d).unwrap().hms(h,n,s)` were valid, `dt.date() == tz.ymd_opt(y,m,d).unwrap()`.

- The date is timezone-agnostic up to one day (i.e. practically always),
  so the local date and UTC date should be equal for most cases
  even though the raw calculation between `NaiveDate` and `TimeDelta` may not.

#### Implementations

- <span id="date-from-utc"></span>`fn from_utc(date: NaiveDate, offset: <Tz as >::Offset) -> Date<Tz>` — [`NaiveDate`](naive/date/index.md#naivedate), [`TimeZone`](offset/index.md#timezone), [`Date`](date/index.md#date)

  Makes a new `Date` with given *UTC* date and offset.

  The local date should be constructed via the `TimeZone` trait.

- <span id="date-and-time"></span>`fn and_time(&self, time: NaiveTime) -> Option<DateTime<Tz>>` — [`NaiveTime`](naive/time/index.md#naivetime), [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date and given `NaiveTime`.

  The offset in the current date is preserved.

  

  Returns `None` on invalid datetime.

- <span id="date-and-hms"></span>`fn and_hms(&self, hour: u32, min: u32, sec: u32) -> DateTime<Tz>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute and second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute and/or second.

- <span id="date-and-hms-opt"></span>`fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute and second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute and/or second.

- <span id="date-and-hms-milli"></span>`fn and_hms_milli(&self, hour: u32, min: u32, sec: u32, milli: u32) -> DateTime<Tz>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and millisecond.

  The millisecond part can exceed 1,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="date-and-hms-milli-opt"></span>`fn and_hms_milli_opt(&self, hour: u32, min: u32, sec: u32, milli: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and millisecond.

  The millisecond part can exceed 1,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute, second and/or millisecond.

- <span id="date-and-hms-micro"></span>`fn and_hms_micro(&self, hour: u32, min: u32, sec: u32, micro: u32) -> DateTime<Tz>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and microsecond.

  The microsecond part can exceed 1,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute, second and/or microsecond.

- <span id="date-and-hms-micro-opt"></span>`fn and_hms_micro_opt(&self, hour: u32, min: u32, sec: u32, micro: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and microsecond.

  The microsecond part can exceed 1,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute, second and/or microsecond.

- <span id="date-and-hms-nano"></span>`fn and_hms_nano(&self, hour: u32, min: u32, sec: u32, nano: u32) -> DateTime<Tz>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.

  The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="date-and-hms-nano-opt"></span>`fn and_hms_nano_opt(&self, hour: u32, min: u32, sec: u32, nano: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from the current date, hour, minute, second and nanosecond.

  The nanosecond part can exceed 1,000,000,000 in order to represent the leap second.

  The offset in the current date is preserved.

  

  Returns `None` on invalid hour, minute, second and/or nanosecond.

- <span id="date-succ"></span>`fn succ(&self) -> Date<Tz>` — [`Date`](date/index.md#date)

  Makes a new `Date` for the next date.

  

  Panics when `self` is the last representable date.

- <span id="date-succ-opt"></span>`fn succ_opt(&self) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

  Makes a new `Date` for the next date.

  

  Returns `None` when `self` is the last representable date.

- <span id="date-pred"></span>`fn pred(&self) -> Date<Tz>` — [`Date`](date/index.md#date)

  Makes a new `Date` for the prior date.

  

  Panics when `self` is the first representable date.

- <span id="date-pred-opt"></span>`fn pred_opt(&self) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

  Makes a new `Date` for the prior date.

  

  Returns `None` when `self` is the first representable date.

- <span id="date-offset"></span>`fn offset(&self) -> &<Tz as >::Offset` — [`TimeZone`](offset/index.md#timezone)

  Retrieves an associated offset from UTC.

- <span id="date-timezone"></span>`fn timezone(&self) -> Tz`

  Retrieves an associated time zone.

- <span id="date-with-timezone"></span>`fn with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> Date<Tz2>` — [`Date`](date/index.md#date)

  Changes the associated time zone.

  This does not change the actual `Date` (but will change the string representation).

- <span id="date-checked-add-signed"></span>`fn checked_add_signed(self, rhs: TimeDelta) -> Option<Date<Tz>>` — [`TimeDelta`](time_delta/index.md#timedelta), [`Date`](date/index.md#date)

  Adds given `TimeDelta` to the current date.

  

  Returns `None` when it will result in overflow.

- <span id="date-checked-sub-signed"></span>`fn checked_sub_signed(self, rhs: TimeDelta) -> Option<Date<Tz>>` — [`TimeDelta`](time_delta/index.md#timedelta), [`Date`](date/index.md#date)

  Subtracts given `TimeDelta` from the current date.

  

  Returns `None` when it will result in overflow.

- <span id="date-signed-duration-since"></span>`fn signed_duration_since<Tz2: TimeZone>(self, rhs: Date<Tz2>) -> TimeDelta` — [`Date`](date/index.md#date), [`TimeDelta`](time_delta/index.md#timedelta)

  Subtracts another `Date` from the current date.

  Returns a `TimeDelta` of integral numbers.

  

  This does not overflow or underflow at all,

  as all possible output fits in the range of `TimeDelta`.

- <span id="date-naive-utc"></span>`fn naive_utc(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns a view to the naive UTC date.

- <span id="date-naive-local"></span>`fn naive_local(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns a view to the naive local date.

  

  This is technically the same as [`naive_utc`](#method.naive_utc)

  because the offset is restricted to never exceed one day,

  but provided for the consistency.

- <span id="date-years-since"></span>`fn years_since(&self, base: Self) -> Option<u32>`

  Returns the number of whole years from the given `base` until `self`.

- <span id="date-const-min-utc"></span>`const MIN_UTC: Date<Utc>`

- <span id="date-const-max-utc"></span>`const MAX_UTC: Date<Utc>`

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for Date<Tz>`

- <span id="date-add-type-output"></span>`type Output = Date<Tz>`

- <span id="date-add"></span>`fn add(self, rhs: TimeDelta) -> Date<Tz>` — [`TimeDelta`](time_delta/index.md#timedelta), [`Date`](date/index.md#date)

##### `impl<Tz: TimeZone> AddAssign for Date<Tz>`

- <span id="date-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl<Tz: clone::Clone + TimeZone> Clone for Date<Tz>`

- <span id="date-clone"></span>`fn clone(&self) -> Date<Tz>` — [`Date`](date/index.md#date)

##### `impl<Tz: TimeZone> Copy for Date<Tz>`

##### `impl<Tz: TimeZone> Datelike for Date<Tz>`

- <span id="date-datelike-year"></span>`fn year(&self) -> i32`

- <span id="date-datelike-month"></span>`fn month(&self) -> u32`

- <span id="date-datelike-month0"></span>`fn month0(&self) -> u32`

- <span id="date-datelike-day"></span>`fn day(&self) -> u32`

- <span id="date-datelike-day0"></span>`fn day0(&self) -> u32`

- <span id="date-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

- <span id="date-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

- <span id="date-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

- <span id="date-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

- <span id="date-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

- <span id="date-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

- <span id="date-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

- <span id="date-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

- <span id="date-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

- <span id="date-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

- <span id="date-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<Date<Tz>>` — [`Date`](date/index.md#date)

##### `impl<Tz: TimeZone> Debug for Date<Tz>`

- <span id="date-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Tz: TimeZone> Display for Date<Tz>`

- <span id="date-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Tz: TimeZone> Eq for Date<Tz>`

##### `impl<Tz: TimeZone> Hash for Date<Tz>`

- <span id="date-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<Tz: TimeZone> Ord for Date<Tz>`

- <span id="date-ord-cmp"></span>`fn cmp(&self, other: &Date<Tz>) -> Ordering` — [`Date`](date/index.md#date)

##### `impl<Tz: TimeZone, Tz2: TimeZone> PartialEq for Date<Tz>`

- <span id="date-partialeq-eq"></span>`fn eq(&self, other: &Date<Tz2>) -> bool` — [`Date`](date/index.md#date)

##### `impl<Tz: TimeZone> PartialOrd for Date<Tz>`

- <span id="date-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Date<Tz>) -> Option<Ordering>` — [`Date`](date/index.md#date)

##### `impl<Tz: TimeZone> Send for Date<Tz>`

##### `impl<Tz: TimeZone> Sub for Date<Tz>`

- <span id="date-sub-type-output"></span>`type Output = Date<Tz>`

- <span id="date-sub"></span>`fn sub(self, rhs: TimeDelta) -> Date<Tz>` — [`TimeDelta`](time_delta/index.md#timedelta), [`Date`](date/index.md#date)

##### `impl<Tz: TimeZone> SubAssign for Date<Tz>`

- <span id="date-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl ToString for Date<Tz>`

- <span id="date-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DateTime<Tz: TimeZone>`

```rust
struct DateTime<Tz: TimeZone> {
    datetime: crate::naive::NaiveDateTime,
    offset: <Tz as >::Offset,
}
```

ISO 8601 combined date and time with time zone.

There are some constructors implemented here (the `from_*` methods), but
the general-purpose constructors are all via the methods on the
[`TimeZone`](./offset/trait.TimeZone.html) implementations.

#### Implementations

- <span id="datetime-from-naive-utc-and-offset"></span>`const fn from_naive_utc_and_offset(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`TimeZone`](offset/index.md#timezone), [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from its components: a `NaiveDateTime` in UTC and an `Offset`.

  

  This is a low-level method, intended for use cases such as deserializing a `DateTime` or

  passing it through FFI.

  

  For regular use you will probably want to use a method such as

  `TimeZone::from_local_datetime` or `NaiveDateTime::and_local_timezone` instead.

  

  # Example

  

  ```rust

  #[cfg(feature = "clock")] {

  use chrono::{DateTime, Local};

  

  let dt = Local::now();

  // Get components

  let naive_utc = dt.naive_utc();

  let offset = dt.offset().clone();

  // Serialize, pass through FFI... and recreate the `DateTime`:

  let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);

  assert_eq!(dt, dt_new);

  }

  ```

- <span id="datetime-from-utc"></span>`fn from_utc(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`TimeZone`](offset/index.md#timezone), [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from its components: a `NaiveDateTime` in UTC and an `Offset`.

- <span id="datetime-from-local"></span>`fn from_local(datetime: NaiveDateTime, offset: <Tz as >::Offset) -> DateTime<Tz>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`TimeZone`](offset/index.md#timezone), [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` from a `NaiveDateTime` in *local* time and an `Offset`.

  

  # Panics

  

  Panics if the local datetime can't be converted to UTC because it would be out of range.

  

  This can happen if `datetime` is near the end of the representable range of `NaiveDateTime`,

  and the offset from UTC pushes it beyond that.

- <span id="datetime-date"></span>`fn date(&self) -> Date<Tz>` — [`Date`](date/index.md#date)

  Retrieves the date component with an associated timezone.

  

  Unless you are immediately planning on turning this into a `DateTime`

  with the same timezone you should use the [`date_naive`](DateTime::date_naive) method.

  

  [`NaiveDate`](naive/date/index.md) is a more well-defined type, and has more traits implemented on it,

  so should be preferred to [`Date`](date/index.md) any time you truly want to operate on dates.

  

  # Panics

  

  [`DateTime`](datetime/index.md) internally stores the date and time in UTC with a [`NaiveDateTime`](naive/datetime/index.md). This

  method will panic if the offset from UTC would push the local date outside of the

  representable range of a [`Date`](date/index.md).

- <span id="datetime-date-naive"></span>`fn date_naive(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Retrieves the date component.

  

  # Panics

  

  [`DateTime`](datetime/index.md) internally stores the date and time in UTC with a [`NaiveDateTime`](naive/datetime/index.md). This

  method will panic if the offset from UTC would push the local date outside of the

  representable range of a [`NaiveDate`](naive/date/index.md).

  

  # Example

  

  ```rust

  use chrono::prelude::*;

  

  let date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

  let other: DateTime<FixedOffset> =

      FixedOffset::east_opt(23).unwrap().with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

  assert_eq!(date.date_naive(), other.date_naive());

  ```

- <span id="datetime-time"></span>`fn time(&self) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

  Retrieves the time component.

- <span id="datetime-timestamp"></span>`const fn timestamp(&self) -> i64`

  Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC

  (aka "UNIX timestamp").

  

  The reverse operation of creating a [`DateTime`](datetime/index.md) from a timestamp can be performed

  using [`from_timestamp`](DateTime::from_timestamp) or `TimeZone::timestamp_opt`.

  

  ```rust

  use chrono::{DateTime, TimeZone, Utc};

  

  let dt: DateTime<Utc> = Utc.with_ymd_and_hms(2015, 5, 15, 0, 0, 0).unwrap();

  assert_eq!(dt.timestamp(), 1431648000);

  

  assert_eq!(DateTime::from_timestamp(dt.timestamp(), dt.timestamp_subsec_nanos()).unwrap(), dt);

  ```

- <span id="datetime-timestamp-millis"></span>`const fn timestamp_millis(&self) -> i64`

  Returns the number of non-leap-milliseconds since January 1, 1970 UTC.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  

  let dt = NaiveDate::from_ymd_opt(1970, 1, 1)

      .unwrap()

      .and_hms_milli_opt(0, 0, 1, 444)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_millis(), 1_444);

  

  let dt = NaiveDate::from_ymd_opt(2001, 9, 9)

      .unwrap()

      .and_hms_milli_opt(1, 46, 40, 555)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_millis(), 1_000_000_000_555);

  ```

- <span id="datetime-timestamp-micros"></span>`const fn timestamp_micros(&self) -> i64`

  Returns the number of non-leap-microseconds since January 1, 1970 UTC.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  

  let dt = NaiveDate::from_ymd_opt(1970, 1, 1)

      .unwrap()

      .and_hms_micro_opt(0, 0, 1, 444)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_micros(), 1_000_444);

  

  let dt = NaiveDate::from_ymd_opt(2001, 9, 9)

      .unwrap()

      .and_hms_micro_opt(1, 46, 40, 555)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_micros(), 1_000_000_000_000_555);

  ```

- <span id="datetime-timestamp-nanos"></span>`const fn timestamp_nanos(&self) -> i64`

  Returns the number of non-leap-nanoseconds since January 1, 1970 UTC.

  

  # Panics

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function panics on

  an out of range `DateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

- <span id="datetime-timestamp-nanos-opt"></span>`const fn timestamp_nanos_opt(&self) -> Option<i64>`

  Returns the number of non-leap-nanoseconds since January 1, 1970 UTC.

  

  # Errors

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function returns

  `None` on an out of range `DateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  

  let dt = NaiveDate::from_ymd_opt(1970, 1, 1)

      .unwrap()

      .and_hms_nano_opt(0, 0, 1, 444)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(1_000_000_444));

  

  let dt = NaiveDate::from_ymd_opt(2001, 9, 9)

      .unwrap()

      .and_hms_nano_opt(1, 46, 40, 555)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(1_000_000_000_000_000_555));

  

  let dt = NaiveDate::from_ymd_opt(1677, 9, 21)

      .unwrap()

      .and_hms_nano_opt(0, 12, 43, 145_224_192)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(-9_223_372_036_854_775_808));

  

  let dt = NaiveDate::from_ymd_opt(2262, 4, 11)

      .unwrap()

      .and_hms_nano_opt(23, 47, 16, 854_775_807)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), Some(9_223_372_036_854_775_807));

  

  let dt = NaiveDate::from_ymd_opt(1677, 9, 21)

      .unwrap()

      .and_hms_nano_opt(0, 12, 43, 145_224_191)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), None);

  

  let dt = NaiveDate::from_ymd_opt(2262, 4, 11)

      .unwrap()

      .and_hms_nano_opt(23, 47, 16, 854_775_808)

      .unwrap()

      .and_local_timezone(Utc)

      .unwrap();

  assert_eq!(dt.timestamp_nanos_opt(), None);

  ```

- <span id="datetime-timestamp-subsec-millis"></span>`const fn timestamp_subsec_millis(&self) -> u32`

  Returns the number of milliseconds since the last second boundary.

  

  In event of a leap second this may exceed 999.

- <span id="datetime-timestamp-subsec-micros"></span>`const fn timestamp_subsec_micros(&self) -> u32`

  Returns the number of microseconds since the last second boundary.

  

  In event of a leap second this may exceed 999,999.

- <span id="datetime-timestamp-subsec-nanos"></span>`const fn timestamp_subsec_nanos(&self) -> u32`

  Returns the number of nanoseconds since the last second boundary

  

  In event of a leap second this may exceed 999,999,999.

- <span id="datetime-offset"></span>`const fn offset(&self) -> &<Tz as >::Offset` — [`TimeZone`](offset/index.md#timezone)

  Retrieves an associated offset from UTC.

- <span id="datetime-timezone"></span>`fn timezone(&self) -> Tz`

  Retrieves an associated time zone.

- <span id="datetime-with-timezone"></span>`fn with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> DateTime<Tz2>` — [`DateTime`](datetime/index.md#datetime)

  Changes the associated time zone.

  The returned `DateTime` references the same instant of time from the perspective of the

  provided time zone.

- <span id="datetime-fixed-offset"></span>`fn fixed_offset(&self) -> DateTime<FixedOffset>` — [`DateTime`](datetime/index.md#datetime), [`FixedOffset`](offset/fixed/index.md#fixedoffset)

  Fix the offset from UTC to its current value, dropping the associated timezone information.

  This is useful for converting a generic `DateTime<Tz: Timezone>` to `DateTime<FixedOffset>`.

- <span id="datetime-to-utc"></span>`const fn to_utc(&self) -> DateTime<Utc>` — [`DateTime`](datetime/index.md#datetime), [`Utc`](offset/utc/index.md#utc)

  Turn this `DateTime` into a `DateTime<Utc>`, dropping the offset and associated timezone

  information.

- <span id="datetime-checked-add-signed"></span>`fn checked_add_signed(self, rhs: TimeDelta) -> Option<DateTime<Tz>>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DateTime`](datetime/index.md#datetime)

  Adds given `TimeDelta` to the current date and time.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

- <span id="datetime-checked-add-months"></span>`fn checked_add_months(self, months: Months) -> Option<DateTime<Tz>>` — [`Months`](month/index.md#months), [`DateTime`](datetime/index.md#datetime)

  Adds given `Months` to the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  See `NaiveDate::checked_add_months` for more details on behavior.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `months` is zero).

- <span id="datetime-checked-sub-signed"></span>`fn checked_sub_signed(self, rhs: TimeDelta) -> Option<DateTime<Tz>>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DateTime`](datetime/index.md#datetime)

  Subtracts given `TimeDelta` from the current date and time.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

- <span id="datetime-checked-sub-months"></span>`fn checked_sub_months(self, months: Months) -> Option<DateTime<Tz>>` — [`Months`](month/index.md#months), [`DateTime`](datetime/index.md#datetime)

  Subtracts given `Months` from the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  See `NaiveDate::checked_sub_months` for more details on behavior.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `months` is zero).

- <span id="datetime-checked-add-days"></span>`fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](naive/index.md#days)

  Add a duration in [`Days`](naive/index.md) to the date part of the `DateTime`.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `days` is zero).

- <span id="datetime-checked-sub-days"></span>`fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](naive/index.md#days)

  Subtract a duration in [`Days`](naive/index.md) from the date part of the `DateTime`.

  

  # Errors

  

  Returns `None` if:

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless `days` is zero).

- <span id="datetime-signed-duration-since"></span>`fn signed_duration_since<Tz2: TimeZone>(self, rhs: impl Borrow<DateTime<Tz2>>) -> TimeDelta` — [`DateTime`](datetime/index.md#datetime), [`TimeDelta`](time_delta/index.md#timedelta)

  Subtracts another `DateTime` from the current date and time.

  This does not overflow or underflow at all.

- <span id="datetime-naive-utc"></span>`const fn naive_utc(&self) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Returns a view to the naive UTC datetime.

- <span id="datetime-naive-local"></span>`fn naive_local(&self) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Returns a view to the naive local datetime.

  

  # Panics

  

  [`DateTime`](datetime/index.md) internally stores the date and time in UTC with a [`NaiveDateTime`](naive/datetime/index.md). This

  method will panic if the offset from UTC would push the local datetime outside of the

  representable range of a [`NaiveDateTime`](naive/datetime/index.md).

- <span id="datetime-overflowing-naive-local"></span>`fn overflowing_naive_local(&self) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Returns the naive local datetime.

  

  This makes use of the buffer space outside of the representable range of values of

  `NaiveDateTime`. The result can be used as intermediate value, but should never be exposed

  outside chrono.

- <span id="datetime-years-since"></span>`fn years_since(&self, base: Self) -> Option<u32>`

  Retrieve the elapsed years from now to the given [`DateTime`](datetime/index.md).

  

  # Errors

  

  Returns `None` if `base > self`.

- <span id="datetime-with-time"></span>`fn with_time(&self, time: NaiveTime) -> LocalResult<Self>` — [`NaiveTime`](naive/time/index.md#naivetime), [`LocalResult`](offset/index.md#localresult)

  Set the time to a new fixed time on the existing date.

  

  # Errors

  

  Returns `LocalResult::None` if the datetime is at the edge of the representable range for a

  `DateTime`, and `with_time` would push the value in UTC out of range.

  

  # Example

  

  ```rust

  #[cfg(feature = "clock")] {

  use chrono::{Local, NaiveTime};

  

  let noon = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

  let today_noon = Local::now().with_time(noon);

  let today_midnight = Local::now().with_time(NaiveTime::MIN);

  

  assert_eq!(today_noon.single().unwrap().time(), noon);

  assert_eq!(today_midnight.single().unwrap().time(), NaiveTime::MIN);

  }

  ```

- <span id="datetime-const-min-utc"></span>`const MIN_UTC: DateTime<Utc>`

- <span id="datetime-const-max-utc"></span>`const MAX_UTC: DateTime<Utc>`

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, rhs: TimeDelta) -> DateTime<Tz>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DateTime`](datetime/index.md#datetime)

##### `impl<Tz: TimeZone> AddAssign for DateTime<Tz>`

- <span id="datetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl<Tz: clone::Clone + TimeZone> Clone for DateTime<Tz>`

- <span id="datetime-clone"></span>`fn clone(&self) -> DateTime<Tz>` — [`DateTime`](datetime/index.md#datetime)

##### `impl<Tz: TimeZone> Copy for DateTime<Tz>`

##### `impl<Tz: TimeZone> Datelike for DateTime<Tz>`

- <span id="datetime-datelike-year"></span>`fn year(&self) -> i32`

- <span id="datetime-datelike-month"></span>`fn month(&self) -> u32`

- <span id="datetime-datelike-month0"></span>`fn month0(&self) -> u32`

- <span id="datetime-datelike-day"></span>`fn day(&self) -> u32`

- <span id="datetime-datelike-day0"></span>`fn day0(&self) -> u32`

- <span id="datetime-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

- <span id="datetime-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

- <span id="datetime-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

- <span id="datetime-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

- <span id="datetime-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the year number changed, while keeping the same month and day.

  

  See also the `NaiveDate::with_year` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (February 29 in a non-leap year).

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

  - The resulting UTC datetime would be out of range.

  - The resulting local datetime would be out of range (unless the year remains the same).

- <span id="datetime-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the month number (starting from 1) changed.

  

  Don't combine multiple `Datelike::with_*` methods. The intermediate value may not exist.

  

  See also the `NaiveDate::with_month` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month(4)` when day of the month is 31).

  - The value for `month` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the month number (starting from 0) changed.

  

  See also the `NaiveDate::with_month0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month0(3)` when day of the month is 31).

  - The value for `month0` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the day of month (starting from 1) changed.

  

  See also the `NaiveDate::with_day` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(31)` in April).

  - The value for `day` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the day of month (starting from 0) changed.

  

  See also the `NaiveDate::with_day0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(30)` in April).

  - The value for `day0` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the day of year (starting from 1) changed.

  

  See also the `NaiveDate::with_ordinal` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal(366)` in a non-leap year).

  - The value for `ordinal` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the day of year (starting from 0) changed.

  

  See also the `NaiveDate::with_ordinal0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal0(365)` in a non-leap year).

  - The value for `ordinal0` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

##### `impl<Tz: TimeZone> Debug for DateTime<Tz>`

- <span id="datetime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DateTime<crate::offset::Utc>`

- <span id="datetime-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::DateTime<crate::offset::FixedOffset>`

- <span id="superdatetime-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for DateTime<Tz>`

##### `impl<Tz: TimeZone> Display for DateTime<Tz>`

- <span id="datetime-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Tz: TimeZone> DurationRound for crate::DateTime<Tz>`

- <span id="cratedatetime-durationround-type-err"></span>`type Err = RoundingError`

- <span id="cratedatetime-durationround-duration-round"></span>`fn duration_round(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DurationRound`](round/index.md#durationround)

- <span id="cratedatetime-durationround-duration-trunc"></span>`fn duration_trunc(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DurationRound`](round/index.md#durationround)

- <span id="cratedatetime-durationround-duration-round-up"></span>`fn duration_round_up(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DurationRound`](round/index.md#durationround)

##### `impl<Tz: TimeZone> Eq for DateTime<Tz>`

##### `impl FromStr for DateTime<crate::offset::Utc>`

- <span id="datetime-fromstr-type-err"></span>`type Err = ParseError`

- <span id="datetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<DateTime<Utc>>` — [`ParseResult`](format/index.md#parseresult), [`DateTime`](datetime/index.md#datetime), [`Utc`](offset/utc/index.md#utc)

##### `impl<Tz: TimeZone> Hash for DateTime<Tz>`

- <span id="datetime-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<Tz: TimeZone> Ord for DateTime<Tz>`

- <span id="datetime-ord-cmp"></span>`fn cmp(&self, other: &DateTime<Tz>) -> Ordering` — [`DateTime`](datetime/index.md#datetime)

##### `impl<Tz: TimeZone, Tz2: TimeZone> PartialEq for DateTime<Tz>`

- <span id="datetime-partialeq-eq"></span>`fn eq(&self, other: &DateTime<Tz2>) -> bool` — [`DateTime`](datetime/index.md#datetime)

##### `impl<Tz: TimeZone, Tz2: TimeZone> PartialOrd for DateTime<Tz>`

- <span id="datetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DateTime<Tz2>) -> Option<Ordering>` — [`DateTime`](datetime/index.md#datetime)

  Compare two DateTimes based on their true time, ignoring time zones

  

  # Example

  

  ```rust

  use chrono::prelude::*;

  

  let earlier = Utc

      .with_ymd_and_hms(2015, 5, 15, 2, 0, 0)

      .unwrap()

      .with_timezone(&FixedOffset::west_opt(1 * 3600).unwrap());

  let later = Utc

      .with_ymd_and_hms(2015, 5, 15, 3, 0, 0)

      .unwrap()

      .with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());

  

  assert_eq!(earlier.to_string(), "2015-05-15 01:00:00 -01:00");

  assert_eq!(later.to_string(), "2015-05-14 22:00:00 -05:00");

  

  assert!(later > earlier);

  ```

##### `impl<Tz: TimeZone> Serialize for super::DateTime<Tz>`

- <span id="superdatetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> DateTime<Tz>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DateTime`](datetime/index.md#datetime)

##### `impl<Tz: TimeZone> SubAssign for DateTime<Tz>`

- <span id="datetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl SubsecRound for DateTime<Tz>`

- <span id="datetime-subsecround-round-subsecs"></span>`fn round_subsecs(self, digits: u16) -> T`

- <span id="datetime-subsecround-trunc-subsecs"></span>`fn trunc_subsecs(self, digits: u16) -> T`

##### `impl<Tz: TimeZone> Timelike for DateTime<Tz>`

- <span id="datetime-timelike-hour"></span>`fn hour(&self) -> u32`

- <span id="datetime-timelike-minute"></span>`fn minute(&self) -> u32`

- <span id="datetime-timelike-second"></span>`fn second(&self) -> u32`

- <span id="datetime-timelike-nanosecond"></span>`fn nanosecond(&self) -> u32`

- <span id="datetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the hour number changed.

  

  See also the `NaiveTime::with_hour` method.

  

  # Errors

  

  Returns `None` if:

  - The value for `hour` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the minute number changed.

  

  See also the `NaiveTime::with_minute` method.

  

  # Errors

  

  - The value for `minute` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with the second number changed.

  

  As with the [`second`](#method.second) method,

  the input range is restricted to 0 through 59.

  

  See also the `NaiveTime::with_second` method.

  

  # Errors

  

  Returns `None` if:

  - The value for `second` is invalid.

  - The local time at the resulting date does not exist or is ambiguous, for example during a

    daylight saving time transition.

- <span id="datetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<DateTime<Tz>>` — [`DateTime`](datetime/index.md#datetime)

  Makes a new `DateTime` with nanoseconds since the whole non-leap second changed.

  

  Returns `None` when the resulting `NaiveDateTime` would be invalid.

  As with the `NaiveDateTime::nanosecond` method,

  the input range can exceed 1,000,000,000 for leap seconds.

  

  See also the `NaiveTime::with_nanosecond` method.

  

  # Errors

  

  Returns `None` if `nanosecond >= 2,000,000,000`.

##### `impl ToString for DateTime<Tz>`

- <span id="datetime-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ParseError`

```rust
struct ParseError(ParseErrorKind);
```

An error from the `parse` function.

#### Implementations

- <span id="parseerror-kind"></span>`const fn kind(&self) -> ParseErrorKind` — [`ParseErrorKind`](format/index.md#parseerrorkind)

  The category of parse error

#### Trait Implementations

##### `impl Clone for ParseError`

- <span id="parseerror-clone"></span>`fn clone(&self) -> ParseError` — [`ParseError`](format/index.md#parseerror)

##### `impl Copy for ParseError`

##### `impl Debug for ParseError`

- <span id="parseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseError`

- <span id="parseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseError`

##### `impl Hash for ParseError`

- <span id="parseerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ParseError`

- <span id="parseerror-partialeq-eq"></span>`fn eq(&self, other: &ParseError) -> bool` — [`ParseError`](format/index.md#parseerror)

##### `impl StructuralPartialEq for ParseError`

##### `impl ToString for ParseError`

- <span id="parseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Days`

```rust
struct Days(u64);
```

A duration in calendar days.

This is useful because when using `TimeDelta` it is possible that adding `TimeDelta::days(1)`
doesn't increment the day value as expected due to it being a fixed number of seconds. This
difference applies only when dealing with `DateTime<TimeZone>` data types and in other cases
`TimeDelta::days(n)` and `Days::new(n)` are equivalent.

#### Implementations

- <span id="days-new"></span>`const fn new(num: u64) -> Self`

  Construct a new `Days` from a number of days

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, days: Days) -> <Self as >::Output` — [`Days`](naive/index.md#days)

##### `impl Clone for Days`

- <span id="days-clone"></span>`fn clone(&self) -> Days` — [`Days`](naive/index.md#days)

##### `impl Copy for Days`

##### `impl Debug for Days`

- <span id="days-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Days`

##### `impl Hash for Days`

- <span id="days-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Days`

- <span id="days-ord-cmp"></span>`fn cmp(&self, other: &Days) -> cmp::Ordering` — [`Days`](naive/index.md#days)

##### `impl PartialEq for Days`

- <span id="days-partialeq-eq"></span>`fn eq(&self, other: &Days) -> bool` — [`Days`](naive/index.md#days)

##### `impl PartialOrd for Days`

- <span id="days-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Days) -> option::Option<cmp::Ordering>` — [`Days`](naive/index.md#days)

##### `impl StructuralPartialEq for Days`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, days: Days) -> <Self as >::Output` — [`Days`](naive/index.md#days)

### `NaiveDate`

```rust
struct NaiveDate {
    yof: core::num::NonZeroI32,
}
```

ISO 8601 calendar date without timezone.
Allows for every [proleptic Gregorian date] from Jan 1, 262145 BCE to Dec 31, 262143 CE.
Also supports the conversion from ISO 8601 ordinal and week date.

# Calendar Date

The ISO 8601 **calendar date** follows the proleptic Gregorian calendar.
It is like a normal civil calendar but note some slight differences:

* Dates before the Gregorian calendar's inception in 1582 are defined via the extrapolation.
  Be careful, as historical dates are often noted in the Julian calendar and others
  and the transition to Gregorian may differ across countries (as late as early 20C).

  (Some example: Both Shakespeare from Britain and Cervantes from Spain seemingly died
  on the same calendar date---April 23, 1616---but in the different calendar.
  Britain used the Julian calendar at that time, so Shakespeare's death is later.)

* ISO 8601 calendars have the year 0, which is 1 BCE (a year before 1 CE).
  If you need a typical BCE/BC and CE/AD notation for year numbers,
  use the `Datelike::year_ce` method.

# Week Date

The ISO 8601 **week date** is a triple of year number, week number
and [day of the week](Weekday) with the following rules:

* A week consists of Monday through Sunday, and is always numbered within some year.
  The week number ranges from 1 to 52 or 53 depending on the year.

* The week 1 of given year is defined as the first week containing January 4 of that year,
  or equivalently, the first week containing four or more days in that year.

* The year number in the week date may *not* correspond to the actual Gregorian year.
  For example, January 3, 2016 (Sunday) was on the last (53rd) week of 2015.

Chrono's date types default to the ISO 8601 [calendar date](#calendar-date), but
`Datelike::iso_week` and `Datelike::weekday` methods can be used to get the corresponding
week date.

# Ordinal Date

The ISO 8601 **ordinal date** is a pair of year number and day of the year ("ordinal").
The ordinal number ranges from 1 to 365 or 366 depending on the year.
The year number is the same as that of the [calendar date](#calendar-date).

This is currently the internal format of Chrono's date types.


#### Implementations

- <span id="naivedate-weeks-from"></span>`fn weeks_from(&self, day: Weekday) -> i32` — [`Weekday`](weekday/index.md#weekday)

- <span id="naivedate-from-ordinal-and-flags"></span>`const fn from_ordinal_and_flags(year: i32, ordinal: u32, flags: YearFlags) -> Option<NaiveDate>` — [`YearFlags`](naive/internals/index.md#yearflags), [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from year, ordinal and flags.

  Does not check whether the flags are correct for the provided year.

- <span id="naivedate-from-mdf"></span>`const fn from_mdf(year: i32, mdf: Mdf) -> Option<NaiveDate>` — [`Mdf`](naive/internals/index.md#mdf), [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from year and packed month-day-flags.

  Does not check whether the flags are correct for the provided year.

- <span id="naivedate-from-ymd"></span>`const fn from_ymd(year: i32, month: u32, day: u32) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from the [calendar date](#calendar-date)

  (year, month and day).

  

  # Panics

  

  Panics if the specified calendar day does not exist, on invalid values for `month` or `day`,

  or if `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-ymd-opt"></span>`const fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from the [calendar date](#calendar-date)

  (year, month and day).

  

  # Errors

  

  Returns `None` if:

  - The specified calendar day does not exist (for example 2023-04-31).

  - The value for `month` or `day` is invalid.

  - `year` is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ymd_opt = NaiveDate::from_ymd_opt;

  

  assert!(from_ymd_opt(2015, 3, 14).is_some());

  assert!(from_ymd_opt(2015, 0, 14).is_none());

  assert!(from_ymd_opt(2015, 2, 29).is_none());

  assert!(from_ymd_opt(-4, 2, 29).is_some()); // 5 BCE is a leap year

  assert!(from_ymd_opt(400000, 1, 1).is_none());

  assert!(from_ymd_opt(-400000, 1, 1).is_none());

  ```

- <span id="naivedate-from-yo"></span>`const fn from_yo(year: i32, ordinal: u32) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)

  (year and day of the year).

  

  # Panics

  

  Panics if the specified ordinal day does not exist, on invalid values for `ordinal`, or if

  `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-yo-opt"></span>`const fn from_yo_opt(year: i32, ordinal: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ordinal date](#ordinal-date)

  (year and day of the year).

  

  # Errors

  

  Returns `None` if:

  - The specified ordinal day does not exist (for example 2023-366).

  - The value for `ordinal` is invalid (for example: `0`, `400`).

  - `year` is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_yo_opt = NaiveDate::from_yo_opt;

  

  assert!(from_yo_opt(2015, 100).is_some());

  assert!(from_yo_opt(2015, 0).is_none());

  assert!(from_yo_opt(2015, 365).is_some());

  assert!(from_yo_opt(2015, 366).is_none());

  assert!(from_yo_opt(-4, 366).is_some()); // 5 BCE is a leap year

  assert!(from_yo_opt(400000, 1).is_none());

  assert!(from_yo_opt(-400000, 1).is_none());

  ```

- <span id="naivedate-from-isoywd"></span>`const fn from_isoywd(year: i32, week: u32, weekday: Weekday) -> NaiveDate` — [`Weekday`](weekday/index.md#weekday), [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ISO week date](#week-date)

  (year, week number and day of the week).

  The resulting `NaiveDate` may have a different year from the input year.

  

  # Panics

  

  Panics if the specified week does not exist in that year, on invalid values for `week`, or

  if the resulting date is out of range for `NaiveDate`.

- <span id="naivedate-from-isoywd-opt"></span>`const fn from_isoywd_opt(year: i32, week: u32, weekday: Weekday) -> Option<NaiveDate>` — [`Weekday`](weekday/index.md#weekday), [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from the [ISO week date](#week-date)

  (year, week number and day of the week).

  The resulting `NaiveDate` may have a different year from the input year.

  

  # Errors

  

  Returns `None` if:

  - The specified week does not exist in that year (for example 2023 week 53).

  - The value for `week` is invalid (for example: `0`, `60`).

  - If the resulting date is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let from_isoywd_opt = NaiveDate::from_isoywd_opt;

  

  assert_eq!(from_isoywd_opt(2015, 0, Weekday::Sun), None);

  assert_eq!(from_isoywd_opt(2015, 10, Weekday::Sun), Some(from_ymd(2015, 3, 8)));

  assert_eq!(from_isoywd_opt(2015, 30, Weekday::Mon), Some(from_ymd(2015, 7, 20)));

  assert_eq!(from_isoywd_opt(2015, 60, Weekday::Mon), None);

  

  assert_eq!(from_isoywd_opt(400000, 10, Weekday::Fri), None);

  assert_eq!(from_isoywd_opt(-400000, 10, Weekday::Sat), None);

  ```

  

  The year number of ISO week date may differ from that of the calendar date.

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let from_isoywd_opt = NaiveDate::from_isoywd_opt;

  //           Mo Tu We Th Fr Sa Su

  // 2014-W52  22 23 24 25 26 27 28    has 4+ days of new year,

  // 2015-W01  29 30 31  1  2  3  4 <- so this is the first week

  assert_eq!(from_isoywd_opt(2014, 52, Weekday::Sun), Some(from_ymd(2014, 12, 28)));

  assert_eq!(from_isoywd_opt(2014, 53, Weekday::Mon), None);

  assert_eq!(from_isoywd_opt(2015, 1, Weekday::Mon), Some(from_ymd(2014, 12, 29)));

  

  // 2015-W52  21 22 23 24 25 26 27    has 4+ days of old year,

  // 2015-W53  28 29 30 31  1  2  3 <- so this is the last week

  // 2016-W01   4  5  6  7  8  9 10

  assert_eq!(from_isoywd_opt(2015, 52, Weekday::Sun), Some(from_ymd(2015, 12, 27)));

  assert_eq!(from_isoywd_opt(2015, 53, Weekday::Sun), Some(from_ymd(2016, 1, 3)));

  assert_eq!(from_isoywd_opt(2015, 54, Weekday::Mon), None);

  assert_eq!(from_isoywd_opt(2016, 1, Weekday::Mon), Some(from_ymd(2016, 1, 4)));

  ```

- <span id="naivedate-from-num-days-from-ce"></span>`const fn from_num_days_from_ce(days: i32) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with

  January 1, 1 being day 1.

  

  # Panics

  

  Panics if the date is out of range.

- <span id="naivedate-from-num-days-from-ce-opt"></span>`const fn from_num_days_from_ce_opt(days: i32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with

  January 1, 1 being day 1.

  

  # Errors

  

  Returns `None` if the date is out of range.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ndays_opt = NaiveDate::from_num_days_from_ce_opt;

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  assert_eq!(from_ndays_opt(730_000), Some(from_ymd(1999, 9, 3)));

  assert_eq!(from_ndays_opt(1), Some(from_ymd(1, 1, 1)));

  assert_eq!(from_ndays_opt(0), Some(from_ymd(0, 12, 31)));

  assert_eq!(from_ndays_opt(-1), Some(from_ymd(0, 12, 30)));

  assert_eq!(from_ndays_opt(100_000_000), None);

  assert_eq!(from_ndays_opt(-100_000_000), None);

  ```

- <span id="naivedate-from-epoch-days"></span>`const fn from_epoch_days(days: i32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` from a day's number in the proleptic Gregorian calendar, with

  January 1, 1970 being day 0.

  

  # Errors

  

  Returns `None` if the date is out of range.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ndays_opt = NaiveDate::from_epoch_days;

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  assert_eq!(from_ndays_opt(-719_162), Some(from_ymd(1, 1, 1)));

  assert_eq!(from_ndays_opt(1), Some(from_ymd(1970, 1, 2)));

  assert_eq!(from_ndays_opt(0), Some(from_ymd(1970, 1, 1)));

  assert_eq!(from_ndays_opt(-1), Some(from_ymd(1969, 12, 31)));

  assert_eq!(from_ndays_opt(13036), Some(from_ymd(2005, 9, 10)));

  assert_eq!(from_ndays_opt(100_000_000), None);

  assert_eq!(from_ndays_opt(-100_000_000), None);

  ```

- <span id="naivedate-from-weekday-of-month"></span>`const fn from_weekday_of_month(year: i32, month: u32, weekday: Weekday, n: u8) -> NaiveDate` — [`Weekday`](weekday/index.md#weekday), [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week

  since the beginning of the given month. For instance, if you want the 2nd Friday of March

  2017, you would use `NaiveDate::from_weekday_of_month(2017, 3, Weekday::Fri, 2)`.

  

  `n` is 1-indexed.

  

  # Panics

  

  Panics if the specified day does not exist in that month, on invalid values for `month` or

  `n`, or if `year` is out of range for `NaiveDate`.

- <span id="naivedate-from-weekday-of-month-opt"></span>`const fn from_weekday_of_month_opt(year: i32, month: u32, weekday: Weekday, n: u8) -> Option<NaiveDate>` — [`Weekday`](weekday/index.md#weekday), [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` by counting the number of occurrences of a particular day-of-week

  since the beginning of the given month. For instance, if you want the 2nd Friday of March

  2017, you would use `NaiveDate::from_weekday_of_month(2017, 3, Weekday::Fri, 2)`.

  

  `n` is 1-indexed.

  

  # Errors

  

  Returns `None` if:

  - The specified day does not exist in that month (for example the 5th Monday of Apr. 2023).

  - The value for `month` or `n` is invalid.

  - `year` is out of range for `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  assert_eq!(

      NaiveDate::from_weekday_of_month_opt(2017, 3, Weekday::Fri, 2),

      NaiveDate::from_ymd_opt(2017, 3, 10)

  )

  ```

- <span id="naivedate-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDate>` — [`ParseResult`](format/index.md#parseresult), [`NaiveDate`](naive/date/index.md#naivedate)

  Parses a string with the specified format string and returns a new `NaiveDate`.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let parse_from_str = NaiveDate::parse_from_str;

  

  assert_eq!(

      parse_from_str("2015-09-05", "%Y-%m-%d"),

      Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap())

  );

  assert_eq!(

      parse_from_str("5sep2015", "%d%b%Y"),

      Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap())

  );

  ```

  

  Time and offset is ignored for the purpose of parsing.

  

  ```rust

  use chrono::NaiveDate;

  let parse_from_str = NaiveDate::parse_from_str;

  assert_eq!(

      parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),

      Ok(NaiveDate::from_ymd_opt(2014, 5, 17).unwrap())

  );

  ```

  

  Out-of-bound dates or insufficient fields are errors.

  

  ```rust

  use chrono::NaiveDate;

  let parse_from_str = NaiveDate::parse_from_str;

  assert!(parse_from_str("2015/9", "%Y/%m").is_err());

  assert!(parse_from_str("2015/9/31", "%Y/%m/%d").is_err());

  ```

  

  All parsed fields should be consistent to each other, otherwise it's an error.

  

  ```rust

  use chrono::NaiveDate;

  let parse_from_str = NaiveDate::parse_from_str;

  assert!(parse_from_str("Sat, 09 Aug 2013", "%a, %d %b %Y").is_err());

  ```

- <span id="naivedate-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDate, &'a str)>` — [`ParseResult`](format/index.md#parseresult), [`NaiveDate`](naive/date/index.md#naivedate)

  Parses a string from a user-specified format into a new `NaiveDate` value, and a slice with

  the remaining portion of the string.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  Similar to [`parse_from_str`](#method.parse_from_str).

  

  # Example

  

  ```rust

  use chrono::{NaiveDate};

  let (date, remainder) =

      NaiveDate::parse_and_remainder("2015-02-18 trailing text", "%Y-%m-%d").unwrap();

  assert_eq!(date, NaiveDate::from_ymd_opt(2015, 2, 18).unwrap());

  assert_eq!(remainder, " trailing text");

  ```

- <span id="naivedate-checked-add-months"></span>`const fn checked_add_months(self, months: Months) -> Option<Self>` — [`Months`](month/index.md#months)

  Add a duration in [`Months`](month/index.md) to the date

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Months};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_add_months(Months::new(6)),

      Some(NaiveDate::from_ymd_opt(2022, 8, 20).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_months(Months::new(2)),

      Some(NaiveDate::from_ymd_opt(2022, 9, 30).unwrap())

  );

  ```

- <span id="naivedate-checked-sub-months"></span>`const fn checked_sub_months(self, months: Months) -> Option<Self>` — [`Months`](month/index.md#months)

  Subtract a duration in [`Months`](month/index.md) from the date

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Months};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_months(Months::new(6)),

      Some(NaiveDate::from_ymd_opt(2021, 8, 20).unwrap())

  );

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),

      None

  );

  ```

- <span id="naivedate-diff-months"></span>`const fn diff_months(self, months: i32) -> Option<Self>`

- <span id="naivedate-checked-add-days"></span>`const fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](naive/index.md#days)

  Add a duration in [`Days`](naive/index.md) to the date

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Days};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_add_days(Days::new(9)),

      Some(NaiveDate::from_ymd_opt(2022, 3, 1).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_days(Days::new(2)),

      Some(NaiveDate::from_ymd_opt(2022, 8, 2).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 7, 31).unwrap().checked_add_days(Days::new(1000000000000)),

      None

  );

  ```

- <span id="naivedate-checked-sub-days"></span>`const fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](naive/index.md#days)

  Subtract a duration in [`Days`](naive/index.md) from the date

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Days};

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_days(Days::new(6)),

      Some(NaiveDate::from_ymd_opt(2022, 2, 14).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().checked_sub_days(Days::new(1000000000000)),

      None

  );

  ```

- <span id="naivedate-add-days"></span>`const fn add_days(self, days: i32) -> Option<Self>`

  Add a duration of `i32` days to the date.

- <span id="naivedate-and-time"></span>`const fn and_time(&self, time: NaiveTime) -> NaiveDateTime` — [`NaiveTime`](naive/time/index.md#naivetime), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date and given `NaiveTime`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

  

  let dt: NaiveDateTime = d.and_time(t);

  assert_eq!(dt.date(), d);

  assert_eq!(dt.time(), t);

  ```

- <span id="naivedate-and-hms"></span>`const fn and_hms(&self, hour: u32, min: u32, sec: u32) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute and second.

  

  No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;

  use `NaiveDate::and_hms_*` methods with a subsecond parameter instead.

  

  # Panics

  

  Panics on invalid hour, minute and/or second.

- <span id="naivedate-and-hms-opt"></span>`const fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute and second.

  

  No [leap second](./struct.NaiveTime.html#leap-second-handling) is allowed here;

  use `NaiveDate::and_hms_*_opt` methods with a subsecond parameter instead.

  

  # Errors

  

  Returns `None` on invalid hour, minute and/or second.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_opt(12, 34, 56).is_some());

  assert!(d.and_hms_opt(12, 34, 60).is_none()); // use `and_hms_milli_opt` instead

  assert!(d.and_hms_opt(12, 60, 56).is_none());

  assert!(d.and_hms_opt(24, 34, 56).is_none());

  ```

- <span id="naivedate-and-hms-milli"></span>`const fn and_hms_milli(&self, hour: u32, min: u32, sec: u32, milli: u32) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.

  

  The millisecond part is allowed to exceed 1,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="naivedate-and-hms-milli-opt"></span>`const fn and_hms_milli_opt(&self, hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and millisecond.

  

  The millisecond part is allowed to exceed 1,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or millisecond.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_milli_opt(12, 34, 56, 789).is_some());

  assert!(d.and_hms_milli_opt(12, 34, 59, 1_789).is_some()); // leap second

  assert!(d.and_hms_milli_opt(12, 34, 59, 2_789).is_none());

  assert!(d.and_hms_milli_opt(12, 34, 60, 789).is_none());

  assert!(d.and_hms_milli_opt(12, 60, 56, 789).is_none());

  assert!(d.and_hms_milli_opt(24, 34, 56, 789).is_none());

  ```

- <span id="naivedate-and-hms-micro"></span>`const fn and_hms_micro(&self, hour: u32, min: u32, sec: u32, micro: u32) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or microsecond.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Weekday};

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  

  let dt: NaiveDateTime = d.and_hms_micro_opt(12, 34, 56, 789_012).unwrap();

  assert_eq!(dt.year(), 2015);

  assert_eq!(dt.weekday(), Weekday::Wed);

  assert_eq!(dt.second(), 56);

  assert_eq!(dt.nanosecond(), 789_012_000);

  ```

- <span id="naivedate-and-hms-micro-opt"></span>`const fn and_hms_micro_opt(&self, hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or microsecond.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_micro_opt(12, 34, 56, 789_012).is_some());

  assert!(d.and_hms_micro_opt(12, 34, 59, 1_789_012).is_some()); // leap second

  assert!(d.and_hms_micro_opt(12, 34, 59, 2_789_012).is_none());

  assert!(d.and_hms_micro_opt(12, 34, 60, 789_012).is_none());

  assert!(d.and_hms_micro_opt(12, 60, 56, 789_012).is_none());

  assert!(d.and_hms_micro_opt(24, 34, 56, 789_012).is_none());

  ```

- <span id="naivedate-and-hms-nano"></span>`const fn and_hms_nano(&self, hour: u32, min: u32, sec: u32, nano: u32) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="naivedate-and-hms-nano-opt"></span>`const fn and_hms_nano_opt(&self, hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from the current date, hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a [leap second](

  ./struct.NaiveTime.html#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or nanosecond.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  assert!(d.and_hms_nano_opt(12, 34, 56, 789_012_345).is_some());

  assert!(d.and_hms_nano_opt(12, 34, 59, 1_789_012_345).is_some()); // leap second

  assert!(d.and_hms_nano_opt(12, 34, 59, 2_789_012_345).is_none());

  assert!(d.and_hms_nano_opt(12, 34, 60, 789_012_345).is_none());

  assert!(d.and_hms_nano_opt(12, 60, 56, 789_012_345).is_none());

  assert!(d.and_hms_nano_opt(24, 34, 56, 789_012_345).is_none());

  ```

- <span id="naivedate-mdf"></span>`const fn mdf(&self) -> Mdf` — [`Mdf`](naive/internals/index.md#mdf)

  Returns the packed month-day-flags.

- <span id="naivedate-with-mdf"></span>`const fn with_mdf(&self, mdf: Mdf) -> Option<NaiveDate>` — [`Mdf`](naive/internals/index.md#mdf), [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the packed month-day-flags changed.

  

  Returns `None` when the resulting `NaiveDate` would be invalid.

- <span id="naivedate-succ"></span>`const fn succ(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` for the next calendar date.

  

  # Panics

  

  Panics when `self` is the last representable date.

- <span id="naivedate-succ-opt"></span>`const fn succ_opt(&self) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` for the next calendar date.

  

  # Errors

  

  Returns `None` when `self` is the last representable date.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 6, 3).unwrap().succ_opt(),

      Some(NaiveDate::from_ymd_opt(2015, 6, 4).unwrap())

  );

  assert_eq!(NaiveDate::MAX.succ_opt(), None);

  ```

- <span id="naivedate-pred"></span>`const fn pred(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` for the previous calendar date.

  

  # Panics

  

  Panics when `self` is the first representable date.

- <span id="naivedate-pred-opt"></span>`const fn pred_opt(&self) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` for the previous calendar date.

  

  # Errors

  

  Returns `None` when `self` is the first representable date.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 6, 3).unwrap().pred_opt(),

      Some(NaiveDate::from_ymd_opt(2015, 6, 2).unwrap())

  );

  assert_eq!(NaiveDate::MIN.pred_opt(), None);

  ```

- <span id="naivedate-checked-add-signed"></span>`const fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDate>` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveDate`](naive/date/index.md#naivedate)

  Adds the number of whole days in the given `TimeDelta` to the current date.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

  assert_eq!(

      d.checked_add_signed(TimeDelta::try_days(40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 10, 15).unwrap())

  );

  assert_eq!(

      d.checked_add_signed(TimeDelta::try_days(-40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 7, 27).unwrap())

  );

  assert_eq!(d.checked_add_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  assert_eq!(d.checked_add_signed(TimeDelta::try_days(-1_000_000_000).unwrap()), None);

  assert_eq!(NaiveDate::MAX.checked_add_signed(TimeDelta::try_days(1).unwrap()), None);

  ```

- <span id="naivedate-checked-sub-signed"></span>`const fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDate>` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveDate`](naive/date/index.md#naivedate)

  Subtracts the number of whole days in the given `TimeDelta` from the current date.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let d = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

  assert_eq!(

      d.checked_sub_signed(TimeDelta::try_days(40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 7, 27).unwrap())

  );

  assert_eq!(

      d.checked_sub_signed(TimeDelta::try_days(-40).unwrap()),

      Some(NaiveDate::from_ymd_opt(2015, 10, 15).unwrap())

  );

  assert_eq!(d.checked_sub_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  assert_eq!(d.checked_sub_signed(TimeDelta::try_days(-1_000_000_000).unwrap()), None);

  assert_eq!(NaiveDate::MIN.checked_sub_signed(TimeDelta::try_days(1).unwrap()), None);

  ```

- <span id="naivedate-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: Self) -> TimeDelta` — [`TimeDelta`](time_delta/index.md#timedelta)

  Subtracts another `NaiveDate` from the current date.

  Returns a `TimeDelta` of integral numbers.

  

  This does not overflow or underflow at all,

  as all possible output fits in the range of `TimeDelta`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let since = NaiveDate::signed_duration_since;

  

  assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2014, 1, 1)), TimeDelta::zero());

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2013, 12, 31)),

      TimeDelta::try_days(1).unwrap()

  );

  assert_eq!(since(from_ymd(2014, 1, 1), from_ymd(2014, 1, 2)), TimeDelta::try_days(-1).unwrap());

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2013, 9, 23)),

      TimeDelta::try_days(100).unwrap()

  );

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2013, 1, 1)),

      TimeDelta::try_days(365).unwrap()

  );

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(2010, 1, 1)),

      TimeDelta::try_days(365 * 4 + 1).unwrap()

  );

  assert_eq!(

      since(from_ymd(2014, 1, 1), from_ymd(1614, 1, 1)),

      TimeDelta::try_days(365 * 400 + 97).unwrap()

  );

  ```

- <span id="naivedate-abs-diff"></span>`const fn abs_diff(self, rhs: Self) -> Days` — [`Days`](naive/index.md#days)

  Returns the absolute difference between two `NaiveDate`s measured as the number of days.

  

  This is always an integer, non-negative number, similar to `abs_diff` in `std`.

  

  # Example

  

  ```rust

  use chrono::{Days, NaiveDate};

  

  let date1: NaiveDate = "2020-01-01".parse().unwrap();

  let date2: NaiveDate = "2020-01-31".parse().unwrap();

  assert_eq!(date2.abs_diff(date1), Days::new(30));

  assert_eq!(date1.abs_diff(date2), Days::new(30));

  ```

- <span id="naivedate-years-since"></span>`const fn years_since(&self, base: Self) -> Option<u32>`

  Returns the number of whole years from the given `base` until `self`.

  

  # Errors

  

  Returns `None` if `base > self`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate};

  

  let base: NaiveDate = "2025-01-01".parse().unwrap();

  let date: NaiveDate = "2030-01-01".parse().unwrap();

  

  assert_eq!(date.years_since(base), Some(5))

  ```

- <span id="naivedate-iter-days"></span>`const fn iter_days(&self) -> NaiveDateDaysIterator` — [`NaiveDateDaysIterator`](naive/date/index.md#naivedatedaysiterator)

  Returns an iterator that steps by days across all representable dates.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let expected = [

      NaiveDate::from_ymd_opt(2016, 2, 27).unwrap(),

      NaiveDate::from_ymd_opt(2016, 2, 28).unwrap(),

      NaiveDate::from_ymd_opt(2016, 2, 29).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 1).unwrap(),

  ];

  

  let mut count = 0;

  for (idx, d) in NaiveDate::from_ymd_opt(2016, 2, 27).unwrap().iter_days().take(4).enumerate() {

      assert_eq!(d, expected[idx]);

      count += 1;

  }

  assert_eq!(count, 4);

  

  for d in NaiveDate::from_ymd_opt(2016, 3, 1).unwrap().iter_days().rev().take(4) {

      count -= 1;

      assert_eq!(d, expected[count]);

  }

  ```

- <span id="naivedate-iter-weeks"></span>`const fn iter_weeks(&self) -> NaiveDateWeeksIterator` — [`NaiveDateWeeksIterator`](naive/date/index.md#naivedateweeksiterator)

  Returns an iterator that steps by weeks across all representable dates.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let expected = [

      NaiveDate::from_ymd_opt(2016, 2, 27).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 5).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 12).unwrap(),

      NaiveDate::from_ymd_opt(2016, 3, 19).unwrap(),

  ];

  

  let mut count = 0;

  for (idx, d) in NaiveDate::from_ymd_opt(2016, 2, 27).unwrap().iter_weeks().take(4).enumerate() {

      assert_eq!(d, expected[idx]);

      count += 1;

  }

  assert_eq!(count, 4);

  

  for d in NaiveDate::from_ymd_opt(2016, 3, 19).unwrap().iter_weeks().rev().take(4) {

      count -= 1;

      assert_eq!(d, expected[count]);

  }

  ```

- <span id="naivedate-week"></span>`const fn week(&self, start: Weekday) -> NaiveWeek` — [`Weekday`](weekday/index.md#weekday), [`NaiveWeek`](naive/index.md#naiveweek)

  Returns the [`NaiveWeek`](naive/index.md) that the date belongs to, starting with the [`Weekday`](weekday/index.md)

  specified.

- <span id="naivedate-leap-year"></span>`const fn leap_year(&self) -> bool`

  Returns `true` if this is a leap year.

  

  ```rust

  use chrono::NaiveDate;

  assert_eq!(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap().leap_year(), true);

  assert_eq!(NaiveDate::from_ymd_opt(2001, 1, 1).unwrap().leap_year(), false);

  assert_eq!(NaiveDate::from_ymd_opt(2002, 1, 1).unwrap().leap_year(), false);

  assert_eq!(NaiveDate::from_ymd_opt(2003, 1, 1).unwrap().leap_year(), false);

  assert_eq!(NaiveDate::from_ymd_opt(2004, 1, 1).unwrap().leap_year(), true);

  assert_eq!(NaiveDate::from_ymd_opt(2100, 1, 1).unwrap().leap_year(), false);

  ```

- <span id="naivedate-year"></span>`const fn year(&self) -> i32`

- <span id="naivedate-ordinal"></span>`const fn ordinal(&self) -> u32`

  Returns the day of year starting from 1.

- <span id="naivedate-month"></span>`const fn month(&self) -> u32`

- <span id="naivedate-day"></span>`const fn day(&self) -> u32`

- <span id="naivedate-weekday"></span>`const fn weekday(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

  Returns the day of week.

- <span id="naivedate-year-flags"></span>`const fn year_flags(&self) -> YearFlags` — [`YearFlags`](naive/internals/index.md#yearflags)

- <span id="naivedate-num-days-from-ce"></span>`const fn num_days_from_ce(&self) -> i32`

  Counts the days in the proleptic Gregorian calendar, with January 1, Year 1 (CE) as day 1.

- <span id="naivedate-to-epoch-days"></span>`const fn to_epoch_days(&self) -> i32`

  Counts the days in the proleptic Gregorian calendar, with January 1, Year 1970 as day 0.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  assert_eq!(from_ymd(1, 1, 1).to_epoch_days(), -719162);

  assert_eq!(from_ymd(1970, 1, 1).to_epoch_days(), 0);

  assert_eq!(from_ymd(2005, 9, 10).to_epoch_days(), 13036);

  ```

- <span id="naivedate-from-yof"></span>`const fn from_yof(yof: i32) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Create a new `NaiveDate` from a raw year-ordinal-flags `i32`.

  

  In a valid value an ordinal is never `0`, and neither are the year flags. This method

  doesn't do any validation in release builds.

- <span id="naivedate-yof"></span>`const fn yof(&self) -> i32`

  Get the raw year-ordinal-flags `i32`.

- <span id="naivedate-const-min"></span>`const MIN: NaiveDate`

- <span id="naivedate-const-max"></span>`const MAX: NaiveDate`

- <span id="naivedate-const-before-min"></span>`const BEFORE_MIN: NaiveDate`

- <span id="naivedate-const-after-max"></span>`const AFTER_MAX: NaiveDate`

#### Trait Implementations

##### `impl Add for NaiveDate`

- <span id="naivedate-add-type-output"></span>`type Output = NaiveDate`

- <span id="naivedate-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveDate` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveDate`](naive/date/index.md#naivedate)

##### `impl AddAssign for NaiveDate`

- <span id="naivedate-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Clone for NaiveDate`

- <span id="naivedate-clone"></span>`fn clone(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

##### `impl Copy for NaiveDate`

##### `impl Datelike for NaiveDate`

- <span id="naivedate-datelike-year"></span>`fn year(&self) -> i32`

  Returns the year number in the [calendar date](#calendar-date).

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().year(), 2015);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().year(), -308); // 309 BCE

  ```

- <span id="naivedate-datelike-month"></span>`fn month(&self) -> u32`

  Returns the month number starting from 1.

  

  The return value ranges from 1 to 12.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().month(), 9);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().month(), 3);

  ```

- <span id="naivedate-datelike-month0"></span>`fn month0(&self) -> u32`

  Returns the month number starting from 0.

  

  The return value ranges from 0 to 11.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().month0(), 8);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().month0(), 2);

  ```

- <span id="naivedate-datelike-day"></span>`fn day(&self) -> u32`

  Returns the day of month starting from 1.

  

  The return value ranges from 1 to 31. (The last day of month differs by months.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().day(), 8);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().day(), 14);

  ```

  

  Combined with [`NaiveDate::pred_opt`](#method.pred_opt),

  one can determine the number of days in a particular month.

  (Note that this panics when `year` is out of range.)

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  fn ndays_in_month(year: i32, month: u32) -> u32 {

      // the first day of the next month...

      let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };

      let d = NaiveDate::from_ymd_opt(y, m, 1).unwrap();

  

      // ...is preceded by the last day of the original month

      d.pred_opt().unwrap().day()

  }

  

  assert_eq!(ndays_in_month(2015, 8), 31);

  assert_eq!(ndays_in_month(2015, 9), 30);

  assert_eq!(ndays_in_month(2015, 12), 31);

  assert_eq!(ndays_in_month(2016, 2), 29);

  assert_eq!(ndays_in_month(2017, 2), 28);

  ```

- <span id="naivedate-datelike-day0"></span>`fn day0(&self) -> u32`

  Returns the day of month starting from 0.

  

  The return value ranges from 0 to 30. (The last day of month differs by months.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().day0(), 7);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().day0(), 13);

  ```

- <span id="naivedate-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

  Returns the day of year starting from 1.

  

  The return value ranges from 1 to 366. (The last day of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().ordinal(), 251);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().ordinal(), 74);

  ```

  

  Combined with [`NaiveDate::pred_opt`](#method.pred_opt),

  one can determine the number of days in a particular year.

  (Note that this panics when `year` is out of range.)

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  fn ndays_in_year(year: i32) -> u32 {

      // the first day of the next year...

      let d = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();

  

      // ...is preceded by the last day of the original year

      d.pred_opt().unwrap().ordinal()

  }

  

  assert_eq!(ndays_in_year(2015), 365);

  assert_eq!(ndays_in_year(2016), 366);

  assert_eq!(ndays_in_year(2017), 365);

  assert_eq!(ndays_in_year(2000), 366);

  assert_eq!(ndays_in_year(2100), 365);

  ```

- <span id="naivedate-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

  Returns the day of year starting from 0.

  

  The return value ranges from 0 to 365. (The last day of year differs by years.)

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().ordinal0(), 250);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().ordinal0(), 73);

  ```

- <span id="naivedate-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

  Returns the day of week.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, Weekday};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().weekday(), Weekday::Tue);

  assert_eq!(NaiveDate::from_ymd_opt(-308, 3, 14).unwrap().weekday(), Weekday::Fri);

  ```

- <span id="naivedate-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

- <span id="naivedate-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the year number changed, while keeping the same month and day.

  

  This method assumes you want to work on the date as a year-month-day value. Don't use it if

  you want the ordinal to stay the same after changing the year, of if you want the week and

  weekday values to stay the same.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (February 29 in a non-leap year).

  - The year is out of range for a `NaiveDate`.

  

  # Examples

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_year(2016),

      Some(NaiveDate::from_ymd_opt(2016, 9, 8).unwrap())

  );

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_year(-308),

      Some(NaiveDate::from_ymd_opt(-308, 9, 8).unwrap())

  );

  ```

  

  A leap day (February 29) is a case where this method can return `None`.

  

  ```rust

  use chrono::{NaiveDate, Datelike};

  assert!(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().with_year(2015).is_none());

  assert!(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().with_year(2020).is_some());

  ```

  

  Don't use `with_year` if you want the ordinal date to stay the same:

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  assert_ne!(

      NaiveDate::from_yo_opt(2020, 100).unwrap().with_year(2023).unwrap(),

      NaiveDate::from_yo_opt(2023, 100).unwrap() // result is 2023-101

  );

  ```

- <span id="naivedate-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the month number (starting from 1) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month(4)` when day of the month is 31).

  - The value for `month` is invalid.

  

  # Examples

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month(10),

      Some(NaiveDate::from_ymd_opt(2015, 10, 8).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month(13), None); // No month 13

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().with_month(2), None); // No Feb 30

  ```

  

  Don't combine multiple `Datelike::with_*` methods. The intermediate value may not exist.

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  fn with_year_month(date: NaiveDate, year: i32, month: u32) -> Option<NaiveDate> {

      date.with_year(year)?.with_month(month)

  }

  let d = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();

  assert!(with_year_month(d, 2019, 1).is_none()); // fails because of invalid intermediate value

  

  // Correct version:

  fn with_year_month_fixed(date: NaiveDate, year: i32, month: u32) -> Option<NaiveDate> {

      NaiveDate::from_ymd_opt(year, month, date.day())

  }

  let d = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();

  assert_eq!(with_year_month_fixed(d, 2019, 1), NaiveDate::from_ymd_opt(2019, 1, 29));

  ```

- <span id="naivedate-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the month number (starting from 0) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month0(3)` when day of the month is 31).

  - The value for `month0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month0(9),

      Some(NaiveDate::from_ymd_opt(2015, 10, 8).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_month0(12), None); // No month 12

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().with_month0(1), None); // No Feb 30

  ```

- <span id="naivedate-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of month (starting from 1) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(31)` in April).

  - The value for `day` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day(30),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day(31), None);

  // no September 31

  ```

- <span id="naivedate-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of month (starting from 0) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(30)` in April).

  - The value for `day0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day0(29),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap())

  );

  assert_eq!(NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().with_day0(30), None);

  // no September 31

  ```

- <span id="naivedate-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of year (starting from 1) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal(366)` in a non-leap year).

  - The value for `ordinal` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Datelike};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal(60),

             Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal(366),

             None); // 2015 had only 365 days

  

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal(60),

             Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal(366),

             Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap()));

  ```

- <span id="naivedate-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Makes a new `NaiveDate` with the day of year (starting from 0) changed.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal0(365)` in a non-leap year).

  - The value for `ordinal0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Datelike};

  

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal0(59),

             Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().with_ordinal0(365),

             None); // 2015 had only 365 days

  

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal0(59),

             Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap()));

  assert_eq!(NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().with_ordinal0(365),

             Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap()));

  ```

##### `impl Debug for NaiveDate`

- <span id="naivedate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NaiveDate`

- <span id="naivedate-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::NaiveDate`

- <span id="supernaivedate-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for NaiveDate`

##### `impl Display for NaiveDate`

- <span id="naivedate-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NaiveDate`

##### `impl FromStr for NaiveDate`

- <span id="naivedate-fromstr-type-err"></span>`type Err = ParseError`

- <span id="naivedate-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveDate>` — [`ParseResult`](format/index.md#parseresult), [`NaiveDate`](naive/date/index.md#naivedate)

##### `impl Hash for NaiveDate`

- <span id="naivedate-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveDate`

- <span id="naivedate-ord-cmp"></span>`fn cmp(&self, other: &NaiveDate) -> cmp::Ordering` — [`NaiveDate`](naive/date/index.md#naivedate)

##### `impl PartialEq for NaiveDate`

- <span id="naivedate-partialeq-eq"></span>`fn eq(&self, other: &NaiveDate) -> bool` — [`NaiveDate`](naive/date/index.md#naivedate)

##### `impl PartialOrd for NaiveDate`

- <span id="naivedate-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDate) -> option::Option<cmp::Ordering>` — [`NaiveDate`](naive/date/index.md#naivedate)

##### `impl Serialize for super::NaiveDate`

- <span id="supernaivedate-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveDate`

##### `impl Sub for NaiveDate`

- <span id="naivedate-sub-type-output"></span>`type Output = NaiveDate`

- <span id="naivedate-sub"></span>`fn sub(self, months: Months) -> <Self as >::Output` — [`Months`](month/index.md#months)

##### `impl SubAssign for NaiveDate`

- <span id="naivedate-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl ToString for NaiveDate`

- <span id="naivedate-tostring-to-string"></span>`fn to_string(&self) -> String`

### `NaiveDateTime`

```rust
struct NaiveDateTime {
    date: crate::naive::NaiveDate,
    time: crate::naive::NaiveTime,
}
```

ISO 8601 combined date and time without timezone.

# Example

`NaiveDateTime` is commonly created from [`NaiveDate`](naive/date/index.md).

```rust
use chrono::{NaiveDate, NaiveDateTime};

let dt: NaiveDateTime =
    NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
let _ = dt;
```

You can use typical [date-like](Datelike) and [time-like](Timelike) methods,
provided that relevant traits are in the scope.

```rust
use chrono::{NaiveDate, NaiveDateTime};
let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();
use chrono::{Datelike, Timelike, Weekday};

assert_eq!(dt.weekday(), Weekday::Fri);
assert_eq!(dt.num_seconds_from_midnight(), 33011);
```

#### Implementations

- <span id="naivedatetime-new"></span>`const fn new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime` — [`NaiveDate`](naive/date/index.md#naivedate), [`NaiveTime`](naive/time/index.md#naivetime), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` from date and time components.

  Equivalent to [`date.and_time(time)`](./struct.NaiveDate.html#method.and_time)

  and many other helper constructors on `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

  

  let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();

  let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

  

  let dt = NaiveDateTime::new(d, t);

  assert_eq!(dt.date(), d);

  assert_eq!(dt.time(), t);

  ```

- <span id="naivedatetime-from-timestamp"></span>`const fn from_timestamp(secs: i64, nsecs: u32) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` corresponding to a UTC date and time,

  from the number of non-leap seconds

  since the midnight UTC on January 1, 1970 (aka "UNIX timestamp")

  and the number of nanoseconds since the last whole non-leap second.

  

  For a non-naive version of this function see `TimeZone::timestamp`.

  

  The nanosecond part can exceed 1,000,000,000 in order to represent a

  [leap second](NaiveTime#leap-second-handling), but only when `secs % 60 == 59`.

  (The true "UNIX timestamp" cannot represent a leap second unambiguously.)

  

  # Panics

  

  Panics if the number of seconds would be out of range for a `NaiveDateTime` (more than

  ca. 262,000 years away from common era), and panics on an invalid nanosecond (2 seconds or

  more).

- <span id="naivedatetime-from-timestamp-millis"></span>`const fn from_timestamp_millis(millis: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Creates a new [NaiveDateTime] from milliseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of milliseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-micros"></span>`const fn from_timestamp_micros(micros: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Creates a new [NaiveDateTime] from microseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of microseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-nanos"></span>`const fn from_timestamp_nanos(nanos: i64) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Creates a new [NaiveDateTime] from nanoseconds since the UNIX epoch.

  

  The UNIX epoch starts on midnight, January 1, 1970, UTC.

  

  # Errors

  

  Returns `None` if the number of nanoseconds would be out of range for a `NaiveDateTime`

  (more than ca. 262,000 years away from common era)

- <span id="naivedatetime-from-timestamp-opt"></span>`const fn from_timestamp_opt(secs: i64, nsecs: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` corresponding to a UTC date and time,

  from the number of non-leap seconds

  since the midnight UTC on January 1, 1970 (aka "UNIX timestamp")

  and the number of nanoseconds since the last whole non-leap second.

  

  The nanosecond part can exceed 1,000,000,000 in order to represent a

  [leap second](NaiveTime#leap-second-handling), but only when `secs % 60 == 59`.

  (The true "UNIX timestamp" cannot represent a leap second unambiguously.)

  

  # Errors

  

  Returns `None` if the number of seconds would be out of range for a `NaiveDateTime` (more

  than ca. 262,000 years away from common era), and panics on an invalid nanosecond

  (2 seconds or more).

- <span id="naivedatetime-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveDateTime>` — [`ParseResult`](format/index.md#parseresult), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

   Parses a string with the specified format string and returns a new `NaiveDateTime`.

   See the [`format::strftime` module](crate::format::strftime)

   on the supported escape sequences.

  

   # Example

  

   ```rust

   use chrono::{NaiveDate, NaiveDateTime};

  

   let parse_from_str = NaiveDateTime::parse_from_str;

  

   assert_eq!(

       parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S"),

       Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().and_hms_opt(23, 56, 4).unwrap())

   );

   assert_eq!(

       parse_from_str("5sep2015pm012345.6789", "%d%b%Y%p%I%M%S%.f"),

       Ok(NaiveDate::from_ymd_opt(2015, 9, 5)

           .unwrap()

           .and_hms_micro_opt(13, 23, 45, 678_900)

           .unwrap())

   );

   ```

  

   Offset is ignored for the purpose of parsing.

  

   ```rust

   use chrono::{NaiveDateTime, NaiveDate};

   let parse_from_str = NaiveDateTime::parse_from_str;

   assert_eq!(

       parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),

       Ok(NaiveDate::from_ymd_opt(2014, 5, 17).unwrap().and_hms_opt(12, 34, 56).unwrap())

   );

   ```

  

   [Leap seconds](./struct.NaiveTime.html#leap-second-handling) are correctly handled by

   treating any time of the form `hh:mm:60` as a leap second.

   (This equally applies to the formatting, so the round trip is possible.)

  

   ```rust

   use chrono::{NaiveDateTime, NaiveDate};

   let parse_from_str = NaiveDateTime::parse_from_str;

   assert_eq!(

       parse_from_str("2015-07-01 08:59:60.123", "%Y-%m-%d %H:%M:%S%.f"),

       Ok(NaiveDate::from_ymd_opt(2015, 7, 1)

           .unwrap()

           .and_hms_milli_opt(8, 59, 59, 1_123)

           .unwrap())

   );

   ```

  

   Missing seconds are assumed to be zero,

   but out-of-bound times or insufficient fields are errors otherwise.

  

   ```rust

   use chrono::{NaiveDateTime, NaiveDate};

   let parse_from_str = NaiveDateTime::parse_from_str;

   assert_eq!(

       parse_from_str("94/9/4 7:15", "%y/%m/%d %H:%M"),

       Ok(NaiveDate::from_ymd_opt(1994, 9, 4).unwrap().and_hms_opt(7, 15, 0).unwrap())

   );

  

   assert!(parse_from_str("04m33s", "%Mm%Ss").is_err());

   assert!(parse_from_str("94/9/4 12", "%y/%m/%d %H").is_err());

   assert!(parse_from_str("94/9/4 17:60", "%y/%m/%d %H:%M").is_err());

   assert!(parse_from_str("94/9/4 24:00:00", "%y/%m/%d %H:%M:%S").is_err());

   ```

  

   All parsed fields should be consistent to each other, otherwise it's an error.

  

   ```rust

   use chrono::NaiveDateTime;

   let parse_from_str = NaiveDateTime::parse_from_str;

   let fmt = "%Y-%m-%d %H:%M:%S = UNIX timestamp %s";

   assert!(parse_from_str("2001-09-09 01:46:39 = UNIX timestamp 999999999", fmt).is_ok());

   assert!(parse_from_str("1970-01-01 00:00:00 = UNIX timestamp 1", fmt).is_err());

   ```

  

   Years before 1 BCE or after 9999 CE, require an initial sign

  

  ```rust

   use chrono::NaiveDateTime;

   let parse_from_str = NaiveDateTime::parse_from_str;

   let fmt = "%Y-%m-%d %H:%M:%S";

   assert!(parse_from_str("10000-09-09 01:46:39", fmt).is_err());

   assert!(parse_from_str("+10000-09-09 01:46:39", fmt).is_ok());

   ```

- <span id="naivedatetime-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveDateTime, &'a str)>` — [`ParseResult`](format/index.md#parseresult), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Parses a string with the specified format string and returns a new `NaiveDateTime`, and a

  slice with the remaining portion of the string.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  Similar to [`parse_from_str`](#method.parse_from_str).

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime};

  let (datetime, remainder) = NaiveDateTime::parse_and_remainder(

      "2015-02-18 23:16:09 trailing text",

      "%Y-%m-%d %H:%M:%S",

  )

  .unwrap();

  assert_eq!(

      datetime,

      NaiveDate::from_ymd_opt(2015, 2, 18).unwrap().and_hms_opt(23, 16, 9).unwrap()

  );

  assert_eq!(remainder, " trailing text");

  ```

- <span id="naivedatetime-date"></span>`const fn date(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Retrieves a date component.

  

  # Example

  

  ```rust

  use chrono::NaiveDate;

  

  let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();

  assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2016, 7, 8).unwrap());

  ```

- <span id="naivedatetime-time"></span>`const fn time(&self) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

  Retrieves a time component.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveTime};

  

  let dt = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();

  assert_eq!(dt.time(), NaiveTime::from_hms_opt(9, 10, 11).unwrap());

  ```

- <span id="naivedatetime-timestamp"></span>`const fn timestamp(&self) -> i64`

  Returns the number of non-leap seconds since the midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

- <span id="naivedatetime-timestamp-millis"></span>`const fn timestamp_millis(&self) -> i64`

  Returns the number of non-leap *milliseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

- <span id="naivedatetime-timestamp-micros"></span>`const fn timestamp_micros(&self) -> i64`

  Returns the number of non-leap *microseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

- <span id="naivedatetime-timestamp-nanos"></span>`const fn timestamp_nanos(&self) -> i64`

  Returns the number of non-leap *nanoseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

  

  # Panics

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function panics on

  an out of range `NaiveDateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

- <span id="naivedatetime-timestamp-nanos-opt"></span>`const fn timestamp_nanos_opt(&self) -> Option<i64>`

  Returns the number of non-leap *nanoseconds* since midnight on January 1, 1970.

  

  Note that this does *not* account for the timezone!

  The true "UNIX timestamp" would count seconds since the midnight *UTC* on the epoch.

  

  # Errors

  

  An `i64` with nanosecond precision can span a range of ~584 years. This function returns

  `None` on an out of range `NaiveDateTime`.

  

  The dates that can be represented as nanoseconds are between 1677-09-21T00:12:43.145224192

  and 2262-04-11T23:47:16.854775807.

- <span id="naivedatetime-timestamp-subsec-millis"></span>`const fn timestamp_subsec_millis(&self) -> u32`

  Returns the number of milliseconds since the last whole non-leap second.

  

  The return value ranges from 0 to 999,

  or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999.

- <span id="naivedatetime-timestamp-subsec-micros"></span>`const fn timestamp_subsec_micros(&self) -> u32`

  Returns the number of microseconds since the last whole non-leap second.

  

  The return value ranges from 0 to 999,999,

  or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999,999.

- <span id="naivedatetime-timestamp-subsec-nanos"></span>`const fn timestamp_subsec_nanos(&self) -> u32`

  Returns the number of nanoseconds since the last whole non-leap second.

  

  The return value ranges from 0 to 999,999,999,

  or for [leap seconds](./struct.NaiveTime.html#leap-second-handling), to 1,999,999,999.

- <span id="naivedatetime-checked-add-signed"></span>`const fn checked_add_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime>` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Adds given `TimeDelta` to the current date and time.

  

  As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),

  the addition assumes that **there is no leap second ever**,

  except when the `NaiveDateTime` itself represents a leap second

  in which case the assumption becomes that **there is exactly a single leap second ever**.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  let d = from_ymd(2016, 7, 8);

  let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::zero()), Some(hms(3, 5, 7)));

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(1).unwrap()),

      Some(hms(3, 5, 8))

  );

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(-1).unwrap()),

      Some(hms(3, 5, 6))

  );

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(3600 + 60).unwrap()),

      Some(hms(4, 6, 7))

  );

  assert_eq!(

      hms(3, 5, 7).checked_add_signed(TimeDelta::try_seconds(86_400).unwrap()),

      Some(from_ymd(2016, 7, 9).and_hms_opt(3, 5, 7).unwrap())

  );

  

  let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();

  assert_eq!(

      hmsm(3, 5, 7, 980).checked_add_signed(TimeDelta::try_milliseconds(450).unwrap()),

      Some(hmsm(3, 5, 8, 430))

  );

  ```

  

  Overflow returns `None`.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let hms = |h, m, s| NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_add_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  ```

  

  Leap seconds are handled,

  but the addition assumes that it is the only leap second happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();

  let leap = hmsm(3, 5, 59, 1_300);

  assert_eq!(leap.checked_add_signed(TimeDelta::zero()),

             Some(hmsm(3, 5, 59, 1_300)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_milliseconds(-500).unwrap()),

             Some(hmsm(3, 5, 59, 800)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_milliseconds(500).unwrap()),

             Some(hmsm(3, 5, 59, 1_800)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_milliseconds(800).unwrap()),

             Some(hmsm(3, 6, 0, 100)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_seconds(10).unwrap()),

             Some(hmsm(3, 6, 9, 300)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_seconds(-10).unwrap()),

             Some(hmsm(3, 5, 50, 300)));

  assert_eq!(leap.checked_add_signed(TimeDelta::try_days(1).unwrap()),

             Some(from_ymd(2016, 7, 9).and_hms_milli_opt(3, 5, 59, 300).unwrap()));

  ```

- <span id="naivedatetime-checked-add-months"></span>`const fn checked_add_months(self, rhs: Months) -> Option<NaiveDateTime>` — [`Months`](month/index.md#months), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Adds given `Months` to the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{Months, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_add_months(Months::new(1)),

      Some(NaiveDate::from_ymd_opt(2014, 2, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())

  );

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_add_months(Months::new(core::i32::MAX as u32 + 1)),

      None

  );

  ```

- <span id="naivedatetime-checked-add-offset"></span>`const fn checked_add_offset(self, rhs: FixedOffset) -> Option<NaiveDateTime>` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Adds given `FixedOffset` to the current datetime.

  Returns `None` if the result would be outside the valid range for [`NaiveDateTime`](naive/datetime/index.md).

  

  This method is similar to [`checked_add_signed`](#method.checked_add_offset), but preserves

  leap seconds.

- <span id="naivedatetime-checked-sub-offset"></span>`const fn checked_sub_offset(self, rhs: FixedOffset) -> Option<NaiveDateTime>` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Subtracts given `FixedOffset` from the current datetime.

  Returns `None` if the result would be outside the valid range for [`NaiveDateTime`](naive/datetime/index.md).

  

  This method is similar to [`checked_sub_signed`](#method.checked_sub_signed), but preserves

  leap seconds.

- <span id="naivedatetime-overflowing-add-offset"></span>`fn overflowing_add_offset(self, rhs: FixedOffset) -> NaiveDateTime` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Adds given `FixedOffset` to the current datetime.

  The resulting value may be outside the valid range of [`NaiveDateTime`](naive/datetime/index.md).

  

  This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`

  should not be exposed to library users.

- <span id="naivedatetime-overflowing-sub-offset"></span>`fn overflowing_sub_offset(self, rhs: FixedOffset) -> NaiveDateTime` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Subtracts given `FixedOffset` from the current datetime.

  The resulting value may be outside the valid range of [`NaiveDateTime`](naive/datetime/index.md).

  

  This can be useful for intermediate values, but the resulting out-of-range `NaiveDate`

  should not be exposed to library users.

- <span id="naivedatetime-checked-sub-signed"></span>`const fn checked_sub_signed(self, rhs: TimeDelta) -> Option<NaiveDateTime>` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Subtracts given `TimeDelta` from the current date and time.

  

  As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),

  the subtraction assumes that **there is no leap second ever**,

  except when the `NaiveDateTime` itself represents a leap second

  in which case the assumption becomes that **there is exactly a single leap second ever**.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  let d = from_ymd(2016, 7, 8);

  let hms = |h, m, s| d.and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::zero()), Some(hms(3, 5, 7)));

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(1).unwrap()),

      Some(hms(3, 5, 6))

  );

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(-1).unwrap()),

      Some(hms(3, 5, 8))

  );

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(3600 + 60).unwrap()),

      Some(hms(2, 4, 7))

  );

  assert_eq!(

      hms(3, 5, 7).checked_sub_signed(TimeDelta::try_seconds(86_400).unwrap()),

      Some(from_ymd(2016, 7, 7).and_hms_opt(3, 5, 7).unwrap())

  );

  

  let hmsm = |h, m, s, milli| d.and_hms_milli_opt(h, m, s, milli).unwrap();

  assert_eq!(

      hmsm(3, 5, 7, 450).checked_sub_signed(TimeDelta::try_milliseconds(670).unwrap()),

      Some(hmsm(3, 5, 6, 780))

  );

  ```

  

  Overflow returns `None`.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let hms = |h, m, s| NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(h, m, s).unwrap();

  assert_eq!(hms(3, 5, 7).checked_sub_signed(TimeDelta::try_days(1_000_000_000).unwrap()), None);

  ```

  

  Leap seconds are handled,

  but the subtraction assumes that it is the only leap second happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli_opt(h, m, s, milli).unwrap();

  let leap = hmsm(3, 5, 59, 1_300);

  assert_eq!(leap.checked_sub_signed(TimeDelta::zero()),

             Some(hmsm(3, 5, 59, 1_300)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_milliseconds(200).unwrap()),

             Some(hmsm(3, 5, 59, 1_100)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_milliseconds(500).unwrap()),

             Some(hmsm(3, 5, 59, 800)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_seconds(60).unwrap()),

             Some(hmsm(3, 5, 0, 300)));

  assert_eq!(leap.checked_sub_signed(TimeDelta::try_days(1).unwrap()),

             Some(from_ymd(2016, 7, 7).and_hms_milli_opt(3, 6, 0, 300).unwrap()));

  ```

- <span id="naivedatetime-checked-sub-months"></span>`const fn checked_sub_months(self, rhs: Months) -> Option<NaiveDateTime>` — [`Months`](month/index.md#months), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Subtracts given `Months` from the current date and time.

  

  Uses the last day of the month if the day does not exist in the resulting month.

  

  # Errors

  

  Returns `None` if the resulting date would be out of range.

  

  # Example

  

  ```rust

  use chrono::{Months, NaiveDate};

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_sub_months(Months::new(1)),

      Some(NaiveDate::from_ymd_opt(2013, 12, 1).unwrap().and_hms_opt(1, 0, 0).unwrap())

  );

  

  assert_eq!(

      NaiveDate::from_ymd_opt(2014, 1, 1)

          .unwrap()

          .and_hms_opt(1, 0, 0)

          .unwrap()

          .checked_sub_months(Months::new(core::i32::MAX as u32 + 1)),

      None

  );

  ```

- <span id="naivedatetime-checked-add-days"></span>`const fn checked_add_days(self, days: Days) -> Option<Self>` — [`Days`](naive/index.md#days)

  Add a duration in [`Days`](naive/index.md) to the date part of the `NaiveDateTime`

  

  Returns `None` if the resulting date would be out of range.

- <span id="naivedatetime-checked-sub-days"></span>`const fn checked_sub_days(self, days: Days) -> Option<Self>` — [`Days`](naive/index.md#days)

  Subtract a duration in [`Days`](naive/index.md) from the date part of the `NaiveDateTime`

  

  Returns `None` if the resulting date would be out of range.

- <span id="naivedatetime-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: NaiveDateTime) -> TimeDelta` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`TimeDelta`](time_delta/index.md#timedelta)

  Subtracts another `NaiveDateTime` from the current date and time.

  This does not overflow or underflow at all.

  

  As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),

  the subtraction assumes that **there is no leap second ever**,

  except when any of the `NaiveDateTime`s themselves represents a leap second

  in which case the assumption becomes that

  **there are exactly one (or two) leap second(s) ever**.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, TimeDelta};

  

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  

  let d = from_ymd(2016, 7, 8);

  assert_eq!(

      d.and_hms_opt(3, 5, 7).unwrap().signed_duration_since(d.and_hms_opt(2, 4, 6).unwrap()),

      TimeDelta::try_seconds(3600 + 60 + 1).unwrap()

  );

  

  // July 8 is 190th day in the year 2016

  let d0 = from_ymd(2016, 1, 1);

  assert_eq!(

      d.and_hms_milli_opt(0, 7, 6, 500)

          .unwrap()

          .signed_duration_since(d0.and_hms_opt(0, 0, 0).unwrap()),

      TimeDelta::try_seconds(189 * 86_400 + 7 * 60 + 6).unwrap()

          + TimeDelta::try_milliseconds(500).unwrap()

  );

  ```

  

  Leap seconds are handled, but the subtraction assumes that

  there were no other leap seconds happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveDate};

  let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

  let leap = from_ymd(2015, 6, 30).and_hms_milli_opt(23, 59, 59, 1_500).unwrap();

  assert_eq!(

      leap.signed_duration_since(from_ymd(2015, 6, 30).and_hms_opt(23, 0, 0).unwrap()),

      TimeDelta::try_seconds(3600).unwrap() + TimeDelta::try_milliseconds(500).unwrap()

  );

  assert_eq!(

      from_ymd(2015, 7, 1).and_hms_opt(1, 0, 0).unwrap().signed_duration_since(leap),

      TimeDelta::try_seconds(3600).unwrap() - TimeDelta::try_milliseconds(500).unwrap()

  );

  ```

- <span id="naivedatetime-and-local-timezone"></span>`fn and_local_timezone<Tz: TimeZone>(&self, tz: Tz) -> MappedLocalTime<DateTime<Tz>>` — [`MappedLocalTime`](offset/index.md#mappedlocaltime), [`DateTime`](datetime/index.md#datetime)

  Converts the `NaiveDateTime` into a timezone-aware `DateTime<Tz>` with the provided

  time zone.

  

  # Example

  

  ```rust

  use chrono::{FixedOffset, NaiveDate};

  let hour = 3600;

  let tz = FixedOffset::east_opt(5 * hour).unwrap();

  let dt = NaiveDate::from_ymd_opt(2015, 9, 5)

      .unwrap()

      .and_hms_opt(23, 56, 4)

      .unwrap()

      .and_local_timezone(tz)

      .unwrap();

  assert_eq!(dt.timezone(), tz);

  ```

- <span id="naivedatetime-and-utc"></span>`const fn and_utc(&self) -> DateTime<Utc>` — [`DateTime`](datetime/index.md#datetime), [`Utc`](offset/utc/index.md#utc)

  Converts the `NaiveDateTime` into the timezone-aware `DateTime<Utc>`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, Utc};

  let dt =

      NaiveDate::from_ymd_opt(2023, 1, 30).unwrap().and_hms_opt(19, 32, 33).unwrap().and_utc();

  assert_eq!(dt.timezone(), Utc);

  ```

- <span id="naivedatetime-const-min"></span>`const MIN: Self`

- <span id="naivedatetime-const-max"></span>`const MAX: Self`

- <span id="naivedatetime-const-unix-epoch"></span>`const UNIX_EPOCH: Self`

#### Trait Implementations

##### `impl Add for NaiveDateTime`

- <span id="naivedatetime-add-type-output"></span>`type Output = NaiveDateTime`

- <span id="naivedatetime-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveDateTime` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

##### `impl AddAssign for NaiveDateTime`

- <span id="naivedatetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Clone for NaiveDateTime`

- <span id="naivedatetime-clone"></span>`fn clone(&self) -> NaiveDateTime` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

##### `impl Copy for NaiveDateTime`

##### `impl Datelike for NaiveDateTime`

- <span id="naivedatetime-datelike-year"></span>`fn year(&self) -> i32`

  Returns the year number in the [calendar date](./struct.NaiveDate.html#calendar-date).

  

  See also the [`NaiveDate::year`](./struct.NaiveDate.html#method.year) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.year(), 2015);

  ```

- <span id="naivedatetime-datelike-month"></span>`fn month(&self) -> u32`

  Returns the month number starting from 1.

  

  The return value ranges from 1 to 12.

  

  See also the [`NaiveDate::month`](./struct.NaiveDate.html#method.month) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.month(), 9);

  ```

- <span id="naivedatetime-datelike-month0"></span>`fn month0(&self) -> u32`

  Returns the month number starting from 0.

  

  The return value ranges from 0 to 11.

  

  See also the `NaiveDate::month0` method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.month0(), 8);

  ```

- <span id="naivedatetime-datelike-day"></span>`fn day(&self) -> u32`

  Returns the day of month starting from 1.

  

  The return value ranges from 1 to 31. (The last day of month differs by months.)

  

  See also the [`NaiveDate::day`](./struct.NaiveDate.html#method.day) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.day(), 25);

  ```

- <span id="naivedatetime-datelike-day0"></span>`fn day0(&self) -> u32`

  Returns the day of month starting from 0.

  

  The return value ranges from 0 to 30. (The last day of month differs by months.)

  

  See also the `NaiveDate::day0` method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.day0(), 24);

  ```

- <span id="naivedatetime-datelike-ordinal"></span>`fn ordinal(&self) -> u32`

  Returns the day of year starting from 1.

  

  The return value ranges from 1 to 366. (The last day of year differs by years.)

  

  See also the [`NaiveDate::ordinal`](./struct.NaiveDate.html#method.ordinal) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.ordinal(), 268);

  ```

- <span id="naivedatetime-datelike-ordinal0"></span>`fn ordinal0(&self) -> u32`

  Returns the day of year starting from 0.

  

  The return value ranges from 0 to 365. (The last day of year differs by years.)

  

  See also the `NaiveDate::ordinal0` method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.ordinal0(), 267);

  ```

- <span id="naivedatetime-datelike-weekday"></span>`fn weekday(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

  Returns the day of week.

  

  See also the [`NaiveDate::weekday`](./struct.NaiveDate.html#method.weekday) method.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime, Weekday};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(dt.weekday(), Weekday::Fri);

  ```

- <span id="naivedatetime-datelike-iso-week"></span>`fn iso_week(&self) -> IsoWeek` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

- <span id="naivedatetime-datelike-with-year"></span>`fn with_year(&self, year: i32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the year number changed, while keeping the same month and

  day.

  

  See also the `NaiveDate::with_year` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (February 29 in a non-leap year).

  - The year is out of range for a `NaiveDate`.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_year(2016),

      Some(NaiveDate::from_ymd_opt(2016, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(

      dt.with_year(-308),

      Some(NaiveDate::from_ymd_opt(-308, 9, 25).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  ```

- <span id="naivedatetime-datelike-with-month"></span>`fn with_month(&self, month: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the month number (starting from 1) changed.

  

  Don't combine multiple `Datelike::with_*` methods. The intermediate value may not exist.

  

  See also the `NaiveDate::with_month` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month(4)` when day of the month is 31).

  - The value for `month` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_month(10),

      Some(NaiveDate::from_ymd_opt(2015, 10, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_month(13), None); // No month 13

  assert_eq!(dt.with_month(2), None); // No February 30

  ```

- <span id="naivedatetime-datelike-with-month0"></span>`fn with_month0(&self, month0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the month number (starting from 0) changed.

  

  See also the `NaiveDate::with_month0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `month0(3)` when day of the month is 31).

  - The value for `month0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_month0(9),

      Some(NaiveDate::from_ymd_opt(2015, 10, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_month0(12), None); // No month 13

  assert_eq!(dt.with_month0(1), None); // No February 30

  ```

- <span id="naivedatetime-datelike-with-day"></span>`fn with_day(&self, day: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the day of month (starting from 1) changed.

  

  See also the `NaiveDate::with_day` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(31)` in April).

  - The value for `day` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_day(30),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_day(31), None); // no September 31

  ```

- <span id="naivedatetime-datelike-with-day0"></span>`fn with_day0(&self, day0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the day of month (starting from 0) changed.

  

  See also the `NaiveDate::with_day0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (for example `day(30)` in April).

  - The value for `day0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_day0(29),

      Some(NaiveDate::from_ymd_opt(2015, 9, 30).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_day0(30), None); // no September 31

  ```

- <span id="naivedatetime-datelike-with-ordinal"></span>`fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the day of year (starting from 1) changed.

  

  See also the `NaiveDate::with_ordinal` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal(366)` in a non-leap year).

  - The value for `ordinal` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal(60),

      Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_ordinal(366), None); // 2015 had only 365 days

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2016, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal(60),

      Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(

      dt.with_ordinal(366),

      Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  ```

- <span id="naivedatetime-datelike-with-ordinal0"></span>`fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the day of year (starting from 0) changed.

  

  See also the `NaiveDate::with_ordinal0` method.

  

  # Errors

  

  Returns `None` if:

  - The resulting date does not exist (`with_ordinal0(365)` in a non-leap year).

  - The value for `ordinal0` is invalid.

  

  # Example

  

  ```rust

  use chrono::{Datelike, NaiveDate, NaiveDateTime};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal0(59),

      Some(NaiveDate::from_ymd_opt(2015, 3, 1).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(dt.with_ordinal0(365), None); // 2015 had only 365 days

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2016, 9, 8).unwrap().and_hms_opt(12, 34, 56).unwrap();

  assert_eq!(

      dt.with_ordinal0(59),

      Some(NaiveDate::from_ymd_opt(2016, 2, 29).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  assert_eq!(

      dt.with_ordinal0(365),

      Some(NaiveDate::from_ymd_opt(2016, 12, 31).unwrap().and_hms_opt(12, 34, 56).unwrap())

  );

  ```

##### `impl Debug for NaiveDateTime`

- <span id="naivedatetime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NaiveDateTime`

- <span id="naivedatetime-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::NaiveDateTime`

- <span id="supernaivedatetime-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for NaiveDateTime`

##### `impl Display for NaiveDateTime`

- <span id="naivedatetime-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DurationRound for crate::NaiveDateTime`

- <span id="cratenaivedatetime-durationround-type-err"></span>`type Err = RoundingError`

- <span id="cratenaivedatetime-durationround-duration-round"></span>`fn duration_round(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DurationRound`](round/index.md#durationround)

- <span id="cratenaivedatetime-durationround-duration-trunc"></span>`fn duration_trunc(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DurationRound`](round/index.md#durationround)

- <span id="cratenaivedatetime-durationround-duration-round-up"></span>`fn duration_round_up(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>` — [`TimeDelta`](time_delta/index.md#timedelta), [`DurationRound`](round/index.md#durationround)

##### `impl Eq for NaiveDateTime`

##### `impl FromStr for NaiveDateTime`

- <span id="naivedatetime-fromstr-type-err"></span>`type Err = ParseError`

- <span id="naivedatetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveDateTime>` — [`ParseResult`](format/index.md#parseresult), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

##### `impl Hash for NaiveDateTime`

- <span id="naivedatetime-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveDateTime`

- <span id="naivedatetime-ord-cmp"></span>`fn cmp(&self, other: &NaiveDateTime) -> cmp::Ordering` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

##### `impl PartialEq for NaiveDateTime`

- <span id="naivedatetime-partialeq-eq"></span>`fn eq(&self, other: &NaiveDateTime) -> bool` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

##### `impl PartialOrd for NaiveDateTime`

- <span id="naivedatetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveDateTime) -> option::Option<cmp::Ordering>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

##### `impl Serialize for super::NaiveDateTime`

- <span id="supernaivedatetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveDateTime`

##### `impl Sub for NaiveDateTime`

- <span id="naivedatetime-sub-type-output"></span>`type Output = NaiveDateTime`

- <span id="naivedatetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> NaiveDateTime` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

##### `impl SubAssign for NaiveDateTime`

- <span id="naivedatetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl SubsecRound for NaiveDateTime`

- <span id="naivedatetime-subsecround-round-subsecs"></span>`fn round_subsecs(self, digits: u16) -> T`

- <span id="naivedatetime-subsecround-trunc-subsecs"></span>`fn trunc_subsecs(self, digits: u16) -> T`

##### `impl Timelike for NaiveDateTime`

- <span id="naivedatetime-timelike-hour"></span>`fn hour(&self) -> u32`

  Returns the hour number from 0 to 23.

  

  See also the `NaiveTime::hour` method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.hour(), 12);

  ```

- <span id="naivedatetime-timelike-minute"></span>`fn minute(&self) -> u32`

  Returns the minute number from 0 to 59.

  

  See also the `NaiveTime::minute` method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.minute(), 34);

  ```

- <span id="naivedatetime-timelike-second"></span>`fn second(&self) -> u32`

  Returns the second number from 0 to 59.

  

  See also the `NaiveTime::second` method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.second(), 56);

  ```

- <span id="naivedatetime-timelike-nanosecond"></span>`fn nanosecond(&self) -> u32`

  Returns the number of nanoseconds since the whole non-leap second.

  The range from 1,000,000,000 to 1,999,999,999 represents

  the [leap second](./struct.NaiveTime.html#leap-second-handling).

  

  See also the [`NaiveTime#method.nanosecond`](naive/time/index.md) method.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(dt.nanosecond(), 789_000_000);

  ```

- <span id="naivedatetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the hour number changed.

  

  See also the `NaiveTime::with_hour` method.

  

  # Errors

  

  Returns `None` if the value for `hour` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(

      dt.with_hour(7),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(7, 34, 56, 789).unwrap()

      )

  );

  assert_eq!(dt.with_hour(24), None);

  ```

- <span id="naivedatetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the minute number changed.

  

  See also the `NaiveTime::with_minute` method.

  

  # Errors

  

  Returns `None` if the value for `minute` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(

      dt.with_minute(45),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_milli_opt(12, 45, 56, 789)

              .unwrap()

      )

  );

  assert_eq!(dt.with_minute(60), None);

  ```

- <span id="naivedatetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with the second number changed.

  

  As with the [`second`](#method.second) method,

  the input range is restricted to 0 through 59.

  

  See also the `NaiveTime::with_second` method.

  

  # Errors

  

  Returns `None` if the value for `second` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 56, 789).unwrap();

  assert_eq!(

      dt.with_second(17),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_milli_opt(12, 34, 17, 789)

              .unwrap()

      )

  );

  assert_eq!(dt.with_second(60), None);

  ```

- <span id="naivedatetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<NaiveDateTime>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

  Makes a new `NaiveDateTime` with nanoseconds since the whole non-leap second changed.

  

  Returns `None` when the resulting `NaiveDateTime` would be invalid.

  As with the `NaiveDateTime::nanosecond` method,

  the input range can exceed 1,000,000,000 for leap seconds.

  

  See also the `NaiveTime::with_nanosecond` method.

  

  # Errors

  

  Returns `None` if `nanosecond >= 2,000,000,000`.

  

  # Example

  

  ```rust

  use chrono::{NaiveDate, NaiveDateTime, Timelike};

  

  let dt: NaiveDateTime =

      NaiveDate::from_ymd_opt(2015, 9, 8).unwrap().and_hms_milli_opt(12, 34, 59, 789).unwrap();

  assert_eq!(

      dt.with_nanosecond(333_333_333),

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_nano_opt(12, 34, 59, 333_333_333)

              .unwrap()

      )

  );

  assert_eq!(

      dt.with_nanosecond(1_333_333_333), // leap second

      Some(

          NaiveDate::from_ymd_opt(2015, 9, 8)

              .unwrap()

              .and_hms_nano_opt(12, 34, 59, 1_333_333_333)

              .unwrap()

      )

  );

  assert_eq!(dt.with_nanosecond(2_000_000_000), None);

  ```

##### `impl ToString for NaiveDateTime`

- <span id="naivedatetime-tostring-to-string"></span>`fn to_string(&self) -> String`

### `NaiveTime`

```rust
struct NaiveTime {
    secs: u32,
    frac: u32,
}
```

ISO 8601 time without timezone.
Allows for the nanosecond precision and optional leap second representation.

# Leap Second Handling

Since 1960s, the manmade atomic clock has been so accurate that
it is much more accurate than Earth's own motion.
It became desirable to define the civil time in terms of the atomic clock,
but that risks the desynchronization of the civil time from Earth.
To account for this, the designers of the Coordinated Universal Time (UTC)
made that the UTC should be kept within 0.9 seconds of the observed Earth-bound time.
When the mean solar day is longer than the ideal (86,400 seconds),
the error slowly accumulates and it is necessary to add a **leap second**
to slow the UTC down a bit.
(We may also remove a second to speed the UTC up a bit, but it never happened.)
The leap second, if any, follows 23:59:59 of June 30 or December 31 in the UTC.

Fast forward to the 21st century,
we have seen 26 leap seconds from January 1972 to December 2015.
Yes, 26 seconds. Probably you can read this paragraph within 26 seconds.
But those 26 seconds, and possibly more in the future, are never predictable,
and whether to add a leap second or not is known only before 6 months.
Internet-based clocks (via NTP) do account for known leap seconds,
but the system API normally doesn't (and often can't, with no network connection)
and there is no reliable way to retrieve leap second information.

Chrono does not try to accurately implement leap seconds; it is impossible.
Rather, **it allows for leap seconds but behaves as if there are *no other* leap seconds.**
Various operations will ignore any possible leap second(s)
except when any of the operands were actually leap seconds.

If you cannot tolerate this behavior,
you must use a separate `TimeZone` for the International Atomic Time (TAI).
TAI is like UTC but has no leap seconds, and thus slightly differs from UTC.
Chrono does not yet provide such implementation, but it is planned.

## Representing Leap Seconds

The leap second is indicated via fractional seconds more than 1 second.
This makes possible to treat a leap second as the prior non-leap second
if you don't care about sub-second accuracy.
You should use the proper formatting to get the raw leap second.

All methods accepting fractional seconds will accept such values.

```rust
use chrono::{NaiveDate, NaiveTime};

let t = NaiveTime::from_hms_milli_opt(8, 59, 59, 1_000).unwrap();

let dt1 = NaiveDate::from_ymd_opt(2015, 7, 1)
    .unwrap()
    .and_hms_micro_opt(8, 59, 59, 1_000_000)
    .unwrap();

let dt2 = NaiveDate::from_ymd_opt(2015, 6, 30)
    .unwrap()
    .and_hms_nano_opt(23, 59, 59, 1_000_000_000)
    .unwrap()
    .and_utc();
let _ = (t, dt1, dt2);
```

Note that the leap second can happen anytime given an appropriate time zone;
2015-07-01 01:23:60 would be a proper leap second if UTC+01:24 had existed.
Practically speaking, though, by the time of the first leap second on 1972-06-30,
every time zone offset around the world has standardized to the 5-minute alignment.

## Date And Time Arithmetic

As a concrete example, let's assume that `03:00:60` and `04:00:60` are leap seconds.
In reality, of course, leap seconds are separated by at least 6 months.
We will also use some intuitive concise notations for the explanation.

`Time + TimeDelta`
(short for [`NaiveTime::overflowing_add_signed`](#method.overflowing_add_signed)):

- `03:00:00 + 1s = 03:00:01`.
- `03:00:59 + 60s = 03:01:59`.
- `03:00:59 + 61s = 03:02:00`.
- `03:00:59 + 1s = 03:01:00`.
- `03:00:60 + 1s = 03:01:00`.
  Note that the sum is identical to the previous.
- `03:00:60 + 60s = 03:01:59`.
- `03:00:60 + 61s = 03:02:00`.
- `03:00:60.1 + 0.8s = 03:00:60.9`.

`Time - TimeDelta`
(short for [`NaiveTime::overflowing_sub_signed`](#method.overflowing_sub_signed)):

- `03:00:00 - 1s = 02:59:59`.
- `03:01:00 - 1s = 03:00:59`.
- `03:01:00 - 60s = 03:00:00`.
- `03:00:60 - 60s = 03:00:00`.
  Note that the result is identical to the previous.
- `03:00:60.7 - 0.4s = 03:00:60.3`.
- `03:00:60.7 - 0.9s = 03:00:59.8`.

`Time - Time`
(short for [`NaiveTime::signed_duration_since`](#method.signed_duration_since)):

- `04:00:00 - 03:00:00 = 3600s`.
- `03:01:00 - 03:00:00 = 60s`.
- `03:00:60 - 03:00:00 = 60s`.
  Note that the difference is identical to the previous.
- `03:00:60.6 - 03:00:59.4 = 1.2s`.
- `03:01:00 - 03:00:59.8 = 0.2s`.
- `03:01:00 - 03:00:60.5 = 0.5s`.
  Note that the difference is larger than the previous,
  even though the leap second clearly follows the previous whole second.
- `04:00:60.9 - 03:00:60.1 =
  (04:00:60.9 - 04:00:00) + (04:00:00 - 03:01:00) + (03:01:00 - 03:00:60.1) =
  60.9s + 3540s + 0.9s = 3601.8s`.

In general,

- `Time + TimeDelta` unconditionally equals to `TimeDelta + Time`.

- `Time - TimeDelta` unconditionally equals to `Time + (-TimeDelta)`.

- `Time1 - Time2` unconditionally equals to `-(Time2 - Time1)`.

- Associativity does not generally hold, because
  `(Time + TimeDelta1) - TimeDelta2` no longer equals to `Time + (TimeDelta1 - TimeDelta2)`
  for two positive durations.

    - As a special case, `(Time + TimeDelta) - TimeDelta` also does not equal to `Time`.

    - If you can assume that all durations have the same sign, however,
      then the associativity holds:
      `(Time + TimeDelta1) + TimeDelta2` equals to `Time + (TimeDelta1 + TimeDelta2)`
      for two positive durations.

## Reading And Writing Leap Seconds

The "typical" leap seconds on the minute boundary are
correctly handled both in the formatting and parsing.
The leap second in the human-readable representation
will be represented as the second part being 60, as required by ISO 8601.

```rust
use chrono::NaiveDate;

let dt = NaiveDate::from_ymd_opt(2015, 6, 30)
    .unwrap()
    .and_hms_milli_opt(23, 59, 59, 1_000)
    .unwrap()
    .and_utc();
assert_eq!(format!("{:?}", dt), "2015-06-30T23:59:60Z");
```

There are hypothetical leap seconds not on the minute boundary nevertheless supported by Chrono.
They are allowed for the sake of completeness and consistency; there were several "exotic" time
zone offsets with fractional minutes prior to UTC after all.
For such cases the human-readable representation is ambiguous and would be read back to the next
non-leap second.

A `NaiveTime` with a leap second that is not on a minute boundary can only be created from a
[`DateTime`](crate::DateTime) with fractional minutes as offset, or using
`Timelike::with_nanosecond()`.

```rust
use chrono::{FixedOffset, NaiveDate, TimeZone};

let paramaribo_pre1945 = FixedOffset::east_opt(-13236).unwrap(); // -03:40:36
let leap_sec_2015 =
    NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_milli_opt(23, 59, 59, 1_000).unwrap();
let dt1 = paramaribo_pre1945.from_utc_datetime(&leap_sec_2015);
assert_eq!(format!("{:?}", dt1), "2015-06-30T20:19:24-03:40:36");
assert_eq!(format!("{:?}", dt1.time()), "20:19:24");

let next_sec = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
let dt2 = paramaribo_pre1945.from_utc_datetime(&next_sec);
assert_eq!(format!("{:?}", dt2), "2015-06-30T20:19:24-03:40:36");
assert_eq!(format!("{:?}", dt2.time()), "20:19:24");

assert!(dt1.time() != dt2.time());
assert!(dt1.time().to_string() == dt2.time().to_string());
```

Since Chrono alone cannot determine any existence of leap seconds,
**there is absolutely no guarantee that the leap second read has actually happened**.

#### Implementations

- <span id="naivetime-from-hms"></span>`const fn from_hms(hour: u32, min: u32, sec: u32) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute and second.

  

  No [leap second](#leap-second-handling) is allowed here;

  use `NaiveTime::from_hms_*` methods with a subsecond parameter instead.

  

  # Panics

  

  Panics on invalid hour, minute and/or second.

- <span id="naivetime-from-hms-opt"></span>`const fn from_hms_opt(hour: u32, min: u32, sec: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute and second.

  

  The millisecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute and/or second.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hms_opt = NaiveTime::from_hms_opt;

  

  assert!(from_hms_opt(0, 0, 0).is_some());

  assert!(from_hms_opt(23, 59, 59).is_some());

  assert!(from_hms_opt(24, 0, 0).is_none());

  assert!(from_hms_opt(23, 60, 0).is_none());

  assert!(from_hms_opt(23, 59, 60).is_none());

  ```

- <span id="naivetime-from-hms-milli"></span>`const fn from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and millisecond.

  

  The millisecond part can exceed 1,000

  in order to represent the [leap second](#leap-second-handling).

  

  # Panics

  

  Panics on invalid hour, minute, second and/or millisecond.

- <span id="naivetime-from-hms-milli-opt"></span>`const fn from_hms_milli_opt(hour: u32, min: u32, sec: u32, milli: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and millisecond.

  

  The millisecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or millisecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hmsm_opt = NaiveTime::from_hms_milli_opt;

  

  assert!(from_hmsm_opt(0, 0, 0, 0).is_some());

  assert!(from_hmsm_opt(23, 59, 59, 999).is_some());

  assert!(from_hmsm_opt(23, 59, 59, 1_999).is_some()); // a leap second after 23:59:59

  assert!(from_hmsm_opt(24, 0, 0, 0).is_none());

  assert!(from_hmsm_opt(23, 60, 0, 0).is_none());

  assert!(from_hmsm_opt(23, 59, 60, 0).is_none());

  assert!(from_hmsm_opt(23, 59, 59, 2_000).is_none());

  ```

- <span id="naivetime-from-hms-micro"></span>`const fn from_hms_micro(hour: u32, min: u32, sec: u32, micro: u32) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or microsecond.

- <span id="naivetime-from-hms-micro-opt"></span>`const fn from_hms_micro_opt(hour: u32, min: u32, sec: u32, micro: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and microsecond.

  

  The microsecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or microsecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hmsu_opt = NaiveTime::from_hms_micro_opt;

  

  assert!(from_hmsu_opt(0, 0, 0, 0).is_some());

  assert!(from_hmsu_opt(23, 59, 59, 999_999).is_some());

  assert!(from_hmsu_opt(23, 59, 59, 1_999_999).is_some()); // a leap second after 23:59:59

  assert!(from_hmsu_opt(24, 0, 0, 0).is_none());

  assert!(from_hmsu_opt(23, 60, 0, 0).is_none());

  assert!(from_hmsu_opt(23, 59, 60, 0).is_none());

  assert!(from_hmsu_opt(23, 59, 59, 2_000_000).is_none());

  ```

- <span id="naivetime-from-hms-nano"></span>`const fn from_hms_nano(hour: u32, min: u32, sec: u32, nano: u32) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Panics

  

  Panics on invalid hour, minute, second and/or nanosecond.

- <span id="naivetime-from-hms-nano-opt"></span>`const fn from_hms_nano_opt(hour: u32, min: u32, sec: u32, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from hour, minute, second and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `sec == 59`.

  

  # Errors

  

  Returns `None` on invalid hour, minute, second and/or nanosecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_hmsn_opt = NaiveTime::from_hms_nano_opt;

  

  assert!(from_hmsn_opt(0, 0, 0, 0).is_some());

  assert!(from_hmsn_opt(23, 59, 59, 999_999_999).is_some());

  assert!(from_hmsn_opt(23, 59, 59, 1_999_999_999).is_some()); // a leap second after 23:59:59

  assert!(from_hmsn_opt(24, 0, 0, 0).is_none());

  assert!(from_hmsn_opt(23, 60, 0, 0).is_none());

  assert!(from_hmsn_opt(23, 59, 60, 0).is_none());

  assert!(from_hmsn_opt(23, 59, 59, 2_000_000_000).is_none());

  ```

- <span id="naivetime-from-num-seconds-from-midnight"></span>`const fn from_num_seconds_from_midnight(secs: u32, nano: u32) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `secs % 60 == 59`.

  

  # Panics

  

  Panics on invalid number of seconds and/or nanosecond.

- <span id="naivetime-from-num-seconds-from-midnight-opt"></span>`const fn from_num_seconds_from_midnight_opt(secs: u32, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` from the number of seconds since midnight and nanosecond.

  

  The nanosecond part is allowed to exceed 1,000,000,000 in order to represent a

  [leap second](#leap-second-handling), but only when `secs % 60 == 59`.

  

  # Errors

  

  Returns `None` on invalid number of seconds and/or nanosecond.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let from_nsecs_opt = NaiveTime::from_num_seconds_from_midnight_opt;

  

  assert!(from_nsecs_opt(0, 0).is_some());

  assert!(from_nsecs_opt(86399, 999_999_999).is_some());

  assert!(from_nsecs_opt(86399, 1_999_999_999).is_some()); // a leap second after 23:59:59

  assert!(from_nsecs_opt(86_400, 0).is_none());

  assert!(from_nsecs_opt(86399, 2_000_000_000).is_none());

  ```

- <span id="naivetime-parse-from-str"></span>`fn parse_from_str(s: &str, fmt: &str) -> ParseResult<NaiveTime>` — [`ParseResult`](format/index.md#parseresult), [`NaiveTime`](naive/time/index.md#naivetime)

  Parses a string with the specified format string and returns a new `NaiveTime`.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  # Example

  

  ```rust

  use chrono::NaiveTime;

  

  let parse_from_str = NaiveTime::parse_from_str;

  

  assert_eq!(

      parse_from_str("23:56:04", "%H:%M:%S"),

      Ok(NaiveTime::from_hms_opt(23, 56, 4).unwrap())

  );

  assert_eq!(

      parse_from_str("pm012345.6789", "%p%I%M%S%.f"),

      Ok(NaiveTime::from_hms_micro_opt(13, 23, 45, 678_900).unwrap())

  );

  ```

  

  Date and offset is ignored for the purpose of parsing.

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert_eq!(

      parse_from_str("2014-5-17T12:34:56+09:30", "%Y-%m-%dT%H:%M:%S%z"),

      Ok(NaiveTime::from_hms_opt(12, 34, 56).unwrap())

  );

  ```

  

  [Leap seconds](#leap-second-handling) are correctly handled by

  treating any time of the form `hh:mm:60` as a leap second.

  (This equally applies to the formatting, so the round trip is possible.)

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert_eq!(

      parse_from_str("08:59:60.123", "%H:%M:%S%.f"),

      Ok(NaiveTime::from_hms_milli_opt(8, 59, 59, 1_123).unwrap())

  );

  ```

  

  Missing seconds are assumed to be zero,

  but out-of-bound times or insufficient fields are errors otherwise.

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert_eq!(parse_from_str("7:15", "%H:%M"), Ok(NaiveTime::from_hms_opt(7, 15, 0).unwrap()));

  

  assert!(parse_from_str("04m33s", "%Mm%Ss").is_err());

  assert!(parse_from_str("12", "%H").is_err());

  assert!(parse_from_str("17:60", "%H:%M").is_err());

  assert!(parse_from_str("24:00:00", "%H:%M:%S").is_err());

  ```

  

  All parsed fields should be consistent to each other, otherwise it's an error.

  Here `%H` is for 24-hour clocks, unlike `%I`,

  and thus can be independently determined without AM/PM.

  

  ```rust

  use chrono::NaiveTime;

  let parse_from_str = NaiveTime::parse_from_str;

  assert!(parse_from_str("13:07 AM", "%H:%M %p").is_err());

  ```

- <span id="naivetime-parse-and-remainder"></span>`fn parse_and_remainder<'a>(s: &'a str, fmt: &str) -> ParseResult<(NaiveTime, &'a str)>` — [`ParseResult`](format/index.md#parseresult), [`NaiveTime`](naive/time/index.md#naivetime)

  Parses a string from a user-specified format into a new `NaiveTime` value, and a slice with

  the remaining portion of the string.

  See the [`format::strftime` module](crate::format::strftime)

  on the supported escape sequences.

  

  Similar to [`parse_from_str`](#method.parse_from_str).

  

  # Example

  

  ```rust

  use chrono::{NaiveTime};

  let (time, remainder) =

      NaiveTime::parse_and_remainder("3h4m33s trailing text", "%-Hh%-Mm%-Ss").unwrap();

  assert_eq!(time, NaiveTime::from_hms_opt(3, 4, 33).unwrap());

  assert_eq!(remainder, " trailing text");

  ```

- <span id="naivetime-overflowing-add-signed"></span>`const fn overflowing_add_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64)` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveTime`](naive/time/index.md#naivetime)

  Adds given `TimeDelta` to the current time, and also returns the number of *seconds*

  in the integral number of days ignored from the addition.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, TimeDelta};

  

  let from_hms = |h, m, s| NaiveTime::from_hms_opt(h, m, s).unwrap();

  

  assert_eq!(

      from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::try_hours(11).unwrap()),

      (from_hms(14, 4, 5), 0)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::try_hours(23).unwrap()),

      (from_hms(2, 4, 5), 86_400)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_add_signed(TimeDelta::try_hours(-7).unwrap()),

      (from_hms(20, 4, 5), -86_400)

  );

  ```

- <span id="naivetime-overflowing-sub-signed"></span>`const fn overflowing_sub_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64)` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveTime`](naive/time/index.md#naivetime)

  Subtracts given `TimeDelta` from the current time, and also returns the number of *seconds*

  in the integral number of days ignored from the subtraction.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, TimeDelta};

  

  let from_hms = |h, m, s| NaiveTime::from_hms_opt(h, m, s).unwrap();

  

  assert_eq!(

      from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::try_hours(2).unwrap()),

      (from_hms(1, 4, 5), 0)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::try_hours(17).unwrap()),

      (from_hms(10, 4, 5), 86_400)

  );

  assert_eq!(

      from_hms(3, 4, 5).overflowing_sub_signed(TimeDelta::try_hours(-22).unwrap()),

      (from_hms(1, 4, 5), -86_400)

  );

  ```

- <span id="naivetime-signed-duration-since"></span>`const fn signed_duration_since(self, rhs: NaiveTime) -> TimeDelta` — [`NaiveTime`](naive/time/index.md#naivetime), [`TimeDelta`](time_delta/index.md#timedelta)

  Subtracts another `NaiveTime` from the current time.

  Returns a `TimeDelta` within +/- 1 day.

  This does not overflow or underflow at all.

  

  As a part of Chrono's [leap second handling](#leap-second-handling),

  the subtraction assumes that **there is no leap second ever**,

  except when any of the `NaiveTime`s themselves represents a leap second

  in which case the assumption becomes that

  **there are exactly one (or two) leap second(s) ever**.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, TimeDelta};

  

  let from_hmsm = |h, m, s, milli| NaiveTime::from_hms_milli_opt(h, m, s, milli).unwrap();

  let since = NaiveTime::signed_duration_since;

  

  assert_eq!(since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 7, 900)), TimeDelta::zero());

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 7, 875)),

      TimeDelta::try_milliseconds(25).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 6, 925)),

      TimeDelta::try_milliseconds(975).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 5, 0, 900)),

      TimeDelta::try_seconds(7).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(3, 0, 7, 900)),

      TimeDelta::try_seconds(5 * 60).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(0, 5, 7, 900)),

      TimeDelta::try_seconds(3 * 3600).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(4, 5, 7, 900)),

      TimeDelta::try_seconds(-3600).unwrap()

  );

  assert_eq!(

      since(from_hmsm(3, 5, 7, 900), from_hmsm(2, 4, 6, 800)),

      TimeDelta::try_seconds(3600 + 60 + 1).unwrap() + TimeDelta::try_milliseconds(100).unwrap()

  );

  ```

  

  Leap seconds are handled, but the subtraction assumes that

  there were no other leap seconds happened.

  

  ```rust

  use chrono::{TimeDelta, NaiveTime};

  let from_hmsm = |h, m, s, milli| { NaiveTime::from_hms_milli_opt(h, m, s, milli).unwrap() };

  let since = NaiveTime::signed_duration_since;

  assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(3, 0, 59, 0)),

             TimeDelta::try_seconds(1).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 59, 1_500), from_hmsm(3, 0, 59, 0)),

             TimeDelta::try_milliseconds(1500).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(3, 0, 0, 0)),

             TimeDelta::try_seconds(60).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 0, 0), from_hmsm(2, 59, 59, 1_000)),

             TimeDelta::try_seconds(1).unwrap());

  assert_eq!(since(from_hmsm(3, 0, 59, 1_000), from_hmsm(2, 59, 59, 1_000)),

             TimeDelta::try_seconds(61).unwrap());

  ```

- <span id="naivetime-overflowing-add-offset"></span>`const fn overflowing_add_offset(&self, offset: FixedOffset) -> (NaiveTime, i32)` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`NaiveTime`](naive/time/index.md#naivetime)

  Adds given `FixedOffset` to the current time, and returns the number of days that should be

  added to a date as a result of the offset (either `-1`, `0`, or `1` because the offset is

  always less than 24h).

  

  This method is similar to [`overflowing_add_signed`](#method.overflowing_add_signed), but

  preserves leap seconds.

- <span id="naivetime-overflowing-sub-offset"></span>`const fn overflowing_sub_offset(&self, offset: FixedOffset) -> (NaiveTime, i32)` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`NaiveTime`](naive/time/index.md#naivetime)

  Subtracts given `FixedOffset` from the current time, and returns the number of days that

  should be added to a date as a result of the offset (either `-1`, `0`, or `1` because the

  offset is always less than 24h).

  

  This method is similar to [`overflowing_sub_signed`](#method.overflowing_sub_signed), but

  preserves leap seconds.

- <span id="naivetime-hms"></span>`fn hms(&self) -> (u32, u32, u32)`

  Returns a triple of the hour, minute and second numbers.

- <span id="naivetime-num-seconds-from-midnight"></span>`const fn num_seconds_from_midnight(&self) -> u32`

  Returns the number of non-leap seconds past the last midnight.

- <span id="naivetime-nanosecond"></span>`const fn nanosecond(&self) -> u32`

  Returns the number of nanoseconds since the whole non-leap second.

- <span id="naivetime-const-min"></span>`const MIN: Self`

- <span id="naivetime-const-max"></span>`const MAX: Self`

#### Trait Implementations

##### `impl Add for NaiveTime`

- <span id="naivetime-add-type-output"></span>`type Output = NaiveTime`

- <span id="naivetime-add"></span>`fn add(self, rhs: TimeDelta) -> NaiveTime` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveTime`](naive/time/index.md#naivetime)

##### `impl AddAssign for NaiveTime`

- <span id="naivetime-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl Clone for NaiveTime`

- <span id="naivetime-clone"></span>`fn clone(&self) -> NaiveTime` — [`NaiveTime`](naive/time/index.md#naivetime)

##### `impl Copy for NaiveTime`

##### `impl Debug for NaiveTime`

- <span id="naivetime-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NaiveTime`

- <span id="naivetime-default"></span>`fn default() -> Self`

##### `impl Deserialize for super::NaiveTime`

- <span id="supernaivetime-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for NaiveTime`

##### `impl Display for NaiveTime`

- <span id="naivetime-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NaiveTime`

##### `impl FromStr for NaiveTime`

- <span id="naivetime-fromstr-type-err"></span>`type Err = ParseError`

- <span id="naivetime-fromstr-from-str"></span>`fn from_str(s: &str) -> ParseResult<NaiveTime>` — [`ParseResult`](format/index.md#parseresult), [`NaiveTime`](naive/time/index.md#naivetime)

##### `impl Hash for NaiveTime`

- <span id="naivetime-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for NaiveTime`

- <span id="naivetime-ord-cmp"></span>`fn cmp(&self, other: &NaiveTime) -> cmp::Ordering` — [`NaiveTime`](naive/time/index.md#naivetime)

##### `impl PartialEq for NaiveTime`

- <span id="naivetime-partialeq-eq"></span>`fn eq(&self, other: &NaiveTime) -> bool` — [`NaiveTime`](naive/time/index.md#naivetime)

##### `impl PartialOrd for NaiveTime`

- <span id="naivetime-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NaiveTime) -> option::Option<cmp::Ordering>` — [`NaiveTime`](naive/time/index.md#naivetime)

##### `impl Serialize for super::NaiveTime`

- <span id="supernaivetime-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for NaiveTime`

##### `impl Sub for NaiveTime`

- <span id="naivetime-sub-type-output"></span>`type Output = NaiveTime`

- <span id="naivetime-sub"></span>`fn sub(self, rhs: TimeDelta) -> NaiveTime` — [`TimeDelta`](time_delta/index.md#timedelta), [`NaiveTime`](naive/time/index.md#naivetime)

##### `impl SubAssign for NaiveTime`

- <span id="naivetime-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: TimeDelta)` — [`TimeDelta`](time_delta/index.md#timedelta)

##### `impl SubsecRound for NaiveTime`

- <span id="naivetime-subsecround-round-subsecs"></span>`fn round_subsecs(self, digits: u16) -> T`

- <span id="naivetime-subsecround-trunc-subsecs"></span>`fn trunc_subsecs(self, digits: u16) -> T`

##### `impl Timelike for NaiveTime`

- <span id="naivetime-timelike-hour"></span>`fn hour(&self) -> u32`

  Returns the hour number from 0 to 23.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().hour(), 0);

  assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().hour(), 23);

  ```

- <span id="naivetime-timelike-minute"></span>`fn minute(&self) -> u32`

  Returns the minute number from 0 to 59.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().minute(), 0);

  assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().minute(), 56);

  ```

- <span id="naivetime-timelike-second"></span>`fn second(&self) -> u32`

  Returns the second number from 0 to 59.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().second(), 0);

  assert_eq!(NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().second(), 4);

  ```

  

  This method never returns 60 even when it is a leap second.

  ([Why?](#leap-second-handling))

  Use the proper [formatting method](#method.format) to get a human-readable representation.

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{NaiveTime, Timelike};

  let leap = NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap();

  assert_eq!(leap.second(), 59);

  assert_eq!(leap.format("%H:%M:%S").to_string(), "23:59:60");

  }

  ```

- <span id="naivetime-timelike-nanosecond"></span>`fn nanosecond(&self) -> u32`

  Returns the number of nanoseconds since the whole non-leap second.

  The range from 1,000,000,000 to 1,999,999,999 represents

  the [leap second](#leap-second-handling).

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap().nanosecond(), 0);

  assert_eq!(

      NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().nanosecond(),

      12_345_678

  );

  ```

  

  Leap seconds may have seemingly out-of-range return values.

  You can reduce the range with `time.nanosecond() % 1_000_000_000`, or

  use the proper [formatting method](#method.format) to get a human-readable representation.

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{NaiveTime, Timelike};

  let leap = NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap();

  assert_eq!(leap.nanosecond(), 1_000_000_000);

  assert_eq!(leap.format("%H:%M:%S%.9f").to_string(), "23:59:60.000000000");

  }

  ```

- <span id="naivetime-timelike-with-hour"></span>`fn with_hour(&self, hour: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` with the hour number changed.

  

  # Errors

  

  Returns `None` if the value for `hour` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(dt.with_hour(7), Some(NaiveTime::from_hms_nano_opt(7, 56, 4, 12_345_678).unwrap()));

  assert_eq!(dt.with_hour(24), None);

  ```

- <span id="naivetime-timelike-with-minute"></span>`fn with_minute(&self, min: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` with the minute number changed.

  

  # Errors

  

  Returns `None` if the value for `minute` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(

      dt.with_minute(45),

      Some(NaiveTime::from_hms_nano_opt(23, 45, 4, 12_345_678).unwrap())

  );

  assert_eq!(dt.with_minute(60), None);

  ```

- <span id="naivetime-timelike-with-second"></span>`fn with_second(&self, sec: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` with the second number changed.

  

  As with the [`second`](#method.second) method,

  the input range is restricted to 0 through 59.

  

  # Errors

  

  Returns `None` if the value for `second` is invalid.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(

      dt.with_second(17),

      Some(NaiveTime::from_hms_nano_opt(23, 56, 17, 12_345_678).unwrap())

  );

  assert_eq!(dt.with_second(60), None);

  ```

- <span id="naivetime-timelike-with-nanosecond"></span>`fn with_nanosecond(&self, nano: u32) -> Option<NaiveTime>` — [`NaiveTime`](naive/time/index.md#naivetime)

  Makes a new `NaiveTime` with nanoseconds since the whole non-leap second changed.

  

  As with the [`nanosecond`](#method.nanosecond) method,

  the input range can exceed 1,000,000,000 for leap seconds.

  

  # Errors

  

  Returns `None` if `nanosecond >= 2,000,000,000`.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  assert_eq!(

      dt.with_nanosecond(333_333_333),

      Some(NaiveTime::from_hms_nano_opt(23, 56, 4, 333_333_333).unwrap())

  );

  assert_eq!(dt.with_nanosecond(2_000_000_000), None);

  ```

  

  Leap seconds can theoretically follow *any* whole second.

  The following would be a proper leap second at the time zone offset of UTC-00:03:57

  (there are several historical examples comparable to this "non-sense" offset),

  and therefore is allowed.

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  let dt = NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap();

  let strange_leap_second = dt.with_nanosecond(1_333_333_333).unwrap();

  assert_eq!(strange_leap_second.nanosecond(), 1_333_333_333);

  ```

- <span id="naivetime-timelike-num-seconds-from-midnight"></span>`fn num_seconds_from_midnight(&self) -> u32`

  Returns the number of non-leap seconds past the last midnight.

  

  # Example

  

  ```rust

  use chrono::{NaiveTime, Timelike};

  

  assert_eq!(NaiveTime::from_hms_opt(1, 2, 3).unwrap().num_seconds_from_midnight(), 3723);

  assert_eq!(

      NaiveTime::from_hms_nano_opt(23, 56, 4, 12_345_678).unwrap().num_seconds_from_midnight(),

      86164

  );

  assert_eq!(

      NaiveTime::from_hms_milli_opt(23, 59, 59, 1_000).unwrap().num_seconds_from_midnight(),

      86399

  );

  ```

##### `impl ToString for NaiveTime`

- <span id="naivetime-tostring-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="isoweek-from-yof"></span>`fn from_yof(year: i32, ordinal: u32, year_flags: YearFlags) -> Self` — [`YearFlags`](naive/internals/index.md#yearflags)

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

- <span id="isoweek-clone"></span>`fn clone(&self) -> IsoWeek` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

##### `impl Copy for IsoWeek`

##### `impl Debug for IsoWeek`

- <span id="isoweek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IsoWeek`

##### `impl Hash for IsoWeek`

- <span id="isoweek-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for IsoWeek`

- <span id="isoweek-ord-cmp"></span>`fn cmp(&self, other: &IsoWeek) -> cmp::Ordering` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

##### `impl PartialEq for IsoWeek`

- <span id="isoweek-partialeq-eq"></span>`fn eq(&self, other: &IsoWeek) -> bool` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

##### `impl PartialOrd for IsoWeek`

- <span id="isoweek-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &IsoWeek) -> option::Option<cmp::Ordering>` — [`IsoWeek`](naive/isoweek/index.md#isoweek)

##### `impl StructuralPartialEq for IsoWeek`

### `NaiveWeek`

```rust
struct NaiveWeek {
    date: NaiveDate,
    start: crate::Weekday,
}
```

A week represented by a [`NaiveDate`](naive/date/index.md) and a [`Weekday`](weekday/index.md) which is the first
day of the week.

#### Implementations

- <span id="naiveweek-new"></span>`const fn new(date: NaiveDate, start: Weekday) -> Self` — [`NaiveDate`](naive/date/index.md#naivedate), [`Weekday`](weekday/index.md#weekday)

  Create a new `NaiveWeek`

- <span id="naiveweek-first-day"></span>`const fn first_day(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns a date representing the first day of the week.

  

  # Panics

  

  Panics if the first day of the week happens to fall just out of range of `NaiveDate`

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();

  let week = date.week(Weekday::Mon);

  assert!(week.first_day() <= date);

  ```

- <span id="naiveweek-checked-first-day"></span>`const fn checked_first_day(&self) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns a date representing the first day of the week or

  `None` if the date is out of `NaiveDate`'s range

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::MIN;

  let week = date.week(Weekday::Mon);

  if let Some(first_day) = week.checked_first_day() {

      assert!(first_day == date);

  } else {

      // error handling code

      return;

  };

  ```

- <span id="naiveweek-last-day"></span>`const fn last_day(&self) -> NaiveDate` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns a date representing the last day of the week.

  

  # Panics

  

  Panics if the last day of the week happens to fall just out of range of `NaiveDate`

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();

  let week = date.week(Weekday::Mon);

  assert!(week.last_day() >= date);

  ```

- <span id="naiveweek-checked-last-day"></span>`const fn checked_last_day(&self) -> Option<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns a date representing the last day of the week or

  `None` if the date is out of `NaiveDate`'s range

  (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::MAX;

  let week = date.week(Weekday::Mon);

  if let Some(last_day) = week.checked_last_day() {

      assert!(last_day == date);

  } else {

      // error handling code

      return;

  };

  ```

- <span id="naiveweek-days"></span>`const fn days(&self) -> RangeInclusive<NaiveDate>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns a [`RangeInclusive<T>`](../num_iter/index.md) representing the whole week bounded by

  [first_day](NaiveWeek::first_day) and [last_day](NaiveWeek::last_day) functions.

  

  # Panics

  

  Panics if the either the first or last day of the week happens to fall just out of range of

  `NaiveDate` (more than ca. 262,000 years away from common era).

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::from_ymd_opt(2022, 4, 18).unwrap();

  let week = date.week(Weekday::Mon);

  let days = week.days();

  assert!(days.contains(&date));

  ```

- <span id="naiveweek-checked-days"></span>`const fn checked_days(&self) -> Option<RangeInclusive<NaiveDate>>` — [`NaiveDate`](naive/date/index.md#naivedate)

  Returns an [`Option<RangeInclusive<T>>`](../serde_core/index.md) representing the whole week bounded by

  [checked_first_day](NaiveWeek::checked_first_day) and

  [checked_last_day](NaiveWeek::checked_last_day) functions.

  

  Returns `None` if either of the boundaries are out of `NaiveDate`'s range

  (more than ca. 262,000 years away from common era).

  

  

  # Examples

  

  ```rust

  use chrono::{NaiveDate, Weekday};

  

  let date = NaiveDate::MAX;

  let week = date.week(Weekday::Mon);

  let _days = match week.checked_days() {

      Some(d) => d,

      None => {

          // error handling code

          return;

      }

  };

  ```

#### Trait Implementations

##### `impl Clone for NaiveWeek`

- <span id="naiveweek-clone"></span>`fn clone(&self) -> NaiveWeek` — [`NaiveWeek`](naive/index.md#naiveweek)

##### `impl Copy for NaiveWeek`

##### `impl Debug for NaiveWeek`

- <span id="naiveweek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NaiveWeek`

##### `impl Hash for NaiveWeek`

- <span id="naiveweek-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for NaiveWeek`

- <span id="naiveweek-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

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

- <span id="fixedoffset-east"></span>`fn east(secs: i32) -> FixedOffset` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

  Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.

  The negative `secs` means the Western Hemisphere.

  

  Panics on the out-of-bound `secs`.

- <span id="fixedoffset-east-opt"></span>`const fn east_opt(secs: i32) -> Option<FixedOffset>` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

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

- <span id="fixedoffset-west"></span>`fn west(secs: i32) -> FixedOffset` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

  Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.

  The negative `secs` means the Eastern Hemisphere.

  

  Panics on the out-of-bound `secs`.

- <span id="fixedoffset-west-opt"></span>`const fn west_opt(secs: i32) -> Option<FixedOffset>` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

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

- <span id="datetime-add"></span>`fn add(self, rhs: FixedOffset) -> DateTime<Tz>` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`DateTime`](datetime/index.md#datetime)

##### `impl Clone for FixedOffset`

- <span id="fixedoffset-clone"></span>`fn clone(&self) -> FixedOffset` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

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

- <span id="fixedoffset-offset-fix"></span>`fn fix(&self) -> FixedOffset` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

##### `impl PartialEq for FixedOffset`

- <span id="fixedoffset-partialeq-eq"></span>`fn eq(&self, other: &FixedOffset) -> bool` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

##### `impl StructuralPartialEq for FixedOffset`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, rhs: FixedOffset) -> DateTime<Tz>` — [`FixedOffset`](offset/fixed/index.md#fixedoffset), [`DateTime`](datetime/index.md#datetime)

##### `impl TimeZone for FixedOffset`

- <span id="fixedoffset-timezone-type-offset"></span>`type Offset = FixedOffset`

- <span id="fixedoffset-timezone-from-offset"></span>`fn from_offset(offset: &FixedOffset) -> FixedOffset` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-local-date"></span>`fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<FixedOffset>` — [`NaiveDate`](naive/date/index.md#naivedate), [`MappedLocalTime`](offset/index.md#mappedlocaltime), [`FixedOffset`](offset/fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-local-datetime"></span>`fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> MappedLocalTime<FixedOffset>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`MappedLocalTime`](offset/index.md#mappedlocaltime), [`FixedOffset`](offset/fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-utc-date"></span>`fn offset_from_utc_date(&self, _utc: &NaiveDate) -> FixedOffset` — [`NaiveDate`](naive/date/index.md#naivedate), [`FixedOffset`](offset/fixed/index.md#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-utc-datetime"></span>`fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> FixedOffset` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`FixedOffset`](offset/fixed/index.md#fixedoffset)

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

- <span id="utc-clone"></span>`fn clone(&self) -> Utc` — [`Utc`](offset/utc/index.md#utc)

##### `impl Copy for Utc`

##### `impl Debug for Utc`

- <span id="utc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utc`

- <span id="utc-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utc`

##### `impl Hash for Utc`

- <span id="utc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Offset for Utc`

- <span id="utc-offset-fix"></span>`fn fix(&self) -> FixedOffset` — [`FixedOffset`](offset/fixed/index.md#fixedoffset)

##### `impl PartialEq for Utc`

- <span id="utc-partialeq-eq"></span>`fn eq(&self, other: &Utc) -> bool` — [`Utc`](offset/utc/index.md#utc)

##### `impl StructuralPartialEq for Utc`

##### `impl TimeZone for Utc`

- <span id="utc-timezone-type-offset"></span>`type Offset = Utc`

- <span id="utc-timezone-from-offset"></span>`fn from_offset(_state: &Utc) -> Utc` — [`Utc`](offset/utc/index.md#utc)

- <span id="utc-timezone-offset-from-local-date"></span>`fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<Utc>` — [`NaiveDate`](naive/date/index.md#naivedate), [`MappedLocalTime`](offset/index.md#mappedlocaltime), [`Utc`](offset/utc/index.md#utc)

- <span id="utc-timezone-offset-from-local-datetime"></span>`fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> MappedLocalTime<Utc>` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`MappedLocalTime`](offset/index.md#mappedlocaltime), [`Utc`](offset/utc/index.md#utc)

- <span id="utc-timezone-offset-from-utc-date"></span>`fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Utc` — [`NaiveDate`](naive/date/index.md#naivedate), [`Utc`](offset/utc/index.md#utc)

- <span id="utc-timezone-offset-from-utc-datetime"></span>`fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Utc` — [`NaiveDateTime`](naive/datetime/index.md#naivedatetime), [`Utc`](offset/utc/index.md#utc)

##### `impl ToString for Utc`

- <span id="utc-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ParseWeekdayError`

```rust
struct ParseWeekdayError {
    _dummy: (),
}
```

An error resulting from reading `Weekday` value with `FromStr`.

#### Trait Implementations

##### `impl Clone for ParseWeekdayError`

- <span id="parseweekdayerror-clone"></span>`fn clone(&self) -> ParseWeekdayError` — [`ParseWeekdayError`](weekday/index.md#parseweekdayerror)

##### `impl Debug for ParseWeekdayError`

- <span id="parseweekdayerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseWeekdayError`

- <span id="parseweekdayerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseWeekdayError`

##### `impl PartialEq for ParseWeekdayError`

- <span id="parseweekdayerror-partialeq-eq"></span>`fn eq(&self, other: &ParseWeekdayError) -> bool` — [`ParseWeekdayError`](weekday/index.md#parseweekdayerror)

##### `impl StructuralPartialEq for ParseWeekdayError`

##### `impl ToString for ParseWeekdayError`

- <span id="parseweekdayerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `WeekdaySet`

```rust
struct WeekdaySet(u8);
```

A collection of [`Weekday`](weekday/index.md)s stored as a single byte.

This type is `Copy` and provides efficient set-like and slice-like operations.
Many operations are `const` as well.

Implemented as a bitmask where bits 1-7 correspond to Monday-Sunday.

#### Implementations

- <span id="weekdayset-from-array"></span>`const fn from_array<const C: usize>(days: [Weekday; C]) -> Self` — [`Weekday`](weekday/index.md#weekday)

  Create a `WeekdaySet` from an array of [`Weekday`](weekday/index.md)s.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::EMPTY, WeekdaySet::from_array([]));

  assert_eq!(WeekdaySet::single(Mon), WeekdaySet::from_array([Mon]));

  assert_eq!(WeekdaySet::ALL, WeekdaySet::from_array([Mon, Tue, Wed, Thu, Fri, Sat, Sun]));

  ```

- <span id="weekdayset-single"></span>`const fn single(weekday: Weekday) -> Self` — [`Weekday`](weekday/index.md#weekday)

  Create a `WeekdaySet` from a single [`Weekday`](weekday/index.md).

- <span id="weekdayset-single-day"></span>`const fn single_day(self) -> Option<Weekday>` — [`Weekday`](weekday/index.md#weekday)

  Returns `Some(day)` if this collection contains exactly one day.

  

  Returns `None` otherwise.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).single_day(), Some(Mon));

  assert_eq!(WeekdaySet::from_array([Mon, Tue]).single_day(), None);

  assert_eq!(WeekdaySet::EMPTY.single_day(), None);

  assert_eq!(WeekdaySet::ALL.single_day(), None);

  ```

- <span id="weekdayset-insert"></span>`fn insert(&mut self, day: Weekday) -> bool` — [`Weekday`](weekday/index.md#weekday)

  Adds a day to the collection.

  

  Returns `true` if the day was new to the collection.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  let mut weekdays = WeekdaySet::single(Mon);

  assert!(weekdays.insert(Tue));

  assert!(!weekdays.insert(Tue));

  ```

- <span id="weekdayset-remove"></span>`fn remove(&mut self, day: Weekday) -> bool` — [`Weekday`](weekday/index.md#weekday)

  Removes a day from the collection.

  

  Returns `true` if the collection did contain the day.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  let mut weekdays = WeekdaySet::single(Mon);

  assert!(weekdays.remove(Mon));

  assert!(!weekdays.remove(Mon));

  ```

- <span id="weekdayset-is-subset"></span>`const fn is_subset(self, other: Self) -> bool`

  Returns `true` if `other` contains all days in `self`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert!(WeekdaySet::single(Mon).is_subset(WeekdaySet::ALL));

  assert!(!WeekdaySet::single(Mon).is_subset(WeekdaySet::EMPTY));

  assert!(WeekdaySet::EMPTY.is_subset(WeekdaySet::single(Mon)));

  ```

- <span id="weekdayset-intersection"></span>`const fn intersection(self, other: Self) -> Self`

  Returns days that are in both `self` and `other`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).intersection(WeekdaySet::single(Mon)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::single(Mon).intersection(WeekdaySet::single(Tue)), WeekdaySet::EMPTY);

  assert_eq!(WeekdaySet::ALL.intersection(WeekdaySet::single(Mon)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::ALL.intersection(WeekdaySet::EMPTY), WeekdaySet::EMPTY);

  ```

- <span id="weekdayset-union"></span>`const fn union(self, other: Self) -> Self`

  Returns days that are in either `self` or `other`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).union(WeekdaySet::single(Mon)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::single(Mon).union(WeekdaySet::single(Tue)), WeekdaySet::from_array([Mon, Tue]));

  assert_eq!(WeekdaySet::ALL.union(WeekdaySet::single(Mon)), WeekdaySet::ALL);

  assert_eq!(WeekdaySet::ALL.union(WeekdaySet::EMPTY), WeekdaySet::ALL);

  ```

- <span id="weekdayset-symmetric-difference"></span>`const fn symmetric_difference(self, other: Self) -> Self`

  Returns days that are in `self` or `other` but not in both.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).symmetric_difference(WeekdaySet::single(Mon)), WeekdaySet::EMPTY);

  assert_eq!(WeekdaySet::single(Mon).symmetric_difference(WeekdaySet::single(Tue)), WeekdaySet::from_array([Mon, Tue]));

  assert_eq!(

      WeekdaySet::ALL.symmetric_difference(WeekdaySet::single(Mon)),

      WeekdaySet::from_array([Tue, Wed, Thu, Fri, Sat, Sun]),

  );

  assert_eq!(WeekdaySet::ALL.symmetric_difference(WeekdaySet::EMPTY), WeekdaySet::ALL);

  ```

- <span id="weekdayset-difference"></span>`const fn difference(self, other: Self) -> Self`

  Returns days that are in `self` but not in `other`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).difference(WeekdaySet::single(Mon)), WeekdaySet::EMPTY);

  assert_eq!(WeekdaySet::single(Mon).difference(WeekdaySet::single(Tue)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::EMPTY.difference(WeekdaySet::single(Mon)), WeekdaySet::EMPTY);

  ```

- <span id="weekdayset-first"></span>`const fn first(self) -> Option<Weekday>` — [`Weekday`](weekday/index.md#weekday)

  Get the first day in the collection, starting from Monday.

  

  Returns `None` if the collection is empty.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).first(), Some(Mon));

  assert_eq!(WeekdaySet::single(Tue).first(), Some(Tue));

  assert_eq!(WeekdaySet::ALL.first(), Some(Mon));

  assert_eq!(WeekdaySet::EMPTY.first(), None);

  ```

- <span id="weekdayset-last"></span>`fn last(self) -> Option<Weekday>` — [`Weekday`](weekday/index.md#weekday)

  Get the last day in the collection, starting from Sunday.

  

  Returns `None` if the collection is empty.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).last(), Some(Mon));

  assert_eq!(WeekdaySet::single(Sun).last(), Some(Sun));

  assert_eq!(WeekdaySet::from_array([Mon, Tue]).last(), Some(Tue));

  assert_eq!(WeekdaySet::EMPTY.last(), None);

  ```

- <span id="weekdayset-split-at"></span>`const fn split_at(self, weekday: Weekday) -> (Self, Self)` — [`Weekday`](weekday/index.md#weekday)

  Split the collection in two at the given day.

  

  Returns a tuple `(before, after)`. `before` contains all days starting from Monday

  up to but __not__ including `weekday`. `after` contains all days starting from `weekday`

  up to and including Sunday.

- <span id="weekdayset-iter"></span>`const fn iter(self, start: Weekday) -> WeekdaySetIter` — [`Weekday`](weekday/index.md#weekday), [`WeekdaySetIter`](weekday_set/index.md#weekdaysetiter)

  Iterate over the [`Weekday`](weekday/index.md)s in the collection starting from a given day.

  

  Wraps around from Sunday to Monday if necessary.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  let weekdays = WeekdaySet::from_array([Mon, Wed, Fri]);

  let mut iter = weekdays.iter(Wed);

  assert_eq!(iter.next(), Some(Wed));

  assert_eq!(iter.next(), Some(Fri));

  assert_eq!(iter.next(), Some(Mon));

  assert_eq!(iter.next(), None);

  ```

- <span id="weekdayset-contains"></span>`const fn contains(self, day: Weekday) -> bool` — [`Weekday`](weekday/index.md#weekday)

  Returns `true` if the collection contains the given day.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert!(WeekdaySet::single(Mon).contains(Mon));

  assert!(WeekdaySet::from_array([Mon, Tue]).contains(Tue));

  assert!(!WeekdaySet::single(Mon).contains(Tue));

  ```

- <span id="weekdayset-is-empty"></span>`const fn is_empty(self) -> bool`

  Returns `true` if the collection is empty.

  

  # Example

  ```rust

  use chrono::{Weekday, WeekdaySet};

  assert!(WeekdaySet::EMPTY.is_empty());

  assert!(!WeekdaySet::single(Weekday::Mon).is_empty());

  ```

- <span id="weekdayset-len"></span>`const fn len(self) -> u8`

  Returns the number of days in the collection.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).len(), 1);

  assert_eq!(WeekdaySet::from_array([Mon, Wed, Fri]).len(), 3);

  assert_eq!(WeekdaySet::ALL.len(), 7);

  ```

- <span id="weekdayset-const-empty"></span>`const EMPTY: Self`

- <span id="weekdayset-const-all"></span>`const ALL: Self`

#### Trait Implementations

##### `impl Clone for WeekdaySet`

- <span id="weekdayset-clone"></span>`fn clone(&self) -> WeekdaySet` — [`WeekdaySet`](weekday_set/index.md#weekdayset)

##### `impl Copy for WeekdaySet`

##### `impl Debug for WeekdaySet`

- <span id="weekdayset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WeekdaySet`

- <span id="weekdayset-default"></span>`fn default() -> WeekdaySet` — [`WeekdaySet`](weekday_set/index.md#weekdayset)

##### `impl Display for WeekdaySet`

- <span id="weekdayset-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for WeekdaySet`

##### `impl FromIterator for WeekdaySet`

- <span id="weekdayset-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Weekday>>(iter: T) -> Self`

##### `impl Hash for WeekdaySet`

- <span id="weekdayset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for WeekdaySet`

- <span id="weekdayset-ord-cmp"></span>`fn cmp(&self, other: &WeekdaySet) -> cmp::Ordering` — [`WeekdaySet`](weekday_set/index.md#weekdayset)

##### `impl PartialEq for WeekdaySet`

- <span id="weekdayset-partialeq-eq"></span>`fn eq(&self, other: &WeekdaySet) -> bool` — [`WeekdaySet`](weekday_set/index.md#weekdayset)

##### `impl PartialOrd for WeekdaySet`

- <span id="weekdayset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &WeekdaySet) -> option::Option<cmp::Ordering>` — [`WeekdaySet`](weekday_set/index.md#weekdayset)

##### `impl StructuralPartialEq for WeekdaySet`

##### `impl ToString for WeekdaySet`

- <span id="weekdayset-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ParseMonthError`

```rust
struct ParseMonthError {
    _dummy: (),
}
```

An error resulting from reading `<Month>` value with `FromStr`.

#### Trait Implementations

##### `impl Clone for ParseMonthError`

- <span id="parsemontherror-clone"></span>`fn clone(&self) -> ParseMonthError` — [`ParseMonthError`](month/index.md#parsemontherror)

##### `impl Debug for ParseMonthError`

- <span id="parsemontherror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseMonthError`

- <span id="parsemontherror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseMonthError`

##### `impl PartialEq for ParseMonthError`

- <span id="parsemontherror-partialeq-eq"></span>`fn eq(&self, other: &ParseMonthError) -> bool` — [`ParseMonthError`](month/index.md#parsemontherror)

##### `impl StructuralPartialEq for ParseMonthError`

##### `impl ToString for ParseMonthError`

- <span id="parsemontherror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Months`

```rust
struct Months(u32);
```

A duration in calendar months

#### Implementations

- <span id="months-new"></span>`const fn new(num: u32) -> Self`

  Construct a new `Months` from a number of months

- <span id="months-as-u32"></span>`const fn as_u32(&self) -> u32`

  Returns the total number of months in the `Months` instance.

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, rhs: Months) -> <Self as >::Output` — [`Months`](month/index.md#months)

##### `impl Clone for Months`

- <span id="months-clone"></span>`fn clone(&self) -> Months` — [`Months`](month/index.md#months)

##### `impl Copy for Months`

##### `impl Debug for Months`

- <span id="months-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Months`

##### `impl Hash for Months`

- <span id="months-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Months`

- <span id="months-ord-cmp"></span>`fn cmp(&self, other: &Months) -> cmp::Ordering` — [`Months`](month/index.md#months)

##### `impl PartialEq for Months`

- <span id="months-partialeq-eq"></span>`fn eq(&self, other: &Months) -> bool` — [`Months`](month/index.md#months)

##### `impl PartialOrd for Months`

- <span id="months-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Months) -> option::Option<cmp::Ordering>` — [`Months`](month/index.md#months)

##### `impl StructuralPartialEq for Months`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, rhs: Months) -> <Self as >::Output` — [`Months`](month/index.md#months)

### `OutOfRange`

```rust
struct OutOfRange {
    _private: (),
}
```

Out of range error type used in various converting APIs

#### Implementations

- <span id="outofrange-new"></span>`const fn new() -> OutOfRange` — [`OutOfRange`](#outofrange)

#### Trait Implementations

##### `impl Clone for OutOfRange`

- <span id="outofrange-clone"></span>`fn clone(&self) -> OutOfRange` — [`OutOfRange`](#outofrange)

##### `impl Copy for OutOfRange`

##### `impl Debug for OutOfRange`

- <span id="outofrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for OutOfRange`

- <span id="outofrange-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OutOfRange`

##### `impl Hash for OutOfRange`

- <span id="outofrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for OutOfRange`

- <span id="outofrange-partialeq-eq"></span>`fn eq(&self, other: &OutOfRange) -> bool` — [`OutOfRange`](#outofrange)

##### `impl StructuralPartialEq for OutOfRange`

##### `impl ToString for OutOfRange`

- <span id="outofrange-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `SecondsFormat`

```rust
enum SecondsFormat {
    Secs,
    Millis,
    Micros,
    Nanos,
    AutoSi,
}
```

Specific formatting options for seconds. This may be extended in the
future, so exhaustive matching in external code is not recommended.

See the `TimeZone::to_rfc3339_opts` function for usage.

#### Variants

- **`Secs`**

  Format whole seconds only, with no decimal point nor subseconds.

- **`Millis`**

  Use fixed 3 subsecond digits. This corresponds to [Fixed::Nanosecond3].

- **`Micros`**

  Use fixed 6 subsecond digits. This corresponds to [Fixed::Nanosecond6].

- **`Nanos`**

  Use fixed 9 subsecond digits. This corresponds to [Fixed::Nanosecond9].

- **`AutoSi`**

  Automatically select one of `Secs`, `Millis`, `Micros`, or `Nanos` to display all available
  non-zero sub-second digits.  This corresponds to [Fixed::Nanosecond].

#### Trait Implementations

##### `impl Clone for SecondsFormat`

- <span id="secondsformat-clone"></span>`fn clone(&self) -> SecondsFormat` — [`SecondsFormat`](format/formatting/index.md#secondsformat)

##### `impl Copy for SecondsFormat`

##### `impl Debug for SecondsFormat`

- <span id="secondsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SecondsFormat`

##### `impl Hash for SecondsFormat`

- <span id="secondsformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SecondsFormat`

- <span id="secondsformat-partialeq-eq"></span>`fn eq(&self, other: &SecondsFormat) -> bool` — [`SecondsFormat`](format/formatting/index.md#secondsformat)

##### `impl StructuralPartialEq for SecondsFormat`

### `RoundingError`

```rust
enum RoundingError {
    DurationExceedsTimestamp,
    DurationExceedsLimit,
    TimestampExceedsLimit,
}
```

An error from rounding by `TimeDelta`

See: [`DurationRound`](round/index.md)

#### Variants

- **`DurationExceedsTimestamp`**

  Error when the TimeDelta exceeds the TimeDelta from or until the Unix epoch.
  
  Note: this error is not produced anymore.

- **`DurationExceedsLimit`**

  Error when `TimeDelta.num_nanoseconds` exceeds the limit.
  
  ``` rust
  use chrono::{DurationRound, TimeDelta, RoundingError, NaiveDate};
  let dt = NaiveDate::from_ymd_opt(2260, 12, 31)
      .unwrap()
      .and_hms_nano_opt(23, 59, 59, 1_75_500_000)
      .unwrap()
      .and_utc();
  
  assert_eq!(
      dt.duration_round(TimeDelta::try_days(300 * 365).unwrap()),
      Err(RoundingError::DurationExceedsLimit)
  );
  ```

- **`TimestampExceedsLimit`**

  Error when `DateTime.timestamp_nanos` exceeds the limit.
  
  ``` rust
  use chrono::{DurationRound, TimeDelta, RoundingError, TimeZone, Utc};
  let dt = Utc.with_ymd_and_hms(2300, 12, 12, 0, 0, 0).unwrap();
  
  assert_eq!(
      dt.duration_round(TimeDelta::try_days(1).unwrap()),
      Err(RoundingError::TimestampExceedsLimit)
  );
  ```

#### Trait Implementations

##### `impl Clone for RoundingError`

- <span id="roundingerror-clone"></span>`fn clone(&self) -> RoundingError` — [`RoundingError`](round/index.md#roundingerror)

##### `impl Copy for RoundingError`

##### `impl Debug for RoundingError`

- <span id="roundingerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RoundingError`

- <span id="roundingerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RoundingError`

##### `impl PartialEq for RoundingError`

- <span id="roundingerror-partialeq-eq"></span>`fn eq(&self, other: &RoundingError) -> bool` — [`RoundingError`](round/index.md#roundingerror)

##### `impl StructuralPartialEq for RoundingError`

##### `impl ToString for RoundingError`

- <span id="roundingerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Weekday`

```rust
enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}
```

The day of week.

The order of the days of week depends on the context.
(This is why this type does *not* implement `PartialOrd` or `Ord` traits.)
One should prefer `*_from_monday` or `*_from_sunday` methods to get the correct result.

# Example
```rust
use chrono::Weekday;

let monday = "Monday".parse::<Weekday>().unwrap();
assert_eq!(monday, Weekday::Mon);

let sunday = Weekday::try_from(6).unwrap();
assert_eq!(sunday, Weekday::Sun);

assert_eq!(sunday.num_days_from_monday(), 6); // starts counting with Monday = 0
assert_eq!(sunday.number_from_monday(), 7); // starts counting with Monday = 1
assert_eq!(sunday.num_days_from_sunday(), 0); // starts counting with Sunday = 0
assert_eq!(sunday.number_from_sunday(), 1); // starts counting with Sunday = 1

assert_eq!(sunday.succ(), monday);
assert_eq!(sunday.pred(), Weekday::Sat);
```

#### Variants

- **`Mon`**

  Monday.

- **`Tue`**

  Tuesday.

- **`Wed`**

  Wednesday.

- **`Thu`**

  Thursday.

- **`Fri`**

  Friday.

- **`Sat`**

  Saturday.

- **`Sun`**

  Sunday.

#### Implementations

- <span id="weekday-succ"></span>`const fn succ(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

  The next day in the week.

  

  `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.succ()`: | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun` | `Mon`

- <span id="weekday-pred"></span>`const fn pred(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

  The previous day in the week.

  

  `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.pred()`: | `Sun` | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat`

- <span id="weekday-number-from-monday"></span>`const fn number_from_monday(&self) -> u32`

  Returns a day-of-week number starting from Monday = 1. (ISO 8601 weekday number)

  

  `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.number_from_monday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 7

- <span id="weekday-number-from-sunday"></span>`const fn number_from_sunday(&self) -> u32`

  Returns a day-of-week number starting from Sunday = 1.

  

  `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.number_from_sunday()`: | 2     | 3     | 4     | 5     | 6     | 7     | 1

- <span id="weekday-num-days-from-monday"></span>`const fn num_days_from_monday(&self) -> u32`

  Returns a day-of-week number starting from Monday = 0.

  

  `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.num_days_from_monday()`: | 0     | 1     | 2     | 3     | 4     | 5     | 6

  

  # Example

  

  ```rust

  #[cfg(feature = "clock")] {

  use chrono::{Local, Datelike};

  // MTWRFSU is occasionally used as a single-letter abbreviation of the weekdays.

  // Use `num_days_from_monday` to index into the array.

  const MTWRFSU: [char; 7] = ['M', 'T', 'W', 'R', 'F', 'S', 'U'];

  

  let today = Local::now().weekday();

  println!("{}", MTWRFSU[today.num_days_from_monday() as usize]);

  }

  ```

- <span id="weekday-num-days-from-sunday"></span>`const fn num_days_from_sunday(&self) -> u32`

  Returns a day-of-week number starting from Sunday = 0.

  

  `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`

  --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----

  `w.num_days_from_sunday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 0

- <span id="weekday-days-since"></span>`const fn days_since(&self, other: Weekday) -> u32` — [`Weekday`](weekday/index.md#weekday)

  The number of days since the given day.

  

  # Examples

  

  ```rust

  use chrono::Weekday::*;

  assert_eq!(Mon.days_since(Mon), 0);

  assert_eq!(Sun.days_since(Tue), 5);

  assert_eq!(Wed.days_since(Sun), 3);

  ```

#### Trait Implementations

##### `impl Clone for Weekday`

- <span id="weekday-clone"></span>`fn clone(&self) -> Weekday` — [`Weekday`](weekday/index.md#weekday)

##### `impl Copy for Weekday`

##### `impl Debug for Weekday`

- <span id="weekday-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for super::Weekday`

- <span id="superweekday-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Weekday`

##### `impl Display for Weekday`

- <span id="weekday-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Weekday`

##### `impl FromIterator for WeekdaySet`

- <span id="weekdayset-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Weekday>>(iter: T) -> Self`

##### `impl FromPrimitive for Weekday`

- <span id="weekday-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<Weekday>` — [`Weekday`](weekday/index.md#weekday)

- <span id="weekday-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<Weekday>` — [`Weekday`](weekday/index.md#weekday)

##### `impl FromStr for crate::Weekday`

- <span id="crateweekday-fromstr-type-err"></span>`type Err = ParseWeekdayError`

- <span id="crateweekday-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Weekday`

- <span id="weekday-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Weekday`

- <span id="weekday-partialeq-eq"></span>`fn eq(&self, other: &Weekday) -> bool` — [`Weekday`](weekday/index.md#weekday)

##### `impl Serialize for super::Weekday`

- <span id="superweekday-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Weekday`

##### `impl ToString for Weekday`

- <span id="weekday-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Month`

```rust
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
```

The month of the year.

This enum is just a convenience implementation.
The month in dates created by DateLike objects does not return this enum.

It is possible to convert from a date to a month independently
```rust
use chrono::prelude::*;
let date = Utc.with_ymd_and_hms(2019, 10, 28, 9, 10, 11).unwrap();
// `2019-10-28T09:10:11Z`
let month = Month::try_from(u8::try_from(date.month()).unwrap()).ok();
assert_eq!(month, Some(Month::October))
```
Or from a Month to an integer usable by dates
```rust
use chrono::prelude::*;
let month = Month::January;
let dt = Utc.with_ymd_and_hms(2019, month.number_from_month(), 28, 9, 10, 11).unwrap();
assert_eq!((dt.year(), dt.month(), dt.day()), (2019, 1, 28));
```
Allows mapping from and to month, from 1-January to 12-December.
Can be Serialized/Deserialized with serde

#### Variants

- **`January`**

  January

- **`February`**

  February

- **`March`**

  March

- **`April`**

  April

- **`May`**

  May

- **`June`**

  June

- **`July`**

  July

- **`August`**

  August

- **`September`**

  September

- **`October`**

  October

- **`November`**

  November

- **`December`**

  December

#### Implementations

- <span id="month-succ"></span>`const fn succ(&self) -> Month` — [`Month`](month/index.md#month)

  The next month.

  

  `m`:        | `January`  | `February` | `...` | `December`

  ----------- | ---------  | ---------- | --- | ---------

  `m.succ()`: | `February` | `March`    | `...` | `January`

- <span id="month-pred"></span>`const fn pred(&self) -> Month` — [`Month`](month/index.md#month)

  The previous month.

  

  `m`:        | `January`  | `February` | `...` | `December`

  ----------- | ---------  | ---------- | --- | ---------

  `m.pred()`: | `December` | `January`  | `...` | `November`

- <span id="month-number-from-month"></span>`const fn number_from_month(&self) -> u32`

  Returns a month-of-year number starting from January = 1.

  

  `m`:                     | `January` | `February` | `...` | `December`

  -------------------------| --------- | ---------- | --- | -----

  `m.number_from_month()`: | 1         | 2          | `...` | 12

- <span id="month-name"></span>`const fn name(&self) -> &'static str`

  Get the name of the month

  

  ```rust

  use chrono::Month;

  

  assert_eq!(Month::January.name(), "January")

  ```

- <span id="month-num-days"></span>`fn num_days(&self, year: i32) -> Option<u8>`

  Get the length in days of the month

  

  Yields `None` if `year` is out of range for `NaiveDate`.

#### Trait Implementations

##### `impl Clone for Month`

- <span id="month-clone"></span>`fn clone(&self) -> Month` — [`Month`](month/index.md#month)

##### `impl Copy for Month`

##### `impl Debug for Month`

- <span id="month-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for super::Month`

- <span id="supermonth-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Month`

##### `impl Eq for Month`

##### `impl FromPrimitive for Month`

- <span id="month-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<Month>` — [`Month`](month/index.md#month)

  Returns an `Option<Month>` from a i64, assuming a 1-index, January = 1.

  

  `Month::from_i64(n: i64)`: | `1`                  | `2`                   | ... | `12`

  ---------------------------| -------------------- | --------------------- | ... | -----

  ``:                        | Some(Month::January) | Some(Month::February) | ... | Some(Month::December)

- <span id="month-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<Month>` — [`Month`](month/index.md#month)

- <span id="month-fromprimitive-from-u32"></span>`fn from_u32(n: u32) -> Option<Month>` — [`Month`](month/index.md#month)

##### `impl FromStr for crate::Month`

- <span id="cratemonth-fromstr-type-err"></span>`type Err = ParseMonthError`

- <span id="cratemonth-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Month`

- <span id="month-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Month`

- <span id="month-ord-cmp"></span>`fn cmp(&self, other: &Month) -> cmp::Ordering` — [`Month`](month/index.md#month)

##### `impl PartialEq for Month`

- <span id="month-partialeq-eq"></span>`fn eq(&self, other: &Month) -> bool` — [`Month`](month/index.md#month)

##### `impl PartialOrd for Month`

- <span id="month-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Month) -> option::Option<cmp::Ordering>` — [`Month`](month/index.md#month)

##### `impl Serialize for super::Month`

- <span id="supermonth-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Month`

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

- [`FixedOffset`](offset/fixed/index.md#fixedoffset)
- [`Utc`](offset/utc/index.md#utc)

### `TimeZone`

```rust
trait TimeZone: Sized + Clone { ... }
```

The time zone.

The methods here are the primary constructors for the [`DateTime`](datetime/index.md) type.

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

- [`FixedOffset`](offset/fixed/index.md#fixedoffset)
- [`Utc`](offset/utc/index.md#utc)

### `DurationRound`

```rust
trait DurationRound: Sized { ... }
```

Extension trait for rounding or truncating a DateTime by a TimeDelta.

# Limitations
Both rounding and truncating are done via `TimeDelta::num_nanoseconds` and
`DateTime::timestamp_nanos_opt`. This means that they will fail if either the
`TimeDelta` or the `DateTime` are too big to represented as nanoseconds. They
will also fail if the `TimeDelta` is bigger than the timestamp, negative or zero.

#### Associated Types

- `type Err: 2`

#### Required Methods

- `fn duration_round(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`

  Return a copy rounded by TimeDelta.

- `fn duration_trunc(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`

  Return a copy truncated by TimeDelta.

- `fn duration_round_up(self, duration: TimeDelta) -> Result<Self, <Self as >::Err>`

  Return a copy rounded **up** by TimeDelta.

#### Implementors

- [`DateTime`](datetime/index.md#datetime)
- [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)

### `SubsecRound`

```rust
trait SubsecRound { ... }
```

Extension trait for subsecond rounding or truncation to a maximum number
of digits. Rounding can be used to decrease the error variance when
serializing/persisting to lower precision. Truncation is the default
behavior in Chrono display formatting.  Either can be used to guarantee
equality (e.g. for testing) when round-tripping through a lower precision
format.

#### Required Methods

- `fn round_subsecs(self, digits: u16) -> Self`

  Return a copy rounded to the specified number of subsecond digits. With

- `fn trunc_subsecs(self, digits: u16) -> Self`

  Return a copy truncated to the specified number of subsecond

#### Implementors

- `T`

### `Datelike`

```rust
trait Datelike: Sized { ... }
```

The common set of methods for date component.

Methods such as `year`, [`month`](month/index.md), `day` and [`weekday`](weekday/index.md) can be used to get basic
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

For more complex changes to a date, it is best to use the methods on [`NaiveDate`](naive/date/index.md) to create a
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

- [`DateTime`](datetime/index.md#datetime)
- [`Date`](date/index.md#date)
- [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)
- [`NaiveDate`](naive/date/index.md#naivedate)

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

- [`DateTime`](datetime/index.md#datetime)
- [`NaiveDateTime`](naive/datetime/index.md#naivedatetime)
- [`NaiveTime`](naive/time/index.md#naivetime)

## Functions

### `expect`

```rust
const fn expect<T: Copy>(opt: Option<T>, msg: &str) -> T
```

Workaround because `.expect()` is not (yet) available in const context.

## Type Aliases

### `Duration`

```rust
type Duration = TimeDelta;
```

Alias of [`TimeDelta`](time_delta/index.md).

### `ParseResult<T>`

```rust
type ParseResult<T> = Result<T, ParseError>;
```

Same as `Result<T, ParseError>`.

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

In wasm, when using `Local`, only the [`LocalResult::Single`](#localresultsingle) variant is returned.
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

The type of `T` is usually a [`DateTime`](datetime/index.md) but may also be only an offset.

## Constants

### `MAX_DATE`
```rust
const MAX_DATE: Date<crate::offset::Utc>;
```

The maximum possible `Date`.

### `MIN_DATE`
```rust
const MIN_DATE: Date<crate::offset::Utc>;
```

The minimum possible `Date`.

### `MAX_DATETIME`
```rust
const MAX_DATETIME: DateTime<crate::offset::Utc>;
```

The maximum possible `DateTime<Utc>`.

### `MIN_DATETIME`
```rust
const MIN_DATETIME: DateTime<crate::offset::Utc>;
```

The minimum possible `DateTime<Utc>`.

