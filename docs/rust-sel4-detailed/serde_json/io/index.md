*[serde_json](../index.md) / [io](index.md)*

---

# Module `io`

A tiny, `no_std`-friendly facade around `std::io`.
Reexports types from `std` when available; otherwise reimplements and
provides some of the core logic.

The main reason that `std::io` hasn't found itself reexported as part of
the `core` crate is the `std::io::{Read, Write}` traits' reliance on
`std::io::Error`, which may contain internally a heap-allocated `Box<Error>`
and/or now relying on OS-specific `std::backtrace::Backtrace`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct |  |
| [`Bytes`](#bytes) | fn |  |

## Structs

### `Error<'ctx>`

```rust
struct Error<'ctx> {
    lines: &'ctx Lines,
    seq_idx: usize,
    row_idx: usize,
    probe_high: u64,
}
```

*Re-exported from `addr2line`*

#### Trait Implementations

##### `impl IntoIterator for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="linelocationrangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="linelocationrangeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-iterator-type-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="linelocationrangeiter-iterator-next"></span>`fn next(&mut self) -> Option<(u64, u64, Location<'ctx>)>` — [`StrRead`](../de/index.md#strread), [`IoRead`](../read/index.md#ioread)

##### `impl IteratorExt for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-iteratorext-transpose-into-fallible"></span>`fn transpose_into_fallible<T, E>(self) -> Convert<I>`

  Convert an iterator of `Result`s into `FallibleIterator` by transposition

- <span id="linelocationrangeiter-iteratorext-into-fallible"></span>`fn into_fallible<T>(self) -> IntoFallible<I>`

  Convert an iterator of anything into `FallibleIterator` by wrapping

  into `Result<T, Infallible>` where `Infallible` is an error that can never actually

  happen.

## Functions

### `Bytes`

```rust
fn Bytes(s: &str, fmt: &str) -> ParseResult<NaiveTime>
```

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

