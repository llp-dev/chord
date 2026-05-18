**ttf_parser > tables > name**

# Module: tables::name

## Contents

**Modules**

- [`name_id`](#name_id) - A list of [name ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids)'s.

**Structs**

- [`Name`](#name) - A [Name Record](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records).
- [`NameRecord`](#namerecord)
- [`Names`](#names) - A list of face names.
- [`NamesIter`](#namesiter) - An iterator over face names.
- [`Table`](#table) - A [Naming Table](

**Enums**

- [`PlatformId`](#platformid) - A [platform ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-ids).

**Functions**

- [`is_unicode_encoding`](#is_unicode_encoding)

---

## ttf_parser::tables::name::Name

*Struct*

A [Name Record](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records).

**Generic Parameters:**
- 'a

**Fields:**
- `platform_id: PlatformId` - A platform ID.
- `encoding_id: u16` - A platform-specific encoding ID.
- `language_id: u16` - A language ID.
- `name_id: u16` - A [Name ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids).
- `name: &'a [u8]` - A raw name data.

**Methods:**

- `fn is_unicode(self: &Self) -> bool` - Checks that the current Name data has a Unicode encoding.
- `fn language(self: &Self) -> Language` - Returns a Name language.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Name<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::name::NameRecord

*Struct*

**Fields:**
- `platform_id: PlatformId`
- `encoding_id: u16`
- `language_id: u16`
- `name_id: u16`
- `length: u16`
- `offset: crate::parser::Offset16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NameRecord`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::name::Names

*Struct*

A list of face names.

**Generic Parameters:**
- 'a

**Fields:**
- `records: crate::parser::LazyArray16<'a, NameRecord>`
- `storage: &'a [u8]`

**Methods:**

- `fn get(self: &Self, index: u16) -> Option<Name<'a>>` - Returns a name at index.
- `fn len(self: &Self) -> u16` - Returns a number of name records.
- `fn is_empty(self: &Self) -> bool` - Checks if there are any name records.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Clone**
  - `fn clone(self: &Self) -> Names<'a>`
- **Default**
  - `fn default() -> Names<'a>`



## ttf_parser::tables::name::NamesIter

*Struct*

An iterator over face names.

**Generic Parameters:**
- 'a

**Fields:**
- `names: Names<'a>`
- `index: u16`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NamesIter<'a>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn count(self: Self) -> usize`



## ttf_parser::tables::name::PlatformId

*Enum*

A [platform ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-ids).

**Variants:**
- `Unicode`
- `Macintosh`
- `Iso`
- `Windows`
- `Custom`

**Traits:** Eq, Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **PartialEq**
  - `fn eq(self: &Self, other: &PlatformId) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PlatformId`



## ttf_parser::tables::name::Table

*Struct*

A [Naming Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/name).

**Generic Parameters:**
- 'a

**Fields:**
- `names: Names<'a>` - A list of names.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Default**
  - `fn default() -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::name::is_unicode_encoding

*Function*

```rust
fn is_unicode_encoding(platform_id: PlatformId, encoding_id: u16) -> bool
```



## Module: name_id

A list of [name ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids)'s.



