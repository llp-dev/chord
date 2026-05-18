# Crate `num_rational`

Rational numbers

## Compatibility

The `num-rational` crate is tested for rustc 1.60 and greater.

## Contents

- [Modules](#modules)
  - [`pow`](#pow)
  - [`iter_sum_product`](#iter-sum-product)
  - [`opassign`](#opassign)
- [Structs](#structs)
  - [`Ratio`](#ratio)
  - [`ParseRatioError`](#parseratioerror)
- [Enums](#enums)
  - [`RatioErrorKind`](#ratioerrorkind)
- [Traits](#traits)
  - [`Bits`](#bits)
- [Functions](#functions)
  - [`approximate_float`](#approximate-float)
  - [`approximate_float_unsigned`](#approximate-float-unsigned)
  - [`ratio_to_f64`](#ratio-to-f64)
  - [`ldexp`](#ldexp)
- [Type Aliases](#type-aliases)
  - [`Rational`](#rational)
  - [`Rational32`](#rational32)
  - [`Rational64`](#rational64)
  - [`BigRational`](#bigrational)
- [Macros](#macros)
  - [`forward_ref_ref_binop!`](#forward-ref-ref-binop)
  - [`forward_ref_val_binop!`](#forward-ref-val-binop)
  - [`forward_val_ref_binop!`](#forward-val-ref-binop)
  - [`forward_all_binop!`](#forward-all-binop)
  - [`arith_impl!`](#arith-impl)
  - [`checked_arith_impl!`](#checked-arith-impl)
  - [`impl_formatting!`](#impl-formatting)
  - [`from_primitive_integer!`](#from-primitive-integer)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pow`](#pow) | mod |  |
| [`iter_sum_product`](#iter-sum-product) | mod |  |
| [`opassign`](#opassign) | mod |  |
| [`Ratio`](#ratio) | struct | Represents the ratio between two numbers. |
| [`ParseRatioError`](#parseratioerror) | struct |  |
| [`RatioErrorKind`](#ratioerrorkind) | enum |  |
| [`Bits`](#bits) | trait |  |
| [`approximate_float`](#approximate-float) | fn |  |
| [`approximate_float_unsigned`](#approximate-float-unsigned) | fn |  |
| [`ratio_to_f64`](#ratio-to-f64) | fn | Converts a ratio of `T` to an f64. |
| [`ldexp`](#ldexp) | fn | Multiply `x` by 2 to the power of `exp`. |
| [`Rational`](#rational) | type | Alias for a `Ratio` of machine-sized integers. |
| [`Rational32`](#rational32) | type | Alias for a `Ratio` of 32-bit-sized integers. |
| [`Rational64`](#rational64) | type | Alias for a `Ratio` of 64-bit-sized integers. |
| [`BigRational`](#bigrational) | type | Alias for arbitrary precision rationals. |
| [`forward_ref_ref_binop!`](#forward-ref-ref-binop) | macro |  |
| [`forward_ref_val_binop!`](#forward-ref-val-binop) | macro |  |
| [`forward_val_ref_binop!`](#forward-val-ref-binop) | macro |  |
| [`forward_all_binop!`](#forward-all-binop) | macro |  |
| [`arith_impl!`](#arith-impl) | macro |  |
| [`checked_arith_impl!`](#checked-arith-impl) | macro |  |
| [`impl_formatting!`](#impl-formatting) | macro |  |
| [`from_primitive_integer!`](#from-primitive-integer) | macro |  |

## Modules

- [`pow`](pow/index.md)
- [`iter_sum_product`](iter_sum_product/index.md)
- [`opassign`](opassign/index.md)

## Structs

### `Ratio<T>`

```rust
struct Ratio<T> {
    numer: T,
    denom: T,
}
```

Represents the ratio between two numbers.

#### Fields

- **`numer`**: `T`

  Numerator.

- **`denom`**: `T`

  Denominator.

#### Implementations

- <span id="ratio-new-raw"></span>`const fn new_raw(numer: T, denom: T) -> Ratio<T>` — [`Ratio`](#ratio)

  Creates a `Ratio` without checking for `denom == 0` or reducing.

  

  **There are several methods that will panic if used on a `Ratio` with

  `denom == 0`.**

- <span id="ratio-into-raw"></span>`fn into_raw(self) -> (T, T)`

  Deconstructs a `Ratio` into its numerator and denominator.

- <span id="ratio-numer"></span>`const fn numer(&self) -> &T`

  Gets an immutable reference to the numerator.

- <span id="ratio-denom"></span>`const fn denom(&self) -> &T`

  Gets an immutable reference to the denominator.

#### Trait Implementations

##### `impl<T: Clone + Integer> Add for &'a Ratio<T>`

- <span id="a-ratio-add-type-output"></span>`type Output = Ratio<T>`

- <span id="a-ratio-add"></span>`fn add(self, other: &'b Ratio<T>) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + NumAssign> AddAssign for crate::Ratio<T>`

- <span id="crateratio-addassign-add-assign"></span>`fn add_assign(&mut self, other: Ratio<T>)` — [`Ratio`](#ratio)

##### `impl<T: Binary + Clone + Integer> Binary for Ratio<T>`

- <span id="ratio-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: Clone + Integer + CheckedMul + CheckedAdd> CheckedAdd for Ratio<T>`

- <span id="ratio-checkedadd-checked-add"></span>`fn checked_add(&self, rhs: &Ratio<T>) -> Option<Ratio<T>>` — [`Ratio`](#ratio)

##### `impl<T> CheckedDiv for Ratio<T>`

- <span id="ratio-checkeddiv-checked-div"></span>`fn checked_div(&self, rhs: &Ratio<T>) -> Option<Ratio<T>>` — [`Ratio`](#ratio)

##### `impl<T> CheckedMul for Ratio<T>`

- <span id="ratio-checkedmul-checked-mul"></span>`fn checked_mul(&self, rhs: &Ratio<T>) -> Option<Ratio<T>>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + CheckedMul + CheckedSub> CheckedSub for Ratio<T>`

- <span id="ratio-checkedsub-checked-sub"></span>`fn checked_sub(&self, rhs: &Ratio<T>) -> Option<Ratio<T>>` — [`Ratio`](#ratio)

##### `impl<T: clone::Clone> Clone for Ratio<T>`

- <span id="ratio-clone"></span>`fn clone(&self) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + ConstOne> ConstOne for Ratio<T>`

- <span id="ratio-constone-const-one"></span>`const ONE: Self`

##### `impl<T: Clone + Integer + ConstZero + ConstOne> ConstZero for Ratio<T>`

- <span id="ratio-constzero-const-zero"></span>`const ZERO: Self`

##### `impl<T: marker::Copy> Copy for Ratio<T>`

##### `impl<T: fmt::Debug> Debug for Ratio<T>`

- <span id="ratio-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Clone + Integer> Default for Ratio<T>`

- <span id="ratio-default"></span>`fn default() -> Self`

  Returns zero

##### `impl<T: Display + Clone + Integer> Display for Ratio<T>`

- <span id="ratio-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: Clone + Integer> Div for &'a Ratio<T>`

- <span id="a-ratio-div-type-output"></span>`type Output = Ratio<T>`

- <span id="a-ratio-div"></span>`fn div(self, other: &'b Ratio<T>) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + NumAssign> DivAssign for crate::Ratio<T>`

- <span id="crateratio-divassign-div-assign"></span>`fn div_assign(&mut self, other: Ratio<T>)` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer> Eq for Ratio<T>`

##### `impl FromPrimitive for Ratio<num_bigint::BigInt>`

- <span id="ratio-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<Self>`

- <span id="ratio-fromprimitive-from-i128"></span>`fn from_i128(n: i128) -> Option<Self>`

- <span id="ratio-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<Self>`

- <span id="ratio-fromprimitive-from-u128"></span>`fn from_u128(n: u128) -> Option<Self>`

- <span id="ratio-fromprimitive-from-f32"></span>`fn from_f32(n: f32) -> Option<Self>`

- <span id="ratio-fromprimitive-from-f64"></span>`fn from_f64(n: f64) -> Option<Self>`

##### `impl<T: FromStr + Clone + Integer> FromStr for Ratio<T>`

- <span id="ratio-fromstr-type-err"></span>`type Err = ParseRatioError`

- <span id="ratio-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Ratio<T>, ParseRatioError>` — [`Ratio`](#ratio), [`ParseRatioError`](#parseratioerror)

  Parses `numer/denom` or just `numer`.

##### `impl<T: Clone + Integer + Hash> Hash for Ratio<T>`

- <span id="ratio-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T> Inv for Ratio<T>`

- <span id="ratio-inv-type-output"></span>`type Output = Ratio<T>`

- <span id="ratio-inv"></span>`fn inv(self) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: LowerExp + Clone + Integer> LowerExp for Ratio<T>`

- <span id="ratio-lowerexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: LowerHex + Clone + Integer> LowerHex for Ratio<T>`

- <span id="ratio-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: Clone + Integer> Mul for &'a Ratio<T>`

- <span id="a-ratio-mul-type-output"></span>`type Output = Ratio<T>`

- <span id="a-ratio-mul"></span>`fn mul(self, other: &'b Ratio<T>) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + NumAssign> MulAssign for crate::Ratio<T>`

- <span id="crateratio-mulassign-mul-assign"></span>`fn mul_assign(&mut self, other: Ratio<T>)` — [`Ratio`](#ratio)

##### `impl<T> Neg for Ratio<T>`

- <span id="ratio-neg-type-output"></span>`type Output = Ratio<T>`

- <span id="ratio-neg"></span>`fn neg(self) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer> Num for Ratio<T>`

- <span id="ratio-num-type-fromstrradixerr"></span>`type FromStrRadixErr = ParseRatioError`

- <span id="ratio-num-from-str-radix"></span>`fn from_str_radix(s: &str, radix: u32) -> Result<Ratio<T>, ParseRatioError>` — [`Ratio`](#ratio), [`ParseRatioError`](#parseratioerror)

  Parses `numer/denom` where the numbers are in base `radix`.

##### `impl<T> NumAssign for Ratio<T>`

##### `impl<T, Rhs> NumAssignOps for Ratio<T>`

##### `impl<T> NumAssignRef for Ratio<T>`

##### `impl<T, Rhs, Output> NumOps for Ratio<T>`

##### `impl<T> NumRef for Ratio<T>`

##### `impl<T: Octal + Clone + Integer> Octal for Ratio<T>`

- <span id="ratio-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: Clone + Integer> One for Ratio<T>`

- <span id="ratio-one"></span>`fn one() -> Ratio<T>` — [`Ratio`](#ratio)

- <span id="ratio-one-is-one"></span>`fn is_one(&self) -> bool`

- <span id="ratio-one-set-one"></span>`fn set_one(&mut self)`

##### `impl<T: Clone + Integer> Ord for Ratio<T>`

- <span id="ratio-ord-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl<T: Clone + Integer> PartialEq for Ratio<T>`

- <span id="ratio-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T: Clone + Integer> PartialOrd for Ratio<T>`

- <span id="ratio-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>`

##### `impl<T: Clone + Integer + Pow<u8, Output = T>> Pow for Ratio<T>`

- <span id="ratio-pow-type-output"></span>`type Output = Ratio<T>`

- <span id="ratio-pow"></span>`fn pow(self, expon: u8) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Integer + Clone> Product for crate::Ratio<T>`

- <span id="crateratio-product"></span>`fn product<I>(iter: I) -> Self`

##### `impl<T, Base> RefNum for Ratio<T>`

##### `impl<T: Clone + Integer> Rem for &'a Ratio<T>`

- <span id="a-ratio-rem-type-output"></span>`type Output = Ratio<T>`

- <span id="a-ratio-rem"></span>`fn rem(self, other: &'b Ratio<T>) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + NumAssign> RemAssign for crate::Ratio<T>`

- <span id="crateratio-remassign-rem-assign"></span>`fn rem_assign(&mut self, other: Ratio<T>)` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + Signed> Signed for Ratio<T>`

- <span id="ratio-signed-abs"></span>`fn abs(&self) -> Ratio<T>` — [`Ratio`](#ratio)

- <span id="ratio-signed-abs-sub"></span>`fn abs_sub(&self, other: &Ratio<T>) -> Ratio<T>` — [`Ratio`](#ratio)

- <span id="ratio-signed-signum"></span>`fn signum(&self) -> Ratio<T>` — [`Ratio`](#ratio)

- <span id="ratio-signed-is-positive"></span>`fn is_positive(&self) -> bool`

- <span id="ratio-signed-is-negative"></span>`fn is_negative(&self) -> bool`

##### `impl<T: Clone + Integer> Sub for &'a Ratio<T>`

- <span id="a-ratio-sub-type-output"></span>`type Output = Ratio<T>`

- <span id="a-ratio-sub"></span>`fn sub(self, other: &'b Ratio<T>) -> Ratio<T>` — [`Ratio`](#ratio)

##### `impl<T: Clone + Integer + NumAssign> SubAssign for crate::Ratio<T>`

- <span id="crateratio-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Ratio<T>)` — [`Ratio`](#ratio)

##### `impl<T: Integer + Clone> Sum for crate::Ratio<T>`

- <span id="crateratio-sum"></span>`fn sum<I>(iter: I) -> Self`

##### `impl<T: Clone + Integer + ToPrimitive + ToBigInt> ToPrimitive for Ratio<T>`

- <span id="ratio-toprimitive-to-i64"></span>`fn to_i64(&self) -> Option<i64>`

- <span id="ratio-toprimitive-to-i128"></span>`fn to_i128(&self) -> Option<i128>`

- <span id="ratio-toprimitive-to-u64"></span>`fn to_u64(&self) -> Option<u64>`

- <span id="ratio-toprimitive-to-u128"></span>`fn to_u128(&self) -> Option<u128>`

- <span id="ratio-toprimitive-to-f64"></span>`fn to_f64(&self) -> Option<f64>`

##### `impl<T> ToString for Ratio<T>`

- <span id="ratio-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T: UpperExp + Clone + Integer> UpperExp for Ratio<T>`

- <span id="ratio-upperexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: UpperHex + Clone + Integer> UpperHex for Ratio<T>`

- <span id="ratio-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: Clone + Integer> Zero for Ratio<T>`

- <span id="ratio-zero"></span>`fn zero() -> Ratio<T>` — [`Ratio`](#ratio)

- <span id="ratio-zero-is-zero"></span>`fn is_zero(&self) -> bool`

- <span id="ratio-zero-set-zero"></span>`fn set_zero(&mut self)`

### `ParseRatioError`

```rust
struct ParseRatioError {
    kind: RatioErrorKind,
}
```

#### Trait Implementations

##### `impl Clone for ParseRatioError`

- <span id="parseratioerror-clone"></span>`fn clone(&self) -> ParseRatioError` — [`ParseRatioError`](#parseratioerror)

##### `impl Copy for ParseRatioError`

##### `impl Debug for ParseRatioError`

- <span id="parseratioerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseRatioError`

- <span id="parseratioerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseRatioError`

- <span id="parseratioerror-error-description"></span>`fn description(&self) -> &str`

##### `impl PartialEq for ParseRatioError`

- <span id="parseratioerror-partialeq-eq"></span>`fn eq(&self, other: &ParseRatioError) -> bool` — [`ParseRatioError`](#parseratioerror)

##### `impl StructuralPartialEq for ParseRatioError`

##### `impl ToString for ParseRatioError`

- <span id="parseratioerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `RatioErrorKind`

```rust
enum RatioErrorKind {
    ParseError,
    ZeroDenominator,
}
```

#### Implementations

- <span id="ratioerrorkind-description"></span>`fn description(&self) -> &'static str`

#### Trait Implementations

##### `impl Clone for RatioErrorKind`

- <span id="ratioerrorkind-clone"></span>`fn clone(&self) -> RatioErrorKind` — [`RatioErrorKind`](#ratioerrorkind)

##### `impl Copy for RatioErrorKind`

##### `impl Debug for RatioErrorKind`

- <span id="ratioerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for RatioErrorKind`

- <span id="ratioerrorkind-partialeq-eq"></span>`fn eq(&self, other: &RatioErrorKind) -> bool` — [`RatioErrorKind`](#ratioerrorkind)

##### `impl StructuralPartialEq for RatioErrorKind`

## Traits

### `Bits`

```rust
trait Bits { ... }
```

#### Required Methods

- `fn bits(&self) -> u64`

#### Implementors

- `i128`
- `num_bigint::BigInt`

## Functions

### `approximate_float`

```rust
fn approximate_float<T, F>(val: F, max_error: F, max_iterations: usize) -> Option<Ratio<T>>
where
    T: Integer + Signed + Bounded + NumCast + Clone,
    F: FloatCore + NumCast
```

### `approximate_float_unsigned`

```rust
fn approximate_float_unsigned<T, F>(val: F, max_error: F, max_iterations: usize) -> Option<Ratio<T>>
where
    T: Integer + Bounded + NumCast + Clone,
    F: FloatCore + NumCast
```

### `ratio_to_f64`

```rust
fn ratio_to_f64<T: Bits + Clone + Integer + Signed + ShlAssign<usize> + ToPrimitive>(numer: T, denom: T) -> f64
```

Converts a ratio of `T` to an f64.

In addition to stated trait bounds, `T` must be able to hold numbers 56 bits larger than
the largest of `numer` and `denom`. This is automatically true if `T` is `BigInt`.

### `ldexp`

```rust
fn ldexp(x: f64, exp: i32) -> f64
```

Multiply `x` by 2 to the power of `exp`. Returns an accurate result even if `2^exp` is not
representable.

## Type Aliases

### `Rational`

```rust
type Rational = Ratio<isize>;
```

Alias for a `Ratio` of machine-sized integers.

### `Rational32`

```rust
type Rational32 = Ratio<i32>;
```

Alias for a `Ratio` of 32-bit-sized integers.

### `Rational64`

```rust
type Rational64 = Ratio<i64>;
```

Alias for a `Ratio` of 64-bit-sized integers.

### `BigRational`

```rust
type BigRational = Ratio<num_bigint::BigInt>;
```

Alias for arbitrary precision rationals.

## Macros

### `forward_ref_ref_binop!`

### `forward_ref_val_binop!`

### `forward_val_ref_binop!`

### `forward_all_binop!`

### `arith_impl!`

### `checked_arith_impl!`

### `impl_formatting!`

### `from_primitive_integer!`

