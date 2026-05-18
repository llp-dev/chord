**ttf_parser > tables > colr**

# Module: tables::colr

## Contents

**Structs**

- [`BaseGlyphPaintRecord`](#baseglyphpaintrecord) - A [BaseGlyphPaintRecord](
- [`BaseGlyphRecord`](#baseglyphrecord) - A [base glyph](
- [`ClipList`](#cliplist) - A [clip list](
- [`ClipRecord`](#cliprecord) - A [clip record](
- [`ColorStop`](#colorstop) - A [gradient extend](
- [`ColorStopRaw`](#colorstopraw) - A [color stop](
- [`GradientStopsIter`](#gradientstopsiter) - An iterator over stops of a gradient.
- [`LayerRecord`](#layerrecord) - A [layer](
- [`LinearGradient`](#lineargradient) - A [linear gradient](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-4-and-5-paintlineargradient-paintvarlineargradient)
- [`NonVarColorLine`](#nonvarcolorline)
- [`RadialGradient`](#radialgradient) - A [radial gradient](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-6-and-7-paintradialgradient-paintvarradialgradient)
- [`RecursionStack`](#recursionstack)
- [`SweepGradient`](#sweepgradient) - A [sweep gradient](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-8-and-9-paintsweepgradient-paintvarsweepgradient)
- [`Table`](#table) - A [Color Table](

**Enums**

- [`ColorLine`](#colorline)
- [`CompositeMode`](#compositemode) - A [composite mode](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#format-32-paintcomposite)
- [`GradientExtend`](#gradientextend) - A [gradient extend](
- [`Paint`](#paint) - A paint.

**Traits**

- [`Painter`](#painter) - A trait for color glyph painting.

**Type Aliases**

- [`ClipBox`](#clipbox) - A [ClipBox](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist).

---

## ttf_parser::tables::colr::BaseGlyphPaintRecord

*Struct*

A [BaseGlyphPaintRecord](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist).

**Fields:**
- `glyph_id: crate::GlyphId`
- `paint_table_offset: crate::parser::Offset32`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> BaseGlyphPaintRecord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::colr::BaseGlyphRecord

*Struct*

A [base glyph](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyph-and-layer-records).

**Fields:**
- `glyph_id: crate::GlyphId`
- `first_layer_index: u16`
- `num_layers: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> BaseGlyphRecord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::colr::ClipBox

*Type Alias*: `crate::RectF`

A [ClipBox](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist).



## ttf_parser::tables::colr::ClipList

*Struct*

A [clip list](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist).

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `records: crate::LazyArray32<'a, ClipRecord>`

**Methods:**

- `fn get(self: &Self, index: u32) -> Option<ClipBox>`
- `fn find(self: &Self, glyph_id: GlyphId) -> Option<ClipBox>` - Returns a ClipBox by glyph ID.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ClipList<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> ClipList<'a>`



## ttf_parser::tables::colr::ClipRecord

*Struct*

A [clip record](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist).

**Fields:**
- `start_glyph_id: crate::GlyphId` - The first glyph ID for the range covered by this record.
- `end_glyph_id: crate::GlyphId` - The last glyph ID, *inclusive*, for the range covered by this record.
- `clip_box_offset: crate::parser::Offset24` - The offset to the clip box.

**Methods:**

- `fn glyphs_range(self: &Self) -> core::ops::RangeInclusive<GlyphId>` - Returns the glyphs range.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> ClipRecord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::colr::ColorLine

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `NonVarColorLine(NonVarColorLine<'a>)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ColorLine<'a>`



## ttf_parser::tables::colr::ColorStop

*Struct*

A [gradient extend](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist).

**Fields:**
- `stop_offset: f32` - The offset of the color stop.
- `color: crate::RgbaColor` - The color of the color stop.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ColorStop`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::colr::ColorStopRaw

*Struct*

A [color stop](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#color-references-colorstop-and-colorline).

**Fields:**
- `stop_offset: crate::parser::F2DOT14`
- `palette_index: u16`
- `alpha: crate::parser::F2DOT14`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ColorStopRaw`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::colr::CompositeMode

*Enum*

A [composite mode](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#format-32-paintcomposite)

**Variants:**
- `Clear` - The composite mode 'Clear'.
- `Source` - The composite mode 'Source'.
- `Destination` - The composite mode 'Destination'.
- `SourceOver` - The composite mode 'SourceOver'.
- `DestinationOver` - The composite mode 'DestinationOver'.
- `SourceIn` - The composite mode 'SourceIn'.
- `DestinationIn` - The composite mode 'DestinationIn'.
- `SourceOut` - The composite mode 'SourceOut'.
- `DestinationOut` - The composite mode 'DestinationOut'.
- `SourceAtop` - The composite mode 'SourceAtop'.
- `DestinationAtop` - The composite mode 'DestinationAtop'.
- `Xor` - The composite mode 'Xor'.
- `Plus` - The composite mode 'Plus'.
- `Screen` - The composite mode 'Screen'.
- `Overlay` - The composite mode 'Overlay'.
- `Darken` - The composite mode 'Darken'.
- `Lighten` - The composite mode 'Lighten'.
- `ColorDodge` - The composite mode 'ColorDodge'.
- `ColorBurn` - The composite mode 'ColorBurn'.
- `HardLight` - The composite mode 'HardLight'.
- `SoftLight` - The composite mode 'SoftLight'.
- `Difference` - The composite mode 'Difference'.
- `Exclusion` - The composite mode 'Exclusion'.
- `Multiply` - The composite mode 'Multiply'.
- `Hue` - The composite mode 'Hue'.
- `Saturation` - The composite mode 'Saturation'.
- `Color` - The composite mode 'Color'.
- `Luminosity` - The composite mode 'Luminosity'.

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CompositeMode) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> CompositeMode`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::colr::GradientExtend

*Enum*

A [gradient extend](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist).

**Variants:**
- `Pad` - The `Pad` gradient extend mode.
- `Repeat` - The `Repeat` gradient extend mode.
- `Reflect` - The `Reflect` gradient extend mode.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> GradientExtend`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &GradientExtend) -> bool`



## ttf_parser::tables::colr::GradientStopsIter

*Struct*

An iterator over stops of a gradient.

**Generic Parameters:**
- 'a
- 'b

**Fields:**
- `color_line: &'b ColorLine<'a>`
- `palette: u16`
- `index: u16`

**Traits:** Copy

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> GradientStopsIter<'a, 'b>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::colr::LayerRecord

*Struct*

A [layer](
https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyph-and-layer-records).

**Fields:**
- `glyph_id: crate::GlyphId`
- `palette_index: u16`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> LayerRecord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::colr::LinearGradient

*Struct*

A [linear gradient](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-4-and-5-paintlineargradient-paintvarlineargradient)

**Generic Parameters:**
- 'a

**Fields:**
- `x0: f32` - The `x0` value.
- `y0: f32` - The `y0` value.
- `x1: f32` - The `x1` value.
- `y1: f32` - The `y1` value.
- `x2: f32` - The `x2` value.
- `y2: f32` - The `y2` value.
- `extend: GradientExtend` - The extend.
- `color_line: ColorLine<'a>`

**Methods:**

- `fn stops<'b>(self: &'b Self, palette: u16) -> GradientStopsIter<'a, 'b>` - Returns an iterator over the stops of the linear gradient. Stops need to be sorted

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LinearGradient<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::colr::NonVarColorLine

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `extend: GradientExtend`
- `colors: crate::parser::LazyArray16<'a, ColorStopRaw>`
- `palettes: cpal::Table<'a>`
- `foreground_color: crate::RgbaColor`

**Methods:**

- `fn get(self: &Self, palette: u16, index: u16) -> Option<ColorStop>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NonVarColorLine<'a>`



## ttf_parser::tables::colr::Paint

*Enum*

A paint.

**Generic Parameters:**
- 'a

**Variants:**
- `Solid(crate::RgbaColor)` - A paint with a solid color.
- `LinearGradient(LinearGradient<'a>)` - A paint with a linear gradient.
- `RadialGradient(RadialGradient<'a>)` - A paint with a radial gradient.
- `SweepGradient(SweepGradient<'a>)` - A paint with a sweep gradient.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Paint<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::colr::Painter

*Trait*

A trait for color glyph painting.

See [COLR](https://learn.microsoft.com/en-us/typography/opentype/spec/colr) for details.

**Methods:**

- `outline_glyph`: Outline a glyph and store it.
- `paint`: Paint the stored outline using the provided color.
- `push_clip`: Push a new clip path using the currently stored outline.
- `push_clip_box`: Push a new clip path using the clip box.
- `pop_clip`: Pop the last clip path.
- `push_layer`: Push a new layer with the given composite mode.
- `pop_layer`: Pop the last layer.
- `push_transform`: Push a transform.
- `pop_transform`: Pop the last transform.



## ttf_parser::tables::colr::RadialGradient

*Struct*

A [radial gradient](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-6-and-7-paintradialgradient-paintvarradialgradient)

**Generic Parameters:**
- 'a

**Fields:**
- `x0: f32` - The `x0` value.
- `y0: f32` - The `y0` value.
- `r0: f32` - The `r0` value.
- `r1: f32` - The `r1` value.
- `x1: f32` - The `x1` value.
- `y1: f32` - The `y1` value.
- `extend: GradientExtend` - The extend.
- `color_line: ColorLine<'a>`

**Methods:**

- `fn stops<'b>(self: &'b Self, palette: u16) -> GradientStopsIter<'a, 'b>` - Returns an iterator over the stops of the radial gradient. Stops need to be sorted

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RadialGradient<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::colr::RecursionStack

*Struct*

**Fields:**
- `stack: [usize; 64]`
- `len: usize`

**Methods:**

- `fn is_empty(self: &Self) -> bool`
- `fn push(self: & mut Self, offset: usize) -> Result<(), ()>`
- `fn contains(self: &Self, offset: usize) -> bool`
- `fn pop(self: & mut Self)`



## ttf_parser::tables::colr::SweepGradient

*Struct*

A [sweep gradient](https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-8-and-9-paintsweepgradient-paintvarsweepgradient)

**Generic Parameters:**
- 'a

**Fields:**
- `center_x: f32` - The x of the center.
- `center_y: f32` - The y of the center.
- `start_angle: f32` - The start angle.
- `end_angle: f32` - The end angle.
- `extend: GradientExtend` - The extend.
- `color_line: ColorLine<'a>`

**Methods:**

- `fn stops<'b>(self: &'b Self, palette: u16) -> GradientStopsIter<'a, 'b>` - Returns an iterator over the stops of the sweep gradient. Stops need to be sorted

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SweepGradient<'a>`



## ttf_parser::tables::colr::Table

*Struct*

A [Color Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/colr).

Currently, only version 0 is supported.

**Generic Parameters:**
- 'a

**Fields:**
- `palettes: cpal::Table<'a>`
- `data: &'a [u8]`
- `version: u8`
- `base_glyphs: crate::parser::LazyArray16<'a, BaseGlyphRecord>`
- `layers: crate::parser::LazyArray16<'a, LayerRecord>`
- `base_glyph_paints_offset: crate::parser::Offset32`
- `base_glyph_paints: crate::LazyArray32<'a, BaseGlyphPaintRecord>`
- `layer_paint_offsets_offset: crate::parser::Offset32`
- `layer_paint_offsets: crate::LazyArray32<'a, crate::parser::Offset32>`
- `clip_list_offsets_offset: crate::parser::Offset32`
- `clip_list: ClipList<'a>`

**Methods:**

- `fn parse(palettes: cpal::Table<'a>, data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn is_simple(self: &Self) -> bool` - Returns `true` if the current table has version 0.
- `fn get_v0(self: &Self, glyph_id: GlyphId) -> Option<BaseGlyphRecord>`
- `fn get_v1(self: &Self, glyph_id: GlyphId) -> Option<BaseGlyphPaintRecord>`
- `fn contains(self: &Self, glyph_id: GlyphId) -> bool` - Whether the table contains a definition for the given glyph.
- `fn clip_box(self: &Self, glyph_id: GlyphId) -> Option<ClipBox>` - Returns the clip box for a glyph.
- `fn paint(self: &Self, glyph_id: GlyphId, palette: u16, painter: & mut dyn Painter, foreground_color: RgbaColor) -> Option<()>` - Paints the color glyph.
- `fn paint_impl(self: &Self, glyph_id: GlyphId, palette: u16, painter: & mut dyn Painter, recursion_stack: & mut RecursionStack, foreground_color: RgbaColor) -> Option<()>`
- `fn paint_v0(self: &Self, base: BaseGlyphRecord, palette: u16, painter: & mut dyn Painter, foreground_color: RgbaColor) -> Option<()>`
- `fn paint_v1(self: &Self, base: BaseGlyphPaintRecord, palette: u16, painter: & mut dyn Painter, recursion_stack: & mut RecursionStack, foreground_color: RgbaColor) -> Option<()>`
- `fn parse_paint(self: &Self, offset: usize, palette: u16, painter: & mut dyn Painter, recursion_stack: & mut RecursionStack, foreground_color: RgbaColor) -> Option<()>`
- `fn parse_paint_impl(self: &Self, offset: usize, palette: u16, painter: & mut dyn Painter, recursion_stack: & mut RecursionStack, s: & mut Stream, format: u8, foreground_color: RgbaColor) -> Option<()>`
- `fn parse_color_line(self: &Self, offset: usize, foreground_color: RgbaColor) -> Option<NonVarColorLine<'a>>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



