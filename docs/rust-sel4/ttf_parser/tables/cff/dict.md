**ttf_parser > tables > cff > dict**

# Module: tables::cff::dict

## Contents

**Structs**

- [`DictionaryParser`](#dictionaryparser)
- [`Operator`](#operator)

**Functions**

- [`is_dict_one_byte_op`](#is_dict_one_byte_op)
- [`parse_float`](#parse_float)
- [`parse_float_nibble`](#parse_float_nibble)
- [`parse_number`](#parse_number)
- [`skip_number`](#skip_number)

**Constants**

- [`END_OF_FLOAT_FLAG`](#end_of_float_flag)
- [`FLOAT_STACK_LEN`](#float_stack_len)
- [`TWO_BYTE_OPERATOR_MARK`](#two_byte_operator_mark)

---

## ttf_parser::tables::cff::dict::DictionaryParser

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a [u8]`
- `offset: usize`
- `operands_offset: usize`
- `operands: &'a  mut [f64]`
- `operands_len: u16`

**Methods:**

- `fn new(data: &'a [u8], operands_buffer: &'a  mut [f64]) -> Self`
- `fn parse_next(self: & mut Self) -> Option<Operator>`
- `fn parse_operands(self: & mut Self) -> Option<()>` - Parses operands of the current operator.
- `fn operands(self: &Self) -> &[f64]`
- `fn parse_number(self: & mut Self) -> Option<f64>`
- `fn parse_offset(self: & mut Self) -> Option<usize>`
- `fn parse_range(self: & mut Self) -> Option<Range<usize>>`



## ttf_parser::tables::cff::dict::END_OF_FLOAT_FLAG

*Constant*: `u8`



## ttf_parser::tables::cff::dict::FLOAT_STACK_LEN

*Constant*: `usize`



## ttf_parser::tables::cff::dict::Operator

*Struct*

**Tuple Struct**: `(u16)`

**Methods:**

- `fn get(self: Self) -> u16`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Operator`



## ttf_parser::tables::cff::dict::TWO_BYTE_OPERATOR_MARK

*Constant*: `u8`



## ttf_parser::tables::cff::dict::is_dict_one_byte_op

*Function*

```rust
fn is_dict_one_byte_op(b: u8) -> bool
```



## ttf_parser::tables::cff::dict::parse_float

*Function*

```rust
fn parse_float(s: & mut parser::Stream) -> Option<f64>
```



## ttf_parser::tables::cff::dict::parse_float_nibble

*Function*

```rust
fn parse_float_nibble(nibble: u8, idx: usize, data: & mut [u8]) -> Option<usize>
```



## ttf_parser::tables::cff::dict::parse_number

*Function*

```rust
fn parse_number(b0: u8, s: & mut parser::Stream) -> Option<f64>
```



## ttf_parser::tables::cff::dict::skip_number

*Function*

```rust
fn skip_number(b0: u8, s: & mut parser::Stream) -> Option<()>
```



