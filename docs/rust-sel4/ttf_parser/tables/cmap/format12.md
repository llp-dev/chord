**ttf_parser > tables > cmap > format12**

# Module: tables::cmap::format12

## Contents

**Structs**

- [`SequentialMapGroup`](#sequentialmapgroup)
- [`Subtable12`](#subtable12) - A [format 12](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-12-segmented-coverage)

---

## ttf_parser::tables::cmap::format12::SequentialMapGroup

*Struct*

**Fields:**
- `start_char_code: u32`
- `end_char_code: u32`
- `start_glyph_id: u32`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> SequentialMapGroup`



## ttf_parser::tables::cmap::format12::Subtable12

*Struct*

A [format 12](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-12-segmented-coverage)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `groups: crate::parser::LazyArray32<'a, SequentialMapGroup>`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Returns a glyph index for a code point.
- `fn codepoints<impl FnMut(u32)>(self: &Self, f: impl Trait)` - Calls `f` for each codepoint defined in this table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable12<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



