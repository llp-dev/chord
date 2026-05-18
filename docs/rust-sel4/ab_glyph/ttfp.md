**ab_glyph > ttfp**

# Module: ttfp

## Contents

**Structs**

- [`FontRef`](#fontref) - Font data handle stored as a `&[u8]` + parsed data.
- [`FontVec`](#fontvec) - Font data handle stored in a `Vec<u8>`  + parsed data.

---

## ab_glyph::ttfp::FontRef

*Struct*

Font data handle stored as a `&[u8]` + parsed data.
See [`Font`] for more methods.

Also see the owned version [`FontVec`].

# Example
```
use ab_glyph::{Font, FontRef};

# fn main() -> Result<(), ab_glyph::InvalidFont> {
let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;

assert_eq!(font.glyph_id('s'), ab_glyph::GlyphId(56));
# Ok(()) }
```

**Generic Parameters:**
- 'font

**Tuple Struct**: `()`

**Methods:**

- `fn try_from_slice(data: &'font [u8]) -> Result<Self, InvalidFont>` - Creates an `FontRef` from a byte-slice.
- `fn try_from_slice_and_index(data: &'font [u8], index: u32) -> Result<Self, InvalidFont>` - Creates an `FontRef` from byte-slice.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FontRef<'font>`
- **Font**
  - `fn units_per_em(self: &Self) -> Option<f32>`
  - `fn ascent_unscaled(self: &Self) -> f32`
  - `fn descent_unscaled(self: &Self) -> f32`
  - `fn line_gap_unscaled(self: &Self) -> f32`
  - `fn italic_angle(self: &Self) -> f32`
  - `fn glyph_id(self: &Self, c: char) -> GlyphId`
  - `fn h_advance_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn h_side_bearing_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn v_advance_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn v_side_bearing_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn kern_unscaled(self: &Self, first: GlyphId, second: GlyphId) -> f32`
  - `fn outline(self: &Self, id: GlyphId) -> Option<Outline>`
  - `fn glyph_count(self: &Self) -> usize`
  - `fn codepoint_ids(self: &Self) -> crate::CodepointIdIter`
  - `fn glyph_raster_image2(self: &Self, id: GlyphId, size: u16) -> Option<v2::GlyphImage>`
  - `fn glyph_svg_image(self: &Self, id: GlyphId) -> Option<GlyphSvg>`
  - `fn font_data(self: &Self) -> &[u8]`



## ab_glyph::ttfp::FontVec

*Struct*

Font data handle stored in a `Vec<u8>`  + parsed data.
See [`Font`] for more methods.

Also see [`FontRef`].

# Example
```
use ab_glyph::{Font, FontVec};

# fn main() -> Result<(), ab_glyph::InvalidFont> {
# let owned_font_data = include_bytes!("../../dev/fonts/Exo2-Light.otf").to_vec();
let font = FontVec::try_from_vec_and_index(owned_font_data, 0)?;

assert_eq!(font.glyph_id('s'), ab_glyph::GlyphId(56));
# Ok(()) }
```

**Tuple Struct**: `()`

**Methods:**

- `fn try_from_vec(data: Vec<u8>) -> Result<Self, InvalidFont>` - Creates an `FontVec` from owned data.
- `fn try_from_vec_and_index(data: Vec<u8>, index: u32) -> Result<Self, InvalidFont>` - Creates an `FontVec` from owned data.
- `fn as_slice(self: &Self) -> &[u8]` - Extracts a slice containing the data passed into e.g. [`FontVec::try_from_vec`].
- `fn into_vec(self: Self) -> Vec<u8>` - Unwraps the data passed into e.g. [`FontVec::try_from_vec`].

**Trait Implementations:**

- **Font**
  - `fn units_per_em(self: &Self) -> Option<f32>`
  - `fn ascent_unscaled(self: &Self) -> f32`
  - `fn descent_unscaled(self: &Self) -> f32`
  - `fn line_gap_unscaled(self: &Self) -> f32`
  - `fn italic_angle(self: &Self) -> f32`
  - `fn glyph_id(self: &Self, c: char) -> GlyphId`
  - `fn h_advance_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn h_side_bearing_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn v_advance_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn v_side_bearing_unscaled(self: &Self, id: GlyphId) -> f32`
  - `fn kern_unscaled(self: &Self, first: GlyphId, second: GlyphId) -> f32`
  - `fn outline(self: &Self, id: GlyphId) -> Option<Outline>`
  - `fn glyph_count(self: &Self) -> usize`
  - `fn codepoint_ids(self: &Self) -> crate::CodepointIdIter`
  - `fn glyph_raster_image2(self: &Self, id: GlyphId, size: u16) -> Option<v2::GlyphImage>`
  - `fn glyph_svg_image(self: &Self, id: GlyphId) -> Option<GlyphSvg>`
  - `fn font_data(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



