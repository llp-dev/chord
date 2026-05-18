**ttf_parser > tables > cff > cff1**

# Module: tables::cff::cff1

## Contents

**Modules**

- [`charset_id`](#charset_id) - Enumerates Charset IDs defined in the Adobe Technical Note #5176, Table 22
- [`encoding_id`](#encoding_id) - Enumerates Charset IDs defined in the Adobe Technical Note #5176, Table 16
- [`operator`](#operator) - Enumerates some operators defined in the Adobe Technical Note #5177.
- [`private_dict_operator`](#private_dict_operator) - Enumerates some operators defined in the Adobe Technical Note #5176,
- [`top_dict_operator`](#top_dict_operator) - Enumerates some operators defined in the Adobe Technical Note #5176,

**Structs**

- [`CIDMetadata`](#cidmetadata)
- [`CharStringParserContext`](#charstringparsercontext)
- [`Matrix`](#matrix) - An affine transformation matrix.
- [`PrivateDict`](#privatedict)
- [`SIDMetadata`](#sidmetadata)
- [`Table`](#table) - A [Compact Font Format Table](
- [`TopDict`](#topdict)

**Enums**

- [`FDSelect`](#fdselect)
- [`FontKind`](#fontkind)

**Functions**

- [`_parse_char_string`](#_parse_char_string)
- [`parse_char_string`](#parse_char_string)
- [`parse_cid_local_subrs`](#parse_cid_local_subrs) - In CID fonts, to get local subroutines we have to:
- [`parse_cid_metadata`](#parse_cid_metadata)
- [`parse_fd_select`](#parse_fd_select)
- [`parse_font_dict`](#parse_font_dict)
- [`parse_private_dict`](#parse_private_dict)
- [`parse_sid_metadata`](#parse_sid_metadata)
- [`parse_top_dict`](#parse_top_dict)
- [`seac_code_to_glyph_id`](#seac_code_to_glyph_id)

**Constants**

- [`MAX_ARGUMENTS_STACK_LEN`](#max_arguments_stack_len)
- [`MAX_OPERANDS_LEN`](#max_operands_len)
- [`STACK_LIMIT`](#stack_limit)
- [`TWO_BYTE_OPERATOR_MARK`](#two_byte_operator_mark)

---

## ttf_parser::tables::cff::cff1::CIDMetadata

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `fd_array: super::index::Index<'a>`
- `fd_select: FDSelect<'a>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CIDMetadata<'a>`
- **Default**
  - `fn default() -> CIDMetadata<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::cff1::CharStringParserContext

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `metadata: &'a Table<'a>`
- `width: Option<f32>`
- `stems_len: u32`
- `has_endchar: bool`
- `has_seac: bool`
- `glyph_id: crate::GlyphId`
- `local_subrs: Option<super::index::Index<'a>>`



## ttf_parser::tables::cff::cff1::FDSelect

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Format0(crate::parser::LazyArray16<'a, u8>)`
- `Format3(&'a [u8])`

**Methods:**

- `fn font_dict_index(self: &Self, glyph_id: GlyphId) -> Option<u8>`

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> FDSelect<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::cff1::FontKind

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `SID(SIDMetadata<'a>)`
- `CID(CIDMetadata<'a>)`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FontKind<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::cff1::MAX_ARGUMENTS_STACK_LEN

*Constant*: `usize`



## ttf_parser::tables::cff::cff1::MAX_OPERANDS_LEN

*Constant*: `usize`



## ttf_parser::tables::cff::cff1::Matrix

*Struct*

An affine transformation matrix.

**Fields:**
- `sx: f32`
- `ky: f32`
- `kx: f32`
- `sy: f32`
- `tx: f32`
- `ty: f32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Matrix`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## ttf_parser::tables::cff::cff1::PrivateDict

*Struct*

**Fields:**
- `local_subroutines_offset: Option<usize>`
- `default_width: Option<f32>`
- `nominal_width: Option<f32>`

**Trait Implementations:**

- **Default**
  - `fn default() -> PrivateDict`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::cff1::SIDMetadata

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `local_subrs: super::index::Index<'a>`
- `default_width: f32` - Can be zero.
- `nominal_width: f32` - Can be zero.
- `encoding: super::encoding::Encoding<'a>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SIDMetadata<'a>`
- **Default**
  - `fn default() -> SIDMetadata<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::cff1::STACK_LIMIT

*Constant*: `u8`



## ttf_parser::tables::cff::cff1::TWO_BYTE_OPERATOR_MARK

*Constant*: `u8`



## ttf_parser::tables::cff::cff1::Table

*Struct*

A [Compact Font Format Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cff).

**Generic Parameters:**
- 'a

**Fields:**
- `table_data: &'a [u8]`
- `strings: super::index::Index<'a>`
- `global_subrs: super::index::Index<'a>`
- `charset: super::charset::Charset<'a>`
- `number_of_glyphs: core::num::NonZeroU16`
- `matrix: Matrix`
- `char_strings: super::index::Index<'a>`
- `kind: FontKind<'a>`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn number_of_glyphs(self: &Self) -> u16` - Returns a total number of glyphs in the font.
- `fn matrix(self: &Self) -> Matrix` - Returns a font transformation matrix.
- `fn outline(self: &Self, glyph_id: GlyphId, builder: & mut dyn OutlineBuilder) -> Result<Rect, CFFError>` - Outlines a glyph.
- `fn glyph_index(self: &Self, code_point: u8) -> Option<GlyphId>` - Resolves a Glyph ID for a code point.
- `fn glyph_width(self: &Self, glyph_id: GlyphId) -> Option<u16>` - Returns a glyph width.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::cff::cff1::TopDict

*Struct*

**Fields:**
- `charset_offset: Option<usize>`
- `encoding_offset: Option<usize>`
- `char_strings_offset: usize`
- `private_dict_range: Option<core::ops::Range<usize>>`
- `matrix: Matrix`
- `has_ros: bool`
- `fd_array_offset: Option<usize>`
- `fd_select_offset: Option<usize>`

**Trait Implementations:**

- **Default**
  - `fn default() -> TopDict`



## ttf_parser::tables::cff::cff1::_parse_char_string

*Function*

```rust
fn _parse_char_string(ctx: & mut CharStringParserContext, char_string: &[u8], depth: u8, p: & mut super::charstring::CharStringParser) -> Result<(), super::CFFError>
```



## Module: charset_id

Enumerates Charset IDs defined in the Adobe Technical Note #5176, Table 22



## Module: encoding_id

Enumerates Charset IDs defined in the Adobe Technical Note #5176, Table 16



## Module: operator

Enumerates some operators defined in the Adobe Technical Note #5177.



## ttf_parser::tables::cff::cff1::parse_char_string

*Function*

```rust
fn parse_char_string(data: &[u8], metadata: &Table, glyph_id: crate::GlyphId, width_only: bool, builder: & mut dyn OutlineBuilder) -> Result<(crate::Rect, Option<f32>), super::CFFError>
```



## ttf_parser::tables::cff::cff1::parse_cid_local_subrs

*Function*

In CID fonts, to get local subroutines we have to:
  1. Find Font DICT index via FDSelect by GID.
  2. Get Font DICT data from FDArray using this index.
  3. Get a Private DICT offset from a Font DICT.
  4. Get a local subroutine offset from Private DICT.
  5. Parse a local subroutine at offset.

```rust
fn parse_cid_local_subrs<'a>(data: &'a [u8], glyph_id: crate::GlyphId, cid: &CIDMetadata) -> Option<super::index::Index<'a>>
```



## ttf_parser::tables::cff::cff1::parse_cid_metadata

*Function*

```rust
fn parse_cid_metadata(data: &[u8], top_dict: TopDict, number_of_glyphs: u16) -> Option<FontKind>
```



## ttf_parser::tables::cff::cff1::parse_fd_select

*Function*

```rust
fn parse_fd_select<'a>(number_of_glyphs: u16, s: & mut crate::parser::Stream<'a>) -> Option<FDSelect<'a>>
```



## ttf_parser::tables::cff::cff1::parse_font_dict

*Function*

```rust
fn parse_font_dict(data: &[u8]) -> Option<core::ops::Range<usize>>
```



## ttf_parser::tables::cff::cff1::parse_private_dict

*Function*

```rust
fn parse_private_dict(data: &[u8]) -> PrivateDict
```



## ttf_parser::tables::cff::cff1::parse_sid_metadata

*Function*

```rust
fn parse_sid_metadata<'a>(data: &'a [u8], top_dict: TopDict, encoding: super::encoding::Encoding<'a>) -> Option<FontKind<'a>>
```



## ttf_parser::tables::cff::cff1::parse_top_dict

*Function*

```rust
fn parse_top_dict(s: & mut crate::parser::Stream) -> Option<TopDict>
```



## Module: private_dict_operator

Enumerates some operators defined in the Adobe Technical Note #5176,
Table 23 Private DICT Operators



## ttf_parser::tables::cff::cff1::seac_code_to_glyph_id

*Function*

```rust
fn seac_code_to_glyph_id(charset: &super::charset::Charset, n: f32) -> Option<crate::GlyphId>
```



## Module: top_dict_operator

Enumerates some operators defined in the Adobe Technical Note #5176,
Table 9 Top DICT Operator Entries



