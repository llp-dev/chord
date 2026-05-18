*[num_bigint](../index.md) / [bigint](index.md)*

---

# Module `bigint`

## Contents

- [Modules](#modules)
  - [`addition`](#addition)
  - [`division`](#division)
  - [`multiplication`](#multiplication)
  - [`subtraction`](#subtraction)
  - [`bits`](#bits)
  - [`convert`](#convert)
  - [`power`](#power)
  - [`shift`](#shift)
- [Structs](#structs)
  - [`BigInt`](#bigint)
- [Enums](#enums)
  - [`Sign`](#sign)
  - [`CheckedUnsignedAbs`](#checkedunsignedabs)
- [Traits](#traits)
  - [`UnsignedAbs`](#unsignedabs)
  - [`ToBigInt`](#tobigint)
- [Macros](#macros)
  - [`impl_unsigned_abs!`](#impl-unsigned-abs)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`addition`](#addition) | mod |  |
| [`division`](#division) | mod |  |
| [`multiplication`](#multiplication) | mod |  |
| [`subtraction`](#subtraction) | mod |  |
| [`bits`](#bits) | mod |  |
| [`convert`](#convert) | mod |  |
| [`power`](#power) | mod |  |
| [`shift`](#shift) | mod |  |
| [`BigInt`](#bigint) | struct | A big signed integer type. |
| [`Sign`](#sign) | enum | A `Sign` is a [`BigInt`]'s composing element. |
| [`CheckedUnsignedAbs`](#checkedunsignedabs) | enum |  |
| [`UnsignedAbs`](#unsignedabs) | trait |  |
| [`ToBigInt`](#tobigint) | trait | A generic trait for converting a value to a [`BigInt`]. |
| [`impl_unsigned_abs!`](#impl-unsigned-abs) | macro |  |

## Modules

- [`addition`](addition/index.md)
- [`division`](division/index.md)
- [`multiplication`](multiplication/index.md)
- [`subtraction`](subtraction/index.md)
- [`bits`](bits/index.md)
- [`convert`](convert/index.md)
- [`power`](power/index.md)
- [`shift`](shift/index.md)

## Structs

### `BigInt`

```rust
struct BigInt {
    sign: Sign,
    data: crate::biguint::BigUint,
}
```

A big signed integer type.

#### Implementations

- <span id="bigint-const-zero"></span>`const ZERO: Self`

- <span id="bigint-new"></span>`fn new(sign: Sign, digits: Vec<u32>) -> BigInt` — [`Sign`](#sign), [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-from-biguint"></span>`fn from_biguint(sign: Sign, data: BigUint) -> BigInt` — [`Sign`](#sign), [`BigUint`](../biguint/index.md#biguint), [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-from-slice"></span>`fn from_slice(sign: Sign, slice: &[u32]) -> BigInt` — [`Sign`](#sign), [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-assign-from-slice"></span>`fn assign_from_slice(&mut self, sign: Sign, slice: &[u32])` — [`Sign`](#sign)

  Reinitializes a [`BigInt`](#bigint).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-from-bytes-be"></span>`fn from_bytes_be(sign: Sign, bytes: &[u8]) -> BigInt` — [`Sign`](#sign), [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint).

  

  The bytes are in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"A"),

             BigInt::parse_bytes(b"65", 10).unwrap());

  assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"AA"),

             BigInt::parse_bytes(b"16705", 10).unwrap());

  assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"AB"),

             BigInt::parse_bytes(b"16706", 10).unwrap());

  assert_eq!(BigInt::from_bytes_be(Sign::Plus, b"Hello world!"),

             BigInt::parse_bytes(b"22405534230753963835153736737", 10).unwrap());

  ```

- <span id="bigint-from-bytes-le"></span>`fn from_bytes_le(sign: Sign, bytes: &[u8]) -> BigInt` — [`Sign`](#sign), [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint).

  

  The bytes are in little-endian byte order.

- <span id="bigint-from-signed-bytes-be"></span>`fn from_signed_bytes_be(digits: &[u8]) -> BigInt` — [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint) from an array of bytes in

  two's complement binary representation.

  

  The digits are in big-endian base 2<sup>8</sup>.

- <span id="bigint-from-signed-bytes-le"></span>`fn from_signed_bytes_le(digits: &[u8]) -> BigInt` — [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint) from an array of bytes in two's complement.

  

  The digits are in little-endian base 2<sup>8</sup>.

- <span id="bigint-parse-bytes"></span>`fn parse_bytes(buf: &[u8], radix: u32) -> Option<BigInt>` — [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint).

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, ToBigInt};

  

  assert_eq!(BigInt::parse_bytes(b"1234", 10), ToBigInt::to_bigint(&1234));

  assert_eq!(BigInt::parse_bytes(b"ABCD", 16), ToBigInt::to_bigint(&0xABCD));

  assert_eq!(BigInt::parse_bytes(b"G", 16), None);

  ```

- <span id="bigint-from-radix-be"></span>`fn from_radix_be(sign: Sign, buf: &[u8], radix: u32) -> Option<BigInt>` — [`Sign`](#sign), [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint). Each `u8` of the input slice is

  interpreted as one digit of the number

  and must therefore be less than `radix`.

  

  The bytes are in big-endian byte order.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  let inbase190 = vec![15, 33, 125, 12, 14];

  let a = BigInt::from_radix_be(Sign::Minus, &inbase190, 190).unwrap();

  assert_eq!(a.to_radix_be(190), (Sign:: Minus, inbase190));

  ```

- <span id="bigint-from-radix-le"></span>`fn from_radix_le(sign: Sign, buf: &[u8], radix: u32) -> Option<BigInt>` — [`Sign`](#sign), [`BigInt`](#bigint)

  Creates and initializes a [`BigInt`](#bigint). Each `u8` of the input slice is

  interpreted as one digit of the number

  and must therefore be less than `radix`.

  

  The bytes are in little-endian byte order.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  let inbase190 = vec![14, 12, 125, 33, 15];

  let a = BigInt::from_radix_be(Sign::Minus, &inbase190, 190).unwrap();

  assert_eq!(a.to_radix_be(190), (Sign::Minus, inbase190));

  ```

- <span id="bigint-to-bytes-be"></span>`fn to_bytes_be(&self) -> (Sign, Vec<u8>)` — [`Sign`](#sign)

  Returns the sign and the byte representation of the [`BigInt`](#bigint) in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::{ToBigInt, Sign};

  

  let i = -1125.to_bigint().unwrap();

  assert_eq!(i.to_bytes_be(), (Sign::Minus, vec![4, 101]));

  ```

- <span id="bigint-to-bytes-le"></span>`fn to_bytes_le(&self) -> (Sign, Vec<u8>)` — [`Sign`](#sign)

  Returns the sign and the byte representation of the [`BigInt`](#bigint) in little-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::{ToBigInt, Sign};

  

  let i = -1125.to_bigint().unwrap();

  assert_eq!(i.to_bytes_le(), (Sign::Minus, vec![101, 4]));

  ```

- <span id="bigint-to-u32-digits"></span>`fn to_u32_digits(&self) -> (Sign, Vec<u32>)` — [`Sign`](#sign)

  Returns the sign and the `u32` digits representation of the [`BigInt`](#bigint) ordered least

  significant digit first.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  assert_eq!(BigInt::from(-1125).to_u32_digits(), (Sign::Minus, vec![1125]));

  assert_eq!(BigInt::from(4294967295u32).to_u32_digits(), (Sign::Plus, vec![4294967295]));

  assert_eq!(BigInt::from(4294967296u64).to_u32_digits(), (Sign::Plus, vec![0, 1]));

  assert_eq!(BigInt::from(-112500000000i64).to_u32_digits(), (Sign::Minus, vec![830850304, 26]));

  assert_eq!(BigInt::from(112500000000i64).to_u32_digits(), (Sign::Plus, vec![830850304, 26]));

  ```

- <span id="bigint-to-u64-digits"></span>`fn to_u64_digits(&self) -> (Sign, Vec<u64>)` — [`Sign`](#sign)

  Returns the sign and the `u64` digits representation of the [`BigInt`](#bigint) ordered least

  significant digit first.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  assert_eq!(BigInt::from(-1125).to_u64_digits(), (Sign::Minus, vec![1125]));

  assert_eq!(BigInt::from(4294967295u32).to_u64_digits(), (Sign::Plus, vec![4294967295]));

  assert_eq!(BigInt::from(4294967296u64).to_u64_digits(), (Sign::Plus, vec![4294967296]));

  assert_eq!(BigInt::from(-112500000000i64).to_u64_digits(), (Sign::Minus, vec![112500000000]));

  assert_eq!(BigInt::from(112500000000i64).to_u64_digits(), (Sign::Plus, vec![112500000000]));

  assert_eq!(BigInt::from(1u128 << 64).to_u64_digits(), (Sign::Plus, vec![0, 1]));

  ```

- <span id="bigint-iter-u32-digits"></span>`fn iter_u32_digits(&self) -> U32Digits<'_>` — [`U32Digits`](../biguint/iter/index.md#u32digits)

  Returns an iterator of `u32` digits representation of the [`BigInt`](#bigint) ordered least

  significant digit first.

  

  # Examples

  

  ```rust

  use num_bigint::BigInt;

  

  assert_eq!(BigInt::from(-1125).iter_u32_digits().collect::<Vec<u32>>(), vec![1125]);

  assert_eq!(BigInt::from(4294967295u32).iter_u32_digits().collect::<Vec<u32>>(), vec![4294967295]);

  assert_eq!(BigInt::from(4294967296u64).iter_u32_digits().collect::<Vec<u32>>(), vec![0, 1]);

  assert_eq!(BigInt::from(-112500000000i64).iter_u32_digits().collect::<Vec<u32>>(), vec![830850304, 26]);

  assert_eq!(BigInt::from(112500000000i64).iter_u32_digits().collect::<Vec<u32>>(), vec![830850304, 26]);

  ```

- <span id="bigint-iter-u64-digits"></span>`fn iter_u64_digits(&self) -> U64Digits<'_>` — [`U64Digits`](../biguint/iter/index.md#u64digits)

  Returns an iterator of `u64` digits representation of the [`BigInt`](#bigint) ordered least

  significant digit first.

  

  # Examples

  

  ```rust

  use num_bigint::BigInt;

  

  assert_eq!(BigInt::from(-1125).iter_u64_digits().collect::<Vec<u64>>(), vec![1125u64]);

  assert_eq!(BigInt::from(4294967295u32).iter_u64_digits().collect::<Vec<u64>>(), vec![4294967295u64]);

  assert_eq!(BigInt::from(4294967296u64).iter_u64_digits().collect::<Vec<u64>>(), vec![4294967296u64]);

  assert_eq!(BigInt::from(-112500000000i64).iter_u64_digits().collect::<Vec<u64>>(), vec![112500000000u64]);

  assert_eq!(BigInt::from(112500000000i64).iter_u64_digits().collect::<Vec<u64>>(), vec![112500000000u64]);

  assert_eq!(BigInt::from(1u128 << 64).iter_u64_digits().collect::<Vec<u64>>(), vec![0, 1]);

  ```

- <span id="bigint-to-signed-bytes-be"></span>`fn to_signed_bytes_be(&self) -> Vec<u8>`

  Returns the two's-complement byte representation of the [`BigInt`](#bigint) in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::ToBigInt;

  

  let i = -1125.to_bigint().unwrap();

  assert_eq!(i.to_signed_bytes_be(), vec![251, 155]);

  ```

- <span id="bigint-to-signed-bytes-le"></span>`fn to_signed_bytes_le(&self) -> Vec<u8>`

  Returns the two's-complement byte representation of the [`BigInt`](#bigint) in little-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::ToBigInt;

  

  let i = -1125.to_bigint().unwrap();

  assert_eq!(i.to_signed_bytes_le(), vec![155, 251]);

  ```

- <span id="bigint-to-str-radix"></span>`fn to_str_radix(&self, radix: u32) -> String`

  Returns the integer formatted as a string in the given radix.

  `radix` must be in the range `2...36`.

  

  # Examples

  

  ```rust

  use num_bigint::BigInt;

  

  let i = BigInt::parse_bytes(b"ff", 16).unwrap();

  assert_eq!(i.to_str_radix(16), "ff");

  ```

- <span id="bigint-to-radix-be"></span>`fn to_radix_be(&self, radix: u32) -> (Sign, Vec<u8>)` — [`Sign`](#sign)

  Returns the integer in the requested base in big-endian digit order.

  The output is not given in a human readable alphabet but as a zero

  based `u8` number.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  assert_eq!(BigInt::from(-0xFFFFi64).to_radix_be(159),

             (Sign::Minus, vec![2, 94, 27]));

  // 0xFFFF = 65535 = 2*(159^2) + 94*159 + 27

  ```

- <span id="bigint-to-radix-le"></span>`fn to_radix_le(&self, radix: u32) -> (Sign, Vec<u8>)` — [`Sign`](#sign)

  Returns the integer in the requested base in little-endian digit order.

  The output is not given in a human readable alphabet but as a zero

  based `u8` number.

  `radix` must be in the range `2...256`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  assert_eq!(BigInt::from(-0xFFFFi64).to_radix_le(159),

             (Sign::Minus, vec![27, 94, 2]));

  // 0xFFFF = 65535 = 27 + 94*159 + 2*(159^2)

  ```

- <span id="bigint-sign"></span>`fn sign(&self) -> Sign` — [`Sign`](#sign)

  Returns the sign of the [`BigInt`](#bigint) as a [`Sign`](#sign).

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  assert_eq!(BigInt::from(1234).sign(), Sign::Plus);

  assert_eq!(BigInt::from(-4321).sign(), Sign::Minus);

  assert_eq!(BigInt::ZERO.sign(), Sign::NoSign);

  ```

- <span id="bigint-magnitude"></span>`fn magnitude(&self) -> &BigUint` — [`BigUint`](../biguint/index.md#biguint)

  Returns the magnitude of the [`BigInt`](#bigint) as a [`BigUint`](../biguint/index.md).

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, BigUint};

  use num_traits::Zero;

  

  assert_eq!(BigInt::from(1234).magnitude(), &BigUint::from(1234u32));

  assert_eq!(BigInt::from(-4321).magnitude(), &BigUint::from(4321u32));

  assert!(BigInt::ZERO.magnitude().is_zero());

  ```

- <span id="bigint-into-parts"></span>`fn into_parts(self) -> (Sign, BigUint)` — [`Sign`](#sign), [`BigUint`](../biguint/index.md#biguint)

  Convert this [`BigInt`](#bigint) into its [`Sign`](#sign) and [`BigUint`](../biguint/index.md) magnitude,

  the reverse of `BigInt::from_biguint()`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, BigUint, Sign};

  

  assert_eq!(BigInt::from(1234).into_parts(), (Sign::Plus, BigUint::from(1234u32)));

  assert_eq!(BigInt::from(-4321).into_parts(), (Sign::Minus, BigUint::from(4321u32)));

  assert_eq!(BigInt::ZERO.into_parts(), (Sign::NoSign, BigUint::ZERO));

  ```

- <span id="bigint-bits"></span>`fn bits(&self) -> u64`

  Determines the fewest bits necessary to express the [`BigInt`](#bigint),

  not including the sign.

- <span id="bigint-to-biguint"></span>`fn to_biguint(&self) -> Option<BigUint>` — [`BigUint`](../biguint/index.md#biguint)

  Converts this [`BigInt`](#bigint) into a [`BigUint`](../biguint/index.md), if it's not negative.

- <span id="bigint-checked-add"></span>`fn checked_add(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="bigint-checked-sub"></span>`fn checked_sub(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="bigint-checked-mul"></span>`fn checked_mul(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="bigint-checked-div"></span>`fn checked_div(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="bigint-pow"></span>`fn pow(&self, exponent: u32) -> Self`

  Returns `self ^ exponent`.

- <span id="bigint-modpow"></span>`fn modpow(&self, exponent: &Self, modulus: &Self) -> Self`

  Returns `(self ^ exponent) mod modulus`

  

  Note that this rounds like `mod_floor`, not like the `%` operator,

  which makes a difference when given a negative `self` or `modulus`.

  The result will be in the interval `[0, modulus)` for `modulus > 0`,

  or in the interval `(modulus, 0]` for `modulus < 0`

  

  Panics if the exponent is negative or the modulus is zero.

- <span id="bigint-modinv"></span>`fn modinv(&self, modulus: &Self) -> Option<Self>`

  Returns the modular multiplicative inverse if it exists, otherwise `None`.

  

  This solves for `x` such that `self * x ≡ 1 (mod modulus)`.

  Note that this rounds like `mod_floor`, not like the `%` operator,

  which makes a difference when given a negative `self` or `modulus`.

  The solution will be in the interval `[0, modulus)` for `modulus > 0`,

  or in the interval `(modulus, 0]` for `modulus < 0`,

  and it exists if and only if `gcd(self, modulus) == 1`.

  

  ```rust

  use num_bigint::BigInt;

  use num_integer::Integer;

  use num_traits::{One, Zero};

  

  let m = BigInt::from(383);

  

  // Trivial cases

  assert_eq!(BigInt::zero().modinv(&m), None);

  assert_eq!(BigInt::one().modinv(&m), Some(BigInt::one()));

  let neg1 = &m - 1u32;

  assert_eq!(neg1.modinv(&m), Some(neg1));

  

  // Positive self and modulus

  let a = BigInt::from(271);

  let x = a.modinv(&m).unwrap();

  assert_eq!(x, BigInt::from(106));

  assert_eq!(x.modinv(&m).unwrap(), a);

  assert_eq!((&a * x).mod_floor(&m), BigInt::one());

  

  // Negative self and positive modulus

  let b = -&a;

  let x = b.modinv(&m).unwrap();

  assert_eq!(x, BigInt::from(277));

  assert_eq!((&b * x).mod_floor(&m), BigInt::one());

  

  // Positive self and negative modulus

  let n = -&m;

  let x = a.modinv(&n).unwrap();

  assert_eq!(x, BigInt::from(-277));

  assert_eq!((&a * x).mod_floor(&n), &n + 1);

  

  // Negative self and modulus

  let x = b.modinv(&n).unwrap();

  assert_eq!(x, BigInt::from(-106));

  assert_eq!((&b * x).mod_floor(&n), &n + 1);

  ```

- <span id="bigint-sqrt"></span>`fn sqrt(&self) -> Self`

  Returns the truncated principal square root of `self` --

  see `num_integer::Roots::sqrt()`.

- <span id="bigint-cbrt"></span>`fn cbrt(&self) -> Self`

  Returns the truncated principal cube root of `self` --

  see `num_integer::Roots::cbrt()`.

- <span id="bigint-nth-root"></span>`fn nth_root(&self, n: u32) -> Self`

  Returns the truncated principal `n`th root of `self` --

  See `num_integer::Roots::nth_root()`.

- <span id="bigint-trailing-zeros"></span>`fn trailing_zeros(&self) -> Option<u64>`

  Returns the number of least-significant bits that are zero,

  or `None` if the entire number is zero.

- <span id="bigint-bit"></span>`fn bit(&self, bit: u64) -> bool`

  Returns whether the bit in position `bit` is set,

  using the two's complement for negative numbers

- <span id="bigint-set-bit"></span>`fn set_bit(&mut self, bit: u64, value: bool)`

  Sets or clears the bit in the given position,

  using the two's complement for negative numbers

  

  Note that setting/clearing a bit (for positive/negative numbers,

  respectively) greater than the current bit length, a reallocation

  may be needed to store the new digits

#### Trait Implementations

##### `impl Add for &super::BigInt`

- <span id="superbigint-add-type-output"></span>`type Output = BigInt`

- <span id="superbigint-add"></span>`fn add(self, other: &BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl AddAssign for super::BigInt`

- <span id="superbigint-addassign-add-assign"></span>`fn add_assign(&mut self, other: &BigInt)` — [`BigInt`](#bigint)

##### `impl Average for BigInt`

- <span id="bigint-average-average-floor"></span>`fn average_floor(&self, other: &I) -> I`

  Returns the floor value of the average of `self` and `other`.

- <span id="bigint-average-average-ceil"></span>`fn average_ceil(&self, other: &I) -> I`

  Returns the ceil value of the average of `self` and `other`.

##### `impl Binary for BigInt`

- <span id="bigint-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl BitAnd for super::BigInt`

- <span id="superbigint-bitand-type-output"></span>`type Output = BigInt`

- <span id="superbigint-bitand"></span>`fn bitand(self, other: BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl BitAndAssign for super::BigInt`

- <span id="superbigint-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: BigInt)` — [`BigInt`](#bigint)

##### `impl BitOr for super::BigInt`

- <span id="superbigint-bitor-type-output"></span>`type Output = BigInt`

- <span id="superbigint-bitor"></span>`fn bitor(self, other: BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl BitOrAssign for super::BigInt`

- <span id="superbigint-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: BigInt)` — [`BigInt`](#bigint)

##### `impl BitXor for super::BigInt`

- <span id="superbigint-bitxor-type-output"></span>`type Output = BigInt`

- <span id="superbigint-bitxor"></span>`fn bitxor(self, other: BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl BitXorAssign for super::BigInt`

- <span id="superbigint-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: BigInt)` — [`BigInt`](#bigint)

##### `impl Bits for num_bigint::BigInt`

- <span id="num-bigintbigint-bits-type-output"></span>`type Output = BigInt`

##### `impl CheckedAdd for super::BigInt`

- <span id="superbigint-checkedadd-checked-add"></span>`fn checked_add(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

##### `impl CheckedDiv for super::BigInt`

- <span id="superbigint-checkeddiv-checked-div"></span>`fn checked_div(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

##### `impl CheckedEuclid for super::BigInt`

- <span id="superbigint-checkedeuclid-checked-div-euclid"></span>`fn checked_div_euclid(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="superbigint-checkedeuclid-checked-rem-euclid"></span>`fn checked_rem_euclid(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="superbigint-checkedeuclid-checked-div-rem-euclid"></span>`fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)>`

##### `impl CheckedMul for super::BigInt`

- <span id="superbigint-checkedmul-checked-mul"></span>`fn checked_mul(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

##### `impl CheckedSub for super::BigInt`

- <span id="superbigint-checkedsub-checked-sub"></span>`fn checked_sub(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](#bigint)

##### `impl Clone for BigInt`

- <span id="bigint-clone"></span>`fn clone(&self) -> Self`

- <span id="bigint-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl ConstZero for BigInt`

- <span id="bigint-constzero-const-zero"></span>`const ZERO: Self`

##### `impl Debug for BigInt`

- <span id="bigint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigInt`

- <span id="bigint-default"></span>`fn default() -> BigInt` — [`BigInt`](#bigint)

##### `impl Display for BigInt`

- <span id="bigint-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Div for super::BigInt`

- <span id="superbigint-div-type-output"></span>`type Output = BigInt`

- <span id="superbigint-div"></span>`fn div(self, other: BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl DivAssign for super::BigInt`

- <span id="superbigint-divassign-div-assign"></span>`fn div_assign(&mut self, other: &BigInt)` — [`BigInt`](#bigint)

##### `impl Eq for BigInt`

##### `impl Euclid for super::BigInt`

- <span id="superbigint-euclid-div-euclid"></span>`fn div_euclid(&self, v: &BigInt) -> BigInt` — [`BigInt`](#bigint)

- <span id="superbigint-euclid-rem-euclid"></span>`fn rem_euclid(&self, v: &BigInt) -> BigInt` — [`BigInt`](#bigint)

- <span id="superbigint-euclid-div-rem-euclid"></span>`fn div_rem_euclid(&self, v: &Self) -> (Self, Self)`

##### `impl FromBytes for BigInt`

- <span id="bigint-frombytes-type-bytes"></span>`type Bytes = [u8]`

- <span id="bigint-frombytes-from-be-bytes"></span>`fn from_be_bytes(bytes: &<Self as >::Bytes) -> Self`

- <span id="bigint-frombytes-from-le-bytes"></span>`fn from_le_bytes(bytes: &<Self as >::Bytes) -> Self`

##### `impl FromPrimitive for super::BigInt`

- <span id="superbigint-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="superbigint-fromprimitive-from-i128"></span>`fn from_i128(n: i128) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="superbigint-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="superbigint-fromprimitive-from-u128"></span>`fn from_u128(n: u128) -> Option<BigInt>` — [`BigInt`](#bigint)

- <span id="superbigint-fromprimitive-from-f64"></span>`fn from_f64(n: f64) -> Option<BigInt>` — [`BigInt`](#bigint)

##### `impl FromStr for super::BigInt`

- <span id="superbigint-fromstr-type-err"></span>`type Err = ParseBigIntError`

- <span id="superbigint-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<BigInt, ParseBigIntError>` — [`BigInt`](#bigint), [`ParseBigIntError`](../index.md#parsebiginterror)

##### `impl Hash for BigInt`

- <span id="bigint-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl IntDigits for BigInt`

- <span id="bigint-intdigits-digits"></span>`fn digits(&self) -> &[u64]`

- <span id="bigint-intdigits-digits-mut"></span>`fn digits_mut(&mut self) -> &mut Vec<u64>`

- <span id="bigint-intdigits-normalize"></span>`fn normalize(&mut self)`

- <span id="bigint-intdigits-capacity"></span>`fn capacity(&self) -> usize`

- <span id="bigint-intdigits-len"></span>`fn len(&self) -> usize`

##### `impl Integer for BigInt`

- <span id="bigint-integer-div-rem"></span>`fn div_rem(&self, other: &BigInt) -> (BigInt, BigInt)` — [`BigInt`](#bigint)

- <span id="bigint-integer-div-floor"></span>`fn div_floor(&self, other: &BigInt) -> BigInt` — [`BigInt`](#bigint)

- <span id="bigint-integer-mod-floor"></span>`fn mod_floor(&self, other: &BigInt) -> BigInt` — [`BigInt`](#bigint)

- <span id="bigint-integer-div-mod-floor"></span>`fn div_mod_floor(&self, other: &BigInt) -> (BigInt, BigInt)` — [`BigInt`](#bigint)

- <span id="bigint-integer-div-ceil"></span>`fn div_ceil(&self, other: &Self) -> Self`

- <span id="bigint-integer-gcd"></span>`fn gcd(&self, other: &BigInt) -> BigInt` — [`BigInt`](#bigint)

  Calculates the Greatest Common Divisor (GCD) of the number and `other`.

  

  The result is always positive.

- <span id="bigint-integer-lcm"></span>`fn lcm(&self, other: &BigInt) -> BigInt` — [`BigInt`](#bigint)

  Calculates the Lowest Common Multiple (LCM) of the number and `other`.

- <span id="bigint-integer-gcd-lcm"></span>`fn gcd_lcm(&self, other: &BigInt) -> (BigInt, BigInt)` — [`BigInt`](#bigint)

  Calculates the Greatest Common Divisor (GCD) and

  Lowest Common Multiple (LCM) together.

- <span id="bigint-integer-extended-gcd-lcm"></span>`fn extended_gcd_lcm(&self, other: &BigInt) -> (num_integer::ExtendedGcd<BigInt>, BigInt)` — [`BigInt`](#bigint)

  Greatest common divisor, least common multiple, and Bézout coefficients.

- <span id="bigint-integer-divides"></span>`fn divides(&self, other: &BigInt) -> bool` — [`BigInt`](#bigint)

  Deprecated, use `is_multiple_of` instead.

- <span id="bigint-integer-is-multiple-of"></span>`fn is_multiple_of(&self, other: &BigInt) -> bool` — [`BigInt`](#bigint)

  Returns `true` if the number is a multiple of `other`.

- <span id="bigint-integer-is-even"></span>`fn is_even(&self) -> bool`

  Returns `true` if the number is divisible by `2`.

- <span id="bigint-integer-is-odd"></span>`fn is_odd(&self) -> bool`

  Returns `true` if the number is not divisible by `2`.

- <span id="bigint-integer-next-multiple-of"></span>`fn next_multiple_of(&self, other: &Self) -> Self`

  Rounds up to nearest multiple of argument.

- <span id="bigint-integer-prev-multiple-of"></span>`fn prev_multiple_of(&self, other: &Self) -> Self`

  Rounds down to nearest multiple of argument.

- <span id="bigint-integer-dec"></span>`fn dec(&mut self)`

- <span id="bigint-integer-inc"></span>`fn inc(&mut self)`

##### `impl LowerHex for BigInt`

- <span id="bigint-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Mul for super::BigInt`

- <span id="superbigint-mul-type-output"></span>`type Output = BigInt`

- <span id="superbigint-mul"></span>`fn mul(self, other: BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl MulAssign for BigInt`

- <span id="bigint-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: BigInt)` — [`BigInt`](#bigint)

##### `impl Neg for BigInt`

- <span id="bigint-neg-type-output"></span>`type Output = BigInt`

- <span id="bigint-neg"></span>`fn neg(self) -> BigInt` — [`BigInt`](#bigint)

##### `impl Not for BigInt`

- <span id="bigint-not-type-output"></span>`type Output = BigInt`

- <span id="bigint-not"></span>`fn not(self) -> BigInt` — [`BigInt`](#bigint)

##### `impl Num for super::BigInt`

- <span id="superbigint-num-type-fromstrradixerr"></span>`type FromStrRadixErr = ParseBigIntError`

- <span id="superbigint-num-from-str-radix"></span>`fn from_str_radix(s: &str, radix: u32) -> Result<BigInt, ParseBigIntError>` — [`BigInt`](#bigint), [`ParseBigIntError`](../index.md#parsebiginterror)

  Creates and initializes a [`BigInt`](#bigint).

##### `impl NumAssign for BigInt`

##### `impl<Rhs> NumAssignOps for BigInt`

##### `impl NumAssignRef for BigInt`

##### `impl<Rhs, Output> NumOps for BigInt`

##### `impl NumRef for BigInt`

##### `impl Octal for BigInt`

- <span id="bigint-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl One for BigInt`

- <span id="bigint-one"></span>`fn one() -> BigInt` — [`BigInt`](#bigint)

- <span id="bigint-one-set-one"></span>`fn set_one(&mut self)`

- <span id="bigint-one-is-one"></span>`fn is_one(&self) -> bool`

##### `impl Ord for BigInt`

- <span id="bigint-ord-cmp"></span>`fn cmp(&self, other: &BigInt) -> Ordering` — [`BigInt`](#bigint)

##### `impl PartialEq for BigInt`

- <span id="bigint-partialeq-eq"></span>`fn eq(&self, other: &BigInt) -> bool` — [`BigInt`](#bigint)

##### `impl PartialOrd for BigInt`

- <span id="bigint-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BigInt) -> Option<Ordering>` — [`BigInt`](#bigint)

##### `impl Pow for BigInt`

- <span id="bigint-pow-type-output"></span>`type Output = BigInt`

- <span id="bigint-pow"></span>`fn pow(self, rhs: u8) -> BigInt` — [`BigInt`](#bigint)

##### `impl<T> Product for super::BigInt`

- <span id="superbigint-product"></span>`fn product<I>(iter: I) -> Self`

##### `impl<Base> RefNum for BigInt`

##### `impl Rem for super::BigInt`

- <span id="superbigint-rem-type-output"></span>`type Output = BigInt`

- <span id="superbigint-rem"></span>`fn rem(self, other: BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl RemAssign for super::BigInt`

- <span id="superbigint-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: &BigInt)` — [`BigInt`](#bigint)

##### `impl Roots for BigInt`

- <span id="bigint-roots-nth-root"></span>`fn nth_root(&self, n: u32) -> Self`

- <span id="bigint-roots-sqrt"></span>`fn sqrt(&self) -> Self`

- <span id="bigint-roots-cbrt"></span>`fn cbrt(&self) -> Self`

##### `impl Shl for BigInt`

- <span id="bigint-shl-type-output"></span>`type Output = BigInt`

- <span id="bigint-shl"></span>`fn shl(self, rhs: u8) -> BigInt` — [`BigInt`](#bigint)

##### `impl ShlAssign for BigInt`

- <span id="bigint-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: u8)`

##### `impl Shr for BigInt`

- <span id="bigint-shr-type-output"></span>`type Output = BigInt`

- <span id="bigint-shr"></span>`fn shr(self, rhs: u8) -> BigInt` — [`BigInt`](#bigint)

##### `impl ShrAssign for BigInt`

- <span id="bigint-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: u8)`

##### `impl Signed for BigInt`

- <span id="bigint-signed-abs"></span>`fn abs(&self) -> BigInt` — [`BigInt`](#bigint)

- <span id="bigint-signed-abs-sub"></span>`fn abs_sub(&self, other: &BigInt) -> BigInt` — [`BigInt`](#bigint)

- <span id="bigint-signed-signum"></span>`fn signum(&self) -> BigInt` — [`BigInt`](#bigint)

- <span id="bigint-signed-is-positive"></span>`fn is_positive(&self) -> bool`

- <span id="bigint-signed-is-negative"></span>`fn is_negative(&self) -> bool`

##### `impl Sub for &super::BigInt`

- <span id="superbigint-sub-type-output"></span>`type Output = BigInt`

- <span id="superbigint-sub"></span>`fn sub(self, other: &BigInt) -> BigInt` — [`BigInt`](#bigint)

##### `impl SubAssign for super::BigInt`

- <span id="superbigint-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: &BigInt)` — [`BigInt`](#bigint)

##### `impl<T> Sum for super::BigInt`

- <span id="superbigint-sum"></span>`fn sum<I>(iter: I) -> Self`

##### `impl ToBigInt for super::BigInt`

- <span id="superbigint-tobigint-to-bigint"></span>`fn to_bigint(&self) -> Option<BigInt>` — [`BigInt`](#bigint)

##### `impl ToBigUint for super::BigInt`

- <span id="superbigint-tobiguint-to-biguint"></span>`fn to_biguint(&self) -> Option<BigUint>` — [`BigUint`](../biguint/index.md#biguint)

##### `impl ToBytes for BigInt`

- <span id="bigint-tobytes-type-bytes"></span>`type Bytes = Vec<u8>`

- <span id="bigint-tobytes-to-be-bytes"></span>`fn to_be_bytes(&self) -> <Self as >::Bytes`

- <span id="bigint-tobytes-to-le-bytes"></span>`fn to_le_bytes(&self) -> <Self as >::Bytes`

##### `impl ToPrimitive for super::BigInt`

- <span id="superbigint-toprimitive-to-i64"></span>`fn to_i64(&self) -> Option<i64>`

- <span id="superbigint-toprimitive-to-i128"></span>`fn to_i128(&self) -> Option<i128>`

- <span id="superbigint-toprimitive-to-u64"></span>`fn to_u64(&self) -> Option<u64>`

- <span id="superbigint-toprimitive-to-u128"></span>`fn to_u128(&self) -> Option<u128>`

- <span id="superbigint-toprimitive-to-f32"></span>`fn to_f32(&self) -> Option<f32>`

- <span id="superbigint-toprimitive-to-f64"></span>`fn to_f64(&self) -> Option<f64>`

##### `impl ToString for BigInt`

- <span id="bigint-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl UpperHex for BigInt`

- <span id="bigint-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Zero for BigInt`

- <span id="bigint-zero"></span>`fn zero() -> BigInt` — [`BigInt`](#bigint)

- <span id="bigint-zero-set-zero"></span>`fn set_zero(&mut self)`

- <span id="bigint-zero-is-zero"></span>`fn is_zero(&self) -> bool`

## Enums

### `Sign`

```rust
enum Sign {
    Minus,
    NoSign,
    Plus,
}
```

A `Sign` is a [`BigInt`](#bigint)'s composing element.

#### Trait Implementations

##### `impl Clone for Sign`

- <span id="sign-clone"></span>`fn clone(&self) -> Sign` — [`Sign`](#sign)

##### `impl Copy for Sign`

##### `impl Debug for Sign`

- <span id="sign-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Sign`

##### `impl Hash for Sign`

- <span id="sign-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Mul for super::Sign`

- <span id="supersign-mul-type-output"></span>`type Output = Sign`

- <span id="supersign-mul"></span>`fn mul(self, other: Sign) -> Sign` — [`Sign`](#sign)

##### `impl Neg for Sign`

- <span id="sign-neg-type-output"></span>`type Output = Sign`

- <span id="sign-neg"></span>`fn neg(self) -> Sign` — [`Sign`](#sign)

  Negate `Sign` value.

##### `impl Ord for Sign`

- <span id="sign-ord-cmp"></span>`fn cmp(&self, other: &Sign) -> cmp::Ordering` — [`Sign`](#sign)

##### `impl PartialEq for Sign`

- <span id="sign-partialeq-eq"></span>`fn eq(&self, other: &Sign) -> bool` — [`Sign`](#sign)

##### `impl PartialOrd for Sign`

- <span id="sign-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Sign) -> option::Option<cmp::Ordering>` — [`Sign`](#sign)

##### `impl StructuralPartialEq for Sign`

### `CheckedUnsignedAbs<T>`

```rust
enum CheckedUnsignedAbs<T> {
    Positive(T),
    Negative(T),
}
```

## Traits

### `UnsignedAbs`

```rust
trait UnsignedAbs { ... }
```

#### Associated Types

- `type Unsigned`

#### Required Methods

- `fn checked_uabs(self) -> CheckedUnsignedAbs<<Self as >::Unsigned>`

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`

### `ToBigInt`

```rust
trait ToBigInt { ... }
```

A generic trait for converting a value to a [`BigInt`](#bigint). This may return
`None` when converting from `f32` or `f64`, and will always succeed
when converting from any integer or unsigned primitive, or [`BigUint`](../biguint/index.md).

#### Required Methods

- `fn to_bigint(&self) -> Option<BigInt>`

  Converts the value of `self` to a [`BigInt`](#bigint).

#### Implementors

- [`BigInt`](#bigint)
- [`BigUint`](../biguint/index.md#biguint)
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

## Macros

### `impl_unsigned_abs!`

