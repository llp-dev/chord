**ab_glyph > outlined**

# Module: outlined

## Contents

**Structs**

- [`Outline`](#outline) - A "raw" collection of outline curves for a glyph, unscaled & unpositioned.
- [`OutlinedGlyph`](#outlinedglyph) - A glyph that has been outlined at a scale & position.
- [`Rect`](#rect) - A rectangle, with top-left corner at `min`, and bottom-right corner at `max`.

**Enums**

- [`OutlineCurve`](#outlinecurve) - Glyph outline primitives.

---

## ab_glyph::outlined::Outline

*Struct*

A "raw" collection of outline curves for a glyph, unscaled & unpositioned.

**Fields:**
- `bounds: Rect` - Unscaled bounding box.
- `curves: alloc::vec::Vec<OutlineCurve>` - Unscaled & unpositioned outline curves.

**Methods:**

- `fn px_bounds(self: &Self, scale_factor: PxScaleFactor, position: Point) -> Rect` - Convert unscaled bounds into pixel bounds at a given scale & position.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Outline`



## ab_glyph::outlined::OutlineCurve

*Enum*

Glyph outline primitives.

**Variants:**
- `Line(crate::Point, crate::Point)` - Straight line from `.0` to `.1`.
- `Quad(crate::Point, crate::Point, crate::Point)` - Quadratic Bézier curve from `.0` to `.2` using `.1` as the control.
- `Cubic(crate::Point, crate::Point, crate::Point, crate::Point)` - Cubic Bézier curve from `.0` to `.3` using `.1` as the control at the beginning of the

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OutlineCurve`



## ab_glyph::outlined::OutlinedGlyph

*Struct*

A glyph that has been outlined at a scale & position.

**Methods:**

- `fn new(glyph: Glyph, outline: Outline, scale_factor: PxScaleFactor) -> Self` - Constructs an `OutlinedGlyph` from the source `Glyph`, pixel bounds
- `fn glyph(self: &Self) -> &Glyph` - Glyph info.
- `fn px_bounds(self: &Self) -> Rect` - Conservative whole number pixel bounding box for this glyph outline.
- `fn draw<O>(self: &Self, o: O)` - Draw this glyph outline using a pixel & coverage handling function.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OutlinedGlyph`
- **AsRef**
  - `fn as_ref(self: &Self) -> &Glyph`



## ab_glyph::outlined::Rect

*Struct*

A rectangle, with top-left corner at `min`, and bottom-right corner at `max`.

**Fields:**
- `min: crate::Point`
- `max: crate::Point`

**Methods:**

- `fn width(self: &Self) -> f32`
- `fn height(self: &Self) -> f32`

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Rect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Rect`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Rect) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Rect) -> bool`



