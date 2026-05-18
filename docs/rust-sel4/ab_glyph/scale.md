**ab_glyph > scale**

# Module: scale

## Contents

**Structs**

- [`PxScale`](#pxscale) - Pixel scale.
- [`PxScaleFactor`](#pxscalefactor) - 2D scale factors for use with unscaled metrics.
- [`PxScaleFont`](#pxscalefont) - A [`Font`] and an associated pixel scale.

**Traits**

- [`ScaleFont`](#scalefont) - A [`Font`] with an associated pixel scale. This can be used to provide

---

## ab_glyph::scale::PxScale

*Struct*

Pixel scale.

This is the pixel-height of text.

Usually one uses `x == y`, but one may use a different ratio to stretch a
font horizontally or vertically.

To convert pt size into pixel-scale see [`Font::pt_to_px_scale`].

# Example
```
use ab_glyph::PxScale;

let uniform_scale_24px = PxScale::from(24.0);
```

**Fields:**
- `x: f32` - Horizontal scale in pixels.
- `y: f32` - Vertical scale in pixels.

**Methods:**

- `fn round(self: Self) -> Self` - Returns a `PxScale` with both x & y scale values set to the nearest integer.

**Traits:** Copy

**Trait Implementations:**

- **From**
  - `fn from(s: f32) -> Self` - Uniform scaling where x & y are the same.
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PxScale) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &PxScale) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PxScale`



## ab_glyph::scale::PxScaleFactor

*Struct*

2D scale factors for use with unscaled metrics.

**Fields:**
- `horizontal: f32`
- `vertical: f32`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PxScaleFactor`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PxScaleFactor) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &PxScaleFactor) -> bool`



## ab_glyph::scale::PxScaleFont

*Struct*

A [`Font`] and an associated pixel scale.

**Generic Parameters:**
- F

**Fields:**
- `font: F`
- `scale: PxScale`

**Methods:**

- `fn with_scale<S>(self: Self, scale: S) -> Self`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PxScaleFont<F>`
- **ScaleFont**
  - `fn scale(self: &Self) -> PxScale`
  - `fn font(self: &Self) -> &F`
  - `fn codepoint_ids(self: &Self) -> crate::CodepointIdIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ab_glyph::scale::ScaleFont

*Trait*

A [`Font`] with an associated pixel scale. This can be used to provide
pixel scale values for glyph advances, heights etc.

# Example
```
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};

# fn main() -> Result<(), ab_glyph::InvalidFont> {
let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;

// Associate the font with a scale of 45px
let scaled_font = font.as_scaled(PxScale::from(45.0));

assert_eq!(scaled_font.height(), 45.0);
assert_eq!(scaled_font.h_advance(scaled_font.glyph_id('b')), 21.225);

// Replace associated scale with another
let scaled_font = scaled_font.with_scale(180.0);

assert_eq!(scaled_font.height(), 180.0);
assert_eq!(scaled_font.h_advance(scaled_font.glyph_id('b')), 84.9);
# Ok(()) }
```

**Methods:**

- `scale`: Returns the pixel scale associated with this font.
- `font`: Returns a font reference.
- `h_scale_factor`: Scale factor for unscaled font horizontal values.
- `v_scale_factor`: Scale factor for unscaled font vertical values.
- `scale_factor`
- `ascent`: Pixel scaled glyph ascent. See [glyph layout concepts](Font#glyph-layout-concepts).
- `descent`: Pixel scaled glyph descent. See [glyph layout concepts](Font#glyph-layout-concepts).
- `height`: Pixel scaled height `ascent - descent`. See [glyph layout concepts](Font#glyph-layout-concepts).
- `line_gap`: Pixel scaled line gap. See [glyph layout concepts](Font#glyph-layout-concepts).
- `glyph_id`: Lookup a `GlyphId` matching a given `char`.
- `scaled_glyph`: Construct a [`Glyph`] with the font's pixel scale at
- `h_advance`: Pixel scaled horizontal advance for a given glyph.
- `h_side_bearing`: Pixel scaled horizontal side bearing for a given glyph.
- `v_advance`: Pixel scaled vertical advance for a given glyph.
- `v_side_bearing`: Pixel scaled vertical side bearing for a given glyph.
- `kern`: Returns additional pixel scaled kerning to apply for a particular pair of glyphs.
- `glyph_bounds`: Returns the layout bounds of this glyph.
- `glyph_count`: The number of glyphs present in this font. Glyph identifiers for this
- `codepoint_ids`: Returns an iterator of all distinct `(GlyphId, char)` pairs. Not ordered.
- `outline_glyph`: Compute glyph outline ready for drawing.



