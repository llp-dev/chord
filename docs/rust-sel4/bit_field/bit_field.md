**bit_field**

# Module: bit_field

## Contents

**Traits**

- [`BitArray`](#bitarray)
- [`BitField`](#bitfield) - A generic trait which provides methods for extracting and setting specific bits or ranges of

---

## bit_field::BitArray

*Trait*

**Methods:**

- `bit_length`: Returns the length, eg number of bits, in this bit array.
- `get_bit`: Obtains the bit at the index `bit`; note that index 0 is the least significant bit, while
- `get_bits`: Obtains the range of bits specified by `range`; note that index 0 is the least significant
- `set_bit`: Sets the bit at the index `bit` to the value `value` (where true means a value of '1' and
- `set_bits`: Sets the range of bits defined by the range `range` to the lower bits of `value`; to be



## bit_field::BitField

*Trait*

A generic trait which provides methods for extracting and setting specific bits or ranges of
bits.

**Methods:**

- `BIT_LENGTH`: The number of bits in this bit field.
- `get_bit`: Obtains the bit at the index `bit`; note that index 0 is the least significant bit, while
- `get_bits`: Obtains the range of bits specified by `range`; note that index 0 is the least significant
- `set_bit`: Sets the bit at the index `bit` to the value `value` (where true means a value of '1' and
- `set_bits`: Sets the range of bits defined by the range `range` to the lower bits of `value`; to be



