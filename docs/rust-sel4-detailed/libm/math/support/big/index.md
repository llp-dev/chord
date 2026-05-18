*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [big](index.md)*

---

# Module `big`

Integers used for wide operations, larger than `u128`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`u256`](#u256) | struct | A 256-bit unsigned integer represented as two 128-bit native-endian limbs. |
| [`i256`](#i256) | struct | A 256-bit signed integer represented as two 128-bit native-endian limbs. |
| [`U128_LO_MASK`](#u128-lo-mask) | const |  |
| [`impl_common!`](#impl-common) | macro |  |

## Structs

### `u256`

```rust
struct u256 {
    pub hi: u128,
    pub lo: u128,
}
```

A 256-bit unsigned integer represented as two 128-bit native-endian limbs.

#### Implementations

- <span id="u256-signed"></span>`fn signed(self) -> i256` — [`i256`](#i256)

  Reinterpret as a signed integer

#### Trait Implementations

##### `impl Add for u256`

- <span id="u256-add-type-output"></span>`type Output = u256`

- <span id="u256-add"></span>`fn add(self, rhs: Self) -> <Self as >::Output`

##### `impl BitOr for u256`

- <span id="u256-bitor-type-output"></span>`type Output = u256`

- <span id="u256-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for u256`

- <span id="u256-clone"></span>`fn clone(&self) -> u256` — [`u256`](#u256)

##### `impl Copy for u256`

##### `impl DInt for u256`

- <span id="u256-dint-type-h"></span>`type H = u128`

- <span id="u256-dint-lo"></span>`fn lo(self) -> <Self as >::H` — [`DInt`](../int_traits/index.md#dint)

- <span id="u256-dint-hi"></span>`fn hi(self) -> <Self as >::H` — [`DInt`](../int_traits/index.md#dint)

##### `impl Debug for u256`

- <span id="u256-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for u256`

##### `impl MinInt for u256`

- <span id="u256-minint-type-othersign"></span>`type OtherSign = i256`

- <span id="u256-minint-type-unsigned"></span>`type Unsigned = u256`

- <span id="u256-minint-const-signed"></span>`const SIGNED: bool`

- <span id="u256-minint-const-bits"></span>`const BITS: u32`

- <span id="u256-minint-const-zero"></span>`const ZERO: Self`

- <span id="u256-minint-const-one"></span>`const ONE: Self`

- <span id="u256-minint-const-min"></span>`const MIN: Self`

- <span id="u256-minint-const-max"></span>`const MAX: Self`

##### `impl NarrowingDiv for crate::support::u256`

- <span id="cratesupportu256-narrowingdiv-unchecked-narrowing-div-rem"></span>`unsafe fn unchecked_narrowing_div_rem(self, n: <Self as >::H) -> (<Self as >::H, <Self as >::H)` — [`DInt`](../int_traits/index.md#dint)

##### `impl Not for u256`

- <span id="u256-not-type-output"></span>`type Output = u256`

- <span id="u256-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Ord for u256`

- <span id="u256-ord-cmp"></span>`fn cmp(&self, other: &u256) -> cmp::Ordering` — [`u256`](#u256)

##### `impl PartialEq for u256`

- <span id="u256-partialeq-eq"></span>`fn eq(&self, other: &u256) -> bool` — [`u256`](#u256)

##### `impl PartialOrd for u256`

- <span id="u256-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &u256) -> option::Option<cmp::Ordering>` — [`u256`](#u256)

##### `impl Shl for u256`

- <span id="u256-shl-type-output"></span>`type Output = u256`

- <span id="u256-shl"></span>`fn shl(self, rhs: u32) -> <Self as >::Output`

##### `impl Shr for u256`

- <span id="u256-shr-type-output"></span>`type Output = u256`

- <span id="u256-shr"></span>`fn shr(self, rhs: u32) -> <Self as >::Output`

##### `impl StructuralPartialEq for u256`

##### `impl Sub for u256`

- <span id="u256-sub-type-output"></span>`type Output = u256`

- <span id="u256-sub"></span>`fn sub(self, rhs: Self) -> <Self as >::Output`

### `i256`

```rust
struct i256 {
    pub hi: i128,
    pub lo: u128,
}
```

A 256-bit signed integer represented as two 128-bit native-endian limbs.

#### Trait Implementations

##### `impl Add for i256`

- <span id="i256-add-type-output"></span>`type Output = i256`

- <span id="i256-add"></span>`fn add(self, rhs: Self) -> <Self as >::Output`

##### `impl BitOr for i256`

- <span id="i256-bitor-type-output"></span>`type Output = i256`

- <span id="i256-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for i256`

- <span id="i256-clone"></span>`fn clone(&self) -> i256` — [`i256`](#i256)

##### `impl Copy for i256`

##### `impl DInt for i256`

- <span id="i256-dint-type-h"></span>`type H = i128`

- <span id="i256-dint-lo"></span>`fn lo(self) -> <Self as >::H` — [`DInt`](../int_traits/index.md#dint)

- <span id="i256-dint-hi"></span>`fn hi(self) -> <Self as >::H` — [`DInt`](../int_traits/index.md#dint)

##### `impl Debug for i256`

- <span id="i256-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for i256`

##### `impl MinInt for i256`

- <span id="i256-minint-type-othersign"></span>`type OtherSign = u256`

- <span id="i256-minint-type-unsigned"></span>`type Unsigned = u256`

- <span id="i256-minint-const-signed"></span>`const SIGNED: bool`

- <span id="i256-minint-const-bits"></span>`const BITS: u32`

- <span id="i256-minint-const-zero"></span>`const ZERO: Self`

- <span id="i256-minint-const-one"></span>`const ONE: Self`

- <span id="i256-minint-const-min"></span>`const MIN: Self`

- <span id="i256-minint-const-max"></span>`const MAX: Self`

##### `impl Not for i256`

- <span id="i256-not-type-output"></span>`type Output = i256`

- <span id="i256-not"></span>`fn not(self) -> <Self as >::Output`

##### `impl Ord for i256`

- <span id="i256-ord-cmp"></span>`fn cmp(&self, other: &i256) -> cmp::Ordering` — [`i256`](#i256)

##### `impl PartialEq for i256`

- <span id="i256-partialeq-eq"></span>`fn eq(&self, other: &i256) -> bool` — [`i256`](#i256)

##### `impl PartialOrd for i256`

- <span id="i256-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &i256) -> option::Option<cmp::Ordering>` — [`i256`](#i256)

##### `impl Shl for i256`

- <span id="i256-shl-type-output"></span>`type Output = i256`

- <span id="i256-shl"></span>`fn shl(self, rhs: u32) -> <Self as >::Output`

##### `impl Shr for i256`

- <span id="i256-shr-type-output"></span>`type Output = i256`

- <span id="i256-shr"></span>`fn shr(self, rhs: u32) -> <Self as >::Output`

##### `impl StructuralPartialEq for i256`

##### `impl Sub for i256`

- <span id="i256-sub-type-output"></span>`type Output = i256`

- <span id="i256-sub"></span>`fn sub(self, rhs: Self) -> <Self as >::Output`

## Constants

### `U128_LO_MASK`
```rust
const U128_LO_MASK: u128 = 18_446_744_073_709_551_615u128;
```

## Macros

### `impl_common!`

