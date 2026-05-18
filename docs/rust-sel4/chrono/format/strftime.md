**chrono > format > strftime**

# Module: format::strftime

## Contents

**Structs**

- [`StrftimeItems`](#strftimeitems) - Parsing iterator for `strftime`-like format strings.

---

## chrono::format::strftime::StrftimeItems

*Struct*

Parsing iterator for `strftime`-like format strings.

See the [`format::strftime` module](crate::format::strftime) for supported formatting
specifiers.

`StrftimeItems` is used in combination with more low-level methods such as [`format::parse()`]
or [`format_with_items`].

If formatting or parsing date and time values is not performance-critical, the methods
[`parse_from_str`] and [`format`] on types such as [`DateTime`](crate::DateTime) are easier to
use.

[`format`]: crate::DateTime::format
[`format_with_items`]: crate::DateTime::format
[`parse_from_str`]: crate::DateTime::parse_from_str
[`DateTime`]: crate::DateTime
[`format::parse()`]: crate::format::parse()

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(s: &'a str) -> StrftimeItems<'a>` - Creates a new parsing iterator from a `strftime`-like format string.
- `fn new_lenient(s: &'a str) -> StrftimeItems<'a>` - The same as [`StrftimeItems::new`], but returns [`Item::Literal`] instead of [`Item::Error`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<Item<'a>>`
- **Clone**
  - `fn clone(self: &Self) -> StrftimeItems<'a>`



