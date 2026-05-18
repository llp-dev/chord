**ttf_parser > tables > cblc**

# Module: tables::cblc

## Contents

**Structs**

- [`BitmapFormat`](#bitmapformat)
- [`BitmapSizeTable`](#bitmapsizetable)
- [`GlyphIdOffsetPair`](#glyphidoffsetpair)
- [`IndexSubtableInfo`](#indexsubtableinfo)
- [`Location`](#location)
- [`Metrics`](#metrics)
- [`Table`](#table) - A [Color Bitmap Location Table](

**Enums**

- [`BitmapDataFormat`](#bitmapdataformat)
- [`MetricsFormat`](#metricsformat)

**Functions**

- [`select_bitmap_size_table`](#select_bitmap_size_table)
- [`select_index_subtable`](#select_index_subtable)

---

## ttf_parser::tables::cblc::BitmapDataFormat

*Enum*

**Variants:**
- `ByteAligned{ bit_depth: u8 }`
- `BitAligned{ bit_depth: u8 }`
- `PNG`

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &BitmapDataFormat) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BitmapDataFormat`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cblc::BitmapFormat

*Struct*

**Fields:**
- `metrics: MetricsFormat`
- `data: BitmapDataFormat`

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &BitmapFormat) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BitmapFormat`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cblc::BitmapSizeTable

*Struct*

**Fields:**
- `subtable_array_offset: crate::parser::Offset32`
- `number_of_subtables: u32`
- `ppem: u16`
- `bit_depth: u8`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> BitmapSizeTable`



## ttf_parser::tables::cblc::GlyphIdOffsetPair

*Struct*

**Fields:**
- `glyph_id: crate::GlyphId`
- `offset: crate::parser::Offset16`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> GlyphIdOffsetPair`



## ttf_parser::tables::cblc::IndexSubtableInfo

*Struct*

**Fields:**
- `start_glyph_id: crate::GlyphId`
- `offset: usize`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> IndexSubtableInfo`



## ttf_parser::tables::cblc::Location

*Struct*

**Fields:**
- `format: BitmapFormat`
- `offset: usize`
- `metrics: Metrics`
- `ppem: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Location`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cblc::Metrics

*Struct*

**Fields:**
- `x: i8`
- `y: i8`
- `width: u8`
- `height: u8`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Metrics`
- **Default**
  - `fn default() -> Metrics`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cblc::MetricsFormat

*Enum*

**Variants:**
- `Small`
- `Big`
- `Shared`

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &MetricsFormat) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> MetricsFormat`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cblc::Table

*Struct*

A [Color Bitmap Location Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cblc).

EBLC and bloc also share the same structure, so this is re-used for them.

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn get(self: &Self, glyph_id: GlyphId, pixels_per_em: u16) -> Option<Location>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::cblc::select_bitmap_size_table

*Function*

```rust
fn select_bitmap_size_table(glyph_id: crate::GlyphId, pixels_per_em: u16, s: crate::parser::Stream) -> Option<BitmapSizeTable>
```



## ttf_parser::tables::cblc::select_index_subtable

*Function*

```rust
fn select_index_subtable(data: &[u8], size_table: BitmapSizeTable, glyph_id: crate::GlyphId) -> Option<IndexSubtableInfo>
```



