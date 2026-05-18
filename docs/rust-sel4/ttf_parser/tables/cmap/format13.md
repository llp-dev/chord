**ttf_parser > tables > cmap > format13**

# Module: tables::cmap::format13

## Contents

**Structs**

- [`Subtable13`](#subtable13) - A [format 13](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-13-segmented-coverage)

---

## ttf_parser::tables::cmap::format13::Subtable13

*Struct*

A [format 13](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-13-segmented-coverage)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `groups: crate::parser::LazyArray32<'a, super::format12::SequentialMapGroup>`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Returns a glyph index for a code point.
- `fn codepoints<impl FnMut(u32)>(self: &Self, f: impl Trait)` - Calls `f` for each codepoint defined in this table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable13<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



