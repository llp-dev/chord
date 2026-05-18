*[num_bigint](../index.md) / [biguint](index.md)*

---

# Module `biguint`

## Contents

- [Modules](#modules)
  - [`addition`](#addition)
  - [`division`](#division)
  - [`multiplication`](#multiplication)
  - [`subtraction`](#subtraction)
  - [`bits`](#bits)
  - [`convert`](#convert)
  - [`iter`](#iter)
  - [`monty`](#monty)
  - [`power`](#power)
  - [`shift`](#shift)
- [Structs](#structs)
  - [`U32Digits`](#u32digits)
  - [`U64Digits`](#u64digits)
  - [`BigUint`](#biguint)
- [Traits](#traits)
  - [`ToBigUint`](#tobiguint)
  - [`IntDigits`](#intdigits)
- [Functions](#functions)
  - [`cmp_slice`](#cmp-slice)
  - [`fixpoint`](#fixpoint)
  - [`biguint_from_vec`](#biguint-from-vec)
  - [`u32_chunk_to_u64`](#u32-chunk-to-u64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`addition`](#addition) | mod |  |
| [`division`](#division) | mod |  |
| [`multiplication`](#multiplication) | mod |  |
| [`subtraction`](#subtraction) | mod |  |
| [`bits`](#bits) | mod |  |
| [`convert`](#convert) | mod |  |
| [`iter`](#iter) | mod |  |
| [`monty`](#monty) | mod |  |
| [`power`](#power) | mod |  |
| [`shift`](#shift) | mod |  |
| [`U32Digits`](#u32digits) | struct |  |
| [`U64Digits`](#u64digits) | struct |  |
| [`BigUint`](#biguint) | struct | A big unsigned integer type. |
| [`ToBigUint`](#tobiguint) | trait | A generic trait for converting a value to a [`BigUint`]. |
| [`IntDigits`](#intdigits) | trait |  |
| [`cmp_slice`](#cmp-slice) | fn |  |
| [`fixpoint`](#fixpoint) | fn |  |
| [`biguint_from_vec`](#biguint-from-vec) | fn | Creates and initializes a [`BigUint`]. |
| [`u32_chunk_to_u64`](#u32-chunk-to-u64) | fn | Convert a `u32` chunk (len is either 1 or 2) to a single `u64` digit |

## Modules

- [`addition`](addition/index.md)
- [`division`](division/index.md)
- [`multiplication`](multiplication/index.md)
- [`subtraction`](subtraction/index.md)
- [`bits`](bits/index.md)
- [`convert`](convert/index.md)
- [`iter`](iter/index.md)
- [`monty`](monty/index.md)
- [`power`](power/index.md)
- [`shift`](shift/index.md)

## Structs

### `U32Digits<'a>`

```rust
struct U32Digits<'a> {
    data: &'a [u64],
    next_is_lo: bool,
    last_hi_is_zero: bool,
}
```

An iterator of `u32` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

#### Implementations

- <span id="u32digits-new"></span>`fn new(data: &'a [u64]) -> Self`

#### Trait Implementations

##### `impl DoubleEndedIterator for U32Digits<'_>`

- <span id="u32digits-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for U32Digits<'_>`

- <span id="u32digits-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for U32Digits<'_>`

##### `impl IntoIterator for U32Digits<'a>`

- <span id="u32digits-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="u32digits-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="u32digits-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for U32Digits<'_>`

- <span id="u32digits-iterator-type-item"></span>`type Item = u32`

- <span id="u32digits-iterator-next"></span>`fn next(&mut self) -> Option<u32>`

- <span id="u32digits-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="u32digits-iterator-last"></span>`fn last(self) -> Option<u32>`

- <span id="u32digits-iterator-count"></span>`fn count(self) -> usize`

### `U64Digits<'a>`

```rust
struct U64Digits<'a> {
    it: core::slice::Iter<'a, u64>,
}
```

An iterator of `u64` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

#### Implementations

- <span id="u64digits-new"></span>`fn new(data: &'a [u64]) -> Self`

#### Trait Implementations

##### `impl DoubleEndedIterator for U64Digits<'_>`

- <span id="u64digits-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for U64Digits<'_>`

- <span id="u64digits-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for U64Digits<'_>`

##### `impl IntoIterator for U64Digits<'a>`

- <span id="u64digits-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="u64digits-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="u64digits-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for U64Digits<'_>`

- <span id="u64digits-iterator-type-item"></span>`type Item = u64`

- <span id="u64digits-iterator-next"></span>`fn next(&mut self) -> Option<u64>`

- <span id="u64digits-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="u64digits-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<u64>`

- <span id="u64digits-iterator-last"></span>`fn last(self) -> Option<u64>`

- <span id="u64digits-iterator-count"></span>`fn count(self) -> usize`

### `BigUint`

```rust
struct BigUint {
    data: alloc::vec::Vec<u64>,
}
```

A big unsigned integer type.

#### Implementations

- <span id="biguint-const-zero"></span>`const ZERO: Self`

- <span id="biguint-new"></span>`fn new(digits: Vec<u32>) -> BigUint` — [`BigUint`](#biguint)

  Creates and initializes a [`BigUint`](#biguint).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="biguint-from-slice"></span>`fn from_slice(slice: &[u32]) -> BigUint` — [`BigUint`](#biguint)

  Creates and initializes a [`BigUint`](#biguint).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="biguint-assign-from-slice"></span>`fn assign_from_slice(&mut self, slice: &[u32])`

  Assign a value to a [`BigUint`](#biguint).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="biguint-from-bytes-be"></span>`fn from_bytes_be(bytes: &[u8]) -> BigUint` — [`BigUint`](#biguint)

  Creates and initializes a [`BigUint`](#biguint).

  

  The bytes are in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from_bytes_be(b"A"),

             BigUint::parse_bytes(b"65", 10).unwrap());

  assert_eq!(BigUint::from_bytes_be(b"AA"),

             BigUint::parse_bytes(b"16705", 10).unwrap());

  assert_eq!(BigUint::from_bytes_be(b"AB"),

             BigUint::parse_bytes(b"16706", 10).unwrap());

  assert_eq!(BigUint::from_bytes_be(b"Hello world!"),

             BigUint::parse_bytes(b"22405534230753963835153736737", 10).unwrap());

  ```

- <span id="biguint-from-bytes-le"></span>`fn from_bytes_le(bytes: &[u8]) -> BigUint` — [`BigUint`](#biguint)

  Creates and initializes a [`BigUint`](#biguint).

  

  The bytes are in little-endian byte order.

- <span id="biguint-parse-bytes"></span>`fn parse_bytes(buf: &[u8], radix: u32) -> Option<BigUint>` — [`BigUint`](#biguint)

  Creates and initializes a [`BigUint`](#biguint). The input slice must contain

  ascii/utf8 characters in [0-9a-zA-Z].

  `radix` must be in the range `2...36`.

  

  The function `from_str_radix` from the `Num` trait provides the same logic

  for `&str` buffers.

  

  # Examples

  

  ```rust

  use num_bigint::{BigUint, ToBigUint};

  

  assert_eq!(BigUint::parse_bytes(b"1234", 10), ToBigUint::to_biguint(&1234));

  assert_eq!(BigUint::parse_bytes(b"ABCD", 16), ToBigUint::to_biguint(&0xABCD));

  assert_eq!(BigUint::parse_bytes(b"G", 16), None);

  ```

- <span id="biguint-from-radix-be"></span>`fn from_radix_be(buf: &[u8], radix: u32) -> Option<BigUint>` — [`BigUint`](#biguint)

  Creates and initializes a [`BigUint`](#biguint). Each `u8` of the input slice is

  interpreted as one digit of the number

  and must therefore be less than `radix`.

  

  The bytes are in big-endian byte order.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigUint};

  

  let inbase190 = &[15, 33, 125, 12, 14];

  let a = BigUint::from_radix_be(inbase190, 190).unwrap();

  assert_eq!(a.to_radix_be(190), inbase190);

  ```

- <span id="biguint-from-radix-le"></span>`fn from_radix_le(buf: &[u8], radix: u32) -> Option<BigUint>` — [`BigUint`](#biguint)

  Creates and initializes a [`BigUint`](#biguint). Each `u8` of the input slice is

  interpreted as one digit of the number

  and must therefore be less than `radix`.

  

  The bytes are in little-endian byte order.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigUint};

  

  let inbase190 = &[14, 12, 125, 33, 15];

  let a = BigUint::from_radix_be(inbase190, 190).unwrap();

  assert_eq!(a.to_radix_be(190), inbase190);

  ```

- <span id="biguint-to-bytes-be"></span>`fn to_bytes_be(&self) -> Vec<u8>`

  Returns the byte representation of the [`BigUint`](#biguint) in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  let i = BigUint::parse_bytes(b"1125", 10).unwrap();

  assert_eq!(i.to_bytes_be(), vec![4, 101]);

  ```

- <span id="biguint-to-bytes-le"></span>`fn to_bytes_le(&self) -> Vec<u8>`

  Returns the byte representation of the [`BigUint`](#biguint) in little-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  let i = BigUint::parse_bytes(b"1125", 10).unwrap();

  assert_eq!(i.to_bytes_le(), vec![101, 4]);

  ```

- <span id="biguint-to-u32-digits"></span>`fn to_u32_digits(&self) -> Vec<u32>`

  Returns the `u32` digits representation of the [`BigUint`](#biguint) ordered least significant digit

  first.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from(1125u32).to_u32_digits(), vec![1125]);

  assert_eq!(BigUint::from(4294967295u32).to_u32_digits(), vec![4294967295]);

  assert_eq!(BigUint::from(4294967296u64).to_u32_digits(), vec![0, 1]);

  assert_eq!(BigUint::from(112500000000u64).to_u32_digits(), vec![830850304, 26]);

  ```

- <span id="biguint-to-u64-digits"></span>`fn to_u64_digits(&self) -> Vec<u64>`

  Returns the `u64` digits representation of the [`BigUint`](#biguint) ordered least significant digit

  first.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from(1125u32).to_u64_digits(), vec![1125]);

  assert_eq!(BigUint::from(4294967295u32).to_u64_digits(), vec![4294967295]);

  assert_eq!(BigUint::from(4294967296u64).to_u64_digits(), vec![4294967296]);

  assert_eq!(BigUint::from(112500000000u64).to_u64_digits(), vec![112500000000]);

  assert_eq!(BigUint::from(1u128 << 64).to_u64_digits(), vec![0, 1]);

  ```

- <span id="biguint-iter-u32-digits"></span>`fn iter_u32_digits(&self) -> U32Digits<'_>` — [`U32Digits`](iter/index.md#u32digits)

  Returns an iterator of `u32` digits representation of the [`BigUint`](#biguint) ordered least

  significant digit first.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from(1125u32).iter_u32_digits().collect::<Vec<u32>>(), vec![1125]);

  assert_eq!(BigUint::from(4294967295u32).iter_u32_digits().collect::<Vec<u32>>(), vec![4294967295]);

  assert_eq!(BigUint::from(4294967296u64).iter_u32_digits().collect::<Vec<u32>>(), vec![0, 1]);

  assert_eq!(BigUint::from(112500000000u64).iter_u32_digits().collect::<Vec<u32>>(), vec![830850304, 26]);

  ```

- <span id="biguint-iter-u64-digits"></span>`fn iter_u64_digits(&self) -> U64Digits<'_>` — [`U64Digits`](iter/index.md#u64digits)

  Returns an iterator of `u64` digits representation of the [`BigUint`](#biguint) ordered least

  significant digit first.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from(1125u32).iter_u64_digits().collect::<Vec<u64>>(), vec![1125]);

  assert_eq!(BigUint::from(4294967295u32).iter_u64_digits().collect::<Vec<u64>>(), vec![4294967295]);

  assert_eq!(BigUint::from(4294967296u64).iter_u64_digits().collect::<Vec<u64>>(), vec![4294967296]);

  assert_eq!(BigUint::from(112500000000u64).iter_u64_digits().collect::<Vec<u64>>(), vec![112500000000]);

  assert_eq!(BigUint::from(1u128 << 64).iter_u64_digits().collect::<Vec<u64>>(), vec![0, 1]);

  ```

- <span id="biguint-to-str-radix"></span>`fn to_str_radix(&self, radix: u32) -> String`

  Returns the integer formatted as a string in the given radix.

  `radix` must be in the range `2...36`.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  let i = BigUint::parse_bytes(b"ff", 16).unwrap();

  assert_eq!(i.to_str_radix(16), "ff");

  ```

- <span id="biguint-to-radix-be"></span>`fn to_radix_be(&self, radix: u32) -> Vec<u8>`

  Returns the integer in the requested base in big-endian digit order.

  The output is not given in a human readable alphabet but as a zero

  based `u8` number.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from(0xFFFFu64).to_radix_be(159),

             vec![2, 94, 27]);

  // 0xFFFF = 65535 = 2*(159^2) + 94*159 + 27

  ```

- <span id="biguint-to-radix-le"></span>`fn to_radix_le(&self, radix: u32) -> Vec<u8>`

  Returns the integer in the requested base in little-endian digit order.

  The output is not given in a human readable alphabet but as a zero

  based u8 number.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from(0xFFFFu64).to_radix_le(159),

             vec![27, 94, 2]);

  // 0xFFFF = 65535 = 27 + 94*159 + 2*(159^2)

  ```

- <span id="biguint-bits"></span>`fn bits(&self) -> u64`

  Determines the fewest bits necessary to express the [`BigUint`](#biguint).

- <span id="biguint-normalize"></span>`fn normalize(&mut self)`

  Strips off trailing zero bigdigits - comparisons require the last element in the vector to

  be nonzero.

- <span id="biguint-normalized"></span>`fn normalized(self) -> BigUint` — [`BigUint`](#biguint)

  Returns a normalized [`BigUint`](#biguint).

- <span id="biguint-pow"></span>`fn pow(&self, exponent: u32) -> Self`

  Returns `self ^ exponent`.

- <span id="biguint-modpow"></span>`fn modpow(&self, exponent: &Self, modulus: &Self) -> Self`

  Returns `(self ^ exponent) % modulus`.

  

  Panics if the modulus is zero.

- <span id="biguint-modinv"></span>`fn modinv(&self, modulus: &Self) -> Option<Self>`

  Returns the modular multiplicative inverse if it exists, otherwise `None`.

  

  This solves for `x` in the interval `[0, modulus)` such that `self * x ≡ 1 (mod modulus)`.

  The solution exists if and only if `gcd(self, modulus) == 1`.

  

  ```rust

  use num_bigint::BigUint;

  use num_traits::{One, Zero};

  

  let m = BigUint::from(383_u32);

  

  // Trivial cases

  assert_eq!(BigUint::zero().modinv(&m), None);

  assert_eq!(BigUint::one().modinv(&m), Some(BigUint::one()));

  let neg1 = &m - 1u32;

  assert_eq!(neg1.modinv(&m), Some(neg1));

  

  let a = BigUint::from(271_u32);

  let x = a.modinv(&m).unwrap();

  assert_eq!(x, BigUint::from(106_u32));

  assert_eq!(x.modinv(&m).unwrap(), a);

  assert!((a * x % m).is_one());

  ```

- <span id="biguint-sqrt"></span>`fn sqrt(&self) -> Self`

  Returns the truncated principal square root of `self` --

  see [Roots::sqrt](https://docs.rs/num-integer/0.1/num_integer/trait.Roots.html#method.sqrt)

- <span id="biguint-cbrt"></span>`fn cbrt(&self) -> Self`

  Returns the truncated principal cube root of `self` --

  see [Roots::cbrt](https://docs.rs/num-integer/0.1/num_integer/trait.Roots.html#method.cbrt).

- <span id="biguint-nth-root"></span>`fn nth_root(&self, n: u32) -> Self`

  Returns the truncated principal `n`th root of `self` --

  see [Roots::nth_root](https://docs.rs/num-integer/0.1/num_integer/trait.Roots.html#tymethod.nth_root).

- <span id="biguint-trailing-zeros"></span>`fn trailing_zeros(&self) -> Option<u64>`

  Returns the number of least-significant bits that are zero,

  or `None` if the entire number is zero.

- <span id="biguint-trailing-ones"></span>`fn trailing_ones(&self) -> u64`

  Returns the number of least-significant bits that are ones.

- <span id="biguint-count-ones"></span>`fn count_ones(&self) -> u64`

  Returns the number of one bits.

- <span id="biguint-bit"></span>`fn bit(&self, bit: u64) -> bool`

  Returns whether the bit in the given position is set

- <span id="biguint-set-bit"></span>`fn set_bit(&mut self, bit: u64, value: bool)`

  Sets or clears the bit in the given position

  

  Note that setting a bit greater than the current bit length, a reallocation may be needed

  to store the new digits

#### Trait Implementations

##### `impl Add for super::BigUint`

- <span id="superbiguint-add-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-add"></span>`fn add(self, other: BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl AddAssign for super::BigUint`

- <span id="superbiguint-addassign-add-assign"></span>`fn add_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl Average for BigUint`

- <span id="biguint-average-average-floor"></span>`fn average_floor(&self, other: &I) -> I`

  Returns the floor value of the average of `self` and `other`.

- <span id="biguint-average-average-ceil"></span>`fn average_ceil(&self, other: &I) -> I`

  Returns the ceil value of the average of `self` and `other`.

##### `impl Binary for BigUint`

- <span id="biguint-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl BitAnd for super::BigUint`

- <span id="superbiguint-bitand-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-bitand"></span>`fn bitand(self, other: BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl BitAndAssign for super::BigUint`

- <span id="superbiguint-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl BitOr for super::BigUint`

- <span id="superbiguint-bitor-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-bitor"></span>`fn bitor(self, other: BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl BitOrAssign for super::BigUint`

- <span id="superbiguint-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl BitXor for super::BigUint`

- <span id="superbiguint-bitxor-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-bitxor"></span>`fn bitxor(self, other: BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl BitXorAssign for super::BigUint`

- <span id="superbiguint-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl CheckedAdd for super::BigUint`

- <span id="superbiguint-checkedadd-checked-add"></span>`fn checked_add(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](#biguint)

##### `impl CheckedDiv for super::BigUint`

- <span id="superbiguint-checkeddiv-checked-div"></span>`fn checked_div(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](#biguint)

##### `impl CheckedEuclid for super::BigUint`

- <span id="superbiguint-checkedeuclid-checked-div-euclid"></span>`fn checked_div_euclid(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](#biguint)

- <span id="superbiguint-checkedeuclid-checked-rem-euclid"></span>`fn checked_rem_euclid(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](#biguint)

- <span id="superbiguint-checkedeuclid-checked-div-rem-euclid"></span>`fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)>`

##### `impl CheckedMul for super::BigUint`

- <span id="superbiguint-checkedmul-checked-mul"></span>`fn checked_mul(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](#biguint)

##### `impl CheckedSub for super::BigUint`

- <span id="superbiguint-checkedsub-checked-sub"></span>`fn checked_sub(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](#biguint)

##### `impl Clone for BigUint`

- <span id="biguint-clone"></span>`fn clone(&self) -> Self`

- <span id="biguint-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl ConstZero for BigUint`

- <span id="biguint-constzero-const-zero"></span>`const ZERO: Self`

##### `impl Debug for BigUint`

- <span id="biguint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigUint`

- <span id="biguint-default"></span>`fn default() -> BigUint` — [`BigUint`](#biguint)

##### `impl Display for BigUint`

- <span id="biguint-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Div for super::BigUint`

- <span id="superbiguint-div-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-div"></span>`fn div(self, other: &BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl DivAssign for super::BigUint`

- <span id="superbiguint-divassign-div-assign"></span>`fn div_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl Eq for BigUint`

##### `impl Euclid for super::BigUint`

- <span id="superbiguint-euclid-div-euclid"></span>`fn div_euclid(&self, v: &BigUint) -> BigUint` — [`BigUint`](#biguint)

- <span id="superbiguint-euclid-rem-euclid"></span>`fn rem_euclid(&self, v: &BigUint) -> BigUint` — [`BigUint`](#biguint)

- <span id="superbiguint-euclid-div-rem-euclid"></span>`fn div_rem_euclid(&self, v: &Self) -> (Self, Self)`

##### `impl FromBytes for BigUint`

- <span id="biguint-frombytes-type-bytes"></span>`type Bytes = [u8]`

- <span id="biguint-frombytes-from-be-bytes"></span>`fn from_be_bytes(bytes: &<Self as >::Bytes) -> Self`

- <span id="biguint-frombytes-from-le-bytes"></span>`fn from_le_bytes(bytes: &<Self as >::Bytes) -> Self`

##### `impl FromPrimitive for super::BigUint`

- <span id="superbiguint-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<BigUint>` — [`BigUint`](#biguint)

- <span id="superbiguint-fromprimitive-from-i128"></span>`fn from_i128(n: i128) -> Option<BigUint>` — [`BigUint`](#biguint)

- <span id="superbiguint-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<BigUint>` — [`BigUint`](#biguint)

- <span id="superbiguint-fromprimitive-from-u128"></span>`fn from_u128(n: u128) -> Option<BigUint>` — [`BigUint`](#biguint)

- <span id="superbiguint-fromprimitive-from-f64"></span>`fn from_f64(n: f64) -> Option<BigUint>` — [`BigUint`](#biguint)

##### `impl FromStr for super::BigUint`

- <span id="superbiguint-fromstr-type-err"></span>`type Err = ParseBigIntError`

- <span id="superbiguint-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<BigUint, ParseBigIntError>` — [`BigUint`](#biguint), [`ParseBigIntError`](../index.md#parsebiginterror)

##### `impl Hash for BigUint`

- <span id="biguint-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl IntDigits for BigUint`

- <span id="biguint-intdigits-digits"></span>`fn digits(&self) -> &[u64]`

- <span id="biguint-intdigits-digits-mut"></span>`fn digits_mut(&mut self) -> &mut Vec<u64>`

- <span id="biguint-intdigits-normalize"></span>`fn normalize(&mut self)`

- <span id="biguint-intdigits-capacity"></span>`fn capacity(&self) -> usize`

- <span id="biguint-intdigits-len"></span>`fn len(&self) -> usize`

##### `impl Integer for BigUint`

- <span id="biguint-integer-div-rem"></span>`fn div_rem(&self, other: &BigUint) -> (BigUint, BigUint)` — [`BigUint`](#biguint)

- <span id="biguint-integer-div-floor"></span>`fn div_floor(&self, other: &BigUint) -> BigUint` — [`BigUint`](#biguint)

- <span id="biguint-integer-mod-floor"></span>`fn mod_floor(&self, other: &BigUint) -> BigUint` — [`BigUint`](#biguint)

- <span id="biguint-integer-div-mod-floor"></span>`fn div_mod_floor(&self, other: &BigUint) -> (BigUint, BigUint)` — [`BigUint`](#biguint)

- <span id="biguint-integer-div-ceil"></span>`fn div_ceil(&self, other: &BigUint) -> BigUint` — [`BigUint`](#biguint)

- <span id="biguint-integer-gcd"></span>`fn gcd(&self, other: &Self) -> Self`

  Calculates the Greatest Common Divisor (GCD) of the number and `other`.

  

  The result is always positive.

- <span id="biguint-integer-lcm"></span>`fn lcm(&self, other: &BigUint) -> BigUint` — [`BigUint`](#biguint)

  Calculates the Lowest Common Multiple (LCM) of the number and `other`.

- <span id="biguint-integer-gcd-lcm"></span>`fn gcd_lcm(&self, other: &Self) -> (Self, Self)`

  Calculates the Greatest Common Divisor (GCD) and

  Lowest Common Multiple (LCM) together.

- <span id="biguint-integer-divides"></span>`fn divides(&self, other: &BigUint) -> bool` — [`BigUint`](#biguint)

  Deprecated, use `is_multiple_of` instead.

- <span id="biguint-integer-is-multiple-of"></span>`fn is_multiple_of(&self, other: &BigUint) -> bool` — [`BigUint`](#biguint)

  Returns `true` if the number is a multiple of `other`.

- <span id="biguint-integer-is-even"></span>`fn is_even(&self) -> bool`

  Returns `true` if the number is divisible by `2`.

- <span id="biguint-integer-is-odd"></span>`fn is_odd(&self) -> bool`

  Returns `true` if the number is not divisible by `2`.

- <span id="biguint-integer-next-multiple-of"></span>`fn next_multiple_of(&self, other: &Self) -> Self`

  Rounds up to nearest multiple of argument.

- <span id="biguint-integer-prev-multiple-of"></span>`fn prev_multiple_of(&self, other: &Self) -> Self`

  Rounds down to nearest multiple of argument.

- <span id="biguint-integer-dec"></span>`fn dec(&mut self)`

- <span id="biguint-integer-inc"></span>`fn inc(&mut self)`

##### `impl LowerHex for BigUint`

- <span id="biguint-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Mul for super::BigUint`

- <span id="superbiguint-mul-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-mul"></span>`fn mul(self, other: BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl MulAssign for BigUint`

- <span id="biguint-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl Num for super::BigUint`

- <span id="superbiguint-num-type-fromstrradixerr"></span>`type FromStrRadixErr = ParseBigIntError`

- <span id="superbiguint-num-from-str-radix"></span>`fn from_str_radix(s: &str, radix: u32) -> Result<BigUint, ParseBigIntError>` — [`BigUint`](#biguint), [`ParseBigIntError`](../index.md#parsebiginterror)

  Creates and initializes a `BigUint`.

##### `impl NumAssign for BigUint`

##### `impl<Rhs> NumAssignOps for BigUint`

##### `impl NumAssignRef for BigUint`

##### `impl<Rhs, Output> NumOps for BigUint`

##### `impl NumRef for BigUint`

##### `impl Octal for BigUint`

- <span id="biguint-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl One for BigUint`

- <span id="biguint-one"></span>`fn one() -> BigUint` — [`BigUint`](#biguint)

- <span id="biguint-one-set-one"></span>`fn set_one(&mut self)`

- <span id="biguint-one-is-one"></span>`fn is_one(&self) -> bool`

##### `impl Ord for BigUint`

- <span id="biguint-ord-cmp"></span>`fn cmp(&self, other: &BigUint) -> Ordering` — [`BigUint`](#biguint)

##### `impl PartialEq for BigUint`

- <span id="biguint-partialeq-eq"></span>`fn eq(&self, other: &BigUint) -> bool` — [`BigUint`](#biguint)

##### `impl PartialOrd for BigUint`

- <span id="biguint-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BigUint) -> Option<Ordering>` — [`BigUint`](#biguint)

##### `impl Pow for BigInt`

- <span id="bigint-pow-type-output"></span>`type Output = BigInt`

- <span id="bigint-pow"></span>`fn pow(self, rhs: BigUint) -> BigInt` — [`BigUint`](#biguint), [`BigInt`](../bigint/index.md#bigint)

##### `impl<T> Product for super::BigUint`

- <span id="superbiguint-product"></span>`fn product<I>(iter: I) -> Self`

##### `impl<Base> RefNum for BigUint`

##### `impl Rem for super::BigUint`

- <span id="superbiguint-rem-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-rem"></span>`fn rem(self, other: &BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl RemAssign for super::BigUint`

- <span id="superbiguint-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl Roots for BigUint`

- <span id="biguint-roots-nth-root"></span>`fn nth_root(&self, n: u32) -> Self`

- <span id="biguint-roots-sqrt"></span>`fn sqrt(&self) -> Self`

- <span id="biguint-roots-cbrt"></span>`fn cbrt(&self) -> Self`

##### `impl Shl for BigUint`

- <span id="biguint-shl-type-output"></span>`type Output = BigUint`

- <span id="biguint-shl"></span>`fn shl(self, rhs: u8) -> BigUint` — [`BigUint`](#biguint)

##### `impl ShlAssign for BigUint`

- <span id="biguint-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: u8)`

##### `impl Shr for BigUint`

- <span id="biguint-shr-type-output"></span>`type Output = BigUint`

- <span id="biguint-shr"></span>`fn shr(self, rhs: u8) -> BigUint` — [`BigUint`](#biguint)

##### `impl ShrAssign for BigUint`

- <span id="biguint-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: u8)`

##### `impl Sub for super::BigUint`

- <span id="superbiguint-sub-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-sub"></span>`fn sub(self, other: BigUint) -> BigUint` — [`BigUint`](#biguint)

##### `impl SubAssign for super::BigUint`

- <span id="superbiguint-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: BigUint)` — [`BigUint`](#biguint)

##### `impl<T> Sum for super::BigUint`

- <span id="superbiguint-sum"></span>`fn sum<I>(iter: I) -> Self`

##### `impl ToBigInt for crate::BigUint`

- <span id="cratebiguint-tobigint-to-bigint"></span>`fn to_bigint(&self) -> Option<BigInt>` — [`BigInt`](../bigint/index.md#bigint)

##### `impl ToBigUint for super::BigUint`

- <span id="superbiguint-tobiguint-to-biguint"></span>`fn to_biguint(&self) -> Option<BigUint>` — [`BigUint`](#biguint)

##### `impl ToBytes for BigUint`

- <span id="biguint-tobytes-type-bytes"></span>`type Bytes = Vec<u8>`

- <span id="biguint-tobytes-to-be-bytes"></span>`fn to_be_bytes(&self) -> <Self as >::Bytes`

- <span id="biguint-tobytes-to-le-bytes"></span>`fn to_le_bytes(&self) -> <Self as >::Bytes`

##### `impl ToPrimitive for super::BigUint`

- <span id="superbiguint-toprimitive-to-i64"></span>`fn to_i64(&self) -> Option<i64>`

- <span id="superbiguint-toprimitive-to-i128"></span>`fn to_i128(&self) -> Option<i128>`

- <span id="superbiguint-toprimitive-to-u64"></span>`fn to_u64(&self) -> Option<u64>`

- <span id="superbiguint-toprimitive-to-u128"></span>`fn to_u128(&self) -> Option<u128>`

- <span id="superbiguint-toprimitive-to-f32"></span>`fn to_f32(&self) -> Option<f32>`

- <span id="superbiguint-toprimitive-to-f64"></span>`fn to_f64(&self) -> Option<f64>`

##### `impl ToString for BigUint`

- <span id="biguint-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl Unsigned for BigUint`

##### `impl UpperHex for BigUint`

- <span id="biguint-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Zero for BigUint`

- <span id="biguint-zero"></span>`fn zero() -> BigUint` — [`BigUint`](#biguint)

- <span id="biguint-zero-set-zero"></span>`fn set_zero(&mut self)`

- <span id="biguint-zero-is-zero"></span>`fn is_zero(&self) -> bool`

## Traits

### `ToBigUint`

```rust
trait ToBigUint { ... }
```

A generic trait for converting a value to a [`BigUint`](#biguint).

#### Required Methods

- `fn to_biguint(&self) -> Option<BigUint>`

  Converts the value of `self` to a [`BigUint`](#biguint).

#### Implementors

- [`BigInt`](../bigint/index.md#bigint)
- [`BigUint`](#biguint)
- `f32`
- `f64`
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

### `IntDigits`

```rust
trait IntDigits { ... }
```

#### Required Methods

- `fn digits(&self) -> &[u64]`

- `fn digits_mut(&mut self) -> &mut Vec<u64>`

- `fn normalize(&mut self)`

- `fn capacity(&self) -> usize`

- `fn len(&self) -> usize`

#### Implementors

- [`BigInt`](../bigint/index.md#bigint)
- [`BigUint`](#biguint)

## Functions

### `cmp_slice`

```rust
fn cmp_slice(a: &[u64], b: &[u64]) -> core::cmp::Ordering
```

### `fixpoint`

```rust
fn fixpoint<F>(x: BigUint, max_bits: u64, f: F) -> BigUint
where
    F: Fn(&BigUint) -> BigUint
```

### `biguint_from_vec`

```rust
fn biguint_from_vec(digits: alloc::vec::Vec<u64>) -> BigUint
```

Creates and initializes a [`BigUint`](#biguint).

The digits are in little-endian base matching `BigDigit`.

### `u32_chunk_to_u64`

```rust
fn u32_chunk_to_u64(chunk: &[u32]) -> u64
```

Convert a `u32` chunk (len is either 1 or 2) to a single `u64` digit

