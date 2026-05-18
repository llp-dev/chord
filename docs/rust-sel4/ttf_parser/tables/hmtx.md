**ttf_parser > tables > hmtx**

# Module: tables::hmtx

## Contents

**Structs**

- [`Metrics`](#metrics) - Horizontal/Vertical Metrics.
- [`Table`](#table) - A [Horizontal/Vertical Metrics Table](

---

## ttf_parser::tables::hmtx::Metrics

*Struct*

Horizontal/Vertical Metrics.

**Fields:**
- `advance: u16` - Width/Height advance for `hmtx`/`vmtx`.
- `side_bearing: i16` - Left/Top side bearing for `hmtx`/`vmtx`.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Metrics`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::hmtx::Table

*Struct*

A [Horizontal/Vertical Metrics Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx).

`hmtx` and `vmtx` tables has the same structure, so we're reusing the same struct for both.

**Generic Parameters:**
- 'a

**Fields:**
- `metrics: crate::parser::LazyArray16<'a, Metrics>` - A list of metrics indexed by glyph ID.
- `bearings: crate::parser::LazyArray16<'a, i16>` - Side bearings for glyph IDs greater than or equal to the number of `metrics` values.
- `number_of_metrics: u16` - Sum of long metrics + bearings.

**Methods:**

- `fn parse(number_of_metrics: u16, number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn advance(self: &Self, glyph_id: GlyphId) -> Option<u16>` - Returns advance for a glyph.
- `fn side_bearing(self: &Self, glyph_id: GlyphId) -> Option<i16>` - Returns side bearing for a glyph.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



