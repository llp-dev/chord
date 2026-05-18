# ab_glyph

API for loading, scaling, positioning and rasterizing OpenType font glyphs.

# Example
```
use ab_glyph::{point, Font, FontRef, Glyph};

# fn main() -> Result<(), ab_glyph::InvalidFont> {
let font = FontRef::try_from_slice(include_bytes!("../../dev/fonts/Exo2-Light.otf"))?;

// Get a glyph for 'q' with a scale & position.
let q_glyph: Glyph = font
    .glyph_id('q')
    .with_scale_and_position(24.0, point(100.0, 0.0));

// Draw it.
if let Some(q) = font.outline_glyph(q_glyph) {
    q.draw(|x, y, c| { /* draw pixel `(x, y)` with coverage: `c` */ });
}
# Ok(()) }
```

## Modules

### [`codepoint_ids`](codepoint_ids.md)

*1 struct*

### [`err`](err.md)

*1 struct*

### [`font`](font.md)

*1 trait*

### [`font_arc`](font_arc.md)

*1 struct*

### [`glyph`](glyph.md)

*1 enum, 1 module, 4 structs*

### [`glyph::v2`](glyph/v2.md)

*1 struct*

### [`outlined`](outlined.md)

*1 enum, 3 structs*

### [`scale`](scale.md)

*1 trait, 3 structs*

### [`ttfp`](ttfp.md)

*2 structs*

