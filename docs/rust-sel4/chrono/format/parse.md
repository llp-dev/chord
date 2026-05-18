**chrono > format > parse**

# Module: format::parse

## Contents

**Functions**

- [`parse`](#parse) - Tries to parse given string into `parsed` with given formatting items.
- [`parse_and_remainder`](#parse_and_remainder) - Tries to parse given string into `parsed` with given formatting items.

---

## chrono::format::parse::parse

*Function*

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

```rust
fn parse<'a, I, B>(parsed: & mut super::Parsed, s: &str, items: I) -> super::ParseResult<()>
```



## chrono::format::parse::parse_and_remainder

*Function*

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

```rust
fn parse_and_remainder<'a, 'b, I, B>(parsed: & mut super::Parsed, s: &'b str, items: I) -> super::ParseResult<&'b str>
```



