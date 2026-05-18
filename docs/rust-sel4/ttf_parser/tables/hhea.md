**ttf_parser > tables > hhea**

# Module: tables::hhea

## Contents

**Structs**

- [`Table`](#table) - A [Horizontal Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea).

---

## ttf_parser::tables::hhea::Table

*Struct*

A [Horizontal Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea).

**Fields:**
- `ascender: i16` - Face ascender.
- `descender: i16` - Face descender.
- `line_gap: i16` - Face line gap.
- `number_of_metrics: u16` - Number of metrics in the `hmtx` table.

**Methods:**

- `fn parse(data: &[u8]) -> Option<Self>` - Parses a table from raw data.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



