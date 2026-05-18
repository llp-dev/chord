**ab_glyph > glyph**

# Module: glyph

## Contents

**Modules**

- [`v2`](#v2)

**Structs**

- [`Glyph`](#glyph) - A glyph with pixel scale & position.
- [`GlyphId`](#glyphid) - Glyph id.
- [`GlyphImage`](#glyphimage) - Old version of [`v2::GlyphImage`].
- [`GlyphSvg`](#glyphsvg)

**Enums**

- [`GlyphImageFormat`](#glyphimageformat) - Valid formats for a [`GlyphImage`].

---

## ab_glyph::glyph::Glyph

*Struct*

A glyph with pixel scale & position.

**Fields:**
- `id: GlyphId` - Glyph id.
- `scale: crate::PxScale` - Pixel scale of this glyph.
- `position: crate::Point` - Position of this glyph.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Glyph`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Glyph) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Glyph) -> bool`



## ab_glyph::glyph::GlyphId

*Struct*

Glyph id.

# Example
```
use ab_glyph::{Font, FontRef, GlyphId};
# fn main() -> Result<(), ab_glyph::InvalidFont> {
let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;

let q_id: GlyphId = font.glyph_id('q');
# Ok(()) }
```

**Tuple Struct**: `(u16)`

**Methods:**

- `fn with_scale_and_position<S, P>(self: Self, scale: S, position: P) -> Glyph` - Construct a `Glyph` with given scale & position.
- `fn with_scale<S>(self: Self, scale: S) -> Glyph` - Construct a `Glyph` with given scale and position `point(0.0, 0.0)`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &GlyphId) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> GlyphId`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GlyphId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &GlyphId) -> bool`
- **Default**
  - `fn default() -> GlyphId`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ab_glyph::glyph::GlyphImage

*Struct*

Old version of [`v2::GlyphImage`].

**Generic Parameters:**
- 'a

**Fields:**
- `origin: crate::Point` - Offset of the image from the normal origin (top at the baseline plus
- `scale: f32` - Current scale of the image in pixels per em.
- `data: &'a [u8]` - Raw image data, not a bitmap in the case of [`GlyphImageFormat::Png`] format.
- `format: GlyphImageFormat` - Format of the raw data.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GlyphImage<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ab_glyph::glyph::GlyphImageFormat

*Enum*

Valid formats for a [`GlyphImage`].

**Variants:**
- `Png`
- `BitmapMono` - A monochrome bitmap.
- `BitmapMonoPacked` - A packed monochrome bitmap.
- `BitmapGray2` - A grayscale bitmap with 2 bits per pixel.
- `BitmapGray2Packed` - A packed grayscale bitmap with 2 bits per pixel.
- `BitmapGray4` - A grayscale bitmap with 4 bits per pixel.
- `BitmapGray4Packed` - A packed grayscale bitmap with 4 bits per pixel.
- `BitmapGray8` - A grayscale bitmap with 8 bits per pixel.
- `BitmapPremulBgra32` - A color bitmap with 32 bits per pixel.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GlyphImageFormat`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ab_glyph::glyph::GlyphSvg

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]` - Raw image data, it should be rendered or decompressed (in case of SVGZ)
- `start_glyph_id: GlyphId` - The first glyph ID for the range covered by this record.
- `end_glyph_id: GlyphId` - The last glyph ID, *inclusive*, for the range covered by this record.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GlyphSvg<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: v2



