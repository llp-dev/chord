**bitfield**

# Module: bitfield

## Contents

**Macros**

- [`bitfield`](#bitfield) - Combines `bitfield_bitrange` and `bitfield_fields`.
- [`bitfield_bitrange`](#bitfield_bitrange) - Implements `BitRange` and `BitRangeMut` for a tuple struct (or "newtype").
- [`bitfield_impl`](#bitfield_impl) - Generates and dispatches trait implementations for a struct
- [`check_msb_lsb_order`](#check_msb_lsb_order) - Internal use macro, that `debug_assert` than msb >= lsb and thus they are not inverted

**Traits**

- [`Bit`](#bit) - A trait to get a single bit.
- [`BitMut`](#bitmut) - A trait to set a single bit.
- [`BitRange`](#bitrange) - A trait to get ranges of bits.
- [`BitRangeMut`](#bitrangemut) - A trait to set ranges of bits.

---

## bitfield::Bit

*Trait*

A trait to get a single bit.

This trait is implemented for all type that implement `BitRange<u8>`.

**Methods:**

- `bit`: Get a single bit.



## bitfield::BitMut

*Trait*

A trait to set a single bit.

This trait is implemented for all type that implement `BitRangeMut<u8>`.

**Methods:**

- `set_bit`: Set a single bit.



## bitfield::BitRange

*Trait*

A trait to get ranges of bits.

**Methods:**

- `bit_range`: Get a range of bits.



## bitfield::BitRangeMut

*Trait*

A trait to set ranges of bits.

**Methods:**

- `set_bit_range`: Set a range of bits.



## bitfield::bitfield

*Declarative Macro*

Combines `bitfield_bitrange` and `bitfield_fields`.

The syntax of this macro is the syntax of a tuple struct, including attributes and
documentation comments, followed by a semicolon, some optional elements, and finally the fields
as described in the `bitfield_fields` documentation.

The first optional element is `no default BitRange;`. With that, no implementation of
`BitRange` will be generated.

The second optional element is a set of lines of the form `impl <Trait>;`. The following traits are supported:
* `Debug`; This will generate an implementation of `fmt::Debug` with the `bitfield_debug` macro.
* `BitAnd`, `BitOr`, `BitXor`; These will generate implementations of the relevant `ops::Bit___` and `ops::Bit___Assign` traits.
* `new`; This will generate a constructor that calls all of the bitfield's setter methods with an argument of the appropriate type

The difference with calling those macros separately is that `bitfield_fields` is called
from an appropriate `impl` block. If you use the non-slice form of `bitfield_bitrange`, the
default type for `bitfield_fields` will be set to the wrapped fields.

See the documentation of these macros for more information on their respective syntax.

# Example

```rust
# use bitfield::bitfield;
# fn main() {}
bitfield!{
  pub struct BitField1(u16);
  impl Debug;
  // The fields default to u16
  field1, set_field1: 10, 0;
  pub field2, _ : 12, 3;
}
```

or with a custom `BitRange` and `BitRangeMut` implementation :
```rust
# use bitfield::{bitfield, BitRange, BitRangeMut};
# fn main() {}
bitfield!{
  pub struct BitField1(u16);
  no default BitRange;
  impl Debug;
  impl BitAnd;
  u8;
  field1, set_field1: 10, 0;
  pub field2, _ : 12, 3;
}
impl BitRange<u8> for BitField1 {
    fn bit_range(&self, msb: usize, lsb: usize) -> u8 {
        let width = msb - lsb + 1;
        let mask = (1 << width) - 1;
        ((self.0 >> lsb) & mask) as u8
    }
}
impl BitRangeMut<u8> for BitField1 {
    fn set_bit_range(&mut self, msb: usize, lsb: usize, value: u8) {
        self.0 = (value as u16) << lsb;
    }
}
```

```rust
macro_rules! bitfield {
    ($(#[$attribute:meta])* $vis:vis struct $name:ident($($type:tt)*); $(impl $trait:ident$({$($trait_arg:tt)*})?;)+ no default BitRange; $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident([$t:ty]); no default BitRange; impl $trait:ident$({$($trait_arg:tt)*})?; $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident([$t:ty]); no default BitRange; $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident([$t:ty]); $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident(MSB0 [$t:ty]); no default BitRange; $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident(MSB0 [$t:ty]); $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident($t:ty); no default BitRange; impl $trait:ident$({$($trait_arg:tt)*})?; $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident($t:ty); no default BitRange; $($rest:tt)*) => { ... };
    ($(#[$attribute:meta])* $vis:vis struct $name:ident($t:ty); $($rest:tt)*) => { ... };
}
```



## bitfield::bitfield_bitrange

*Declarative Macro*

Implements `BitRange` and `BitRangeMut` for a tuple struct (or "newtype").

This macro will generate an implementation of the `BitRange` trait for an existing single
element tuple struct.

The syntax is more or less the same as declaring a "newtype", **without** the attributes,
documentation comments and pub keyword.

The difference with a normal "newtype" is the type in parentheses. If the type is `[t]` (where
`t` is any of the unsigned integer type), the "newtype" will be generic and implement
`BitRange` for `T: AsRef<[t]>` and `BitRangeMut` for `T: AsMut<[t]>` (for example a slice, an array or a `Vec`). You can
also use `MSB0 [t]`. The difference will be the positions of the bit. You can use the
`bits_positions` example to see where each bits is. If the type is neither of this two, the
"newtype" will wrap a value of the specified type and implements `BitRange` the same ways as
the wrapped type.

# Examples

```rust
# use bitfield::bitfield_bitrange;
# fn main() {}
struct BitField1(u32);
bitfield_bitrange!{struct BitField1(u32)}

struct BitField2<T>(T);
bitfield_bitrange!{struct BitField2([u8])}

struct BitField3<T>(T);
bitfield_bitrange!{struct BitField3(MSB0 [u8])}
```


```rust
macro_rules! bitfield_bitrange {
    (@impl_bitrange_slice $name:ident, $slice_ty:ty, $bitrange_ty:ty) => { ... };
    (@impl_bitrange_slice_msb0 $name:ident, $slice_ty:ty, $bitrange_ty:ty) => { ... };
    (struct $name:ident([$t:ty])) => { ... };
    (struct $name:ident(MSB0 [$t:ty])) => { ... };
    (struct $name:ident($t:ty)) => { ... };
}
```



## bitfield::bitfield_impl

*Declarative Macro*

Generates and dispatches trait implementations for a struct

This must be called outside of any `impl` block.

The syntax is `TheNameOfTheTrait for struct TheNameOfTheStruct(TheInnerType);` followed by the syntax of bitfield_fields.

Supported traits:
* Debug
* BitAnd
* BitOr
* BitXor

Additional derivations:
* new
  * Creates a constructor, including parameters for all fields with a setter

```rust
macro_rules! bitfield_impl {
    (Debug for struct $name:ident([$t:ty]); $($rest:tt)*) => { ... };
    (Debug for struct $name:ident($t:ty); $($rest:tt)*) => { ... };
    (BitAnd for struct $name:ident([$t:ty]); $($rest:tt)*) => { ... };
    (BitAnd for struct $name:ident($t:ty); $($rest:tt)*) => { ... };
    (BitOr for struct $name:ident([$t:ty]); $($rest:tt)*) => { ... };
    (BitOr for struct $name:ident($t:ty); $($rest:tt)*) => { ... };
    (BitXor for struct $name:ident([$t:ty]); $($rest:tt)*) => { ... };
    (BitXor for struct $name:ident($t:ty); $($rest:tt)*) => { ... };
    (@bitwise $bitwise:ident $func:ident $bitwise_assign:ident $func_assign:ident $name:ident([$t:ty]) $op:tt) => { ... };
    (@bitwise $bitwise:ident $func:ident $bitwise_assign:ident $func_assign:ident $name:ident($t:ty) $op:tt) => { ... };
    (@mutate $self:ident $rhs:ident $op:tt) => { ... };
    (new for struct $name:ident([$t:ty]); $($rest:tt)*) => { ... };
    (new for struct $name:ident($t:ty); $($rest:tt)*) => { ... };
    (new{$new:ident ($($setter_name:ident: $setter_type:ty),*$(,)?)} for struct $name:ident([$t:ty]); $($rest:tt)*) => { ... };
    (new{$new:ident ($($setter_name:ident: $setter_type:ty),*$(,)?)} for struct $name:ident($t:ty); $($rest:tt)*) => { ... };
    ($macro:ident for struct $name:ident $($rest:tt)*) => { ... };
}
```



## bitfield::check_msb_lsb_order

*Declarative Macro*

Internal use macro, that `debug_assert` than msb >= lsb and thus they are not inverted

```rust
macro_rules! check_msb_lsb_order {
    ($msb:expr, $lsb:expr) => { ... };
}
```



