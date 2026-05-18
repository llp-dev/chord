**ttf_parser > tables > cpal**

# Module: tables::cpal

## Contents

**Structs**

- [`BgraColor`](#bgracolor)
- [`Table`](#table) - A [Color Palette Table](

---

## ttf_parser::tables::cpal::BgraColor

*Struct*

**Fields:**
- `blue: u8`
- `green: u8`
- `red: u8`
- `alpha: u8`

**Methods:**

- `fn to_rgba(self: Self) -> RgbaColor`

**Traits:** Eq, Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **PartialEq**
  - `fn eq(self: &Self, other: &BgraColor) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BgraColor`



## ttf_parser::tables::cpal::Table

*Struct*

A [Color Palette Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cpal).

**Generic Parameters:**
- 'a

**Fields:**
- `color_indices: crate::parser::LazyArray16<'a, u16>`
- `colors: crate::parser::LazyArray16<'a, BgraColor>`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn palettes(self: &Self) -> NonZeroU16` - Returns the number of palettes.
- `fn get(self: &Self, palette_index: u16, palette_entry: u16) -> Option<RgbaColor>` - Returns the color at the given index into the given palette.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



