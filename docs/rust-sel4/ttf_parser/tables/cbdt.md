**ttf_parser > tables > cbdt**

# Module: tables::cbdt

## Contents

**Structs**

- [`Table`](#table) - A [Color Bitmap Data Table](

---

## ttf_parser::tables::cbdt::Table

*Struct*

A [Color Bitmap Data Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cbdt).

EBDT and bdat also share the same structure, so this is re-used for them.

**Generic Parameters:**
- 'a

**Fields:**
- `locations: cblc::Table<'a>`
- `data: &'a [u8]`

**Methods:**

- `fn parse(locations: cblc::Table<'a>, data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn get(self: &Self, glyph_id: GlyphId, pixels_per_em: u16) -> Option<RasterGlyphImage<'a>>` - Returns a raster image for the glyph.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



