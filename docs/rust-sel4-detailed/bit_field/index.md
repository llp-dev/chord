# Crate `bit_field`

Provides the abstraction of a bit field, which allows for bit-level update and retrieval
operations.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BitField`](#bitfield) | trait | A generic trait which provides methods for extracting and setting specific bits or ranges of bits. |
| [`BitArray`](#bitarray) | trait |  |
| [`to_regular_range`](#to-regular-range) | fn |  |
| [`bitfield_numeric_impl!`](#bitfield-numeric-impl) | macro | An internal macro used for implementing BitField on the standard integral types. |

## Traits

### `BitField`

```rust
trait BitField { ... }
```

A generic trait which provides methods for extracting and setting specific bits or ranges of
bits.

#### Associated Constants

- `const BIT_LENGTH: usize`

#### Required Methods

- `fn get_bit(&self, bit: usize) -> bool`

  Obtains the bit at the index `bit`; note that index 0 is the least significant bit, while

- `fn get_bits<T: RangeBounds<usize>>(&self, range: T) -> Self`

  Obtains the range of bits specified by `range`; note that index 0 is the least significant

- `fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self`

  Sets the bit at the index `bit` to the value `value` (where true means a value of '1' and

- `fn set_bits<T: RangeBounds<usize>>(&mut self, range: T, value: Self) -> &mut Self`

  Sets the range of bits defined by the range `range` to the lower bits of `value`; to be

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `BitArray<T: BitField>`

```rust
trait BitArray<T: BitField> { ... }
```

#### Required Methods

- `fn bit_length(&self) -> usize`

  Returns the length, eg number of bits, in this bit array.

- `fn get_bit(&self, bit: usize) -> bool`

  Obtains the bit at the index `bit`; note that index 0 is the least significant bit, while

- `fn get_bits<U: RangeBounds<usize>>(&self, range: U) -> T`

  Obtains the range of bits specified by `range`; note that index 0 is the least significant

- `fn set_bit(&mut self, bit: usize, value: bool)`

  Sets the bit at the index `bit` to the value `value` (where true means a value of '1' and

- `fn set_bits<U: RangeBounds<usize>>(&mut self, range: U, value: T)`

  Sets the range of bits defined by the range `range` to the lower bits of `value`; to be

#### Implementors

- `[T]`

## Functions

### `to_regular_range`

```rust
fn to_regular_range<T: RangeBounds<usize>>(generic_rage: &T, bit_length: usize) -> core::ops::Range<usize>
```

## Macros

### `bitfield_numeric_impl!`

An internal macro used for implementing BitField on the standard integral types.

