**ttf_parser > tables > cff > encoding**

# Module: tables::cff::encoding

## Contents

**Structs**

- [`Encoding`](#encoding)
- [`Format1Range`](#format1range)
- [`Supplement`](#supplement)

**Enums**

- [`EncodingKind`](#encodingkind)

**Functions**

- [`parse_encoding`](#parse_encoding)

**Constants**

- [`STANDARD_ENCODING`](#standard_encoding) - The Standard Encoding as defined in the Adobe Technical Note #5176 Appendix B.

---

## ttf_parser::tables::cff::encoding::Encoding

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `kind: EncodingKind<'a>`
- `supplemental: crate::parser::LazyArray16<'a, Supplement>`

**Methods:**

- `fn new_standard() -> Self`
- `fn new_expert() -> Self`
- `fn code_to_gid(self: &Self, charset: &Charset, code: u8) -> Option<GlyphId>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Encoding<'a>`
- **Default**
  - `fn default() -> Encoding<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::encoding::EncodingKind

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Standard`
- `Expert`
- `Format0(crate::parser::LazyArray16<'a, u8>)`
- `Format1(crate::parser::LazyArray16<'a, Format1Range>)`

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> EncodingKind<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::encoding::Format1Range

*Struct*

**Fields:**
- `first: u8`
- `left: u8`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> Format1Range`



## ttf_parser::tables::cff::encoding::STANDARD_ENCODING

*Constant*: `[u8; 256]`

The Standard Encoding as defined in the Adobe Technical Note #5176 Appendix B.



## ttf_parser::tables::cff::encoding::Supplement

*Struct*

**Fields:**
- `code: u8`
- `name: super::StringId`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Supplement`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::cff::encoding::parse_encoding

*Function*

```rust
fn parse_encoding<'a>(s: & mut crate::parser::Stream<'a>) -> Option<Encoding<'a>>
```



