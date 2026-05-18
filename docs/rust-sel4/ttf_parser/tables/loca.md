**ttf_parser > tables > loca**

# Module: tables::loca

## Contents

**Enums**

- [`Table`](#table) - An [Index to Location Table](https://docs.microsoft.com/en-us/typography/opentype/spec/loca).

---

## ttf_parser::tables::loca::Table

*Enum*

An [Index to Location Table](https://docs.microsoft.com/en-us/typography/opentype/spec/loca).

**Generic Parameters:**
- 'a

**Variants:**
- `Short(crate::parser::LazyArray16<'a, u16>)` - Short offsets.
- `Long(crate::parser::LazyArray16<'a, u32>)` - Long offsets.

**Methods:**

- `fn parse(number_of_glyphs: NonZeroU16, format: IndexToLocationFormat, data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn len(self: &Self) -> u16` - Returns the number of offsets.
- `fn is_empty(self: &Self) -> bool` - Checks if there are any offsets.
- `fn glyph_range(self: &Self, glyph_id: GlyphId) -> Option<Range<usize>>` - Returns glyph's range in the `glyf` table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



