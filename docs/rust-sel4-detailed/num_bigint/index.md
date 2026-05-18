# Crate `num_bigint`

Big Integer Types for Rust

* A [`BigUint`](biguint/index.md) is unsigned and represented as a vector of digits.
* A [`BigInt`](bigint/index.md) is signed and is a combination of [`BigUint`](biguint/index.md) and [`Sign`](bigint/index.md).

Common numerical operations are overloaded, so we can treat them
the same way we treat other numbers.

## Example

```rust
fn main() {
use num_bigint::BigUint;
use num_traits::One;

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0 = BigUint::ZERO;
    let mut f1 = BigUint::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

// This is a very large number.
println!("fib(1000) = {}", fib(1000));
}
```

It's easy to generate large random numbers:

```rust,ignore
use num_bigint::{ToBigInt, RandBigInt};

let mut rng = rand::thread_rng();
let a = rng.gen_bigint(1000);

let low = -10000.to_bigint().unwrap();
let high = 10000.to_bigint().unwrap();
let b = rng.gen_bigint_range(&low, &high);

// Probably an even larger number.
println!("{}", a * b);
```

See the "Features" section for instructions for enabling random number generation.

## Features

The `std` crate feature is enabled by default, which enables [`std::error::Error`](../sel4/error/index.md)
implementations and some internal use of floating point approximations. This can be disabled by
depending on `num-bigint` with `default-features = false`. Either way, the `alloc` crate is
always required for heap allocation of the `BigInt`/`BigUint` digits.

### Random Generation

`num-bigint` supports the generation of random big integers when the `rand`
feature is enabled. To enable it include rand as

```toml
rand = "0.8"
num-bigint = { version = "0.4", features = ["rand"] }
```

Note that you must use the version of `rand` that `num-bigint` is compatible
with: `0.8`.

### Arbitrary Big Integers

`num-bigint` supports `arbitrary` and `quickcheck` features to implement
`arbitrary::Arbitrary` and `quickcheck::Arbitrary`, respectively, for both `BigInt` and
`BigUint`. These are useful for fuzzing and other forms of randomized testing.

### Serialization

The `serde` feature adds implementations of `Serialize` and
`Deserialize` for both `BigInt` and `BigUint`. Their serialized data is
generated portably, regardless of platform differences like the internal digit size.


## Compatibility

The `num-bigint` crate is tested for rustc 1.60 and greater.

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`bigint`](#bigint)
  - [`biguint`](#biguint)
  - [`big_digit`](#big-digit)
- [Structs](#structs)
  - [`ParseBigIntError`](#parsebiginterror)
  - [`TryFromBigIntError`](#tryfrombiginterror)
  - [`BigUint`](#biguint)
  - [`U32Digits`](#u32digits)
  - [`U64Digits`](#u64digits)
  - [`BigInt`](#bigint)
- [Enums](#enums)
  - [`BigIntErrorKind`](#biginterrorkind)
  - [`Sign`](#sign)
- [Traits](#traits)
  - [`ToBigUint`](#tobiguint)
  - [`ToBigInt`](#tobigint)
- [Type Aliases](#type-aliases)
  - [`UsizePromotion`](#usizepromotion)
  - [`IsizePromotion`](#isizepromotion)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`bigint`](#bigint) | mod |  |
| [`biguint`](#biguint) | mod |  |
| [`big_digit`](#big-digit) | mod |  |
| [`ParseBigIntError`](#parsebiginterror) | struct |  |
| [`TryFromBigIntError`](#tryfrombiginterror) | struct | The error type returned when a checked conversion regarding big integer fails. |
| [`BigUint`](#biguint) | struct |  |
| [`U32Digits`](#u32digits) | struct |  |
| [`U64Digits`](#u64digits) | struct |  |
| [`BigInt`](#bigint) | struct |  |
| [`BigIntErrorKind`](#biginterrorkind) | enum |  |
| [`Sign`](#sign) | enum |  |
| [`ToBigUint`](#tobiguint) | trait |  |
| [`ToBigInt`](#tobigint) | trait |  |
| [`UsizePromotion`](#usizepromotion) | type |  |
| [`IsizePromotion`](#isizepromotion) | type |  |

## Modules

- [`macros`](macros/index.md)
- [`bigint`](bigint/index.md)
- [`biguint`](biguint/index.md)
- [`big_digit`](big_digit/index.md)

## Structs

### `ParseBigIntError`

```rust
struct ParseBigIntError {
    kind: BigIntErrorKind,
}
```

#### Implementations

- <span id="parsebiginterror-description"></span>`fn __description(&self) -> &str`

- <span id="parsebiginterror-empty"></span>`fn empty() -> Self`

- <span id="parsebiginterror-invalid"></span>`fn invalid() -> Self`

#### Trait Implementations

##### `impl Clone for ParseBigIntError`

- <span id="parsebiginterror-clone"></span>`fn clone(&self) -> ParseBigIntError` — [`ParseBigIntError`](#parsebiginterror)

##### `impl Debug for ParseBigIntError`

- <span id="parsebiginterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseBigIntError`

- <span id="parsebiginterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseBigIntError`

##### `impl Error for ParseBigIntError`

- <span id="parsebiginterror-error-description"></span>`fn description(&self) -> &str`

##### `impl PartialEq for ParseBigIntError`

- <span id="parsebiginterror-partialeq-eq"></span>`fn eq(&self, other: &ParseBigIntError) -> bool` — [`ParseBigIntError`](#parsebiginterror)

##### `impl StructuralPartialEq for ParseBigIntError`

##### `impl ToString for ParseBigIntError`

- <span id="parsebiginterror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `TryFromBigIntError<T>`

```rust
struct TryFromBigIntError<T> {
    original: T,
}
```

The error type returned when a checked conversion regarding big integer fails.

#### Implementations

- <span id="tryfrombiginterror-new"></span>`fn new(original: T) -> Self`

- <span id="tryfrombiginterror-description"></span>`fn __description(&self) -> &str`

- <span id="tryfrombiginterror-into-original"></span>`fn into_original(self) -> T`

  Extract the original value, if available. The value will be available

  if the type before conversion was either [`BigInt`](bigint/index.md) or [`BigUint`](biguint/index.md).

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for TryFromBigIntError<T>`

- <span id="tryfrombiginterror-clone"></span>`fn clone(&self) -> TryFromBigIntError<T>` — [`TryFromBigIntError`](#tryfrombiginterror)

##### `impl<T: marker::Copy> Copy for TryFromBigIntError<T>`

##### `impl<T: fmt::Debug> Debug for TryFromBigIntError<T>`

- <span id="tryfrombiginterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Display for TryFromBigIntError<T>`

- <span id="tryfrombiginterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for TryFromBigIntError<T>`

##### `impl<T> Error for TryFromBigIntError<T>`

- <span id="tryfrombiginterror-error-description"></span>`fn description(&self) -> &str`

##### `impl<T: cmp::PartialEq> PartialEq for TryFromBigIntError<T>`

- <span id="tryfrombiginterror-partialeq-eq"></span>`fn eq(&self, other: &TryFromBigIntError<T>) -> bool` — [`TryFromBigIntError`](#tryfrombiginterror)

##### `impl<T> StructuralPartialEq for TryFromBigIntError<T>`

##### `impl<T> ToString for TryFromBigIntError<T>`

- <span id="tryfrombiginterror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `BigUint`

```rust
struct BigUint {
    data: alloc::vec::Vec<u64>,
}
```

A big unsigned integer type.

#### Implementations

- <span id="biguint-const-zero"></span>`const ZERO: Self`

- <span id="biguint-new"></span>`fn new(digits: Vec<u32>) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

  Creates and initializes a [`BigUint`](biguint/index.md).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="biguint-from-slice"></span>`fn from_slice(slice: &[u32]) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

  Creates and initializes a [`BigUint`](biguint/index.md).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="biguint-assign-from-slice"></span>`fn assign_from_slice(&mut self, slice: &[u32])`

  Assign a value to a [`BigUint`](biguint/index.md).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="biguint-from-bytes-be"></span>`fn from_bytes_be(bytes: &[u8]) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

  Creates and initializes a [`BigUint`](biguint/index.md).

  

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

- <span id="biguint-from-bytes-le"></span>`fn from_bytes_le(bytes: &[u8]) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

  Creates and initializes a [`BigUint`](biguint/index.md).

  

  The bytes are in little-endian byte order.

- <span id="biguint-parse-bytes"></span>`fn parse_bytes(buf: &[u8], radix: u32) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

  Creates and initializes a [`BigUint`](biguint/index.md). The input slice must contain

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

- <span id="biguint-from-radix-be"></span>`fn from_radix_be(buf: &[u8], radix: u32) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

  Creates and initializes a [`BigUint`](biguint/index.md). Each `u8` of the input slice is

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

- <span id="biguint-from-radix-le"></span>`fn from_radix_le(buf: &[u8], radix: u32) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

  Creates and initializes a [`BigUint`](biguint/index.md). Each `u8` of the input slice is

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

  Returns the byte representation of the [`BigUint`](biguint/index.md) in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  let i = BigUint::parse_bytes(b"1125", 10).unwrap();

  assert_eq!(i.to_bytes_be(), vec![4, 101]);

  ```

- <span id="biguint-to-bytes-le"></span>`fn to_bytes_le(&self) -> Vec<u8>`

  Returns the byte representation of the [`BigUint`](biguint/index.md) in little-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  let i = BigUint::parse_bytes(b"1125", 10).unwrap();

  assert_eq!(i.to_bytes_le(), vec![101, 4]);

  ```

- <span id="biguint-to-u32-digits"></span>`fn to_u32_digits(&self) -> Vec<u32>`

  Returns the `u32` digits representation of the [`BigUint`](biguint/index.md) ordered least significant digit

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

  Returns the `u64` digits representation of the [`BigUint`](biguint/index.md) ordered least significant digit

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

- <span id="biguint-iter-u32-digits"></span>`fn iter_u32_digits(&self) -> U32Digits<'_>` — [`U32Digits`](biguint/iter/index.md#u32digits)

  Returns an iterator of `u32` digits representation of the [`BigUint`](biguint/index.md) ordered least

  significant digit first.

  

  # Examples

  

  ```rust

  use num_bigint::BigUint;

  

  assert_eq!(BigUint::from(1125u32).iter_u32_digits().collect::<Vec<u32>>(), vec![1125]);

  assert_eq!(BigUint::from(4294967295u32).iter_u32_digits().collect::<Vec<u32>>(), vec![4294967295]);

  assert_eq!(BigUint::from(4294967296u64).iter_u32_digits().collect::<Vec<u32>>(), vec![0, 1]);

  assert_eq!(BigUint::from(112500000000u64).iter_u32_digits().collect::<Vec<u32>>(), vec![830850304, 26]);

  ```

- <span id="biguint-iter-u64-digits"></span>`fn iter_u64_digits(&self) -> U64Digits<'_>` — [`U64Digits`](biguint/iter/index.md#u64digits)

  Returns an iterator of `u64` digits representation of the [`BigUint`](biguint/index.md) ordered least

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

  Determines the fewest bits necessary to express the [`BigUint`](biguint/index.md).

- <span id="biguint-normalize"></span>`fn normalize(&mut self)`

  Strips off trailing zero bigdigits - comparisons require the last element in the vector to

  be nonzero.

- <span id="biguint-normalized"></span>`fn normalized(self) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

  Returns a normalized [`BigUint`](biguint/index.md).

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

- <span id="superbiguint-add"></span>`fn add(self, other: BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl AddAssign for super::BigUint`

- <span id="superbiguint-addassign-add-assign"></span>`fn add_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl Average for BigUint`

- <span id="biguint-average-average-floor"></span>`fn average_floor(&self, other: &I) -> I`

  Returns the floor value of the average of `self` and `other`.

- <span id="biguint-average-average-ceil"></span>`fn average_ceil(&self, other: &I) -> I`

  Returns the ceil value of the average of `self` and `other`.

##### `impl Binary for BigUint`

- <span id="biguint-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl BitAnd for super::BigUint`

- <span id="superbiguint-bitand-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-bitand"></span>`fn bitand(self, other: BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl BitAndAssign for super::BigUint`

- <span id="superbiguint-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl BitOr for super::BigUint`

- <span id="superbiguint-bitor-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-bitor"></span>`fn bitor(self, other: BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl BitOrAssign for super::BigUint`

- <span id="superbiguint-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl BitXor for super::BigUint`

- <span id="superbiguint-bitxor-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-bitxor"></span>`fn bitxor(self, other: BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl BitXorAssign for super::BigUint`

- <span id="superbiguint-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl CheckedAdd for super::BigUint`

- <span id="superbiguint-checkedadd-checked-add"></span>`fn checked_add(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

##### `impl CheckedDiv for super::BigUint`

- <span id="superbiguint-checkeddiv-checked-div"></span>`fn checked_div(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

##### `impl CheckedEuclid for super::BigUint`

- <span id="superbiguint-checkedeuclid-checked-div-euclid"></span>`fn checked_div_euclid(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-checkedeuclid-checked-rem-euclid"></span>`fn checked_rem_euclid(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-checkedeuclid-checked-div-rem-euclid"></span>`fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)>`

##### `impl CheckedMul for super::BigUint`

- <span id="superbiguint-checkedmul-checked-mul"></span>`fn checked_mul(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

##### `impl CheckedSub for super::BigUint`

- <span id="superbiguint-checkedsub-checked-sub"></span>`fn checked_sub(&self, v: &BigUint) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

##### `impl Clone for BigUint`

- <span id="biguint-clone"></span>`fn clone(&self) -> Self`

- <span id="biguint-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl ConstZero for BigUint`

- <span id="biguint-constzero-const-zero"></span>`const ZERO: Self`

##### `impl Debug for BigUint`

- <span id="biguint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigUint`

- <span id="biguint-default"></span>`fn default() -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl Display for BigUint`

- <span id="biguint-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Div for super::BigUint`

- <span id="superbiguint-div-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-div"></span>`fn div(self, other: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl DivAssign for super::BigUint`

- <span id="superbiguint-divassign-div-assign"></span>`fn div_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl Eq for BigUint`

##### `impl Euclid for super::BigUint`

- <span id="superbiguint-euclid-div-euclid"></span>`fn div_euclid(&self, v: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-euclid-rem-euclid"></span>`fn rem_euclid(&self, v: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-euclid-div-rem-euclid"></span>`fn div_rem_euclid(&self, v: &Self) -> (Self, Self)`

##### `impl FromBytes for BigUint`

- <span id="biguint-frombytes-type-bytes"></span>`type Bytes = [u8]`

- <span id="biguint-frombytes-from-be-bytes"></span>`fn from_be_bytes(bytes: &<Self as >::Bytes) -> Self`

- <span id="biguint-frombytes-from-le-bytes"></span>`fn from_le_bytes(bytes: &<Self as >::Bytes) -> Self`

##### `impl FromPrimitive for super::BigUint`

- <span id="superbiguint-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-fromprimitive-from-i128"></span>`fn from_i128(n: i128) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-fromprimitive-from-u128"></span>`fn from_u128(n: u128) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

- <span id="superbiguint-fromprimitive-from-f64"></span>`fn from_f64(n: f64) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

##### `impl FromStr for super::BigUint`

- <span id="superbiguint-fromstr-type-err"></span>`type Err = ParseBigIntError`

- <span id="superbiguint-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<BigUint, ParseBigIntError>` — [`BigUint`](biguint/index.md#biguint), [`ParseBigIntError`](#parsebiginterror)

##### `impl Hash for BigUint`

- <span id="biguint-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl IntDigits for BigUint`

- <span id="biguint-intdigits-digits"></span>`fn digits(&self) -> &[u64]`

- <span id="biguint-intdigits-digits-mut"></span>`fn digits_mut(&mut self) -> &mut Vec<u64>`

- <span id="biguint-intdigits-normalize"></span>`fn normalize(&mut self)`

- <span id="biguint-intdigits-capacity"></span>`fn capacity(&self) -> usize`

- <span id="biguint-intdigits-len"></span>`fn len(&self) -> usize`

##### `impl Integer for BigUint`

- <span id="biguint-integer-div-rem"></span>`fn div_rem(&self, other: &BigUint) -> (BigUint, BigUint)` — [`BigUint`](biguint/index.md#biguint)

- <span id="biguint-integer-div-floor"></span>`fn div_floor(&self, other: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

- <span id="biguint-integer-mod-floor"></span>`fn mod_floor(&self, other: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

- <span id="biguint-integer-div-mod-floor"></span>`fn div_mod_floor(&self, other: &BigUint) -> (BigUint, BigUint)` — [`BigUint`](biguint/index.md#biguint)

- <span id="biguint-integer-div-ceil"></span>`fn div_ceil(&self, other: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

- <span id="biguint-integer-gcd"></span>`fn gcd(&self, other: &Self) -> Self`

  Calculates the Greatest Common Divisor (GCD) of the number and `other`.

  

  The result is always positive.

- <span id="biguint-integer-lcm"></span>`fn lcm(&self, other: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

  Calculates the Lowest Common Multiple (LCM) of the number and `other`.

- <span id="biguint-integer-gcd-lcm"></span>`fn gcd_lcm(&self, other: &Self) -> (Self, Self)`

  Calculates the Greatest Common Divisor (GCD) and

  Lowest Common Multiple (LCM) together.

- <span id="biguint-integer-divides"></span>`fn divides(&self, other: &BigUint) -> bool` — [`BigUint`](biguint/index.md#biguint)

  Deprecated, use `is_multiple_of` instead.

- <span id="biguint-integer-is-multiple-of"></span>`fn is_multiple_of(&self, other: &BigUint) -> bool` — [`BigUint`](biguint/index.md#biguint)

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

- <span id="superbiguint-mul"></span>`fn mul(self, other: BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl MulAssign for BigUint`

- <span id="biguint-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl Num for super::BigUint`

- <span id="superbiguint-num-type-fromstrradixerr"></span>`type FromStrRadixErr = ParseBigIntError`

- <span id="superbiguint-num-from-str-radix"></span>`fn from_str_radix(s: &str, radix: u32) -> Result<BigUint, ParseBigIntError>` — [`BigUint`](biguint/index.md#biguint), [`ParseBigIntError`](#parsebiginterror)

  Creates and initializes a `BigUint`.

##### `impl NumAssign for BigUint`

##### `impl<Rhs> NumAssignOps for BigUint`

##### `impl NumAssignRef for BigUint`

##### `impl<Rhs, Output> NumOps for BigUint`

##### `impl NumRef for BigUint`

##### `impl Octal for BigUint`

- <span id="biguint-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl One for BigUint`

- <span id="biguint-one"></span>`fn one() -> BigUint` — [`BigUint`](biguint/index.md#biguint)

- <span id="biguint-one-set-one"></span>`fn set_one(&mut self)`

- <span id="biguint-one-is-one"></span>`fn is_one(&self) -> bool`

##### `impl Ord for BigUint`

- <span id="biguint-ord-cmp"></span>`fn cmp(&self, other: &BigUint) -> Ordering` — [`BigUint`](biguint/index.md#biguint)

##### `impl PartialEq for BigUint`

- <span id="biguint-partialeq-eq"></span>`fn eq(&self, other: &BigUint) -> bool` — [`BigUint`](biguint/index.md#biguint)

##### `impl PartialOrd for BigUint`

- <span id="biguint-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BigUint) -> Option<Ordering>` — [`BigUint`](biguint/index.md#biguint)

##### `impl Pow for BigInt`

- <span id="bigint-pow-type-output"></span>`type Output = BigInt`

- <span id="bigint-pow"></span>`fn pow(self, rhs: BigUint) -> BigInt` — [`BigUint`](biguint/index.md#biguint), [`BigInt`](bigint/index.md#bigint)

##### `impl<T> Product for super::BigUint`

- <span id="superbiguint-product"></span>`fn product<I>(iter: I) -> Self`

##### `impl<Base> RefNum for BigUint`

##### `impl Rem for super::BigUint`

- <span id="superbiguint-rem-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-rem"></span>`fn rem(self, other: &BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl RemAssign for super::BigUint`

- <span id="superbiguint-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl Roots for BigUint`

- <span id="biguint-roots-nth-root"></span>`fn nth_root(&self, n: u32) -> Self`

- <span id="biguint-roots-sqrt"></span>`fn sqrt(&self) -> Self`

- <span id="biguint-roots-cbrt"></span>`fn cbrt(&self) -> Self`

##### `impl Shl for BigUint`

- <span id="biguint-shl-type-output"></span>`type Output = BigUint`

- <span id="biguint-shl"></span>`fn shl(self, rhs: u8) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl ShlAssign for BigUint`

- <span id="biguint-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: u8)`

##### `impl Shr for BigUint`

- <span id="biguint-shr-type-output"></span>`type Output = BigUint`

- <span id="biguint-shr"></span>`fn shr(self, rhs: u8) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl ShrAssign for BigUint`

- <span id="biguint-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: u8)`

##### `impl Sub for super::BigUint`

- <span id="superbiguint-sub-type-output"></span>`type Output = BigUint`

- <span id="superbiguint-sub"></span>`fn sub(self, other: BigUint) -> BigUint` — [`BigUint`](biguint/index.md#biguint)

##### `impl SubAssign for super::BigUint`

- <span id="superbiguint-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: BigUint)` — [`BigUint`](biguint/index.md#biguint)

##### `impl<T> Sum for super::BigUint`

- <span id="superbiguint-sum"></span>`fn sum<I>(iter: I) -> Self`

##### `impl ToBigInt for crate::BigUint`

- <span id="cratebiguint-tobigint-to-bigint"></span>`fn to_bigint(&self) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

##### `impl ToBigUint for super::BigUint`

- <span id="superbiguint-tobiguint-to-biguint"></span>`fn to_biguint(&self) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

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

- <span id="biguint-zero"></span>`fn zero() -> BigUint` — [`BigUint`](biguint/index.md#biguint)

- <span id="biguint-zero-set-zero"></span>`fn set_zero(&mut self)`

- <span id="biguint-zero-is-zero"></span>`fn is_zero(&self) -> bool`

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

- <span id="bigint-new"></span>`fn new(sign: Sign, digits: Vec<u32>) -> BigInt` — [`Sign`](bigint/index.md#sign), [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-from-biguint"></span>`fn from_biguint(sign: Sign, data: BigUint) -> BigInt` — [`Sign`](bigint/index.md#sign), [`BigUint`](biguint/index.md#biguint), [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-from-slice"></span>`fn from_slice(sign: Sign, slice: &[u32]) -> BigInt` — [`Sign`](bigint/index.md#sign), [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-assign-from-slice"></span>`fn assign_from_slice(&mut self, sign: Sign, slice: &[u32])` — [`Sign`](bigint/index.md#sign)

  Reinitializes a [`BigInt`](bigint/index.md).

  

  The base 2<sup>32</sup> digits are ordered least significant digit first.

- <span id="bigint-from-bytes-be"></span>`fn from_bytes_be(sign: Sign, bytes: &[u8]) -> BigInt` — [`Sign`](bigint/index.md#sign), [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md).

  

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

- <span id="bigint-from-bytes-le"></span>`fn from_bytes_le(sign: Sign, bytes: &[u8]) -> BigInt` — [`Sign`](bigint/index.md#sign), [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md).

  

  The bytes are in little-endian byte order.

- <span id="bigint-from-signed-bytes-be"></span>`fn from_signed_bytes_be(digits: &[u8]) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md) from an array of bytes in

  two's complement binary representation.

  

  The digits are in big-endian base 2<sup>8</sup>.

- <span id="bigint-from-signed-bytes-le"></span>`fn from_signed_bytes_le(digits: &[u8]) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md) from an array of bytes in two's complement.

  

  The digits are in little-endian base 2<sup>8</sup>.

- <span id="bigint-parse-bytes"></span>`fn parse_bytes(buf: &[u8], radix: u32) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md).

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, ToBigInt};

  

  assert_eq!(BigInt::parse_bytes(b"1234", 10), ToBigInt::to_bigint(&1234));

  assert_eq!(BigInt::parse_bytes(b"ABCD", 16), ToBigInt::to_bigint(&0xABCD));

  assert_eq!(BigInt::parse_bytes(b"G", 16), None);

  ```

- <span id="bigint-from-radix-be"></span>`fn from_radix_be(sign: Sign, buf: &[u8], radix: u32) -> Option<BigInt>` — [`Sign`](bigint/index.md#sign), [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md). Each `u8` of the input slice is

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

- <span id="bigint-from-radix-le"></span>`fn from_radix_le(sign: Sign, buf: &[u8], radix: u32) -> Option<BigInt>` — [`Sign`](bigint/index.md#sign), [`BigInt`](bigint/index.md#bigint)

  Creates and initializes a [`BigInt`](bigint/index.md). Each `u8` of the input slice is

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

- <span id="bigint-to-bytes-be"></span>`fn to_bytes_be(&self) -> (Sign, Vec<u8>)` — [`Sign`](bigint/index.md#sign)

  Returns the sign and the byte representation of the [`BigInt`](bigint/index.md) in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::{ToBigInt, Sign};

  

  let i = -1125.to_bigint().unwrap();

  assert_eq!(i.to_bytes_be(), (Sign::Minus, vec![4, 101]));

  ```

- <span id="bigint-to-bytes-le"></span>`fn to_bytes_le(&self) -> (Sign, Vec<u8>)` — [`Sign`](bigint/index.md#sign)

  Returns the sign and the byte representation of the [`BigInt`](bigint/index.md) in little-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::{ToBigInt, Sign};

  

  let i = -1125.to_bigint().unwrap();

  assert_eq!(i.to_bytes_le(), (Sign::Minus, vec![101, 4]));

  ```

- <span id="bigint-to-u32-digits"></span>`fn to_u32_digits(&self) -> (Sign, Vec<u32>)` — [`Sign`](bigint/index.md#sign)

  Returns the sign and the `u32` digits representation of the [`BigInt`](bigint/index.md) ordered least

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

- <span id="bigint-to-u64-digits"></span>`fn to_u64_digits(&self) -> (Sign, Vec<u64>)` — [`Sign`](bigint/index.md#sign)

  Returns the sign and the `u64` digits representation of the [`BigInt`](bigint/index.md) ordered least

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

- <span id="bigint-iter-u32-digits"></span>`fn iter_u32_digits(&self) -> U32Digits<'_>` — [`U32Digits`](biguint/iter/index.md#u32digits)

  Returns an iterator of `u32` digits representation of the [`BigInt`](bigint/index.md) ordered least

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

- <span id="bigint-iter-u64-digits"></span>`fn iter_u64_digits(&self) -> U64Digits<'_>` — [`U64Digits`](biguint/iter/index.md#u64digits)

  Returns an iterator of `u64` digits representation of the [`BigInt`](bigint/index.md) ordered least

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

  Returns the two's-complement byte representation of the [`BigInt`](bigint/index.md) in big-endian byte order.

  

  # Examples

  

  ```rust

  use num_bigint::ToBigInt;

  

  let i = -1125.to_bigint().unwrap();

  assert_eq!(i.to_signed_bytes_be(), vec![251, 155]);

  ```

- <span id="bigint-to-signed-bytes-le"></span>`fn to_signed_bytes_le(&self) -> Vec<u8>`

  Returns the two's-complement byte representation of the [`BigInt`](bigint/index.md) in little-endian byte order.

  

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

- <span id="bigint-to-radix-be"></span>`fn to_radix_be(&self, radix: u32) -> (Sign, Vec<u8>)` — [`Sign`](bigint/index.md#sign)

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

- <span id="bigint-to-radix-le"></span>`fn to_radix_le(&self, radix: u32) -> (Sign, Vec<u8>)` — [`Sign`](bigint/index.md#sign)

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

- <span id="bigint-sign"></span>`fn sign(&self) -> Sign` — [`Sign`](bigint/index.md#sign)

  Returns the sign of the [`BigInt`](bigint/index.md) as a [`Sign`](bigint/index.md).

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, Sign};

  

  assert_eq!(BigInt::from(1234).sign(), Sign::Plus);

  assert_eq!(BigInt::from(-4321).sign(), Sign::Minus);

  assert_eq!(BigInt::ZERO.sign(), Sign::NoSign);

  ```

- <span id="bigint-magnitude"></span>`fn magnitude(&self) -> &BigUint` — [`BigUint`](biguint/index.md#biguint)

  Returns the magnitude of the [`BigInt`](bigint/index.md) as a [`BigUint`](biguint/index.md).

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, BigUint};

  use num_traits::Zero;

  

  assert_eq!(BigInt::from(1234).magnitude(), &BigUint::from(1234u32));

  assert_eq!(BigInt::from(-4321).magnitude(), &BigUint::from(4321u32));

  assert!(BigInt::ZERO.magnitude().is_zero());

  ```

- <span id="bigint-into-parts"></span>`fn into_parts(self) -> (Sign, BigUint)` — [`Sign`](bigint/index.md#sign), [`BigUint`](biguint/index.md#biguint)

  Convert this [`BigInt`](bigint/index.md) into its [`Sign`](bigint/index.md) and [`BigUint`](biguint/index.md) magnitude,

  the reverse of `BigInt::from_biguint()`.

  

  # Examples

  

  ```rust

  use num_bigint::{BigInt, BigUint, Sign};

  

  assert_eq!(BigInt::from(1234).into_parts(), (Sign::Plus, BigUint::from(1234u32)));

  assert_eq!(BigInt::from(-4321).into_parts(), (Sign::Minus, BigUint::from(4321u32)));

  assert_eq!(BigInt::ZERO.into_parts(), (Sign::NoSign, BigUint::ZERO));

  ```

- <span id="bigint-bits"></span>`fn bits(&self) -> u64`

  Determines the fewest bits necessary to express the [`BigInt`](bigint/index.md),

  not including the sign.

- <span id="bigint-to-biguint"></span>`fn to_biguint(&self) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

  Converts this [`BigInt`](bigint/index.md) into a [`BigUint`](biguint/index.md), if it's not negative.

- <span id="bigint-checked-add"></span>`fn checked_add(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-checked-sub"></span>`fn checked_sub(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-checked-mul"></span>`fn checked_mul(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-checked-div"></span>`fn checked_div(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

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

- <span id="superbigint-add"></span>`fn add(self, other: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl AddAssign for super::BigInt`

- <span id="superbigint-addassign-add-assign"></span>`fn add_assign(&mut self, other: &BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl Average for BigInt`

- <span id="bigint-average-average-floor"></span>`fn average_floor(&self, other: &I) -> I`

  Returns the floor value of the average of `self` and `other`.

- <span id="bigint-average-average-ceil"></span>`fn average_ceil(&self, other: &I) -> I`

  Returns the ceil value of the average of `self` and `other`.

##### `impl Binary for BigInt`

- <span id="bigint-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl BitAnd for super::BigInt`

- <span id="superbigint-bitand-type-output"></span>`type Output = BigInt`

- <span id="superbigint-bitand"></span>`fn bitand(self, other: BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl BitAndAssign for super::BigInt`

- <span id="superbigint-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl BitOr for super::BigInt`

- <span id="superbigint-bitor-type-output"></span>`type Output = BigInt`

- <span id="superbigint-bitor"></span>`fn bitor(self, other: BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl BitOrAssign for super::BigInt`

- <span id="superbigint-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl BitXor for super::BigInt`

- <span id="superbigint-bitxor-type-output"></span>`type Output = BigInt`

- <span id="superbigint-bitxor"></span>`fn bitxor(self, other: BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl BitXorAssign for super::BigInt`

- <span id="superbigint-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl Bits for num_bigint::BigInt`

- <span id="num-bigintbigint-bits-type-output"></span>`type Output = BigInt`

##### `impl CheckedAdd for super::BigInt`

- <span id="superbigint-checkedadd-checked-add"></span>`fn checked_add(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

##### `impl CheckedDiv for super::BigInt`

- <span id="superbigint-checkeddiv-checked-div"></span>`fn checked_div(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

##### `impl CheckedEuclid for super::BigInt`

- <span id="superbigint-checkedeuclid-checked-div-euclid"></span>`fn checked_div_euclid(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-checkedeuclid-checked-rem-euclid"></span>`fn checked_rem_euclid(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-checkedeuclid-checked-div-rem-euclid"></span>`fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)>`

##### `impl CheckedMul for super::BigInt`

- <span id="superbigint-checkedmul-checked-mul"></span>`fn checked_mul(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

##### `impl CheckedSub for super::BigInt`

- <span id="superbigint-checkedsub-checked-sub"></span>`fn checked_sub(&self, v: &BigInt) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

##### `impl Clone for BigInt`

- <span id="bigint-clone"></span>`fn clone(&self) -> Self`

- <span id="bigint-clone-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl ConstZero for BigInt`

- <span id="bigint-constzero-const-zero"></span>`const ZERO: Self`

##### `impl Debug for BigInt`

- <span id="bigint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigInt`

- <span id="bigint-default"></span>`fn default() -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl Display for BigInt`

- <span id="bigint-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Div for super::BigInt`

- <span id="superbigint-div-type-output"></span>`type Output = BigInt`

- <span id="superbigint-div"></span>`fn div(self, other: BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl DivAssign for super::BigInt`

- <span id="superbigint-divassign-div-assign"></span>`fn div_assign(&mut self, other: &BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl Eq for BigInt`

##### `impl Euclid for super::BigInt`

- <span id="superbigint-euclid-div-euclid"></span>`fn div_euclid(&self, v: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-euclid-rem-euclid"></span>`fn rem_euclid(&self, v: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-euclid-div-rem-euclid"></span>`fn div_rem_euclid(&self, v: &Self) -> (Self, Self)`

##### `impl FromBytes for BigInt`

- <span id="bigint-frombytes-type-bytes"></span>`type Bytes = [u8]`

- <span id="bigint-frombytes-from-be-bytes"></span>`fn from_be_bytes(bytes: &<Self as >::Bytes) -> Self`

- <span id="bigint-frombytes-from-le-bytes"></span>`fn from_le_bytes(bytes: &<Self as >::Bytes) -> Self`

##### `impl FromPrimitive for super::BigInt`

- <span id="superbigint-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-fromprimitive-from-i128"></span>`fn from_i128(n: i128) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-fromprimitive-from-u128"></span>`fn from_u128(n: u128) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

- <span id="superbigint-fromprimitive-from-f64"></span>`fn from_f64(n: f64) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

##### `impl FromStr for super::BigInt`

- <span id="superbigint-fromstr-type-err"></span>`type Err = ParseBigIntError`

- <span id="superbigint-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<BigInt, ParseBigIntError>` — [`BigInt`](bigint/index.md#bigint), [`ParseBigIntError`](#parsebiginterror)

##### `impl Hash for BigInt`

- <span id="bigint-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl IntDigits for BigInt`

- <span id="bigint-intdigits-digits"></span>`fn digits(&self) -> &[u64]`

- <span id="bigint-intdigits-digits-mut"></span>`fn digits_mut(&mut self) -> &mut Vec<u64>`

- <span id="bigint-intdigits-normalize"></span>`fn normalize(&mut self)`

- <span id="bigint-intdigits-capacity"></span>`fn capacity(&self) -> usize`

- <span id="bigint-intdigits-len"></span>`fn len(&self) -> usize`

##### `impl Integer for BigInt`

- <span id="bigint-integer-div-rem"></span>`fn div_rem(&self, other: &BigInt) -> (BigInt, BigInt)` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-integer-div-floor"></span>`fn div_floor(&self, other: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-integer-mod-floor"></span>`fn mod_floor(&self, other: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-integer-div-mod-floor"></span>`fn div_mod_floor(&self, other: &BigInt) -> (BigInt, BigInt)` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-integer-div-ceil"></span>`fn div_ceil(&self, other: &Self) -> Self`

- <span id="bigint-integer-gcd"></span>`fn gcd(&self, other: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

  Calculates the Greatest Common Divisor (GCD) of the number and `other`.

  

  The result is always positive.

- <span id="bigint-integer-lcm"></span>`fn lcm(&self, other: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

  Calculates the Lowest Common Multiple (LCM) of the number and `other`.

- <span id="bigint-integer-gcd-lcm"></span>`fn gcd_lcm(&self, other: &BigInt) -> (BigInt, BigInt)` — [`BigInt`](bigint/index.md#bigint)

  Calculates the Greatest Common Divisor (GCD) and

  Lowest Common Multiple (LCM) together.

- <span id="bigint-integer-extended-gcd-lcm"></span>`fn extended_gcd_lcm(&self, other: &BigInt) -> (num_integer::ExtendedGcd<BigInt>, BigInt)` — [`BigInt`](bigint/index.md#bigint)

  Greatest common divisor, least common multiple, and Bézout coefficients.

- <span id="bigint-integer-divides"></span>`fn divides(&self, other: &BigInt) -> bool` — [`BigInt`](bigint/index.md#bigint)

  Deprecated, use `is_multiple_of` instead.

- <span id="bigint-integer-is-multiple-of"></span>`fn is_multiple_of(&self, other: &BigInt) -> bool` — [`BigInt`](bigint/index.md#bigint)

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

- <span id="superbigint-mul"></span>`fn mul(self, other: BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl MulAssign for BigInt`

- <span id="bigint-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl Neg for BigInt`

- <span id="bigint-neg-type-output"></span>`type Output = BigInt`

- <span id="bigint-neg"></span>`fn neg(self) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl Not for BigInt`

- <span id="bigint-not-type-output"></span>`type Output = BigInt`

- <span id="bigint-not"></span>`fn not(self) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl Num for super::BigInt`

- <span id="superbigint-num-type-fromstrradixerr"></span>`type FromStrRadixErr = ParseBigIntError`

- <span id="superbigint-num-from-str-radix"></span>`fn from_str_radix(s: &str, radix: u32) -> Result<BigInt, ParseBigIntError>` — [`BigInt`](bigint/index.md#bigint), [`ParseBigIntError`](#parsebiginterror)

  Creates and initializes a [`BigInt`](bigint/index.md).

##### `impl NumAssign for BigInt`

##### `impl<Rhs> NumAssignOps for BigInt`

##### `impl NumAssignRef for BigInt`

##### `impl<Rhs, Output> NumOps for BigInt`

##### `impl NumRef for BigInt`

##### `impl Octal for BigInt`

- <span id="bigint-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl One for BigInt`

- <span id="bigint-one"></span>`fn one() -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-one-set-one"></span>`fn set_one(&mut self)`

- <span id="bigint-one-is-one"></span>`fn is_one(&self) -> bool`

##### `impl Ord for BigInt`

- <span id="bigint-ord-cmp"></span>`fn cmp(&self, other: &BigInt) -> Ordering` — [`BigInt`](bigint/index.md#bigint)

##### `impl PartialEq for BigInt`

- <span id="bigint-partialeq-eq"></span>`fn eq(&self, other: &BigInt) -> bool` — [`BigInt`](bigint/index.md#bigint)

##### `impl PartialOrd for BigInt`

- <span id="bigint-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BigInt) -> Option<Ordering>` — [`BigInt`](bigint/index.md#bigint)

##### `impl Pow for BigInt`

- <span id="bigint-pow-type-output"></span>`type Output = BigInt`

- <span id="bigint-pow"></span>`fn pow(self, rhs: u8) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl<T> Product for super::BigInt`

- <span id="superbigint-product"></span>`fn product<I>(iter: I) -> Self`

##### `impl<Base> RefNum for BigInt`

##### `impl Rem for super::BigInt`

- <span id="superbigint-rem-type-output"></span>`type Output = BigInt`

- <span id="superbigint-rem"></span>`fn rem(self, other: BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl RemAssign for super::BigInt`

- <span id="superbigint-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: &BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl Roots for BigInt`

- <span id="bigint-roots-nth-root"></span>`fn nth_root(&self, n: u32) -> Self`

- <span id="bigint-roots-sqrt"></span>`fn sqrt(&self) -> Self`

- <span id="bigint-roots-cbrt"></span>`fn cbrt(&self) -> Self`

##### `impl Shl for BigInt`

- <span id="bigint-shl-type-output"></span>`type Output = BigInt`

- <span id="bigint-shl"></span>`fn shl(self, rhs: u8) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl ShlAssign for BigInt`

- <span id="bigint-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: u8)`

##### `impl Shr for BigInt`

- <span id="bigint-shr-type-output"></span>`type Output = BigInt`

- <span id="bigint-shr"></span>`fn shr(self, rhs: u8) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl ShrAssign for BigInt`

- <span id="bigint-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: u8)`

##### `impl Signed for BigInt`

- <span id="bigint-signed-abs"></span>`fn abs(&self) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-signed-abs-sub"></span>`fn abs_sub(&self, other: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-signed-signum"></span>`fn signum(&self) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-signed-is-positive"></span>`fn is_positive(&self) -> bool`

- <span id="bigint-signed-is-negative"></span>`fn is_negative(&self) -> bool`

##### `impl Sub for &super::BigInt`

- <span id="superbigint-sub-type-output"></span>`type Output = BigInt`

- <span id="superbigint-sub"></span>`fn sub(self, other: &BigInt) -> BigInt` — [`BigInt`](bigint/index.md#bigint)

##### `impl SubAssign for super::BigInt`

- <span id="superbigint-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: &BigInt)` — [`BigInt`](bigint/index.md#bigint)

##### `impl<T> Sum for super::BigInt`

- <span id="superbigint-sum"></span>`fn sum<I>(iter: I) -> Self`

##### `impl ToBigInt for super::BigInt`

- <span id="superbigint-tobigint-to-bigint"></span>`fn to_bigint(&self) -> Option<BigInt>` — [`BigInt`](bigint/index.md#bigint)

##### `impl ToBigUint for super::BigInt`

- <span id="superbigint-tobiguint-to-biguint"></span>`fn to_biguint(&self) -> Option<BigUint>` — [`BigUint`](biguint/index.md#biguint)

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

- <span id="bigint-zero"></span>`fn zero() -> BigInt` — [`BigInt`](bigint/index.md#bigint)

- <span id="bigint-zero-set-zero"></span>`fn set_zero(&mut self)`

- <span id="bigint-zero-is-zero"></span>`fn is_zero(&self) -> bool`

## Enums

### `BigIntErrorKind`

```rust
enum BigIntErrorKind {
    Empty,
    InvalidDigit,
}
```

#### Trait Implementations

##### `impl Clone for BigIntErrorKind`

- <span id="biginterrorkind-clone"></span>`fn clone(&self) -> BigIntErrorKind` — [`BigIntErrorKind`](#biginterrorkind)

##### `impl Debug for BigIntErrorKind`

- <span id="biginterrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BigIntErrorKind`

##### `impl PartialEq for BigIntErrorKind`

- <span id="biginterrorkind-partialeq-eq"></span>`fn eq(&self, other: &BigIntErrorKind) -> bool` — [`BigIntErrorKind`](#biginterrorkind)

##### `impl StructuralPartialEq for BigIntErrorKind`

### `Sign`

```rust
enum Sign {
    Minus,
    NoSign,
    Plus,
}
```

A `Sign` is a [`BigInt`](bigint/index.md)'s composing element.

#### Trait Implementations

##### `impl Clone for Sign`

- <span id="sign-clone"></span>`fn clone(&self) -> Sign` — [`Sign`](bigint/index.md#sign)

##### `impl Copy for Sign`

##### `impl Debug for Sign`

- <span id="sign-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Sign`

##### `impl Hash for Sign`

- <span id="sign-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Mul for super::Sign`

- <span id="supersign-mul-type-output"></span>`type Output = Sign`

- <span id="supersign-mul"></span>`fn mul(self, other: Sign) -> Sign` — [`Sign`](bigint/index.md#sign)

##### `impl Neg for Sign`

- <span id="sign-neg-type-output"></span>`type Output = Sign`

- <span id="sign-neg"></span>`fn neg(self) -> Sign` — [`Sign`](bigint/index.md#sign)

  Negate `Sign` value.

##### `impl Ord for Sign`

- <span id="sign-ord-cmp"></span>`fn cmp(&self, other: &Sign) -> cmp::Ordering` — [`Sign`](bigint/index.md#sign)

##### `impl PartialEq for Sign`

- <span id="sign-partialeq-eq"></span>`fn eq(&self, other: &Sign) -> bool` — [`Sign`](bigint/index.md#sign)

##### `impl PartialOrd for Sign`

- <span id="sign-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Sign) -> option::Option<cmp::Ordering>` — [`Sign`](bigint/index.md#sign)

##### `impl StructuralPartialEq for Sign`

## Traits

### `ToBigUint`

```rust
trait ToBigUint { ... }
```

A generic trait for converting a value to a [`BigUint`](biguint/index.md).

#### Required Methods

- `fn to_biguint(&self) -> Option<BigUint>`

  Converts the value of `self` to a [`BigUint`](biguint/index.md).

#### Implementors

- [`BigInt`](bigint/index.md#bigint)
- [`BigUint`](biguint/index.md#biguint)
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

### `ToBigInt`

```rust
trait ToBigInt { ... }
```

A generic trait for converting a value to a [`BigInt`](bigint/index.md). This may return
`None` when converting from `f32` or `f64`, and will always succeed
when converting from any integer or unsigned primitive, or [`BigUint`](biguint/index.md).

#### Required Methods

- `fn to_bigint(&self) -> Option<BigInt>`

  Converts the value of `self` to a [`BigInt`](bigint/index.md).

#### Implementors

- [`BigInt`](bigint/index.md#bigint)
- [`BigUint`](biguint/index.md#biguint)
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

## Type Aliases

### `UsizePromotion`

```rust
type UsizePromotion = u64;
```

### `IsizePromotion`

```rust
type IsizePromotion = i64;
```

