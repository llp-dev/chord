# ab_glyph_rasterizer

Coverage rasterization for lines, quadratic & cubic beziers.
Useful for drawing .otf font glyphs.

```
use ab_glyph_rasterizer::Rasterizer;
# let (width, height) = (1, 1);
let mut rasterizer = Rasterizer::new(width, height);

// draw outlines
# let [l0, l1, q0, q1, q2, c0, c1, c2, c3] = [ab_glyph_rasterizer::point(0.0, 0.0); 9];
rasterizer.draw_line(l0, l1);
rasterizer.draw_quad(q0, q1, q2);
rasterizer.draw_cubic(c0, c1, c2, c3);

// iterate over the resultant pixel alphas, e.g. save pixel to a buffer
rasterizer.for_each_pixel(|index, alpha| {
    // ...
});
```

## Modules

### [`geometry`](geometry.md)

*1 function, 1 struct*

### [`raster`](raster.md)

*1 struct*

