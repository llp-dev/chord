**ttf_parser > tables > cff**

# Module: tables::cff

## Contents

**Modules**

- [`argstack`](#argstack)
- [`cff1`](#cff1) - A [Compact Font Format Table](
- [`charset`](#charset)
- [`charstring`](#charstring)
- [`dict`](#dict)
- [`encoding`](#encoding)
- [`index`](#index)

**Structs**

- [`Builder`](#builder)
- [`StringId`](#stringid) - A type-safe wrapper for string ID.

**Enums**

- [`CFFError`](#cfferror) - A list of errors that can occur during a CFF glyph outlining.

**Functions**

- [`calc_subroutine_bias`](#calc_subroutine_bias)
- [`conv_subroutine_index`](#conv_subroutine_index)
- [`conv_subroutine_index_impl`](#conv_subroutine_index_impl)
- [`f32_abs`](#f32_abs)

**Traits**

- [`IsEven`](#iseven)

---

## ttf_parser::tables::cff::Builder

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `builder: &'a  mut dyn OutlineBuilder`
- `bbox: crate::RectF`

**Methods:**

- `fn move_to(self: & mut Self, x: f32, y: f32)`
- `fn line_to(self: & mut Self, x: f32, y: f32)`
- `fn curve_to(self: & mut Self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32)`
- `fn close(self: & mut Self)`



## ttf_parser::tables::cff::CFFError

*Enum*

A list of errors that can occur during a CFF glyph outlining.

**Variants:**
- `NoGlyph`
- `ReadOutOfBounds`
- `ZeroBBox`
- `InvalidOperator`
- `UnsupportedOperator`
- `MissingEndChar`
- `DataAfterEndChar`
- `NestingLimitReached`
- `ArgumentsStackLimitReached`
- `InvalidArgumentsStackLength`
- `BboxOverflow`
- `MissingMoveTo`
- `InvalidSubroutineIndex`
- `NoLocalSubroutines`
- `InvalidSeacCode`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CFFError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CFFError`



## ttf_parser::tables::cff::IsEven

*Trait*

**Methods:**

- `is_even`
- `is_odd`



## ttf_parser::tables::cff::StringId

*Struct*

A type-safe wrapper for string ID.

**Tuple Struct**: `(u16)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **PartialEq**
  - `fn eq(self: &Self, other: &StringId) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &StringId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StringId`



## Module: argstack



## ttf_parser::tables::cff::calc_subroutine_bias

*Function*

```rust
fn calc_subroutine_bias(len: u32) -> u16
```



## Module: cff1

A [Compact Font Format Table](
https://docs.microsoft.com/en-us/typography/opentype/spec/cff) implementation.



## Module: charset



## Module: charstring



## ttf_parser::tables::cff::conv_subroutine_index

*Function*

```rust
fn conv_subroutine_index(index: f32, bias: u16) -> Result<u32, CFFError>
```



## ttf_parser::tables::cff::conv_subroutine_index_impl

*Function*

```rust
fn conv_subroutine_index_impl(index: f32, bias: u16) -> Option<u32>
```



## Module: dict



## Module: encoding



## ttf_parser::tables::cff::f32_abs

*Function*

```rust
fn f32_abs(n: f32) -> f32
```



## Module: index



