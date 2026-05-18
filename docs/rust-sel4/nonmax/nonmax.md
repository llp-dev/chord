**nonmax**

# Module: nonmax

## Contents

**Structs**

- [`NonMaxI128`](#nonmaxi128) - An integer that is known not to equal its maximum value.
- [`NonMaxI16`](#nonmaxi16) - An integer that is known not to equal its maximum value.
- [`NonMaxI32`](#nonmaxi32) - An integer that is known not to equal its maximum value.
- [`NonMaxI64`](#nonmaxi64) - An integer that is known not to equal its maximum value.
- [`NonMaxI8`](#nonmaxi8) - An integer that is known not to equal its maximum value.
- [`NonMaxIsize`](#nonmaxisize) - An integer that is known not to equal its maximum value.
- [`NonMaxU128`](#nonmaxu128) - An integer that is known not to equal its maximum value.
- [`NonMaxU16`](#nonmaxu16) - An integer that is known not to equal its maximum value.
- [`NonMaxU32`](#nonmaxu32) - An integer that is known not to equal its maximum value.
- [`NonMaxU64`](#nonmaxu64) - An integer that is known not to equal its maximum value.
- [`NonMaxU8`](#nonmaxu8) - An integer that is known not to equal its maximum value.
- [`NonMaxUsize`](#nonmaxusize) - An integer that is known not to equal its maximum value.
- [`ParseIntError`](#parseinterror) - An error type returned when an integer cannot be parsed (mimics [std::num::ParseIntError])
- [`TryFromIntError`](#tryfrominterror) - An error type returned when a checked integral type conversion fails (mimics [std::num::TryFromIntError])

---

## nonmax::NonMaxI128

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i128) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: i128) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> i128` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **From**
  - `fn from(small: u64) -> Self`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(small: u16) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: i64) -> Self`
- **From**
  - `fn from(small: NonMaxI32) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: i16) -> Self`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxI128) -> <Self as >::Output`
- **From**
  - `fn from(small: NonMaxU64) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxI128) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxI128`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **From**
  - `fn from(small: NonMaxU16) -> Self`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: u32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(small: u8) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: i32) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxI128)`
- **From**
  - `fn from(small: i8) -> Self`
- **TryFrom**
  - `fn try_from(value: i128) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(small: NonMaxI64) -> Self`
- **From**
  - `fn from(small: NonMaxI8) -> Self`
- **From**
  - `fn from(small: NonMaxI16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **From**
  - `fn from(small: NonMaxU32) -> Self`



## nonmax::NonMaxI16

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i16) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: i16) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> i16` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: i8) -> Self`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxI16) -> <Self as >::Output`
- **From**
  - `fn from(small: NonMaxI8) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxI16) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxI16`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: u8) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxI16)`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **TryFrom**
  - `fn try_from(value: i16) -> Result<Self, <Self as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`



## nonmax::NonMaxI32

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i32) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: i32) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> i32` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxI32) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxI32`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: u8) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxI32)`
- **From**
  - `fn from(small: i8) -> Self`
- **TryFrom**
  - `fn try_from(value: i32) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(small: NonMaxI8) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: u16) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: i16) -> Self`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxI32) -> <Self as >::Output`
- **From**
  - `fn from(small: NonMaxU16) -> Self`
- **From**
  - `fn from(small: NonMaxI16) -> Self`



## nonmax::NonMaxI64

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i64) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: i64) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> i64` - Returns the value as a primitive type.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(small: i16) -> Self`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxI64) -> <Self as >::Output`
- **From**
  - `fn from(small: NonMaxU32) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxI64) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxI64`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: NonMaxI16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(small: u16) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: i32) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxI64)`
- **From**
  - `fn from(small: i8) -> Self`
- **TryFrom**
  - `fn try_from(value: i64) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(small: NonMaxI8) -> Self`
- **From**
  - `fn from(small: NonMaxI32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **From**
  - `fn from(small: NonMaxU16) -> Self`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(small: u32) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: u8) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## nonmax::NonMaxI8

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i8) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: i8) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> i8` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxI8)`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **TryFrom**
  - `fn try_from(value: i8) -> Result<Self, <Self as >::Error>`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxI8) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxI8) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxI8`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## nonmax::NonMaxIsize

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: isize) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: isize) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> isize` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(small: NonMaxI8) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **From**
  - `fn from(small: i16) -> Self`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxIsize) -> <Self as >::Output`
- **From**
  - `fn from(small: NonMaxI16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxIsize) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxIsize`
- **From**
  - `fn from(small: u8) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **From**
  - `fn from(small: i8) -> Self`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxIsize)`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`



## nonmax::NonMaxU128

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u128) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: u128) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> u128` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxU128) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxU128) -> bool`
- **From**
  - `fn from(small: u32) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxU128`
- **From**
  - `fn from(small: u8) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: u128)`
- **From**
  - `fn from(small: NonMaxU64) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxU128)`
- **TryFrom**
  - `fn try_from(value: u128) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(small: u64) -> Self`
- **From**
  - `fn from(small: u16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **BitAnd**
  - `fn bitand(self: Self, rhs: u128) -> <Self as >::Output`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **From**
  - `fn from(small: NonMaxU32) -> Self`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: NonMaxU16) -> Self`
- **Default**
  - `fn default() -> Self`



## nonmax::NonMaxU16

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u16) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: u16) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> u16` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NonMaxU16`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: u16)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxU16)`
- **TryFrom**
  - `fn try_from(value: u16) -> Result<Self, <Self as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **BitAnd**
  - `fn bitand(self: Self, rhs: u16) -> <Self as >::Output`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxU16) -> <Self as >::Output`
- **From**
  - `fn from(small: u8) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxU16) -> bool`



## nonmax::NonMaxU32

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u32) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: u32) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> u32` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(small: NonMaxU16) -> Self`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxU32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxU32) -> bool`
- **From**
  - `fn from(small: u8) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxU32`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: u32)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxU32)`
- **TryFrom**
  - `fn try_from(value: u32) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(small: u16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **BitAnd**
  - `fn bitand(self: Self, rhs: u32) -> <Self as >::Output`



## nonmax::NonMaxU64

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u64) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: u64) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> u64` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxU64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxU64) -> bool`
- **From**
  - `fn from(small: u16) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxU64`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: u64)`
- **From**
  - `fn from(small: NonMaxU32) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **From**
  - `fn from(small: NonMaxU16) -> Self`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxU64)`
- **TryFrom**
  - `fn try_from(value: u64) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(small: u32) -> Self`
- **From**
  - `fn from(small: u8) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **BitAnd**
  - `fn bitand(self: Self, rhs: u64) -> <Self as >::Output`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`



## nonmax::NonMaxU8

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u8) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: u8) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> u8` - Returns the value as a primitive type.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxU8) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxU8`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: u8)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxU8)`
- **TryFrom**
  - `fn try_from(value: u8) -> Result<Self, <Self as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **BitAnd**
  - `fn bitand(self: Self, rhs: u8) -> <Self as >::Output`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxU8) -> <Self as >::Output`



## nonmax::NonMaxUsize

*Struct*

An integer that is known not to equal its maximum value.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: usize) -> Option<Self>` - Creates a new non-max if the given value is not the maximum
- `fn new_unchecked(value: usize) -> Self` - Creates a new non-max without checking the value.
- `fn get(self: &Self) -> usize` - Returns the value as a primitive type.

**Traits:** Copy, Eq

**Trait Implementations:**

- **BitAnd**
  - `fn bitand(self: Self, rhs: NonMaxUsize) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonMaxUsize) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> NonMaxUsize`
- **From**
  - `fn from(small: u8) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: usize)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(small: NonMaxU16) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: NonMaxUsize)`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **From**
  - `fn from(small: u16) -> Self`
- **From**
  - `fn from(small: NonMaxU8) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`
- **BitAnd**
  - `fn bitand(self: Self, rhs: usize) -> <Self as >::Output`
- **FromStr**
  - `fn from_str(value: &str) -> Result<Self, <Self as >::Err>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## nonmax::ParseIntError

*Struct*

An error type returned when an integer cannot be parsed (mimics [std::num::ParseIntError])

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(_: core::num::ParseIntError) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseIntError) -> bool`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParseIntError`



## nonmax::TryFromIntError

*Struct*

An error type returned when a checked integral type conversion fails (mimics [std::num::TryFromIntError])

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TryFromIntError`
- **From**
  - `fn from(_: core::num::TryFromIntError) -> Self`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TryFromIntError) -> bool`
- **From**
  - `fn from(never: core::convert::Infallible) -> Self`



