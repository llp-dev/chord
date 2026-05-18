**ttf_parser > tables > cmap > format6**

# Module: tables::cmap::format6

## Contents

**Structs**

- [`Subtable6`](#subtable6) - A [format 6](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-6-trimmed-table-mapping)

---

## ttf_parser::tables::cmap::format6::Subtable6

*Struct*

A [format 6](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-6-trimmed-table-mapping)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `first_code_point: u16` - First character code of subrange.
- `glyphs: crate::parser::LazyArray16<'a, crate::GlyphId>` - Array of glyph indexes for character codes in the range.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Returns a glyph index for a code point.
- `fn codepoints<impl FnMut(u32)>(self: &Self, f: impl Trait)` - Calls `f` for each codepoint defined in this table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable6<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



