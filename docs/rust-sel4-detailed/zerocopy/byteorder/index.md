*[zerocopy](../index.md) / [byteorder](index.md)*

---

# Module `byteorder`

Byte order-aware numeric primitives.

This module contains equivalents of the native multi-byte integer types with
no alignment requirement and supporting byte order conversions.

For each native multi-byte integer type - `u16`, `i16`, `u32`, etc - and
floating point type - `f32` and `f64` - an equivalent type is defined by
this module - [`U16`](../index.md), [`I16`](../index.md), [`U32`](../index.md), [`F32`](../index.md), [`F64`](../index.md), etc. Unlike their
native counterparts, these types have alignment 1, and take a type parameter
specifying the byte order in which the bytes are stored in memory. Each type
implements this crate's relevant conversion and marker traits.

These two properties, taken together, make these types useful for defining
data structures whose memory layout matches a wire format such as that of a
network protocol or a file format. Such formats often have multi-byte values
at offsets that do not respect the alignment requirements of the equivalent
native types, and stored in a byte order not necessarily the same as that of
the target platform.

Type aliases are provided for common byte orders in the [`big_endian`](../index.md),
[`little_endian`](../index.md), [`network_endian`](../index.md), and [`native_endian`](../index.md) submodules.
Note that network-endian is a synonym for big-endian.

# Example

One use of these types is for representing network packet formats, such as
UDP:

```rust
use zerocopy::{*, byteorder::network_endian::U16};
use zerocopy_derive::*;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
struct UdpHeader {
    src_port: U16,
    dst_port: U16,
    length: U16,
    checksum: U16,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C, packed)]
struct UdpPacket {
    header: UdpHeader,
    body: [u8],
}

impl UdpPacket {
    fn parse(bytes: &[u8]) -> Option<&UdpPacket> {
        UdpPacket::ref_from_bytes(bytes).ok()
    }
}
```

## Contents

- [Modules](#modules)
  - [`private`](#private)
  - [`f32_ext`](#f32-ext)
  - [`f64_ext`](#f64-ext)
  - [`big_endian`](#big-endian)
  - [`little_endian`](#little-endian)
  - [`network_endian`](#network-endian)
  - [`native_endian`](#native-endian)
- [Structs](#structs)
  - [`U16`](#u16)
  - [`U32`](#u32)
  - [`U64`](#u64)
  - [`U128`](#u128)
  - [`Usize`](#usize)
  - [`I16`](#i16)
  - [`I32`](#i32)
  - [`I64`](#i64)
  - [`I128`](#i128)
  - [`Isize`](#isize)
  - [`F32`](#f32)
  - [`F64`](#f64)
- [Enums](#enums)
  - [`BigEndian`](#bigendian)
  - [`LittleEndian`](#littleendian)
- [Traits](#traits)
  - [`ByteOrder`](#byteorder)
- [Type Aliases](#type-aliases)
  - [`NativeEndian`](#nativeendian)
  - [`NetworkEndian`](#networkendian)
  - [`BE`](#be)
  - [`LE`](#le)
- [Macros](#macros)
  - [`impl_fmt_trait!`](#impl-fmt-trait)
  - [`impl_fmt_traits!`](#impl-fmt-traits)
  - [`impl_ops_traits!`](#impl-ops-traits)
  - [`doc_comment!`](#doc-comment)
  - [`define_max_value_constant!`](#define-max-value-constant)
  - [`define_type!`](#define-type)
  - [`define_float_conversion!`](#define-float-conversion)
  - [`module!`](#module)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`f32_ext`](#f32-ext) | mod |  |
| [`f64_ext`](#f64-ext) | mod |  |
| [`big_endian`](#big-endian) | mod | Numeric primitives stored in big-endian byte order. |
| [`little_endian`](#little-endian) | mod | Numeric primitives stored in little-endian byte order. |
| [`network_endian`](#network-endian) | mod | Numeric primitives stored in network-endian byte order. |
| [`native_endian`](#native-endian) | mod | Numeric primitives stored in native-endian byte order. |
| [`U16`](#u16) | struct | A 16-bit unsigned integer stored in a given byte order. |
| [`U32`](#u32) | struct | A 32-bit unsigned integer stored in a given byte order. |
| [`U64`](#u64) | struct | A 64-bit unsigned integer stored in a given byte order. |
| [`U128`](#u128) | struct | A 128-bit unsigned integer stored in a given byte order. |
| [`Usize`](#usize) | struct | A word-sized unsigned integer stored in a given byte order. |
| [`I16`](#i16) | struct | A 16-bit signed integer stored in a given byte order. |
| [`I32`](#i32) | struct | A 32-bit signed integer stored in a given byte order. |
| [`I64`](#i64) | struct | A 64-bit signed integer stored in a given byte order. |
| [`I128`](#i128) | struct | A 128-bit signed integer stored in a given byte order. |
| [`Isize`](#isize) | struct | A word-sized signed integer stored in a given byte order. |
| [`F32`](#f32) | struct | A 32-bit floating point number stored in a given byte order. |
| [`F64`](#f64) | struct | A 64-bit floating point number stored in a given byte order. |
| [`BigEndian`](#bigendian) | enum | Big-endian byte order. |
| [`LittleEndian`](#littleendian) | enum | Little-endian byte order. |
| [`ByteOrder`](#byteorder) | trait | A type-level representation of byte order. |
| [`NativeEndian`](#nativeendian) | type | The endianness used by this platform. |
| [`NetworkEndian`](#networkendian) | type | The endianness used in many network protocols. |
| [`BE`](#be) | type | A type alias for [`BigEndian`]. |
| [`LE`](#le) | type | A type alias for [`LittleEndian`]. |
| [`impl_fmt_trait!`](#impl-fmt-trait) | macro |  |
| [`impl_fmt_traits!`](#impl-fmt-traits) | macro |  |
| [`impl_ops_traits!`](#impl-ops-traits) | macro |  |
| [`doc_comment!`](#doc-comment) | macro |  |
| [`define_max_value_constant!`](#define-max-value-constant) | macro |  |
| [`define_type!`](#define-type) | macro |  |
| [`define_float_conversion!`](#define-float-conversion) | macro |  |
| [`module!`](#module) | macro |  |

## Modules

- [`private`](private/index.md)
- [`f32_ext`](f32_ext/index.md)
- [`f64_ext`](f64_ext/index.md)
- [`big_endian`](big_endian/index.md) — Numeric primitives stored in
- [`little_endian`](little_endian/index.md) — Numeric primitives stored in
- [`network_endian`](network_endian/index.md) — Numeric primitives stored in
- [`native_endian`](native_endian/index.md) — Numeric primitives stored in

## Structs

### `U16<O>`

```rust
struct U16<O>([u8; 2], PhantomData<O>);
```

A 16-bit unsigned integer stored in a given byte order.

`U16` is like the native `u16` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

A `U16` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u16` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U16`
has endianness `O` and that, b) the layout of `u16` has
the platform's native endianness.

`U16` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u16-const-zero"></span>`const ZERO: U16<O>`

- <span id="u16-const-max-value"></span>`const MAX_VALUE: U16<O>`

- <span id="u16-from-bytes"></span>`const fn from_bytes(bytes: [u8; 2]) -> U16<O>` — [`U16`](../index.md#u16)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u16-to-bytes"></span>`const fn to_bytes(self) -> [u8; 2]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U16<O>`

- <span id="u16-add-type-output"></span>`type Output = U16<O>`

- <span id="u16-add"></span>`fn add(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> AddAssign for U16<O>`

- <span id="u16-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O> AsMut for U16<O>`

- <span id="u16-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 2]`

##### `impl<O> AsRef for U16<O>`

- <span id="u16-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 2]`

##### `impl<O: ByteOrder> Binary for U16<O>`

- <span id="u16-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U16<O>`

- <span id="u16-bitand-type-output"></span>`type Output = U16<O>`

- <span id="u16-bitand"></span>`fn bitand(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> BitAndAssign for U16<O>`

- <span id="u16-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> BitOr for U16<O>`

- <span id="u16-bitor-type-output"></span>`type Output = U16<O>`

- <span id="u16-bitor"></span>`fn bitor(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> BitOrAssign for U16<O>`

- <span id="u16-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> BitXor for U16<O>`

- <span id="u16-bitxor-type-output"></span>`type Output = U16<O>`

- <span id="u16-bitxor"></span>`fn bitxor(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> BitXorAssign for U16<O>`

- <span id="u16-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O: clone::Clone> Clone for U16<O>`

- <span id="u16-clone"></span>`fn clone(&self) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: marker::Copy> Copy for U16<O>`

##### `impl<O: ByteOrder> Debug for U16<O>`

- <span id="u16-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U16<O>`

- <span id="u16-default"></span>`fn default() -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> Display for U16<O>`

- <span id="u16-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U16<O>`

- <span id="u16-div-type-output"></span>`type Output = U16<O>`

- <span id="u16-div"></span>`fn div(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> DivAssign for U16<O>`

- <span id="u16-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O: cmp::Eq> Eq for U16<O>`

##### `impl<O> FromBytes for U16<O>`

##### `impl<O> FromZeros for U16<O>`

##### `impl<O: hash::Hash> Hash for U16<O>`

- <span id="u16-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U16<O>`

##### `impl<O> IntoBytes for U16<O>`

##### `impl<O> KnownLayout for U16<O>`

- <span id="u16-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U16<O>`

- <span id="u16-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U16<O>`

- <span id="u16-mul-type-output"></span>`type Output = U16<O>`

- <span id="u16-mul"></span>`fn mul(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> MulAssign for U16<O>`

- <span id="u16-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O> Not for U16<O>`

- <span id="u16-not-type-output"></span>`type Output = U16<O>`

- <span id="u16-not"></span>`fn not(self) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> Octal for U16<O>`

- <span id="u16-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U16<O>`

- <span id="u16-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U16<O>`

- <span id="u16-partialeq-eq"></span>`fn eq(&self, other: &U16<O>) -> bool` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> PartialOrd for U16<O>`

- <span id="u16-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U16<O>`

- <span id="u16-rem-type-output"></span>`type Output = U16<O>`

- <span id="u16-rem"></span>`fn rem(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> RemAssign for U16<O>`

- <span id="u16-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> Shl for U16<O>`

- <span id="u16-shl-type-output"></span>`type Output = U16<O>`

- <span id="u16-shl"></span>`fn shl(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> ShlAssign for U16<O>`

- <span id="u16-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> Shr for U16<O>`

- <span id="u16-shr-type-output"></span>`type Output = U16<O>`

- <span id="u16-shr"></span>`fn shr(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> ShrAssign for U16<O>`

- <span id="u16-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl<O> StructuralPartialEq for U16<O>`

##### `impl<O: ByteOrder> Sub for U16<O>`

- <span id="u16-sub-type-output"></span>`type Output = U16<O>`

- <span id="u16-sub"></span>`fn sub(self, rhs: U16<O>) -> U16<O>` — [`U16`](../index.md#u16)

##### `impl<O: ByteOrder> SubAssign for U16<O>`

- <span id="u16-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U16<O>)` — [`U16`](../index.md#u16)

##### `impl ToString for U16<O>`

- <span id="u16-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U16<O>`

##### `impl<O> Unaligned for U16<O>`

##### `impl<O: ByteOrder> UpperHex for U16<O>`

- <span id="u16-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `U32<O>`

```rust
struct U32<O>([u8; 4], PhantomData<O>);
```

A 32-bit unsigned integer stored in a given byte order.

`U32` is like the native `u32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

A `U32` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u32` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U32`
has endianness `O` and that, b) the layout of `u32` has
the platform's native endianness.

`U32` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u32-const-zero"></span>`const ZERO: U32<O>`

- <span id="u32-const-max-value"></span>`const MAX_VALUE: U32<O>`

- <span id="u32-from-bytes"></span>`const fn from_bytes(bytes: [u8; 4]) -> U32<O>` — [`U32`](../index.md#u32)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u32-to-bytes"></span>`const fn to_bytes(self) -> [u8; 4]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U32<O>`

- <span id="u32-add-type-output"></span>`type Output = U32<O>`

- <span id="u32-add"></span>`fn add(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> AddAssign for U32<O>`

- <span id="u32-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O> AsMut for U32<O>`

- <span id="u32-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 4]`

##### `impl<O> AsRef for U32<O>`

- <span id="u32-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl<O: ByteOrder> Binary for U32<O>`

- <span id="u32-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U32<O>`

- <span id="u32-bitand-type-output"></span>`type Output = U32<O>`

- <span id="u32-bitand"></span>`fn bitand(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> BitAndAssign for U32<O>`

- <span id="u32-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> BitOr for U32<O>`

- <span id="u32-bitor-type-output"></span>`type Output = U32<O>`

- <span id="u32-bitor"></span>`fn bitor(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> BitOrAssign for U32<O>`

- <span id="u32-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> BitXor for U32<O>`

- <span id="u32-bitxor-type-output"></span>`type Output = U32<O>`

- <span id="u32-bitxor"></span>`fn bitxor(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> BitXorAssign for U32<O>`

- <span id="u32-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O: clone::Clone> Clone for U32<O>`

- <span id="u32-clone"></span>`fn clone(&self) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: marker::Copy> Copy for U32<O>`

##### `impl<O: ByteOrder> Debug for U32<O>`

- <span id="u32-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U32<O>`

- <span id="u32-default"></span>`fn default() -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> Display for U32<O>`

- <span id="u32-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U32<O>`

- <span id="u32-div-type-output"></span>`type Output = U32<O>`

- <span id="u32-div"></span>`fn div(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> DivAssign for U32<O>`

- <span id="u32-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O: cmp::Eq> Eq for U32<O>`

##### `impl<O> FromBytes for U32<O>`

##### `impl<O> FromZeros for U32<O>`

##### `impl<O: hash::Hash> Hash for U32<O>`

- <span id="u32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U32<O>`

##### `impl<O> IntoBytes for U32<O>`

##### `impl<O> KnownLayout for U32<O>`

- <span id="u32-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U32<O>`

- <span id="u32-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U32<O>`

- <span id="u32-mul-type-output"></span>`type Output = U32<O>`

- <span id="u32-mul"></span>`fn mul(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> MulAssign for U32<O>`

- <span id="u32-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O> Not for U32<O>`

- <span id="u32-not-type-output"></span>`type Output = U32<O>`

- <span id="u32-not"></span>`fn not(self) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> Octal for U32<O>`

- <span id="u32-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U32<O>`

- <span id="u32-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U32<O>`

- <span id="u32-partialeq-eq"></span>`fn eq(&self, other: &U32<O>) -> bool` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> PartialOrd for U32<O>`

- <span id="u32-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U32<O>`

- <span id="u32-rem-type-output"></span>`type Output = U32<O>`

- <span id="u32-rem"></span>`fn rem(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> RemAssign for U32<O>`

- <span id="u32-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> Shl for U32<O>`

- <span id="u32-shl-type-output"></span>`type Output = U32<O>`

- <span id="u32-shl"></span>`fn shl(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> ShlAssign for U32<O>`

- <span id="u32-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> Shr for U32<O>`

- <span id="u32-shr-type-output"></span>`type Output = U32<O>`

- <span id="u32-shr"></span>`fn shr(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> ShrAssign for U32<O>`

- <span id="u32-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl<O> StructuralPartialEq for U32<O>`

##### `impl<O: ByteOrder> Sub for U32<O>`

- <span id="u32-sub-type-output"></span>`type Output = U32<O>`

- <span id="u32-sub"></span>`fn sub(self, rhs: U32<O>) -> U32<O>` — [`U32`](../index.md#u32)

##### `impl<O: ByteOrder> SubAssign for U32<O>`

- <span id="u32-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U32<O>)` — [`U32`](../index.md#u32)

##### `impl ToString for U32<O>`

- <span id="u32-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U32<O>`

##### `impl<O> Unaligned for U32<O>`

##### `impl<O: ByteOrder> UpperHex for U32<O>`

- <span id="u32-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `U64<O>`

```rust
struct U64<O>([u8; 8], PhantomData<O>);
```

A 64-bit unsigned integer stored in a given byte order.

`U64` is like the native `u64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

A `U64` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u64` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U64`
has endianness `O` and that, b) the layout of `u64` has
the platform's native endianness.

`U64` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u64-const-zero"></span>`const ZERO: U64<O>`

- <span id="u64-const-max-value"></span>`const MAX_VALUE: U64<O>`

- <span id="u64-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> U64<O>` — [`U64`](../index.md#u64)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u64-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U64<O>`

- <span id="u64-add-type-output"></span>`type Output = U64<O>`

- <span id="u64-add"></span>`fn add(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> AddAssign for U64<O>`

- <span id="u64-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O> AsMut for U64<O>`

- <span id="u64-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for U64<O>`

- <span id="u64-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for U64<O>`

- <span id="u64-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U64<O>`

- <span id="u64-bitand-type-output"></span>`type Output = U64<O>`

- <span id="u64-bitand"></span>`fn bitand(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> BitAndAssign for U64<O>`

- <span id="u64-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> BitOr for U64<O>`

- <span id="u64-bitor-type-output"></span>`type Output = U64<O>`

- <span id="u64-bitor"></span>`fn bitor(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> BitOrAssign for U64<O>`

- <span id="u64-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> BitXor for U64<O>`

- <span id="u64-bitxor-type-output"></span>`type Output = U64<O>`

- <span id="u64-bitxor"></span>`fn bitxor(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> BitXorAssign for U64<O>`

- <span id="u64-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O: clone::Clone> Clone for U64<O>`

- <span id="u64-clone"></span>`fn clone(&self) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: marker::Copy> Copy for U64<O>`

##### `impl<O: ByteOrder> Debug for U64<O>`

- <span id="u64-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U64<O>`

- <span id="u64-default"></span>`fn default() -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> Display for U64<O>`

- <span id="u64-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U64<O>`

- <span id="u64-div-type-output"></span>`type Output = U64<O>`

- <span id="u64-div"></span>`fn div(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> DivAssign for U64<O>`

- <span id="u64-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O: cmp::Eq> Eq for U64<O>`

##### `impl<O> FromBytes for U64<O>`

##### `impl<O> FromZeros for U64<O>`

##### `impl<O: hash::Hash> Hash for U64<O>`

- <span id="u64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U64<O>`

##### `impl<O> IntoBytes for U64<O>`

##### `impl<O> KnownLayout for U64<O>`

- <span id="u64-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U64<O>`

- <span id="u64-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U64<O>`

- <span id="u64-mul-type-output"></span>`type Output = U64<O>`

- <span id="u64-mul"></span>`fn mul(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> MulAssign for U64<O>`

- <span id="u64-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O> Not for U64<O>`

- <span id="u64-not-type-output"></span>`type Output = U64<O>`

- <span id="u64-not"></span>`fn not(self) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> Octal for U64<O>`

- <span id="u64-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U64<O>`

- <span id="u64-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U64<O>`

- <span id="u64-partialeq-eq"></span>`fn eq(&self, other: &U64<O>) -> bool` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> PartialOrd for U64<O>`

- <span id="u64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U64<O>`

- <span id="u64-rem-type-output"></span>`type Output = U64<O>`

- <span id="u64-rem"></span>`fn rem(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> RemAssign for U64<O>`

- <span id="u64-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> Shl for U64<O>`

- <span id="u64-shl-type-output"></span>`type Output = U64<O>`

- <span id="u64-shl"></span>`fn shl(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> ShlAssign for U64<O>`

- <span id="u64-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> Shr for U64<O>`

- <span id="u64-shr-type-output"></span>`type Output = U64<O>`

- <span id="u64-shr"></span>`fn shr(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> ShrAssign for U64<O>`

- <span id="u64-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl<O> StructuralPartialEq for U64<O>`

##### `impl<O: ByteOrder> Sub for U64<O>`

- <span id="u64-sub-type-output"></span>`type Output = U64<O>`

- <span id="u64-sub"></span>`fn sub(self, rhs: U64<O>) -> U64<O>` — [`U64`](../index.md#u64)

##### `impl<O: ByteOrder> SubAssign for U64<O>`

- <span id="u64-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U64<O>)` — [`U64`](../index.md#u64)

##### `impl ToString for U64<O>`

- <span id="u64-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U64<O>`

##### `impl<O> Unaligned for U64<O>`

##### `impl<O: ByteOrder> UpperHex for U64<O>`

- <span id="u64-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `U128<O>`

```rust
struct U128<O>([u8; 16], PhantomData<O>);
```

A 128-bit unsigned integer stored in a given byte order.

`U128` is like the native `u128` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

A `U128` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u128` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U128`
has endianness `O` and that, b) the layout of `u128` has
the platform's native endianness.

`U128` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u128-const-zero"></span>`const ZERO: U128<O>`

- <span id="u128-const-max-value"></span>`const MAX_VALUE: U128<O>`

- <span id="u128-from-bytes"></span>`const fn from_bytes(bytes: [u8; 16]) -> U128<O>` — [`U128`](../index.md#u128)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u128-to-bytes"></span>`const fn to_bytes(self) -> [u8; 16]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U128<O>`

- <span id="u128-add-type-output"></span>`type Output = U128<O>`

- <span id="u128-add"></span>`fn add(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> AddAssign for U128<O>`

- <span id="u128-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O> AsMut for U128<O>`

- <span id="u128-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 16]`

##### `impl<O> AsRef for U128<O>`

- <span id="u128-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 16]`

##### `impl<O: ByteOrder> Binary for U128<O>`

- <span id="u128-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U128<O>`

- <span id="u128-bitand-type-output"></span>`type Output = U128<O>`

- <span id="u128-bitand"></span>`fn bitand(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> BitAndAssign for U128<O>`

- <span id="u128-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> BitOr for U128<O>`

- <span id="u128-bitor-type-output"></span>`type Output = U128<O>`

- <span id="u128-bitor"></span>`fn bitor(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> BitOrAssign for U128<O>`

- <span id="u128-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> BitXor for U128<O>`

- <span id="u128-bitxor-type-output"></span>`type Output = U128<O>`

- <span id="u128-bitxor"></span>`fn bitxor(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> BitXorAssign for U128<O>`

- <span id="u128-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O: clone::Clone> Clone for U128<O>`

- <span id="u128-clone"></span>`fn clone(&self) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: marker::Copy> Copy for U128<O>`

##### `impl<O: ByteOrder> Debug for U128<O>`

- <span id="u128-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U128<O>`

- <span id="u128-default"></span>`fn default() -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> Display for U128<O>`

- <span id="u128-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U128<O>`

- <span id="u128-div-type-output"></span>`type Output = U128<O>`

- <span id="u128-div"></span>`fn div(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> DivAssign for U128<O>`

- <span id="u128-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O: cmp::Eq> Eq for U128<O>`

##### `impl<O> FromBytes for U128<O>`

##### `impl<O> FromZeros for U128<O>`

##### `impl<O: hash::Hash> Hash for U128<O>`

- <span id="u128-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U128<O>`

##### `impl<O> IntoBytes for U128<O>`

##### `impl<O> KnownLayout for U128<O>`

- <span id="u128-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U128<O>`

- <span id="u128-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U128<O>`

- <span id="u128-mul-type-output"></span>`type Output = U128<O>`

- <span id="u128-mul"></span>`fn mul(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> MulAssign for U128<O>`

- <span id="u128-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O> Not for U128<O>`

- <span id="u128-not-type-output"></span>`type Output = U128<O>`

- <span id="u128-not"></span>`fn not(self) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> Octal for U128<O>`

- <span id="u128-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U128<O>`

- <span id="u128-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U128<O>`

- <span id="u128-partialeq-eq"></span>`fn eq(&self, other: &U128<O>) -> bool` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> PartialOrd for U128<O>`

- <span id="u128-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U128<O>`

- <span id="u128-rem-type-output"></span>`type Output = U128<O>`

- <span id="u128-rem"></span>`fn rem(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> RemAssign for U128<O>`

- <span id="u128-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> Shl for U128<O>`

- <span id="u128-shl-type-output"></span>`type Output = U128<O>`

- <span id="u128-shl"></span>`fn shl(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> ShlAssign for U128<O>`

- <span id="u128-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> Shr for U128<O>`

- <span id="u128-shr-type-output"></span>`type Output = U128<O>`

- <span id="u128-shr"></span>`fn shr(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> ShrAssign for U128<O>`

- <span id="u128-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl<O> StructuralPartialEq for U128<O>`

##### `impl<O: ByteOrder> Sub for U128<O>`

- <span id="u128-sub-type-output"></span>`type Output = U128<O>`

- <span id="u128-sub"></span>`fn sub(self, rhs: U128<O>) -> U128<O>` — [`U128`](../index.md#u128)

##### `impl<O: ByteOrder> SubAssign for U128<O>`

- <span id="u128-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U128<O>)` — [`U128`](../index.md#u128)

##### `impl ToString for U128<O>`

- <span id="u128-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U128<O>`

##### `impl<O> Unaligned for U128<O>`

##### `impl<O: ByteOrder> UpperHex for U128<O>`

- <span id="u128-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `Usize<O>`

```rust
struct Usize<O>([u8; 8], PhantomData<O>);
```

A word-sized unsigned integer stored in a given byte order.

`Usize` is like the native `usize` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

A `Usize` can be constructed using
the `new` method, and its contained value can be obtained as a native
`usize` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `Usize`
has endianness `O` and that, b) the layout of `usize` has
the platform's native endianness.

`Usize` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="usize-const-zero"></span>`const ZERO: Usize<O>`

- <span id="usize-const-max-value"></span>`const MAX_VALUE: Usize<O>`

- <span id="usize-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> Usize<O>` — [`Usize`](../index.md#usize)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="usize-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for Usize<O>`

- <span id="usize-add-type-output"></span>`type Output = Usize<O>`

- <span id="usize-add"></span>`fn add(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> AddAssign for Usize<O>`

- <span id="usize-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O> AsMut for Usize<O>`

- <span id="usize-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for Usize<O>`

- <span id="usize-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for Usize<O>`

- <span id="usize-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for Usize<O>`

- <span id="usize-bitand-type-output"></span>`type Output = Usize<O>`

- <span id="usize-bitand"></span>`fn bitand(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> BitAndAssign for Usize<O>`

- <span id="usize-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> BitOr for Usize<O>`

- <span id="usize-bitor-type-output"></span>`type Output = Usize<O>`

- <span id="usize-bitor"></span>`fn bitor(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> BitOrAssign for Usize<O>`

- <span id="usize-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> BitXor for Usize<O>`

- <span id="usize-bitxor-type-output"></span>`type Output = Usize<O>`

- <span id="usize-bitxor"></span>`fn bitxor(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> BitXorAssign for Usize<O>`

- <span id="usize-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O: clone::Clone> Clone for Usize<O>`

- <span id="usize-clone"></span>`fn clone(&self) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: marker::Copy> Copy for Usize<O>`

##### `impl<O: ByteOrder> Debug for Usize<O>`

- <span id="usize-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for Usize<O>`

- <span id="usize-default"></span>`fn default() -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> Display for Usize<O>`

- <span id="usize-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for Usize<O>`

- <span id="usize-div-type-output"></span>`type Output = Usize<O>`

- <span id="usize-div"></span>`fn div(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> DivAssign for Usize<O>`

- <span id="usize-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O: cmp::Eq> Eq for Usize<O>`

##### `impl<O> FromBytes for Usize<O>`

##### `impl<O> FromZeros for Usize<O>`

##### `impl<O: hash::Hash> Hash for Usize<O>`

- <span id="usize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for Usize<O>`

##### `impl<O> IntoBytes for Usize<O>`

##### `impl<O> KnownLayout for Usize<O>`

- <span id="usize-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for Usize<O>`

- <span id="usize-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for Usize<O>`

- <span id="usize-mul-type-output"></span>`type Output = Usize<O>`

- <span id="usize-mul"></span>`fn mul(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> MulAssign for Usize<O>`

- <span id="usize-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O> Not for Usize<O>`

- <span id="usize-not-type-output"></span>`type Output = Usize<O>`

- <span id="usize-not"></span>`fn not(self) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> Octal for Usize<O>`

- <span id="usize-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for Usize<O>`

- <span id="usize-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for Usize<O>`

- <span id="usize-partialeq-eq"></span>`fn eq(&self, other: &Usize<O>) -> bool` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> PartialOrd for Usize<O>`

- <span id="usize-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for Usize<O>`

- <span id="usize-rem-type-output"></span>`type Output = Usize<O>`

- <span id="usize-rem"></span>`fn rem(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> RemAssign for Usize<O>`

- <span id="usize-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> Shl for Usize<O>`

- <span id="usize-shl-type-output"></span>`type Output = Usize<O>`

- <span id="usize-shl"></span>`fn shl(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> ShlAssign for Usize<O>`

- <span id="usize-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> Shr for Usize<O>`

- <span id="usize-shr-type-output"></span>`type Output = Usize<O>`

- <span id="usize-shr"></span>`fn shr(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> ShrAssign for Usize<O>`

- <span id="usize-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl<O> StructuralPartialEq for Usize<O>`

##### `impl<O: ByteOrder> Sub for Usize<O>`

- <span id="usize-sub-type-output"></span>`type Output = Usize<O>`

- <span id="usize-sub"></span>`fn sub(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](../index.md#usize)

##### `impl<O: ByteOrder> SubAssign for Usize<O>`

- <span id="usize-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: Usize<O>)` — [`Usize`](../index.md#usize)

##### `impl ToString for Usize<O>`

- <span id="usize-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for Usize<O>`

##### `impl<O> Unaligned for Usize<O>`

##### `impl<O: ByteOrder> UpperHex for Usize<O>`

- <span id="usize-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I16<O>`

```rust
struct I16<O>([u8; 2], PhantomData<O>);
```

A 16-bit signed integer stored in a given byte order.

`I16` is like the native `i16` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

An `I16` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i16` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I16`
has endianness `O` and that, b) the layout of `i16` has
the platform's native endianness.

`I16` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i16-const-zero"></span>`const ZERO: I16<O>`

- <span id="i16-from-bytes"></span>`const fn from_bytes(bytes: [u8; 2]) -> I16<O>` — [`I16`](../index.md#i16)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i16-to-bytes"></span>`const fn to_bytes(self) -> [u8; 2]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I16<O>`

- <span id="i16-add-type-output"></span>`type Output = I16<O>`

- <span id="i16-add"></span>`fn add(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> AddAssign for I16<O>`

- <span id="i16-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O> AsMut for I16<O>`

- <span id="i16-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 2]`

##### `impl<O> AsRef for I16<O>`

- <span id="i16-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 2]`

##### `impl<O: ByteOrder> Binary for I16<O>`

- <span id="i16-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I16<O>`

- <span id="i16-bitand-type-output"></span>`type Output = I16<O>`

- <span id="i16-bitand"></span>`fn bitand(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> BitAndAssign for I16<O>`

- <span id="i16-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> BitOr for I16<O>`

- <span id="i16-bitor-type-output"></span>`type Output = I16<O>`

- <span id="i16-bitor"></span>`fn bitor(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> BitOrAssign for I16<O>`

- <span id="i16-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> BitXor for I16<O>`

- <span id="i16-bitxor-type-output"></span>`type Output = I16<O>`

- <span id="i16-bitxor"></span>`fn bitxor(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> BitXorAssign for I16<O>`

- <span id="i16-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O: clone::Clone> Clone for I16<O>`

- <span id="i16-clone"></span>`fn clone(&self) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: marker::Copy> Copy for I16<O>`

##### `impl<O: ByteOrder> Debug for I16<O>`

- <span id="i16-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I16<O>`

- <span id="i16-default"></span>`fn default() -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> Display for I16<O>`

- <span id="i16-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I16<O>`

- <span id="i16-div-type-output"></span>`type Output = I16<O>`

- <span id="i16-div"></span>`fn div(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> DivAssign for I16<O>`

- <span id="i16-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O: cmp::Eq> Eq for I16<O>`

##### `impl<O> FromBytes for I16<O>`

##### `impl<O> FromZeros for I16<O>`

##### `impl<O: hash::Hash> Hash for I16<O>`

- <span id="i16-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I16<O>`

##### `impl<O> IntoBytes for I16<O>`

##### `impl<O> KnownLayout for I16<O>`

- <span id="i16-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I16<O>`

- <span id="i16-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I16<O>`

- <span id="i16-mul-type-output"></span>`type Output = I16<O>`

- <span id="i16-mul"></span>`fn mul(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> MulAssign for I16<O>`

- <span id="i16-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> Neg for I16<O>`

- <span id="i16-neg-type-output"></span>`type Output = I16<O>`

- <span id="i16-neg"></span>`fn neg(self) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O> Not for I16<O>`

- <span id="i16-not-type-output"></span>`type Output = I16<O>`

- <span id="i16-not"></span>`fn not(self) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> Octal for I16<O>`

- <span id="i16-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I16<O>`

- <span id="i16-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I16<O>`

- <span id="i16-partialeq-eq"></span>`fn eq(&self, other: &I16<O>) -> bool` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> PartialOrd for I16<O>`

- <span id="i16-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I16<O>`

- <span id="i16-rem-type-output"></span>`type Output = I16<O>`

- <span id="i16-rem"></span>`fn rem(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> RemAssign for I16<O>`

- <span id="i16-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> Shl for I16<O>`

- <span id="i16-shl-type-output"></span>`type Output = I16<O>`

- <span id="i16-shl"></span>`fn shl(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> ShlAssign for I16<O>`

- <span id="i16-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> Shr for I16<O>`

- <span id="i16-shr-type-output"></span>`type Output = I16<O>`

- <span id="i16-shr"></span>`fn shr(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> ShrAssign for I16<O>`

- <span id="i16-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl<O> StructuralPartialEq for I16<O>`

##### `impl<O: ByteOrder> Sub for I16<O>`

- <span id="i16-sub-type-output"></span>`type Output = I16<O>`

- <span id="i16-sub"></span>`fn sub(self, rhs: I16<O>) -> I16<O>` — [`I16`](../index.md#i16)

##### `impl<O: ByteOrder> SubAssign for I16<O>`

- <span id="i16-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I16<O>)` — [`I16`](../index.md#i16)

##### `impl ToString for I16<O>`

- <span id="i16-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I16<O>`

##### `impl<O> Unaligned for I16<O>`

##### `impl<O: ByteOrder> UpperHex for I16<O>`

- <span id="i16-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I32<O>`

```rust
struct I32<O>([u8; 4], PhantomData<O>);
```

A 32-bit signed integer stored in a given byte order.

`I32` is like the native `i32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

An `I32` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i32` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I32`
has endianness `O` and that, b) the layout of `i32` has
the platform's native endianness.

`I32` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i32-const-zero"></span>`const ZERO: I32<O>`

- <span id="i32-from-bytes"></span>`const fn from_bytes(bytes: [u8; 4]) -> I32<O>` — [`I32`](../index.md#i32)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i32-to-bytes"></span>`const fn to_bytes(self) -> [u8; 4]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I32<O>`

- <span id="i32-add-type-output"></span>`type Output = I32<O>`

- <span id="i32-add"></span>`fn add(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> AddAssign for I32<O>`

- <span id="i32-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O> AsMut for I32<O>`

- <span id="i32-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 4]`

##### `impl<O> AsRef for I32<O>`

- <span id="i32-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl<O: ByteOrder> Binary for I32<O>`

- <span id="i32-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I32<O>`

- <span id="i32-bitand-type-output"></span>`type Output = I32<O>`

- <span id="i32-bitand"></span>`fn bitand(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> BitAndAssign for I32<O>`

- <span id="i32-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> BitOr for I32<O>`

- <span id="i32-bitor-type-output"></span>`type Output = I32<O>`

- <span id="i32-bitor"></span>`fn bitor(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> BitOrAssign for I32<O>`

- <span id="i32-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> BitXor for I32<O>`

- <span id="i32-bitxor-type-output"></span>`type Output = I32<O>`

- <span id="i32-bitxor"></span>`fn bitxor(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> BitXorAssign for I32<O>`

- <span id="i32-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O: clone::Clone> Clone for I32<O>`

- <span id="i32-clone"></span>`fn clone(&self) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: marker::Copy> Copy for I32<O>`

##### `impl<O: ByteOrder> Debug for I32<O>`

- <span id="i32-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I32<O>`

- <span id="i32-default"></span>`fn default() -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> Display for I32<O>`

- <span id="i32-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I32<O>`

- <span id="i32-div-type-output"></span>`type Output = I32<O>`

- <span id="i32-div"></span>`fn div(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> DivAssign for I32<O>`

- <span id="i32-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O: cmp::Eq> Eq for I32<O>`

##### `impl<O> FromBytes for I32<O>`

##### `impl<O> FromZeros for I32<O>`

##### `impl<O: hash::Hash> Hash for I32<O>`

- <span id="i32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I32<O>`

##### `impl<O> IntoBytes for I32<O>`

##### `impl<O> KnownLayout for I32<O>`

- <span id="i32-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I32<O>`

- <span id="i32-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I32<O>`

- <span id="i32-mul-type-output"></span>`type Output = I32<O>`

- <span id="i32-mul"></span>`fn mul(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> MulAssign for I32<O>`

- <span id="i32-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> Neg for I32<O>`

- <span id="i32-neg-type-output"></span>`type Output = I32<O>`

- <span id="i32-neg"></span>`fn neg(self) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O> Not for I32<O>`

- <span id="i32-not-type-output"></span>`type Output = I32<O>`

- <span id="i32-not"></span>`fn not(self) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> Octal for I32<O>`

- <span id="i32-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I32<O>`

- <span id="i32-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I32<O>`

- <span id="i32-partialeq-eq"></span>`fn eq(&self, other: &I32<O>) -> bool` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> PartialOrd for I32<O>`

- <span id="i32-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I32<O>`

- <span id="i32-rem-type-output"></span>`type Output = I32<O>`

- <span id="i32-rem"></span>`fn rem(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> RemAssign for I32<O>`

- <span id="i32-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> Shl for I32<O>`

- <span id="i32-shl-type-output"></span>`type Output = I32<O>`

- <span id="i32-shl"></span>`fn shl(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> ShlAssign for I32<O>`

- <span id="i32-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> Shr for I32<O>`

- <span id="i32-shr-type-output"></span>`type Output = I32<O>`

- <span id="i32-shr"></span>`fn shr(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> ShrAssign for I32<O>`

- <span id="i32-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl<O> StructuralPartialEq for I32<O>`

##### `impl<O: ByteOrder> Sub for I32<O>`

- <span id="i32-sub-type-output"></span>`type Output = I32<O>`

- <span id="i32-sub"></span>`fn sub(self, rhs: I32<O>) -> I32<O>` — [`I32`](../index.md#i32)

##### `impl<O: ByteOrder> SubAssign for I32<O>`

- <span id="i32-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I32<O>)` — [`I32`](../index.md#i32)

##### `impl ToString for I32<O>`

- <span id="i32-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I32<O>`

##### `impl<O> Unaligned for I32<O>`

##### `impl<O: ByteOrder> UpperHex for I32<O>`

- <span id="i32-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I64<O>`

```rust
struct I64<O>([u8; 8], PhantomData<O>);
```

A 64-bit signed integer stored in a given byte order.

`I64` is like the native `i64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

An `I64` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i64` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I64`
has endianness `O` and that, b) the layout of `i64` has
the platform's native endianness.

`I64` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i64-const-zero"></span>`const ZERO: I64<O>`

- <span id="i64-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> I64<O>` — [`I64`](../index.md#i64)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i64-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I64<O>`

- <span id="i64-add-type-output"></span>`type Output = I64<O>`

- <span id="i64-add"></span>`fn add(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> AddAssign for I64<O>`

- <span id="i64-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O> AsMut for I64<O>`

- <span id="i64-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for I64<O>`

- <span id="i64-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for I64<O>`

- <span id="i64-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I64<O>`

- <span id="i64-bitand-type-output"></span>`type Output = I64<O>`

- <span id="i64-bitand"></span>`fn bitand(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> BitAndAssign for I64<O>`

- <span id="i64-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> BitOr for I64<O>`

- <span id="i64-bitor-type-output"></span>`type Output = I64<O>`

- <span id="i64-bitor"></span>`fn bitor(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> BitOrAssign for I64<O>`

- <span id="i64-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> BitXor for I64<O>`

- <span id="i64-bitxor-type-output"></span>`type Output = I64<O>`

- <span id="i64-bitxor"></span>`fn bitxor(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> BitXorAssign for I64<O>`

- <span id="i64-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O: clone::Clone> Clone for I64<O>`

- <span id="i64-clone"></span>`fn clone(&self) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: marker::Copy> Copy for I64<O>`

##### `impl<O: ByteOrder> Debug for I64<O>`

- <span id="i64-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I64<O>`

- <span id="i64-default"></span>`fn default() -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> Display for I64<O>`

- <span id="i64-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I64<O>`

- <span id="i64-div-type-output"></span>`type Output = I64<O>`

- <span id="i64-div"></span>`fn div(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> DivAssign for I64<O>`

- <span id="i64-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O: cmp::Eq> Eq for I64<O>`

##### `impl<O> FromBytes for I64<O>`

##### `impl<O> FromZeros for I64<O>`

##### `impl<O: hash::Hash> Hash for I64<O>`

- <span id="i64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I64<O>`

##### `impl<O> IntoBytes for I64<O>`

##### `impl<O> KnownLayout for I64<O>`

- <span id="i64-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I64<O>`

- <span id="i64-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I64<O>`

- <span id="i64-mul-type-output"></span>`type Output = I64<O>`

- <span id="i64-mul"></span>`fn mul(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> MulAssign for I64<O>`

- <span id="i64-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> Neg for I64<O>`

- <span id="i64-neg-type-output"></span>`type Output = I64<O>`

- <span id="i64-neg"></span>`fn neg(self) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O> Not for I64<O>`

- <span id="i64-not-type-output"></span>`type Output = I64<O>`

- <span id="i64-not"></span>`fn not(self) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> Octal for I64<O>`

- <span id="i64-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I64<O>`

- <span id="i64-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I64<O>`

- <span id="i64-partialeq-eq"></span>`fn eq(&self, other: &I64<O>) -> bool` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> PartialOrd for I64<O>`

- <span id="i64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I64<O>`

- <span id="i64-rem-type-output"></span>`type Output = I64<O>`

- <span id="i64-rem"></span>`fn rem(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> RemAssign for I64<O>`

- <span id="i64-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> Shl for I64<O>`

- <span id="i64-shl-type-output"></span>`type Output = I64<O>`

- <span id="i64-shl"></span>`fn shl(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> ShlAssign for I64<O>`

- <span id="i64-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> Shr for I64<O>`

- <span id="i64-shr-type-output"></span>`type Output = I64<O>`

- <span id="i64-shr"></span>`fn shr(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> ShrAssign for I64<O>`

- <span id="i64-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl<O> StructuralPartialEq for I64<O>`

##### `impl<O: ByteOrder> Sub for I64<O>`

- <span id="i64-sub-type-output"></span>`type Output = I64<O>`

- <span id="i64-sub"></span>`fn sub(self, rhs: I64<O>) -> I64<O>` — [`I64`](../index.md#i64)

##### `impl<O: ByteOrder> SubAssign for I64<O>`

- <span id="i64-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I64<O>)` — [`I64`](../index.md#i64)

##### `impl ToString for I64<O>`

- <span id="i64-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I64<O>`

##### `impl<O> Unaligned for I64<O>`

##### `impl<O: ByteOrder> UpperHex for I64<O>`

- <span id="i64-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I128<O>`

```rust
struct I128<O>([u8; 16], PhantomData<O>);
```

A 128-bit signed integer stored in a given byte order.

`I128` is like the native `i128` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

An `I128` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i128` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I128`
has endianness `O` and that, b) the layout of `i128` has
the platform's native endianness.

`I128` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i128-const-zero"></span>`const ZERO: I128<O>`

- <span id="i128-from-bytes"></span>`const fn from_bytes(bytes: [u8; 16]) -> I128<O>` — [`I128`](../index.md#i128)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i128-to-bytes"></span>`const fn to_bytes(self) -> [u8; 16]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I128<O>`

- <span id="i128-add-type-output"></span>`type Output = I128<O>`

- <span id="i128-add"></span>`fn add(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> AddAssign for I128<O>`

- <span id="i128-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O> AsMut for I128<O>`

- <span id="i128-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 16]`

##### `impl<O> AsRef for I128<O>`

- <span id="i128-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 16]`

##### `impl<O: ByteOrder> Binary for I128<O>`

- <span id="i128-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I128<O>`

- <span id="i128-bitand-type-output"></span>`type Output = I128<O>`

- <span id="i128-bitand"></span>`fn bitand(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> BitAndAssign for I128<O>`

- <span id="i128-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> BitOr for I128<O>`

- <span id="i128-bitor-type-output"></span>`type Output = I128<O>`

- <span id="i128-bitor"></span>`fn bitor(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> BitOrAssign for I128<O>`

- <span id="i128-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> BitXor for I128<O>`

- <span id="i128-bitxor-type-output"></span>`type Output = I128<O>`

- <span id="i128-bitxor"></span>`fn bitxor(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> BitXorAssign for I128<O>`

- <span id="i128-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O: clone::Clone> Clone for I128<O>`

- <span id="i128-clone"></span>`fn clone(&self) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: marker::Copy> Copy for I128<O>`

##### `impl<O: ByteOrder> Debug for I128<O>`

- <span id="i128-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I128<O>`

- <span id="i128-default"></span>`fn default() -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> Display for I128<O>`

- <span id="i128-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I128<O>`

- <span id="i128-div-type-output"></span>`type Output = I128<O>`

- <span id="i128-div"></span>`fn div(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> DivAssign for I128<O>`

- <span id="i128-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O: cmp::Eq> Eq for I128<O>`

##### `impl<O> FromBytes for I128<O>`

##### `impl<O> FromZeros for I128<O>`

##### `impl<O: hash::Hash> Hash for I128<O>`

- <span id="i128-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I128<O>`

##### `impl<O> IntoBytes for I128<O>`

##### `impl<O> KnownLayout for I128<O>`

- <span id="i128-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I128<O>`

- <span id="i128-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I128<O>`

- <span id="i128-mul-type-output"></span>`type Output = I128<O>`

- <span id="i128-mul"></span>`fn mul(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> MulAssign for I128<O>`

- <span id="i128-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> Neg for I128<O>`

- <span id="i128-neg-type-output"></span>`type Output = I128<O>`

- <span id="i128-neg"></span>`fn neg(self) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O> Not for I128<O>`

- <span id="i128-not-type-output"></span>`type Output = I128<O>`

- <span id="i128-not"></span>`fn not(self) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> Octal for I128<O>`

- <span id="i128-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I128<O>`

- <span id="i128-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I128<O>`

- <span id="i128-partialeq-eq"></span>`fn eq(&self, other: &I128<O>) -> bool` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> PartialOrd for I128<O>`

- <span id="i128-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I128<O>`

- <span id="i128-rem-type-output"></span>`type Output = I128<O>`

- <span id="i128-rem"></span>`fn rem(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> RemAssign for I128<O>`

- <span id="i128-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> Shl for I128<O>`

- <span id="i128-shl-type-output"></span>`type Output = I128<O>`

- <span id="i128-shl"></span>`fn shl(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> ShlAssign for I128<O>`

- <span id="i128-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> Shr for I128<O>`

- <span id="i128-shr-type-output"></span>`type Output = I128<O>`

- <span id="i128-shr"></span>`fn shr(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> ShrAssign for I128<O>`

- <span id="i128-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl<O> StructuralPartialEq for I128<O>`

##### `impl<O: ByteOrder> Sub for I128<O>`

- <span id="i128-sub-type-output"></span>`type Output = I128<O>`

- <span id="i128-sub"></span>`fn sub(self, rhs: I128<O>) -> I128<O>` — [`I128`](../index.md#i128)

##### `impl<O: ByteOrder> SubAssign for I128<O>`

- <span id="i128-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I128<O>)` — [`I128`](../index.md#i128)

##### `impl ToString for I128<O>`

- <span id="i128-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I128<O>`

##### `impl<O> Unaligned for I128<O>`

##### `impl<O: ByteOrder> UpperHex for I128<O>`

- <span id="i128-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `Isize<O>`

```rust
struct Isize<O>([u8; 8], PhantomData<O>);
```

A word-sized signed integer stored in a given byte order.

`Isize` is like the native `isize` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

An `Isize` can be constructed using
the `new` method, and its contained value can be obtained as a native
`isize` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `Isize`
has endianness `O` and that, b) the layout of `isize` has
the platform's native endianness.

`Isize` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="isize-const-zero"></span>`const ZERO: Isize<O>`

- <span id="isize-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> Isize<O>` — [`Isize`](../index.md#isize)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="isize-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for Isize<O>`

- <span id="isize-add-type-output"></span>`type Output = Isize<O>`

- <span id="isize-add"></span>`fn add(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> AddAssign for Isize<O>`

- <span id="isize-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O> AsMut for Isize<O>`

- <span id="isize-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for Isize<O>`

- <span id="isize-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for Isize<O>`

- <span id="isize-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for Isize<O>`

- <span id="isize-bitand-type-output"></span>`type Output = Isize<O>`

- <span id="isize-bitand"></span>`fn bitand(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> BitAndAssign for Isize<O>`

- <span id="isize-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> BitOr for Isize<O>`

- <span id="isize-bitor-type-output"></span>`type Output = Isize<O>`

- <span id="isize-bitor"></span>`fn bitor(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> BitOrAssign for Isize<O>`

- <span id="isize-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> BitXor for Isize<O>`

- <span id="isize-bitxor-type-output"></span>`type Output = Isize<O>`

- <span id="isize-bitxor"></span>`fn bitxor(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> BitXorAssign for Isize<O>`

- <span id="isize-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O: clone::Clone> Clone for Isize<O>`

- <span id="isize-clone"></span>`fn clone(&self) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: marker::Copy> Copy for Isize<O>`

##### `impl<O: ByteOrder> Debug for Isize<O>`

- <span id="isize-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for Isize<O>`

- <span id="isize-default"></span>`fn default() -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> Display for Isize<O>`

- <span id="isize-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for Isize<O>`

- <span id="isize-div-type-output"></span>`type Output = Isize<O>`

- <span id="isize-div"></span>`fn div(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> DivAssign for Isize<O>`

- <span id="isize-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O: cmp::Eq> Eq for Isize<O>`

##### `impl<O> FromBytes for Isize<O>`

##### `impl<O> FromZeros for Isize<O>`

##### `impl<O: hash::Hash> Hash for Isize<O>`

- <span id="isize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for Isize<O>`

##### `impl<O> IntoBytes for Isize<O>`

##### `impl<O> KnownLayout for Isize<O>`

- <span id="isize-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for Isize<O>`

- <span id="isize-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for Isize<O>`

- <span id="isize-mul-type-output"></span>`type Output = Isize<O>`

- <span id="isize-mul"></span>`fn mul(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> MulAssign for Isize<O>`

- <span id="isize-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> Neg for Isize<O>`

- <span id="isize-neg-type-output"></span>`type Output = Isize<O>`

- <span id="isize-neg"></span>`fn neg(self) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O> Not for Isize<O>`

- <span id="isize-not-type-output"></span>`type Output = Isize<O>`

- <span id="isize-not"></span>`fn not(self) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> Octal for Isize<O>`

- <span id="isize-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for Isize<O>`

- <span id="isize-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for Isize<O>`

- <span id="isize-partialeq-eq"></span>`fn eq(&self, other: &Isize<O>) -> bool` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> PartialOrd for Isize<O>`

- <span id="isize-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for Isize<O>`

- <span id="isize-rem-type-output"></span>`type Output = Isize<O>`

- <span id="isize-rem"></span>`fn rem(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> RemAssign for Isize<O>`

- <span id="isize-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> Shl for Isize<O>`

- <span id="isize-shl-type-output"></span>`type Output = Isize<O>`

- <span id="isize-shl"></span>`fn shl(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> ShlAssign for Isize<O>`

- <span id="isize-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> Shr for Isize<O>`

- <span id="isize-shr-type-output"></span>`type Output = Isize<O>`

- <span id="isize-shr"></span>`fn shr(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> ShrAssign for Isize<O>`

- <span id="isize-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl<O> StructuralPartialEq for Isize<O>`

##### `impl<O: ByteOrder> Sub for Isize<O>`

- <span id="isize-sub-type-output"></span>`type Output = Isize<O>`

- <span id="isize-sub"></span>`fn sub(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](../index.md#isize)

##### `impl<O: ByteOrder> SubAssign for Isize<O>`

- <span id="isize-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: Isize<O>)` — [`Isize`](../index.md#isize)

##### `impl ToString for Isize<O>`

- <span id="isize-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for Isize<O>`

##### `impl<O> Unaligned for Isize<O>`

##### `impl<O: ByteOrder> UpperHex for Isize<O>`

- <span id="isize-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `F32<O>`

```rust
struct F32<O>([u8; 4], PhantomData<O>);
```

A 32-bit floating point number stored in a given byte order.

`F32` is like the native `f32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

An `F32` can be constructed using
the `new` method, and its contained value can be obtained as a native
`f32` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `F32`
has endianness `O` and that, b) the layout of `f32` has
the platform's native endianness.

`F32` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="f32-const-zero"></span>`const ZERO: F32<O>`

- <span id="f32-from-bytes"></span>`const fn from_bytes(bytes: [u8; 4]) -> F32<O>` — [`F32`](../index.md#f32)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="f32-to-bytes"></span>`const fn to_bytes(self) -> [u8; 4]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for F32<O>`

- <span id="f32-add-type-output"></span>`type Output = F32<O>`

- <span id="f32-add"></span>`fn add(self, rhs: F32<O>) -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> AddAssign for F32<O>`

- <span id="f32-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: F32<O>)` — [`F32`](../index.md#f32)

##### `impl<O> AsMut for F32<O>`

- <span id="f32-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 4]`

##### `impl<O> AsRef for F32<O>`

- <span id="f32-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl<O: clone::Clone> Clone for F32<O>`

- <span id="f32-clone"></span>`fn clone(&self) -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: marker::Copy> Copy for F32<O>`

##### `impl<O: ByteOrder> Debug for F32<O>`

- <span id="f32-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for F32<O>`

- <span id="f32-default"></span>`fn default() -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> Display for F32<O>`

- <span id="f32-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for F32<O>`

- <span id="f32-div-type-output"></span>`type Output = F32<O>`

- <span id="f32-div"></span>`fn div(self, rhs: F32<O>) -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> DivAssign for F32<O>`

- <span id="f32-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: F32<O>)` — [`F32`](../index.md#f32)

##### `impl<O: cmp::Eq> Eq for F32<O>`

##### `impl<O> FromBytes for F32<O>`

##### `impl<O> FromZeros for F32<O>`

##### `impl<O: hash::Hash> Hash for F32<O>`

- <span id="f32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for F32<O>`

##### `impl<O> IntoBytes for F32<O>`

##### `impl<O> KnownLayout for F32<O>`

- <span id="f32-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> Mul for F32<O>`

- <span id="f32-mul-type-output"></span>`type Output = F32<O>`

- <span id="f32-mul"></span>`fn mul(self, rhs: F32<O>) -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> MulAssign for F32<O>`

- <span id="f32-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: F32<O>)` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> Neg for F32<O>`

- <span id="f32-neg-type-output"></span>`type Output = F32<O>`

- <span id="f32-neg"></span>`fn neg(self) -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: cmp::PartialEq> PartialEq for F32<O>`

- <span id="f32-partialeq-eq"></span>`fn eq(&self, other: &F32<O>) -> bool` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> PartialOrd for F32<O>`

- <span id="f32-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for F32<O>`

- <span id="f32-rem-type-output"></span>`type Output = F32<O>`

- <span id="f32-rem"></span>`fn rem(self, rhs: F32<O>) -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> RemAssign for F32<O>`

- <span id="f32-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: F32<O>)` — [`F32`](../index.md#f32)

##### `impl<O> StructuralPartialEq for F32<O>`

##### `impl<O: ByteOrder> Sub for F32<O>`

- <span id="f32-sub-type-output"></span>`type Output = F32<O>`

- <span id="f32-sub"></span>`fn sub(self, rhs: F32<O>) -> F32<O>` — [`F32`](../index.md#f32)

##### `impl<O: ByteOrder> SubAssign for F32<O>`

- <span id="f32-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: F32<O>)` — [`F32`](../index.md#f32)

##### `impl ToString for F32<O>`

- <span id="f32-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for F32<O>`

##### `impl<O> Unaligned for F32<O>`

### `F64<O>`

```rust
struct F64<O>([u8; 8], PhantomData<O>);
```

A 64-bit floating point number stored in a given byte order.

`F64` is like the native `f64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](../index.md). In particular, this refers
to [`BigEndian`](../index.md), [`LittleEndian`](../index.md), [`NativeEndian`](../index.md), and [`NetworkEndian`](../index.md).

An `F64` can be constructed using
the `new` method, and its contained value can be obtained as a native
`f64` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `F64`
has endianness `O` and that, b) the layout of `f64` has
the platform's native endianness.

`F64` implements [`FromBytes`](../index.md), [`IntoBytes`](../index.md), and [`Unaligned`](../index.md),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="f64-const-zero"></span>`const ZERO: F64<O>`

- <span id="f64-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> F64<O>` — [`F64`](../index.md#f64)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="f64-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for F64<O>`

- <span id="f64-add-type-output"></span>`type Output = F64<O>`

- <span id="f64-add"></span>`fn add(self, rhs: F64<O>) -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> AddAssign for F64<O>`

- <span id="f64-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: F64<O>)` — [`F64`](../index.md#f64)

##### `impl<O> AsMut for F64<O>`

- <span id="f64-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for F64<O>`

- <span id="f64-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: clone::Clone> Clone for F64<O>`

- <span id="f64-clone"></span>`fn clone(&self) -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: marker::Copy> Copy for F64<O>`

##### `impl<O: ByteOrder> Debug for F64<O>`

- <span id="f64-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for F64<O>`

- <span id="f64-default"></span>`fn default() -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> Display for F64<O>`

- <span id="f64-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for F64<O>`

- <span id="f64-div-type-output"></span>`type Output = F64<O>`

- <span id="f64-div"></span>`fn div(self, rhs: F64<O>) -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> DivAssign for F64<O>`

- <span id="f64-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: F64<O>)` — [`F64`](../index.md#f64)

##### `impl<O: cmp::Eq> Eq for F64<O>`

##### `impl<O> FromBytes for F64<O>`

##### `impl<O> FromZeros for F64<O>`

##### `impl<O: hash::Hash> Hash for F64<O>`

- <span id="f64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for F64<O>`

##### `impl<O> IntoBytes for F64<O>`

##### `impl<O> KnownLayout for F64<O>`

- <span id="f64-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> Mul for F64<O>`

- <span id="f64-mul-type-output"></span>`type Output = F64<O>`

- <span id="f64-mul"></span>`fn mul(self, rhs: F64<O>) -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> MulAssign for F64<O>`

- <span id="f64-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: F64<O>)` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> Neg for F64<O>`

- <span id="f64-neg-type-output"></span>`type Output = F64<O>`

- <span id="f64-neg"></span>`fn neg(self) -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: cmp::PartialEq> PartialEq for F64<O>`

- <span id="f64-partialeq-eq"></span>`fn eq(&self, other: &F64<O>) -> bool` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> PartialOrd for F64<O>`

- <span id="f64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for F64<O>`

- <span id="f64-rem-type-output"></span>`type Output = F64<O>`

- <span id="f64-rem"></span>`fn rem(self, rhs: F64<O>) -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> RemAssign for F64<O>`

- <span id="f64-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: F64<O>)` — [`F64`](../index.md#f64)

##### `impl<O> StructuralPartialEq for F64<O>`

##### `impl<O: ByteOrder> Sub for F64<O>`

- <span id="f64-sub-type-output"></span>`type Output = F64<O>`

- <span id="f64-sub"></span>`fn sub(self, rhs: F64<O>) -> F64<O>` — [`F64`](../index.md#f64)

##### `impl<O: ByteOrder> SubAssign for F64<O>`

- <span id="f64-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: F64<O>)` — [`F64`](../index.md#f64)

##### `impl ToString for F64<O>`

- <span id="f64-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for F64<O>`

##### `impl<O> Unaligned for F64<O>`

## Enums

### `BigEndian`

```rust
enum BigEndian {
}
```

Big-endian byte order.

See [`ByteOrder`](../index.md) for more details.

#### Trait Implementations

##### `impl ByteOrder for BigEndian`

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](../index.md#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BigEndian`

- <span id="bigendian-display-fmt"></span>`fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for BigEndian`

- <span id="bigendian-ord-cmp"></span>`fn cmp(&self, other: &BigEndian) -> cmp::Ordering` — [`BigEndian`](../index.md#bigendian)

##### `impl PartialEq for BigEndian`

- <span id="bigendian-partialeq-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](../index.md#bigendian)

##### `impl PartialOrd for BigEndian`

- <span id="bigendian-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BigEndian) -> option::Option<cmp::Ordering>` — [`BigEndian`](../index.md#bigendian)

##### `impl Sealed for super::BigEndian`

##### `impl StructuralPartialEq for BigEndian`

##### `impl ToString for BigEndian`

- <span id="bigendian-tostring-to-string"></span>`fn to_string(&self) -> String`

### `LittleEndian`

```rust
enum LittleEndian {
}
```

Little-endian byte order.

See [`ByteOrder`](../index.md) for more details.

#### Trait Implementations

##### `impl ByteOrder for LittleEndian`

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](../index.md#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LittleEndian`

- <span id="littleendian-display-fmt"></span>`fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LittleEndian`

- <span id="littleendian-ord-cmp"></span>`fn cmp(&self, other: &LittleEndian) -> cmp::Ordering` — [`LittleEndian`](../index.md#littleendian)

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-partialeq-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](../index.md#littleendian)

##### `impl PartialOrd for LittleEndian`

- <span id="littleendian-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LittleEndian) -> option::Option<cmp::Ordering>` — [`LittleEndian`](../index.md#littleendian)

##### `impl Sealed for super::LittleEndian`

##### `impl StructuralPartialEq for LittleEndian`

##### `impl ToString for LittleEndian`

- <span id="littleendian-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `ByteOrder`

```rust
trait ByteOrder: Copy + Clone + Debug + Display + Eq + PartialEq + Ord + PartialOrd + Hash + private::Sealed { ... }
```

A type-level representation of byte order.

This type is implemented by [`BigEndian`](../index.md) and [`LittleEndian`](../index.md), which
represent big-endian and little-endian byte order respectively. This module
also provides a number of useful aliases for those types: [`NativeEndian`](../index.md),
[`NetworkEndian`](../index.md), [`BE`](../index.md), and [`LE`](../index.md).

`ByteOrder` types can be used to specify the byte order of the types in this
module - for example, `U32<BigEndian>` is a 32-bit integer stored in
big-endian byte order.


#### Implementors

- [`BigEndian`](../index.md#bigendian)
- [`LittleEndian`](../index.md#littleendian)

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The endianness used by this platform.

This is a type alias for [`BigEndian`](../index.md) or [`LittleEndian`](../index.md) depending on the
endianness of the target platform.

### `NetworkEndian`

```rust
type NetworkEndian = BigEndian;
```

The endianness used in many network protocols.

This is a type alias for [`BigEndian`](../index.md).

### `BE`

```rust
type BE = BigEndian;
```

A type alias for [`BigEndian`](../index.md).

### `LE`

```rust
type LE = LittleEndian;
```

A type alias for [`LittleEndian`](../index.md).

## Macros

### `impl_fmt_trait!`

### `impl_fmt_traits!`

### `impl_ops_traits!`

### `doc_comment!`

### `define_max_value_constant!`

### `define_type!`

### `define_float_conversion!`

### `module!`

