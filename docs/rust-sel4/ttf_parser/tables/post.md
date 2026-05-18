**ttf_parser > tables > post**

# Module: tables::post

## Contents

**Structs**

- [`Names`](#names) - An iterator over glyph names.
- [`Table`](#table) - A [PostScript Table](https://docs.microsoft.com/en-us/typography/opentype/spec/post).

**Constants**

- [`IS_FIXED_PITCH_OFFSET`](#is_fixed_pitch_offset)
- [`ITALIC_ANGLE_OFFSET`](#italic_angle_offset)
- [`UNDERLINE_POSITION_OFFSET`](#underline_position_offset)
- [`UNDERLINE_THICKNESS_OFFSET`](#underline_thickness_offset)

---

## ttf_parser::tables::post::IS_FIXED_PITCH_OFFSET

*Constant*: `usize`



## ttf_parser::tables::post::ITALIC_ANGLE_OFFSET

*Constant*: `usize`



## ttf_parser::tables::post::Names

*Struct*

An iterator over glyph names.

The `post` table doesn't provide the glyph names count,
so we have to simply iterate over all of them to find it out.

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `offset: usize`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Names<'a>`
- **Default**
  - `fn default() -> Names<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## ttf_parser::tables::post::Table

*Struct*

A [PostScript Table](https://docs.microsoft.com/en-us/typography/opentype/spec/post).

**Generic Parameters:**
- 'a

**Fields:**
- `italic_angle: f32` - Italic angle in counter-clockwise degrees from the vertical.
- `underline_metrics: crate::LineMetrics` - Underline metrics.
- `is_monospaced: bool` - Flag that indicates that the font is monospaced.
- `glyph_indexes: crate::parser::LazyArray16<'a, u16>`
- `names_data: &'a [u8]`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn names(self: &Self) -> Names<'a>` - Returns an iterator over glyph names.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::post::UNDERLINE_POSITION_OFFSET

*Constant*: `usize`



## ttf_parser::tables::post::UNDERLINE_THICKNESS_OFFSET

*Constant*: `usize`



