**ttf_parser > tables > kern**

# Module: tables::kern

## Contents

**Structs**

- [`AATCoverage`](#aatcoverage)
- [`KerningPair`](#kerningpair) - A kerning pair.
- [`OTCoverage`](#otcoverage)
- [`Subtable`](#subtable) - A kerning subtable.
- [`Subtable0`](#subtable0) - A format 0 subtable.
- [`Subtable2`](#subtable2) - A format 2 subtable.
- [`Subtable3`](#subtable3) - A format 3 subtable.
- [`Subtables`](#subtables) - A list of subtables.
- [`SubtablesIter`](#subtablesiter) - An iterator over kerning subtables.
- [`Table`](#table) - A [Kerning Table](https://docs.microsoft.com/en-us/typography/opentype/spec/kern).

**Enums**

- [`Format`](#format) - A kerning subtable format.

**Functions**

- [`get_format2_class`](#get_format2_class)

---

## ttf_parser::tables::kern::AATCoverage

*Struct*

**Tuple Struct**: `(u8)`

**Methods:**

- `fn is_horizontal(self: Self) -> bool`
- `fn has_cross_stream(self: Self) -> bool`
- `fn is_variable(self: Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AATCoverage`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::kern::Format

*Enum*

A kerning subtable format.

**Generic Parameters:**
- 'a

**Variants:**
- `Format0(Subtable0<'a>)`
- `Format1`
- `Format2(Subtable2<'a>)`
- `Format3(Subtable3<'a>)`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Format<'a>`



## ttf_parser::tables::kern::KerningPair

*Struct*

A kerning pair.

**Fields:**
- `pair: u32` - Glyphs pair.
- `value: i16` - Kerning value.

**Methods:**

- `fn left(self: &Self) -> GlyphId` - Returns left glyph ID.
- `fn right(self: &Self) -> GlyphId` - Returns right glyph ID.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> KerningPair`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::kern::OTCoverage

*Struct*

**Tuple Struct**: `(u8)`

**Methods:**

- `fn is_horizontal(self: Self) -> bool`
- `fn has_cross_stream(self: Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> OTCoverage`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::kern::Subtable

*Struct*

A kerning subtable.

**Generic Parameters:**
- 'a

**Fields:**
- `horizontal: bool` - Indicates that subtable is for horizontal text.
- `variable: bool` - Indicates that subtable is variable.
- `has_cross_stream: bool` - Indicates that subtable has a cross-stream values.
- `has_state_machine: bool` - Indicates that subtable uses a state machine.
- `format: Format<'a>` - Subtable format.

**Methods:**

- `fn glyphs_kerning(self: &Self, left: GlyphId, right: GlyphId) -> Option<i16>` - Returns kerning for a pair of glyphs.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::kern::Subtable0

*Struct*

A format 0 subtable.

Ordered List of Kerning Pairs.

**Generic Parameters:**
- 'a

**Fields:**
- `pairs: crate::parser::LazyArray16<'a, KerningPair>` - A list of kerning pairs.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyphs_kerning(self: &Self, left: GlyphId, right: GlyphId) -> Option<i16>` - Returns kerning for a pair of glyphs.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable0<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::kern::Subtable2

*Struct*

A format 2 subtable.

Simple n x m Array of Kerning Values.

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `header_len: u8`

**Methods:**

- `fn parse(header_len: u8, data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyphs_kerning(self: &Self, left: GlyphId, right: GlyphId) -> Option<i16>` - Returns kerning for a pair of glyphs.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable2<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::kern::Subtable3

*Struct*

A format 3 subtable.

Simple n x m Array of Kerning Indices.

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a subtable from raw data.
- `fn glyphs_kerning(self: &Self, left: GlyphId, right: GlyphId) -> Option<i16>` - Returns kerning for a pair of glyphs.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtable3<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::kern::Subtables

*Struct*

A list of subtables.

The internal data layout is not designed for random access,
therefore we're not providing the `get()` method and only an iterator.

**Generic Parameters:**
- 'a

**Fields:**
- `is_aat: bool` - Indicates an Apple Advanced Typography format.
- `count: u32` - The total number of tables.
- `data: &'a [u8]` - Actual data. Starts right after the `kern` header.

**Methods:**

- `fn len(self: &Self) -> u32` - Returns the number of subtables.
- `fn is_empty(self: &Self) -> bool` - Checks if there are any subtables.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Subtables<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



## ttf_parser::tables::kern::SubtablesIter

*Struct*

An iterator over kerning subtables.

**Generic Parameters:**
- 'a

**Fields:**
- `is_aat: bool` - Indicates an Apple Advanced Typography format.
- `table_index: u32` - The current table index,
- `number_of_tables: u32` - The total number of tables.
- `stream: crate::parser::Stream<'a>` - Actual data. Starts right after `kern` header.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SubtablesIter<'a>`
- **Default**
  - `fn default() -> SubtablesIter<'a>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## ttf_parser::tables::kern::Table

*Struct*

A [Kerning Table](https://docs.microsoft.com/en-us/typography/opentype/spec/kern).

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



## ttf_parser::tables::kern::get_format2_class

*Function*

```rust
fn get_format2_class(glyph_id: u16, offset: usize, data: &[u8]) -> Option<u16>
```



