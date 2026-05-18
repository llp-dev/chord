**ring > io > positive**

# Module: io::positive

## Contents

**Structs**

- [`Positive`](#positive) - A serialized positive integer.

---

## ring::io::positive::Positive

*Struct*

A serialized positive integer.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn big_endian_without_leading_zero(self: &Self) -> &'a [u8]` - Returns the value, ordered from significant byte to least significant
- `fn first_byte(self: &Self) -> u8` - Returns the first byte.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Positive<'a>`



