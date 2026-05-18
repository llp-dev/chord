**owned_ttf_parser > preparse**

# Module: preparse

## Contents

**Structs**

- [`PreParsedSubtables`](#preparsedsubtables) - A `Face` with cmap & kern subtables parsed once on initialization.

---

## owned_ttf_parser::preparse::PreParsedSubtables

*Struct*

A `Face` with cmap & kern subtables parsed once on initialization.

Provides much faster [`PreParsedSubtables::glyph_index`] &
[`PreParsedSubtables::glyphs_hor_kerning`] methods compared to the
`.as_face_ref()` equivalents that must parse their subtables on each call.

# Example
```
use owned_ttf_parser::{AsFaceRef, GlyphId, OwnedFace, PreParsedSubtables};

# let owned_font_data = include_bytes!("../fonts/font.ttf").to_vec();
let owned_face = OwnedFace::from_vec(owned_font_data, 0).unwrap();
let faster_face = PreParsedSubtables::from(owned_face);

// Lookup a GlyphId using the pre-parsed cmap subtables
// this is much faster than doing: .as_face_ref().glyph_index('x')
assert_eq!(faster_face.glyph_index('x'), Some(GlyphId(91)));

// The rest of the methods are still available as normal
assert_eq!(faster_face.as_face_ref().ascender(), 2254);
```

**Generic Parameters:**
- 'face
- F

**Fields:**
- `face: F` - Underlying face.

**Methods:**

- `fn glyph_index(self: &Self, c: char) -> Option<GlyphId>` - Maps a character to a `GlyphId` using pre-parsed unicode cmap subtables.
- `fn glyph_variation_index(self: &Self, c: char, v: char) -> Option<GlyphId>` - Maps a variation of a character to a `GlyphId` using pre-parsed unicode cmap subtables.
- `fn glyphs_hor_kerning(self: &Self, first: GlyphId, second: GlyphId) -> Option<i16>` - Returns horizontal kerning for a pair of glyphs using pre-parsed kern subtables.

**Trait Implementations:**

- **From**
  - `fn from(face: OwnedFace) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FaceMut**
  - `fn set_variation(self: & mut Self, axis: ttf_parser::Tag, value: f32) -> Option<()>`
- **From**
  - `fn from(face: Face<'face>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> PreParsedSubtables<'face, F>`
- **AsFaceRef**
  - `fn as_face_ref(self: &Self) -> &ttf_parser::Face`



