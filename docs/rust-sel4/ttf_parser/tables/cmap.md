**ttf_parser > tables > cmap**

# Module: tables::cmap

## Contents

**Modules**

- [`format0`](#format0)
- [`format10`](#format10)
- [`format12`](#format12)
- [`format13`](#format13)
- [`format14`](#format14)
- [`format2`](#format2)
- [`format4`](#format4)
- [`format6`](#format6)

**Structs**

- [`EncodingRecord`](#encodingrecord)
- [`Subtable`](#subtable) - A character encoding subtable.
- [`Subtables`](#subtables) - A list of subtables.
- [`SubtablesIter`](#subtablesiter) - An iterator over [`Subtables`].
- [`Table`](#table) - A [Character to Glyph Index Mapping Table](

**Enums**

- [`Format`](#format) - A character encoding subtable variant.

---

## ttf_parser::tables::cmap::EncodingRecord

*Struct*

**Fields:**
- `platform_id: crate::name::PlatformId`
- `encoding_id: u16`
- `offset: crate::parser::Offset32`

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> EncodingRecord`



## ttf_parser::tables::cmap::Format

*Enum*

A character encoding subtable variant.

**Generic Parameters:**
- 'a

**Variants:**
- `ByteEncodingTable(Subtable0<'a>)`
- `HighByteMappingThroughTable(Subtable2<'a>)`
- `SegmentMappingToDeltaValues(Subtable4<'a>)`
- `TrimmedTableMapping(Subtable6<'a>)`
- `MixedCoverage`
- `TrimmedArray(Subtable10<'a>)`
- `SegmentedCoverage(Subtable12<'a>)`
- `ManyToOneRangeMappings(Subtable13<'a>)`
- `UnicodeVariationSequences(Subtable14<'a>)`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Format<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cmap::Subtable

*Struct*

A character encoding subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `platform_id: crate::name::PlatformId` - Subtable platform.
- `encoding_id: u16` - Subtable encoding.
- `format: Format<'a>` - A subtable format.

**Methods:**

- `fn is_unicode(self: &Self) -> bool` - Checks that the current encoding is Unicode compatible.
- `fn glyph_index(self: &Self, code_point: u32) -> Option<GlyphId>` - Maps a character to a glyph ID.
- `fn glyph_variation_index(self: &Self, code_point: u32, variation: u32) -> Option<GlyphVariationResult>` - Resolves a variation of a glyph ID from two code points.
- `fn codepoints<F>(self: &Self, f: F)` - Calls `f` for all codepoints contained in this subtable.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cmap::Subtables

*Struct*

A list of subtables.

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `records: crate::parser::LazyArray16<'a, EncodingRecord>`

**Methods:**

- `fn get(self: &Self, index: u16) -> Option<Subtable<'a>>` - Returns a subtable at an index.
- `fn len(self: &Self) -> u16` - Returns the number of subtables.
- `fn is_empty(self: &Self) -> bool` - Checks if there are any subtables.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Clone**
  - `fn clone(self: &Self) -> Subtables<'a>`
- **Default**
  - `fn default() -> Subtables<'a>`



## ttf_parser::tables::cmap::SubtablesIter

*Struct*

An iterator over [`Subtables`].

**Generic Parameters:**
- 'a

**Fields:**
- `subtables: Subtables<'a>`
- `index: u16`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## ttf_parser::tables::cmap::Table

*Struct*

A [Character to Glyph Index Mapping Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cmap).

**Generic Parameters:**
- 'a

**Fields:**
- `subtables: Subtables<'a>` - A list of subtables.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: format0



## Module: format10



## Module: format12



## Module: format13



## Module: format14



## Module: format2



## Module: format4



## Module: format6



