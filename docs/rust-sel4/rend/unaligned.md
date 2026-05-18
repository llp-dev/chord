**rend > unaligned**

# Module: unaligned

## Contents

**Structs**

- [`NonZeroI128_ube`](#nonzeroi128_ube) - A big-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.
- [`NonZeroI128_ule`](#nonzeroi128_ule) - A little-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.
- [`NonZeroI16_ube`](#nonzeroi16_ube) - A big-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.
- [`NonZeroI16_ule`](#nonzeroi16_ule) - A little-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.
- [`NonZeroI32_ube`](#nonzeroi32_ube) - A big-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.
- [`NonZeroI32_ule`](#nonzeroi32_ule) - A little-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.
- [`NonZeroI64_ube`](#nonzeroi64_ube) - A big-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.
- [`NonZeroI64_ule`](#nonzeroi64_ule) - A little-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.
- [`NonZeroU128_ube`](#nonzerou128_ube) - A big-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.
- [`NonZeroU128_ule`](#nonzerou128_ule) - A little-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.
- [`NonZeroU16_ube`](#nonzerou16_ube) - A big-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.
- [`NonZeroU16_ule`](#nonzerou16_ule) - A little-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.
- [`NonZeroU32_ube`](#nonzerou32_ube) - A big-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.
- [`NonZeroU32_ule`](#nonzerou32_ule) - A little-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.
- [`NonZeroU64_ube`](#nonzerou64_ube) - A big-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.
- [`NonZeroU64_ule`](#nonzerou64_ule) - A little-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.
- [`char_ube`](#char_ube) - A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.
- [`char_ule`](#char_ule) - A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.
- [`f32_ube`](#f32_ube) - A big-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.
- [`f32_ule`](#f32_ule) - A little-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.
- [`f64_ube`](#f64_ube) - A big-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.
- [`f64_ule`](#f64_ule) - A little-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.
- [`i128_ube`](#i128_ube) - A big-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.
- [`i128_ule`](#i128_ule) - A little-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.
- [`i16_ube`](#i16_ube) - A big-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.
- [`i16_ule`](#i16_ule) - A little-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.
- [`i32_ube`](#i32_ube) - A big-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.
- [`i32_ule`](#i32_ule) - A little-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.
- [`i64_ube`](#i64_ube) - A big-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.
- [`i64_ule`](#i64_ule) - A little-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.
- [`u128_ube`](#u128_ube) - A big-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.
- [`u128_ule`](#u128_ule) - A little-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.
- [`u16_ube`](#u16_ube) - A big-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.
- [`u16_ule`](#u16_ule) - A little-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.
- [`u32_ube`](#u32_ube) - A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.
- [`u32_ule`](#u32_ule) - A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.
- [`u64_ube`](#u64_ube) - A big-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.
- [`u64_ule`](#u64_ule) - A little-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.

---

## rend::unaligned::NonZeroI128_ube

*Struct*

A big-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI128) -> Self` - Returns a `NonZeroI128_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroI128` - Returns a `NonZeroI128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI128_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128_ube)`
- **From**
  - `fn from(value: NonZeroI128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`



## rend::unaligned::NonZeroI128_ule

*Struct*

A little-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI128) -> Self` - Returns a `NonZeroI128_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroI128` - Returns a `NonZeroI128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI128_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI128_ule)`
- **From**
  - `fn from(value: NonZeroI128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI128) -> <Self as >::Output`



## rend::unaligned::NonZeroI16_ube

*Struct*

A big-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI16) -> Self` - Returns a `NonZeroI16_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroI16` - Returns a `NonZeroI16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16_ube)`
- **From**
  - `fn from(value: NonZeroI16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI16_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::unaligned::NonZeroI16_ule

*Struct*

A little-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI16) -> Self` - Returns a `NonZeroI16_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroI16` - Returns a `NonZeroI16` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16_ule)`
- **From**
  - `fn from(value: NonZeroI16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI16_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI16_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::unaligned::NonZeroI32_ube

*Struct*

A big-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI32) -> Self` - Returns a `NonZeroI32_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroI32` - Returns a `NonZeroI32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32_ube)`
- **From**
  - `fn from(value: NonZeroI32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI32_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI32) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::unaligned::NonZeroI32_ule

*Struct*

A little-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI32) -> Self` - Returns a `NonZeroI32_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroI32` - Returns a `NonZeroI32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32_ule)`
- **From**
  - `fn from(value: NonZeroI32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI32_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI32_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI32) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::unaligned::NonZeroI64_ube

*Struct*

A big-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI64) -> Self` - Returns a `NonZeroI64_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroI64` - Returns a `NonZeroI64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(value: NonZeroI64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI64_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64_ube)`



## rend::unaligned::NonZeroI64_ule

*Struct*

A little-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: i64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: i64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> i64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroI64) -> Self` - Returns a `NonZeroI64_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroI64` - Returns a `NonZeroI64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroI64_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroI64_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroI64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroI64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroI64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroI64_ule)`
- **From**
  - `fn from(value: NonZeroI64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::unaligned::NonZeroU128_ube

*Struct*

A big-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU128) -> Self` - Returns a `NonZeroU128_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroU128` - Returns a `NonZeroU128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128_ube)`
- **From**
  - `fn from(value: NonZeroU128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU128_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128)`



## rend::unaligned::NonZeroU128_ule

*Struct*

A little-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u128) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u128) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u128` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU128) -> Self` - Returns a `NonZeroU128_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroU128` - Returns a `NonZeroU128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU128) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128_ule)`
- **From**
  - `fn from(value: NonZeroU128) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU128_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU128_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU128) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU128) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::unaligned::NonZeroU16_ube

*Struct*

A big-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU16) -> Self` - Returns a `NonZeroU16_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroU16` - Returns a `NonZeroU16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU16_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16_ube)`
- **From**
  - `fn from(value: NonZeroU16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output`



## rend::unaligned::NonZeroU16_ule

*Struct*

A little-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u16) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u16) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u16` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU16) -> Self` - Returns a `NonZeroU16_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroU16` - Returns a `NonZeroU16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(value: &'a NonZeroU16) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU16) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU16) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU16_ule)`
- **From**
  - `fn from(value: NonZeroU16) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU16_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU16_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::unaligned::NonZeroU32_ube

*Struct*

A big-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU32) -> Self` - Returns a `NonZeroU32_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroU32` - Returns a `NonZeroU32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU32) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32_ube)`
- **From**
  - `fn from(value: NonZeroU32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU32_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`



## rend::unaligned::NonZeroU32_ule

*Struct*

A little-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u32) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u32) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u32` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU32) -> Self` - Returns a `NonZeroU32_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroU32` - Returns a `NonZeroU32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU32) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU32_ule)`
- **From**
  - `fn from(value: NonZeroU32) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU32_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU32_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU32) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU32) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`



## rend::unaligned::NonZeroU64_ube

*Struct*

A big-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU64) -> Self` - Returns a `NonZeroU64_ube` containing `value`.
- `fn to_native(self: Self) -> NonZeroU64` - Returns a `NonZeroU64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64_ube)`
- **From**
  - `fn from(value: NonZeroU64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU64_ube) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`



## rend::unaligned::NonZeroU64_ule

*Struct*

A little-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: u64) -> Option<Self>` - Creates a non-zero if the given value is not zero.
- `fn new_unchecked(value: u64) -> Self` - Creates a non-zero without checking whether it is non-zero. This
- `fn get(self: Self) -> u64` - Returns the value as a primitive type.
- `fn from_native(value: NonZeroU64) -> Self` - Returns a `NonZeroU64_ule` containing `value`.
- `fn to_native(self: Self) -> NonZeroU64` - Returns a `NonZeroU64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &NonZeroU64) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: NonZeroU64_ule)`
- **From**
  - `fn from(value: NonZeroU64) -> Self`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: NonZeroU64_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &NonZeroU64_ule) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: &'a NonZeroU64) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &NonZeroU64) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`



## rend::unaligned::char_ube

*Struct*

A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: char) -> Self` - Returns a `char_ube` containing `value`.
- `fn to_native(self: Self) -> char` - Returns a `$prim` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(value: &'a char) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: char) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &char) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::unaligned::char_ule

*Struct*

A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: char) -> Self` - Returns a `char_ule` containing `value`.
- `fn to_native(self: Self) -> char` - Returns a `$prim` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(value: &'a char) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &char) -> Option<::core::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(value: char) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &char) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::unaligned::f32_ube

*Struct*

A big-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f32) -> Self` - Returns a `f32_ube` containing `value`.
- `fn to_native(self: Self) -> f32` - Returns a `f32` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Sub**
  - `fn sub(self: Self, other: f32_ube) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32_ube)`
- **Add**
  - `fn add(self: Self, other: f32_ube) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f32_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f32_ube) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32_ube)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32_ube)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32_ube)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f32) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32)`
- **Rem**
  - `fn rem(self: Self, other: &f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32_ube)`
- **Rem**
  - `fn rem(self: Self, other: f32_ube) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f32_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32)`
- **Div**
  - `fn div(self: Self, other: &f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32_ube)`
- **Div**
  - `fn div(self: Self, other: f32_ube) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f32_ube) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32)`
- **From**
  - `fn from(value: &'a f32) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32_ube)`
- **Mul**
  - `fn mul(self: Self, other: f32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f32_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32)`
- **Mul**
  - `fn mul(self: Self, other: &f32_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32_ube)`
- **Sub**
  - `fn sub(self: Self, other: f32) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32)`
- **Sub**
  - `fn sub(self: Self, other: &f32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f32) -> bool`
- **Add**
  - `fn add(self: Self, other: &f32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32)`
- **Product**
  - `fn product<I>(iter: I) -> Self`



## rend::unaligned::f32_ule

*Struct*

A little-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f32) -> Self` - Returns a `f32_ule` containing `value`.
- `fn to_native(self: Self) -> f32` - Returns a `f32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f32) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32)`
- **From**
  - `fn from(value: &'a f32) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32_ule)`
- **Mul**
  - `fn mul(self: Self, other: f32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f32_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32)`
- **Mul**
  - `fn mul(self: Self, other: &f32_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32_ule)`
- **Sub**
  - `fn sub(self: Self, other: f32) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32)`
- **Sub**
  - `fn sub(self: Self, other: &f32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f32) -> bool`
- **Add**
  - `fn add(self: Self, other: &f32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: f32_ule) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f32_ule)`
- **Add**
  - `fn add(self: Self, other: f32_ule) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f32_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f32_ule) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f32_ule)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f32_ule)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f32_ule)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f32) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f32)`
- **Rem**
  - `fn rem(self: Self, other: &f32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f32_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f32_ule)`
- **Rem**
  - `fn rem(self: Self, other: f32_ule) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f32_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f32)`
- **Div**
  - `fn div(self: Self, other: &f32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f32_ule)`
- **Div**
  - `fn div(self: Self, other: f32_ule) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f32_ule) -> <Self as >::Output`



## rend::unaligned::f64_ube

*Struct*

A big-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f64) -> Self` - Returns a `f64_ube` containing `value`.
- `fn to_native(self: Self) -> f64` - Returns a `f64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Mul**
  - `fn mul(self: Self, other: f64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f64_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64)`
- **Mul**
  - `fn mul(self: Self, other: &f64_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64_ube)`
- **Sub**
  - `fn sub(self: Self, other: f64) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64)`
- **Sub**
  - `fn sub(self: Self, other: &f64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f64) -> bool`
- **Add**
  - `fn add(self: Self, other: &f64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: f64_ube) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64_ube)`
- **Add**
  - `fn add(self: Self, other: f64_ube) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f64_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f64_ube) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64_ube)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64_ube)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64_ube)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f64) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64)`
- **Rem**
  - `fn rem(self: Self, other: &f64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64_ube)`
- **Rem**
  - `fn rem(self: Self, other: f64_ube) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f64_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64)`
- **Div**
  - `fn div(self: Self, other: &f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64_ube)`
- **Div**
  - `fn div(self: Self, other: f64_ube) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f64_ube) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64)`
- **From**
  - `fn from(value: &'a f64) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64_ube)`



## rend::unaligned::f64_ule

*Struct*

A little-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: f64) -> Self` - Returns a `f64_ule` containing `value`.
- `fn to_native(self: Self) -> f64` - Returns a `f64` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64)`
- **Rem**
  - `fn rem(self: Self, other: &f64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64_ule)`
- **Rem**
  - `fn rem(self: Self, other: f64_ule) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Div**
  - `fn div(self: Self, other: f64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &f64_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64)`
- **Div**
  - `fn div(self: Self, other: &f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64_ule)`
- **Div**
  - `fn div(self: Self, other: f64_ule) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Div**
  - `fn div(self: Self, other: &f64_ule) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &f64) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64)`
- **From**
  - `fn from(value: &'a f64) -> Self`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: f64_ule)`
- **Mul**
  - `fn mul(self: Self, other: f64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: f64_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &f64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: f64_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64)`
- **Mul**
  - `fn mul(self: Self, other: &f64_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &f64_ule)`
- **Sub**
  - `fn sub(self: Self, other: f64) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: f64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64)`
- **Sub**
  - `fn sub(self: Self, other: &f64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &f64) -> bool`
- **Add**
  - `fn add(self: Self, other: &f64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: f64_ule) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &f64_ule)`
- **Add**
  - `fn add(self: Self, other: f64_ule) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &f64_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &f64_ule) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &f64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &f64)`
- **Default**
  - `fn default() -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: f64_ule)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: f64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &f64_ule)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: f64_ule)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **From**
  - `fn from(value: f64) -> Self`
- **Rem**
  - `fn rem(self: Self, other: f64) -> <Self as >::Output`



## rend::unaligned::i128_ube

*Struct*

A big-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i128) -> Self` - Returns a `i128_ube` containing `value`.
- `fn to_native(self: Self) -> i128` - Returns a `i128` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitAnd**
  - `fn bitand(self: Self, other: &i128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128)`
- **Shr**
  - `fn shr(self: Self, other: i128_ube) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i128_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128_ube)`
- **Shr**
  - `fn shr(self: Self, other: &i128_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i128_ube) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128_ube)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128)`
- **Add**
  - `fn add(self: Self, other: &i128) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i128) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128_ube) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i128_ube) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i128_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128_ube)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128_ube)`
- **Shl**
  - `fn shl(self: Self, other: i128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128_ube)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128)`
- **From**
  - `fn from(value: i128) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128_ube)`
- **Sub**
  - `fn sub(self: Self, other: i128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128_ube)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i128_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128)`
- **Sub**
  - `fn sub(self: Self, other: &i128) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i128_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128_ube)`
- **Mul**
  - `fn mul(self: Self, other: i128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128)`
- **Sub**
  - `fn sub(self: Self, other: i128_ube) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128_ube)`
- **Div**
  - `fn div(self: Self, other: i128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i128_ube) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i128_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128)`
- **Mul**
  - `fn mul(self: Self, other: i128_ube) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i128_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i128_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128_ube)`
- **Div**
  - `fn div(self: Self, other: &i128_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128_ube)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128_ube)`
- **From**
  - `fn from(value: &'a i128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128_ube)`
- **Rem**
  - `fn rem(self: Self, other: i128) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128_ube)`
- **Rem**
  - `fn rem(self: Self, other: &i128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128_ube)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i128) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128)`
- **Rem**
  - `fn rem(self: Self, other: i128_ube) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128)`
- **Shr**
  - `fn shr(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128_ube)`
- **Rem**
  - `fn rem(self: Self, other: &i128_ube) -> <Self as >::Output`



## rend::unaligned::i128_ule

*Struct*

A little-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i128) -> Self` - Returns a `i128_ule` containing `value`.
- `fn to_native(self: Self) -> i128` - Returns a `i128` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128)`
- **From**
  - `fn from(value: i128) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128_ule)`
- **Sub**
  - `fn sub(self: Self, other: i128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128_ule)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i128_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128)`
- **Sub**
  - `fn sub(self: Self, other: &i128) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i128_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128_ule)`
- **Mul**
  - `fn mul(self: Self, other: i128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128)`
- **Sub**
  - `fn sub(self: Self, other: i128_ule) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128_ule)`
- **Div**
  - `fn div(self: Self, other: i128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i128_ule) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i128_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i128_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128)`
- **Mul**
  - `fn mul(self: Self, other: i128_ule) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i128_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i128_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128_ule)`
- **Div**
  - `fn div(self: Self, other: &i128_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i128_ule)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i128_ule)`
- **From**
  - `fn from(value: &'a i128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i128) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i128_ule)`
- **Rem**
  - `fn rem(self: Self, other: i128) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i128_ule)`
- **Rem**
  - `fn rem(self: Self, other: &i128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i128_ule)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i128) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128)`
- **Rem**
  - `fn rem(self: Self, other: i128_ule) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128)`
- **Shr**
  - `fn shr(self: Self, other: &i128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i128_ule)`
- **Rem**
  - `fn rem(self: Self, other: &i128_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i128_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128)`
- **Shr**
  - `fn shr(self: Self, other: i128_ule) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i128_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i128_ule)`
- **Shr**
  - `fn shr(self: Self, other: &i128_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i128)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i128_ule) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i128_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i128)`
- **Add**
  - `fn add(self: Self, other: &i128) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i128) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i128)`
- **BitXor**
  - `fn bitxor(self: Self, other: i128_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i128_ule) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i128_ule) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i128_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i128)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i128_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i128)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i128_ule)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i128_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i128)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i128_ule)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i128_ule)`
- **Shl**
  - `fn shl(self: Self, other: i128) -> <Self as >::Output`



## rend::unaligned::i16_ube

*Struct*

A big-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i16) -> Self` - Returns a `i16_ube` containing `value`.
- `fn to_native(self: Self) -> i16` - Returns a `i16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16_ube)`
- **From**
  - `fn from(value: &'a i16) -> Self`
- **Sub**
  - `fn sub(self: Self, other: i16) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i16_ube) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16_ube)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i16_ube) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: i16) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16_ube)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16)`
- **Sub**
  - `fn sub(self: Self, other: i16_ube) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Mul**
  - `fn mul(self: Self, other: &i16) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i16_ube) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16)`
- **Mul**
  - `fn mul(self: Self, other: i16_ube) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16_ube)`
- **Mul**
  - `fn mul(self: Self, other: &i16_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: i16) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16_ube)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16)`
- **Add**
  - `fn add(self: Self, other: i16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i16_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16)`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16_ube) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16_ube)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16_ube)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Add**
  - `fn add(self: Self, other: &i16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16)`
- **BitXor**
  - `fn bitxor(self: Self, other: i16_ube) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering>`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: i16_ube) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Rem**
  - `fn rem(self: Self, other: i16) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &i16_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16)`
- **Rem**
  - `fn rem(self: Self, other: &i16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16_ube)`
- **Shr**
  - `fn shr(self: Self, other: i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16)`
- **Rem**
  - `fn rem(self: Self, other: i16_ube) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &i16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16_ube)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16_ube)`
- **Rem**
  - `fn rem(self: Self, other: &i16_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16)`
- **Shr**
  - `fn shr(self: Self, other: i16_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16_ube)`
- **Shr**
  - `fn shr(self: Self, other: &i16_ube) -> <Self as >::Output`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16_ube)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16)`
- **From**
  - `fn from(value: i16) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &i16) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16_ube)`
- **BitOr**
  - `fn bitor(self: Self, other: i16) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &i16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16_ube)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16_ube)`
- **Div**
  - `fn div(self: Self, other: i16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i16_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i16_ube) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16_ube)`
- **Div**
  - `fn div(self: Self, other: &i16) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16)`
- **Div**
  - `fn div(self: Self, other: i16_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16_ube)`
- **Div**
  - `fn div(self: Self, other: &i16_ube) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: i16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16_ube)`



## rend::unaligned::i16_ule

*Struct*

A little-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i16) -> Self` - Returns a `i16_ule` containing `value`.
- `fn to_native(self: Self) -> i16` - Returns a `i16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16_ule)`
- **Mul**
  - `fn mul(self: Self, other: i16_ule) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i16_ule) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i16_ule)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i16_ule)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16_ule)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16)`
- **From**
  - `fn from(value: i16) -> Self`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i16_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16)`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i16_ule)`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i16) -> Option<::core::cmp::Ordering>`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16)`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: i16) -> <Self as >::Output`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Sub**
  - `fn sub(self: Self, other: &i16) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16)`
- **BitXor**
  - `fn bitxor(self: Self, other: i16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16)`
- **Sub**
  - `fn sub(self: Self, other: i16_ule) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: i16) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i16_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i16_ule)`
- **Shr**
  - `fn shr(self: Self, other: &i16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i16_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: i16_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i16) -> <Self as >::Output`
- **From**
  - `fn from(value: &'a i16) -> Self`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16)`
- **BitXor**
  - `fn bitxor(self: Self, other: &i16_ule) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: i16_ule) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: i16) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &i16_ule) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16)`
- **BitOr**
  - `fn bitor(self: Self, other: &i16) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i16_ule)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i16) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i16_ule)`
- **BitOr**
  - `fn bitor(self: Self, other: i16_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i16) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i16_ule) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: i16_ule) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: i16) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: &i16_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i16_ule)`
- **Add**
  - `fn add(self: Self, other: i16) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: &i16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i16_ule)`
- **BitAnd**
  - `fn bitand(self: Self, other: i16_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i16_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &i16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i16_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i16_ule) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: i16_ule) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i16)`
- **Rem**
  - `fn rem(self: Self, other: &i16_ule) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i16_ule)`
- **Add**
  - `fn add(self: Self, other: i16_ule) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i16_ule)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i16_ule)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i16_ule) -> <Self as >::Output`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i16_ule)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16)`
- **PartialEq**
  - `fn eq(self: &Self, other: &i16) -> bool`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i16_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i16_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i16_ule)`
- **Default**
  - `fn default() -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: i16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16)`
- **Mul**
  - `fn mul(self: Self, other: &i16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i16_ule)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i16)`



## rend::unaligned::i32_ube

*Struct*

A big-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i32) -> Self` - Returns a `i32_ube` containing `value`.
- `fn to_native(self: Self) -> i32` - Returns a `i32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32)`
- **Mul**
  - `fn mul(self: Self, other: i32_ube) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i32_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i32_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32_ube)`
- **Div**
  - `fn div(self: Self, other: &i32_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32_ube)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32_ube)`
- **From**
  - `fn from(value: &'a i32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32_ube)`
- **Rem**
  - `fn rem(self: Self, other: i32) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32_ube)`
- **Rem**
  - `fn rem(self: Self, other: &i32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32_ube)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i32) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32)`
- **Rem**
  - `fn rem(self: Self, other: i32_ube) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32)`
- **Shr**
  - `fn shr(self: Self, other: &i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32_ube)`
- **Rem**
  - `fn rem(self: Self, other: &i32_ube) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32)`
- **Shr**
  - `fn shr(self: Self, other: i32_ube) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i32_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32_ube)`
- **Shr**
  - `fn shr(self: Self, other: &i32_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32_ube) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32_ube)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32)`
- **Add**
  - `fn add(self: Self, other: &i32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i32) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32_ube) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i32_ube) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i32_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32_ube)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32_ube)`
- **Shl**
  - `fn shl(self: Self, other: i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32_ube)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32)`
- **From**
  - `fn from(value: i32) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32_ube)`
- **Sub**
  - `fn sub(self: Self, other: i32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32_ube)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i32_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32)`
- **Sub**
  - `fn sub(self: Self, other: &i32) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i32_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32_ube)`
- **Mul**
  - `fn mul(self: Self, other: i32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32)`
- **Sub**
  - `fn sub(self: Self, other: i32_ube) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32_ube)`
- **Div**
  - `fn div(self: Self, other: i32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i32_ube) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i32_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i32) -> <Self as >::Output`



## rend::unaligned::i32_ule

*Struct*

A little-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i32) -> Self` - Returns a `i32_ule` containing `value`.
- `fn to_native(self: Self) -> i32` - Returns a `i32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Add**
  - `fn add(self: Self, other: i32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i32_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32_ule)`
- **Shr**
  - `fn shr(self: Self, other: &i32_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32_ule) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32)`
- **Add**
  - `fn add(self: Self, other: &i32) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i32) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i32_ule) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i32_ule) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i32_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32_ule)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32_ule)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32_ule)`
- **Shl**
  - `fn shl(self: Self, other: i32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i32_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32)`
- **From**
  - `fn from(value: i32) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i32_ule)`
- **Sub**
  - `fn sub(self: Self, other: i32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i32_ule)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i32_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32)`
- **Sub**
  - `fn sub(self: Self, other: &i32) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i32_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i32_ule)`
- **Mul**
  - `fn mul(self: Self, other: i32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32)`
- **Sub**
  - `fn sub(self: Self, other: i32_ule) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i32_ule)`
- **Div**
  - `fn div(self: Self, other: i32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i32_ule) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i32_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i32_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32)`
- **Mul**
  - `fn mul(self: Self, other: i32_ule) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i32_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i32_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i32_ule)`
- **Div**
  - `fn div(self: Self, other: &i32_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i32_ule)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i32_ule)`
- **From**
  - `fn from(value: &'a i32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i32) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i32_ule)`
- **Rem**
  - `fn rem(self: Self, other: i32) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i32)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i32_ule)`
- **Rem**
  - `fn rem(self: Self, other: &i32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i32_ule)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i32) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i32)`
- **Rem**
  - `fn rem(self: Self, other: i32_ule) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i32)`
- **Shr**
  - `fn shr(self: Self, other: &i32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i32_ule)`
- **Rem**
  - `fn rem(self: Self, other: &i32_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i32_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: i32) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i32)`
- **Shr**
  - `fn shr(self: Self, other: i32_ule) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`



## rend::unaligned::i64_ube

*Struct*

A big-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i64) -> Self` - Returns a `i64_ube` containing `value`.
- `fn to_native(self: Self) -> i64` - Returns a `i64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64_ube)`
- **Rem**
  - `fn rem(self: Self, other: i64) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64_ube)`
- **Rem**
  - `fn rem(self: Self, other: &i64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64_ube)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i64) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64)`
- **Rem**
  - `fn rem(self: Self, other: i64_ube) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64)`
- **Shr**
  - `fn shr(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64_ube)`
- **Rem**
  - `fn rem(self: Self, other: &i64_ube) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64)`
- **Shr**
  - `fn shr(self: Self, other: i64_ube) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i64_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64_ube)`
- **Shr**
  - `fn shr(self: Self, other: &i64_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64_ube) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64_ube)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64)`
- **Add**
  - `fn add(self: Self, other: &i64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i64) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64_ube) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i64_ube) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i64_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64)`
- **Default**
  - `fn default() -> Self`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64_ube)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64_ube)`
- **Shl**
  - `fn shl(self: Self, other: i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64_ube)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64)`
- **From**
  - `fn from(value: i64) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64_ube)`
- **Sub**
  - `fn sub(self: Self, other: i64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64_ube)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i64_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64)`
- **Sub**
  - `fn sub(self: Self, other: &i64) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i64_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64_ube)`
- **Mul**
  - `fn mul(self: Self, other: i64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64)`
- **Sub**
  - `fn sub(self: Self, other: i64_ube) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64_ube)`
- **Div**
  - `fn div(self: Self, other: i64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i64_ube) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i64_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64)`
- **Mul**
  - `fn mul(self: Self, other: i64_ube) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i64_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i64_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64_ube)`
- **Div**
  - `fn div(self: Self, other: &i64_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64_ube)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64_ube)`
- **From**
  - `fn from(value: &'a i64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering>`



## rend::unaligned::i64_ule

*Struct*

A little-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: i64) -> Self` - Returns a `i64_ule` containing `value`.
- `fn to_native(self: Self) -> i64` - Returns a `i64` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64_ule)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64_ule)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64_ule)`
- **Shl**
  - `fn shl(self: Self, other: i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64)`
- **From**
  - `fn from(value: i64) -> Self`
- **Shl**
  - `fn shl(self: Self, other: &i64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64_ule)`
- **Sub**
  - `fn sub(self: Self, other: i64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64_ule)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: i64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: isize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: i64_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64)`
- **Sub**
  - `fn sub(self: Self, other: &i64) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &i64_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64_ule)`
- **Mul**
  - `fn mul(self: Self, other: i64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64)`
- **Sub**
  - `fn sub(self: Self, other: i64_ule) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &i64_ule)`
- **Div**
  - `fn div(self: Self, other: i64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: i64_ule) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &i64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &i64_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &i64_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64)`
- **Mul**
  - `fn mul(self: Self, other: i64_ule) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64)`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: &i64_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: i64_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: i64_ule)`
- **Div**
  - `fn div(self: Self, other: &i64_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: i64_ule)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: i64_ule)`
- **From**
  - `fn from(value: &'a i64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &i64) -> Option<::core::cmp::Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: i64_ule)`
- **Rem**
  - `fn rem(self: Self, other: i64) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: i64)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: i64_ule)`
- **Rem**
  - `fn rem(self: Self, other: &i64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &i64_ule)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Shr**
  - `fn shr(self: Self, other: i64) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: i64)`
- **Rem**
  - `fn rem(self: Self, other: i64_ule) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: i64)`
- **Shr**
  - `fn shr(self: Self, other: &i64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &i64_ule)`
- **Rem**
  - `fn rem(self: Self, other: &i64_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &i64_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: i64)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &i64)`
- **Shr**
  - `fn shr(self: Self, other: i64_ule) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Add**
  - `fn add(self: Self, other: i64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: i64_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &i64_ule)`
- **Shr**
  - `fn shr(self: Self, other: &i64_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: i64)`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &i64_ule) -> <Self as >::Output`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &i64_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &i64)`
- **Add**
  - `fn add(self: Self, other: &i64) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &i64) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &i64)`
- **BitXor**
  - `fn bitxor(self: Self, other: i64_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &i64_ule) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **Add**
  - `fn add(self: Self, other: i64_ule) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &i64_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &i64)`
- **Default**
  - `fn default() -> Self`



## rend::unaligned::u128_ube

*Struct*

A big-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u128) -> Self` - Returns a `u128_ube` containing `value`.
- `fn to_native(self: Self) -> u128` - Returns a `u128` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u128_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128_ube)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128_ube)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128_ube)`
- **Shl**
  - `fn shl(self: Self, other: u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128_ube)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128_ube)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128)`
- **From**
  - `fn from(value: u128) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u128) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128_ube)`
- **BitOr**
  - `fn bitor(self: Self, other: u128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u128_ube) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128)`
- **Shl**
  - `fn shl(self: Self, other: &u128_ube) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128_ube)`
- **Mul**
  - `fn mul(self: Self, other: u128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128)`
- **Sub**
  - `fn sub(self: Self, other: u128_ube) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128_ube)`
- **Div**
  - `fn div(self: Self, other: u128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u128_ube) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u128_ube) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128)`
- **Div**
  - `fn div(self: Self, other: &u128) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u128_ube) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128)`
- **Mul**
  - `fn mul(self: Self, other: &u128_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128_ube)`
- **Div**
  - `fn div(self: Self, other: u128_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u128_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128_ube)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128_ube)`
- **From**
  - `fn from(value: &'a u128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128_ube)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128_ube)`
- **Rem**
  - `fn rem(self: Self, other: &u128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128_ube)`
- **Shr**
  - `fn shr(self: Self, other: u128) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128)`
- **BitAnd**
  - `fn bitand(self: Self, other: u128) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u128_ube) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128_ube)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128)`
- **Rem**
  - `fn rem(self: Self, other: &u128_ube) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128)`
- **Shr**
  - `fn shr(self: Self, other: u128_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u128_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u128_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128_ube) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128_ube)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u128) -> bool`
- **Add**
  - `fn add(self: Self, other: &u128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128_ube) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128_ube) -> <Self as >::Output`



## rend::unaligned::u128_ule

*Struct*

A little-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u128) -> Self` - Returns a `u128_ule` containing `value`.
- `fn to_native(self: Self) -> u128` - Returns a `u128` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128_ule)`
- **Div**
  - `fn div(self: Self, other: u128) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u128_ule) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u128) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u128_ule) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128)`
- **Div**
  - `fn div(self: Self, other: &u128) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u128_ule) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128)`
- **Mul**
  - `fn mul(self: Self, other: &u128_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128_ule)`
- **Div**
  - `fn div(self: Self, other: u128_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u128_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128_ule)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128_ule)`
- **From**
  - `fn from(value: &'a u128) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u128) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128_ule)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128_ule)`
- **Rem**
  - `fn rem(self: Self, other: &u128) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128_ule)`
- **Shr**
  - `fn shr(self: Self, other: u128) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128)`
- **BitAnd**
  - `fn bitand(self: Self, other: u128) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u128_ule) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u128) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u128_ule)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128)`
- **Rem**
  - `fn rem(self: Self, other: &u128_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u128_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128)`
- **Shr**
  - `fn shr(self: Self, other: u128_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u128_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u128_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u128_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u128_ule) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u128_ule)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u128) -> bool`
- **Add**
  - `fn add(self: Self, other: &u128) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128)`
- **BitXor**
  - `fn bitxor(self: Self, other: u128_ule) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u128_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u128_ule) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u128_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u128_ule)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u128)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u128_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u128_ule)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u128)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u128_ule)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u128_ule)`
- **Shl**
  - `fn shl(self: Self, other: u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u128)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u128_ule)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u128) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u128_ule)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u128)`
- **From**
  - `fn from(value: u128) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u128) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u128_ule)`
- **BitOr**
  - `fn bitor(self: Self, other: u128) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u128_ule) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u128)`
- **Shl**
  - `fn shl(self: Self, other: &u128_ule) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u128) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u128_ule)`
- **Mul**
  - `fn mul(self: Self, other: u128) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u128)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u128)`
- **Sub**
  - `fn sub(self: Self, other: u128_ule) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## rend::unaligned::u16_ube

*Struct*

A big-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u16) -> Self` - Returns a `u16_ube` containing `value`.
- `fn to_native(self: Self) -> u16` - Returns a `u16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16)`
- **Sub**
  - `fn sub(self: Self, other: u16_ube) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16_ube)`
- **Sub**
  - `fn sub(self: Self, other: &u16_ube) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Mul**
  - `fn mul(self: Self, other: &u16) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16)`
- **Mul**
  - `fn mul(self: Self, other: u16_ube) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16_ube)`
- **Mul**
  - `fn mul(self: Self, other: &u16_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16)`
- **Add**
  - `fn add(self: Self, other: u16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u16_ube) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16_ube)`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16_ube) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16_ube)`
- **Add**
  - `fn add(self: Self, other: &u16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16_ube) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering>`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16_ube) -> <Self as >::Output`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Add**
  - `fn add(self: Self, other: u16_ube) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u16) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &u16_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16)`
- **Default**
  - `fn default() -> Self`
- **Rem**
  - `fn rem(self: Self, other: &u16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16)`
- **Rem**
  - `fn rem(self: Self, other: u16_ube) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16_ube)`
- **Rem**
  - `fn rem(self: Self, other: &u16_ube) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16)`
- **Shr**
  - `fn shr(self: Self, other: &u16_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16_ube)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16_ube)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u16) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16)`
- **From**
  - `fn from(value: u16) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16_ube)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOr**
  - `fn bitor(self: Self, other: u16) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16_ube)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16_ube)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16_ube)`
- **Div**
  - `fn div(self: Self, other: u16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u16_ube) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16_ube)`
- **BitOr**
  - `fn bitor(self: Self, other: &u16_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16)`
- **Div**
  - `fn div(self: Self, other: u16_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16_ube)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16_ube)`
- **Shl**
  - `fn shl(self: Self, other: u16) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16_ube) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16)`
- **Shl**
  - `fn shl(self: Self, other: &u16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16_ube)`
- **Sub**
  - `fn sub(self: Self, other: u16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16_ube)`
- **From**
  - `fn from(value: &'a u16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u16_ube) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u16) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: &u16_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16_ube)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: u16) -> <Self as >::Output`



## rend::unaligned::u16_ule

*Struct*

A little-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u16) -> Self` - Returns a `u16_ule` containing `value`.
- `fn to_native(self: Self) -> u16` - Returns a `u16` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16_ule)`
- **BitOr**
  - `fn bitor(self: Self, other: &u16_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16) -> <Self as >::Output`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16)`
- **Div**
  - `fn div(self: Self, other: u16_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16_ule)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16_ule)`
- **Shl**
  - `fn shl(self: Self, other: u16) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u16_ule) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16)`
- **Shl**
  - `fn shl(self: Self, other: &u16) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16_ule)`
- **Sub**
  - `fn sub(self: Self, other: u16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16_ule)`
- **From**
  - `fn from(value: &'a u16) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u16_ule) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u16) -> <Self as >::Output`
- **Shl**
  - `fn shl(self: Self, other: &u16_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16_ule)`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Mul**
  - `fn mul(self: Self, other: u16) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16)`
- **Sub**
  - `fn sub(self: Self, other: u16_ule) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16_ule)`
- **Sub**
  - `fn sub(self: Self, other: &u16_ule) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Mul**
  - `fn mul(self: Self, other: &u16) -> <Self as >::Output`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16)`
- **Mul**
  - `fn mul(self: Self, other: u16_ule) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u16_ule)`
- **Mul**
  - `fn mul(self: Self, other: &u16_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u16_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16) -> <Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16)`
- **Add**
  - `fn add(self: Self, other: u16) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u16_ule) -> <Self as >::Output`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u16_ule)`
- **BitAnd**
  - `fn bitand(self: Self, other: &u16_ule) -> <Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u16_ule)`
- **Add**
  - `fn add(self: Self, other: &u16) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16)`
- **BitXor**
  - `fn bitxor(self: Self, other: u16_ule) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u16) -> Option<::core::cmp::Ordering>`
- **BitXor**
  - `fn bitxor(self: Self, other: &u16_ule) -> <Self as >::Output`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Add**
  - `fn add(self: Self, other: u16_ule) -> <Self as >::Output`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u16) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: &u16_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16)`
- **Default**
  - `fn default() -> Self`
- **Rem**
  - `fn rem(self: Self, other: &u16) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u16_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u16)`
- **Rem**
  - `fn rem(self: Self, other: u16_ule) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u16) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u16_ule)`
- **Rem**
  - `fn rem(self: Self, other: &u16_ule) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u16_ule)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u16)`
- **Shr**
  - `fn shr(self: Self, other: u16_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u16)`
- **Shr**
  - `fn shr(self: Self, other: &u16_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u16_ule)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u16_ule)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u16)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u16) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u16)`
- **From**
  - `fn from(value: u16) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u16_ule)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitOr**
  - `fn bitor(self: Self, other: u16) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u16)`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u16) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u16_ule)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u16_ule)`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u16)`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u16_ule)`
- **Div**
  - `fn div(self: Self, other: u16) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u16_ule) -> <Self as >::Output`



## rend::unaligned::u32_ube

*Struct*

A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u32) -> Self` - Returns a `u32_ube` containing `value`.
- `fn to_native(self: Self) -> u32` - Returns a `u32` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32_ube)`
- **From**
  - `fn from(value: &'a u32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32_ube)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32_ube)`
- **Rem**
  - `fn rem(self: Self, other: &u32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32_ube)`
- **Shr**
  - `fn shr(self: Self, other: u32) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32)`
- **BitAnd**
  - `fn bitand(self: Self, other: u32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u32_ube) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32_ube)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32)`
- **Rem**
  - `fn rem(self: Self, other: &u32_ube) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32)`
- **Shr**
  - `fn shr(self: Self, other: u32_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u32_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u32_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u32_ube) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32_ube)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u32) -> bool`
- **Add**
  - `fn add(self: Self, other: &u32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32_ube) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u32_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32_ube) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u32_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32_ube)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32_ube)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32_ube)`
- **Shl**
  - `fn shl(self: Self, other: u32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32_ube)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32_ube)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32)`
- **From**
  - `fn from(value: u32) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u32) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32_ube)`
- **BitOr**
  - `fn bitor(self: Self, other: u32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u32_ube) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32)`
- **Shl**
  - `fn shl(self: Self, other: &u32_ube) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32_ube)`
- **Mul**
  - `fn mul(self: Self, other: u32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32)`
- **Sub**
  - `fn sub(self: Self, other: u32_ube) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32_ube)`
- **Div**
  - `fn div(self: Self, other: u32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u32_ube) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u32_ube) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32)`
- **Div**
  - `fn div(self: Self, other: &u32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u32_ube) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32)`
- **Mul**
  - `fn mul(self: Self, other: &u32_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32_ube)`
- **Div**
  - `fn div(self: Self, other: u32_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u32_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32_ube)`



## rend::unaligned::u32_ule

*Struct*

A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u32) -> Self` - Returns a `u32_ule` containing `value`.
- `fn to_native(self: Self) -> u32` - Returns a `u32` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **BitXor**
  - `fn bitxor(self: Self, other: &u32_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32_ule) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u32_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32_ule)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32_ule)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32_ule)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32_ule)`
- **Shl**
  - `fn shl(self: Self, other: u32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32_ule)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u32) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32_ule)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32)`
- **From**
  - `fn from(value: u32) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u32) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32_ule)`
- **BitOr**
  - `fn bitor(self: Self, other: u32) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u32_ule) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32)`
- **Shl**
  - `fn shl(self: Self, other: &u32_ule) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u32_ule)`
- **Mul**
  - `fn mul(self: Self, other: u32) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32)`
- **Sub**
  - `fn sub(self: Self, other: u32_ule) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u32_ule)`
- **Div**
  - `fn div(self: Self, other: u32) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u32_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u32_ule) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u32) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u32_ule) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32)`
- **Div**
  - `fn div(self: Self, other: &u32) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u32_ule) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32)`
- **Mul**
  - `fn mul(self: Self, other: &u32_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u32_ule)`
- **Div**
  - `fn div(self: Self, other: u32_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u32_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u32_ule)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u32_ule)`
- **From**
  - `fn from(value: &'a u32) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u32) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u32) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u32_ule)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u32)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u32_ule)`
- **Rem**
  - `fn rem(self: Self, other: &u32) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u32_ule)`
- **Shr**
  - `fn shr(self: Self, other: u32) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u32)`
- **BitAnd**
  - `fn bitand(self: Self, other: u32) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u32_ule) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u32) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u32_ule)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u32)`
- **Rem**
  - `fn rem(self: Self, other: &u32_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u32) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u32_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u32)`
- **Shr**
  - `fn shr(self: Self, other: u32_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u32)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u32_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u32) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u32_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u32_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u32) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u32)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u32_ule) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u32)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u32_ule)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u32) -> bool`
- **Add**
  - `fn add(self: Self, other: &u32) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u32)`
- **BitXor**
  - `fn bitxor(self: Self, other: u32_ule) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`



## rend::unaligned::u64_ube

*Struct*

A big-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u64) -> Self` - Returns a `u64_ube` containing `value`.
- `fn to_native(self: Self) -> u64` - Returns a `u64` with the same value as `self`.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Shr**
  - `fn shr(self: Self, other: &u64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64_ube)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64)`
- **Rem**
  - `fn rem(self: Self, other: &u64_ube) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64)`
- **Shr**
  - `fn shr(self: Self, other: u64_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u64_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u64_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64_ube)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64_ube) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64_ube)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u64) -> bool`
- **Add**
  - `fn add(self: Self, other: &u64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64_ube) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64_ube) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64_ube) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u64_ube) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64_ube)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64_ube)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64_ube)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64_ube)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64_ube)`
- **Shl**
  - `fn shl(self: Self, other: u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64_ube)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64_ube)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64)`
- **From**
  - `fn from(value: u64) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u64) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64_ube)`
- **BitOr**
  - `fn bitor(self: Self, other: u64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u64_ube) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64)`
- **Shl**
  - `fn shl(self: Self, other: &u64_ube) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64_ube)`
- **Mul**
  - `fn mul(self: Self, other: u64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64)`
- **Sub**
  - `fn sub(self: Self, other: u64_ube) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64_ube)`
- **Div**
  - `fn div(self: Self, other: u64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64_ube) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u64_ube) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u64_ube) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64)`
- **Div**
  - `fn div(self: Self, other: &u64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u64_ube) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64)`
- **Mul**
  - `fn mul(self: Self, other: &u64_ube) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64_ube)`
- **Div**
  - `fn div(self: Self, other: u64_ube) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u64_ube) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64_ube)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64_ube)`
- **From**
  - `fn from(value: &'a u64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64_ube)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64_ube)`
- **Rem**
  - `fn rem(self: Self, other: &u64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64_ube)`
- **Shr**
  - `fn shr(self: Self, other: u64) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64)`
- **BitAnd**
  - `fn bitand(self: Self, other: u64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u64_ube) -> <Self as >::Output`



## rend::unaligned::u64_ule

*Struct*

A little-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_native(value: u64) -> Self` - Returns a `u64_ule` containing `value`.
- `fn to_native(self: Self) -> u64` - Returns a `u64` with the same value as `self`.

**Traits:** Copy, Eq

**Trait Implementations:**

- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64_ule)`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64_ule)`
- **Shl**
  - `fn shl(self: Self, other: u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64_ule)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Shl**
  - `fn shl(self: Self, other: &u64) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64_ule)`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64)`
- **From**
  - `fn from(value: u64) -> Self`
- **Sub**
  - `fn sub(self: Self, other: u64) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<::core::cmp::Ordering>`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64_ule)`
- **BitOr**
  - `fn bitor(self: Self, other: u64) -> <Self as >::Output`
- **TryFrom**
  - `fn try_from(value: usize) -> Result<Self, <Self as >::Error>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Shl**
  - `fn shl(self: Self, other: u64_ule) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64)`
- **Shl**
  - `fn shl(self: Self, other: &u64_ule) -> <Self as >::Output`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: &u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64_ule)`
- **Mul**
  - `fn mul(self: Self, other: u64) -> <Self as >::Output`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64)`
- **Sub**
  - `fn sub(self: Self, other: u64_ule) -> <Self as >::Output`
- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64_ule)`
- **Div**
  - `fn div(self: Self, other: u64) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, other: &u64_ule) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: u64_ule) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: &u64) -> <Self as >::Output`
- **BitOr**
  - `fn bitor(self: Self, other: &u64_ule) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64)`
- **Div**
  - `fn div(self: Self, other: &u64) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, other: u64_ule) -> <Self as >::Output`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64)`
- **Mul**
  - `fn mul(self: Self, other: &u64_ule) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64_ule)`
- **Div**
  - `fn div(self: Self, other: u64_ule) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, other: &u64_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64)`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: u64_ule)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: u64_ule)`
- **From**
  - `fn from(value: &'a u64) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &u64) -> Option<::core::cmp::Ordering>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Rem**
  - `fn rem(self: Self, other: u64) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: u64_ule)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: u64_ule)`
- **Rem**
  - `fn rem(self: Self, other: &u64) -> <Self as >::Output`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: &u64_ule)`
- **Shr**
  - `fn shr(self: Self, other: u64) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Binary**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64)`
- **BitAnd**
  - `fn bitand(self: Self, other: u64) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, other: u64_ule) -> <Self as >::Output`
- **Shr**
  - `fn shr(self: Self, other: &u64) -> <Self as >::Output`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &u64_ule)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64)`
- **Rem**
  - `fn rem(self: Self, other: &u64_ule) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64) -> <Self as >::Output`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: &u64_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64) -> <Self as >::Output`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &u64)`
- **Shr**
  - `fn shr(self: Self, other: u64_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: u64)`
- **CheckBytes**
  - `fn check_bytes(_: *const Self, _: & mut C) -> Result<(), <C as >::Error>`
- **Shr**
  - `fn shr(self: Self, other: &u64_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: u64_ule) -> <Self as >::Output`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &u64_ule)`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64) -> <Self as >::Output`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: u64)`
- **Not**
  - `fn not(self: Self) -> <Self as >::Output`
- **BitAnd**
  - `fn bitand(self: Self, other: &u64_ule) -> <Self as >::Output`
- **ShrAssign**
  - `fn shr_assign(self: & mut Self, other: &u64)`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &u64_ule)`
- **PartialEq**
  - `fn eq(self: &Self, other: &u64) -> bool`
- **Add**
  - `fn add(self: Self, other: &u64) -> <Self as >::Output`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: &u64)`
- **BitXor**
  - `fn bitxor(self: Self, other: u64_ule) -> <Self as >::Output`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> ::core::cmp::Ordering`
- **BitXor**
  - `fn bitxor(self: Self, other: &u64_ule) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, other: u64_ule) -> <Self as >::Output`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Add**
  - `fn add(self: Self, other: &u64_ule) -> <Self as >::Output`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: &u64)`
- **ShlAssign**
  - `fn shl_assign(self: & mut Self, other: u64_ule)`
- **Default**
  - `fn default() -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &u64)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: u64_ule)`
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: u64_ule)`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: u64)`



