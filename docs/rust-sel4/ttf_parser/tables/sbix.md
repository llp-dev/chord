**ttf_parser > tables > sbix**

# Module: tables::sbix

## Contents

**Structs**

- [`Strike`](#strike) - A strike of glyphs.
- [`Strikes`](#strikes) - A list of [`Strike`]s.
- [`StrikesIter`](#strikesiter) - An iterator over [`Strikes`].
- [`Table`](#table) - A [Standard Bitmap Graphics Table](

**Functions**

- [`png_size`](#png_size)

---

## ttf_parser::tables::sbix::Strike

*Struct*

A strike of glyphs.

**Generic Parameters:**
- 'a

**Fields:**
- `pixels_per_em: u16` - The pixels per EM size for which this strike was designed.
- `ppi: u16` - The device pixel density (in PPI) for which this strike was designed.
- `offsets: crate::parser::LazyArray16<'a, crate::parser::Offset32>`
- `data: &'a [u8]` - Data from the beginning of the `Strikes` table.

**Methods:**

- `fn parse(number_of_glyphs: u16, data: &'a [u8]) -> Option<Self>`
- `fn get(self: &Self, glyph_id: GlyphId) -> Option<RasterGlyphImage<'a>>` - Returns a glyph data.
- `fn get_inner(self: &Self, glyph_id: GlyphId, depth: u8) -> Option<RasterGlyphImage<'a>>`
- `fn len(self: &Self) -> u16` - Returns the number of glyphs in this strike.
- `fn is_empty(self: &Self) -> bool` - Checks if there are any glyphs.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Strike<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::sbix::Strikes

*Struct*

A list of [`Strike`]s.

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]` - `sbix` table data.
- `offsets: crate::parser::LazyArray32<'a, crate::parser::Offset32>`
- `number_of_glyphs: u16`

**Methods:**

- `fn get(self: &Self, index: u32) -> Option<Strike<'a>>` - Returns a strike at the index.
- `fn len(self: &Self) -> u32` - Returns the number of strikes.
- `fn is_empty(self: &Self) -> bool` - Checks if there are any strikes.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Strikes<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



## ttf_parser::tables::sbix::StrikesIter

*Struct*

An iterator over [`Strikes`].

**Generic Parameters:**
- 'a

**Fields:**
- `strikes: Strikes<'a>`
- `index: u32`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## ttf_parser::tables::sbix::Table

*Struct*

A [Standard Bitmap Graphics Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/sbix).

**Generic Parameters:**
- 'a

**Fields:**
- `strikes: Strikes<'a>` - A list of [`Strike`]s.

**Methods:**

- `fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn best_strike(self: &Self, pixels_per_em: u16) -> Option<Strike<'a>>` - Selects the best matching [`Strike`] based on `pixels_per_em`.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::sbix::png_size

*Function*

```rust
fn png_size(data: &[u8]) -> Option<(u16, u16)>
```



