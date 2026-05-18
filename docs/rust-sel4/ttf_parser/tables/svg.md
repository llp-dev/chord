**ttf_parser > tables > svg**

# Module: tables::svg

## Contents

**Structs**

- [`SvgDocument`](#svgdocument) - An [SVG documents](
- [`SvgDocumentRecord`](#svgdocumentrecord)
- [`SvgDocumentsList`](#svgdocumentslist) - A list of [SVG documents](
- [`SvgDocumentsListIter`](#svgdocumentslistiter) - An iterator over [`SvgDocumentsList`] values.
- [`Table`](#table) - An [SVG Table](https://docs.microsoft.com/en-us/typography/opentype/spec/svg).

---

## ttf_parser::tables::svg::SvgDocument

*Struct*

An [SVG documents](
https://docs.microsoft.com/en-us/typography/opentype/spec/svg#svg-document-list).

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]` - The SVG document data.
- `start_glyph_id: crate::GlyphId` - The first glyph ID for the range covered by this record.
- `end_glyph_id: crate::GlyphId` - The last glyph ID, *inclusive*, for the range covered by this record.

**Methods:**

- `fn glyphs_range(self: &Self) -> core::ops::RangeInclusive<GlyphId>` - Returns the glyphs range.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SvgDocument<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::svg::SvgDocumentRecord

*Struct*

**Fields:**
- `start_glyph_id: crate::GlyphId`
- `end_glyph_id: crate::GlyphId`
- `svg_doc_offset: Option<crate::parser::Offset32>`
- `svg_doc_length: u32`

**Methods:**

- `fn glyphs_range(self: &Self) -> core::ops::RangeInclusive<GlyphId>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SvgDocumentRecord`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::svg::SvgDocumentsList

*Struct*

A list of [SVG documents](
https://docs.microsoft.com/en-us/typography/opentype/spec/svg#svg-document-list).

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `records: crate::parser::LazyArray16<'a, SvgDocumentRecord>`

**Methods:**

- `fn get(self: &Self, index: u16) -> Option<SvgDocument<'a>>` - Returns SVG document data at index.
- `fn find(self: &Self, glyph_id: GlyphId) -> Option<SvgDocument<'a>>` - Returns a SVG document data by glyph ID.
- `fn len(self: &Self) -> u16` - Returns the number of SVG documents in the list.
- `fn is_empty(self: &Self) -> bool` - Checks if the list is empty.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SvgDocumentsList<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



## ttf_parser::tables::svg::SvgDocumentsListIter

*Struct*

An iterator over [`SvgDocumentsList`] values.

**Generic Parameters:**
- 'a

**Fields:**
- `list: SvgDocumentsList<'a>`
- `index: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SvgDocumentsListIter<'a>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn count(self: Self) -> usize`



## ttf_parser::tables::svg::Table

*Struct*

An [SVG Table](https://docs.microsoft.com/en-us/typography/opentype/spec/svg).

**Generic Parameters:**
- 'a

**Fields:**
- `documents: SvgDocumentsList<'a>` - A list of SVG documents.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



