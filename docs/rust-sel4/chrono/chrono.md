**chrono**

# Module: chrono

## Contents

**Modules**

- [`format`](#format) - Formatting (and parsing) utilities for date and time.
- [`naive`](#naive) - Date and time types unconcerned with timezones.
- [`offset`](#offset) - The time zone, which calculates offsets from the local time to UTC.
- [`prelude`](#prelude) - A convenience module appropriate for glob imports (`use chrono::prelude::*;`).
- [`round`](#round) - Functionality for rounding or truncating a `DateTime` by a `TimeDelta`.
- [`serde`](#serde) - Serialization/Deserialization with serde

**Structs**

- [`OutOfRange`](#outofrange) - Out of range error type used in various converting APIs

**Type Aliases**

- [`Duration`](#duration) - Alias of [`TimeDelta`].

---

## chrono::Duration

*Type Alias*: `TimeDelta`

Alias of [`TimeDelta`].



## chrono::OutOfRange

*Struct*

Out of range error type used in various converting APIs

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &OutOfRange) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OutOfRange`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## Module: format

Formatting (and parsing) utilities for date and time.

This module provides the common types and routines to implement,
for example, [`DateTime::format`](../struct.DateTime.html#method.format) or
[`DateTime::parse_from_str`](../struct.DateTime.html#method.parse_from_str) methods.
For most cases you should use these high-level interfaces.

Internally the formatting and parsing shares the same abstract **formatting items**,
which are just an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) of
the [`Item`](./enum.Item.html) type.
They are generated from more readable **format strings**;
currently Chrono supports a built-in syntax closely resembling
C's `strftime` format. The available options can be found [here](./strftime/index.html).

# Example
```
# #[cfg(feature = "alloc")] {
use chrono::{NaiveDateTime, TimeZone, Utc};

let date_time = Utc.with_ymd_and_hms(2020, 11, 10, 0, 1, 32).unwrap();

let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
assert_eq!(formatted, "2020-11-10 00:01:32");

let parsed = NaiveDateTime::parse_from_str(&formatted, "%Y-%m-%d %H:%M:%S")?.and_utc();
assert_eq!(parsed, date_time);
# }
# Ok::<(), chrono::ParseError>(())
```



## Module: naive

Date and time types unconcerned with timezones.

They are primarily building blocks for other types
(e.g. [`TimeZone`](../offset/trait.TimeZone.html)),
but can be also used for the simpler date and time handling.



## Module: offset

The time zone, which calculates offsets from the local time to UTC.

There are four operations provided by the `TimeZone` trait:

1. Converting the local `NaiveDateTime` to `DateTime<Tz>`
2. Converting the UTC `NaiveDateTime` to `DateTime<Tz>`
3. Converting `DateTime<Tz>` to the local `NaiveDateTime`
4. Constructing `DateTime<Tz>` objects from various offsets

1 is used for constructors. 2 is used for the `with_timezone` method of date and time types.
3 is used for other methods, e.g. `year()` or `format()`, and provided by an associated type
which implements `Offset` (which then passed to `TimeZone` for actual implementations).
Technically speaking `TimeZone` has a total knowledge about given timescale,
but `Offset` is used as a cache to avoid the repeated conversion
and provides implementations for 1 and 3.
An `TimeZone` instance can be reconstructed from the corresponding `Offset` instance.



## Module: prelude

A convenience module appropriate for glob imports (`use chrono::prelude::*;`).



## Module: round

Functionality for rounding or truncating a `DateTime` by a `TimeDelta`.



## Module: serde

Serialization/Deserialization with serde

The [`DateTime`] type has default implementations for (de)serializing to/from the [RFC 3339]
format. This module provides alternatives for serializing to timestamps.

The alternatives are for use with serde's [`with` annotation] combined with the module name.
Alternatively the individual `serialize` and `deserialize` functions in each module can be used
with serde's [`serialize_with`] and [`deserialize_with`] annotations.

*Available on crate feature 'serde' only.*

[RFC 3339]: https://tools.ietf.org/html/rfc3339
[`with` annotation]: https://serde.rs/field-attrs.html#with
[`serialize_with`]: https://serde.rs/field-attrs.html#serialize_with
[`deserialize_with`]: https://serde.rs/field-attrs.html#deserialize_with



