**ab_glyph > font**

# Module: font

## Contents

**Traits**

- [`Font`](#font) - Functionality required from font data.

---

## ab_glyph::font::Font

*Trait*

Functionality required from font data.

See also [`FontArc`](crate::FontArc), [`FontRef`](crate::FontRef)
and [`FontVec`](crate::FontVec).

## Units

Units of unscaled accessors are "font units", which is an arbitrary unit
defined by the font. See [`Font::units_per_em`].

ab_glyph uses a non-standard scale [`PxScale`] which is the pixel height
of the text. See [`Font::pt_to_px_scale`] to convert standard point sizes.

## Glyph layout concepts
Fonts provide several properties to inform layout of glyphs.
```text
         ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
                  |  .:x++++==              |
                  | .#+                     |
                  | :@            =++=++x=: |
           ascent | +#       x:  +x     x+  |
                  | =#       #:  :#:---:#:  | height
                  | -@-      #:  .#--:--    |
                  |  =#:-.-==#:   #x+===:.  |
baseline ____________ .-::-. ..  #:    .:@. |
                  |              #+--..-=#. |
          descent |               -::=::-   |
         ____________________________________
                | |             |           | line_gap
                | |  h_advance  |           ‾
                 ^                      
           h_side_bearing
```

**Methods:**

- `units_per_em`: Get the size of the font unit
- `pt_to_px_scale`: Converts pt units into [`PxScale`].
- `ascent_unscaled`: Unscaled glyph ascent. See [glyph layout concepts](Font#glyph-layout-concepts).
- `descent_unscaled`: Unscaled glyph descent. See [glyph layout concepts](Font#glyph-layout-concepts).
- `height_unscaled`: Unscaled height `ascent - descent`. See [glyph layout concepts](Font#glyph-layout-concepts).
- `line_gap_unscaled`: Unscaled line gap. See [glyph layout concepts](Font#glyph-layout-concepts).
- `italic_angle`: The slant angle of the font.
- `glyph_id`: Lookup a `GlyphId` matching a given `char`.
- `h_advance_unscaled`: Unscaled horizontal advance for a given glyph id.
- `h_side_bearing_unscaled`: Unscaled horizontal side bearing for a given glyph id.
- `v_advance_unscaled`: Unscaled vertical advance for a given glyph id.
- `v_side_bearing_unscaled`: Unscaled vertical side bearing for a given glyph id.
- `kern_unscaled`: Returns additional unscaled kerning to apply for a particular pair of glyph ids.
- `outline`: Compute unscaled glyph outline curves & bounding box.
- `glyph_count`: The number of glyphs present in this font. Glyph identifiers for this
- `codepoint_ids`: Returns an iterator of all distinct `(GlyphId, char)` pairs. Not ordered.
- `glyph_raster_image`: Returns a pre-rendered image of the glyph.
- `glyph_raster_image2`: Returns a pre-rendered image of the glyph.
- `glyph_svg_image`: Returns raw SVG data of a range of glyphs which includes this one.
- `glyph_bounds`: Returns the layout bounds of this glyph.
- `outline_glyph`: Compute glyph outline ready for drawing.
- `as_scaled`: Construct a [`PxScaleFont`] by associating with the given pixel `scale`.
- `into_scaled`: Move into a [`PxScaleFont`] associated with the given pixel `scale`.
- `font_data`: Extracts a slice containing the data passed into e.g. [`FontArc::try_from_slice`].



