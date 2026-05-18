**ttf_parser > tables > cff > index**

# Module: tables::cff::index

## Contents

**Structs**

- [`Index`](#index)
- [`IndexIter`](#indexiter)
- [`VarOffsets`](#varoffsets)

**Enums**

- [`OffsetSize`](#offsetsize)

**Functions**

- [`parse_index`](#parse_index)
- [`parse_index_impl`](#parse_index_impl)
- [`skip_index`](#skip_index)
- [`skip_index_impl`](#skip_index_impl)

**Traits**

- [`IndexSize`](#indexsize)

---

## ttf_parser::tables::cff::index::Index

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `offsets: VarOffsets<'a>`

**Methods:**

- `fn len(self: &Self) -> u32`
- `fn get(self: &Self, index: u32) -> Option<&'a [u8]>`

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Clone**
  - `fn clone(self: &Self) -> Index<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::index::IndexIter

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `data: Index<'a>`
- `offset_index: u32`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## ttf_parser::tables::cff::index::IndexSize

*Trait*

**Methods:**

- `to_u32`



## ttf_parser::tables::cff::index::OffsetSize

*Enum*

**Variants:**
- `Size1`
- `Size2`
- `Size3`
- `Size4`

**Methods:**

- `fn to_u32(self: Self) -> u32`
- `fn to_usize(self: Self) -> usize`

**Traits:** Eq, Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **PartialEq**
  - `fn eq(self: &Self, other: &OffsetSize) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OffsetSize`



## ttf_parser::tables::cff::index::VarOffsets

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `offset_size: OffsetSize`

**Methods:**

- `fn get(self: &Self, index: u32) -> Option<u32>`
- `fn last(self: &Self) -> Option<u32>`
- `fn len(self: &Self) -> u32`
- `fn is_empty(self: &Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VarOffsets<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::cff::index::parse_index

*Function*

```rust
fn parse_index<'a, T>(s: & mut crate::parser::Stream<'a>) -> Option<Index<'a>>
```



## ttf_parser::tables::cff::index::parse_index_impl

*Function*

```rust
fn parse_index_impl<'a>(count: u32, s: & mut crate::parser::Stream<'a>) -> Option<Index<'a>>
```



## ttf_parser::tables::cff::index::skip_index

*Function*

```rust
fn skip_index<T>(s: & mut crate::parser::Stream) -> Option<()>
```



## ttf_parser::tables::cff::index::skip_index_impl

*Function*

```rust
fn skip_index_impl(count: u32, s: & mut crate::parser::Stream) -> Option<()>
```



