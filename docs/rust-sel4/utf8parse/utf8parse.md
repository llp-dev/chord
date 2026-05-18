**utf8parse**

# Module: utf8parse

## Contents

**Modules**

- [`types`](#types) - Types supporting the UTF-8 parser

**Structs**

- [`Parser`](#parser) - A parser for Utf8 Characters

**Traits**

- [`Receiver`](#receiver) - Handles codepoint and invalid sequence events from the parser.

**Constants**

- [`CONTINUATION_MASK`](#continuation_mask) - Continuation bytes are masked with this value.

---

## utf8parse::CONTINUATION_MASK

*Constant*: `u8`

Continuation bytes are masked with this value.



## utf8parse::Parser

*Struct*

A parser for Utf8 Characters

Repeatedly call `advance` with bytes to emit Utf8 characters

**Fields:**
- `point: u32`
- `state: types::State`

**Methods:**

- `fn new() -> Parser` - Create a new Parser
- `fn advance<R>(self: & mut Self, receiver: & mut R, byte: u8)` - Advance the parser
- `fn perform_action<R>(self: & mut Self, receiver: & mut R, byte: u8, action: Action)`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Parser) -> bool`
- **Default**
  - `fn default() -> Parser`
- **Clone**
  - `fn clone(self: &Self) -> Parser`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## utf8parse::Receiver

*Trait*

Handles codepoint and invalid sequence events from the parser.

**Methods:**

- `codepoint`: Called whenever a codepoint is parsed successfully
- `invalid_sequence`: Called when an invalid_sequence is detected



## Module: types

Types supporting the UTF-8 parser



