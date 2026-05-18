*[chrono](../../index.md) / [format](../index.md) / [scan](index.md)*

---

# Module `scan`

Various scanning routines for the parser.

## Contents

- [Enums](#enums)
  - [`CommentState`](#commentstate)
- [Functions](#functions)
  - [`number`](#number)
  - [`nanosecond`](#nanosecond)
  - [`nanosecond_fixed`](#nanosecond-fixed)
  - [`short_month0`](#short-month0)
  - [`short_weekday`](#short-weekday)
  - [`short_or_long_month0`](#short-or-long-month0)
  - [`short_or_long_weekday`](#short-or-long-weekday)
  - [`char`](#char)
  - [`space`](#space)
  - [`colon_or_space`](#colon-or-space)
  - [`timezone_offset`](#timezone-offset)
  - [`timezone_offset_2822`](#timezone-offset-2822)
  - [`comment_2822`](#comment-2822)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CommentState`](#commentstate) | enum |  |
| [`number`](#number) | fn | Tries to parse the non-negative number from `min` to `max` digits. |
| [`nanosecond`](#nanosecond) | fn | Tries to consume at least one digits as a fractional second. |
| [`nanosecond_fixed`](#nanosecond-fixed) | fn | Tries to consume a fixed number of digits as a fractional second. |
| [`short_month0`](#short-month0) | fn | Tries to parse the month index (0 through 11) with the first three ASCII letters. |
| [`short_weekday`](#short-weekday) | fn | Tries to parse the weekday with the first three ASCII letters. |
| [`short_or_long_month0`](#short-or-long-month0) | fn | Tries to parse the month index (0 through 11) with short or long month names. |
| [`short_or_long_weekday`](#short-or-long-weekday) | fn | Tries to parse the weekday with short or long weekday names. |
| [`char`](#char) | fn | Tries to consume exactly one given character. |
| [`space`](#space) | fn | Tries to consume one or more whitespace. |
| [`colon_or_space`](#colon-or-space) | fn | Consumes any number (including zero) of colon or spaces. |
| [`timezone_offset`](#timezone-offset) | fn | Parse a timezone from `s` and return the offset in seconds. |
| [`timezone_offset_2822`](#timezone-offset-2822) | fn | Same as `timezone_offset` but also allows for RFC 2822 legacy timezones. |
| [`comment_2822`](#comment-2822) | fn | Tries to consume an RFC2822 comment including preceding ` `. |

## Enums

### `CommentState`

```rust
enum CommentState {
    Start,
    Next(usize),
    Escape(usize),
}
```

## Functions

### `number`

```rust
fn number(s: &str, min: usize, max: usize) -> super::ParseResult<(&str, i64)>
```

Tries to parse the non-negative number from `min` to `max` digits.

The absence of digits at all is an unconditional error.
More than `max` digits are consumed up to the first `max` digits.
Any number that does not fit in `i64` is an error.

### `nanosecond`

```rust
fn nanosecond(s: &str) -> super::ParseResult<(&str, u32)>
```

Tries to consume at least one digits as a fractional second.
Returns the number of whole nanoseconds (0--999,999,999).

### `nanosecond_fixed`

```rust
fn nanosecond_fixed(s: &str, digits: usize) -> super::ParseResult<(&str, i64)>
```

Tries to consume a fixed number of digits as a fractional second.
Returns the number of whole nanoseconds (0--999,999,999).

### `short_month0`

```rust
fn short_month0(s: &str) -> super::ParseResult<(&str, u8)>
```

Tries to parse the month index (0 through 11) with the first three ASCII letters.

### `short_weekday`

```rust
fn short_weekday(s: &str) -> super::ParseResult<(&str, crate::Weekday)>
```

Tries to parse the weekday with the first three ASCII letters.

### `short_or_long_month0`

```rust
fn short_or_long_month0(s: &str) -> super::ParseResult<(&str, u8)>
```

Tries to parse the month index (0 through 11) with short or long month names.
It prefers long month names to short month names when both are possible.

### `short_or_long_weekday`

```rust
fn short_or_long_weekday(s: &str) -> super::ParseResult<(&str, crate::Weekday)>
```

Tries to parse the weekday with short or long weekday names.
It prefers long weekday names to short weekday names when both are possible.

### `char`

```rust
fn char(s: &str, c1: u8) -> super::ParseResult<&str>
```

Tries to consume exactly one given character.

### `space`

```rust
fn space(s: &str) -> super::ParseResult<&str>
```

Tries to consume one or more whitespace.

### `colon_or_space`

```rust
fn colon_or_space(s: &str) -> super::ParseResult<&str>
```

Consumes any number (including zero) of colon or spaces.

### `timezone_offset`

```rust
fn timezone_offset<F>(s: &str, consume_colon: F, allow_zulu: bool, allow_missing_minutes: bool, allow_tz_minus_sign: bool) -> super::ParseResult<(&str, i32)>
where
    F: FnMut(&str) -> super::ParseResult<&str>
```

Parse a timezone from `s` and return the offset in seconds.

The `consume_colon` function is used to parse a mandatory or optional `:`
separator between hours offset and minutes offset.

The `allow_missing_minutes` flag allows the timezone minutes offset to be
missing from `s`.

The `allow_tz_minus_sign` flag allows the timezone offset negative character
to also be `−` MINUS SIGN (U+2212) in addition to the typical
ASCII-compatible `-` HYPHEN-MINUS (U+2D).
This is part of [RFC 3339 & ISO 8601].


### `timezone_offset_2822`

```rust
fn timezone_offset_2822(s: &str) -> super::ParseResult<(&str, i32)>
```

Same as `timezone_offset` but also allows for RFC 2822 legacy timezones.
May return `None` which indicates an insufficient offset data (i.e. `-0000`).
See [RFC 2822 Section 4.3].


### `comment_2822`

```rust
fn comment_2822(s: &str) -> super::ParseResult<(&str, ())>
```

Tries to consume an RFC2822 comment including preceding ` `.

Returns the remaining string after the closing parenthesis.

