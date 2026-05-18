**ttf_parser > tables > cmap > format0**

# Module: tables::cmap::format0

## Contents

**Structs**

- [`Subtable0`](#subtable0) - A [format 0](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-0-byte-encoding-table)

---

## ttf_parser::tables::cmap::format0::Subtable0

*Struct*

A [format 0](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-0-byte-encoding-table)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `glyph_ids: &'a [u8]` - Just a list of 256 8bit glyph IDs.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Returns a glyph index for a code point.
- `fn codepoints<impl FnMut(u32)>(self: &Self, f: impl Trait)` - Calls `f` for each codepoint defined in this table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable0<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



