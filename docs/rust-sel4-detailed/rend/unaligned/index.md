*[rend](../index.md) / [unaligned](index.md)*

---

# Module `unaligned`

Cross-platform primitives with unaligned representations.

## Contents

- [Structs](#structs)
  - [`i16_ule`](#i16-ule)
  - [`i16_ube`](#i16-ube)
  - [`i32_ule`](#i32-ule)
  - [`i32_ube`](#i32-ube)
  - [`i64_ule`](#i64-ule)
  - [`i64_ube`](#i64-ube)
  - [`i128_ule`](#i128-ule)
  - [`i128_ube`](#i128-ube)
  - [`u16_ule`](#u16-ule)
  - [`u16_ube`](#u16-ube)
  - [`u32_ule`](#u32-ule)
  - [`u32_ube`](#u32-ube)
  - [`u64_ule`](#u64-ule)
  - [`u64_ube`](#u64-ube)
  - [`u128_ule`](#u128-ule)
  - [`u128_ube`](#u128-ube)
  - [`f32_ule`](#f32-ule)
  - [`f32_ube`](#f32-ube)
  - [`f64_ule`](#f64-ule)
  - [`f64_ube`](#f64-ube)
  - [`char_ule`](#char-ule)
  - [`char_ube`](#char-ube)
  - [`NonZeroI16_ule`](#nonzeroi16-ule)
  - [`NonZeroI16_ube`](#nonzeroi16-ube)
  - [`NonZeroI32_ule`](#nonzeroi32-ule)
  - [`NonZeroI32_ube`](#nonzeroi32-ube)
  - [`NonZeroI64_ule`](#nonzeroi64-ule)
  - [`NonZeroI64_ube`](#nonzeroi64-ube)
  - [`NonZeroI128_ule`](#nonzeroi128-ule)
  - [`NonZeroI128_ube`](#nonzeroi128-ube)
  - [`NonZeroU16_ule`](#nonzerou16-ule)
  - [`NonZeroU16_ube`](#nonzerou16-ube)
  - [`NonZeroU32_ule`](#nonzerou32-ule)
  - [`NonZeroU32_ube`](#nonzerou32-ube)
  - [`NonZeroU64_ule`](#nonzerou64-ule)
  - [`NonZeroU64_ube`](#nonzerou64-ube)
  - [`NonZeroU128_ule`](#nonzerou128-ule)
  - [`NonZeroU128_ube`](#nonzerou128-ube)
- [Macros](#macros)
  - [`define_unaligned_newtype!`](#define-unaligned-newtype)
  - [`define_unaligned_signed_integer!`](#define-unaligned-signed-integer)
  - [`define_unaligned_signed_integers!`](#define-unaligned-signed-integers)
  - [`define_unaligned_unsigned_integer!`](#define-unaligned-unsigned-integer)
  - [`define_unaligned_unsigned_integers!`](#define-unaligned-unsigned-integers)
  - [`define_unaligned_float!`](#define-unaligned-float)
  - [`define_unaligned_floats!`](#define-unaligned-floats)
  - [`define_unaligned_char!`](#define-unaligned-char)
  - [`define_unaligned_nonzero!`](#define-unaligned-nonzero)
  - [`define_unaligned_nonzeros!`](#define-unaligned-nonzeros)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`i16_ule`](#i16-ule) | struct | A little-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`. |
| [`i16_ube`](#i16-ube) | struct | A big-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`. |
| [`i32_ule`](#i32-ule) | struct | A little-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`. |
| [`i32_ube`](#i32-ube) | struct | A big-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`. |
| [`i64_ule`](#i64-ule) | struct | A little-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`. |
| [`i64_ube`](#i64-ube) | struct | A big-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`. |
| [`i128_ule`](#i128-ule) | struct | A little-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`. |
| [`i128_ube`](#i128-ube) | struct | A big-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`. |
| [`u16_ule`](#u16-ule) | struct | A little-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`. |
| [`u16_ube`](#u16-ube) | struct | A big-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`. |
| [`u32_ule`](#u32-ule) | struct | A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`. |
| [`u32_ube`](#u32-ube) | struct | A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`. |
| [`u64_ule`](#u64-ule) | struct | A little-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`. |
| [`u64_ube`](#u64-ube) | struct | A big-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`. |
| [`u128_ule`](#u128-ule) | struct | A little-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`. |
| [`u128_ube`](#u128-ube) | struct | A big-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`. |
| [`f32_ule`](#f32-ule) | struct | A little-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`. |
| [`f32_ube`](#f32-ube) | struct | A big-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`. |
| [`f64_ule`](#f64-ule) | struct | A little-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`. |
| [`f64_ube`](#f64-ube) | struct | A big-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`. |
| [`char_ule`](#char-ule) | struct | A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`. |
| [`char_ube`](#char-ube) | struct | A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`. |
| [`NonZeroI16_ule`](#nonzeroi16-ule) | struct | A little-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`. |
| [`NonZeroI16_ube`](#nonzeroi16-ube) | struct | A big-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`. |
| [`NonZeroI32_ule`](#nonzeroi32-ule) | struct | A little-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`. |
| [`NonZeroI32_ube`](#nonzeroi32-ube) | struct | A big-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`. |
| [`NonZeroI64_ule`](#nonzeroi64-ule) | struct | A little-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`. |
| [`NonZeroI64_ube`](#nonzeroi64-ube) | struct | A big-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`. |
| [`NonZeroI128_ule`](#nonzeroi128-ule) | struct | A little-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`. |
| [`NonZeroI128_ube`](#nonzeroi128-ube) | struct | A big-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`. |
| [`NonZeroU16_ule`](#nonzerou16-ule) | struct | A little-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`. |
| [`NonZeroU16_ube`](#nonzerou16-ube) | struct | A big-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`. |
| [`NonZeroU32_ule`](#nonzerou32-ule) | struct | A little-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`. |
| [`NonZeroU32_ube`](#nonzerou32-ube) | struct | A big-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`. |
| [`NonZeroU64_ule`](#nonzerou64-ule) | struct | A little-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`. |
| [`NonZeroU64_ube`](#nonzerou64-ube) | struct | A big-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`. |
| [`NonZeroU128_ule`](#nonzerou128-ule) | struct | A little-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`. |
| [`NonZeroU128_ube`](#nonzerou128-ube) | struct | A big-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`. |
| [`define_unaligned_newtype!`](#define-unaligned-newtype) | macro |  |
| [`define_unaligned_signed_integer!`](#define-unaligned-signed-integer) | macro |  |
| [`define_unaligned_signed_integers!`](#define-unaligned-signed-integers) | macro |  |
| [`define_unaligned_unsigned_integer!`](#define-unaligned-unsigned-integer) | macro |  |
| [`define_unaligned_unsigned_integers!`](#define-unaligned-unsigned-integers) | macro |  |
| [`define_unaligned_float!`](#define-unaligned-float) | macro |  |
| [`define_unaligned_floats!`](#define-unaligned-floats) | macro |  |
| [`define_unaligned_char!`](#define-unaligned-char) | macro |  |
| [`define_unaligned_nonzero!`](#define-unaligned-nonzero) | macro |  |
| [`define_unaligned_nonzeros!`](#define-unaligned-nonzeros) | macro |  |

## Structs

### `i16_ule`

```rust
struct i16_ule(i16);
```

A little-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="i16-ule-from-native"></span>`const fn from_native(value: i16) -> Self`

  Returns a `i16_ule` containing `value`.

- <span id="i16-ule-to-native"></span>`const fn to_native(self) -> i16`

  Returns a `i16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i16_ule`

- <span id="i16-ule-add-type-output"></span>`type Output = i16`

- <span id="i16-ule-add"></span>`fn add(self, other: i16) -> <Self as >::Output`

##### `impl AddAssign for i16_ule`

- <span id="i16-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: i16)`

##### `impl Binary for i16_ule`

- <span id="i16-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i16_ule`

- <span id="i16-ule-bitand-type-output"></span>`type Output = i16`

- <span id="i16-ule-bitand"></span>`fn bitand(self, other: i16) -> <Self as >::Output`

##### `impl BitAndAssign for i16_ule`

- <span id="i16-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i16)`

##### `impl BitOr for i16_ule`

- <span id="i16-ule-bitor-type-output"></span>`type Output = i16`

- <span id="i16-ule-bitor"></span>`fn bitor(self, other: i16) -> <Self as >::Output`

##### `impl BitOrAssign for i16_ule`

- <span id="i16-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i16)`

##### `impl BitXor for i16_ule`

- <span id="i16-ule-bitxor-type-output"></span>`type Output = i16`

- <span id="i16-ule-bitxor"></span>`fn bitxor(self, other: i16) -> <Self as >::Output`

##### `impl BitXorAssign for i16_ule`

- <span id="i16-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i16)`

##### `impl<C> CheckBytes for i16_ule`

- <span id="i16-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i16_ule`

- <span id="i16-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i16_ule`

##### `impl Debug for i16_ule`

- <span id="i16-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i16_ule`

- <span id="i16-ule-default"></span>`fn default() -> Self`

##### `impl Display for i16_ule`

- <span id="i16-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i16_ule`

- <span id="i16-ule-div-type-output"></span>`type Output = i16`

- <span id="i16-ule-div"></span>`fn div(self, other: i16) -> <Self as >::Output`

##### `impl DivAssign for i16_ule`

- <span id="i16-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: i16)`

##### `impl Eq for i16_ule`

##### `impl Hash for i16_ule`

- <span id="i16-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i16_ule`

- <span id="i16-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i16_ule`

- <span id="i16-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i16_ule`

- <span id="i16-ule-mul-type-output"></span>`type Output = i16`

- <span id="i16-ule-mul"></span>`fn mul(self, other: i16) -> <Self as >::Output`

##### `impl MulAssign for i16_ule`

- <span id="i16-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i16)`

##### `impl Neg for i16_ule`

- <span id="i16-ule-neg-type-output"></span>`type Output = <i16 as Neg>::Output`

- <span id="i16-ule-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i16_ule`

- <span id="i16-ule-not-type-output"></span>`type Output = <i16 as Not>::Output`

- <span id="i16-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i16_ule`

- <span id="i16-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i16_ule`

- <span id="i16-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i16_ule`

- <span id="i16-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i16_ule`

- <span id="i16-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i16_ule`

- <span id="i16-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i16_ule`

- <span id="i16-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i16_ule`

- <span id="i16-ule-rem-type-output"></span>`type Output = i16`

- <span id="i16-ule-rem"></span>`fn rem(self, other: i16) -> <Self as >::Output`

##### `impl RemAssign for i16_ule`

- <span id="i16-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i16)`

##### `impl Shl for i16_ule`

- <span id="i16-ule-shl-type-output"></span>`type Output = i16`

- <span id="i16-ule-shl"></span>`fn shl(self, other: i16) -> <Self as >::Output`

##### `impl ShlAssign for i16_ule`

- <span id="i16-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i16)`

##### `impl Shr for i16_ule`

- <span id="i16-ule-shr-type-output"></span>`type Output = i16`

- <span id="i16-ule-shr"></span>`fn shr(self, other: i16) -> <Self as >::Output`

##### `impl ShrAssign for i16_ule`

- <span id="i16-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i16)`

##### `impl Sub for i16_ule`

- <span id="i16-ule-sub-type-output"></span>`type Output = i16`

- <span id="i16-ule-sub"></span>`fn sub(self, other: i16) -> <Self as >::Output`

##### `impl SubAssign for i16_ule`

- <span id="i16-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i16)`

##### `impl Sum for i16_ule`

- <span id="i16-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i16_ule`

- <span id="i16-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i16_ule`

- <span id="i16-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i16_ule`

- <span id="i16-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i16_ube`

```rust
struct i16_ube(i16);
```

A big-endian unaligned `i16` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="i16-ube-from-native"></span>`const fn from_native(value: i16) -> Self`

  Returns a `i16_ube` containing `value`.

- <span id="i16-ube-to-native"></span>`const fn to_native(self) -> i16`

  Returns a `i16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i16_ube`

- <span id="i16-ube-add-type-output"></span>`type Output = i16`

- <span id="i16-ube-add"></span>`fn add(self, other: i16) -> <Self as >::Output`

##### `impl AddAssign for i16_ube`

- <span id="i16-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: i16)`

##### `impl Binary for i16_ube`

- <span id="i16-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i16_ube`

- <span id="i16-ube-bitand-type-output"></span>`type Output = i16`

- <span id="i16-ube-bitand"></span>`fn bitand(self, other: i16) -> <Self as >::Output`

##### `impl BitAndAssign for i16_ube`

- <span id="i16-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i16)`

##### `impl BitOr for i16_ube`

- <span id="i16-ube-bitor-type-output"></span>`type Output = i16`

- <span id="i16-ube-bitor"></span>`fn bitor(self, other: i16) -> <Self as >::Output`

##### `impl BitOrAssign for i16_ube`

- <span id="i16-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i16)`

##### `impl BitXor for i16_ube`

- <span id="i16-ube-bitxor-type-output"></span>`type Output = i16`

- <span id="i16-ube-bitxor"></span>`fn bitxor(self, other: i16) -> <Self as >::Output`

##### `impl BitXorAssign for i16_ube`

- <span id="i16-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i16)`

##### `impl<C> CheckBytes for i16_ube`

- <span id="i16-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i16_ube`

- <span id="i16-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i16_ube`

##### `impl Debug for i16_ube`

- <span id="i16-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i16_ube`

- <span id="i16-ube-default"></span>`fn default() -> Self`

##### `impl Display for i16_ube`

- <span id="i16-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i16_ube`

- <span id="i16-ube-div-type-output"></span>`type Output = i16`

- <span id="i16-ube-div"></span>`fn div(self, other: i16) -> <Self as >::Output`

##### `impl DivAssign for i16_ube`

- <span id="i16-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: i16)`

##### `impl Eq for i16_ube`

##### `impl Hash for i16_ube`

- <span id="i16-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i16_ube`

- <span id="i16-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i16_ube`

- <span id="i16-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i16_ube`

- <span id="i16-ube-mul-type-output"></span>`type Output = i16`

- <span id="i16-ube-mul"></span>`fn mul(self, other: i16) -> <Self as >::Output`

##### `impl MulAssign for i16_ube`

- <span id="i16-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i16)`

##### `impl Neg for i16_ube`

- <span id="i16-ube-neg-type-output"></span>`type Output = <i16 as Neg>::Output`

- <span id="i16-ube-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i16_ube`

- <span id="i16-ube-not-type-output"></span>`type Output = <i16 as Not>::Output`

- <span id="i16-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i16_ube`

- <span id="i16-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i16_ube`

- <span id="i16-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i16_ube`

- <span id="i16-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i16_ube`

- <span id="i16-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i16_ube`

- <span id="i16-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i16_ube`

- <span id="i16-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i16_ube`

- <span id="i16-ube-rem-type-output"></span>`type Output = i16`

- <span id="i16-ube-rem"></span>`fn rem(self, other: i16) -> <Self as >::Output`

##### `impl RemAssign for i16_ube`

- <span id="i16-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i16)`

##### `impl Shl for i16_ube`

- <span id="i16-ube-shl-type-output"></span>`type Output = i16`

- <span id="i16-ube-shl"></span>`fn shl(self, other: i16) -> <Self as >::Output`

##### `impl ShlAssign for i16_ube`

- <span id="i16-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i16)`

##### `impl Shr for i16_ube`

- <span id="i16-ube-shr-type-output"></span>`type Output = i16`

- <span id="i16-ube-shr"></span>`fn shr(self, other: i16) -> <Self as >::Output`

##### `impl ShrAssign for i16_ube`

- <span id="i16-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i16)`

##### `impl Sub for i16_ube`

- <span id="i16-ube-sub-type-output"></span>`type Output = i16`

- <span id="i16-ube-sub"></span>`fn sub(self, other: i16) -> <Self as >::Output`

##### `impl SubAssign for i16_ube`

- <span id="i16-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i16)`

##### `impl Sum for i16_ube`

- <span id="i16-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i16_ube`

- <span id="i16-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i16_ube`

- <span id="i16-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i16_ube`

- <span id="i16-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i32_ule`

```rust
struct i32_ule(i32);
```

A little-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="i32-ule-from-native"></span>`const fn from_native(value: i32) -> Self`

  Returns a `i32_ule` containing `value`.

- <span id="i32-ule-to-native"></span>`const fn to_native(self) -> i32`

  Returns a `i32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i32_ule`

- <span id="i32-ule-add-type-output"></span>`type Output = i32`

- <span id="i32-ule-add"></span>`fn add(self, other: i32) -> <Self as >::Output`

##### `impl AddAssign for i32_ule`

- <span id="i32-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: i32)`

##### `impl Binary for i32_ule`

- <span id="i32-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i32_ule`

- <span id="i32-ule-bitand-type-output"></span>`type Output = i32`

- <span id="i32-ule-bitand"></span>`fn bitand(self, other: i32) -> <Self as >::Output`

##### `impl BitAndAssign for i32_ule`

- <span id="i32-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i32)`

##### `impl BitOr for i32_ule`

- <span id="i32-ule-bitor-type-output"></span>`type Output = i32`

- <span id="i32-ule-bitor"></span>`fn bitor(self, other: i32) -> <Self as >::Output`

##### `impl BitOrAssign for i32_ule`

- <span id="i32-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i32)`

##### `impl BitXor for i32_ule`

- <span id="i32-ule-bitxor-type-output"></span>`type Output = i32`

- <span id="i32-ule-bitxor"></span>`fn bitxor(self, other: i32) -> <Self as >::Output`

##### `impl BitXorAssign for i32_ule`

- <span id="i32-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i32)`

##### `impl<C> CheckBytes for i32_ule`

- <span id="i32-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i32_ule`

- <span id="i32-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i32_ule`

##### `impl Debug for i32_ule`

- <span id="i32-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i32_ule`

- <span id="i32-ule-default"></span>`fn default() -> Self`

##### `impl Display for i32_ule`

- <span id="i32-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i32_ule`

- <span id="i32-ule-div-type-output"></span>`type Output = i32`

- <span id="i32-ule-div"></span>`fn div(self, other: i32) -> <Self as >::Output`

##### `impl DivAssign for i32_ule`

- <span id="i32-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: i32)`

##### `impl Eq for i32_ule`

##### `impl Hash for i32_ule`

- <span id="i32-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i32_ule`

- <span id="i32-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i32_ule`

- <span id="i32-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i32_ule`

- <span id="i32-ule-mul-type-output"></span>`type Output = i32`

- <span id="i32-ule-mul"></span>`fn mul(self, other: i32) -> <Self as >::Output`

##### `impl MulAssign for i32_ule`

- <span id="i32-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i32)`

##### `impl Neg for i32_ule`

- <span id="i32-ule-neg-type-output"></span>`type Output = <i32 as Neg>::Output`

- <span id="i32-ule-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i32_ule`

- <span id="i32-ule-not-type-output"></span>`type Output = <i32 as Not>::Output`

- <span id="i32-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i32_ule`

- <span id="i32-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i32_ule`

- <span id="i32-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i32_ule`

- <span id="i32-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i32_ule`

- <span id="i32-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i32_ule`

- <span id="i32-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i32_ule`

- <span id="i32-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i32_ule`

- <span id="i32-ule-rem-type-output"></span>`type Output = i32`

- <span id="i32-ule-rem"></span>`fn rem(self, other: i32) -> <Self as >::Output`

##### `impl RemAssign for i32_ule`

- <span id="i32-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i32)`

##### `impl Shl for i32_ule`

- <span id="i32-ule-shl-type-output"></span>`type Output = i32`

- <span id="i32-ule-shl"></span>`fn shl(self, other: i32) -> <Self as >::Output`

##### `impl ShlAssign for i32_ule`

- <span id="i32-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i32)`

##### `impl Shr for i32_ule`

- <span id="i32-ule-shr-type-output"></span>`type Output = i32`

- <span id="i32-ule-shr"></span>`fn shr(self, other: i32) -> <Self as >::Output`

##### `impl ShrAssign for i32_ule`

- <span id="i32-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i32)`

##### `impl Sub for i32_ule`

- <span id="i32-ule-sub-type-output"></span>`type Output = i32`

- <span id="i32-ule-sub"></span>`fn sub(self, other: i32) -> <Self as >::Output`

##### `impl SubAssign for i32_ule`

- <span id="i32-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i32)`

##### `impl Sum for i32_ule`

- <span id="i32-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i32_ule`

- <span id="i32-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i32_ule`

- <span id="i32-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i32_ule`

- <span id="i32-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i32_ube`

```rust
struct i32_ube(i32);
```

A big-endian unaligned `i32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="i32-ube-from-native"></span>`const fn from_native(value: i32) -> Self`

  Returns a `i32_ube` containing `value`.

- <span id="i32-ube-to-native"></span>`const fn to_native(self) -> i32`

  Returns a `i32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i32_ube`

- <span id="i32-ube-add-type-output"></span>`type Output = i32`

- <span id="i32-ube-add"></span>`fn add(self, other: i32) -> <Self as >::Output`

##### `impl AddAssign for i32_ube`

- <span id="i32-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: i32)`

##### `impl Binary for i32_ube`

- <span id="i32-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i32_ube`

- <span id="i32-ube-bitand-type-output"></span>`type Output = i32`

- <span id="i32-ube-bitand"></span>`fn bitand(self, other: i32) -> <Self as >::Output`

##### `impl BitAndAssign for i32_ube`

- <span id="i32-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i32)`

##### `impl BitOr for i32_ube`

- <span id="i32-ube-bitor-type-output"></span>`type Output = i32`

- <span id="i32-ube-bitor"></span>`fn bitor(self, other: i32) -> <Self as >::Output`

##### `impl BitOrAssign for i32_ube`

- <span id="i32-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i32)`

##### `impl BitXor for i32_ube`

- <span id="i32-ube-bitxor-type-output"></span>`type Output = i32`

- <span id="i32-ube-bitxor"></span>`fn bitxor(self, other: i32) -> <Self as >::Output`

##### `impl BitXorAssign for i32_ube`

- <span id="i32-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i32)`

##### `impl<C> CheckBytes for i32_ube`

- <span id="i32-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i32_ube`

- <span id="i32-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i32_ube`

##### `impl Debug for i32_ube`

- <span id="i32-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i32_ube`

- <span id="i32-ube-default"></span>`fn default() -> Self`

##### `impl Display for i32_ube`

- <span id="i32-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i32_ube`

- <span id="i32-ube-div-type-output"></span>`type Output = i32`

- <span id="i32-ube-div"></span>`fn div(self, other: i32) -> <Self as >::Output`

##### `impl DivAssign for i32_ube`

- <span id="i32-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: i32)`

##### `impl Eq for i32_ube`

##### `impl Hash for i32_ube`

- <span id="i32-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i32_ube`

- <span id="i32-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i32_ube`

- <span id="i32-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i32_ube`

- <span id="i32-ube-mul-type-output"></span>`type Output = i32`

- <span id="i32-ube-mul"></span>`fn mul(self, other: i32) -> <Self as >::Output`

##### `impl MulAssign for i32_ube`

- <span id="i32-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i32)`

##### `impl Neg for i32_ube`

- <span id="i32-ube-neg-type-output"></span>`type Output = <i32 as Neg>::Output`

- <span id="i32-ube-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i32_ube`

- <span id="i32-ube-not-type-output"></span>`type Output = <i32 as Not>::Output`

- <span id="i32-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i32_ube`

- <span id="i32-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i32_ube`

- <span id="i32-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i32_ube`

- <span id="i32-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i32_ube`

- <span id="i32-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i32_ube`

- <span id="i32-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i32_ube`

- <span id="i32-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i32_ube`

- <span id="i32-ube-rem-type-output"></span>`type Output = i32`

- <span id="i32-ube-rem"></span>`fn rem(self, other: i32) -> <Self as >::Output`

##### `impl RemAssign for i32_ube`

- <span id="i32-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i32)`

##### `impl Shl for i32_ube`

- <span id="i32-ube-shl-type-output"></span>`type Output = i32`

- <span id="i32-ube-shl"></span>`fn shl(self, other: i32) -> <Self as >::Output`

##### `impl ShlAssign for i32_ube`

- <span id="i32-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i32)`

##### `impl Shr for i32_ube`

- <span id="i32-ube-shr-type-output"></span>`type Output = i32`

- <span id="i32-ube-shr"></span>`fn shr(self, other: i32) -> <Self as >::Output`

##### `impl ShrAssign for i32_ube`

- <span id="i32-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i32)`

##### `impl Sub for i32_ube`

- <span id="i32-ube-sub-type-output"></span>`type Output = i32`

- <span id="i32-ube-sub"></span>`fn sub(self, other: i32) -> <Self as >::Output`

##### `impl SubAssign for i32_ube`

- <span id="i32-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i32)`

##### `impl Sum for i32_ube`

- <span id="i32-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i32_ube`

- <span id="i32-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i32_ube`

- <span id="i32-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i32_ube`

- <span id="i32-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i64_ule`

```rust
struct i64_ule(i64);
```

A little-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="i64-ule-from-native"></span>`const fn from_native(value: i64) -> Self`

  Returns a `i64_ule` containing `value`.

- <span id="i64-ule-to-native"></span>`const fn to_native(self) -> i64`

  Returns a `i64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i64_ule`

- <span id="i64-ule-add-type-output"></span>`type Output = i64`

- <span id="i64-ule-add"></span>`fn add(self, other: i64) -> <Self as >::Output`

##### `impl AddAssign for i64_ule`

- <span id="i64-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: i64)`

##### `impl Binary for i64_ule`

- <span id="i64-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i64_ule`

- <span id="i64-ule-bitand-type-output"></span>`type Output = i64`

- <span id="i64-ule-bitand"></span>`fn bitand(self, other: i64) -> <Self as >::Output`

##### `impl BitAndAssign for i64_ule`

- <span id="i64-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i64)`

##### `impl BitOr for i64_ule`

- <span id="i64-ule-bitor-type-output"></span>`type Output = i64`

- <span id="i64-ule-bitor"></span>`fn bitor(self, other: i64) -> <Self as >::Output`

##### `impl BitOrAssign for i64_ule`

- <span id="i64-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i64)`

##### `impl BitXor for i64_ule`

- <span id="i64-ule-bitxor-type-output"></span>`type Output = i64`

- <span id="i64-ule-bitxor"></span>`fn bitxor(self, other: i64) -> <Self as >::Output`

##### `impl BitXorAssign for i64_ule`

- <span id="i64-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i64)`

##### `impl<C> CheckBytes for i64_ule`

- <span id="i64-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i64_ule`

- <span id="i64-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i64_ule`

##### `impl Debug for i64_ule`

- <span id="i64-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i64_ule`

- <span id="i64-ule-default"></span>`fn default() -> Self`

##### `impl Display for i64_ule`

- <span id="i64-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i64_ule`

- <span id="i64-ule-div-type-output"></span>`type Output = i64`

- <span id="i64-ule-div"></span>`fn div(self, other: i64) -> <Self as >::Output`

##### `impl DivAssign for i64_ule`

- <span id="i64-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: i64)`

##### `impl Eq for i64_ule`

##### `impl Hash for i64_ule`

- <span id="i64-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i64_ule`

- <span id="i64-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i64_ule`

- <span id="i64-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i64_ule`

- <span id="i64-ule-mul-type-output"></span>`type Output = i64`

- <span id="i64-ule-mul"></span>`fn mul(self, other: i64) -> <Self as >::Output`

##### `impl MulAssign for i64_ule`

- <span id="i64-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i64)`

##### `impl Neg for i64_ule`

- <span id="i64-ule-neg-type-output"></span>`type Output = <i64 as Neg>::Output`

- <span id="i64-ule-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i64_ule`

- <span id="i64-ule-not-type-output"></span>`type Output = <i64 as Not>::Output`

- <span id="i64-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i64_ule`

- <span id="i64-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i64_ule`

- <span id="i64-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i64_ule`

- <span id="i64-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i64_ule`

- <span id="i64-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i64_ule`

- <span id="i64-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i64_ule`

- <span id="i64-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i64_ule`

- <span id="i64-ule-rem-type-output"></span>`type Output = i64`

- <span id="i64-ule-rem"></span>`fn rem(self, other: i64) -> <Self as >::Output`

##### `impl RemAssign for i64_ule`

- <span id="i64-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i64)`

##### `impl Shl for i64_ule`

- <span id="i64-ule-shl-type-output"></span>`type Output = i64`

- <span id="i64-ule-shl"></span>`fn shl(self, other: i64) -> <Self as >::Output`

##### `impl ShlAssign for i64_ule`

- <span id="i64-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i64)`

##### `impl Shr for i64_ule`

- <span id="i64-ule-shr-type-output"></span>`type Output = i64`

- <span id="i64-ule-shr"></span>`fn shr(self, other: i64) -> <Self as >::Output`

##### `impl ShrAssign for i64_ule`

- <span id="i64-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i64)`

##### `impl Sub for i64_ule`

- <span id="i64-ule-sub-type-output"></span>`type Output = i64`

- <span id="i64-ule-sub"></span>`fn sub(self, other: i64) -> <Self as >::Output`

##### `impl SubAssign for i64_ule`

- <span id="i64-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i64)`

##### `impl Sum for i64_ule`

- <span id="i64-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i64_ule`

- <span id="i64-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i64_ule`

- <span id="i64-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i64_ule`

- <span id="i64-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i64_ube`

```rust
struct i64_ube(i64);
```

A big-endian unaligned `i64` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="i64-ube-from-native"></span>`const fn from_native(value: i64) -> Self`

  Returns a `i64_ube` containing `value`.

- <span id="i64-ube-to-native"></span>`const fn to_native(self) -> i64`

  Returns a `i64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i64_ube`

- <span id="i64-ube-add-type-output"></span>`type Output = i64`

- <span id="i64-ube-add"></span>`fn add(self, other: i64) -> <Self as >::Output`

##### `impl AddAssign for i64_ube`

- <span id="i64-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: i64)`

##### `impl Binary for i64_ube`

- <span id="i64-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i64_ube`

- <span id="i64-ube-bitand-type-output"></span>`type Output = i64`

- <span id="i64-ube-bitand"></span>`fn bitand(self, other: i64) -> <Self as >::Output`

##### `impl BitAndAssign for i64_ube`

- <span id="i64-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i64)`

##### `impl BitOr for i64_ube`

- <span id="i64-ube-bitor-type-output"></span>`type Output = i64`

- <span id="i64-ube-bitor"></span>`fn bitor(self, other: i64) -> <Self as >::Output`

##### `impl BitOrAssign for i64_ube`

- <span id="i64-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i64)`

##### `impl BitXor for i64_ube`

- <span id="i64-ube-bitxor-type-output"></span>`type Output = i64`

- <span id="i64-ube-bitxor"></span>`fn bitxor(self, other: i64) -> <Self as >::Output`

##### `impl BitXorAssign for i64_ube`

- <span id="i64-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i64)`

##### `impl<C> CheckBytes for i64_ube`

- <span id="i64-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i64_ube`

- <span id="i64-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i64_ube`

##### `impl Debug for i64_ube`

- <span id="i64-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i64_ube`

- <span id="i64-ube-default"></span>`fn default() -> Self`

##### `impl Display for i64_ube`

- <span id="i64-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i64_ube`

- <span id="i64-ube-div-type-output"></span>`type Output = i64`

- <span id="i64-ube-div"></span>`fn div(self, other: i64) -> <Self as >::Output`

##### `impl DivAssign for i64_ube`

- <span id="i64-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: i64)`

##### `impl Eq for i64_ube`

##### `impl Hash for i64_ube`

- <span id="i64-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i64_ube`

- <span id="i64-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i64_ube`

- <span id="i64-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i64_ube`

- <span id="i64-ube-mul-type-output"></span>`type Output = i64`

- <span id="i64-ube-mul"></span>`fn mul(self, other: i64) -> <Self as >::Output`

##### `impl MulAssign for i64_ube`

- <span id="i64-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i64)`

##### `impl Neg for i64_ube`

- <span id="i64-ube-neg-type-output"></span>`type Output = <i64 as Neg>::Output`

- <span id="i64-ube-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i64_ube`

- <span id="i64-ube-not-type-output"></span>`type Output = <i64 as Not>::Output`

- <span id="i64-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i64_ube`

- <span id="i64-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i64_ube`

- <span id="i64-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i64_ube`

- <span id="i64-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i64_ube`

- <span id="i64-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i64_ube`

- <span id="i64-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i64_ube`

- <span id="i64-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i64_ube`

- <span id="i64-ube-rem-type-output"></span>`type Output = i64`

- <span id="i64-ube-rem"></span>`fn rem(self, other: i64) -> <Self as >::Output`

##### `impl RemAssign for i64_ube`

- <span id="i64-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i64)`

##### `impl Shl for i64_ube`

- <span id="i64-ube-shl-type-output"></span>`type Output = i64`

- <span id="i64-ube-shl"></span>`fn shl(self, other: i64) -> <Self as >::Output`

##### `impl ShlAssign for i64_ube`

- <span id="i64-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i64)`

##### `impl Shr for i64_ube`

- <span id="i64-ube-shr-type-output"></span>`type Output = i64`

- <span id="i64-ube-shr"></span>`fn shr(self, other: i64) -> <Self as >::Output`

##### `impl ShrAssign for i64_ube`

- <span id="i64-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i64)`

##### `impl Sub for i64_ube`

- <span id="i64-ube-sub-type-output"></span>`type Output = i64`

- <span id="i64-ube-sub"></span>`fn sub(self, other: i64) -> <Self as >::Output`

##### `impl SubAssign for i64_ube`

- <span id="i64-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i64)`

##### `impl Sum for i64_ube`

- <span id="i64-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i64_ube`

- <span id="i64-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i64_ube`

- <span id="i64-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i64_ube`

- <span id="i64-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i128_ule`

```rust
struct i128_ule(i128);
```

A little-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="i128-ule-from-native"></span>`const fn from_native(value: i128) -> Self`

  Returns a `i128_ule` containing `value`.

- <span id="i128-ule-to-native"></span>`const fn to_native(self) -> i128`

  Returns a `i128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i128_ule`

- <span id="i128-ule-add-type-output"></span>`type Output = i128`

- <span id="i128-ule-add"></span>`fn add(self, other: i128) -> <Self as >::Output`

##### `impl AddAssign for i128_ule`

- <span id="i128-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: i128)`

##### `impl Binary for i128_ule`

- <span id="i128-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i128_ule`

- <span id="i128-ule-bitand-type-output"></span>`type Output = i128`

- <span id="i128-ule-bitand"></span>`fn bitand(self, other: i128) -> <Self as >::Output`

##### `impl BitAndAssign for i128_ule`

- <span id="i128-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i128)`

##### `impl BitOr for i128_ule`

- <span id="i128-ule-bitor-type-output"></span>`type Output = i128`

- <span id="i128-ule-bitor"></span>`fn bitor(self, other: i128) -> <Self as >::Output`

##### `impl BitOrAssign for i128_ule`

- <span id="i128-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i128)`

##### `impl BitXor for i128_ule`

- <span id="i128-ule-bitxor-type-output"></span>`type Output = i128`

- <span id="i128-ule-bitxor"></span>`fn bitxor(self, other: i128) -> <Self as >::Output`

##### `impl BitXorAssign for i128_ule`

- <span id="i128-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i128)`

##### `impl<C> CheckBytes for i128_ule`

- <span id="i128-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i128_ule`

- <span id="i128-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i128_ule`

##### `impl Debug for i128_ule`

- <span id="i128-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i128_ule`

- <span id="i128-ule-default"></span>`fn default() -> Self`

##### `impl Display for i128_ule`

- <span id="i128-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i128_ule`

- <span id="i128-ule-div-type-output"></span>`type Output = i128`

- <span id="i128-ule-div"></span>`fn div(self, other: i128) -> <Self as >::Output`

##### `impl DivAssign for i128_ule`

- <span id="i128-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: i128)`

##### `impl Eq for i128_ule`

##### `impl Hash for i128_ule`

- <span id="i128-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i128_ule`

- <span id="i128-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i128_ule`

- <span id="i128-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i128_ule`

- <span id="i128-ule-mul-type-output"></span>`type Output = i128`

- <span id="i128-ule-mul"></span>`fn mul(self, other: i128) -> <Self as >::Output`

##### `impl MulAssign for i128_ule`

- <span id="i128-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i128)`

##### `impl Neg for i128_ule`

- <span id="i128-ule-neg-type-output"></span>`type Output = <i128 as Neg>::Output`

- <span id="i128-ule-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i128_ule`

- <span id="i128-ule-not-type-output"></span>`type Output = <i128 as Not>::Output`

- <span id="i128-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i128_ule`

- <span id="i128-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i128_ule`

- <span id="i128-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i128_ule`

- <span id="i128-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i128_ule`

- <span id="i128-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i128_ule`

- <span id="i128-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i128_ule`

- <span id="i128-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i128_ule`

- <span id="i128-ule-rem-type-output"></span>`type Output = i128`

- <span id="i128-ule-rem"></span>`fn rem(self, other: i128) -> <Self as >::Output`

##### `impl RemAssign for i128_ule`

- <span id="i128-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i128)`

##### `impl Shl for i128_ule`

- <span id="i128-ule-shl-type-output"></span>`type Output = i128`

- <span id="i128-ule-shl"></span>`fn shl(self, other: i128) -> <Self as >::Output`

##### `impl ShlAssign for i128_ule`

- <span id="i128-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i128)`

##### `impl Shr for i128_ule`

- <span id="i128-ule-shr-type-output"></span>`type Output = i128`

- <span id="i128-ule-shr"></span>`fn shr(self, other: i128) -> <Self as >::Output`

##### `impl ShrAssign for i128_ule`

- <span id="i128-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i128)`

##### `impl Sub for i128_ule`

- <span id="i128-ule-sub-type-output"></span>`type Output = i128`

- <span id="i128-ule-sub"></span>`fn sub(self, other: i128) -> <Self as >::Output`

##### `impl SubAssign for i128_ule`

- <span id="i128-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i128)`

##### `impl Sum for i128_ule`

- <span id="i128-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i128_ule`

- <span id="i128-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i128_ule`

- <span id="i128-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i128_ule`

- <span id="i128-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `i128_ube`

```rust
struct i128_ube(i128);
```

A big-endian unaligned `i128` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="i128-ube-from-native"></span>`const fn from_native(value: i128) -> Self`

  Returns a `i128_ube` containing `value`.

- <span id="i128-ube-to-native"></span>`const fn to_native(self) -> i128`

  Returns a `i128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for i128_ube`

- <span id="i128-ube-add-type-output"></span>`type Output = i128`

- <span id="i128-ube-add"></span>`fn add(self, other: i128) -> <Self as >::Output`

##### `impl AddAssign for i128_ube`

- <span id="i128-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: i128)`

##### `impl Binary for i128_ube`

- <span id="i128-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for i128_ube`

- <span id="i128-ube-bitand-type-output"></span>`type Output = i128`

- <span id="i128-ube-bitand"></span>`fn bitand(self, other: i128) -> <Self as >::Output`

##### `impl BitAndAssign for i128_ube`

- <span id="i128-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: i128)`

##### `impl BitOr for i128_ube`

- <span id="i128-ube-bitor-type-output"></span>`type Output = i128`

- <span id="i128-ube-bitor"></span>`fn bitor(self, other: i128) -> <Self as >::Output`

##### `impl BitOrAssign for i128_ube`

- <span id="i128-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: i128)`

##### `impl BitXor for i128_ube`

- <span id="i128-ube-bitxor-type-output"></span>`type Output = i128`

- <span id="i128-ube-bitxor"></span>`fn bitxor(self, other: i128) -> <Self as >::Output`

##### `impl BitXorAssign for i128_ube`

- <span id="i128-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: i128)`

##### `impl<C> CheckBytes for i128_ube`

- <span id="i128-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for i128_ube`

- <span id="i128-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for i128_ube`

##### `impl Debug for i128_ube`

- <span id="i128-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for i128_ube`

- <span id="i128-ube-default"></span>`fn default() -> Self`

##### `impl Display for i128_ube`

- <span id="i128-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for i128_ube`

- <span id="i128-ube-div-type-output"></span>`type Output = i128`

- <span id="i128-ube-div"></span>`fn div(self, other: i128) -> <Self as >::Output`

##### `impl DivAssign for i128_ube`

- <span id="i128-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: i128)`

##### `impl Eq for i128_ube`

##### `impl Hash for i128_ube`

- <span id="i128-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for i128_ube`

- <span id="i128-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for i128_ube`

- <span id="i128-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for i128_ube`

- <span id="i128-ube-mul-type-output"></span>`type Output = i128`

- <span id="i128-ube-mul"></span>`fn mul(self, other: i128) -> <Self as >::Output`

##### `impl MulAssign for i128_ube`

- <span id="i128-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: i128)`

##### `impl Neg for i128_ube`

- <span id="i128-ube-neg-type-output"></span>`type Output = <i128 as Neg>::Output`

- <span id="i128-ube-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl Not for i128_ube`

- <span id="i128-ube-not-type-output"></span>`type Output = <i128 as Not>::Output`

- <span id="i128-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for i128_ube`

- <span id="i128-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for i128_ube`

- <span id="i128-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for i128_ube`

- <span id="i128-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for i128_ube`

- <span id="i128-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for i128_ube`

- <span id="i128-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for i128_ube`

- <span id="i128-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for i128_ube`

- <span id="i128-ube-rem-type-output"></span>`type Output = i128`

- <span id="i128-ube-rem"></span>`fn rem(self, other: i128) -> <Self as >::Output`

##### `impl RemAssign for i128_ube`

- <span id="i128-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: i128)`

##### `impl Shl for i128_ube`

- <span id="i128-ube-shl-type-output"></span>`type Output = i128`

- <span id="i128-ube-shl"></span>`fn shl(self, other: i128) -> <Self as >::Output`

##### `impl ShlAssign for i128_ube`

- <span id="i128-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: i128)`

##### `impl Shr for i128_ube`

- <span id="i128-ube-shr-type-output"></span>`type Output = i128`

- <span id="i128-ube-shr"></span>`fn shr(self, other: i128) -> <Self as >::Output`

##### `impl ShrAssign for i128_ube`

- <span id="i128-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: i128)`

##### `impl Sub for i128_ube`

- <span id="i128-ube-sub-type-output"></span>`type Output = i128`

- <span id="i128-ube-sub"></span>`fn sub(self, other: i128) -> <Self as >::Output`

##### `impl SubAssign for i128_ube`

- <span id="i128-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: i128)`

##### `impl Sum for i128_ube`

- <span id="i128-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for i128_ube`

- <span id="i128-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for i128_ube`

- <span id="i128-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for i128_ube`

- <span id="i128-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u16_ule`

```rust
struct u16_ule(u16);
```

A little-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="u16-ule-from-native"></span>`const fn from_native(value: u16) -> Self`

  Returns a `u16_ule` containing `value`.

- <span id="u16-ule-to-native"></span>`const fn to_native(self) -> u16`

  Returns a `u16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u16_ule`

- <span id="u16-ule-add-type-output"></span>`type Output = u16`

- <span id="u16-ule-add"></span>`fn add(self, other: u16) -> <Self as >::Output`

##### `impl AddAssign for u16_ule`

- <span id="u16-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: u16)`

##### `impl Binary for u16_ule`

- <span id="u16-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u16_ule`

- <span id="u16-ule-bitand-type-output"></span>`type Output = u16`

- <span id="u16-ule-bitand"></span>`fn bitand(self, other: u16) -> <Self as >::Output`

##### `impl BitAndAssign for u16_ule`

- <span id="u16-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u16)`

##### `impl BitOr for u16_ule`

- <span id="u16-ule-bitor-type-output"></span>`type Output = u16`

- <span id="u16-ule-bitor"></span>`fn bitor(self, other: u16) -> <Self as >::Output`

##### `impl BitOrAssign for u16_ule`

- <span id="u16-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u16)`

##### `impl BitXor for u16_ule`

- <span id="u16-ule-bitxor-type-output"></span>`type Output = u16`

- <span id="u16-ule-bitxor"></span>`fn bitxor(self, other: u16) -> <Self as >::Output`

##### `impl BitXorAssign for u16_ule`

- <span id="u16-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u16)`

##### `impl<C> CheckBytes for u16_ule`

- <span id="u16-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u16_ule`

- <span id="u16-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u16_ule`

##### `impl Debug for u16_ule`

- <span id="u16-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u16_ule`

- <span id="u16-ule-default"></span>`fn default() -> Self`

##### `impl Display for u16_ule`

- <span id="u16-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u16_ule`

- <span id="u16-ule-div-type-output"></span>`type Output = u16`

- <span id="u16-ule-div"></span>`fn div(self, other: u16) -> <Self as >::Output`

##### `impl DivAssign for u16_ule`

- <span id="u16-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: u16)`

##### `impl Eq for u16_ule`

##### `impl Hash for u16_ule`

- <span id="u16-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u16_ule`

- <span id="u16-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u16_ule`

- <span id="u16-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u16_ule`

- <span id="u16-ule-mul-type-output"></span>`type Output = u16`

- <span id="u16-ule-mul"></span>`fn mul(self, other: u16) -> <Self as >::Output`

##### `impl MulAssign for u16_ule`

- <span id="u16-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u16)`

##### `impl Not for u16_ule`

- <span id="u16-ule-not-type-output"></span>`type Output = <u16 as Not>::Output`

- <span id="u16-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u16_ule`

- <span id="u16-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u16_ule`

- <span id="u16-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u16_ule`

- <span id="u16-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u16_ule`

- <span id="u16-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u16_ule`

- <span id="u16-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u16_ule`

- <span id="u16-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u16_ule`

- <span id="u16-ule-rem-type-output"></span>`type Output = u16`

- <span id="u16-ule-rem"></span>`fn rem(self, other: u16) -> <Self as >::Output`

##### `impl RemAssign for u16_ule`

- <span id="u16-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u16)`

##### `impl Shl for u16_ule`

- <span id="u16-ule-shl-type-output"></span>`type Output = u16`

- <span id="u16-ule-shl"></span>`fn shl(self, other: u16) -> <Self as >::Output`

##### `impl ShlAssign for u16_ule`

- <span id="u16-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u16)`

##### `impl Shr for u16_ule`

- <span id="u16-ule-shr-type-output"></span>`type Output = u16`

- <span id="u16-ule-shr"></span>`fn shr(self, other: u16) -> <Self as >::Output`

##### `impl ShrAssign for u16_ule`

- <span id="u16-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u16)`

##### `impl Sub for u16_ule`

- <span id="u16-ule-sub-type-output"></span>`type Output = u16`

- <span id="u16-ule-sub"></span>`fn sub(self, other: u16) -> <Self as >::Output`

##### `impl SubAssign for u16_ule`

- <span id="u16-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u16)`

##### `impl Sum for u16_ule`

- <span id="u16-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u16_ule`

- <span id="u16-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u16_ule`

- <span id="u16-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u16_ule`

- <span id="u16-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u16_ube`

```rust
struct u16_ube(u16);
```

A big-endian unaligned `u16` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="u16-ube-from-native"></span>`const fn from_native(value: u16) -> Self`

  Returns a `u16_ube` containing `value`.

- <span id="u16-ube-to-native"></span>`const fn to_native(self) -> u16`

  Returns a `u16` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u16_ube`

- <span id="u16-ube-add-type-output"></span>`type Output = u16`

- <span id="u16-ube-add"></span>`fn add(self, other: u16) -> <Self as >::Output`

##### `impl AddAssign for u16_ube`

- <span id="u16-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: u16)`

##### `impl Binary for u16_ube`

- <span id="u16-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u16_ube`

- <span id="u16-ube-bitand-type-output"></span>`type Output = u16`

- <span id="u16-ube-bitand"></span>`fn bitand(self, other: u16) -> <Self as >::Output`

##### `impl BitAndAssign for u16_ube`

- <span id="u16-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u16)`

##### `impl BitOr for u16_ube`

- <span id="u16-ube-bitor-type-output"></span>`type Output = u16`

- <span id="u16-ube-bitor"></span>`fn bitor(self, other: u16) -> <Self as >::Output`

##### `impl BitOrAssign for u16_ube`

- <span id="u16-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u16)`

##### `impl BitXor for u16_ube`

- <span id="u16-ube-bitxor-type-output"></span>`type Output = u16`

- <span id="u16-ube-bitxor"></span>`fn bitxor(self, other: u16) -> <Self as >::Output`

##### `impl BitXorAssign for u16_ube`

- <span id="u16-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u16)`

##### `impl<C> CheckBytes for u16_ube`

- <span id="u16-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u16_ube`

- <span id="u16-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u16_ube`

##### `impl Debug for u16_ube`

- <span id="u16-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u16_ube`

- <span id="u16-ube-default"></span>`fn default() -> Self`

##### `impl Display for u16_ube`

- <span id="u16-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u16_ube`

- <span id="u16-ube-div-type-output"></span>`type Output = u16`

- <span id="u16-ube-div"></span>`fn div(self, other: u16) -> <Self as >::Output`

##### `impl DivAssign for u16_ube`

- <span id="u16-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: u16)`

##### `impl Eq for u16_ube`

##### `impl Hash for u16_ube`

- <span id="u16-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u16_ube`

- <span id="u16-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u16_ube`

- <span id="u16-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u16_ube`

- <span id="u16-ube-mul-type-output"></span>`type Output = u16`

- <span id="u16-ube-mul"></span>`fn mul(self, other: u16) -> <Self as >::Output`

##### `impl MulAssign for u16_ube`

- <span id="u16-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u16)`

##### `impl Not for u16_ube`

- <span id="u16-ube-not-type-output"></span>`type Output = <u16 as Not>::Output`

- <span id="u16-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u16_ube`

- <span id="u16-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u16_ube`

- <span id="u16-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u16_ube`

- <span id="u16-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u16_ube`

- <span id="u16-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u16_ube`

- <span id="u16-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u16_ube`

- <span id="u16-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u16_ube`

- <span id="u16-ube-rem-type-output"></span>`type Output = u16`

- <span id="u16-ube-rem"></span>`fn rem(self, other: u16) -> <Self as >::Output`

##### `impl RemAssign for u16_ube`

- <span id="u16-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u16)`

##### `impl Shl for u16_ube`

- <span id="u16-ube-shl-type-output"></span>`type Output = u16`

- <span id="u16-ube-shl"></span>`fn shl(self, other: u16) -> <Self as >::Output`

##### `impl ShlAssign for u16_ube`

- <span id="u16-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u16)`

##### `impl Shr for u16_ube`

- <span id="u16-ube-shr-type-output"></span>`type Output = u16`

- <span id="u16-ube-shr"></span>`fn shr(self, other: u16) -> <Self as >::Output`

##### `impl ShrAssign for u16_ube`

- <span id="u16-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u16)`

##### `impl Sub for u16_ube`

- <span id="u16-ube-sub-type-output"></span>`type Output = u16`

- <span id="u16-ube-sub"></span>`fn sub(self, other: u16) -> <Self as >::Output`

##### `impl SubAssign for u16_ube`

- <span id="u16-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u16)`

##### `impl Sum for u16_ube`

- <span id="u16-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u16_ube`

- <span id="u16-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u16_ube`

- <span id="u16-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u16_ube`

- <span id="u16-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u32_ule`

```rust
struct u32_ule(u32);
```

A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="u32-ule-from-native"></span>`const fn from_native(value: u32) -> Self`

  Returns a `u32_ule` containing `value`.

- <span id="u32-ule-to-native"></span>`const fn to_native(self) -> u32`

  Returns a `u32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u32_ule`

- <span id="u32-ule-add-type-output"></span>`type Output = u32`

- <span id="u32-ule-add"></span>`fn add(self, other: u32) -> <Self as >::Output`

##### `impl AddAssign for u32_ule`

- <span id="u32-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: u32)`

##### `impl Binary for u32_ule`

- <span id="u32-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u32_ule`

- <span id="u32-ule-bitand-type-output"></span>`type Output = u32`

- <span id="u32-ule-bitand"></span>`fn bitand(self, other: u32) -> <Self as >::Output`

##### `impl BitAndAssign for u32_ule`

- <span id="u32-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u32)`

##### `impl BitOr for u32_ule`

- <span id="u32-ule-bitor-type-output"></span>`type Output = u32`

- <span id="u32-ule-bitor"></span>`fn bitor(self, other: u32) -> <Self as >::Output`

##### `impl BitOrAssign for u32_ule`

- <span id="u32-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u32)`

##### `impl BitXor for u32_ule`

- <span id="u32-ule-bitxor-type-output"></span>`type Output = u32`

- <span id="u32-ule-bitxor"></span>`fn bitxor(self, other: u32) -> <Self as >::Output`

##### `impl BitXorAssign for u32_ule`

- <span id="u32-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u32)`

##### `impl<C> CheckBytes for u32_ule`

- <span id="u32-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u32_ule`

- <span id="u32-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u32_ule`

##### `impl Debug for u32_ule`

- <span id="u32-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u32_ule`

- <span id="u32-ule-default"></span>`fn default() -> Self`

##### `impl Display for u32_ule`

- <span id="u32-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u32_ule`

- <span id="u32-ule-div-type-output"></span>`type Output = u32`

- <span id="u32-ule-div"></span>`fn div(self, other: u32) -> <Self as >::Output`

##### `impl DivAssign for u32_ule`

- <span id="u32-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: u32)`

##### `impl Eq for u32_ule`

##### `impl Hash for u32_ule`

- <span id="u32-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u32_ule`

- <span id="u32-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u32_ule`

- <span id="u32-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u32_ule`

- <span id="u32-ule-mul-type-output"></span>`type Output = u32`

- <span id="u32-ule-mul"></span>`fn mul(self, other: u32) -> <Self as >::Output`

##### `impl MulAssign for u32_ule`

- <span id="u32-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u32)`

##### `impl Not for u32_ule`

- <span id="u32-ule-not-type-output"></span>`type Output = <u32 as Not>::Output`

- <span id="u32-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u32_ule`

- <span id="u32-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u32_ule`

- <span id="u32-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u32_ule`

- <span id="u32-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u32_ule`

- <span id="u32-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u32_ule`

- <span id="u32-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u32_ule`

- <span id="u32-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u32_ule`

- <span id="u32-ule-rem-type-output"></span>`type Output = u32`

- <span id="u32-ule-rem"></span>`fn rem(self, other: u32) -> <Self as >::Output`

##### `impl RemAssign for u32_ule`

- <span id="u32-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u32)`

##### `impl Shl for u32_ule`

- <span id="u32-ule-shl-type-output"></span>`type Output = u32`

- <span id="u32-ule-shl"></span>`fn shl(self, other: u32) -> <Self as >::Output`

##### `impl ShlAssign for u32_ule`

- <span id="u32-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u32)`

##### `impl Shr for u32_ule`

- <span id="u32-ule-shr-type-output"></span>`type Output = u32`

- <span id="u32-ule-shr"></span>`fn shr(self, other: u32) -> <Self as >::Output`

##### `impl ShrAssign for u32_ule`

- <span id="u32-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u32)`

##### `impl Sub for u32_ule`

- <span id="u32-ule-sub-type-output"></span>`type Output = u32`

- <span id="u32-ule-sub"></span>`fn sub(self, other: u32) -> <Self as >::Output`

##### `impl SubAssign for u32_ule`

- <span id="u32-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u32)`

##### `impl Sum for u32_ule`

- <span id="u32-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u32_ule`

- <span id="u32-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u32_ule`

- <span id="u32-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u32_ule`

- <span id="u32-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u32_ube`

```rust
struct u32_ube(u32);
```

A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="u32-ube-from-native"></span>`const fn from_native(value: u32) -> Self`

  Returns a `u32_ube` containing `value`.

- <span id="u32-ube-to-native"></span>`const fn to_native(self) -> u32`

  Returns a `u32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u32_ube`

- <span id="u32-ube-add-type-output"></span>`type Output = u32`

- <span id="u32-ube-add"></span>`fn add(self, other: u32) -> <Self as >::Output`

##### `impl AddAssign for u32_ube`

- <span id="u32-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: u32)`

##### `impl Binary for u32_ube`

- <span id="u32-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u32_ube`

- <span id="u32-ube-bitand-type-output"></span>`type Output = u32`

- <span id="u32-ube-bitand"></span>`fn bitand(self, other: u32) -> <Self as >::Output`

##### `impl BitAndAssign for u32_ube`

- <span id="u32-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u32)`

##### `impl BitOr for u32_ube`

- <span id="u32-ube-bitor-type-output"></span>`type Output = u32`

- <span id="u32-ube-bitor"></span>`fn bitor(self, other: u32) -> <Self as >::Output`

##### `impl BitOrAssign for u32_ube`

- <span id="u32-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u32)`

##### `impl BitXor for u32_ube`

- <span id="u32-ube-bitxor-type-output"></span>`type Output = u32`

- <span id="u32-ube-bitxor"></span>`fn bitxor(self, other: u32) -> <Self as >::Output`

##### `impl BitXorAssign for u32_ube`

- <span id="u32-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u32)`

##### `impl<C> CheckBytes for u32_ube`

- <span id="u32-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u32_ube`

- <span id="u32-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u32_ube`

##### `impl Debug for u32_ube`

- <span id="u32-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u32_ube`

- <span id="u32-ube-default"></span>`fn default() -> Self`

##### `impl Display for u32_ube`

- <span id="u32-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u32_ube`

- <span id="u32-ube-div-type-output"></span>`type Output = u32`

- <span id="u32-ube-div"></span>`fn div(self, other: u32) -> <Self as >::Output`

##### `impl DivAssign for u32_ube`

- <span id="u32-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: u32)`

##### `impl Eq for u32_ube`

##### `impl Hash for u32_ube`

- <span id="u32-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u32_ube`

- <span id="u32-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u32_ube`

- <span id="u32-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u32_ube`

- <span id="u32-ube-mul-type-output"></span>`type Output = u32`

- <span id="u32-ube-mul"></span>`fn mul(self, other: u32) -> <Self as >::Output`

##### `impl MulAssign for u32_ube`

- <span id="u32-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u32)`

##### `impl Not for u32_ube`

- <span id="u32-ube-not-type-output"></span>`type Output = <u32 as Not>::Output`

- <span id="u32-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u32_ube`

- <span id="u32-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u32_ube`

- <span id="u32-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u32_ube`

- <span id="u32-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u32_ube`

- <span id="u32-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u32_ube`

- <span id="u32-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u32_ube`

- <span id="u32-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u32_ube`

- <span id="u32-ube-rem-type-output"></span>`type Output = u32`

- <span id="u32-ube-rem"></span>`fn rem(self, other: u32) -> <Self as >::Output`

##### `impl RemAssign for u32_ube`

- <span id="u32-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u32)`

##### `impl Shl for u32_ube`

- <span id="u32-ube-shl-type-output"></span>`type Output = u32`

- <span id="u32-ube-shl"></span>`fn shl(self, other: u32) -> <Self as >::Output`

##### `impl ShlAssign for u32_ube`

- <span id="u32-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u32)`

##### `impl Shr for u32_ube`

- <span id="u32-ube-shr-type-output"></span>`type Output = u32`

- <span id="u32-ube-shr"></span>`fn shr(self, other: u32) -> <Self as >::Output`

##### `impl ShrAssign for u32_ube`

- <span id="u32-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u32)`

##### `impl Sub for u32_ube`

- <span id="u32-ube-sub-type-output"></span>`type Output = u32`

- <span id="u32-ube-sub"></span>`fn sub(self, other: u32) -> <Self as >::Output`

##### `impl SubAssign for u32_ube`

- <span id="u32-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u32)`

##### `impl Sum for u32_ube`

- <span id="u32-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u32_ube`

- <span id="u32-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u32_ube`

- <span id="u32-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u32_ube`

- <span id="u32-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u64_ule`

```rust
struct u64_ule(u64);
```

A little-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="u64-ule-from-native"></span>`const fn from_native(value: u64) -> Self`

  Returns a `u64_ule` containing `value`.

- <span id="u64-ule-to-native"></span>`const fn to_native(self) -> u64`

  Returns a `u64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u64_ule`

- <span id="u64-ule-add-type-output"></span>`type Output = u64`

- <span id="u64-ule-add"></span>`fn add(self, other: u64) -> <Self as >::Output`

##### `impl AddAssign for u64_ule`

- <span id="u64-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: u64)`

##### `impl Binary for u64_ule`

- <span id="u64-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u64_ule`

- <span id="u64-ule-bitand-type-output"></span>`type Output = u64`

- <span id="u64-ule-bitand"></span>`fn bitand(self, other: u64) -> <Self as >::Output`

##### `impl BitAndAssign for u64_ule`

- <span id="u64-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u64)`

##### `impl BitOr for u64_ule`

- <span id="u64-ule-bitor-type-output"></span>`type Output = u64`

- <span id="u64-ule-bitor"></span>`fn bitor(self, other: u64) -> <Self as >::Output`

##### `impl BitOrAssign for u64_ule`

- <span id="u64-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u64)`

##### `impl BitXor for u64_ule`

- <span id="u64-ule-bitxor-type-output"></span>`type Output = u64`

- <span id="u64-ule-bitxor"></span>`fn bitxor(self, other: u64) -> <Self as >::Output`

##### `impl BitXorAssign for u64_ule`

- <span id="u64-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u64)`

##### `impl<C> CheckBytes for u64_ule`

- <span id="u64-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u64_ule`

- <span id="u64-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u64_ule`

##### `impl Debug for u64_ule`

- <span id="u64-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u64_ule`

- <span id="u64-ule-default"></span>`fn default() -> Self`

##### `impl Display for u64_ule`

- <span id="u64-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u64_ule`

- <span id="u64-ule-div-type-output"></span>`type Output = u64`

- <span id="u64-ule-div"></span>`fn div(self, other: u64) -> <Self as >::Output`

##### `impl DivAssign for u64_ule`

- <span id="u64-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: u64)`

##### `impl Eq for u64_ule`

##### `impl Hash for u64_ule`

- <span id="u64-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u64_ule`

- <span id="u64-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u64_ule`

- <span id="u64-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u64_ule`

- <span id="u64-ule-mul-type-output"></span>`type Output = u64`

- <span id="u64-ule-mul"></span>`fn mul(self, other: u64) -> <Self as >::Output`

##### `impl MulAssign for u64_ule`

- <span id="u64-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u64)`

##### `impl Not for u64_ule`

- <span id="u64-ule-not-type-output"></span>`type Output = <u64 as Not>::Output`

- <span id="u64-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u64_ule`

- <span id="u64-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u64_ule`

- <span id="u64-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u64_ule`

- <span id="u64-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u64_ule`

- <span id="u64-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u64_ule`

- <span id="u64-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u64_ule`

- <span id="u64-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u64_ule`

- <span id="u64-ule-rem-type-output"></span>`type Output = u64`

- <span id="u64-ule-rem"></span>`fn rem(self, other: u64) -> <Self as >::Output`

##### `impl RemAssign for u64_ule`

- <span id="u64-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u64)`

##### `impl Shl for u64_ule`

- <span id="u64-ule-shl-type-output"></span>`type Output = u64`

- <span id="u64-ule-shl"></span>`fn shl(self, other: u64) -> <Self as >::Output`

##### `impl ShlAssign for u64_ule`

- <span id="u64-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u64)`

##### `impl Shr for u64_ule`

- <span id="u64-ule-shr-type-output"></span>`type Output = u64`

- <span id="u64-ule-shr"></span>`fn shr(self, other: u64) -> <Self as >::Output`

##### `impl ShrAssign for u64_ule`

- <span id="u64-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u64)`

##### `impl Sub for u64_ule`

- <span id="u64-ule-sub-type-output"></span>`type Output = u64`

- <span id="u64-ule-sub"></span>`fn sub(self, other: u64) -> <Self as >::Output`

##### `impl SubAssign for u64_ule`

- <span id="u64-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u64)`

##### `impl Sum for u64_ule`

- <span id="u64-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u64_ule`

- <span id="u64-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u64_ule`

- <span id="u64-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u64_ule`

- <span id="u64-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u64_ube`

```rust
struct u64_ube(u64);
```

A big-endian unaligned `u64` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="u64-ube-from-native"></span>`const fn from_native(value: u64) -> Self`

  Returns a `u64_ube` containing `value`.

- <span id="u64-ube-to-native"></span>`const fn to_native(self) -> u64`

  Returns a `u64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u64_ube`

- <span id="u64-ube-add-type-output"></span>`type Output = u64`

- <span id="u64-ube-add"></span>`fn add(self, other: u64) -> <Self as >::Output`

##### `impl AddAssign for u64_ube`

- <span id="u64-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: u64)`

##### `impl Binary for u64_ube`

- <span id="u64-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u64_ube`

- <span id="u64-ube-bitand-type-output"></span>`type Output = u64`

- <span id="u64-ube-bitand"></span>`fn bitand(self, other: u64) -> <Self as >::Output`

##### `impl BitAndAssign for u64_ube`

- <span id="u64-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u64)`

##### `impl BitOr for u64_ube`

- <span id="u64-ube-bitor-type-output"></span>`type Output = u64`

- <span id="u64-ube-bitor"></span>`fn bitor(self, other: u64) -> <Self as >::Output`

##### `impl BitOrAssign for u64_ube`

- <span id="u64-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u64)`

##### `impl BitXor for u64_ube`

- <span id="u64-ube-bitxor-type-output"></span>`type Output = u64`

- <span id="u64-ube-bitxor"></span>`fn bitxor(self, other: u64) -> <Self as >::Output`

##### `impl BitXorAssign for u64_ube`

- <span id="u64-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u64)`

##### `impl<C> CheckBytes for u64_ube`

- <span id="u64-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u64_ube`

- <span id="u64-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u64_ube`

##### `impl Debug for u64_ube`

- <span id="u64-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u64_ube`

- <span id="u64-ube-default"></span>`fn default() -> Self`

##### `impl Display for u64_ube`

- <span id="u64-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u64_ube`

- <span id="u64-ube-div-type-output"></span>`type Output = u64`

- <span id="u64-ube-div"></span>`fn div(self, other: u64) -> <Self as >::Output`

##### `impl DivAssign for u64_ube`

- <span id="u64-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: u64)`

##### `impl Eq for u64_ube`

##### `impl Hash for u64_ube`

- <span id="u64-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u64_ube`

- <span id="u64-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u64_ube`

- <span id="u64-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u64_ube`

- <span id="u64-ube-mul-type-output"></span>`type Output = u64`

- <span id="u64-ube-mul"></span>`fn mul(self, other: u64) -> <Self as >::Output`

##### `impl MulAssign for u64_ube`

- <span id="u64-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u64)`

##### `impl Not for u64_ube`

- <span id="u64-ube-not-type-output"></span>`type Output = <u64 as Not>::Output`

- <span id="u64-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u64_ube`

- <span id="u64-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u64_ube`

- <span id="u64-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u64_ube`

- <span id="u64-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u64_ube`

- <span id="u64-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u64_ube`

- <span id="u64-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u64_ube`

- <span id="u64-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u64_ube`

- <span id="u64-ube-rem-type-output"></span>`type Output = u64`

- <span id="u64-ube-rem"></span>`fn rem(self, other: u64) -> <Self as >::Output`

##### `impl RemAssign for u64_ube`

- <span id="u64-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u64)`

##### `impl Shl for u64_ube`

- <span id="u64-ube-shl-type-output"></span>`type Output = u64`

- <span id="u64-ube-shl"></span>`fn shl(self, other: u64) -> <Self as >::Output`

##### `impl ShlAssign for u64_ube`

- <span id="u64-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u64)`

##### `impl Shr for u64_ube`

- <span id="u64-ube-shr-type-output"></span>`type Output = u64`

- <span id="u64-ube-shr"></span>`fn shr(self, other: u64) -> <Self as >::Output`

##### `impl ShrAssign for u64_ube`

- <span id="u64-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u64)`

##### `impl Sub for u64_ube`

- <span id="u64-ube-sub-type-output"></span>`type Output = u64`

- <span id="u64-ube-sub"></span>`fn sub(self, other: u64) -> <Self as >::Output`

##### `impl SubAssign for u64_ube`

- <span id="u64-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u64)`

##### `impl Sum for u64_ube`

- <span id="u64-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u64_ube`

- <span id="u64-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u64_ube`

- <span id="u64-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u64_ube`

- <span id="u64-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u128_ule`

```rust
struct u128_ule(u128);
```

A little-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="u128-ule-from-native"></span>`const fn from_native(value: u128) -> Self`

  Returns a `u128_ule` containing `value`.

- <span id="u128-ule-to-native"></span>`const fn to_native(self) -> u128`

  Returns a `u128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u128_ule`

- <span id="u128-ule-add-type-output"></span>`type Output = u128`

- <span id="u128-ule-add"></span>`fn add(self, other: u128) -> <Self as >::Output`

##### `impl AddAssign for u128_ule`

- <span id="u128-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: u128)`

##### `impl Binary for u128_ule`

- <span id="u128-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u128_ule`

- <span id="u128-ule-bitand-type-output"></span>`type Output = u128`

- <span id="u128-ule-bitand"></span>`fn bitand(self, other: u128) -> <Self as >::Output`

##### `impl BitAndAssign for u128_ule`

- <span id="u128-ule-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u128)`

##### `impl BitOr for u128_ule`

- <span id="u128-ule-bitor-type-output"></span>`type Output = u128`

- <span id="u128-ule-bitor"></span>`fn bitor(self, other: u128) -> <Self as >::Output`

##### `impl BitOrAssign for u128_ule`

- <span id="u128-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u128)`

##### `impl BitXor for u128_ule`

- <span id="u128-ule-bitxor-type-output"></span>`type Output = u128`

- <span id="u128-ule-bitxor"></span>`fn bitxor(self, other: u128) -> <Self as >::Output`

##### `impl BitXorAssign for u128_ule`

- <span id="u128-ule-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u128)`

##### `impl<C> CheckBytes for u128_ule`

- <span id="u128-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u128_ule`

- <span id="u128-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u128_ule`

##### `impl Debug for u128_ule`

- <span id="u128-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u128_ule`

- <span id="u128-ule-default"></span>`fn default() -> Self`

##### `impl Display for u128_ule`

- <span id="u128-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u128_ule`

- <span id="u128-ule-div-type-output"></span>`type Output = u128`

- <span id="u128-ule-div"></span>`fn div(self, other: u128) -> <Self as >::Output`

##### `impl DivAssign for u128_ule`

- <span id="u128-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: u128)`

##### `impl Eq for u128_ule`

##### `impl Hash for u128_ule`

- <span id="u128-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u128_ule`

- <span id="u128-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u128_ule`

- <span id="u128-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u128_ule`

- <span id="u128-ule-mul-type-output"></span>`type Output = u128`

- <span id="u128-ule-mul"></span>`fn mul(self, other: u128) -> <Self as >::Output`

##### `impl MulAssign for u128_ule`

- <span id="u128-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u128)`

##### `impl Not for u128_ule`

- <span id="u128-ule-not-type-output"></span>`type Output = <u128 as Not>::Output`

- <span id="u128-ule-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u128_ule`

- <span id="u128-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u128_ule`

- <span id="u128-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u128_ule`

- <span id="u128-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u128_ule`

- <span id="u128-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u128_ule`

- <span id="u128-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u128_ule`

- <span id="u128-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u128_ule`

- <span id="u128-ule-rem-type-output"></span>`type Output = u128`

- <span id="u128-ule-rem"></span>`fn rem(self, other: u128) -> <Self as >::Output`

##### `impl RemAssign for u128_ule`

- <span id="u128-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u128)`

##### `impl Shl for u128_ule`

- <span id="u128-ule-shl-type-output"></span>`type Output = u128`

- <span id="u128-ule-shl"></span>`fn shl(self, other: u128) -> <Self as >::Output`

##### `impl ShlAssign for u128_ule`

- <span id="u128-ule-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u128)`

##### `impl Shr for u128_ule`

- <span id="u128-ule-shr-type-output"></span>`type Output = u128`

- <span id="u128-ule-shr"></span>`fn shr(self, other: u128) -> <Self as >::Output`

##### `impl ShrAssign for u128_ule`

- <span id="u128-ule-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u128)`

##### `impl Sub for u128_ule`

- <span id="u128-ule-sub-type-output"></span>`type Output = u128`

- <span id="u128-ule-sub"></span>`fn sub(self, other: u128) -> <Self as >::Output`

##### `impl SubAssign for u128_ule`

- <span id="u128-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u128)`

##### `impl Sum for u128_ule`

- <span id="u128-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u128_ule`

- <span id="u128-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u128_ule`

- <span id="u128-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u128_ule`

- <span id="u128-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `u128_ube`

```rust
struct u128_ube(u128);
```

A big-endian unaligned `u128` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="u128-ube-from-native"></span>`const fn from_native(value: u128) -> Self`

  Returns a `u128_ube` containing `value`.

- <span id="u128-ube-to-native"></span>`const fn to_native(self) -> u128`

  Returns a `u128` with the same value as `self`.

#### Trait Implementations

##### `impl Add for u128_ube`

- <span id="u128-ube-add-type-output"></span>`type Output = u128`

- <span id="u128-ube-add"></span>`fn add(self, other: u128) -> <Self as >::Output`

##### `impl AddAssign for u128_ube`

- <span id="u128-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: u128)`

##### `impl Binary for u128_ube`

- <span id="u128-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitAnd for u128_ube`

- <span id="u128-ube-bitand-type-output"></span>`type Output = u128`

- <span id="u128-ube-bitand"></span>`fn bitand(self, other: u128) -> <Self as >::Output`

##### `impl BitAndAssign for u128_ube`

- <span id="u128-ube-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: u128)`

##### `impl BitOr for u128_ube`

- <span id="u128-ube-bitor-type-output"></span>`type Output = u128`

- <span id="u128-ube-bitor"></span>`fn bitor(self, other: u128) -> <Self as >::Output`

##### `impl BitOrAssign for u128_ube`

- <span id="u128-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: u128)`

##### `impl BitXor for u128_ube`

- <span id="u128-ube-bitxor-type-output"></span>`type Output = u128`

- <span id="u128-ube-bitxor"></span>`fn bitxor(self, other: u128) -> <Self as >::Output`

##### `impl BitXorAssign for u128_ube`

- <span id="u128-ube-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: u128)`

##### `impl<C> CheckBytes for u128_ube`

- <span id="u128-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for u128_ube`

- <span id="u128-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for u128_ube`

##### `impl Debug for u128_ube`

- <span id="u128-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for u128_ube`

- <span id="u128-ube-default"></span>`fn default() -> Self`

##### `impl Display for u128_ube`

- <span id="u128-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for u128_ube`

- <span id="u128-ube-div-type-output"></span>`type Output = u128`

- <span id="u128-ube-div"></span>`fn div(self, other: u128) -> <Self as >::Output`

##### `impl DivAssign for u128_ube`

- <span id="u128-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: u128)`

##### `impl Eq for u128_ube`

##### `impl Hash for u128_ube`

- <span id="u128-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerExp for u128_ube`

- <span id="u128-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl LowerHex for u128_ube`

- <span id="u128-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for u128_ube`

- <span id="u128-ube-mul-type-output"></span>`type Output = u128`

- <span id="u128-ube-mul"></span>`fn mul(self, other: u128) -> <Self as >::Output`

##### `impl MulAssign for u128_ube`

- <span id="u128-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: u128)`

##### `impl Not for u128_ube`

- <span id="u128-ube-not-type-output"></span>`type Output = <u128 as Not>::Output`

- <span id="u128-ube-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Octal for u128_ube`

- <span id="u128-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for u128_ube`

- <span id="u128-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for u128_ube`

- <span id="u128-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for u128_ube`

- <span id="u128-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for u128_ube`

- <span id="u128-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for u128_ube`

- <span id="u128-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for u128_ube`

- <span id="u128-ube-rem-type-output"></span>`type Output = u128`

- <span id="u128-ube-rem"></span>`fn rem(self, other: u128) -> <Self as >::Output`

##### `impl RemAssign for u128_ube`

- <span id="u128-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: u128)`

##### `impl Shl for u128_ube`

- <span id="u128-ube-shl-type-output"></span>`type Output = u128`

- <span id="u128-ube-shl"></span>`fn shl(self, other: u128) -> <Self as >::Output`

##### `impl ShlAssign for u128_ube`

- <span id="u128-ube-shlassign-shl-assign"></span>`fn shl_assign(&mut self, other: u128)`

##### `impl Shr for u128_ube`

- <span id="u128-ube-shr-type-output"></span>`type Output = u128`

- <span id="u128-ube-shr"></span>`fn shr(self, other: u128) -> <Self as >::Output`

##### `impl ShrAssign for u128_ube`

- <span id="u128-ube-shrassign-shr-assign"></span>`fn shr_assign(&mut self, other: u128)`

##### `impl Sub for u128_ube`

- <span id="u128-ube-sub-type-output"></span>`type Output = u128`

- <span id="u128-ube-sub"></span>`fn sub(self, other: u128) -> <Self as >::Output`

##### `impl SubAssign for u128_ube`

- <span id="u128-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: u128)`

##### `impl Sum for u128_ube`

- <span id="u128-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for u128_ube`

- <span id="u128-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for u128_ube`

- <span id="u128-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl UpperHex for u128_ube`

- <span id="u128-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f32_ule`

```rust
struct f32_ule(f32);
```

A little-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="f32-ule-from-native"></span>`const fn from_native(value: f32) -> Self`

  Returns a `f32_ule` containing `value`.

- <span id="f32-ule-to-native"></span>`const fn to_native(self) -> f32`

  Returns a `f32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f32_ule`

- <span id="f32-ule-add-type-output"></span>`type Output = f32`

- <span id="f32-ule-add"></span>`fn add(self, other: f32) -> <Self as >::Output`

##### `impl AddAssign for f32_ule`

- <span id="f32-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: f32)`

##### `impl<C> CheckBytes for f32_ule`

- <span id="f32-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f32_ule`

- <span id="f32-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f32_ule`

##### `impl Debug for f32_ule`

- <span id="f32-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f32_ule`

- <span id="f32-ule-default"></span>`fn default() -> Self`

##### `impl Display for f32_ule`

- <span id="f32-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f32_ule`

- <span id="f32-ule-div-type-output"></span>`type Output = f32`

- <span id="f32-ule-div"></span>`fn div(self, other: f32) -> <Self as >::Output`

##### `impl DivAssign for f32_ule`

- <span id="f32-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: f32)`

##### `impl Eq for f32_ule`

##### `impl LowerExp for f32_ule`

- <span id="f32-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f32_ule`

- <span id="f32-ule-mul-type-output"></span>`type Output = f32`

- <span id="f32-ule-mul"></span>`fn mul(self, other: f32) -> <Self as >::Output`

##### `impl MulAssign for f32_ule`

- <span id="f32-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f32)`

##### `impl Neg for f32_ule`

- <span id="f32-ule-neg-type-output"></span>`type Output = <f32 as Neg>::Output`

- <span id="f32-ule-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f32_ule`

- <span id="f32-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f32_ule`

- <span id="f32-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f32_ule`

- <span id="f32-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f32_ule`

- <span id="f32-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f32_ule`

- <span id="f32-ule-rem-type-output"></span>`type Output = f32`

- <span id="f32-ule-rem"></span>`fn rem(self, other: f32) -> <Self as >::Output`

##### `impl RemAssign for f32_ule`

- <span id="f32-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f32)`

##### `impl Sub for f32_ule`

- <span id="f32-ule-sub-type-output"></span>`type Output = f32`

- <span id="f32-ule-sub"></span>`fn sub(self, other: f32) -> <Self as >::Output`

##### `impl SubAssign for f32_ule`

- <span id="f32-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f32)`

##### `impl Sum for f32_ule`

- <span id="f32-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f32_ule`

- <span id="f32-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f32_ule`

- <span id="f32-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f32_ube`

```rust
struct f32_ube(f32);
```

A big-endian unaligned `f32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="f32-ube-from-native"></span>`const fn from_native(value: f32) -> Self`

  Returns a `f32_ube` containing `value`.

- <span id="f32-ube-to-native"></span>`const fn to_native(self) -> f32`

  Returns a `f32` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f32_ube`

- <span id="f32-ube-add-type-output"></span>`type Output = f32`

- <span id="f32-ube-add"></span>`fn add(self, other: f32) -> <Self as >::Output`

##### `impl AddAssign for f32_ube`

- <span id="f32-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: f32)`

##### `impl<C> CheckBytes for f32_ube`

- <span id="f32-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f32_ube`

- <span id="f32-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f32_ube`

##### `impl Debug for f32_ube`

- <span id="f32-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f32_ube`

- <span id="f32-ube-default"></span>`fn default() -> Self`

##### `impl Display for f32_ube`

- <span id="f32-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f32_ube`

- <span id="f32-ube-div-type-output"></span>`type Output = f32`

- <span id="f32-ube-div"></span>`fn div(self, other: f32) -> <Self as >::Output`

##### `impl DivAssign for f32_ube`

- <span id="f32-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: f32)`

##### `impl Eq for f32_ube`

##### `impl LowerExp for f32_ube`

- <span id="f32-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f32_ube`

- <span id="f32-ube-mul-type-output"></span>`type Output = f32`

- <span id="f32-ube-mul"></span>`fn mul(self, other: f32) -> <Self as >::Output`

##### `impl MulAssign for f32_ube`

- <span id="f32-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f32)`

##### `impl Neg for f32_ube`

- <span id="f32-ube-neg-type-output"></span>`type Output = <f32 as Neg>::Output`

- <span id="f32-ube-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f32_ube`

- <span id="f32-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f32_ube`

- <span id="f32-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f32_ube`

- <span id="f32-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f32_ube`

- <span id="f32-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f32_ube`

- <span id="f32-ube-rem-type-output"></span>`type Output = f32`

- <span id="f32-ube-rem"></span>`fn rem(self, other: f32) -> <Self as >::Output`

##### `impl RemAssign for f32_ube`

- <span id="f32-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f32)`

##### `impl Sub for f32_ube`

- <span id="f32-ube-sub-type-output"></span>`type Output = f32`

- <span id="f32-ube-sub"></span>`fn sub(self, other: f32) -> <Self as >::Output`

##### `impl SubAssign for f32_ube`

- <span id="f32-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f32)`

##### `impl Sum for f32_ube`

- <span id="f32-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f32_ube`

- <span id="f32-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f32_ube`

- <span id="f32-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f64_ule`

```rust
struct f64_ule(f64);
```

A little-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="f64-ule-from-native"></span>`const fn from_native(value: f64) -> Self`

  Returns a `f64_ule` containing `value`.

- <span id="f64-ule-to-native"></span>`const fn to_native(self) -> f64`

  Returns a `f64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f64_ule`

- <span id="f64-ule-add-type-output"></span>`type Output = f64`

- <span id="f64-ule-add"></span>`fn add(self, other: f64) -> <Self as >::Output`

##### `impl AddAssign for f64_ule`

- <span id="f64-ule-addassign-add-assign"></span>`fn add_assign(&mut self, other: f64)`

##### `impl<C> CheckBytes for f64_ule`

- <span id="f64-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f64_ule`

- <span id="f64-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f64_ule`

##### `impl Debug for f64_ule`

- <span id="f64-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f64_ule`

- <span id="f64-ule-default"></span>`fn default() -> Self`

##### `impl Display for f64_ule`

- <span id="f64-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f64_ule`

- <span id="f64-ule-div-type-output"></span>`type Output = f64`

- <span id="f64-ule-div"></span>`fn div(self, other: f64) -> <Self as >::Output`

##### `impl DivAssign for f64_ule`

- <span id="f64-ule-divassign-div-assign"></span>`fn div_assign(&mut self, other: f64)`

##### `impl Eq for f64_ule`

##### `impl LowerExp for f64_ule`

- <span id="f64-ule-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f64_ule`

- <span id="f64-ule-mul-type-output"></span>`type Output = f64`

- <span id="f64-ule-mul"></span>`fn mul(self, other: f64) -> <Self as >::Output`

##### `impl MulAssign for f64_ule`

- <span id="f64-ule-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f64)`

##### `impl Neg for f64_ule`

- <span id="f64-ule-neg-type-output"></span>`type Output = <f64 as Neg>::Output`

- <span id="f64-ule-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f64_ule`

- <span id="f64-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f64_ule`

- <span id="f64-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f64_ule`

- <span id="f64-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f64_ule`

- <span id="f64-ule-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f64_ule`

- <span id="f64-ule-rem-type-output"></span>`type Output = f64`

- <span id="f64-ule-rem"></span>`fn rem(self, other: f64) -> <Self as >::Output`

##### `impl RemAssign for f64_ule`

- <span id="f64-ule-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f64)`

##### `impl Sub for f64_ule`

- <span id="f64-ule-sub-type-output"></span>`type Output = f64`

- <span id="f64-ule-sub"></span>`fn sub(self, other: f64) -> <Self as >::Output`

##### `impl SubAssign for f64_ule`

- <span id="f64-ule-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f64)`

##### `impl Sum for f64_ule`

- <span id="f64-ule-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f64_ule`

- <span id="f64-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f64_ule`

- <span id="f64-ule-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `f64_ube`

```rust
struct f64_ube(f64);
```

A big-endian unaligned `f64` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="f64-ube-from-native"></span>`const fn from_native(value: f64) -> Self`

  Returns a `f64_ube` containing `value`.

- <span id="f64-ube-to-native"></span>`const fn to_native(self) -> f64`

  Returns a `f64` with the same value as `self`.

#### Trait Implementations

##### `impl Add for f64_ube`

- <span id="f64-ube-add-type-output"></span>`type Output = f64`

- <span id="f64-ube-add"></span>`fn add(self, other: f64) -> <Self as >::Output`

##### `impl AddAssign for f64_ube`

- <span id="f64-ube-addassign-add-assign"></span>`fn add_assign(&mut self, other: f64)`

##### `impl<C> CheckBytes for f64_ube`

- <span id="f64-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for f64_ube`

- <span id="f64-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for f64_ube`

##### `impl Debug for f64_ube`

- <span id="f64-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for f64_ube`

- <span id="f64-ube-default"></span>`fn default() -> Self`

##### `impl Display for f64_ube`

- <span id="f64-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Div for f64_ube`

- <span id="f64-ube-div-type-output"></span>`type Output = f64`

- <span id="f64-ube-div"></span>`fn div(self, other: f64) -> <Self as >::Output`

##### `impl DivAssign for f64_ube`

- <span id="f64-ube-divassign-div-assign"></span>`fn div_assign(&mut self, other: f64)`

##### `impl Eq for f64_ube`

##### `impl LowerExp for f64_ube`

- <span id="f64-ube-lowerexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Mul for f64_ube`

- <span id="f64-ube-mul-type-output"></span>`type Output = f64`

- <span id="f64-ube-mul"></span>`fn mul(self, other: f64) -> <Self as >::Output`

##### `impl MulAssign for f64_ube`

- <span id="f64-ube-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: f64)`

##### `impl Neg for f64_ube`

- <span id="f64-ube-neg-type-output"></span>`type Output = <f64 as Neg>::Output`

- <span id="f64-ube-neg"></span>`fn neg(self) -> <Self as >::Output`

##### `impl PartialEq for f64_ube`

- <span id="f64-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for f64_ube`

- <span id="f64-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for f64_ube`

- <span id="f64-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl Product for f64_ube`

- <span id="f64-ube-product"></span>`fn product<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl Rem for f64_ube`

- <span id="f64-ube-rem-type-output"></span>`type Output = f64`

- <span id="f64-ube-rem"></span>`fn rem(self, other: f64) -> <Self as >::Output`

##### `impl RemAssign for f64_ube`

- <span id="f64-ube-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: f64)`

##### `impl Sub for f64_ube`

- <span id="f64-ube-sub-type-output"></span>`type Output = f64`

- <span id="f64-ube-sub"></span>`fn sub(self, other: f64) -> <Self as >::Output`

##### `impl SubAssign for f64_ube`

- <span id="f64-ube-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: f64)`

##### `impl Sum for f64_ube`

- <span id="f64-ube-sum"></span>`fn sum<I: Iterator<Item = Self>>(iter: I) -> Self`

##### `impl ToString for f64_ube`

- <span id="f64-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperExp for f64_ube`

- <span id="f64-ube-upperexp-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `char_ule`

```rust
struct char_ule(u32);
```

A little-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="char-ule-from-native"></span>`const fn from_native(value: char) -> Self`

  Returns a `char_ule` containing `value`.

- <span id="char-ule-to-native"></span>`const fn to_native(self) -> char`

  Returns a `$prim` with the same value as `self`.

#### Trait Implementations

##### `impl<C> CheckBytes for char_ule`

- <span id="char-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for char_ule`

- <span id="char-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for char_ule`

##### `impl Debug for char_ule`

- <span id="char-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for char_ule`

- <span id="char-ule-default"></span>`fn default() -> Self`

##### `impl Display for char_ule`

- <span id="char-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for char_ule`

##### `impl Hash for char_ule`

- <span id="char-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl Ord for char_ule`

- <span id="char-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for char_ule`

- <span id="char-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for char_ule`

- <span id="char-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for char_ule`

- <span id="char-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for char_ule`

- <span id="char-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

### `char_ube`

```rust
struct char_ube(u32);
```

A big-endian unaligned `u32` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="char-ube-from-native"></span>`const fn from_native(value: char) -> Self`

  Returns a `char_ube` containing `value`.

- <span id="char-ube-to-native"></span>`const fn to_native(self) -> char`

  Returns a `$prim` with the same value as `self`.

#### Trait Implementations

##### `impl<C> CheckBytes for char_ube`

- <span id="char-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for char_ube`

- <span id="char-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for char_ube`

##### `impl Debug for char_ube`

- <span id="char-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default for char_ube`

- <span id="char-ube-default"></span>`fn default() -> Self`

##### `impl Display for char_ube`

- <span id="char-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for char_ube`

##### `impl Hash for char_ube`

- <span id="char-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl Ord for char_ube`

- <span id="char-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for char_ube`

- <span id="char-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for char_ube`

- <span id="char-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for char_ube`

- <span id="char-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for char_ube`

- <span id="char-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

### `NonZeroI16_ule`

```rust
struct NonZeroI16_ule(core::num::NonZeroI16);
```

A little-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="nonzeroi16-ule-new"></span>`const fn new(value: i16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi16-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: i16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi16-ule-get"></span>`const fn get(self) -> i16`

  Returns the value as a primitive type.

- <span id="nonzeroi16-ule-from-native"></span>`const fn from_native(value: NonZeroI16) -> Self`

  Returns a `NonZeroI16_ule` containing `value`.

- <span id="nonzeroi16-ule-to-native"></span>`const fn to_native(self) -> NonZeroI16`

  Returns a `NonZeroI16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI16_ule`

- <span id="nonzeroi16-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI16_ule`

- <span id="nonzeroi16-ule-bitor-type-output"></span>`type Output = NonZero<i16>`

- <span id="nonzeroi16-ule-bitor"></span>`fn bitor(self, other: NonZeroI16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI16_ule`

- <span id="nonzeroi16-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI16)`

##### `impl<C> CheckBytes for NonZeroI16_ule`

- <span id="nonzeroi16-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI16_ule`

- <span id="nonzeroi16-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI16_ule`

##### `impl Debug for NonZeroI16_ule`

- <span id="nonzeroi16-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI16_ule`

- <span id="nonzeroi16-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI16_ule`

##### `impl Hash for NonZeroI16_ule`

- <span id="nonzeroi16-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI16_ule`

- <span id="nonzeroi16-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI16_ule`

- <span id="nonzeroi16-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI16_ule`

- <span id="nonzeroi16-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI16_ule`

- <span id="nonzeroi16-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI16_ule`

- <span id="nonzeroi16-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI16_ule`

- <span id="nonzeroi16-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI16_ule`

- <span id="nonzeroi16-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI16_ule`

- <span id="nonzeroi16-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI16_ube`

```rust
struct NonZeroI16_ube(core::num::NonZeroI16);
```

A big-endian unaligned `NonZeroI16` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="nonzeroi16-ube-new"></span>`const fn new(value: i16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi16-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: i16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi16-ube-get"></span>`const fn get(self) -> i16`

  Returns the value as a primitive type.

- <span id="nonzeroi16-ube-from-native"></span>`const fn from_native(value: NonZeroI16) -> Self`

  Returns a `NonZeroI16_ube` containing `value`.

- <span id="nonzeroi16-ube-to-native"></span>`const fn to_native(self) -> NonZeroI16`

  Returns a `NonZeroI16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI16_ube`

- <span id="nonzeroi16-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI16_ube`

- <span id="nonzeroi16-ube-bitor-type-output"></span>`type Output = NonZero<i16>`

- <span id="nonzeroi16-ube-bitor"></span>`fn bitor(self, other: NonZeroI16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI16_ube`

- <span id="nonzeroi16-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI16)`

##### `impl<C> CheckBytes for NonZeroI16_ube`

- <span id="nonzeroi16-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI16_ube`

- <span id="nonzeroi16-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI16_ube`

##### `impl Debug for NonZeroI16_ube`

- <span id="nonzeroi16-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI16_ube`

- <span id="nonzeroi16-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI16_ube`

##### `impl Hash for NonZeroI16_ube`

- <span id="nonzeroi16-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI16_ube`

- <span id="nonzeroi16-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI16_ube`

- <span id="nonzeroi16-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI16_ube`

- <span id="nonzeroi16-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI16_ube`

- <span id="nonzeroi16-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI16_ube`

- <span id="nonzeroi16-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI16_ube`

- <span id="nonzeroi16-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI16_ube`

- <span id="nonzeroi16-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI16_ube`

- <span id="nonzeroi16-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI32_ule`

```rust
struct NonZeroI32_ule(core::num::NonZeroI32);
```

A little-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="nonzeroi32-ule-new"></span>`const fn new(value: i32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi32-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: i32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi32-ule-get"></span>`const fn get(self) -> i32`

  Returns the value as a primitive type.

- <span id="nonzeroi32-ule-from-native"></span>`const fn from_native(value: NonZeroI32) -> Self`

  Returns a `NonZeroI32_ule` containing `value`.

- <span id="nonzeroi32-ule-to-native"></span>`const fn to_native(self) -> NonZeroI32`

  Returns a `NonZeroI32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI32_ule`

- <span id="nonzeroi32-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI32_ule`

- <span id="nonzeroi32-ule-bitor-type-output"></span>`type Output = NonZero<i32>`

- <span id="nonzeroi32-ule-bitor"></span>`fn bitor(self, other: NonZeroI32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI32_ule`

- <span id="nonzeroi32-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI32)`

##### `impl<C> CheckBytes for NonZeroI32_ule`

- <span id="nonzeroi32-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI32_ule`

- <span id="nonzeroi32-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI32_ule`

##### `impl Debug for NonZeroI32_ule`

- <span id="nonzeroi32-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI32_ule`

- <span id="nonzeroi32-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI32_ule`

##### `impl Hash for NonZeroI32_ule`

- <span id="nonzeroi32-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI32_ule`

- <span id="nonzeroi32-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI32_ule`

- <span id="nonzeroi32-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI32_ule`

- <span id="nonzeroi32-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI32_ule`

- <span id="nonzeroi32-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI32_ule`

- <span id="nonzeroi32-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI32_ule`

- <span id="nonzeroi32-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI32_ule`

- <span id="nonzeroi32-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI32_ule`

- <span id="nonzeroi32-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI32_ube`

```rust
struct NonZeroI32_ube(core::num::NonZeroI32);
```

A big-endian unaligned `NonZeroI32` with a guaranteed size of `2` and alignment of `1`.

#### Implementations

- <span id="nonzeroi32-ube-new"></span>`const fn new(value: i32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi32-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: i32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi32-ube-get"></span>`const fn get(self) -> i32`

  Returns the value as a primitive type.

- <span id="nonzeroi32-ube-from-native"></span>`const fn from_native(value: NonZeroI32) -> Self`

  Returns a `NonZeroI32_ube` containing `value`.

- <span id="nonzeroi32-ube-to-native"></span>`const fn to_native(self) -> NonZeroI32`

  Returns a `NonZeroI32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI32_ube`

- <span id="nonzeroi32-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI32_ube`

- <span id="nonzeroi32-ube-bitor-type-output"></span>`type Output = NonZero<i32>`

- <span id="nonzeroi32-ube-bitor"></span>`fn bitor(self, other: NonZeroI32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI32_ube`

- <span id="nonzeroi32-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI32)`

##### `impl<C> CheckBytes for NonZeroI32_ube`

- <span id="nonzeroi32-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI32_ube`

- <span id="nonzeroi32-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI32_ube`

##### `impl Debug for NonZeroI32_ube`

- <span id="nonzeroi32-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI32_ube`

- <span id="nonzeroi32-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI32_ube`

##### `impl Hash for NonZeroI32_ube`

- <span id="nonzeroi32-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI32_ube`

- <span id="nonzeroi32-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI32_ube`

- <span id="nonzeroi32-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI32_ube`

- <span id="nonzeroi32-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI32_ube`

- <span id="nonzeroi32-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI32_ube`

- <span id="nonzeroi32-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI32_ube`

- <span id="nonzeroi32-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI32_ube`

- <span id="nonzeroi32-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI32_ube`

- <span id="nonzeroi32-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI64_ule`

```rust
struct NonZeroI64_ule(core::num::NonZeroI64);
```

A little-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="nonzeroi64-ule-new"></span>`const fn new(value: i64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi64-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: i64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi64-ule-get"></span>`const fn get(self) -> i64`

  Returns the value as a primitive type.

- <span id="nonzeroi64-ule-from-native"></span>`const fn from_native(value: NonZeroI64) -> Self`

  Returns a `NonZeroI64_ule` containing `value`.

- <span id="nonzeroi64-ule-to-native"></span>`const fn to_native(self) -> NonZeroI64`

  Returns a `NonZeroI64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI64_ule`

- <span id="nonzeroi64-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI64_ule`

- <span id="nonzeroi64-ule-bitor-type-output"></span>`type Output = NonZero<i64>`

- <span id="nonzeroi64-ule-bitor"></span>`fn bitor(self, other: NonZeroI64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI64_ule`

- <span id="nonzeroi64-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI64)`

##### `impl<C> CheckBytes for NonZeroI64_ule`

- <span id="nonzeroi64-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI64_ule`

- <span id="nonzeroi64-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI64_ule`

##### `impl Debug for NonZeroI64_ule`

- <span id="nonzeroi64-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI64_ule`

- <span id="nonzeroi64-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI64_ule`

##### `impl Hash for NonZeroI64_ule`

- <span id="nonzeroi64-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI64_ule`

- <span id="nonzeroi64-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI64_ule`

- <span id="nonzeroi64-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI64_ule`

- <span id="nonzeroi64-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI64_ule`

- <span id="nonzeroi64-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI64_ule`

- <span id="nonzeroi64-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI64_ule`

- <span id="nonzeroi64-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI64_ule`

- <span id="nonzeroi64-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI64_ule`

- <span id="nonzeroi64-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI64_ube`

```rust
struct NonZeroI64_ube(core::num::NonZeroI64);
```

A big-endian unaligned `NonZeroI64` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="nonzeroi64-ube-new"></span>`const fn new(value: i64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi64-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: i64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi64-ube-get"></span>`const fn get(self) -> i64`

  Returns the value as a primitive type.

- <span id="nonzeroi64-ube-from-native"></span>`const fn from_native(value: NonZeroI64) -> Self`

  Returns a `NonZeroI64_ube` containing `value`.

- <span id="nonzeroi64-ube-to-native"></span>`const fn to_native(self) -> NonZeroI64`

  Returns a `NonZeroI64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI64_ube`

- <span id="nonzeroi64-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI64_ube`

- <span id="nonzeroi64-ube-bitor-type-output"></span>`type Output = NonZero<i64>`

- <span id="nonzeroi64-ube-bitor"></span>`fn bitor(self, other: NonZeroI64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI64_ube`

- <span id="nonzeroi64-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI64)`

##### `impl<C> CheckBytes for NonZeroI64_ube`

- <span id="nonzeroi64-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI64_ube`

- <span id="nonzeroi64-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI64_ube`

##### `impl Debug for NonZeroI64_ube`

- <span id="nonzeroi64-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI64_ube`

- <span id="nonzeroi64-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI64_ube`

##### `impl Hash for NonZeroI64_ube`

- <span id="nonzeroi64-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI64_ube`

- <span id="nonzeroi64-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI64_ube`

- <span id="nonzeroi64-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI64_ube`

- <span id="nonzeroi64-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI64_ube`

- <span id="nonzeroi64-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI64_ube`

- <span id="nonzeroi64-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI64_ube`

- <span id="nonzeroi64-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI64_ube`

- <span id="nonzeroi64-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI64_ube`

- <span id="nonzeroi64-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI128_ule`

```rust
struct NonZeroI128_ule(core::num::NonZeroI128);
```

A little-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="nonzeroi128-ule-new"></span>`const fn new(value: i128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi128-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: i128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi128-ule-get"></span>`const fn get(self) -> i128`

  Returns the value as a primitive type.

- <span id="nonzeroi128-ule-from-native"></span>`const fn from_native(value: NonZeroI128) -> Self`

  Returns a `NonZeroI128_ule` containing `value`.

- <span id="nonzeroi128-ule-to-native"></span>`const fn to_native(self) -> NonZeroI128`

  Returns a `NonZeroI128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI128_ule`

- <span id="nonzeroi128-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI128_ule`

- <span id="nonzeroi128-ule-bitor-type-output"></span>`type Output = NonZero<i128>`

- <span id="nonzeroi128-ule-bitor"></span>`fn bitor(self, other: NonZeroI128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI128_ule`

- <span id="nonzeroi128-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI128)`

##### `impl<C> CheckBytes for NonZeroI128_ule`

- <span id="nonzeroi128-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI128_ule`

- <span id="nonzeroi128-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI128_ule`

##### `impl Debug for NonZeroI128_ule`

- <span id="nonzeroi128-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI128_ule`

- <span id="nonzeroi128-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI128_ule`

##### `impl Hash for NonZeroI128_ule`

- <span id="nonzeroi128-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI128_ule`

- <span id="nonzeroi128-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI128_ule`

- <span id="nonzeroi128-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI128_ule`

- <span id="nonzeroi128-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI128_ule`

- <span id="nonzeroi128-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI128_ule`

- <span id="nonzeroi128-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI128_ule`

- <span id="nonzeroi128-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI128_ule`

- <span id="nonzeroi128-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI128_ule`

- <span id="nonzeroi128-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroI128_ube`

```rust
struct NonZeroI128_ube(core::num::NonZeroI128);
```

A big-endian unaligned `NonZeroI128` with a guaranteed size of `4` and alignment of `1`.

#### Implementations

- <span id="nonzeroi128-ube-new"></span>`const fn new(value: i128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzeroi128-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: i128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzeroi128-ube-get"></span>`const fn get(self) -> i128`

  Returns the value as a primitive type.

- <span id="nonzeroi128-ube-from-native"></span>`const fn from_native(value: NonZeroI128) -> Self`

  Returns a `NonZeroI128_ube` containing `value`.

- <span id="nonzeroi128-ube-to-native"></span>`const fn to_native(self) -> NonZeroI128`

  Returns a `NonZeroI128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroI128_ube`

- <span id="nonzeroi128-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroI128_ube`

- <span id="nonzeroi128-ube-bitor-type-output"></span>`type Output = NonZero<i128>`

- <span id="nonzeroi128-ube-bitor"></span>`fn bitor(self, other: NonZeroI128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroI128_ube`

- <span id="nonzeroi128-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroI128)`

##### `impl<C> CheckBytes for NonZeroI128_ube`

- <span id="nonzeroi128-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroI128_ube`

- <span id="nonzeroi128-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroI128_ube`

##### `impl Debug for NonZeroI128_ube`

- <span id="nonzeroi128-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroI128_ube`

- <span id="nonzeroi128-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroI128_ube`

##### `impl Hash for NonZeroI128_ube`

- <span id="nonzeroi128-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroI128_ube`

- <span id="nonzeroi128-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroI128_ube`

- <span id="nonzeroi128-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroI128_ube`

- <span id="nonzeroi128-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroI128_ube`

- <span id="nonzeroi128-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroI128_ube`

- <span id="nonzeroi128-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroI128_ube`

- <span id="nonzeroi128-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroI128_ube`

- <span id="nonzeroi128-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroI128_ube`

- <span id="nonzeroi128-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU16_ule`

```rust
struct NonZeroU16_ule(core::num::NonZeroU16);
```

A little-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="nonzerou16-ule-new"></span>`const fn new(value: u16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou16-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: u16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou16-ule-get"></span>`const fn get(self) -> u16`

  Returns the value as a primitive type.

- <span id="nonzerou16-ule-from-native"></span>`const fn from_native(value: NonZeroU16) -> Self`

  Returns a `NonZeroU16_ule` containing `value`.

- <span id="nonzerou16-ule-to-native"></span>`const fn to_native(self) -> NonZeroU16`

  Returns a `NonZeroU16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU16_ule`

- <span id="nonzerou16-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU16_ule`

- <span id="nonzerou16-ule-bitor-type-output"></span>`type Output = NonZero<u16>`

- <span id="nonzerou16-ule-bitor"></span>`fn bitor(self, other: NonZeroU16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU16_ule`

- <span id="nonzerou16-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU16)`

##### `impl<C> CheckBytes for NonZeroU16_ule`

- <span id="nonzerou16-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU16_ule`

- <span id="nonzerou16-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU16_ule`

##### `impl Debug for NonZeroU16_ule`

- <span id="nonzerou16-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU16_ule`

- <span id="nonzerou16-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU16_ule`

##### `impl Hash for NonZeroU16_ule`

- <span id="nonzerou16-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU16_ule`

- <span id="nonzerou16-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU16_ule`

- <span id="nonzerou16-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU16_ule`

- <span id="nonzerou16-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU16_ule`

- <span id="nonzerou16-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU16_ule`

- <span id="nonzerou16-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU16_ule`

- <span id="nonzerou16-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU16_ule`

- <span id="nonzerou16-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU16_ule`

- <span id="nonzerou16-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU16_ube`

```rust
struct NonZeroU16_ube(core::num::NonZeroU16);
```

A big-endian unaligned `NonZeroU16` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="nonzerou16-ube-new"></span>`const fn new(value: u16) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou16-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: u16) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou16-ube-get"></span>`const fn get(self) -> u16`

  Returns the value as a primitive type.

- <span id="nonzerou16-ube-from-native"></span>`const fn from_native(value: NonZeroU16) -> Self`

  Returns a `NonZeroU16_ube` containing `value`.

- <span id="nonzerou16-ube-to-native"></span>`const fn to_native(self) -> NonZeroU16`

  Returns a `NonZeroU16` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU16_ube`

- <span id="nonzerou16-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU16_ube`

- <span id="nonzerou16-ube-bitor-type-output"></span>`type Output = NonZero<u16>`

- <span id="nonzerou16-ube-bitor"></span>`fn bitor(self, other: NonZeroU16) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU16_ube`

- <span id="nonzerou16-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU16)`

##### `impl<C> CheckBytes for NonZeroU16_ube`

- <span id="nonzerou16-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU16_ube`

- <span id="nonzerou16-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU16_ube`

##### `impl Debug for NonZeroU16_ube`

- <span id="nonzerou16-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU16_ube`

- <span id="nonzerou16-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU16_ube`

##### `impl Hash for NonZeroU16_ube`

- <span id="nonzerou16-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU16_ube`

- <span id="nonzerou16-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU16_ube`

- <span id="nonzerou16-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU16_ube`

- <span id="nonzerou16-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU16_ube`

- <span id="nonzerou16-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU16_ube`

- <span id="nonzerou16-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU16_ube`

- <span id="nonzerou16-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU16_ube`

- <span id="nonzerou16-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU16_ube`

- <span id="nonzerou16-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU32_ule`

```rust
struct NonZeroU32_ule(core::num::NonZeroU32);
```

A little-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="nonzerou32-ule-new"></span>`const fn new(value: u32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou32-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: u32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou32-ule-get"></span>`const fn get(self) -> u32`

  Returns the value as a primitive type.

- <span id="nonzerou32-ule-from-native"></span>`const fn from_native(value: NonZeroU32) -> Self`

  Returns a `NonZeroU32_ule` containing `value`.

- <span id="nonzerou32-ule-to-native"></span>`const fn to_native(self) -> NonZeroU32`

  Returns a `NonZeroU32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU32_ule`

- <span id="nonzerou32-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU32_ule`

- <span id="nonzerou32-ule-bitor-type-output"></span>`type Output = NonZero<u32>`

- <span id="nonzerou32-ule-bitor"></span>`fn bitor(self, other: NonZeroU32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU32_ule`

- <span id="nonzerou32-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU32)`

##### `impl<C> CheckBytes for NonZeroU32_ule`

- <span id="nonzerou32-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU32_ule`

- <span id="nonzerou32-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU32_ule`

##### `impl Debug for NonZeroU32_ule`

- <span id="nonzerou32-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU32_ule`

- <span id="nonzerou32-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU32_ule`

##### `impl Hash for NonZeroU32_ule`

- <span id="nonzerou32-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU32_ule`

- <span id="nonzerou32-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU32_ule`

- <span id="nonzerou32-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU32_ule`

- <span id="nonzerou32-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU32_ule`

- <span id="nonzerou32-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU32_ule`

- <span id="nonzerou32-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU32_ule`

- <span id="nonzerou32-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU32_ule`

- <span id="nonzerou32-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU32_ule`

- <span id="nonzerou32-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU32_ube`

```rust
struct NonZeroU32_ube(core::num::NonZeroU32);
```

A big-endian unaligned `NonZeroU32` with a guaranteed size of `8` and alignment of `1`.

#### Implementations

- <span id="nonzerou32-ube-new"></span>`const fn new(value: u32) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou32-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: u32) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou32-ube-get"></span>`const fn get(self) -> u32`

  Returns the value as a primitive type.

- <span id="nonzerou32-ube-from-native"></span>`const fn from_native(value: NonZeroU32) -> Self`

  Returns a `NonZeroU32_ube` containing `value`.

- <span id="nonzerou32-ube-to-native"></span>`const fn to_native(self) -> NonZeroU32`

  Returns a `NonZeroU32` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU32_ube`

- <span id="nonzerou32-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU32_ube`

- <span id="nonzerou32-ube-bitor-type-output"></span>`type Output = NonZero<u32>`

- <span id="nonzerou32-ube-bitor"></span>`fn bitor(self, other: NonZeroU32) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU32_ube`

- <span id="nonzerou32-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU32)`

##### `impl<C> CheckBytes for NonZeroU32_ube`

- <span id="nonzerou32-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU32_ube`

- <span id="nonzerou32-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU32_ube`

##### `impl Debug for NonZeroU32_ube`

- <span id="nonzerou32-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU32_ube`

- <span id="nonzerou32-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU32_ube`

##### `impl Hash for NonZeroU32_ube`

- <span id="nonzerou32-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU32_ube`

- <span id="nonzerou32-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU32_ube`

- <span id="nonzerou32-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU32_ube`

- <span id="nonzerou32-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU32_ube`

- <span id="nonzerou32-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU32_ube`

- <span id="nonzerou32-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU32_ube`

- <span id="nonzerou32-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU32_ube`

- <span id="nonzerou32-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU32_ube`

- <span id="nonzerou32-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU64_ule`

```rust
struct NonZeroU64_ule(core::num::NonZeroU64);
```

A little-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="nonzerou64-ule-new"></span>`const fn new(value: u64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou64-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: u64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou64-ule-get"></span>`const fn get(self) -> u64`

  Returns the value as a primitive type.

- <span id="nonzerou64-ule-from-native"></span>`const fn from_native(value: NonZeroU64) -> Self`

  Returns a `NonZeroU64_ule` containing `value`.

- <span id="nonzerou64-ule-to-native"></span>`const fn to_native(self) -> NonZeroU64`

  Returns a `NonZeroU64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU64_ule`

- <span id="nonzerou64-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU64_ule`

- <span id="nonzerou64-ule-bitor-type-output"></span>`type Output = NonZero<u64>`

- <span id="nonzerou64-ule-bitor"></span>`fn bitor(self, other: NonZeroU64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU64_ule`

- <span id="nonzerou64-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU64)`

##### `impl<C> CheckBytes for NonZeroU64_ule`

- <span id="nonzerou64-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU64_ule`

- <span id="nonzerou64-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU64_ule`

##### `impl Debug for NonZeroU64_ule`

- <span id="nonzerou64-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU64_ule`

- <span id="nonzerou64-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU64_ule`

##### `impl Hash for NonZeroU64_ule`

- <span id="nonzerou64-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU64_ule`

- <span id="nonzerou64-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU64_ule`

- <span id="nonzerou64-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU64_ule`

- <span id="nonzerou64-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU64_ule`

- <span id="nonzerou64-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU64_ule`

- <span id="nonzerou64-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU64_ule`

- <span id="nonzerou64-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU64_ule`

- <span id="nonzerou64-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU64_ule`

- <span id="nonzerou64-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU64_ube`

```rust
struct NonZeroU64_ube(core::num::NonZeroU64);
```

A big-endian unaligned `NonZeroU64` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="nonzerou64-ube-new"></span>`const fn new(value: u64) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou64-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: u64) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou64-ube-get"></span>`const fn get(self) -> u64`

  Returns the value as a primitive type.

- <span id="nonzerou64-ube-from-native"></span>`const fn from_native(value: NonZeroU64) -> Self`

  Returns a `NonZeroU64_ube` containing `value`.

- <span id="nonzerou64-ube-to-native"></span>`const fn to_native(self) -> NonZeroU64`

  Returns a `NonZeroU64` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU64_ube`

- <span id="nonzerou64-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU64_ube`

- <span id="nonzerou64-ube-bitor-type-output"></span>`type Output = NonZero<u64>`

- <span id="nonzerou64-ube-bitor"></span>`fn bitor(self, other: NonZeroU64) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU64_ube`

- <span id="nonzerou64-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU64)`

##### `impl<C> CheckBytes for NonZeroU64_ube`

- <span id="nonzerou64-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU64_ube`

- <span id="nonzerou64-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU64_ube`

##### `impl Debug for NonZeroU64_ube`

- <span id="nonzerou64-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU64_ube`

- <span id="nonzerou64-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU64_ube`

##### `impl Hash for NonZeroU64_ube`

- <span id="nonzerou64-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU64_ube`

- <span id="nonzerou64-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU64_ube`

- <span id="nonzerou64-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU64_ube`

- <span id="nonzerou64-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU64_ube`

- <span id="nonzerou64-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU64_ube`

- <span id="nonzerou64-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU64_ube`

- <span id="nonzerou64-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU64_ube`

- <span id="nonzerou64-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU64_ube`

- <span id="nonzerou64-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU128_ule`

```rust
struct NonZeroU128_ule(core::num::NonZeroU128);
```

A little-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="nonzerou128-ule-new"></span>`const fn new(value: u128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou128-ule-new-unchecked"></span>`const unsafe fn new_unchecked(value: u128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou128-ule-get"></span>`const fn get(self) -> u128`

  Returns the value as a primitive type.

- <span id="nonzerou128-ule-from-native"></span>`const fn from_native(value: NonZeroU128) -> Self`

  Returns a `NonZeroU128_ule` containing `value`.

- <span id="nonzerou128-ule-to-native"></span>`const fn to_native(self) -> NonZeroU128`

  Returns a `NonZeroU128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU128_ule`

- <span id="nonzerou128-ule-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU128_ule`

- <span id="nonzerou128-ule-bitor-type-output"></span>`type Output = NonZero<u128>`

- <span id="nonzerou128-ule-bitor"></span>`fn bitor(self, other: NonZeroU128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU128_ule`

- <span id="nonzerou128-ule-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU128)`

##### `impl<C> CheckBytes for NonZeroU128_ule`

- <span id="nonzerou128-ule-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU128_ule`

- <span id="nonzerou128-ule-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU128_ule`

##### `impl Debug for NonZeroU128_ule`

- <span id="nonzerou128-ule-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU128_ule`

- <span id="nonzerou128-ule-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU128_ule`

##### `impl Hash for NonZeroU128_ule`

- <span id="nonzerou128-ule-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU128_ule`

- <span id="nonzerou128-ule-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU128_ule`

- <span id="nonzerou128-ule-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU128_ule`

- <span id="nonzerou128-ule-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU128_ule`

- <span id="nonzerou128-ule-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU128_ule`

- <span id="nonzerou128-ule-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU128_ule`

- <span id="nonzerou128-ule-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU128_ule`

- <span id="nonzerou128-ule-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU128_ule`

- <span id="nonzerou128-ule-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

### `NonZeroU128_ube`

```rust
struct NonZeroU128_ube(core::num::NonZeroU128);
```

A big-endian unaligned `NonZeroU128` with a guaranteed size of `16` and alignment of `1`.

#### Implementations

- <span id="nonzerou128-ube-new"></span>`const fn new(value: u128) -> Option<Self>`

  Creates a non-zero if the given value is not zero.

- <span id="nonzerou128-ube-new-unchecked"></span>`const unsafe fn new_unchecked(value: u128) -> Self`

  Creates a non-zero without checking whether it is non-zero. This

  results in undefined behavior if the value is zero.

  

  # Safety

  

  The value must not be zero.

- <span id="nonzerou128-ube-get"></span>`const fn get(self) -> u128`

  Returns the value as a primitive type.

- <span id="nonzerou128-ube-from-native"></span>`const fn from_native(value: NonZeroU128) -> Self`

  Returns a `NonZeroU128_ube` containing `value`.

- <span id="nonzerou128-ube-to-native"></span>`const fn to_native(self) -> NonZeroU128`

  Returns a `NonZeroU128` with the same value as `self`.

#### Trait Implementations

##### `impl Binary for NonZeroU128_ube`

- <span id="nonzerou128-ube-binary-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl BitOr for NonZeroU128_ube`

- <span id="nonzerou128-ube-bitor-type-output"></span>`type Output = NonZero<u128>`

- <span id="nonzerou128-ube-bitor"></span>`fn bitor(self, other: NonZeroU128) -> <Self as >::Output`

##### `impl BitOrAssign for NonZeroU128_ube`

- <span id="nonzerou128-ube-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: NonZeroU128)`

##### `impl<C> CheckBytes for NonZeroU128_ube`

- <span id="nonzerou128-ube-checkbytes-check-bytes"></span>`unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), <C as >::Error>`

##### `impl Clone for NonZeroU128_ube`

- <span id="nonzerou128-ube-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for NonZeroU128_ube`

##### `impl Debug for NonZeroU128_ube`

- <span id="nonzerou128-ube-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Display for NonZeroU128_ube`

- <span id="nonzerou128-ube-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Eq for NonZeroU128_ube`

##### `impl Hash for NonZeroU128_ube`

- <span id="nonzerou128-ube-hash"></span>`fn hash<H: core::hash::Hasher>(&self, state: &mut H)`

##### `impl LowerHex for NonZeroU128_ube`

- <span id="nonzerou128-ube-lowerhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Octal for NonZeroU128_ube`

- <span id="nonzerou128-ube-octal-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Ord for NonZeroU128_ube`

- <span id="nonzerou128-ube-ord-cmp"></span>`fn cmp(&self, other: &Self) -> ::core::cmp::Ordering`

##### `impl PartialEq for NonZeroU128_ube`

- <span id="nonzerou128-ube-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for NonZeroU128_ube`

- <span id="nonzerou128-ube-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering>`

##### `impl Pointee for NonZeroU128_ube`

- <span id="nonzerou128-ube-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl ToString for NonZeroU128_ube`

- <span id="nonzerou128-ube-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for NonZeroU128_ube`

- <span id="nonzerou128-ube-upperhex-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

## Macros

### `define_unaligned_newtype!`

### `define_unaligned_signed_integer!`

### `define_unaligned_signed_integers!`

### `define_unaligned_unsigned_integer!`

### `define_unaligned_unsigned_integers!`

### `define_unaligned_float!`

### `define_unaligned_floats!`

### `define_unaligned_char!`

### `define_unaligned_nonzero!`

### `define_unaligned_nonzeros!`

