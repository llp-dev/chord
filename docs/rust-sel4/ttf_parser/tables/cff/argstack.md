**ttf_parser > tables > cff > argstack**

# Module: tables::cff::argstack

## Contents

**Structs**

- [`ArgumentsStack`](#argumentsstack)

---

## ttf_parser::tables::cff::argstack::ArgumentsStack

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `data: &'a  mut [f32]`
- `len: usize`
- `max_len: usize`

**Methods:**

- `fn len(self: &Self) -> usize`
- `fn is_empty(self: &Self) -> bool`
- `fn push(self: & mut Self, n: f32) -> Result<(), CFFError>`
- `fn at(self: &Self, index: usize) -> f32`
- `fn pop(self: & mut Self) -> f32`
- `fn reverse(self: & mut Self)`
- `fn clear(self: & mut Self)`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



