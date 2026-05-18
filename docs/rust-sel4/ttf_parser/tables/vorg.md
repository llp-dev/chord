**ttf_parser > tables > vorg**

# Module: tables::vorg

## Contents

**Structs**

- [`Table`](#table) - A [Vertical Origin Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg).
- [`VerticalOriginMetrics`](#verticaloriginmetrics) - Vertical origin metrics for the

---

## ttf_parser::tables::vorg::Table

*Struct*

A [Vertical Origin Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg).

**Generic Parameters:**
- 'a

**Fields:**
- `default_y: i16` - Default origin.
- `metrics: crate::parser::LazyArray16<'a, VerticalOriginMetrics>` - A list of metrics for each glyph.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn glyph_y_origin(self: &Self, glyph_id: GlyphId) -> i16` - Returns glyph's Y origin.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::vorg::VerticalOriginMetrics

*Struct*

Vertical origin metrics for the
[Vertical Origin Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg).

**Fields:**
- `glyph_id: crate::GlyphId` - Glyph ID.
- `y: i16` - Y coordinate, in the font's design coordinate system, of the vertical origin.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> VerticalOriginMetrics`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



