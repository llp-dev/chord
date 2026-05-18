**ttf_parser > tables > cmap > format2**

# Module: tables::cmap::format2

## Contents

**Structs**

- [`SubHeaderRecord`](#subheaderrecord)
- [`Subtable2`](#subtable2) - A [format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-2-high-byte-mapping-through-table)

---

## ttf_parser::tables::cmap::format2::SubHeaderRecord

*Struct*

**Fields:**
- `first_code: u16`
- `entry_count: u16`
- `id_delta: i16`
- `id_range_offset: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SubHeaderRecord`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::cmap::format2::Subtable2

*Struct*

A [format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-2-high-byte-mapping-through-table)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `sub_header_keys: crate::parser::LazyArray16<'a, u16>`
- `sub_headers_offset: usize`
- `sub_headers: crate::parser::LazyArray16<'a, SubHeaderRecord>`
- `data: &'a [u8]`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Returns a glyph index for a code point.
- `fn codepoints<impl FnMut(u32)>(self: &Self, f: impl Trait)` - Calls `f` for each codepoint defined in this table.
- `fn codepoints_inner<impl FnMut(u32)>(self: &Self, f: impl Trait) -> Option<()>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable2<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



