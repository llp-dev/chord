**ttf_parser > tables > cmap > format14**

# Module: tables::cmap::format14

## Contents

**Structs**

- [`Subtable14`](#subtable14) - A [format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
- [`UVSMappingRecord`](#uvsmappingrecord)
- [`UnicodeRangeRecord`](#unicoderangerecord)
- [`VariationSelectorRecord`](#variationselectorrecord)

**Enums**

- [`GlyphVariationResult`](#glyphvariationresult) - A result of a variation glyph mapping.

---

## ttf_parser::tables::cmap::format14::GlyphVariationResult

*Enum*

A result of a variation glyph mapping.

**Variants:**
- `Found(crate::GlyphId)` - Glyph was found in the variation encoding table.
- `UseDefault` - Glyph should be looked in other, non-variation tables.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &GlyphVariationResult) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> GlyphVariationResult`



## ttf_parser::tables::cmap::format14::Subtable14

*Struct*

A [format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `records: crate::parser::LazyArray32<'a, VariationSelectorRecord>`
- `data: &'a [u8]`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyph_index(self: &Self, code_point: u32, variation: u32) -> Option<GlyphVariationResult>` - Returns a glyph index for a code point.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable14<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::cmap::format14::UVSMappingRecord

*Struct*

**Fields:**
- `unicode_value: u32`
- `glyph_id: crate::GlyphId`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> UVSMappingRecord`



## ttf_parser::tables::cmap::format14::UnicodeRangeRecord

*Struct*

**Fields:**
- `start_unicode_value: u32`
- `additional_count: u8`

**Methods:**

- `fn contains(self: &Self, c: u32) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnicodeRangeRecord`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::cmap::format14::VariationSelectorRecord

*Struct*

**Fields:**
- `var_selector: u32`
- `default_uvs_offset: Option<crate::parser::Offset32>`
- `non_default_uvs_offset: Option<crate::parser::Offset32>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VariationSelectorRecord`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



