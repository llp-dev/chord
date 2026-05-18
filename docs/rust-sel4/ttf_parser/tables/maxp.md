**ttf_parser > tables > maxp**

# Module: tables::maxp

## Contents

**Structs**

- [`Table`](#table) - A [Maximum Profile Table](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp).

---

## ttf_parser::tables::maxp::Table

*Struct*

A [Maximum Profile Table](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp).

**Fields:**
- `number_of_glyphs: core::num::NonZeroU16` - The total number of glyphs in the face.

**Methods:**

- `fn parse(data: &[u8]) -> Option<Self>` - Parses a table from raw data.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



