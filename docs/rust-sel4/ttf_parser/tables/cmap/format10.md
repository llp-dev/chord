**ttf_parser > tables > cmap > format10**

# Module: tables::cmap::format10

## Contents

**Structs**

- [`Subtable10`](#subtable10) - A [format 10](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-10-trimmed-array)

---

## ttf_parser::tables::cmap::format10::Subtable10

*Struct*

A [format 10](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-10-trimmed-array)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `first_code_point: u32` - First character code covered.
- `glyphs: crate::parser::LazyArray32<'a, crate::GlyphId>` - Array of glyph indices for the character codes covered.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Returns a glyph index for a code point.
- `fn codepoints<impl FnMut(u32)>(self: &Self, f: impl Trait)` - Calls `f` for each codepoint defined in this table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable10<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



