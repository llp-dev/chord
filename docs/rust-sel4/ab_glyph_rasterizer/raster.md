**ab_glyph_rasterizer > raster**

# Module: raster

## Contents

**Structs**

- [`Rasterizer`](#rasterizer) - Coverage rasterizer for lines, quadratic & cubic beziers.

---

## ab_glyph_rasterizer::raster::Rasterizer

*Struct*

Coverage rasterizer for lines, quadratic & cubic beziers.

**Methods:**

- `fn new(width: usize, height: usize) -> Self` - Allocates a new rasterizer that can draw onto a `width` x `height` alpha grid.
- `fn reset(self: & mut Self, width: usize, height: usize)` - Resets the rasterizer to an empty `width` x `height` alpha grid. This method behaves as if
- `fn clear(self: & mut Self)` - Clears the rasterizer. This method behaves as if the Rasterizer were re-created with the same
- `fn dimensions(self: &Self) -> (usize, usize)` - Returns the dimensions the rasterizer was built to draw to.
- `fn draw_line(self: & mut Self, p0: Point, p1: Point)` - Adds a straight line from `p0` to `p1` to the outline.
- `fn draw_quad(self: & mut Self, p0: Point, p1: Point, p2: Point)` - Adds a quadratic Bézier curve from `p0` to `p2` to the outline using `p1` as the control.
- `fn draw_cubic(self: & mut Self, p0: Point, p1: Point, p2: Point, p3: Point)` - Adds a cubic Bézier curve from `p0` to `p3` to the outline using `p1` as the control
- `fn for_each_pixel<O>(self: &Self, px_fn: O)` - Run a callback for each pixel `index` & `alpha`, with indices in `0..width * height`.
- `fn for_each_pixel_2d<O>(self: &Self, px_fn: O)` - Run a callback for each pixel x position, y position & alpha.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



