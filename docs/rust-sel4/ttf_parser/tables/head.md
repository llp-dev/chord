**ttf_parser > tables > head**

# Module: tables::head

## Contents

**Structs**

- [`Table`](#table) - A [Font Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/head).

**Enums**

- [`IndexToLocationFormat`](#indextolocationformat) - An index format used by the [Index to Location Table](

---

## ttf_parser::tables::head::IndexToLocationFormat

*Enum*

An index format used by the [Index to Location Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/loca).

**Variants:**
- `Short`
- `Long`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IndexToLocationFormat) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> IndexToLocationFormat`



## ttf_parser::tables::head::Table

*Struct*

A [Font Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/head).

**Fields:**
- `units_per_em: u16` - Units per EM.
- `global_bbox: crate::Rect` - A bounding box that large enough to enclose any glyph from the face.
- `index_to_location_format: IndexToLocationFormat` - An index format used by the [Index to Location Table](

**Methods:**

- `fn parse(data: &[u8]) -> Option<Self>` - Parses a table from raw data.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



