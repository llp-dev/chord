*[chrono](../../index.md) / [format](../index.md) / [parse](index.md)*

---

# Module `parse`

Date and time parsing routines.

## Contents

- [Functions](#functions)
  - [`set_weekday_with_num_days_from_sunday`](#set-weekday-with-num-days-from-sunday)
  - [`set_weekday_with_number_from_monday`](#set-weekday-with-number-from-monday)
  - [`parse_rfc2822`](#parse-rfc2822)
  - [`parse_rfc3339`](#parse-rfc3339)
  - [`digit`](#digit)
  - [`parse`](#parse)
  - [`parse_and_remainder`](#parse-and-remainder)
  - [`parse_internal`](#parse-internal)
  - [`parse_rfc3339_relaxed`](#parse-rfc3339-relaxed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`set_weekday_with_num_days_from_sunday`](#set-weekday-with-num-days-from-sunday) | fn |  |
| [`set_weekday_with_number_from_monday`](#set-weekday-with-number-from-monday) | fn |  |
| [`parse_rfc2822`](#parse-rfc2822) | fn |  |
| [`parse_rfc3339`](#parse-rfc3339) | fn |  |
| [`digit`](#digit) | fn |  |
| [`parse`](#parse) | fn | Tries to parse given string into `parsed` with given formatting items. |
| [`parse_and_remainder`](#parse-and-remainder) | fn | Tries to parse given string into `parsed` with given formatting items. |
| [`parse_internal`](#parse-internal) | fn |  |
| [`parse_rfc3339_relaxed`](#parse-rfc3339-relaxed) | fn | Accepts a relaxed form of RFC3339. |

## Functions

### `set_weekday_with_num_days_from_sunday`

```rust
fn set_weekday_with_num_days_from_sunday(p: &mut super::Parsed, v: i64) -> super::ParseResult<()>
```

### `set_weekday_with_number_from_monday`

```rust
fn set_weekday_with_number_from_monday(p: &mut super::Parsed, v: i64) -> super::ParseResult<()>
```

### `parse_rfc2822`

```rust
fn parse_rfc2822<'a>(parsed: &mut super::Parsed, s: &'a str) -> super::ParseResult<(&'a str, ())>
```

### `parse_rfc3339`

```rust
fn parse_rfc3339(s: &str) -> super::ParseResult<crate::DateTime<crate::FixedOffset>>
```

### `digit`

```rust
fn digit(bytes: &[u8; 19], index: usize) -> super::ParseResult<u8>
```

### `parse`

```rust
fn parse<'a, I, B>(parsed: &mut super::Parsed, s: &str, items: I) -> super::ParseResult<()>
where
    I: Iterator<Item = B>,
    B: Borrow<super::Item<'a>>
```

Tries to parse given string into `parsed` with given formatting items.
Returns `Ok` when the entire string has been parsed (otherwise `parsed` should not be used).
There should be no trailing string after parsing;
use a stray [`Item::Space`](./enum.Item.html#variant.Space) to trim whitespaces.

This particular date and time parser is:

- Greedy. It will consume the longest possible prefix.
  For example, `April` is always consumed entirely when the long month name is requested;
  it equally accepts `Apr`, but prefers the longer prefix in this case.

- Padding-agnostic (for numeric items).
  The [`Pad`](./enum.Pad.html) field is completely ignored,
  so one can prepend any number of whitespace then any number of zeroes before numbers.

- (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`.

### `parse_and_remainder`

```rust
fn parse_and_remainder<'a, 'b, I, B>(parsed: &mut super::Parsed, s: &'b str, items: I) -> super::ParseResult<&'b str>
where
    I: Iterator<Item = B>,
    B: Borrow<super::Item<'a>>
```

Tries to parse given string into `parsed` with given formatting items.
Returns `Ok` with a slice of the unparsed remainder.

This particular date and time parser is:

- Greedy. It will consume the longest possible prefix.
  For example, `April` is always consumed entirely when the long month name is requested;
  it equally accepts `Apr`, but prefers the longer prefix in this case.

- Padding-agnostic (for numeric items).
  The [`Pad`](./enum.Pad.html) field is completely ignored,
  so one can prepend any number of zeroes before numbers.

- (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`.

### `parse_internal`

```rust
fn parse_internal<'a, 'b, I, B>(parsed: &mut super::Parsed, s: &'b str, items: I) -> Result<&'b str, super::ParseError>
where
    I: Iterator<Item = B>,
    B: Borrow<super::Item<'a>>
```

### `parse_rfc3339_relaxed`

```rust
fn parse_rfc3339_relaxed<'a>(parsed: &mut super::Parsed, s: &'a str) -> super::ParseResult<(&'a str, ())>
```

Accepts a relaxed form of RFC3339.

Differences with RFC3339:
- Values don't require padding to two digits.
- Years outside the range 0...=9999 are accepted, but they must include a sign.
- `UTC` is accepted as a valid timezone name/offset (for compatibility with the debug format of
  `DateTime<Utc>`.
- There can be spaces between any of the components.
- The colon in the offset may be missing.

