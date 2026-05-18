**ttf_parser > tables > vhea**

# Module: tables::vhea

## Contents

**Structs**

- [`Table`](#table) - A [Vertical Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vhea).

---

## ttf_parser::tables::vhea::Table

*Struct*

A [Vertical Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vhea).

**Fields:**
- `ascender: i16` - Face ascender.
- `descender: i16` - Face descender.
- `line_gap: i16` - Face line gap.
- `number_of_metrics: u16` - Number of metrics in the `vmtx` table.

**Methods:**

- `fn parse(data: &[u8]) -> Option<Self>` - Parses a table from raw data.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table`
- **Default**
  - `fn default() -> Table`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



