**ttf_parser > tables > glyf**

# Module: tables::glyf

## Contents

**Structs**

- [`Builder`](#builder)
- [`CompositeGlyphFlags`](#compositeglyphflags)
- [`CompositeGlyphInfo`](#compositeglyphinfo)
- [`CompositeGlyphIter`](#compositeglyphiter)
- [`CoordsIter`](#coordsiter)
- [`EndpointsIter`](#endpointsiter) - A simple flattening iterator for glyph's endpoints.
- [`FlagsIter`](#flagsiter)
- [`GlyphPoint`](#glyphpoint)
- [`GlyphPointsIter`](#glyphpointsiter)
- [`Point`](#point)
- [`SimpleGlyphFlags`](#simpleglyphflags)
- [`Table`](#table) - A [Glyph Data Table](

**Functions**

- [`outline_impl`](#outline_impl)
- [`parse_simple_outline`](#parse_simple_outline)
- [`resolve_coords_len`](#resolve_coords_len) - Resolves coordinate arrays length.

**Constants**

- [`MAX_COMPONENTS`](#max_components)

---

## ttf_parser::tables::glyf::Builder

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `builder: &'a  mut dyn OutlineBuilder`
- `transform: crate::Transform`
- `is_default_ts: bool`
- `bbox: crate::RectF`
- `first_on_curve: Option<Point>`
- `first_off_curve: Option<Point>`
- `last_off_curve: Option<Point>`

**Methods:**

- `fn new(transform: Transform, bbox: RectF, builder: &'a  mut dyn OutlineBuilder) -> Self`
- `fn move_to(self: & mut Self, x: f32, y: f32)`
- `fn line_to(self: & mut Self, x: f32, y: f32)`
- `fn quad_to(self: & mut Self, x1: f32, y1: f32, x: f32, y: f32)`
- `fn push_point(self: & mut Self, x: f32, y: f32, on_curve_point: bool, last_point: bool)`
- `fn finish_contour(self: & mut Self)`



## ttf_parser::tables::glyf::CompositeGlyphFlags

*Struct*

**Tuple Struct**: `(u16)`

**Methods:**

- `fn arg_1_and_2_are_words(self: Self) -> bool`
- `fn args_are_xy_values(self: Self) -> bool`
- `fn we_have_a_scale(self: Self) -> bool`
- `fn more_components(self: Self) -> bool`
- `fn we_have_an_x_and_y_scale(self: Self) -> bool`
- `fn we_have_a_two_by_two(self: Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CompositeGlyphFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::glyf::CompositeGlyphInfo

*Struct*

**Fields:**
- `glyph_id: crate::GlyphId`
- `transform: crate::Transform`
- `flags: CompositeGlyphFlags`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CompositeGlyphInfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::glyf::CompositeGlyphIter

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `stream: crate::parser::Stream<'a>`

**Methods:**

- `fn new(data: &'a [u8]) -> Self`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CompositeGlyphIter<'a>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## ttf_parser::tables::glyf::CoordsIter

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `stream: crate::parser::Stream<'a>`
- `prev: i16`

**Methods:**

- `fn new(data: &'a [u8]) -> Self`
- `fn next(self: & mut Self, is_short: bool, is_same_or_short: bool) -> i16`

**Trait Implementations:**

- **Default**
  - `fn default() -> CoordsIter<'a>`
- **Clone**
  - `fn clone(self: &Self) -> CoordsIter<'a>`



## ttf_parser::tables::glyf::EndpointsIter

*Struct*

A simple flattening iterator for glyph's endpoints.

Translates endpoints like: 2 4 7
into flags: 0 0 1 0 1 0 0 1

**Generic Parameters:**
- 'a

**Fields:**
- `endpoints: crate::parser::LazyArray16<'a, u16>`
- `index: u16`
- `left: u16`

**Methods:**

- `fn new(endpoints: LazyArray16<'a, u16>) -> Option<Self>`
- `fn next(self: & mut Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> EndpointsIter<'a>`
- **Default**
  - `fn default() -> EndpointsIter<'a>`



## ttf_parser::tables::glyf::FlagsIter

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `stream: crate::parser::Stream<'a>`
- `repeats: u8`
- `flags: SimpleGlyphFlags`

**Methods:**

- `fn new(data: &'a [u8]) -> Self`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FlagsIter<'a>`
- **Default**
  - `fn default() -> FlagsIter<'a>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## ttf_parser::tables::glyf::GlyphPoint

*Struct*

**Fields:**
- `x: i16`
- `y: i16`
- `on_curve_point: bool` - Indicates that a point is a point on curve
- `last_point: bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GlyphPoint`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::glyf::GlyphPointsIter

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `endpoints: EndpointsIter<'a>`
- `flags: FlagsIter<'a>`
- `x_coords: CoordsIter<'a>`
- `y_coords: CoordsIter<'a>`
- `points_left: u16`

**Trait Implementations:**

- **Default**
  - `fn default() -> GlyphPointsIter<'a>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> GlyphPointsIter<'a>`



## ttf_parser::tables::glyf::MAX_COMPONENTS

*Constant*: `u8`



## ttf_parser::tables::glyf::Point

*Struct*

**Fields:**
- `x: f32`
- `y: f32`

**Methods:**

- `fn lerp(self: Self, other: Point, t: f32) -> Point`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Point`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::glyf::SimpleGlyphFlags

*Struct*

**Tuple Struct**: `(u8)`

**Methods:**

- `fn on_curve_point(self: Self) -> bool`
- `fn x_short(self: Self) -> bool`
- `fn y_short(self: Self) -> bool`
- `fn repeat_flag(self: Self) -> bool`
- `fn x_is_same_or_positive_short(self: Self) -> bool`
- `fn y_is_same_or_positive_short(self: Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SimpleGlyphFlags`
- **Default**
  - `fn default() -> SimpleGlyphFlags`



## ttf_parser::tables::glyf::Table

*Struct*

A [Glyph Data Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/glyf).

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `loca_table: loca::Table<'a>`

**Methods:**

- `fn parse(loca_table: loca::Table<'a>, data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn outline(self: &Self, glyph_id: GlyphId, builder: & mut dyn OutlineBuilder) -> Option<Rect>` - Outlines a glyph.
- `fn bbox(self: &Self, glyph_id: GlyphId) -> Option<Rect>` - The bounding box of the glyph. Unlike the `outline` method, this method does not
- `fn get(self: &Self, glyph_id: GlyphId) -> Option<&'a [u8]>`
- `fn outline_points(self: &Self, glyph_id: GlyphId) -> u16` - Returns the number of points in this outline.
- `fn outline_points_impl(self: &Self, glyph_id: GlyphId) -> Option<u16>`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`



## ttf_parser::tables::glyf::outline_impl

*Function*

```rust
fn outline_impl(loca_table: loca::Table, glyf_table: &[u8], data: &[u8], depth: u8, builder: & mut Builder) -> Option<Option<crate::Rect>>
```



## ttf_parser::tables::glyf::parse_simple_outline

*Function*

```rust
fn parse_simple_outline(glyph_data: &[u8], number_of_contours: core::num::NonZeroU16) -> Option<GlyphPointsIter>
```



## ttf_parser::tables::glyf::resolve_coords_len

*Function*

Resolves coordinate arrays length.

The length depends on *Simple Glyph Flags*, so we have to process them all to find it.

```rust
fn resolve_coords_len(s: & mut crate::parser::Stream, points_total: u16) -> Option<(u32, u32)>
```



