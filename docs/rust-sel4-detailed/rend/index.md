# Crate `rend`

rend provides cross-platform, endian-aware primitives for Rust.

rend does not provide cross-platform alternatives for types that are
inherently cross-platform, such as `bool` and `u8`. It also does not provide
cross-platform alternatives for types that have an architecture-dependent
size, such as `isize` and `usize`. rend does not support custom types.

rend is intended to be used to build portable types that can be shared
between different architectures.

## Features

- `bytecheck`: Enables support for validating types using `bytecheck`.

## Crates

- `zerocopy-0_8`

## Example:
```rust
use core::mem::transmute;
use rend::*;

let little_int = i32_le::from_native(0x12345678);
// Internal representation is little-endian
assert_eq!(
    [0x78, 0x56, 0x34, 0x12],
    unsafe { transmute::<_, [u8; 4]>(little_int) }
);

// Can also be made with `.into()`
let little_int: i32_le = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", little_int));
assert_eq!("0x12345678", format!("0x{:x}", little_int));

let big_int = i32_be::from_native(0x12345678);
// Internal representation is big-endian
assert_eq!(
    [0x12, 0x34, 0x56, 0x78],
    unsafe { transmute::<_, [u8; 4]>(big_int) }
);

// Can also be made with `.into()`
let big_int: i32_be = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", big_int));
assert_eq!("0x12345678", format!("0x{:x}", big_int));
```

## Contents

- [Modules](#modules)
  - [`common`](#common)
  - [`context`](#context)
  - [`traits`](#traits)
  - [`util`](#util)
  - [`unaligned`](#unaligned)
- [Structs](#structs)
  - [`i16_le`](#i16-le)
  - [`i16_be`](#i16-be)
  - [`i32_le`](#i32-le)
  - [`i32_be`](#i32-be)
  - [`i64_le`](#i64-le)
  - [`i64_be`](#i64-be)
  - [`i128_le`](#i128-le)
  - [`i128_be`](#i128-be)
  - [`u16_le`](#u16-le)
  - [`u16_be`](#u16-be)
  - [`u32_le`](#u32-le)
  - [`u32_be`](#u32-be)
  - [`u64_le`](#u64-le)
  - [`u64_be`](#u64-be)
  - [`u128_le`](#u128-le)
  - [`u128_be`](#u128-be)
  - [`f32_le`](#f32-le)
  - [`f32_be`](#f32-be)
  - [`f64_le`](#f64-le)
  - [`f64_be`](#f64-be)
  - [`char_le`](#char-le)
  - [`char_be`](#char-be)
  - [`NonZeroI16_le`](#nonzeroi16-le)
  - [`NonZeroI16_be`](#nonzeroi16-be)
  - [`NonZeroI32_le`](#nonzeroi32-le)
  - [`NonZeroI32_be`](#nonzeroi32-be)
  - [`NonZeroI64_le`](#nonzeroi64-le)
  - [`NonZeroI64_be`](#nonzeroi64-be)
  - [`NonZeroI128_le`](#nonzeroi128-le)
  - [`NonZeroI128_be`](#nonzeroi128-be)
  - [`NonZeroU16_le`](#nonzerou16-le)
  - [`NonZeroU16_be`](#nonzerou16-be)
  - [`NonZeroU32_le`](#nonzerou32-le)
  - [`NonZeroU32_be`](#nonzerou32-be)
  - [`NonZeroU64_le`](#nonzerou64-le)
  - [`NonZeroU64_be`](#nonzerou64-be)
  - [`NonZeroU128_le`](#nonzerou128-le)
  - [`NonZeroU128_be`](#nonzerou128-be)
  - [`AtomicI16_le`](#atomici16-le)
  - [`AtomicI16_be`](#atomici16-be)
  - [`AtomicU16_le`](#atomicu16-le)
  - [`AtomicU16_be`](#atomicu16-be)
  - [`AtomicI32_le`](#atomici32-le)
  - [`AtomicI32_be`](#atomici32-be)
  - [`AtomicU32_le`](#atomicu32-le)
  - [`AtomicU32_be`](#atomicu32-be)
  - [`AtomicI64_le`](#atomici64-le)
  - [`AtomicI64_be`](#atomici64-be)
  - [`AtomicU64_le`](#atomicu64-le)
  - [`AtomicU64_be`](#atomicu64-be)
- [Functions](#functions)
  - [`fetch_ordering`](#fetch-ordering)
- [Macros](#macros)
  - [`define_newtype!`](#define-newtype)
  - [`define_signed_integer!`](#define-signed-integer)
  - [`define_signed_integers!`](#define-signed-integers)
  - [`define_unsigned_integer!`](#define-unsigned-integer)
  - [`define_unsigned_integers!`](#define-unsigned-integers)
  - [`define_float!`](#define-float)
  - [`define_floats!`](#define-floats)
  - [`define_char!`](#define-char)
  - [`define_nonzero!`](#define-nonzero)
  - [`define_nonzeros!`](#define-nonzeros)
  - [`define_atomic!`](#define-atomic)
  - [`define_atomics!`](#define-atomics)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | mod |  |
| [`context`](#context) | mod |  |
| [`traits`](#traits) | mod |  |
| [`util`](#util) | mod |  |
| [`unaligned`](#unaligned) | mod | Cross-platform primitives with unaligned representations. |
| [`i16_le`](#i16-le) | struct | A little-endian `i16` with a guaranteed size and alignment of `2`. |
| [`i16_be`](#i16-be) | struct | A big-endian `i16` with a guaranteed size and alignment of `2`. |
| [`i32_le`](#i32-le) | struct | A little-endian `i32` with a guaranteed size and alignment of `4`. |
| [`i32_be`](#i32-be) | struct | A big-endian `i32` with a guaranteed size and alignment of `4`. |
| [`i64_le`](#i64-le) | struct | A little-endian `i64` with a guaranteed size and alignment of `8`. |
| [`i64_be`](#i64-be) | struct | A big-endian `i64` with a guaranteed size and alignment of `8`. |
| [`i128_le`](#i128-le) | struct | A little-endian `i128` with a guaranteed size and alignment of `16`. |
| [`i128_be`](#i128-be) | struct | A big-endian `i128` with a guaranteed size and alignment of `16`. |
| [`u16_le`](#u16-le) | struct | A little-endian `u16` with a guaranteed size and alignment of `2`. |
| [`u16_be`](#u16-be) | struct | A big-endian `u16` with a guaranteed size and alignment of `2`. |
| [`u32_le`](#u32-le) | struct | A little-endian `u32` with a guaranteed size and alignment of `4`. |
| [`u32_be`](#u32-be) | struct | A big-endian `u32` with a guaranteed size and alignment of `4`. |
| [`u64_le`](#u64-le) | struct | A little-endian `u64` with a guaranteed size and alignment of `8`. |
| [`u64_be`](#u64-be) | struct | A big-endian `u64` with a guaranteed size and alignment of `8`. |
| [`u128_le`](#u128-le) | struct | A little-endian `u128` with a guaranteed size and alignment of `16`. |
| [`u128_be`](#u128-be) | struct | A big-endian `u128` with a guaranteed size and alignment of `16`. |
| [`f32_le`](#f32-le) | struct | A little-endian `f32` with a guaranteed size and alignment of `4`. |
| [`f32_be`](#f32-be) | struct | A big-endian `f32` with a guaranteed size and alignment of `4`. |
| [`f64_le`](#f64-le) | struct | A little-endian `f64` with a guaranteed size and alignment of `8`. |
| [`f64_be`](#f64-be) | struct | A big-endian `f64` with a guaranteed size and alignment of `8`. |
| [`char_le`](#char-le) | struct | A little-endian `u32` with a guaranteed size and alignment of `4`. |
| [`char_be`](#char-be) | struct | A big-endian `u32` with a guaranteed size and alignment of `4`. |
| [`NonZeroI16_le`](#nonzeroi16-le) | struct | A little-endian `NonZeroI16` with a guaranteed size and alignment of `2`. |
| [`NonZeroI16_be`](#nonzeroi16-be) | struct | A big-endian `NonZeroI16` with a guaranteed size and alignment of `2`. |
| [`NonZeroI32_le`](#nonzeroi32-le) | struct | A little-endian `NonZeroI32` with a guaranteed size and alignment of `4`. |
| [`NonZeroI32_be`](#nonzeroi32-be) | struct | A big-endian `NonZeroI32` with a guaranteed size and alignment of `4`. |
| [`NonZeroI64_le`](#nonzeroi64-le) | struct | A little-endian `NonZeroI64` with a guaranteed size and alignment of `8`. |
| [`NonZeroI64_be`](#nonzeroi64-be) | struct | A big-endian `NonZeroI64` with a guaranteed size and alignment of `8`. |
| [`NonZeroI128_le`](#nonzeroi128-le) | struct | A little-endian `NonZeroI128` with a guaranteed size and alignment of `16`. |
| [`NonZeroI128_be`](#nonzeroi128-be) | struct | A big-endian `NonZeroI128` with a guaranteed size and alignment of `16`. |
| [`NonZeroU16_le`](#nonzerou16-le) | struct | A little-endian `NonZeroU16` with a guaranteed size and alignment of `2`. |
| [`NonZeroU16_be`](#nonzerou16-be) | struct | A big-endian `NonZeroU16` with a guaranteed size and alignment of `2`. |
| [`NonZeroU32_le`](#nonzerou32-le) | struct | A little-endian `NonZeroU32` with a guaranteed size and alignment of `4`. |
| [`NonZeroU32_be`](#nonzerou32-be) | struct | A big-endian `NonZeroU32` with a guaranteed size and alignment of `4`. |
| [`NonZeroU64_le`](#nonzerou64-le) | struct | A little-endian `NonZeroU64` with a guaranteed size and alignment of `8`. |
| [`NonZeroU64_be`](#nonzerou64-be) | struct | A big-endian `NonZeroU64` with a guaranteed size and alignment of `8`. |
| [`NonZeroU128_le`](#nonzerou128-le) | struct | A little-endian `NonZeroU128` with a guaranteed size and alignment of `16`. |
| [`NonZeroU128_be`](#nonzerou128-be) | struct | A big-endian `NonZeroU128` with a guaranteed size and alignment of `16`. |
| [`AtomicI16_le`](#atomici16-le) | struct | A little-endian `AtomicI16` with a guaranteed size and alignment of `2`. |
| [`AtomicI16_be`](#atomici16-be) | struct | A big-endian `AtomicI16` with a guaranteed size and alignment of `2`. |
| [`AtomicU16_le`](#atomicu16-le) | struct | A little-endian `AtomicU16` with a guaranteed size and alignment of `2`. |
| [`AtomicU16_be`](#atomicu16-be) | struct | A big-endian `AtomicU16` with a guaranteed size and alignment of `2`. |
| [`AtomicI32_le`](#atomici32-le) | struct | A little-endian `AtomicI32` with a guaranteed size and alignment of `4`. |
| [`AtomicI32_be`](#atomici32-be) | struct | A big-endian `AtomicI32` with a guaranteed size and alignment of `4`. |
| [`AtomicU32_le`](#atomicu32-le) | struct | A little-endian `AtomicU32` with a guaranteed size and alignment of `4`. |
| [`AtomicU32_be`](#atomicu32-be) | struct | A big-endian `AtomicU32` with a guaranteed size and alignment of `4`. |
| [`AtomicI64_le`](#atomici64-le) | struct | A little-endian `AtomicI64` with a guaranteed size and alignment of `8`. |
| [`AtomicI64_be`](#atomici64-be) | struct | A big-endian `AtomicI64` with a guaranteed size and alignment of `8`. |
| [`AtomicU64_le`](#atomicu64-le) | struct | A little-endian `AtomicU64` with a guaranteed size and alignment of `8`. |
| [`AtomicU64_be`](#atomicu64-be) | struct | A big-endian `AtomicU64` with a guaranteed size and alignment of `8`. |
| [`fetch_ordering`](#fetch-ordering) | fn |  |
| [`define_newtype!`](#define-newtype) | macro |  |
| [`define_signed_integer!`](#define-signed-integer) | macro |  |
| [`define_signed_integers!`](#define-signed-integers) | macro |  |
| [`define_unsigned_integer!`](#define-unsigned-integer) | macro |  |
| [`define_unsigned_integers!`](#define-unsigned-integers) | macro |  |
| [`define_float!`](#define-float) | macro |  |
| [`define_floats!`](#define-floats) | macro |  |
| [`define_char!`](#define-char) | macro |  |
| [`define_nonzero!`](#define-nonzero) | macro |  |
| [`define_nonzeros!`](#define-nonzeros) | macro |  |
| [`define_atomic!`](#define-atomic) | macro |  |
| [`define_atomics!`](#define-atomics) | macro |  |

## Modules

- [`common`](common/index.md)
- [`context`](context/index.md)
- [`traits`](traits/index.md)
- [`util`](util/index.md)
- [`unaligned`](unaligned/index.md) — Cross-platform primitives with unaligned representations.

## Structs

### `i16_le`

```rust
struct i16_le(i16);
```

A little-endian `i16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="i16-le-from-native"></span>`const fn from_native(value: i16) -> Self`

  Returns a `i16_le` containing `value`.

- <span id="i16-le-to-native"></span>`const fn to_native(self) -> i16`

  Returns a `i16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i16_le`

- <span id="i16-le-add-type-output"></span>`type Output = i16`

- <span id="i16-le-add"></span>`fn add(self, other: i16) -> <Self as >::Output`

##### `impl AddAssign for i16_le`

- <span id="i16-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: i16)`

##### `impl Binary for i16_le`

- <span id="i16-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i16_le`

- <span id="i16-le-bitand-type-output"></span>`type Output = i16`

- <span id="i16-le-bitand"></span>`fn bitand(self, other: i16) -> <Self as >::Output`

##### `impl BitAndAssign for i16_le`

- <span id="i16-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i16)`

##### `impl BitOr for i16_le`

- <span id="i16-le-bitor-type-output"></span>`type Output = i16`

- <span id="i16-le-bitor"></span>`fn bitor(self, other: i16) -> <Self as >::Output`

##### `impl BitOrAssign for i16_le`

- <span id="i16-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i16)`

##### `impl BitXor for i16_le`

- <span id="i16-le-bitxor-type-output"></span>`type Output = i16`

- <span id="i16-le-bitxor"></span>`fn bitxor(self, other: i16) -> <Self as >::Output`

##### `impl BitXorAssign for i16_le`

- <span id="i16-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i16)`

##### `impl<C> CheckBytes for i16_le`

- <span id="i16-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i16_le`

- <span id="i16-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i16_le`

##### `impl Debug for i16_le`

- <span id="i16-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i16_le`

- <span id="i16-le-default"></span>`fn default() -> Self`

##### `impl Display for i16_le`

- <span id="i16-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i16_le`

- <span id="i16-le-div-type-output"></span>`type Output = i16`

- <span id="i16-le-div"></span>`fn div(self, other: i16) -> <Self as >::Output`

##### `impl DivAssign for i16_le`

- <span id="i16-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: i16)`

##### `impl Eq for i16_le`

##### `impl Hash for i16_le`

- <span id="i16-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i16_le`

- <span id="i16-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i16_le`

- <span id="i16-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i16_le`

- <span id="i16-le-mul-type-output"></span>`type Output = i16`

- <span id="i16-le-mul"></span>`fn mul(self, other: i16) -> <Self as >::Output`

##### `impl MulAssign for i16_le`

- <span id="i16-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i16)`

##### `impl Neg for i16_le`

- <span id="i16-le-neg-type-output"></span>`type Output = <i16 as Neg>::Output`

- <span id="i16-le-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i16_le`

- <span id="i16-le-not-type-output"></span>`type Output = <i16 as Not>::Output`

- <span id="i16-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i16_le`

- <span id="i16-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i16_le`

- <span id="i16-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i16_le`

- <span id="i16-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i16_le`

- <span id="i16-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i16_le`

- <span id="i16-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i16_le`

- <span id="i16-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i16_le`

- <span id="i16-le-rem-type-output"></span>`type Output = i16`

- <span id="i16-le-rem"></span>`fn rem(self, other: i16) -> <Self as >::Output`

##### `impl RemAssign for i16_le`

- <span id="i16-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i16)`

##### `impl Shl for i16_le`

- <span id="i16-le-shl-type-output"></span>`type Output = i16`

- <span id="i16-le-shl"></span>`fn shl(self, other: i16) -> <Self as >::Output`

##### `impl ShlAssign for i16_le`

- <span id="i16-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i16)`

##### `impl Shr for i16_le`

- <span id="i16-le-shr-type-output"></span>`type Output = i16`

- <span id="i16-le-shr"></span>`fn shr(self, other: i16) -> <Self as >::Output`

##### `impl ShrAssign for i16_le`

- <span id="i16-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i16)`

##### `impl Sub for i16_le`

- <span id="i16-le-sub-type-output"></span>`type Output = i16`

- <span id="i16-le-sub"></span>`fn sub(self, other: i16) -> <Self as >::Output`

##### `impl SubAssign for i16_le`

- <span id="i16-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i16)`

##### `impl Sum for i16_le`

- <span id="i16-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i16_le`

- <span id="i16-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i16_le`

- <span id="i16-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i16_le`

- <span id="i16-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i16_be`

```rust
struct i16_be(i16);
```

A big-endian `i16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="i16-be-from-native"></span>`const fn from_native(value: i16) -> Self`

  Returns a `i16_be` containing `value`.

- <span id="i16-be-to-native"></span>`const fn to_native(self) -> i16`

  Returns a `i16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i16_be`

- <span id="i16-be-add-type-output"></span>`type Output = i16`

- <span id="i16-be-add"></span>`fn add(self, other: i16) -> <Self as >::Output`

##### `impl AddAssign for i16_be`

- <span id="i16-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: i16)`

##### `impl Binary for i16_be`

- <span id="i16-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i16_be`

- <span id="i16-be-bitand-type-output"></span>`type Output = i16`

- <span id="i16-be-bitand"></span>`fn bitand(self, other: i16) -> <Self as >::Output`

##### `impl BitAndAssign for i16_be`

- <span id="i16-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i16)`

##### `impl BitOr for i16_be`

- <span id="i16-be-bitor-type-output"></span>`type Output = i16`

- <span id="i16-be-bitor"></span>`fn bitor(self, other: i16) -> <Self as >::Output`

##### `impl BitOrAssign for i16_be`

- <span id="i16-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i16)`

##### `impl BitXor for i16_be`

- <span id="i16-be-bitxor-type-output"></span>`type Output = i16`

- <span id="i16-be-bitxor"></span>`fn bitxor(self, other: i16) -> <Self as >::Output`

##### `impl BitXorAssign for i16_be`

- <span id="i16-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i16)`

##### `impl<C> CheckBytes for i16_be`

- <span id="i16-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i16_be`

- <span id="i16-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i16_be`

##### `impl Debug for i16_be`

- <span id="i16-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i16_be`

- <span id="i16-be-default"></span>`fn default() -> Self`

##### `impl Display for i16_be`

- <span id="i16-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i16_be`

- <span id="i16-be-div-type-output"></span>`type Output = i16`

- <span id="i16-be-div"></span>`fn div(self, other: i16) -> <Self as >::Output`

##### `impl DivAssign for i16_be`

- <span id="i16-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: i16)`

##### `impl Eq for i16_be`

##### `impl Hash for i16_be`

- <span id="i16-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i16_be`

- <span id="i16-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i16_be`

- <span id="i16-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i16_be`

- <span id="i16-be-mul-type-output"></span>`type Output = i16`

- <span id="i16-be-mul"></span>`fn mul(self, other: i16) -> <Self as >::Output`

##### `impl MulAssign for i16_be`

- <span id="i16-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i16)`

##### `impl Neg for i16_be`

- <span id="i16-be-neg-type-output"></span>`type Output = <i16 as Neg>::Output`

- <span id="i16-be-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i16_be`

- <span id="i16-be-not-type-output"></span>`type Output = <i16 as Not>::Output`

- <span id="i16-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i16_be`

- <span id="i16-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i16_be`

- <span id="i16-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i16_be`

- <span id="i16-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i16_be`

- <span id="i16-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i16_be`

- <span id="i16-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i16_be`

- <span id="i16-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i16_be`

- <span id="i16-be-rem-type-output"></span>`type Output = i16`

- <span id="i16-be-rem"></span>`fn rem(self, other: i16) -> <Self as >::Output`

##### `impl RemAssign for i16_be`

- <span id="i16-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i16)`

##### `impl Shl for i16_be`

- <span id="i16-be-shl-type-output"></span>`type Output = i16`

- <span id="i16-be-shl"></span>`fn shl(self, other: i16) -> <Self as >::Output`

##### `impl ShlAssign for i16_be`

- <span id="i16-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i16)`

##### `impl Shr for i16_be`

- <span id="i16-be-shr-type-output"></span>`type Output = i16`

- <span id="i16-be-shr"></span>`fn shr(self, other: i16) -> <Self as >::Output`

##### `impl ShrAssign for i16_be`

- <span id="i16-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i16)`

##### `impl Sub for i16_be`

- <span id="i16-be-sub-type-output"></span>`type Output = i16`

- <span id="i16-be-sub"></span>`fn sub(self, other: i16) -> <Self as >::Output`

##### `impl SubAssign for i16_be`

- <span id="i16-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i16)`

##### `impl Sum for i16_be`

- <span id="i16-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i16_be`

- <span id="i16-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i16_be`

- <span id="i16-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i16_be`

- <span id="i16-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i32_le`

```rust
struct i32_le(i32);
```

A little-endian `i32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="i32-le-from-native"></span>`const fn from_native(value: i32) -> Self`

  Returns a `i32_le` containing `value`.

- <span id="i32-le-to-native"></span>`const fn to_native(self) -> i32`

  Returns a `i32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i32_le`

- <span id="i32-le-add-type-output"></span>`type Output = i32`

- <span id="i32-le-add"></span>`fn add(self, other: i32) -> <Self as >::Output`

##### `impl AddAssign for i32_le`

- <span id="i32-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: i32)`

##### `impl Binary for i32_le`

- <span id="i32-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i32_le`

- <span id="i32-le-bitand-type-output"></span>`type Output = i32`

- <span id="i32-le-bitand"></span>`fn bitand(self, other: i32) -> <Self as >::Output`

##### `impl BitAndAssign for i32_le`

- <span id="i32-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i32)`

##### `impl BitOr for i32_le`

- <span id="i32-le-bitor-type-output"></span>`type Output = i32`

- <span id="i32-le-bitor"></span>`fn bitor(self, other: i32) -> <Self as >::Output`

##### `impl BitOrAssign for i32_le`

- <span id="i32-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i32)`

##### `impl BitXor for i32_le`

- <span id="i32-le-bitxor-type-output"></span>`type Output = i32`

- <span id="i32-le-bitxor"></span>`fn bitxor(self, other: i32) -> <Self as >::Output`

##### `impl BitXorAssign for i32_le`

- <span id="i32-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i32)`

##### `impl<C> CheckBytes for i32_le`

- <span id="i32-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i32_le`

- <span id="i32-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i32_le`

##### `impl Debug for i32_le`

- <span id="i32-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i32_le`

- <span id="i32-le-default"></span>`fn default() -> Self`

##### `impl Display for i32_le`

- <span id="i32-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i32_le`

- <span id="i32-le-div-type-output"></span>`type Output = i32`

- <span id="i32-le-div"></span>`fn div(self, other: i32) -> <Self as >::Output`

##### `impl DivAssign for i32_le`

- <span id="i32-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: i32)`

##### `impl Eq for i32_le`

##### `impl Hash for i32_le`

- <span id="i32-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i32_le`

- <span id="i32-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i32_le`

- <span id="i32-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i32_le`

- <span id="i32-le-mul-type-output"></span>`type Output = i32`

- <span id="i32-le-mul"></span>`fn mul(self, other: i32) -> <Self as >::Output`

##### `impl MulAssign for i32_le`

- <span id="i32-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i32)`

##### `impl Neg for i32_le`

- <span id="i32-le-neg-type-output"></span>`type Output = <i32 as Neg>::Output`

- <span id="i32-le-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i32_le`

- <span id="i32-le-not-type-output"></span>`type Output = <i32 as Not>::Output`

- <span id="i32-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i32_le`

- <span id="i32-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i32_le`

- <span id="i32-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i32_le`

- <span id="i32-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i32_le`

- <span id="i32-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i32_le`

- <span id="i32-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i32_le`

- <span id="i32-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i32_le`

- <span id="i32-le-rem-type-output"></span>`type Output = i32`

- <span id="i32-le-rem"></span>`fn rem(self, other: i32) -> <Self as >::Output`

##### `impl RemAssign for i32_le`

- <span id="i32-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i32)`

##### `impl Shl for i32_le`

- <span id="i32-le-shl-type-output"></span>`type Output = i32`

- <span id="i32-le-shl"></span>`fn shl(self, other: i32) -> <Self as >::Output`

##### `impl ShlAssign for i32_le`

- <span id="i32-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i32)`

##### `impl Shr for i32_le`

- <span id="i32-le-shr-type-output"></span>`type Output = i32`

- <span id="i32-le-shr"></span>`fn shr(self, other: i32) -> <Self as >::Output`

##### `impl ShrAssign for i32_le`

- <span id="i32-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i32)`

##### `impl Sub for i32_le`

- <span id="i32-le-sub-type-output"></span>`type Output = i32`

- <span id="i32-le-sub"></span>`fn sub(self, other: i32) -> <Self as >::Output`

##### `impl SubAssign for i32_le`

- <span id="i32-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i32)`

##### `impl Sum for i32_le`

- <span id="i32-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i32_le`

- <span id="i32-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i32_le`

- <span id="i32-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i32_le`

- <span id="i32-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i32_be`

```rust
struct i32_be(i32);
```

A big-endian `i32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="i32-be-from-native"></span>`const fn from_native(value: i32) -> Self`

  Returns a `i32_be` containing `value`.

- <span id="i32-be-to-native"></span>`const fn to_native(self) -> i32`

  Returns a `i32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i32_be`

- <span id="i32-be-add-type-output"></span>`type Output = i32`

- <span id="i32-be-add"></span>`fn add(self, other: i32) -> <Self as >::Output`

##### `impl AddAssign for i32_be`

- <span id="i32-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: i32)`

##### `impl Binary for i32_be`

- <span id="i32-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i32_be`

- <span id="i32-be-bitand-type-output"></span>`type Output = i32`

- <span id="i32-be-bitand"></span>`fn bitand(self, other: i32) -> <Self as >::Output`

##### `impl BitAndAssign for i32_be`

- <span id="i32-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i32)`

##### `impl BitOr for i32_be`

- <span id="i32-be-bitor-type-output"></span>`type Output = i32`

- <span id="i32-be-bitor"></span>`fn bitor(self, other: i32) -> <Self as >::Output`

##### `impl BitOrAssign for i32_be`

- <span id="i32-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i32)`

##### `impl BitXor for i32_be`

- <span id="i32-be-bitxor-type-output"></span>`type Output = i32`

- <span id="i32-be-bitxor"></span>`fn bitxor(self, other: i32) -> <Self as >::Output`

##### `impl BitXorAssign for i32_be`

- <span id="i32-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i32)`

##### `impl<C> CheckBytes for i32_be`

- <span id="i32-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i32_be`

- <span id="i32-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i32_be`

##### `impl Debug for i32_be`

- <span id="i32-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i32_be`

- <span id="i32-be-default"></span>`fn default() -> Self`

##### `impl Display for i32_be`

- <span id="i32-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i32_be`

- <span id="i32-be-div-type-output"></span>`type Output = i32`

- <span id="i32-be-div"></span>`fn div(self, other: i32) -> <Self as >::Output`

##### `impl DivAssign for i32_be`

- <span id="i32-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: i32)`

##### `impl Eq for i32_be`

##### `impl Hash for i32_be`

- <span id="i32-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i32_be`

- <span id="i32-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i32_be`

- <span id="i32-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i32_be`

- <span id="i32-be-mul-type-output"></span>`type Output = i32`

- <span id="i32-be-mul"></span>`fn mul(self, other: i32) -> <Self as >::Output`

##### `impl MulAssign for i32_be`

- <span id="i32-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i32)`

##### `impl Neg for i32_be`

- <span id="i32-be-neg-type-output"></span>`type Output = <i32 as Neg>::Output`

- <span id="i32-be-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i32_be`

- <span id="i32-be-not-type-output"></span>`type Output = <i32 as Not>::Output`

- <span id="i32-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i32_be`

- <span id="i32-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i32_be`

- <span id="i32-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i32_be`

- <span id="i32-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i32_be`

- <span id="i32-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i32_be`

- <span id="i32-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i32_be`

- <span id="i32-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i32_be`

- <span id="i32-be-rem-type-output"></span>`type Output = i32`

- <span id="i32-be-rem"></span>`fn rem(self, other: i32) -> <Self as >::Output`

##### `impl RemAssign for i32_be`

- <span id="i32-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i32)`

##### `impl Shl for i32_be`

- <span id="i32-be-shl-type-output"></span>`type Output = i32`

- <span id="i32-be-shl"></span>`fn shl(self, other: i32) -> <Self as >::Output`

##### `impl ShlAssign for i32_be`

- <span id="i32-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i32)`

##### `impl Shr for i32_be`

- <span id="i32-be-shr-type-output"></span>`type Output = i32`

- <span id="i32-be-shr"></span>`fn shr(self, other: i32) -> <Self as >::Output`

##### `impl ShrAssign for i32_be`

- <span id="i32-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i32)`

##### `impl Sub for i32_be`

- <span id="i32-be-sub-type-output"></span>`type Output = i32`

- <span id="i32-be-sub"></span>`fn sub(self, other: i32) -> <Self as >::Output`

##### `impl SubAssign for i32_be`

- <span id="i32-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i32)`

##### `impl Sum for i32_be`

- <span id="i32-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i32_be`

- <span id="i32-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i32_be`

- <span id="i32-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i32_be`

- <span id="i32-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i64_le`

```rust
struct i64_le(i64);
```

A little-endian `i64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="i64-le-from-native"></span>`const fn from_native(value: i64) -> Self`

  Returns a `i64_le` containing `value`.

- <span id="i64-le-to-native"></span>`const fn to_native(self) -> i64`

  Returns a `i64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i64_le`

- <span id="i64-le-add-type-output"></span>`type Output = i64`

- <span id="i64-le-add"></span>`fn add(self, other: i64) -> <Self as >::Output`

##### `impl AddAssign for i64_le`

- <span id="i64-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: i64)`

##### `impl Binary for i64_le`

- <span id="i64-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i64_le`

- <span id="i64-le-bitand-type-output"></span>`type Output = i64`

- <span id="i64-le-bitand"></span>`fn bitand(self, other: i64) -> <Self as >::Output`

##### `impl BitAndAssign for i64_le`

- <span id="i64-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i64)`

##### `impl BitOr for i64_le`

- <span id="i64-le-bitor-type-output"></span>`type Output = i64`

- <span id="i64-le-bitor"></span>`fn bitor(self, other: i64) -> <Self as >::Output`

##### `impl BitOrAssign for i64_le`

- <span id="i64-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i64)`

##### `impl BitXor for i64_le`

- <span id="i64-le-bitxor-type-output"></span>`type Output = i64`

- <span id="i64-le-bitxor"></span>`fn bitxor(self, other: i64) -> <Self as >::Output`

##### `impl BitXorAssign for i64_le`

- <span id="i64-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i64)`

##### `impl<C> CheckBytes for i64_le`

- <span id="i64-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i64_le`

- <span id="i64-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i64_le`

##### `impl Debug for i64_le`

- <span id="i64-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i64_le`

- <span id="i64-le-default"></span>`fn default() -> Self`

##### `impl Display for i64_le`

- <span id="i64-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i64_le`

- <span id="i64-le-div-type-output"></span>`type Output = i64`

- <span id="i64-le-div"></span>`fn div(self, other: i64) -> <Self as >::Output`

##### `impl DivAssign for i64_le`

- <span id="i64-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: i64)`

##### `impl Eq for i64_le`

##### `impl Hash for i64_le`

- <span id="i64-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i64_le`

- <span id="i64-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i64_le`

- <span id="i64-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i64_le`

- <span id="i64-le-mul-type-output"></span>`type Output = i64`

- <span id="i64-le-mul"></span>`fn mul(self, other: i64) -> <Self as >::Output`

##### `impl MulAssign for i64_le`

- <span id="i64-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i64)`

##### `impl Neg for i64_le`

- <span id="i64-le-neg-type-output"></span>`type Output = <i64 as Neg>::Output`

- <span id="i64-le-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i64_le`

- <span id="i64-le-not-type-output"></span>`type Output = <i64 as Not>::Output`

- <span id="i64-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i64_le`

- <span id="i64-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i64_le`

- <span id="i64-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i64_le`

- <span id="i64-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i64_le`

- <span id="i64-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i64_le`

- <span id="i64-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i64_le`

- <span id="i64-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i64_le`

- <span id="i64-le-rem-type-output"></span>`type Output = i64`

- <span id="i64-le-rem"></span>`fn rem(self, other: i64) -> <Self as >::Output`

##### `impl RemAssign for i64_le`

- <span id="i64-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i64)`

##### `impl Shl for i64_le`

- <span id="i64-le-shl-type-output"></span>`type Output = i64`

- <span id="i64-le-shl"></span>`fn shl(self, other: i64) -> <Self as >::Output`

##### `impl ShlAssign for i64_le`

- <span id="i64-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i64)`

##### `impl Shr for i64_le`

- <span id="i64-le-shr-type-output"></span>`type Output = i64`

- <span id="i64-le-shr"></span>`fn shr(self, other: i64) -> <Self as >::Output`

##### `impl ShrAssign for i64_le`

- <span id="i64-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i64)`

##### `impl Sub for i64_le`

- <span id="i64-le-sub-type-output"></span>`type Output = i64`

- <span id="i64-le-sub"></span>`fn sub(self, other: i64) -> <Self as >::Output`

##### `impl SubAssign for i64_le`

- <span id="i64-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i64)`

##### `impl Sum for i64_le`

- <span id="i64-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i64_le`

- <span id="i64-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i64_le`

- <span id="i64-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i64_le`

- <span id="i64-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i64_be`

```rust
struct i64_be(i64);
```

A big-endian `i64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="i64-be-from-native"></span>`const fn from_native(value: i64) -> Self`

  Returns a `i64_be` containing `value`.

- <span id="i64-be-to-native"></span>`const fn to_native(self) -> i64`

  Returns a `i64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i64_be`

- <span id="i64-be-add-type-output"></span>`type Output = i64`

- <span id="i64-be-add"></span>`fn add(self, other: i64) -> <Self as >::Output`

##### `impl AddAssign for i64_be`

- <span id="i64-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: i64)`

##### `impl Binary for i64_be`

- <span id="i64-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i64_be`

- <span id="i64-be-bitand-type-output"></span>`type Output = i64`

- <span id="i64-be-bitand"></span>`fn bitand(self, other: i64) -> <Self as >::Output`

##### `impl BitAndAssign for i64_be`

- <span id="i64-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i64)`

##### `impl BitOr for i64_be`

- <span id="i64-be-bitor-type-output"></span>`type Output = i64`

- <span id="i64-be-bitor"></span>`fn bitor(self, other: i64) -> <Self as >::Output`

##### `impl BitOrAssign for i64_be`

- <span id="i64-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i64)`

##### `impl BitXor for i64_be`

- <span id="i64-be-bitxor-type-output"></span>`type Output = i64`

- <span id="i64-be-bitxor"></span>`fn bitxor(self, other: i64) -> <Self as >::Output`

##### `impl BitXorAssign for i64_be`

- <span id="i64-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i64)`

##### `impl<C> CheckBytes for i64_be`

- <span id="i64-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i64_be`

- <span id="i64-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i64_be`

##### `impl Debug for i64_be`

- <span id="i64-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i64_be`

- <span id="i64-be-default"></span>`fn default() -> Self`

##### `impl Display for i64_be`

- <span id="i64-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i64_be`

- <span id="i64-be-div-type-output"></span>`type Output = i64`

- <span id="i64-be-div"></span>`fn div(self, other: i64) -> <Self as >::Output`

##### `impl DivAssign for i64_be`

- <span id="i64-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: i64)`

##### `impl Eq for i64_be`

##### `impl Hash for i64_be`

- <span id="i64-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i64_be`

- <span id="i64-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i64_be`

- <span id="i64-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i64_be`

- <span id="i64-be-mul-type-output"></span>`type Output = i64`

- <span id="i64-be-mul"></span>`fn mul(self, other: i64) -> <Self as >::Output`

##### `impl MulAssign for i64_be`

- <span id="i64-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i64)`

##### `impl Neg for i64_be`

- <span id="i64-be-neg-type-output"></span>`type Output = <i64 as Neg>::Output`

- <span id="i64-be-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i64_be`

- <span id="i64-be-not-type-output"></span>`type Output = <i64 as Not>::Output`

- <span id="i64-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i64_be`

- <span id="i64-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i64_be`

- <span id="i64-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i64_be`

- <span id="i64-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i64_be`

- <span id="i64-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i64_be`

- <span id="i64-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i64_be`

- <span id="i64-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i64_be`

- <span id="i64-be-rem-type-output"></span>`type Output = i64`

- <span id="i64-be-rem"></span>`fn rem(self, other: i64) -> <Self as >::Output`

##### `impl RemAssign for i64_be`

- <span id="i64-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i64)`

##### `impl Shl for i64_be`

- <span id="i64-be-shl-type-output"></span>`type Output = i64`

- <span id="i64-be-shl"></span>`fn shl(self, other: i64) -> <Self as >::Output`

##### `impl ShlAssign for i64_be`

- <span id="i64-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i64)`

##### `impl Shr for i64_be`

- <span id="i64-be-shr-type-output"></span>`type Output = i64`

- <span id="i64-be-shr"></span>`fn shr(self, other: i64) -> <Self as >::Output`

##### `impl ShrAssign for i64_be`

- <span id="i64-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i64)`

##### `impl Sub for i64_be`

- <span id="i64-be-sub-type-output"></span>`type Output = i64`

- <span id="i64-be-sub"></span>`fn sub(self, other: i64) -> <Self as >::Output`

##### `impl SubAssign for i64_be`

- <span id="i64-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i64)`

##### `impl Sum for i64_be`

- <span id="i64-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i64_be`

- <span id="i64-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i64_be`

- <span id="i64-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i64_be`

- <span id="i64-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i128_le`

```rust
struct i128_le(i128);
```

A little-endian `i128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="i128-le-from-native"></span>`const fn from_native(value: i128) -> Self`

  Returns a `i128_le` containing `value`.

- <span id="i128-le-to-native"></span>`const fn to_native(self) -> i128`

  Returns a `i128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i128_le`

- <span id="i128-le-add-type-output"></span>`type Output = i128`

- <span id="i128-le-add"></span>`fn add(self, other: i128) -> <Self as >::Output`

##### `impl AddAssign for i128_le`

- <span id="i128-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: i128)`

##### `impl Binary for i128_le`

- <span id="i128-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i128_le`

- <span id="i128-le-bitand-type-output"></span>`type Output = i128`

- <span id="i128-le-bitand"></span>`fn bitand(self, other: i128) -> <Self as >::Output`

##### `impl BitAndAssign for i128_le`

- <span id="i128-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i128)`

##### `impl BitOr for i128_le`

- <span id="i128-le-bitor-type-output"></span>`type Output = i128`

- <span id="i128-le-bitor"></span>`fn bitor(self, other: i128) -> <Self as >::Output`

##### `impl BitOrAssign for i128_le`

- <span id="i128-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i128)`

##### `impl BitXor for i128_le`

- <span id="i128-le-bitxor-type-output"></span>`type Output = i128`

- <span id="i128-le-bitxor"></span>`fn bitxor(self, other: i128) -> <Self as >::Output`

##### `impl BitXorAssign for i128_le`

- <span id="i128-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i128)`

##### `impl<C> CheckBytes for i128_le`

- <span id="i128-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i128_le`

- <span id="i128-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i128_le`

##### `impl Debug for i128_le`

- <span id="i128-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i128_le`

- <span id="i128-le-default"></span>`fn default() -> Self`

##### `impl Display for i128_le`

- <span id="i128-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i128_le`

- <span id="i128-le-div-type-output"></span>`type Output = i128`

- <span id="i128-le-div"></span>`fn div(self, other: i128) -> <Self as >::Output`

##### `impl DivAssign for i128_le`

- <span id="i128-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: i128)`

##### `impl Eq for i128_le`

##### `impl Hash for i128_le`

- <span id="i128-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i128_le`

- <span id="i128-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i128_le`

- <span id="i128-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i128_le`

- <span id="i128-le-mul-type-output"></span>`type Output = i128`

- <span id="i128-le-mul"></span>`fn mul(self, other: i128) -> <Self as >::Output`

##### `impl MulAssign for i128_le`

- <span id="i128-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i128)`

##### `impl Neg for i128_le`

- <span id="i128-le-neg-type-output"></span>`type Output = <i128 as Neg>::Output`

- <span id="i128-le-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i128_le`

- <span id="i128-le-not-type-output"></span>`type Output = <i128 as Not>::Output`

- <span id="i128-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i128_le`

- <span id="i128-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i128_le`

- <span id="i128-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i128_le`

- <span id="i128-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i128_le`

- <span id="i128-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i128_le`

- <span id="i128-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i128_le`

- <span id="i128-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i128_le`

- <span id="i128-le-rem-type-output"></span>`type Output = i128`

- <span id="i128-le-rem"></span>`fn rem(self, other: i128) -> <Self as >::Output`

##### `impl RemAssign for i128_le`

- <span id="i128-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i128)`

##### `impl Shl for i128_le`

- <span id="i128-le-shl-type-output"></span>`type Output = i128`

- <span id="i128-le-shl"></span>`fn shl(self, other: i128) -> <Self as >::Output`

##### `impl ShlAssign for i128_le`

- <span id="i128-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i128)`

##### `impl Shr for i128_le`

- <span id="i128-le-shr-type-output"></span>`type Output = i128`

- <span id="i128-le-shr"></span>`fn shr(self, other: i128) -> <Self as >::Output`

##### `impl ShrAssign for i128_le`

- <span id="i128-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i128)`

##### `impl Sub for i128_le`

- <span id="i128-le-sub-type-output"></span>`type Output = i128`

- <span id="i128-le-sub"></span>`fn sub(self, other: i128) -> <Self as >::Output`

##### `impl SubAssign for i128_le`

- <span id="i128-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i128)`

##### `impl Sum for i128_le`

- <span id="i128-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i128_le`

- <span id="i128-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i128_le`

- <span id="i128-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i128_le`

- <span id="i128-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i128_be`

```rust
struct i128_be(i128);
```

A big-endian `i128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="i128-be-from-native"></span>`const fn from_native(value: i128) -> Self`

  Returns a `i128_be` containing `value`.

- <span id="i128-be-to-native"></span>`const fn to_native(self) -> i128`

  Returns a `i128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i128_be`

- <span id="i128-be-add-type-output"></span>`type Output = i128`

- <span id="i128-be-add"></span>`fn add(self, other: i128) -> <Self as >::Output`

##### `impl AddAssign for i128_be`

- <span id="i128-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: i128)`

##### `impl Binary for i128_be`

- <span id="i128-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i128_be`

- <span id="i128-be-bitand-type-output"></span>`type Output = i128`

- <span id="i128-be-bitand"></span>`fn bitand(self, other: i128) -> <Self as >::Output`

##### `impl BitAndAssign for i128_be`

- <span id="i128-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i128)`

##### `impl BitOr for i128_be`

- <span id="i128-be-bitor-type-output"></span>`type Output = i128`

- <span id="i128-be-bitor"></span>`fn bitor(self, other: i128) -> <Self as >::Output`

##### `impl BitOrAssign for i128_be`

- <span id="i128-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i128)`

##### `impl BitXor for i128_be`

- <span id="i128-be-bitxor-type-output"></span>`type Output = i128`

- <span id="i128-be-bitxor"></span>`fn bitxor(self, other: i128) -> <Self as >::Output`

##### `impl BitXorAssign for i128_be`

- <span id="i128-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i128)`

##### `impl<C> CheckBytes for i128_be`

- <span id="i128-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i128_be`

- <span id="i128-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i128_be`

##### `impl Debug for i128_be`

- <span id="i128-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i128_be`

- <span id="i128-be-default"></span>`fn default() -> Self`

##### `impl Display for i128_be`

- <span id="i128-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i128_be`

- <span id="i128-be-div-type-output"></span>`type Output = i128`

- <span id="i128-be-div"></span>`fn div(self, other: i128) -> <Self as >::Output`

##### `impl DivAssign for i128_be`

- <span id="i128-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: i128)`

##### `impl Eq for i128_be`

##### `impl Hash for i128_be`

- <span id="i128-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i128_be`

- <span id="i128-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i128_be`

- <span id="i128-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i128_be`

- <span id="i128-be-mul-type-output"></span>`type Output = i128`

- <span id="i128-be-mul"></span>`fn mul(self, other: i128) -> <Self as >::Output`

##### `impl MulAssign for i128_be`

- <span id="i128-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i128)`

##### `impl Neg for i128_be`

- <span id="i128-be-neg-type-output"></span>`type Output = <i128 as Neg>::Output`

- <span id="i128-be-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i128_be`

- <span id="i128-be-not-type-output"></span>`type Output = <i128 as Not>::Output`

- <span id="i128-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i128_be`

- <span id="i128-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i128_be`

- <span id="i128-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i128_be`

- <span id="i128-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i128_be`

- <span id="i128-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i128_be`

- <span id="i128-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i128_be`

- <span id="i128-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i128_be`

- <span id="i128-be-rem-type-output"></span>`type Output = i128`

- <span id="i128-be-rem"></span>`fn rem(self, other: i128) -> <Self as >::Output`

##### `impl RemAssign for i128_be`

- <span id="i128-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i128)`

##### `impl Shl for i128_be`

- <span id="i128-be-shl-type-output"></span>`type Output = i128`

- <span id="i128-be-shl"></span>`fn shl(self, other: i128) -> <Self as >::Output`

##### `impl ShlAssign for i128_be`

- <span id="i128-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i128)`

##### `impl Shr for i128_be`

- <span id="i128-be-shr-type-output"></span>`type Output = i128`

- <span id="i128-be-shr"></span>`fn shr(self, other: i128) -> <Self as >::Output`

##### `impl ShrAssign for i128_be`

- <span id="i128-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i128)`

##### `impl Sub for i128_be`

- <span id="i128-be-sub-type-output"></span>`type Output = i128`

- <span id="i128-be-sub"></span>`fn sub(self, other: i128) -> <Self as >::Output`

##### `impl SubAssign for i128_be`

- <span id="i128-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i128)`

##### `impl Sum for i128_be`

- <span id="i128-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i128_be`

- <span id="i128-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i128_be`

- <span id="i128-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i128_be`

- <span id="i128-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u16_le`

```rust
struct u16_le(u16);
```

A little-endian `u16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="u16-le-from-native"></span>`const fn from_native(value: u16) -> Self`

  Returns a `u16_le` containing `value`.

- <span id="u16-le-to-native"></span>`const fn to_native(self) -> u16`

  Returns a `u16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u16_le`

- <span id="u16-le-add-type-output"></span>`type Output = u16`

- <span id="u16-le-add"></span>`fn add(self, other: u16) -> <Self as >::Output`

##### `impl AddAssign for u16_le`

- <span id="u16-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: u16)`

##### `impl Binary for u16_le`

- <span id="u16-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u16_le`

- <span id="u16-le-bitand-type-output"></span>`type Output = u16`

- <span id="u16-le-bitand"></span>`fn bitand(self, other: u16) -> <Self as >::Output`

##### `impl BitAndAssign for u16_le`

- <span id="u16-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u16)`

##### `impl BitOr for u16_le`

- <span id="u16-le-bitor-type-output"></span>`type Output = u16`

- <span id="u16-le-bitor"></span>`fn bitor(self, other: u16) -> <Self as >::Output`

##### `impl BitOrAssign for u16_le`

- <span id="u16-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u16)`

##### `impl BitXor for u16_le`

- <span id="u16-le-bitxor-type-output"></span>`type Output = u16`

- <span id="u16-le-bitxor"></span>`fn bitxor(self, other: u16) -> <Self as >::Output`

##### `impl BitXorAssign for u16_le`

- <span id="u16-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u16)`

##### `impl<C> CheckBytes for u16_le`

- <span id="u16-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u16_le`

- <span id="u16-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u16_le`

##### `impl Debug for u16_le`

- <span id="u16-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u16_le`

- <span id="u16-le-default"></span>`fn default() -> Self`

##### `impl Display for u16_le`

- <span id="u16-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u16_le`

- <span id="u16-le-div-type-output"></span>`type Output = u16`

- <span id="u16-le-div"></span>`fn div(self, other: u16) -> <Self as >::Output`

##### `impl DivAssign for u16_le`

- <span id="u16-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: u16)`

##### `impl Eq for u16_le`

##### `impl Hash for u16_le`

- <span id="u16-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u16_le`

- <span id="u16-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u16_le`

- <span id="u16-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u16_le`

- <span id="u16-le-mul-type-output"></span>`type Output = u16`

- <span id="u16-le-mul"></span>`fn mul(self, other: u16) -> <Self as >::Output`

##### `impl MulAssign for u16_le`

- <span id="u16-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u16)`

##### `impl Not for u16_le`

- <span id="u16-le-not-type-output"></span>`type Output = <u16 as Not>::Output`

- <span id="u16-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u16_le`

- <span id="u16-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u16_le`

- <span id="u16-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u16_le`

- <span id="u16-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u16_le`

- <span id="u16-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u16_le`

- <span id="u16-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u16_le`

- <span id="u16-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u16_le`

- <span id="u16-le-rem-type-output"></span>`type Output = u16`

- <span id="u16-le-rem"></span>`fn rem(self, other: u16) -> <Self as >::Output`

##### `impl RemAssign for u16_le`

- <span id="u16-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u16)`

##### `impl Shl for u16_le`

- <span id="u16-le-shl-type-output"></span>`type Output = u16`

- <span id="u16-le-shl"></span>`fn shl(self, other: u16) -> <Self as >::Output`

##### `impl ShlAssign for u16_le`

- <span id="u16-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u16)`

##### `impl Shr for u16_le`

- <span id="u16-le-shr-type-output"></span>`type Output = u16`

- <span id="u16-le-shr"></span>`fn shr(self, other: u16) -> <Self as >::Output`

##### `impl ShrAssign for u16_le`

- <span id="u16-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u16)`

##### `impl Sub for u16_le`

- <span id="u16-le-sub-type-output"></span>`type Output = u16`

- <span id="u16-le-sub"></span>`fn sub(self, other: u16) -> <Self as >::Output`

##### `impl SubAssign for u16_le`

- <span id="u16-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u16)`

##### `impl Sum for u16_le`

- <span id="u16-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u16_le`

- <span id="u16-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u16_le`

- <span id="u16-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u16_le`

- <span id="u16-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u16_be`

```rust
struct u16_be(u16);
```

A big-endian `u16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="u16-be-from-native"></span>`const fn from_native(value: u16) -> Self`

  Returns a `u16_be` containing `value`.

- <span id="u16-be-to-native"></span>`const fn to_native(self) -> u16`

  Returns a `u16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u16_be`

- <span id="u16-be-add-type-output"></span>`type Output = u16`

- <span id="u16-be-add"></span>`fn add(self, other: u16) -> <Self as >::Output`

##### `impl AddAssign for u16_be`

- <span id="u16-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: u16)`

##### `impl Binary for u16_be`

- <span id="u16-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u16_be`

- <span id="u16-be-bitand-type-output"></span>`type Output = u16`

- <span id="u16-be-bitand"></span>`fn bitand(self, other: u16) -> <Self as >::Output`

##### `impl BitAndAssign for u16_be`

- <span id="u16-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u16)`

##### `impl BitOr for u16_be`

- <span id="u16-be-bitor-type-output"></span>`type Output = u16`

- <span id="u16-be-bitor"></span>`fn bitor(self, other: u16) -> <Self as >::Output`

##### `impl BitOrAssign for u16_be`

- <span id="u16-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u16)`

##### `impl BitXor for u16_be`

- <span id="u16-be-bitxor-type-output"></span>`type Output = u16`

- <span id="u16-be-bitxor"></span>`fn bitxor(self, other: u16) -> <Self as >::Output`

##### `impl BitXorAssign for u16_be`

- <span id="u16-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u16)`

##### `impl<C> CheckBytes for u16_be`

- <span id="u16-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u16_be`

- <span id="u16-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u16_be`

##### `impl Debug for u16_be`

- <span id="u16-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u16_be`

- <span id="u16-be-default"></span>`fn default() -> Self`

##### `impl Display for u16_be`

- <span id="u16-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u16_be`

- <span id="u16-be-div-type-output"></span>`type Output = u16`

- <span id="u16-be-div"></span>`fn div(self, other: u16) -> <Self as >::Output`

##### `impl DivAssign for u16_be`

- <span id="u16-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: u16)`

##### `impl Eq for u16_be`

##### `impl Hash for u16_be`

- <span id="u16-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u16_be`

- <span id="u16-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u16_be`

- <span id="u16-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u16_be`

- <span id="u16-be-mul-type-output"></span>`type Output = u16`

- <span id="u16-be-mul"></span>`fn mul(self, other: u16) -> <Self as >::Output`

##### `impl MulAssign for u16_be`

- <span id="u16-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u16)`

##### `impl Not for u16_be`

- <span id="u16-be-not-type-output"></span>`type Output = <u16 as Not>::Output`

- <span id="u16-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u16_be`

- <span id="u16-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u16_be`

- <span id="u16-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u16_be`

- <span id="u16-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u16_be`

- <span id="u16-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u16_be`

- <span id="u16-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u16_be`

- <span id="u16-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u16_be`

- <span id="u16-be-rem-type-output"></span>`type Output = u16`

- <span id="u16-be-rem"></span>`fn rem(self, other: u16) -> <Self as >::Output`

##### `impl RemAssign for u16_be`

- <span id="u16-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u16)`

##### `impl Shl for u16_be`

- <span id="u16-be-shl-type-output"></span>`type Output = u16`

- <span id="u16-be-shl"></span>`fn shl(self, other: u16) -> <Self as >::Output`

##### `impl ShlAssign for u16_be`

- <span id="u16-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u16)`

##### `impl Shr for u16_be`

- <span id="u16-be-shr-type-output"></span>`type Output = u16`

- <span id="u16-be-shr"></span>`fn shr(self, other: u16) -> <Self as >::Output`

##### `impl ShrAssign for u16_be`

- <span id="u16-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u16)`

##### `impl Sub for u16_be`

- <span id="u16-be-sub-type-output"></span>`type Output = u16`

- <span id="u16-be-sub"></span>`fn sub(self, other: u16) -> <Self as >::Output`

##### `impl SubAssign for u16_be`

- <span id="u16-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u16)`

##### `impl Sum for u16_be`

- <span id="u16-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u16_be`

- <span id="u16-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u16_be`

- <span id="u16-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u16_be`

- <span id="u16-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u32_le`

```rust
struct u32_le(u32);
```

A little-endian `u32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="u32-le-from-native"></span>`const fn from_native(value: u32) -> Self`

  Returns a `u32_le` containing `value`.

- <span id="u32-le-to-native"></span>`const fn to_native(self) -> u32`

  Returns a `u32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u32_le`

- <span id="u32-le-add-type-output"></span>`type Output = u32`

- <span id="u32-le-add"></span>`fn add(self, other: u32) -> <Self as >::Output`

##### `impl AddAssign for u32_le`

- <span id="u32-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: u32)`

##### `impl Binary for u32_le`

- <span id="u32-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u32_le`

- <span id="u32-le-bitand-type-output"></span>`type Output = u32`

- <span id="u32-le-bitand"></span>`fn bitand(self, other: u32) -> <Self as >::Output`

##### `impl BitAndAssign for u32_le`

- <span id="u32-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u32)`

##### `impl BitOr for u32_le`

- <span id="u32-le-bitor-type-output"></span>`type Output = u32`

- <span id="u32-le-bitor"></span>`fn bitor(self, other: u32) -> <Self as >::Output`

##### `impl BitOrAssign for u32_le`

- <span id="u32-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u32)`

##### `impl BitXor for u32_le`

- <span id="u32-le-bitxor-type-output"></span>`type Output = u32`

- <span id="u32-le-bitxor"></span>`fn bitxor(self, other: u32) -> <Self as >::Output`

##### `impl BitXorAssign for u32_le`

- <span id="u32-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u32)`

##### `impl<C> CheckBytes for u32_le`

- <span id="u32-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u32_le`

- <span id="u32-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u32_le`

##### `impl Debug for u32_le`

- <span id="u32-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u32_le`

- <span id="u32-le-default"></span>`fn default() -> Self`

##### `impl Display for u32_le`

- <span id="u32-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u32_le`

- <span id="u32-le-div-type-output"></span>`type Output = u32`

- <span id="u32-le-div"></span>`fn div(self, other: u32) -> <Self as >::Output`

##### `impl DivAssign for u32_le`

- <span id="u32-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: u32)`

##### `impl Eq for u32_le`

##### `impl Hash for u32_le`

- <span id="u32-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u32_le`

- <span id="u32-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u32_le`

- <span id="u32-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u32_le`

- <span id="u32-le-mul-type-output"></span>`type Output = u32`

- <span id="u32-le-mul"></span>`fn mul(self, other: u32) -> <Self as >::Output`

##### `impl MulAssign for u32_le`

- <span id="u32-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u32)`

##### `impl Not for u32_le`

- <span id="u32-le-not-type-output"></span>`type Output = <u32 as Not>::Output`

- <span id="u32-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u32_le`

- <span id="u32-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u32_le`

- <span id="u32-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u32_le`

- <span id="u32-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u32_le`

- <span id="u32-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u32_le`

- <span id="u32-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u32_le`

- <span id="u32-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u32_le`

- <span id="u32-le-rem-type-output"></span>`type Output = u32`

- <span id="u32-le-rem"></span>`fn rem(self, other: u32) -> <Self as >::Output`

##### `impl RemAssign for u32_le`

- <span id="u32-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u32)`

##### `impl Shl for u32_le`

- <span id="u32-le-shl-type-output"></span>`type Output = u32`

- <span id="u32-le-shl"></span>`fn shl(self, other: u32) -> <Self as >::Output`

##### `impl ShlAssign for u32_le`

- <span id="u32-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u32)`

##### `impl Shr for u32_le`

- <span id="u32-le-shr-type-output"></span>`type Output = u32`

- <span id="u32-le-shr"></span>`fn shr(self, other: u32) -> <Self as >::Output`

##### `impl ShrAssign for u32_le`

- <span id="u32-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u32)`

##### `impl Sub for u32_le`

- <span id="u32-le-sub-type-output"></span>`type Output = u32`

- <span id="u32-le-sub"></span>`fn sub(self, other: u32) -> <Self as >::Output`

##### `impl SubAssign for u32_le`

- <span id="u32-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u32)`

##### `impl Sum for u32_le`

- <span id="u32-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u32_le`

- <span id="u32-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u32_le`

- <span id="u32-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u32_le`

- <span id="u32-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u32_be`

```rust
struct u32_be(u32);
```

A big-endian `u32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="u32-be-from-native"></span>`const fn from_native(value: u32) -> Self`

  Returns a `u32_be` containing `value`.

- <span id="u32-be-to-native"></span>`const fn to_native(self) -> u32`

  Returns a `u32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u32_be`

- <span id="u32-be-add-type-output"></span>`type Output = u32`

- <span id="u32-be-add"></span>`fn add(self, other: u32) -> <Self as >::Output`

##### `impl AddAssign for u32_be`

- <span id="u32-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: u32)`

##### `impl Binary for u32_be`

- <span id="u32-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u32_be`

- <span id="u32-be-bitand-type-output"></span>`type Output = u32`

- <span id="u32-be-bitand"></span>`fn bitand(self, other: u32) -> <Self as >::Output`

##### `impl BitAndAssign for u32_be`

- <span id="u32-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u32)`

##### `impl BitOr for u32_be`

- <span id="u32-be-bitor-type-output"></span>`type Output = u32`

- <span id="u32-be-bitor"></span>`fn bitor(self, other: u32) -> <Self as >::Output`

##### `impl BitOrAssign for u32_be`

- <span id="u32-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u32)`

##### `impl BitXor for u32_be`

- <span id="u32-be-bitxor-type-output"></span>`type Output = u32`

- <span id="u32-be-bitxor"></span>`fn bitxor(self, other: u32) -> <Self as >::Output`

##### `impl BitXorAssign for u32_be`

- <span id="u32-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u32)`

##### `impl<C> CheckBytes for u32_be`

- <span id="u32-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u32_be`

- <span id="u32-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u32_be`

##### `impl Debug for u32_be`

- <span id="u32-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u32_be`

- <span id="u32-be-default"></span>`fn default() -> Self`

##### `impl Display for u32_be`

- <span id="u32-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u32_be`

- <span id="u32-be-div-type-output"></span>`type Output = u32`

- <span id="u32-be-div"></span>`fn div(self, other: u32) -> <Self as >::Output`

##### `impl DivAssign for u32_be`

- <span id="u32-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: u32)`

##### `impl Eq for u32_be`

##### `impl Hash for u32_be`

- <span id="u32-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u32_be`

- <span id="u32-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u32_be`

- <span id="u32-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u32_be`

- <span id="u32-be-mul-type-output"></span>`type Output = u32`

- <span id="u32-be-mul"></span>`fn mul(self, other: u32) -> <Self as >::Output`

##### `impl MulAssign for u32_be`

- <span id="u32-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u32)`

##### `impl Not for u32_be`

- <span id="u32-be-not-type-output"></span>`type Output = <u32 as Not>::Output`

- <span id="u32-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u32_be`

- <span id="u32-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u32_be`

- <span id="u32-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u32_be`

- <span id="u32-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u32_be`

- <span id="u32-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u32_be`

- <span id="u32-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u32_be`

- <span id="u32-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u32_be`

- <span id="u32-be-rem-type-output"></span>`type Output = u32`

- <span id="u32-be-rem"></span>`fn rem(self, other: u32) -> <Self as >::Output`

##### `impl RemAssign for u32_be`

- <span id="u32-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u32)`

##### `impl Shl for u32_be`

- <span id="u32-be-shl-type-output"></span>`type Output = u32`

- <span id="u32-be-shl"></span>`fn shl(self, other: u32) -> <Self as >::Output`

##### `impl ShlAssign for u32_be`

- <span id="u32-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u32)`

##### `impl Shr for u32_be`

- <span id="u32-be-shr-type-output"></span>`type Output = u32`

- <span id="u32-be-shr"></span>`fn shr(self, other: u32) -> <Self as >::Output`

##### `impl ShrAssign for u32_be`

- <span id="u32-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u32)`

##### `impl Sub for u32_be`

- <span id="u32-be-sub-type-output"></span>`type Output = u32`

- <span id="u32-be-sub"></span>`fn sub(self, other: u32) -> <Self as >::Output`

##### `impl SubAssign for u32_be`

- <span id="u32-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u32)`

##### `impl Sum for u32_be`

- <span id="u32-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u32_be`

- <span id="u32-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u32_be`

- <span id="u32-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u32_be`

- <span id="u32-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u64_le`

```rust
struct u64_le(u64);
```

A little-endian `u64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="u64-le-from-native"></span>`const fn from_native(value: u64) -> Self`

  Returns a `u64_le` containing `value`.

- <span id="u64-le-to-native"></span>`const fn to_native(self) -> u64`

  Returns a `u64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u64_le`

- <span id="u64-le-add-type-output"></span>`type Output = u64`

- <span id="u64-le-add"></span>`fn add(self, other: u64) -> <Self as >::Output`

##### `impl AddAssign for u64_le`

- <span id="u64-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: u64)`

##### `impl Binary for u64_le`

- <span id="u64-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u64_le`

- <span id="u64-le-bitand-type-output"></span>`type Output = u64`

- <span id="u64-le-bitand"></span>`fn bitand(self, other: u64) -> <Self as >::Output`

##### `impl BitAndAssign for u64_le`

- <span id="u64-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u64)`

##### `impl BitOr for u64_le`

- <span id="u64-le-bitor-type-output"></span>`type Output = u64`

- <span id="u64-le-bitor"></span>`fn bitor(self, other: u64) -> <Self as >::Output`

##### `impl BitOrAssign for u64_le`

- <span id="u64-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u64)`

##### `impl BitXor for u64_le`

- <span id="u64-le-bitxor-type-output"></span>`type Output = u64`

- <span id="u64-le-bitxor"></span>`fn bitxor(self, other: u64) -> <Self as >::Output`

##### `impl BitXorAssign for u64_le`

- <span id="u64-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u64)`

##### `impl<C> CheckBytes for u64_le`

- <span id="u64-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u64_le`

- <span id="u64-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u64_le`

##### `impl Debug for u64_le`

- <span id="u64-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u64_le`

- <span id="u64-le-default"></span>`fn default() -> Self`

##### `impl Display for u64_le`

- <span id="u64-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u64_le`

- <span id="u64-le-div-type-output"></span>`type Output = u64`

- <span id="u64-le-div"></span>`fn div(self, other: u64) -> <Self as >::Output`

##### `impl DivAssign for u64_le`

- <span id="u64-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: u64)`

##### `impl Eq for u64_le`

##### `impl Hash for u64_le`

- <span id="u64-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u64_le`

- <span id="u64-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u64_le`

- <span id="u64-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u64_le`

- <span id="u64-le-mul-type-output"></span>`type Output = u64`

- <span id="u64-le-mul"></span>`fn mul(self, other: u64) -> <Self as >::Output`

##### `impl MulAssign for u64_le`

- <span id="u64-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u64)`

##### `impl Not for u64_le`

- <span id="u64-le-not-type-output"></span>`type Output = <u64 as Not>::Output`

- <span id="u64-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u64_le`

- <span id="u64-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u64_le`

- <span id="u64-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u64_le`

- <span id="u64-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u64_le`

- <span id="u64-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u64_le`

- <span id="u64-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u64_le`

- <span id="u64-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u64_le`

- <span id="u64-le-rem-type-output"></span>`type Output = u64`

- <span id="u64-le-rem"></span>`fn rem(self, other: u64) -> <Self as >::Output`

##### `impl RemAssign for u64_le`

- <span id="u64-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u64)`

##### `impl Shl for u64_le`

- <span id="u64-le-shl-type-output"></span>`type Output = u64`

- <span id="u64-le-shl"></span>`fn shl(self, other: u64) -> <Self as >::Output`

##### `impl ShlAssign for u64_le`

- <span id="u64-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u64)`

##### `impl Shr for u64_le`

- <span id="u64-le-shr-type-output"></span>`type Output = u64`

- <span id="u64-le-shr"></span>`fn shr(self, other: u64) -> <Self as >::Output`

##### `impl ShrAssign for u64_le`

- <span id="u64-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u64)`

##### `impl Sub for u64_le`

- <span id="u64-le-sub-type-output"></span>`type Output = u64`

- <span id="u64-le-sub"></span>`fn sub(self, other: u64) -> <Self as >::Output`

##### `impl SubAssign for u64_le`

- <span id="u64-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u64)`

##### `impl Sum for u64_le`

- <span id="u64-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u64_le`

- <span id="u64-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u64_le`

- <span id="u64-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u64_le`

- <span id="u64-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u64_be`

```rust
struct u64_be(u64);
```

A big-endian `u64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="u64-be-from-native"></span>`const fn from_native(value: u64) -> Self`

  Returns a `u64_be` containing `value`.

- <span id="u64-be-to-native"></span>`const fn to_native(self) -> u64`

  Returns a `u64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u64_be`

- <span id="u64-be-add-type-output"></span>`type Output = u64`

- <span id="u64-be-add"></span>`fn add(self, other: u64) -> <Self as >::Output`

##### `impl AddAssign for u64_be`

- <span id="u64-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: u64)`

##### `impl Binary for u64_be`

- <span id="u64-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u64_be`

- <span id="u64-be-bitand-type-output"></span>`type Output = u64`

- <span id="u64-be-bitand"></span>`fn bitand(self, other: u64) -> <Self as >::Output`

##### `impl BitAndAssign for u64_be`

- <span id="u64-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u64)`

##### `impl BitOr for u64_be`

- <span id="u64-be-bitor-type-output"></span>`type Output = u64`

- <span id="u64-be-bitor"></span>`fn bitor(self, other: u64) -> <Self as >::Output`

##### `impl BitOrAssign for u64_be`

- <span id="u64-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u64)`

##### `impl BitXor for u64_be`

- <span id="u64-be-bitxor-type-output"></span>`type Output = u64`

- <span id="u64-be-bitxor"></span>`fn bitxor(self, other: u64) -> <Self as >::Output`

##### `impl BitXorAssign for u64_be`

- <span id="u64-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u64)`

##### `impl<C> CheckBytes for u64_be`

- <span id="u64-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u64_be`

- <span id="u64-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u64_be`

##### `impl Debug for u64_be`

- <span id="u64-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u64_be`

- <span id="u64-be-default"></span>`fn default() -> Self`

##### `impl Display for u64_be`

- <span id="u64-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u64_be`

- <span id="u64-be-div-type-output"></span>`type Output = u64`

- <span id="u64-be-div"></span>`fn div(self, other: u64) -> <Self as >::Output`

##### `impl DivAssign for u64_be`

- <span id="u64-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: u64)`

##### `impl Eq for u64_be`

##### `impl Hash for u64_be`

- <span id="u64-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u64_be`

- <span id="u64-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u64_be`

- <span id="u64-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u64_be`

- <span id="u64-be-mul-type-output"></span>`type Output = u64`

- <span id="u64-be-mul"></span>`fn mul(self, other: u64) -> <Self as >::Output`

##### `impl MulAssign for u64_be`

- <span id="u64-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u64)`

##### `impl Not for u64_be`

- <span id="u64-be-not-type-output"></span>`type Output = <u64 as Not>::Output`

- <span id="u64-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u64_be`

- <span id="u64-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u64_be`

- <span id="u64-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u64_be`

- <span id="u64-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u64_be`

- <span id="u64-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u64_be`

- <span id="u64-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u64_be`

- <span id="u64-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u64_be`

- <span id="u64-be-rem-type-output"></span>`type Output = u64`

- <span id="u64-be-rem"></span>`fn rem(self, other: u64) -> <Self as >::Output`

##### `impl RemAssign for u64_be`

- <span id="u64-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u64)`

##### `impl Shl for u64_be`

- <span id="u64-be-shl-type-output"></span>`type Output = u64`

- <span id="u64-be-shl"></span>`fn shl(self, other: u64) -> <Self as >::Output`

##### `impl ShlAssign for u64_be`

- <span id="u64-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u64)`

##### `impl Shr for u64_be`

- <span id="u64-be-shr-type-output"></span>`type Output = u64`

- <span id="u64-be-shr"></span>`fn shr(self, other: u64) -> <Self as >::Output`

##### `impl ShrAssign for u64_be`

- <span id="u64-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u64)`

##### `impl Sub for u64_be`

- <span id="u64-be-sub-type-output"></span>`type Output = u64`

- <span id="u64-be-sub"></span>`fn sub(self, other: u64) -> <Self as >::Output`

##### `impl SubAssign for u64_be`

- <span id="u64-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u64)`

##### `impl Sum for u64_be`

- <span id="u64-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u64_be`

- <span id="u64-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u64_be`

- <span id="u64-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u64_be`

- <span id="u64-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u128_le`

```rust
struct u128_le(u128);
```

A little-endian `u128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="u128-le-from-native"></span>`const fn from_native(value: u128) -> Self`

  Returns a `u128_le` containing `value`.

- <span id="u128-le-to-native"></span>`const fn to_native(self) -> u128`

  Returns a `u128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u128_le`

- <span id="u128-le-add-type-output"></span>`type Output = u128`

- <span id="u128-le-add"></span>`fn add(self, other: u128) -> <Self as >::Output`

##### `impl AddAssign for u128_le`

- <span id="u128-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: u128)`

##### `impl Binary for u128_le`

- <span id="u128-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u128_le`

- <span id="u128-le-bitand-type-output"></span>`type Output = u128`

- <span id="u128-le-bitand"></span>`fn bitand(self, other: u128) -> <Self as >::Output`

##### `impl BitAndAssign for u128_le`

- <span id="u128-le-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u128)`

##### `impl BitOr for u128_le`

- <span id="u128-le-bitor-type-output"></span>`type Output = u128`

- <span id="u128-le-bitor"></span>`fn bitor(self, other: u128) -> <Self as >::Output`

##### `impl BitOrAssign for u128_le`

- <span id="u128-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u128)`

##### `impl BitXor for u128_le`

- <span id="u128-le-bitxor-type-output"></span>`type Output = u128`

- <span id="u128-le-bitxor"></span>`fn bitxor(self, other: u128) -> <Self as >::Output`

##### `impl BitXorAssign for u128_le`

- <span id="u128-le-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u128)`

##### `impl<C> CheckBytes for u128_le`

- <span id="u128-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u128_le`

- <span id="u128-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u128_le`

##### `impl Debug for u128_le`

- <span id="u128-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u128_le`

- <span id="u128-le-default"></span>`fn default() -> Self`

##### `impl Display for u128_le`

- <span id="u128-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u128_le`

- <span id="u128-le-div-type-output"></span>`type Output = u128`

- <span id="u128-le-div"></span>`fn div(self, other: u128) -> <Self as >::Output`

##### `impl DivAssign for u128_le`

- <span id="u128-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: u128)`

##### `impl Eq for u128_le`

##### `impl Hash for u128_le`

- <span id="u128-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u128_le`

- <span id="u128-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u128_le`

- <span id="u128-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u128_le`

- <span id="u128-le-mul-type-output"></span>`type Output = u128`

- <span id="u128-le-mul"></span>`fn mul(self, other: u128) -> <Self as >::Output`

##### `impl MulAssign for u128_le`

- <span id="u128-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u128)`

##### `impl Not for u128_le`

- <span id="u128-le-not-type-output"></span>`type Output = <u128 as Not>::Output`

- <span id="u128-le-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u128_le`

- <span id="u128-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u128_le`

- <span id="u128-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u128_le`

- <span id="u128-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u128_le`

- <span id="u128-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u128_le`

- <span id="u128-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u128_le`

- <span id="u128-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u128_le`

- <span id="u128-le-rem-type-output"></span>`type Output = u128`

- <span id="u128-le-rem"></span>`fn rem(self, other: u128) -> <Self as >::Output`

##### `impl RemAssign for u128_le`

- <span id="u128-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u128)`

##### `impl Shl for u128_le`

- <span id="u128-le-shl-type-output"></span>`type Output = u128`

- <span id="u128-le-shl"></span>`fn shl(self, other: u128) -> <Self as >::Output`

##### `impl ShlAssign for u128_le`

- <span id="u128-le-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u128)`

##### `impl Shr for u128_le`

- <span id="u128-le-shr-type-output"></span>`type Output = u128`

- <span id="u128-le-shr"></span>`fn shr(self, other: u128) -> <Self as >::Output`

##### `impl ShrAssign for u128_le`

- <span id="u128-le-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u128)`

##### `impl Sub for u128_le`

- <span id="u128-le-sub-type-output"></span>`type Output = u128`

- <span id="u128-le-sub"></span>`fn sub(self, other: u128) -> <Self as >::Output`

##### `impl SubAssign for u128_le`

- <span id="u128-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u128)`

##### `impl Sum for u128_le`

- <span id="u128-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u128_le`

- <span id="u128-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u128_le`

- <span id="u128-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u128_le`

- <span id="u128-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u128_be`

```rust
struct u128_be(u128);
```

A big-endian `u128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="u128-be-from-native"></span>`const fn from_native(value: u128) -> Self`

  Returns a `u128_be` containing `value`.

- <span id="u128-be-to-native"></span>`const fn to_native(self) -> u128`

  Returns a `u128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u128_be`

- <span id="u128-be-add-type-output"></span>`type Output = u128`

- <span id="u128-be-add"></span>`fn add(self, other: u128) -> <Self as >::Output`

##### `impl AddAssign for u128_be`

- <span id="u128-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: u128)`

##### `impl Binary for u128_be`

- <span id="u128-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u128_be`

- <span id="u128-be-bitand-type-output"></span>`type Output = u128`

- <span id="u128-be-bitand"></span>`fn bitand(self, other: u128) -> <Self as >::Output`

##### `impl BitAndAssign for u128_be`

- <span id="u128-be-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u128)`

##### `impl BitOr for u128_be`

- <span id="u128-be-bitor-type-output"></span>`type Output = u128`

- <span id="u128-be-bitor"></span>`fn bitor(self, other: u128) -> <Self as >::Output`

##### `impl BitOrAssign for u128_be`

- <span id="u128-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u128)`

##### `impl BitXor for u128_be`

- <span id="u128-be-bitxor-type-output"></span>`type Output = u128`

- <span id="u128-be-bitxor"></span>`fn bitxor(self, other: u128) -> <Self as >::Output`

##### `impl BitXorAssign for u128_be`

- <span id="u128-be-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u128)`

##### `impl<C> CheckBytes for u128_be`

- <span id="u128-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u128_be`

- <span id="u128-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u128_be`

##### `impl Debug for u128_be`

- <span id="u128-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u128_be`

- <span id="u128-be-default"></span>`fn default() -> Self`

##### `impl Display for u128_be`

- <span id="u128-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u128_be`

- <span id="u128-be-div-type-output"></span>`type Output = u128`

- <span id="u128-be-div"></span>`fn div(self, other: u128) -> <Self as >::Output`

##### `impl DivAssign for u128_be`

- <span id="u128-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: u128)`

##### `impl Eq for u128_be`

##### `impl Hash for u128_be`

- <span id="u128-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u128_be`

- <span id="u128-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u128_be`

- <span id="u128-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u128_be`

- <span id="u128-be-mul-type-output"></span>`type Output = u128`

- <span id="u128-be-mul"></span>`fn mul(self, other: u128) -> <Self as >::Output`

##### `impl MulAssign for u128_be`

- <span id="u128-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u128)`

##### `impl Not for u128_be`

- <span id="u128-be-not-type-output"></span>`type Output = <u128 as Not>::Output`

- <span id="u128-be-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u128_be`

- <span id="u128-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u128_be`

- <span id="u128-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u128_be`

- <span id="u128-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u128_be`

- <span id="u128-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u128_be`

- <span id="u128-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u128_be`

- <span id="u128-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u128_be`

- <span id="u128-be-rem-type-output"></span>`type Output = u128`

- <span id="u128-be-rem"></span>`fn rem(self, other: u128) -> <Self as >::Output`

##### `impl RemAssign for u128_be`

- <span id="u128-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u128)`

##### `impl Shl for u128_be`

- <span id="u128-be-shl-type-output"></span>`type Output = u128`

- <span id="u128-be-shl"></span>`fn shl(self, other: u128) -> <Self as >::Output`

##### `impl ShlAssign for u128_be`

- <span id="u128-be-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u128)`

##### `impl Shr for u128_be`

- <span id="u128-be-shr-type-output"></span>`type Output = u128`

- <span id="u128-be-shr"></span>`fn shr(self, other: u128) -> <Self as >::Output`

##### `impl ShrAssign for u128_be`

- <span id="u128-be-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u128)`

##### `impl Sub for u128_be`

- <span id="u128-be-sub-type-output"></span>`type Output = u128`

- <span id="u128-be-sub"></span>`fn sub(self, other: u128) -> <Self as >::Output`

##### `impl SubAssign for u128_be`

- <span id="u128-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u128)`

##### `impl Sum for u128_be`

- <span id="u128-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u128_be`

- <span id="u128-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u128_be`

- <span id="u128-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u128_be`

- <span id="u128-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f32_le`

```rust
struct f32_le(f32);
```

A little-endian `f32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="f32-le-from-native"></span>`const fn from_native(value: f32) -> Self`

  Returns a `f32_le` containing `value`.

- <span id="f32-le-to-native"></span>`const fn to_native(self) -> f32`

  Returns a `f32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f32_le`

- <span id="f32-le-add-type-output"></span>`type Output = f32`

- <span id="f32-le-add"></span>`fn add(self, other: f32) -> <Self as >::Output`

##### `impl AddAssign for f32_le`

- <span id="f32-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: f32)`

##### `impl<C> CheckBytes for f32_le`

- <span id="f32-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f32_le`

- <span id="f32-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f32_le`

##### `impl Debug for f32_le`

- <span id="f32-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f32_le`

- <span id="f32-le-default"></span>`fn default() -> Self`

##### `impl Display for f32_le`

- <span id="f32-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f32_le`

- <span id="f32-le-div-type-output"></span>`type Output = f32`

- <span id="f32-le-div"></span>`fn div(self, other: f32) -> <Self as >::Output`

##### `impl DivAssign for f32_le`

- <span id="f32-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: f32)`

##### `impl Eq for f32_le`

##### `impl LowerExp for f32_le`

- <span id="f32-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f32_le`

- <span id="f32-le-mul-type-output"></span>`type Output = f32`

- <span id="f32-le-mul"></span>`fn mul(self, other: f32) -> <Self as >::Output`

##### `impl MulAssign for f32_le`

- <span id="f32-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f32)`

##### `impl Neg for f32_le`

- <span id="f32-le-neg-type-output"></span>`type Output = <f32 as Neg>::Output`

- <span id="f32-le-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f32_le`

- <span id="f32-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f32_le`

- <span id="f32-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f32_le`

- <span id="f32-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f32_le`

- <span id="f32-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f32_le`

- <span id="f32-le-rem-type-output"></span>`type Output = f32`

- <span id="f32-le-rem"></span>`fn rem(self, other: f32) -> <Self as >::Output`

##### `impl RemAssign for f32_le`

- <span id="f32-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f32)`

##### `impl Sub for f32_le`

- <span id="f32-le-sub-type-output"></span>`type Output = f32`

- <span id="f32-le-sub"></span>`fn sub(self, other: f32) -> <Self as >::Output`

##### `impl SubAssign for f32_le`

- <span id="f32-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f32)`

##### `impl Sum for f32_le`

- <span id="f32-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f32_le`

- <span id="f32-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f32_le`

- <span id="f32-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f32_be`

```rust
struct f32_be(f32);
```

A big-endian `f32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="f32-be-from-native"></span>`const fn from_native(value: f32) -> Self`

  Returns a `f32_be` containing `value`.

- <span id="f32-be-to-native"></span>`const fn to_native(self) -> f32`

  Returns a `f32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f32_be`

- <span id="f32-be-add-type-output"></span>`type Output = f32`

- <span id="f32-be-add"></span>`fn add(self, other: f32) -> <Self as >::Output`

##### `impl AddAssign for f32_be`

- <span id="f32-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: f32)`

##### `impl<C> CheckBytes for f32_be`

- <span id="f32-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f32_be`

- <span id="f32-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f32_be`

##### `impl Debug for f32_be`

- <span id="f32-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f32_be`

- <span id="f32-be-default"></span>`fn default() -> Self`

##### `impl Display for f32_be`

- <span id="f32-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f32_be`

- <span id="f32-be-div-type-output"></span>`type Output = f32`

- <span id="f32-be-div"></span>`fn div(self, other: f32) -> <Self as >::Output`

##### `impl DivAssign for f32_be`

- <span id="f32-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: f32)`

##### `impl Eq for f32_be`

##### `impl LowerExp for f32_be`

- <span id="f32-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f32_be`

- <span id="f32-be-mul-type-output"></span>`type Output = f32`

- <span id="f32-be-mul"></span>`fn mul(self, other: f32) -> <Self as >::Output`

##### `impl MulAssign for f32_be`

- <span id="f32-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f32)`

##### `impl Neg for f32_be`

- <span id="f32-be-neg-type-output"></span>`type Output = <f32 as Neg>::Output`

- <span id="f32-be-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f32_be`

- <span id="f32-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f32_be`

- <span id="f32-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f32_be`

- <span id="f32-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f32_be`

- <span id="f32-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f32_be`

- <span id="f32-be-rem-type-output"></span>`type Output = f32`

- <span id="f32-be-rem"></span>`fn rem(self, other: f32) -> <Self as >::Output`

##### `impl RemAssign for f32_be`

- <span id="f32-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f32)`

##### `impl Sub for f32_be`

- <span id="f32-be-sub-type-output"></span>`type Output = f32`

- <span id="f32-be-sub"></span>`fn sub(self, other: f32) -> <Self as >::Output`

##### `impl SubAssign for f32_be`

- <span id="f32-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f32)`

##### `impl Sum for f32_be`

- <span id="f32-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f32_be`

- <span id="f32-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f32_be`

- <span id="f32-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f64_le`

```rust
struct f64_le(f64);
```

A little-endian `f64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="f64-le-from-native"></span>`const fn from_native(value: f64) -> Self`

  Returns a `f64_le` containing `value`.

- <span id="f64-le-to-native"></span>`const fn to_native(self) -> f64`

  Returns a `f64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f64_le`

- <span id="f64-le-add-type-output"></span>`type Output = f64`

- <span id="f64-le-add"></span>`fn add(self, other: f64) -> <Self as >::Output`

##### `impl AddAssign for f64_le`

- <span id="f64-le-addassign-add-assign"></span>`fn add_assign(&mut self, other: f64)`

##### `impl<C> CheckBytes for f64_le`

- <span id="f64-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f64_le`

- <span id="f64-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f64_le`

##### `impl Debug for f64_le`

- <span id="f64-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f64_le`

- <span id="f64-le-default"></span>`fn default() -> Self`

##### `impl Display for f64_le`

- <span id="f64-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f64_le`

- <span id="f64-le-div-type-output"></span>`type Output = f64`

- <span id="f64-le-div"></span>`fn div(self, other: f64) -> <Self as >::Output`

##### `impl DivAssign for f64_le`

- <span id="f64-le-divassign-div-assign"></span>`fn div_assign(&mut self, other: f64)`

##### `impl Eq for f64_le`

##### `impl LowerExp for f64_le`

- <span id="f64-le-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f64_le`

- <span id="f64-le-mul-type-output"></span>`type Output = f64`

- <span id="f64-le-mul"></span>`fn mul(self, other: f64) -> <Self as >::Output`

##### `impl MulAssign for f64_le`

- <span id="f64-le-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f64)`

##### `impl Neg for f64_le`

- <span id="f64-le-neg-type-output"></span>`type Output = <f64 as Neg>::Output`

- <span id="f64-le-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f64_le`

- <span id="f64-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f64_le`

- <span id="f64-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f64_le`

- <span id="f64-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f64_le`

- <span id="f64-le-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f64_le`

- <span id="f64-le-rem-type-output"></span>`type Output = f64`

- <span id="f64-le-rem"></span>`fn rem(self, other: f64) -> <Self as >::Output`

##### `impl RemAssign for f64_le`

- <span id="f64-le-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f64)`

##### `impl Sub for f64_le`

- <span id="f64-le-sub-type-output"></span>`type Output = f64`

- <span id="f64-le-sub"></span>`fn sub(self, other: f64) -> <Self as >::Output`

##### `impl SubAssign for f64_le`

- <span id="f64-le-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f64)`

##### `impl Sum for f64_le`

- <span id="f64-le-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f64_le`

- <span id="f64-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f64_le`

- <span id="f64-le-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f64_be`

```rust
struct f64_be(f64);
```

A big-endian `f64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="f64-be-from-native"></span>`const fn from_native(value: f64) -> Self`

  Returns a `f64_be` containing `value`.

- <span id="f64-be-to-native"></span>`const fn to_native(self) -> f64`

  Returns a `f64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f64_be`

- <span id="f64-be-add-type-output"></span>`type Output = f64`

- <span id="f64-be-add"></span>`fn add(self, other: f64) -> <Self as >::Output`

##### `impl AddAssign for f64_be`

- <span id="f64-be-addassign-add-assign"></span>`fn add_assign(&mut self, other: f64)`

##### `impl<C> CheckBytes for f64_be`

- <span id="f64-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f64_be`

- <span id="f64-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f64_be`

##### `impl Debug for f64_be`

- <span id="f64-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f64_be`

- <span id="f64-be-default"></span>`fn default() -> Self`

##### `impl Display for f64_be`

- <span id="f64-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f64_be`

- <span id="f64-be-div-type-output"></span>`type Output = f64`

- <span id="f64-be-div"></span>`fn div(self, other: f64) -> <Self as >::Output`

##### `impl DivAssign for f64_be`

- <span id="f64-be-divassign-div-assign"></span>`fn div_assign(&mut self, other: f64)`

##### `impl Eq for f64_be`

##### `impl LowerExp for f64_be`

- <span id="f64-be-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f64_be`

- <span id="f64-be-mul-type-output"></span>`type Output = f64`

- <span id="f64-be-mul"></span>`fn mul(self, other: f64) -> <Self as >::Output`

##### `impl MulAssign for f64_be`

- <span id="f64-be-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f64)`

##### `impl Neg for f64_be`

- <span id="f64-be-neg-type-output"></span>`type Output = <f64 as Neg>::Output`

- <span id="f64-be-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f64_be`

- <span id="f64-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f64_be`

- <span id="f64-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f64_be`

- <span id="f64-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f64_be`

- <span id="f64-be-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f64_be`

- <span id="f64-be-rem-type-output"></span>`type Output = f64`

- <span id="f64-be-rem"></span>`fn rem(self, other: f64) -> <Self as >::Output`

##### `impl RemAssign for f64_be`

- <span id="f64-be-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f64)`

##### `impl Sub for f64_be`

- <span id="f64-be-sub-type-output"></span>`type Output = f64`

- <span id="f64-be-sub"></span>`fn sub(self, other: f64) -> <Self as >::Output`

##### `impl SubAssign for f64_be`

- <span id="f64-be-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f64)`

##### `impl Sum for f64_be`

- <span id="f64-be-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f64_be`

- <span id="f64-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f64_be`

- <span id="f64-be-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `char_le`

```rust
struct char_le(u32);
```

A little-endian `u32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="char-le-from-native"></span>`const fn from_native(value: char) -> Self`

  Returns a `char_le` containing `value`.

- <span id="char-le-to-native"></span>`const fn to_native(self) -> char`

  Returns a `$prim` with the same value as `self`.

#### Trait Implementations

##### `impl<C> CheckBytes for char_le`

- <span id="char-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for char_le`

- <span id="char-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for char_le`

##### `impl Debug for char_le`

- <span id="char-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for char_le`

- <span id="char-le-default"></span>`fn default() -> Self`

##### `impl Display for char_le`

- <span id="char-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for char_le`

##### `impl Hash for char_le`

- <span id="char-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl Ord for char_le`

- <span id="char-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for char_le`

- <span id="char-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for char_le`

- <span id="char-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for char_le`

- <span id="char-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for char_le`

- <span id="char-le-tostring-to-string"></span>`fn to_string(&self) -> String`

### `char_be`

```rust
struct char_be(u32);
```

A big-endian `u32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="char-be-from-native"></span>`const fn from_native(value: char) -> Self`

  Returns a `char_be` containing `value`.

- <span id="char-be-to-native"></span>`const fn to_native(self) -> char`

  Returns a `$prim` with the same value as `self`.

#### Trait Implementations

##### `impl<C> CheckBytes for char_be`

- <span id="char-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for char_be`

- <span id="char-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for char_be`

##### `impl Debug for char_be`

- <span id="char-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for char_be`

- <span id="char-be-default"></span>`fn default() -> Self`

##### `impl Display for char_be`

- <span id="char-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for char_be`

##### `impl Hash for char_be`

- <span id="char-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl Ord for char_be`

- <span id="char-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for char_be`

- <span id="char-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for char_be`

- <span id="char-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for char_be`

- <span id="char-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for char_be`

- <span id="char-be-tostring-to-string"></span>`fn to_string(&self) -> String`

### `NonZeroI16_le`

```rust
struct NonZeroI16_le(core::num::NonZeroI16);
```

A little-endian `NonZeroI16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="nonzeroi16-le-new"></span>`const fn new(value: i16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi16-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: i16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi16-le-get"></span>`const fn get(self) -> i16`

  Returns the value as a primitive type.

- <span id="nonzeroi16-le-from-native"></span>`const fn from_native(value: NonZeroI16) -> Self`

  Returns a `NonZeroI16_le` containing `value`.

- <span id="nonzeroi16-le-to-native"></span>`const fn to_native(self) -> NonZeroI16`

  Returns a `NonZeroI16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI16_le`

- <span id="nonzeroi16-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI16_le`

- <span id="nonzeroi16-le-bitor-type-output"></span>`type Output = NonZero<i16>`

- <span id="nonzeroi16-le-bitor"></span>`fn bitor(self, other: NonZeroI16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI16_le`

- <span id="nonzeroi16-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI16)`

##### `impl<C> CheckBytes for NonZeroI16_le`

- <span id="nonzeroi16-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI16_le`

- <span id="nonzeroi16-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI16_le`

##### `impl Debug for NonZeroI16_le`

- <span id="nonzeroi16-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI16_le`

- <span id="nonzeroi16-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI16_le`

##### `impl Hash for NonZeroI16_le`

- <span id="nonzeroi16-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI16_le`

- <span id="nonzeroi16-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI16_le`

- <span id="nonzeroi16-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI16_le`

- <span id="nonzeroi16-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI16_le`

- <span id="nonzeroi16-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI16_le`

- <span id="nonzeroi16-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI16_le`

- <span id="nonzeroi16-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI16_le`

- <span id="nonzeroi16-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI16_le`

- <span id="nonzeroi16-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI16_be`

```rust
struct NonZeroI16_be(core::num::NonZeroI16);
```

A big-endian `NonZeroI16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="nonzeroi16-be-new"></span>`const fn new(value: i16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi16-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: i16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi16-be-get"></span>`const fn get(self) -> i16`

  Returns the value as a primitive type.

- <span id="nonzeroi16-be-from-native"></span>`const fn from_native(value: NonZeroI16) -> Self`

  Returns a `NonZeroI16_be` containing `value`.

- <span id="nonzeroi16-be-to-native"></span>`const fn to_native(self) -> NonZeroI16`

  Returns a `NonZeroI16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI16_be`

- <span id="nonzeroi16-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI16_be`

- <span id="nonzeroi16-be-bitor-type-output"></span>`type Output = NonZero<i16>`

- <span id="nonzeroi16-be-bitor"></span>`fn bitor(self, other: NonZeroI16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI16_be`

- <span id="nonzeroi16-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI16)`

##### `impl<C> CheckBytes for NonZeroI16_be`

- <span id="nonzeroi16-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI16_be`

- <span id="nonzeroi16-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI16_be`

##### `impl Debug for NonZeroI16_be`

- <span id="nonzeroi16-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI16_be`

- <span id="nonzeroi16-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI16_be`

##### `impl Hash for NonZeroI16_be`

- <span id="nonzeroi16-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI16_be`

- <span id="nonzeroi16-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI16_be`

- <span id="nonzeroi16-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI16_be`

- <span id="nonzeroi16-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI16_be`

- <span id="nonzeroi16-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI16_be`

- <span id="nonzeroi16-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI16_be`

- <span id="nonzeroi16-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI16_be`

- <span id="nonzeroi16-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI16_be`

- <span id="nonzeroi16-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI32_le`

```rust
struct NonZeroI32_le(core::num::NonZeroI32);
```

A little-endian `NonZeroI32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="nonzeroi32-le-new"></span>`const fn new(value: i32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi32-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: i32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi32-le-get"></span>`const fn get(self) -> i32`

  Returns the value as a primitive type.

- <span id="nonzeroi32-le-from-native"></span>`const fn from_native(value: NonZeroI32) -> Self`

  Returns a `NonZeroI32_le` containing `value`.

- <span id="nonzeroi32-le-to-native"></span>`const fn to_native(self) -> NonZeroI32`

  Returns a `NonZeroI32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI32_le`

- <span id="nonzeroi32-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI32_le`

- <span id="nonzeroi32-le-bitor-type-output"></span>`type Output = NonZero<i32>`

- <span id="nonzeroi32-le-bitor"></span>`fn bitor(self, other: NonZeroI32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI32_le`

- <span id="nonzeroi32-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI32)`

##### `impl<C> CheckBytes for NonZeroI32_le`

- <span id="nonzeroi32-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI32_le`

- <span id="nonzeroi32-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI32_le`

##### `impl Debug for NonZeroI32_le`

- <span id="nonzeroi32-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI32_le`

- <span id="nonzeroi32-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI32_le`

##### `impl Hash for NonZeroI32_le`

- <span id="nonzeroi32-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI32_le`

- <span id="nonzeroi32-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI32_le`

- <span id="nonzeroi32-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI32_le`

- <span id="nonzeroi32-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI32_le`

- <span id="nonzeroi32-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI32_le`

- <span id="nonzeroi32-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI32_le`

- <span id="nonzeroi32-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI32_le`

- <span id="nonzeroi32-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI32_le`

- <span id="nonzeroi32-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI32_be`

```rust
struct NonZeroI32_be(core::num::NonZeroI32);
```

A big-endian `NonZeroI32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="nonzeroi32-be-new"></span>`const fn new(value: i32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi32-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: i32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi32-be-get"></span>`const fn get(self) -> i32`

  Returns the value as a primitive type.

- <span id="nonzeroi32-be-from-native"></span>`const fn from_native(value: NonZeroI32) -> Self`

  Returns a `NonZeroI32_be` containing `value`.

- <span id="nonzeroi32-be-to-native"></span>`const fn to_native(self) -> NonZeroI32`

  Returns a `NonZeroI32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI32_be`

- <span id="nonzeroi32-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI32_be`

- <span id="nonzeroi32-be-bitor-type-output"></span>`type Output = NonZero<i32>`

- <span id="nonzeroi32-be-bitor"></span>`fn bitor(self, other: NonZeroI32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI32_be`

- <span id="nonzeroi32-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI32)`

##### `impl<C> CheckBytes for NonZeroI32_be`

- <span id="nonzeroi32-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI32_be`

- <span id="nonzeroi32-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI32_be`

##### `impl Debug for NonZeroI32_be`

- <span id="nonzeroi32-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI32_be`

- <span id="nonzeroi32-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI32_be`

##### `impl Hash for NonZeroI32_be`

- <span id="nonzeroi32-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI32_be`

- <span id="nonzeroi32-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI32_be`

- <span id="nonzeroi32-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI32_be`

- <span id="nonzeroi32-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI32_be`

- <span id="nonzeroi32-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI32_be`

- <span id="nonzeroi32-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI32_be`

- <span id="nonzeroi32-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI32_be`

- <span id="nonzeroi32-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI32_be`

- <span id="nonzeroi32-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI64_le`

```rust
struct NonZeroI64_le(core::num::NonZeroI64);
```

A little-endian `NonZeroI64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="nonzeroi64-le-new"></span>`const fn new(value: i64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi64-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: i64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi64-le-get"></span>`const fn get(self) -> i64`

  Returns the value as a primitive type.

- <span id="nonzeroi64-le-from-native"></span>`const fn from_native(value: NonZeroI64) -> Self`

  Returns a `NonZeroI64_le` containing `value`.

- <span id="nonzeroi64-le-to-native"></span>`const fn to_native(self) -> NonZeroI64`

  Returns a `NonZeroI64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI64_le`

- <span id="nonzeroi64-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI64_le`

- <span id="nonzeroi64-le-bitor-type-output"></span>`type Output = NonZero<i64>`

- <span id="nonzeroi64-le-bitor"></span>`fn bitor(self, other: NonZeroI64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI64_le`

- <span id="nonzeroi64-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI64)`

##### `impl<C> CheckBytes for NonZeroI64_le`

- <span id="nonzeroi64-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI64_le`

- <span id="nonzeroi64-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI64_le`

##### `impl Debug for NonZeroI64_le`

- <span id="nonzeroi64-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI64_le`

- <span id="nonzeroi64-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI64_le`

##### `impl Hash for NonZeroI64_le`

- <span id="nonzeroi64-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI64_le`

- <span id="nonzeroi64-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI64_le`

- <span id="nonzeroi64-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI64_le`

- <span id="nonzeroi64-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI64_le`

- <span id="nonzeroi64-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI64_le`

- <span id="nonzeroi64-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI64_le`

- <span id="nonzeroi64-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI64_le`

- <span id="nonzeroi64-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI64_le`

- <span id="nonzeroi64-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI64_be`

```rust
struct NonZeroI64_be(core::num::NonZeroI64);
```

A big-endian `NonZeroI64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="nonzeroi64-be-new"></span>`const fn new(value: i64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi64-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: i64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi64-be-get"></span>`const fn get(self) -> i64`

  Returns the value as a primitive type.

- <span id="nonzeroi64-be-from-native"></span>`const fn from_native(value: NonZeroI64) -> Self`

  Returns a `NonZeroI64_be` containing `value`.

- <span id="nonzeroi64-be-to-native"></span>`const fn to_native(self) -> NonZeroI64`

  Returns a `NonZeroI64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI64_be`

- <span id="nonzeroi64-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI64_be`

- <span id="nonzeroi64-be-bitor-type-output"></span>`type Output = NonZero<i64>`

- <span id="nonzeroi64-be-bitor"></span>`fn bitor(self, other: NonZeroI64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI64_be`

- <span id="nonzeroi64-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI64)`

##### `impl<C> CheckBytes for NonZeroI64_be`

- <span id="nonzeroi64-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI64_be`

- <span id="nonzeroi64-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI64_be`

##### `impl Debug for NonZeroI64_be`

- <span id="nonzeroi64-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI64_be`

- <span id="nonzeroi64-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI64_be`

##### `impl Hash for NonZeroI64_be`

- <span id="nonzeroi64-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI64_be`

- <span id="nonzeroi64-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI64_be`

- <span id="nonzeroi64-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI64_be`

- <span id="nonzeroi64-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI64_be`

- <span id="nonzeroi64-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI64_be`

- <span id="nonzeroi64-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI64_be`

- <span id="nonzeroi64-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI64_be`

- <span id="nonzeroi64-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI64_be`

- <span id="nonzeroi64-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI128_le`

```rust
struct NonZeroI128_le(core::num::NonZeroI128);
```

A little-endian `NonZeroI128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="nonzeroi128-le-new"></span>`const fn new(value: i128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi128-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: i128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi128-le-get"></span>`const fn get(self) -> i128`

  Returns the value as a primitive type.

- <span id="nonzeroi128-le-from-native"></span>`const fn from_native(value: NonZeroI128) -> Self`

  Returns a `NonZeroI128_le` containing `value`.

- <span id="nonzeroi128-le-to-native"></span>`const fn to_native(self) -> NonZeroI128`

  Returns a `NonZeroI128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI128_le`

- <span id="nonzeroi128-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI128_le`

- <span id="nonzeroi128-le-bitor-type-output"></span>`type Output = NonZero<i128>`

- <span id="nonzeroi128-le-bitor"></span>`fn bitor(self, other: NonZeroI128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI128_le`

- <span id="nonzeroi128-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI128)`

##### `impl<C> CheckBytes for NonZeroI128_le`

- <span id="nonzeroi128-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI128_le`

- <span id="nonzeroi128-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI128_le`

##### `impl Debug for NonZeroI128_le`

- <span id="nonzeroi128-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI128_le`

- <span id="nonzeroi128-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI128_le`

##### `impl Hash for NonZeroI128_le`

- <span id="nonzeroi128-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI128_le`

- <span id="nonzeroi128-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI128_le`

- <span id="nonzeroi128-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI128_le`

- <span id="nonzeroi128-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI128_le`

- <span id="nonzeroi128-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI128_le`

- <span id="nonzeroi128-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI128_le`

- <span id="nonzeroi128-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI128_le`

- <span id="nonzeroi128-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI128_le`

- <span id="nonzeroi128-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI128_be`

```rust
struct NonZeroI128_be(core::num::NonZeroI128);
```

A big-endian `NonZeroI128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="nonzeroi128-be-new"></span>`const fn new(value: i128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi128-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: i128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi128-be-get"></span>`const fn get(self) -> i128`

  Returns the value as a primitive type.

- <span id="nonzeroi128-be-from-native"></span>`const fn from_native(value: NonZeroI128) -> Self`

  Returns a `NonZeroI128_be` containing `value`.

- <span id="nonzeroi128-be-to-native"></span>`const fn to_native(self) -> NonZeroI128`

  Returns a `NonZeroI128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI128_be`

- <span id="nonzeroi128-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI128_be`

- <span id="nonzeroi128-be-bitor-type-output"></span>`type Output = NonZero<i128>`

- <span id="nonzeroi128-be-bitor"></span>`fn bitor(self, other: NonZeroI128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI128_be`

- <span id="nonzeroi128-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI128)`

##### `impl<C> CheckBytes for NonZeroI128_be`

- <span id="nonzeroi128-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI128_be`

- <span id="nonzeroi128-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI128_be`

##### `impl Debug for NonZeroI128_be`

- <span id="nonzeroi128-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI128_be`

- <span id="nonzeroi128-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI128_be`

##### `impl Hash for NonZeroI128_be`

- <span id="nonzeroi128-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI128_be`

- <span id="nonzeroi128-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI128_be`

- <span id="nonzeroi128-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI128_be`

- <span id="nonzeroi128-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI128_be`

- <span id="nonzeroi128-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI128_be`

- <span id="nonzeroi128-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI128_be`

- <span id="nonzeroi128-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI128_be`

- <span id="nonzeroi128-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI128_be`

- <span id="nonzeroi128-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU16_le`

```rust
struct NonZeroU16_le(core::num::NonZeroU16);
```

A little-endian `NonZeroU16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="nonzerou16-le-new"></span>`const fn new(value: u16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou16-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: u16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou16-le-get"></span>`const fn get(self) -> u16`

  Returns the value as a primitive type.

- <span id="nonzerou16-le-from-native"></span>`const fn from_native(value: NonZeroU16) -> Self`

  Returns a `NonZeroU16_le` containing `value`.

- <span id="nonzerou16-le-to-native"></span>`const fn to_native(self) -> NonZeroU16`

  Returns a `NonZeroU16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU16_le`

- <span id="nonzerou16-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU16_le`

- <span id="nonzerou16-le-bitor-type-output"></span>`type Output = NonZero<u16>`

- <span id="nonzerou16-le-bitor"></span>`fn bitor(self, other: NonZeroU16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU16_le`

- <span id="nonzerou16-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU16)`

##### `impl<C> CheckBytes for NonZeroU16_le`

- <span id="nonzerou16-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU16_le`

- <span id="nonzerou16-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU16_le`

##### `impl Debug for NonZeroU16_le`

- <span id="nonzerou16-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU16_le`

- <span id="nonzerou16-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU16_le`

##### `impl Hash for NonZeroU16_le`

- <span id="nonzerou16-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU16_le`

- <span id="nonzerou16-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU16_le`

- <span id="nonzerou16-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU16_le`

- <span id="nonzerou16-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU16_le`

- <span id="nonzerou16-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU16_le`

- <span id="nonzerou16-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU16_le`

- <span id="nonzerou16-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU16_le`

- <span id="nonzerou16-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU16_le`

- <span id="nonzerou16-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU16_be`

```rust
struct NonZeroU16_be(core::num::NonZeroU16);
```

A big-endian `NonZeroU16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="nonzerou16-be-new"></span>`const fn new(value: u16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou16-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: u16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou16-be-get"></span>`const fn get(self) -> u16`

  Returns the value as a primitive type.

- <span id="nonzerou16-be-from-native"></span>`const fn from_native(value: NonZeroU16) -> Self`

  Returns a `NonZeroU16_be` containing `value`.

- <span id="nonzerou16-be-to-native"></span>`const fn to_native(self) -> NonZeroU16`

  Returns a `NonZeroU16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU16_be`

- <span id="nonzerou16-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU16_be`

- <span id="nonzerou16-be-bitor-type-output"></span>`type Output = NonZero<u16>`

- <span id="nonzerou16-be-bitor"></span>`fn bitor(self, other: NonZeroU16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU16_be`

- <span id="nonzerou16-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU16)`

##### `impl<C> CheckBytes for NonZeroU16_be`

- <span id="nonzerou16-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU16_be`

- <span id="nonzerou16-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU16_be`

##### `impl Debug for NonZeroU16_be`

- <span id="nonzerou16-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU16_be`

- <span id="nonzerou16-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU16_be`

##### `impl Hash for NonZeroU16_be`

- <span id="nonzerou16-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU16_be`

- <span id="nonzerou16-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU16_be`

- <span id="nonzerou16-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU16_be`

- <span id="nonzerou16-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU16_be`

- <span id="nonzerou16-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU16_be`

- <span id="nonzerou16-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU16_be`

- <span id="nonzerou16-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU16_be`

- <span id="nonzerou16-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU16_be`

- <span id="nonzerou16-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU32_le`

```rust
struct NonZeroU32_le(core::num::NonZeroU32);
```

A little-endian `NonZeroU32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="nonzerou32-le-new"></span>`const fn new(value: u32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou32-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: u32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou32-le-get"></span>`const fn get(self) -> u32`

  Returns the value as a primitive type.

- <span id="nonzerou32-le-from-native"></span>`const fn from_native(value: NonZeroU32) -> Self`

  Returns a `NonZeroU32_le` containing `value`.

- <span id="nonzerou32-le-to-native"></span>`const fn to_native(self) -> NonZeroU32`

  Returns a `NonZeroU32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU32_le`

- <span id="nonzerou32-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU32_le`

- <span id="nonzerou32-le-bitor-type-output"></span>`type Output = NonZero<u32>`

- <span id="nonzerou32-le-bitor"></span>`fn bitor(self, other: NonZeroU32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU32_le`

- <span id="nonzerou32-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU32)`

##### `impl<C> CheckBytes for NonZeroU32_le`

- <span id="nonzerou32-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU32_le`

- <span id="nonzerou32-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU32_le`

##### `impl Debug for NonZeroU32_le`

- <span id="nonzerou32-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU32_le`

- <span id="nonzerou32-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU32_le`

##### `impl Hash for NonZeroU32_le`

- <span id="nonzerou32-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU32_le`

- <span id="nonzerou32-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU32_le`

- <span id="nonzerou32-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU32_le`

- <span id="nonzerou32-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU32_le`

- <span id="nonzerou32-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU32_le`

- <span id="nonzerou32-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU32_le`

- <span id="nonzerou32-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU32_le`

- <span id="nonzerou32-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU32_le`

- <span id="nonzerou32-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU32_be`

```rust
struct NonZeroU32_be(core::num::NonZeroU32);
```

A big-endian `NonZeroU32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="nonzerou32-be-new"></span>`const fn new(value: u32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou32-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: u32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou32-be-get"></span>`const fn get(self) -> u32`

  Returns the value as a primitive type.

- <span id="nonzerou32-be-from-native"></span>`const fn from_native(value: NonZeroU32) -> Self`

  Returns a `NonZeroU32_be` containing `value`.

- <span id="nonzerou32-be-to-native"></span>`const fn to_native(self) -> NonZeroU32`

  Returns a `NonZeroU32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU32_be`

- <span id="nonzerou32-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU32_be`

- <span id="nonzerou32-be-bitor-type-output"></span>`type Output = NonZero<u32>`

- <span id="nonzerou32-be-bitor"></span>`fn bitor(self, other: NonZeroU32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU32_be`

- <span id="nonzerou32-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU32)`

##### `impl<C> CheckBytes for NonZeroU32_be`

- <span id="nonzerou32-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU32_be`

- <span id="nonzerou32-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU32_be`

##### `impl Debug for NonZeroU32_be`

- <span id="nonzerou32-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU32_be`

- <span id="nonzerou32-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU32_be`

##### `impl Hash for NonZeroU32_be`

- <span id="nonzerou32-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU32_be`

- <span id="nonzerou32-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU32_be`

- <span id="nonzerou32-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU32_be`

- <span id="nonzerou32-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU32_be`

- <span id="nonzerou32-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU32_be`

- <span id="nonzerou32-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU32_be`

- <span id="nonzerou32-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU32_be`

- <span id="nonzerou32-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU32_be`

- <span id="nonzerou32-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU64_le`

```rust
struct NonZeroU64_le(core::num::NonZeroU64);
```

A little-endian `NonZeroU64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="nonzerou64-le-new"></span>`const fn new(value: u64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou64-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: u64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou64-le-get"></span>`const fn get(self) -> u64`

  Returns the value as a primitive type.

- <span id="nonzerou64-le-from-native"></span>`const fn from_native(value: NonZeroU64) -> Self`

  Returns a `NonZeroU64_le` containing `value`.

- <span id="nonzerou64-le-to-native"></span>`const fn to_native(self) -> NonZeroU64`

  Returns a `NonZeroU64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU64_le`

- <span id="nonzerou64-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU64_le`

- <span id="nonzerou64-le-bitor-type-output"></span>`type Output = NonZero<u64>`

- <span id="nonzerou64-le-bitor"></span>`fn bitor(self, other: NonZeroU64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU64_le`

- <span id="nonzerou64-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU64)`

##### `impl<C> CheckBytes for NonZeroU64_le`

- <span id="nonzerou64-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU64_le`

- <span id="nonzerou64-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU64_le`

##### `impl Debug for NonZeroU64_le`

- <span id="nonzerou64-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU64_le`

- <span id="nonzerou64-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU64_le`

##### `impl Hash for NonZeroU64_le`

- <span id="nonzerou64-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU64_le`

- <span id="nonzerou64-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU64_le`

- <span id="nonzerou64-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU64_le`

- <span id="nonzerou64-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU64_le`

- <span id="nonzerou64-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU64_le`

- <span id="nonzerou64-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU64_le`

- <span id="nonzerou64-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU64_le`

- <span id="nonzerou64-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU64_le`

- <span id="nonzerou64-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU64_be`

```rust
struct NonZeroU64_be(core::num::NonZeroU64);
```

A big-endian `NonZeroU64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="nonzerou64-be-new"></span>`const fn new(value: u64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou64-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: u64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou64-be-get"></span>`const fn get(self) -> u64`

  Returns the value as a primitive type.

- <span id="nonzerou64-be-from-native"></span>`const fn from_native(value: NonZeroU64) -> Self`

  Returns a `NonZeroU64_be` containing `value`.

- <span id="nonzerou64-be-to-native"></span>`const fn to_native(self) -> NonZeroU64`

  Returns a `NonZeroU64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU64_be`

- <span id="nonzerou64-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU64_be`

- <span id="nonzerou64-be-bitor-type-output"></span>`type Output = NonZero<u64>`

- <span id="nonzerou64-be-bitor"></span>`fn bitor(self, other: NonZeroU64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU64_be`

- <span id="nonzerou64-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU64)`

##### `impl<C> CheckBytes for NonZeroU64_be`

- <span id="nonzerou64-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU64_be`

- <span id="nonzerou64-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU64_be`

##### `impl Debug for NonZeroU64_be`

- <span id="nonzerou64-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU64_be`

- <span id="nonzerou64-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU64_be`

##### `impl Hash for NonZeroU64_be`

- <span id="nonzerou64-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU64_be`

- <span id="nonzerou64-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU64_be`

- <span id="nonzerou64-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU64_be`

- <span id="nonzerou64-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU64_be`

- <span id="nonzerou64-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU64_be`

- <span id="nonzerou64-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU64_be`

- <span id="nonzerou64-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU64_be`

- <span id="nonzerou64-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU64_be`

- <span id="nonzerou64-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU128_le`

```rust
struct NonZeroU128_le(core::num::NonZeroU128);
```

A little-endian `NonZeroU128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="nonzerou128-le-new"></span>`const fn new(value: u128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou128-le-new-unchecked"></span>`const unsafe fn new_unchecked(value: u128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou128-le-get"></span>`const fn get(self) -> u128`

  Returns the value as a primitive type.

- <span id="nonzerou128-le-from-native"></span>`const fn from_native(value: NonZeroU128) -> Self`

  Returns a `NonZeroU128_le` containing `value`.

- <span id="nonzerou128-le-to-native"></span>`const fn to_native(self) -> NonZeroU128`

  Returns a `NonZeroU128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU128_le`

- <span id="nonzerou128-le-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU128_le`

- <span id="nonzerou128-le-bitor-type-output"></span>`type Output = NonZero<u128>`

- <span id="nonzerou128-le-bitor"></span>`fn bitor(self, other: NonZeroU128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU128_le`

- <span id="nonzerou128-le-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU128)`

##### `impl<C> CheckBytes for NonZeroU128_le`

- <span id="nonzerou128-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU128_le`

- <span id="nonzerou128-le-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU128_le`

##### `impl Debug for NonZeroU128_le`

- <span id="nonzerou128-le-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU128_le`

- <span id="nonzerou128-le-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU128_le`

##### `impl Hash for NonZeroU128_le`

- <span id="nonzerou128-le-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU128_le`

- <span id="nonzerou128-le-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU128_le`

- <span id="nonzerou128-le-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU128_le`

- <span id="nonzerou128-le-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU128_le`

- <span id="nonzerou128-le-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU128_le`

- <span id="nonzerou128-le-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU128_le`

- <span id="nonzerou128-le-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU128_le`

- <span id="nonzerou128-le-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU128_le`

- <span id="nonzerou128-le-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU128_be`

```rust
struct NonZeroU128_be(core::num::NonZeroU128);
```

A big-endian `NonZeroU128` with a guaranteed size and alignment of `16`.

#### Implementations

- <span id="nonzerou128-be-new"></span>`const fn new(value: u128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou128-be-new-unchecked"></span>`const unsafe fn new_unchecked(value: u128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou128-be-get"></span>`const fn get(self) -> u128`

  Returns the value as a primitive type.

- <span id="nonzerou128-be-from-native"></span>`const fn from_native(value: NonZeroU128) -> Self`

  Returns a `NonZeroU128_be` containing `value`.

- <span id="nonzerou128-be-to-native"></span>`const fn to_native(self) -> NonZeroU128`

  Returns a `NonZeroU128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU128_be`

- <span id="nonzerou128-be-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU128_be`

- <span id="nonzerou128-be-bitor-type-output"></span>`type Output = NonZero<u128>`

- <span id="nonzerou128-be-bitor"></span>`fn bitor(self, other: NonZeroU128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU128_be`

- <span id="nonzerou128-be-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU128)`

##### `impl<C> CheckBytes for NonZeroU128_be`

- <span id="nonzerou128-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU128_be`

- <span id="nonzerou128-be-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU128_be`

##### `impl Debug for NonZeroU128_be`

- <span id="nonzerou128-be-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU128_be`

- <span id="nonzerou128-be-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU128_be`

##### `impl Hash for NonZeroU128_be`

- <span id="nonzerou128-be-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU128_be`

- <span id="nonzerou128-be-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU128_be`

- <span id="nonzerou128-be-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU128_be`

- <span id="nonzerou128-be-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU128_be`

- <span id="nonzerou128-be-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU128_be`

- <span id="nonzerou128-be-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU128_be`

- <span id="nonzerou128-be-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU128_be`

- <span id="nonzerou128-be-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU128_be`

- <span id="nonzerou128-be-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `AtomicI16_le`

```rust
struct AtomicI16_le(core::sync::atomic::AtomicI16);
```

A little-endian `AtomicI16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="atomici16-le-new"></span>`const fn new(value: i16) -> Self`

  Returns a `AtomicI16_le` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicI16_le`

- <span id="atomici16-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicI16_le`

- <span id="atomici16-le-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicI16_le`

- <span id="atomici16-le-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicI16_le`

- <span id="atomici16-le-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicI16_be`

```rust
struct AtomicI16_be(core::sync::atomic::AtomicI16);
```

A big-endian `AtomicI16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="atomici16-be-new"></span>`const fn new(value: i16) -> Self`

  Returns a `AtomicI16_be` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicI16_be`

- <span id="atomici16-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicI16_be`

- <span id="atomici16-be-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicI16_be`

- <span id="atomici16-be-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicI16_be`

- <span id="atomici16-be-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicU16_le`

```rust
struct AtomicU16_le(core::sync::atomic::AtomicU16);
```

A little-endian `AtomicU16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="atomicu16-le-new"></span>`const fn new(value: u16) -> Self`

  Returns a `AtomicU16_le` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicU16_le`

- <span id="atomicu16-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicU16_le`

- <span id="atomicu16-le-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicU16_le`

- <span id="atomicu16-le-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicU16_le`

- <span id="atomicu16-le-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicU16_be`

```rust
struct AtomicU16_be(core::sync::atomic::AtomicU16);
```

A big-endian `AtomicU16` with a guaranteed size and alignment of `2`.

#### Implementations

- <span id="atomicu16-be-new"></span>`const fn new(value: u16) -> Self`

  Returns a `AtomicU16_be` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicU16_be`

- <span id="atomicu16-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicU16_be`

- <span id="atomicu16-be-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicU16_be`

- <span id="atomicu16-be-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicU16_be`

- <span id="atomicu16-be-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicI32_le`

```rust
struct AtomicI32_le(core::sync::atomic::AtomicI32);
```

A little-endian `AtomicI32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="atomici32-le-new"></span>`const fn new(value: i32) -> Self`

  Returns a `AtomicI32_le` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicI32_le`

- <span id="atomici32-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicI32_le`

- <span id="atomici32-le-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicI32_le`

- <span id="atomici32-le-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicI32_le`

- <span id="atomici32-le-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicI32_be`

```rust
struct AtomicI32_be(core::sync::atomic::AtomicI32);
```

A big-endian `AtomicI32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="atomici32-be-new"></span>`const fn new(value: i32) -> Self`

  Returns a `AtomicI32_be` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicI32_be`

- <span id="atomici32-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicI32_be`

- <span id="atomici32-be-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicI32_be`

- <span id="atomici32-be-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicI32_be`

- <span id="atomici32-be-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicU32_le`

```rust
struct AtomicU32_le(core::sync::atomic::AtomicU32);
```

A little-endian `AtomicU32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="atomicu32-le-new"></span>`const fn new(value: u32) -> Self`

  Returns a `AtomicU32_le` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicU32_le`

- <span id="atomicu32-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicU32_le`

- <span id="atomicu32-le-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicU32_le`

- <span id="atomicu32-le-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicU32_le`

- <span id="atomicu32-le-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicU32_be`

```rust
struct AtomicU32_be(core::sync::atomic::AtomicU32);
```

A big-endian `AtomicU32` with a guaranteed size and alignment of `4`.

#### Implementations

- <span id="atomicu32-be-new"></span>`const fn new(value: u32) -> Self`

  Returns a `AtomicU32_be` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicU32_be`

- <span id="atomicu32-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicU32_be`

- <span id="atomicu32-be-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicU32_be`

- <span id="atomicu32-be-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicU32_be`

- <span id="atomicu32-be-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicI64_le`

```rust
struct AtomicI64_le(core::sync::atomic::AtomicI64);
```

A little-endian `AtomicI64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="atomici64-le-new"></span>`const fn new(value: i64) -> Self`

  Returns a `AtomicI64_le` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicI64_le`

- <span id="atomici64-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicI64_le`

- <span id="atomici64-le-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicI64_le`

- <span id="atomici64-le-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicI64_le`

- <span id="atomici64-le-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicI64_be`

```rust
struct AtomicI64_be(core::sync::atomic::AtomicI64);
```

A big-endian `AtomicI64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="atomici64-be-new"></span>`const fn new(value: i64) -> Self`

  Returns a `AtomicI64_be` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicI64_be`

- <span id="atomici64-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicI64_be`

- <span id="atomici64-be-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicI64_be`

- <span id="atomici64-be-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicI64_be`

- <span id="atomici64-be-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicU64_le`

```rust
struct AtomicU64_le(core::sync::atomic::AtomicU64);
```

A little-endian `AtomicU64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="atomicu64-le-new"></span>`const fn new(value: u64) -> Self`

  Returns a `AtomicU64_le` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicU64_le`

- <span id="atomicu64-le-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicU64_le`

- <span id="atomicu64-le-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicU64_le`

- <span id="atomicu64-le-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicU64_le`

- <span id="atomicu64-le-pointee-type-metadata"></span>`type Metadata = ()`

### `AtomicU64_be`

```rust
struct AtomicU64_be(core::sync::atomic::AtomicU64);
```

A big-endian `AtomicU64` with a guaranteed size and alignment of `8`.

#### Implementations

- <span id="atomicu64-be-new"></span>`const fn new(value: u64) -> Self`

  Returns a `AtomicU64_be` containing `value`.

#### Trait Implementations

##### `impl<C> CheckBytes for AtomicU64_be`

- <span id="atomicu64-be-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Debug for AtomicU64_be`

- <span id="atomicu64-be-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for AtomicU64_be`

- <span id="atomicu64-be-default"></span>`fn default() -> Self`

##### `impl Pointee for AtomicU64_be`

- <span id="atomicu64-be-pointee-type-metadata"></span>`type Metadata = ()`

## Functions

### `fetch_ordering`

```rust
const fn fetch_ordering(order: core::sync::atomic::Ordering) -> core::sync::atomic::Ordering
```

## Macros

### `define_newtype!`

### `define_signed_integer!`

### `define_signed_integers!`

### `define_unsigned_integer!`

### `define_unsigned_integers!`

### `define_float!`

### `define_floats!`

### `define_char!`

### `define_nonzero!`

### `define_nonzeros!`

### `define_atomic!`

### `define_atomics!`

