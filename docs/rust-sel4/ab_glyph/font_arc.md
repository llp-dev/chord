**ab_glyph > font_arc**

# Module: font_arc

## Contents

**Structs**

- [`FontArc`](#fontarc) - `Font` implementor that wraps another concrete `Font + 'static` type storing in an `Arc`.

---

## ab_glyph::font_arc::FontArc

*Struct*

`Font` implementor that wraps another concrete `Font + 'static` type storing in an `Arc`.

Provides convenient type erasure & cheap clones (particularly for `FontVec`).

# Example
```
use ab_glyph::{Font, FontArc};

# fn main() -> Result<(), ab_glyph::InvalidFont> {
let font = FontArc::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;

assert_eq!(font.glyph_id('s'), ab_glyph::GlyphId(56));
# Ok(()) }
```

**Tuple Struct**: `()`

**Methods:**

- `fn new<F>(font: F) -> Self` - # Example
- `fn try_from_vec(data: Vec<u8>) -> Result<Self, InvalidFont>` - Creates an `FontArc` from owned data.
- `fn try_from_slice(data: &'static [u8]) -> Result<Self, InvalidFont>` - Creates an `FontArc` from a byte-slice.

**Trait Implementations:**

- **From**
  - `fn from(font: FontRef<'static>) -> Self`
- **From**
  - `fn from(font: FontVec) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FontArc`
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
  - `fn outline(self: &Self, glyph: GlyphId) -> Option<Outline>`
  - `fn glyph_count(self: &Self) -> usize`
  - `fn codepoint_ids(self: &Self) -> crate::CodepointIdIter`
  - `fn glyph_raster_image2(self: &Self, id: GlyphId, size: u16) -> Option<v2::GlyphImage>`
  - `fn glyph_svg_image(self: &Self, id: GlyphId) -> Option<crate::GlyphSvg>`
  - `fn font_data(self: &Self) -> &[u8]`
- **From**
  - `fn from(font: Arc<dyn Font>) -> Self`



