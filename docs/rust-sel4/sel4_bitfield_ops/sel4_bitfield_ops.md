**sel4_bitfield_ops**

# Module: sel4_bitfield_ops

## Contents

**Structs**

- [`Bitfield`](#bitfield)

**Functions**

- [`get`](#get)
- [`get_bit`](#get_bit)
- [`get_bits`](#get_bits)
- [`set`](#set)
- [`set_bits`](#set_bits)
- [`set_bits_from_slice`](#set_bits_from_slice)

**Traits**

- [`PrimInt`](#primint)
- [`UnsignedPrimInt`](#unsignedprimint)

---

## sel4_bitfield_ops::Bitfield

*Struct*

**Generic Parameters:**
- T
- U

**Methods:**

- `fn zeroed() -> Self`
- `fn bits(self: &Self) -> &[U]`
- `fn get_bits<V>(self: &Self, range: Range<usize>) -> V`
- `fn get_bits_into_slice<V>(self: &Self, range: Range<usize>, dst: & mut [V], dst_start: usize)`
- `fn get<V>(self: &Self, start_bit: usize) -> V`
- `fn bits_mut(self: & mut Self) -> & mut [U]`
- `fn set_bits<V>(self: & mut Self, range: Range<usize>, src: V)`
- `fn set_bits_from_slice<V>(self: & mut Self, range: Range<usize>, src: &[V], src_start: usize)`
- `fn set<V>(self: & mut Self, start_bit: usize, src: V)`
- `fn new(inner: T) -> Self`
- `fn into_inner(self: Self) -> T`
- `fn inner(self: &Self) -> &T`
- `fn inner_mut(self: & mut Self) -> & mut T`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Bitfield<T, U>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Bitfield<T, U>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Bitfield<T, U>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Bitfield<T, U>) -> $crate::cmp::Ordering`



## sel4_bitfield_ops::PrimInt

*Trait*

**Methods:**

- `Unsigned`
- `cast_from_unsigned`
- `cast_to_unsigned`



## sel4_bitfield_ops::UnsignedPrimInt

*Trait*

**Methods:**

- `NUM_BITS`
- `zero`
- `one`



## sel4_bitfield_ops::get

*Function*

```rust
fn get<T, U>(src: &[T], src_start_bit: usize) -> U
```



## sel4_bitfield_ops::get_bit

*Function*

```rust
fn get_bit<T>(src: &[T], i: usize) -> bool
```



## sel4_bitfield_ops::get_bits

*Function*

```rust
fn get_bits<T, U>(src: &[T], src_range: core::ops::Range<usize>) -> U
```



## sel4_bitfield_ops::set

*Function*

```rust
fn set<T, U>(dst: & mut [T], dst_start_bit: usize, src: U)
```



## sel4_bitfield_ops::set_bits

*Function*

```rust
fn set_bits<T, U>(dst: & mut [T], dst_range: core::ops::Range<usize>, src: U)
```



## sel4_bitfield_ops::set_bits_from_slice

*Function*

```rust
fn set_bits_from_slice<T, U>(dst: & mut [T], dst_range: core::ops::Range<usize>, src: &[U], src_start: usize)
```



