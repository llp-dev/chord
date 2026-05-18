**ttf_parser**

# Module: ttf_parser

## Contents

**Modules**

- [`language`](#language)
- [`parser`](#parser) - Binary parsing utils.
- [`tables`](#tables)

**Structs**

- [`DummyOutline`](#dummyoutline)
- [`Face`](#face) - A font face.
- [`FaceTables`](#facetables) - Parsed face tables.
- [`GlyphId`](#glyphid) - A type-safe wrapper for glyph ID.
- [`LineMetrics`](#linemetrics) - A line metrics.
- [`NormalizedCoordinate`](#normalizedcoordinate) - A variation coordinate in a normalized coordinate system.
- [`PhantomPoints`](#phantompoints) - Phantom points.
- [`PointF`](#pointf) - A float point.
- [`RasterGlyphImage`](#rasterglyphimage) - A glyph's raster image.
- [`RawFace`](#rawface) - A raw font face.
- [`RawFaceTables`](#rawfacetables) - A list of all supported tables as raw data.
- [`Rect`](#rect) - A rectangle.
- [`RectF`](#rectf) - A rectangle described by the left-lower and upper-right points.
- [`RgbaColor`](#rgbacolor) - A RGBA color in the sRGB color space.
- [`TableRecord`](#tablerecord) - A raw table record.
- [`Tag`](#tag) - A 4-byte tag.
- [`Transform`](#transform) - An affine transform.
- [`Variation`](#variation) - A font variation value.

**Enums**

- [`FaceParsingError`](#faceparsingerror) - A list of font face parsing errors.
- [`Magic`](#magic) - A TrueType font magic.
- [`RasterImageFormat`](#rasterimageformat) - A glyph raster image format.

**Functions**

- [`fonts_in_collection`](#fonts_in_collection) - Returns the number of fonts stored in a TrueType font collection.

**Traits**

- [`OutlineBuilder`](#outlinebuilder) - A trait for glyph outline construction.

---

## ttf_parser::DummyOutline

*Struct*

**Unit Struct**

**Trait Implementations:**

- **OutlineBuilder**
  - `fn move_to(self: & mut Self, _: f32, _: f32)`
  - `fn line_to(self: & mut Self, _: f32, _: f32)`
  - `fn quad_to(self: & mut Self, _: f32, _: f32, _: f32, _: f32)`
  - `fn curve_to(self: & mut Self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32)`
  - `fn close(self: & mut Self)`



## ttf_parser::Face

*Struct*

A font face.

Provides a high-level API for working with TrueType fonts.
If you're not familiar with how TrueType works internally, you should use this type.
If you do know and want a bit more low-level access - checkout [`FaceTables`].

Note that `Face` doesn't own the font data and doesn't allocate anything in heap.
Therefore you cannot "store" it. The idea is that you should parse the `Face`
when needed, get required data and forget about it.
That's why the initial parsing is highly optimized and should not become a bottleneck.

If you still want to store `Face` - checkout
[owned_ttf_parser](https://crates.io/crates/owned_ttf_parser). Requires `unsafe`.

While `Face` is technically copyable, we disallow it because it's almost 2KB big.

**Generic Parameters:**
- 'a

**Fields:**
- `raw_face: RawFace<'a>`
- `tables: FaceTables<'a>`

**Methods:**

- `fn from_slice(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError>` - Creates a new [`Face`] from a raw data.
- `fn parse(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError>` - Creates a new [`Face`] from a raw data.
- `fn collect_tables(raw_face: RawFace<'a>) -> RawFaceTables<'a>`
- `fn from_raw_tables(raw_tables: RawFaceTables<'a>) -> Result<Self, FaceParsingError>` - Creates a new [`Face`] from provided [`RawFaceTables`].
- `fn parse_tables(raw_tables: RawFaceTables<'a>) -> Result<FaceTables<'a>, FaceParsingError>`
- `fn tables(self: &Self) -> &FaceTables<'a>` - Returns low-level face tables.
- `fn raw_face(self: &Self) -> &RawFace<'a>` - Returns the `RawFace` used to create this `Face`.
- `fn table_data(self: &Self, tag: Tag) -> Option<&'a [u8]>` - Returns the raw data of a selected table.
- `fn names(self: &Self) -> name::Names<'a>` - Returns a list of names.
- `fn is_regular(self: &Self) -> bool` - Checks that face is marked as *Regular*.
- `fn is_italic(self: &Self) -> bool` - Checks that face is marked as *Italic*.
- `fn is_bold(self: &Self) -> bool` - Checks that face is marked as *Bold*.
- `fn is_oblique(self: &Self) -> bool` - Checks that face is marked as *Oblique*.
- `fn style(self: &Self) -> Style` - Returns face style.
- `fn is_monospaced(self: &Self) -> bool` - Checks that face is marked as *Monospaced*.
- `fn is_variable(self: &Self) -> bool` - Checks that face is variable.
- `fn weight(self: &Self) -> Weight` - Returns face's weight.
- `fn width(self: &Self) -> Width` - Returns face's width.
- `fn italic_angle(self: &Self) -> f32` - Returns face's italic angle.
- `fn ascender(self: &Self) -> i16` - Returns a horizontal face ascender.
- `fn descender(self: &Self) -> i16` - Returns a horizontal face descender.
- `fn height(self: &Self) -> i16` - Returns face's height.
- `fn line_gap(self: &Self) -> i16` - Returns a horizontal face line gap.
- `fn typographic_ascender(self: &Self) -> Option<i16>` - Returns a horizontal typographic face ascender.
- `fn typographic_descender(self: &Self) -> Option<i16>` - Returns a horizontal typographic face descender.
- `fn typographic_line_gap(self: &Self) -> Option<i16>` - Returns a horizontal typographic face line gap.
- `fn vertical_ascender(self: &Self) -> Option<i16>` - Returns a vertical face ascender.
- `fn vertical_descender(self: &Self) -> Option<i16>` - Returns a vertical face descender.
- `fn vertical_height(self: &Self) -> Option<i16>` - Returns a vertical face height.
- `fn vertical_line_gap(self: &Self) -> Option<i16>` - Returns a vertical face line gap.
- `fn units_per_em(self: &Self) -> u16` - Returns face's units per EM.
- `fn x_height(self: &Self) -> Option<i16>` - Returns face's x height.
- `fn capital_height(self: &Self) -> Option<i16>` - Returns face's capital height.
- `fn underline_metrics(self: &Self) -> Option<LineMetrics>` - Returns face's underline metrics.
- `fn strikeout_metrics(self: &Self) -> Option<LineMetrics>` - Returns face's strikeout metrics.
- `fn subscript_metrics(self: &Self) -> Option<ScriptMetrics>` - Returns face's subscript metrics.
- `fn superscript_metrics(self: &Self) -> Option<ScriptMetrics>` - Returns face's superscript metrics.
- `fn permissions(self: &Self) -> Option<Permissions>` - Returns face permissions.
- `fn is_subsetting_allowed(self: &Self) -> bool` - Checks if the face allows embedding a subset, further restricted by [`Self::permissions`].
- `fn is_outline_embedding_allowed(self: &Self) -> bool` - Checks if the face allows outline data to be embedded.
- `fn unicode_ranges(self: &Self) -> UnicodeRanges` - Returns [Unicode Ranges](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#ur).
- `fn number_of_glyphs(self: &Self) -> u16` - Returns a total number of glyphs in the face.
- `fn glyph_index(self: &Self, code_point: char) -> Option<GlyphId>` - Resolves a Glyph ID for a code point.
- `fn glyph_variation_index(self: &Self, code_point: char, variation: char) -> Option<GlyphId>` - Resolves a variation of a Glyph ID from two code points.
- `fn glyph_hor_advance(self: &Self, glyph_id: GlyphId) -> Option<u16>` - Returns glyph's horizontal advance.
- `fn glyph_ver_advance(self: &Self, glyph_id: GlyphId) -> Option<u16>` - Returns glyph's vertical advance.
- `fn glyph_hor_side_bearing(self: &Self, glyph_id: GlyphId) -> Option<i16>` - Returns glyph's horizontal side bearing.
- `fn glyph_ver_side_bearing(self: &Self, glyph_id: GlyphId) -> Option<i16>` - Returns glyph's vertical side bearing.
- `fn glyph_y_origin(self: &Self, glyph_id: GlyphId) -> Option<i16>` - Returns glyph's vertical origin according to
- `fn outline_glyph(self: &Self, glyph_id: GlyphId, builder: & mut dyn OutlineBuilder) -> Option<Rect>` - Outlines a glyph and returns its tight bounding box.
- `fn glyph_bounding_box(self: &Self, glyph_id: GlyphId) -> Option<Rect>` - Returns a tight glyph bounding box.
- `fn global_bounding_box(self: &Self) -> Rect` - Returns a bounding box that large enough to enclose any glyph from the face.
- `fn glyph_raster_image(self: &Self, glyph_id: GlyphId, pixels_per_em: u16) -> Option<RasterGlyphImage>` - Returns a reference to a glyph's raster image.
- `fn glyph_svg_image(self: &Self, glyph_id: GlyphId) -> Option<svg::SvgDocument<'a>>` - Returns a reference to a glyph's SVG image.
- `fn is_color_glyph(self: &Self, glyph_id: GlyphId) -> bool` - Returns `true` if the glyph can be colored/painted using the `COLR`+`CPAL` tables.
- `fn color_palettes(self: &Self) -> Option<core::num::NonZeroU16>` - Returns the number of palettes stored in the `COLR`+`CPAL` tables.
- `fn paint_color_glyph(self: &Self, glyph_id: GlyphId, palette: u16, foreground_color: RgbaColor, painter: & mut dyn colr::Painter) -> Option<()>` - Paints a color glyph from the `COLR` table.
- `fn apply_metrics_variation(self: &Self, tag: Tag, value: i16) -> i16`
- `fn apply_metrics_variation_to(self: &Self, _: Tag, _: & mut i16)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Face<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::FaceParsingError

*Enum*

A list of font face parsing errors.

**Variants:**
- `MalformedFont` - An attempt to read out of bounds detected.
- `UnknownMagic` - Face data must start with `0x00010000`, `0x74727565`, `0x4F54544F` or `0x74746366`.
- `FaceIndexOutOfBounds` - The face index is larger than the number of faces in the font.
- `NoHeadTable` - The `head` table is missing or malformed.
- `NoHheaTable` - The `hhea` table is missing or malformed.
- `NoMaxpTable` - The `maxp` table is missing or malformed.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &FaceParsingError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FaceParsingError`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::FaceTables

*Struct*

Parsed face tables.

Unlike [`Face`], provides a low-level parsing abstraction over TrueType tables.
Useful when you need a direct access to tables data.

Also, used when high-level API is problematic to implement.
A good example would be OpenType layout tables (GPOS/GSUB).

**Generic Parameters:**
- 'a

**Fields:**
- `head: head::Table`
- `hhea: hhea::Table`
- `maxp: maxp::Table`
- `bdat: Option<cbdt::Table<'a>>`
- `cbdt: Option<cbdt::Table<'a>>`
- `cff: Option<cff::Table<'a>>`
- `cmap: Option<cmap::Table<'a>>`
- `colr: Option<colr::Table<'a>>`
- `ebdt: Option<cbdt::Table<'a>>`
- `glyf: Option<glyf::Table<'a>>`
- `hmtx: Option<hmtx::Table<'a>>`
- `kern: Option<kern::Table<'a>>`
- `name: Option<name::Table<'a>>`
- `os2: Option<os2::Table<'a>>`
- `post: Option<post::Table<'a>>`
- `sbix: Option<sbix::Table<'a>>`
- `stat: Option<stat::Table<'a>>`
- `svg: Option<svg::Table<'a>>`
- `vhea: Option<vhea::Table>`
- `vmtx: Option<hmtx::Table<'a>>`
- `vorg: Option<vorg::Table<'a>>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FaceTables<'a>`



## ttf_parser::GlyphId

*Struct*

A type-safe wrapper for glyph ID.

**Tuple Struct**: `(u16)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> GlyphId`
- **Ord**
  - `fn cmp(self: &Self, other: &GlyphId) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GlyphId) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GlyphId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Default**
  - `fn default() -> GlyphId`



## ttf_parser::LineMetrics

*Struct*

A line metrics.

Used for underline and strikeout.

**Fields:**
- `position: i16` - Line position.
- `thickness: i16` - Line thickness.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &LineMetrics) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> LineMetrics`



## ttf_parser::Magic

*Enum*

A TrueType font magic.

https://docs.microsoft.com/en-us/typography/opentype/spec/otff#organization-of-an-opentype-font

**Variants:**
- `TrueType`
- `OpenType`
- `FontCollection`

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Magic) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Magic`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::NormalizedCoordinate

*Struct*

A variation coordinate in a normalized coordinate system.

Basically any number in a -1.0..1.0 range.
Where 0 is a default value.

The number is stored as f2.16

**Tuple Struct**: `(i16)`

**Methods:**

- `fn get(self: Self) -> i16` - Returns the coordinate value as f2.14.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(n: i16) -> Self` - Creates a new coordinate.
- **PartialEq**
  - `fn eq(self: &Self, other: &NormalizedCoordinate) -> bool`
- **Default**
  - `fn default() -> NormalizedCoordinate`
- **From**
  - `fn from(n: f32) -> Self` - Creates a new coordinate.
- **Clone**
  - `fn clone(self: &Self) -> NormalizedCoordinate`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::OutlineBuilder

*Trait*

A trait for glyph outline construction.

**Methods:**

- `move_to`: Appends a MoveTo segment.
- `line_to`: Appends a LineTo segment.
- `quad_to`: Appends a QuadTo segment.
- `curve_to`: Appends a CurveTo segment.
- `close`: Appends a ClosePath segment.



## ttf_parser::PhantomPoints

*Struct*

Phantom points.

Available only for variable fonts with the `gvar` table.

**Fields:**
- `left: PointF` - Left side bearing point.
- `right: PointF` - Right side bearing point.
- `top: PointF` - Top side bearing point.
- `bottom: PointF` - Bottom side bearing point.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PhantomPoints`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::PointF

*Struct*

A float point.

**Fields:**
- `x: f32` - The X-axis coordinate.
- `y: f32` - The Y-axis coordinate.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PointF`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::RasterGlyphImage

*Struct*

A glyph's raster image.

Note, that glyph metrics are in pixels and not in font units.

**Generic Parameters:**
- 'a

**Fields:**
- `x: i16` - Horizontal offset.
- `y: i16` - Vertical offset.
- `width: u16` - Image width.
- `height: u16` - Image height.
- `pixels_per_em: u16` - A pixels per em of the selected strike.
- `format: RasterImageFormat` - An image format.
- `data: &'a [u8]` - A raw image data. It's up to the caller to decode it.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RasterGlyphImage<'a>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RasterGlyphImage<'a>`



## ttf_parser::RasterImageFormat

*Enum*

A glyph raster image format.

**Variants:**
- `PNG`
- `BitmapMono` - A monochrome bitmap.
- `BitmapMonoPacked` - A packed monochrome bitmap.
- `BitmapGray2` - A grayscale bitmap with 2 bits per pixel.
- `BitmapGray2Packed` - A packed grayscale bitmap with 2 bits per pixel.
- `BitmapGray4` - A grayscale bitmap with 4 bits per pixel.
- `BitmapGray4Packed` - A packed grayscale bitmap with 4 bits per pixel.
- `BitmapGray8` - A grayscale bitmap with 8 bits per pixel.
- `BitmapPremulBgra32` - A color bitmap with 32 bits per pixel.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RasterImageFormat) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RasterImageFormat`



## ttf_parser::RawFace

*Struct*

A raw font face.

You are probably looking for [`Face`]. This is a low-level type.

Unlike [`Face`], [`RawFace`] parses only face table records.
Meaning all you can get from this type is a raw (`&[u8]`) data of a requested table.
Then you can either parse just a singe table from a font/face or populate [`RawFaceTables`]
manually before passing it to [`Face::from_raw_tables`].

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]` - The input font file data.
- `table_records: LazyArray16<'a, TableRecord>` - An array of table records.

**Methods:**

- `fn from_slice(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError>` - Creates a new [`RawFace`] from a raw data.
- `fn parse(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError>` - Creates a new [`RawFace`] from a raw data.
- `fn table(self: &Self, tag: Tag) -> Option<&'a [u8]>` - Returns the raw data of a selected table.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RawFace<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::RawFaceTables

*Struct*

A list of all supported tables as raw data.

This type should be used in tandem with
[`Face::from_raw_tables()`](struct.Face.html#method.from_raw_tables).

This allows loading font faces not only from TrueType font files,
but from any source. Mainly used for parsing WOFF.

**Generic Parameters:**
- 'a

**Fields:**
- `head: &'a [u8]`
- `hhea: &'a [u8]`
- `maxp: &'a [u8]`
- `bdat: Option<&'a [u8]>`
- `bloc: Option<&'a [u8]>`
- `cbdt: Option<&'a [u8]>`
- `cblc: Option<&'a [u8]>`
- `cff: Option<&'a [u8]>`
- `cmap: Option<&'a [u8]>`
- `colr: Option<&'a [u8]>`
- `cpal: Option<&'a [u8]>`
- `ebdt: Option<&'a [u8]>`
- `eblc: Option<&'a [u8]>`
- `glyf: Option<&'a [u8]>`
- `hmtx: Option<&'a [u8]>`
- `kern: Option<&'a [u8]>`
- `loca: Option<&'a [u8]>`
- `name: Option<&'a [u8]>`
- `os2: Option<&'a [u8]>`
- `post: Option<&'a [u8]>`
- `sbix: Option<&'a [u8]>`
- `stat: Option<&'a [u8]>`
- `svg: Option<&'a [u8]>`
- `vhea: Option<&'a [u8]>`
- `vmtx: Option<&'a [u8]>`
- `vorg: Option<&'a [u8]>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RawFaceTables<'a>`
- **Default**
  - `fn default() -> RawFaceTables<'a>`



## ttf_parser::Rect

*Struct*

A rectangle.

Doesn't guarantee that `x_min` <= `x_max` and/or `y_min` <= `y_max`.

**Fields:**
- `x_min: i16`
- `y_min: i16`
- `x_max: i16`
- `y_max: i16`

**Methods:**

- `fn zero() -> Self`
- `fn width(self: &Self) -> i16` - Returns rect's width.
- `fn height(self: &Self) -> i16` - Returns rect's height.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Rect) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Rect`



## ttf_parser::RectF

*Struct*

A rectangle described by the left-lower and upper-right points.

**Fields:**
- `x_min: f32` - The horizontal minimum of the rect.
- `y_min: f32` - The vertical minimum of the rect.
- `x_max: f32` - The horizontal maximum of the rect.
- `y_max: f32` - The vertical maximum of the rect.

**Methods:**

- `fn new() -> Self`
- `fn is_default(self: &Self) -> bool`
- `fn extend_by(self: & mut Self, x: f32, y: f32)`
- `fn to_rect(self: Self) -> Option<Rect>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RectF`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RectF) -> bool`



## ttf_parser::RgbaColor

*Struct*

A RGBA color in the sRGB color space.

**Fields:**
- `red: u8`
- `green: u8`
- `blue: u8`
- `alpha: u8`

**Methods:**

- `fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self` - Creates a new `RgbaColor`.
- `fn apply_alpha(self: & mut Self, alpha: f32)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RgbaColor`
- **PartialEq**
  - `fn eq(self: &Self, other: &RgbaColor) -> bool`



## ttf_parser::TableRecord

*Struct*

A raw table record.

**Fields:**
- `tag: Tag`
- `check_sum: u32`
- `offset: u32`
- `length: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TableRecord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::Tag

*Struct*

A 4-byte tag.

**Tuple Struct**: `(u32)`

**Methods:**

- `fn from_bytes(bytes: &[u8; 4]) -> Self` - Creates a `Tag` from bytes.
- `fn from_bytes_lossy(bytes: &[u8]) -> Self` - Creates a `Tag` from bytes.
- `fn to_bytes(self: Self) -> [u8; 4]` - Returns tag as 4-element byte array.
- `fn to_chars(self: Self) -> [char; 4]` - Returns tag as 4-element byte array.
- `fn is_null(self: &Self) -> bool` - Checks if tag is null / `[0, 0, 0, 0]`.
- `fn as_u32(self: &Self) -> u32` - Returns tag value as `u32` number.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Tag) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Tag) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Tag`
- **Ord**
  - `fn cmp(self: &Self, other: &Tag) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::Transform

*Struct*

An affine transform.

**Fields:**
- `a: f32` - The 'a' component of the transform.
- `b: f32` - The 'b' component of the transform.
- `c: f32` - The 'c' component of the transform.
- `d: f32` - The 'd' component of the transform.
- `e: f32` - The 'e' component of the transform.
- `f: f32` - The 'f' component of the transform.

**Methods:**

- `fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self` - Creates a new transform with the specified components.
- `fn new_translate(tx: f32, ty: f32) -> Self` - Creates a new translation transform.
- `fn new_rotate(angle: f32) -> Self` - Creates a new rotation transform.
- `fn new_skew(skew_x: f32, skew_y: f32) -> Self` - Creates a new skew transform.
- `fn new_scale(sx: f32, sy: f32) -> Self` - Creates a new scale transform.
- `fn combine(ts1: Self, ts2: Self) -> Self` - Combines two transforms with each other.
- `fn apply_to(self: &Self, x: & mut f32, y: & mut f32)`
- `fn is_default(self: &Self) -> bool` - Checks whether a transform is the identity transform.

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Transform) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Transform`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::Variation

*Struct*

A font variation value.

# Example

```
use ttf_parser::{Variation, Tag};

Variation { axis: Tag::from_bytes(b"wght"), value: 500.0 };
```

**Fields:**
- `axis: Tag` - An axis tag name.
- `value: f32` - An axis value.

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Variation) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Variation`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::fonts_in_collection

*Function*

Returns the number of fonts stored in a TrueType font collection.

Returns `None` if a provided data is not a TrueType font collection.

```rust
fn fonts_in_collection(data: &[u8]) -> Option<u32>
```



## Module: language



## Module: parser

Binary parsing utils.

This module should not be used directly, unless you're planning to parse
some tables manually.



## Module: tables



