**ttf_parser > tables > cmap > format4**

# Module: tables::cmap::format4

## Contents

**Structs**

- [`Subtable4`](#subtable4) - A [format 4](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-4-segment-mapping-to-delta-values)

---

## ttf_parser::tables::cmap::format4::Subtable4

*Struct*

A [format 4](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-4-segment-mapping-to-delta-values)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `start_codes: crate::parser::LazyArray16<'a, u16>`
- `end_codes: crate::parser::LazyArray16<'a, u16>`
- `id_deltas: crate::parser::LazyArray16<'a, i16>`
- `id_range_offsets: crate::parser::LazyArray16<'a, u16>`
- `id_range_offset_pos: usize`
- `data: &'a [u8]`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Returns a glyph index for a code point.
- `fn codepoints<impl FnMut(u32)>(self: &Self, f: impl Trait)` - Calls `f` for each codepoint defined in this table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable4<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



