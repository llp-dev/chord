**ttf_parser > tables > cff > charset**

# Module: tables::cff::charset

## Contents

**Structs**

- [`Format1Range`](#format1range)
- [`Format2Range`](#format2range)

**Enums**

- [`Charset`](#charset)

**Functions**

- [`parse_charset`](#parse_charset)

---

## ttf_parser::tables::cff::charset::Charset

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `ISOAdobe`
- `Expert`
- `ExpertSubset`
- `Format0(crate::parser::LazyArray16<'a, super::StringId>)`
- `Format1(crate::parser::LazyArray16<'a, Format1Range>)`
- `Format2(crate::parser::LazyArray16<'a, Format2Range>)`

**Methods:**

- `fn sid_to_gid(self: &Self, sid: StringId) -> Option<GlyphId>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Charset<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::charset::Format1Range

*Struct*

**Fields:**
- `first: super::StringId`
- `left: u8`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Format1Range`



## ttf_parser::tables::cff::charset::Format2Range

*Struct*

**Fields:**
- `first: super::StringId`
- `left: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Format2Range`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::charset::parse_charset

*Function*

```rust
fn parse_charset<'a>(number_of_glyphs: core::num::NonZeroU16, s: & mut crate::parser::Stream<'a>) -> Option<Charset<'a>>
```



