**num_rational**

# Module: num_rational

## Contents

**Structs**

- [`ParseRatioError`](#parseratioerror)
- [`Ratio`](#ratio) - Represents the ratio between two numbers.

**Type Aliases**

- [`BigRational`](#bigrational) - Alias for arbitrary precision rationals.
- [`Rational`](#rational) - Alias for a `Ratio` of machine-sized integers.
- [`Rational32`](#rational32) - Alias for a `Ratio` of 32-bit-sized integers.
- [`Rational64`](#rational64) - Alias for a `Ratio` of 64-bit-sized integers.

---

## num_rational::BigRational

*Type Alias*: `Ratio<num_bigint::BigInt>`

Alias for arbitrary precision rationals.



## num_rational::ParseRatioError

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseRatioError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParseRatioError`
- **Error**
  - `fn description(self: &Self) -> &str`



## num_rational::Ratio

*Struct*

Represents the ratio between two numbers.

**Generic Parameters:**
- T

**Methods:**

- `fn new(numer: T, denom: T) -> Ratio<T>` - Creates a new `Ratio`.
- `fn from_integer(t: T) -> Ratio<T>` - Creates a `Ratio` representing the integer `t`.
- `fn to_integer(self: &Self) -> T` - Converts to an integer, rounding towards zero.
- `fn is_integer(self: &Self) -> bool` - Returns true if the rational number is an integer (denominator is 1).
- `fn reduced(self: &Self) -> Ratio<T>` - Returns a reduced copy of self.
- `fn recip(self: &Self) -> Ratio<T>` - Returns the reciprocal.
- `fn floor(self: &Self) -> Ratio<T>` - Rounds towards minus infinity.
- `fn ceil(self: &Self) -> Ratio<T>` - Rounds towards plus infinity.
- `fn round(self: &Self) -> Ratio<T>` - Rounds to the nearest integer. Rounds half-way cases away from zero.
- `fn trunc(self: &Self) -> Ratio<T>` - Rounds towards zero.
- `fn fract(self: &Self) -> Ratio<T>` - Returns the fractional part of a number, with division rounded towards zero.
- `fn pow(self: &Self, expon: i32) -> Ratio<T>` - Raises the `Ratio` to the power of an exponent.
- `fn approximate_float_unsigned<F>(f: F) -> Option<Ratio<T>>`
- `fn approximate_float<F>(f: F) -> Option<Ratio<T>>`
- `fn new_raw(numer: T, denom: T) -> Ratio<T>` - Creates a `Ratio` without checking for `denom == 0` or reducing.
- `fn into_raw(self: Self) -> (T, T)` - Deconstructs a `Ratio` into its numerator and denominator.
- `fn numer(self: &Self) -> &T` - Gets an immutable reference to the numerator.
- `fn denom(self: &Self) -> &T` - Gets an immutable reference to the denominator.
- `fn from_float<T>(f: T) -> Option<BigRational>` - Converts a float into a rational number.

**Traits:** Copy, ConstZero, Eq, ConstOne

**Trait Implementations:**

- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: T)`
- **Pow**
  - `fn pow(self: Self, expon: u128) -> Ratio<T>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: T)`
- **Mul**
  - `fn mul(self: Self, rhs: T) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: i32) -> Ratio<T>`
- **Octal**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: T)`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **Add**
  - `fn add(self: Self, rhs: T) -> Ratio<T>`
- **CheckedMul**
  - `fn checked_mul(self: &Self, rhs: &Ratio<T>) -> Option<Ratio<T>>`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Ratio<T>)`
- **Div**
  - `fn div(self: Self, rhs: Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b u64) -> Ratio<T>`
- **Rem**
  - `fn rem(self: Self, rhs: T) -> Ratio<T>`
- **Signed**
  - `fn abs(self: &Self) -> Ratio<T>`
  - `fn abs_sub(self: &Self, other: &Ratio<T>) -> Ratio<T>`
  - `fn signum(self: &Self) -> Ratio<T>`
  - `fn is_positive(self: &Self) -> bool`
  - `fn is_negative(self: &Self) -> bool`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: Ratio<T>)`
- **Mul**
  - `fn mul(self: Self, other: &T) -> Ratio<T>`
- **Sub**
  - `fn sub(self: Self, rhs: Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b i16) -> Ratio<T>`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Ratio<T>, ParseRatioError>` - Parses `numer/denom` or just `numer`.
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: Ratio<T>)`
- **Add**
  - `fn add(self: Self, other: &T) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b isize) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b BigInt) -> Ratio<T>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: Ratio<T>)`
- **Div**
  - `fn div(self: Self, other: &Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: u64) -> Ratio<T>`
- **Rem**
  - `fn rem(self: Self, other: &T) -> Ratio<T>`
- **Num**
  - `fn from_str_radix(s: &str, radix: u32) -> Result<Ratio<T>, ParseRatioError>` - Parses `numer/denom` where the numbers are in base `radix`.
- **UpperExp**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: Ratio<T>)`
- **Sub**
  - `fn sub(self: Self, other: &Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: i16) -> Ratio<T>`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **Neg**
  - `fn neg(self: Self) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: isize) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: BigInt) -> Ratio<T>`
- **From**
  - `fn from(pair: (T, T)) -> Ratio<T>`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **Pow**
  - `fn pow(self: Self, expon: &'b u32) -> Ratio<T>`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **From**
  - `fn from(x: T) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b i8) -> Ratio<T>`
- **CheckedSub**
  - `fn checked_sub(self: &Self, rhs: &Ratio<T>) -> Option<Ratio<T>>`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **Pow**
  - `fn pow(self: Self, expon: &'b i128) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b BigUint) -> Ratio<T>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **Pow**
  - `fn pow(self: Self, expon: u32) -> Ratio<T>`
- **Zero**
  - `fn zero() -> Ratio<T>`
  - `fn is_zero(self: &Self) -> bool`
  - `fn set_zero(self: & mut Self)`
- **Binary**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &T)`
- **Pow**
  - `fn pow(self: Self, expon: i8) -> Ratio<T>`
- **CheckedDiv**
  - `fn checked_div(self: &Self, rhs: &Ratio<T>) -> Option<Ratio<T>>`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: &Ratio<T>)`
- **Div**
  - `fn div(self: Self, rhs: T) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: i128) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: BigUint) -> Ratio<T>`
- **Clone**
  - `fn clone(self: &Self) -> Ratio<T>`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &T)`
- **Mul**
  - `fn mul(self: Self, rhs: Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b u16) -> Ratio<T>`
- **Sub**
  - `fn sub(self: Self, rhs: T) -> Ratio<T>`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: &Ratio<T>)`
- **Add**
  - `fn add(self: Self, rhs: Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b usize) -> Ratio<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &T)`
- **ToPrimitive**
  - `fn to_i64(self: &Self) -> Option<i64>`
  - `fn to_i128(self: &Self) -> Option<i128>`
  - `fn to_u64(self: &Self) -> Option<u64>`
  - `fn to_u128(self: &Self) -> Option<u128>`
  - `fn to_f64(self: &Self) -> Option<f64>`
- **Div**
  - `fn div(self: Self, other: &T) -> Ratio<T>`
- **Rem**
  - `fn rem(self: Self, rhs: Ratio<T>) -> Ratio<T>`
- **Inv**
  - `fn inv(self: Self) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: &'b i64) -> Ratio<T>`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **MulAssign**
  - `fn mul_assign(self: & mut Self, other: &Ratio<T>)`
- **Mul**
  - `fn mul(self: Self, other: &Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: u16) -> Ratio<T>`
- **Sub**
  - `fn sub(self: Self, other: &T) -> Ratio<T>`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &T)`
- **Add**
  - `fn add(self: Self, other: &Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: usize) -> Ratio<T>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **DivAssign**
  - `fn div_assign(self: & mut Self, other: &Ratio<T>)`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **Rem**
  - `fn rem(self: Self, other: &Ratio<T>) -> Ratio<T>`
- **Pow**
  - `fn pow(self: Self, expon: i64) -> Ratio<T>`
- **LowerExp**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &T)`
- **Pow**
  - `fn pow(self: Self, expon: &'b u8) -> Ratio<T>`
- **One**
  - `fn one() -> Ratio<T>`
  - `fn is_one(self: &Self) -> bool`
  - `fn set_one(self: & mut Self)`
- **Product**
  - `fn product<I>(iter: I) -> Self`
- **AddAssign**
  - `fn add_assign(self: & mut Self, other: &Ratio<T>)`
- **Pow**
  - `fn pow(self: Self, expon: &'b u128) -> Ratio<T>`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`
- **Sum**
  - `fn sum<I>(iter: I) -> Self`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: T)`
- **Pow**
  - `fn pow(self: Self, expon: &'b i32) -> Ratio<T>`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self` - Returns zero
- **RemAssign**
  - `fn rem_assign(self: & mut Self, other: T)`
- **Pow**
  - `fn pow(self: Self, expon: u8) -> Ratio<T>`
- **CheckedAdd**
  - `fn checked_add(self: &Self, rhs: &Ratio<T>) -> Option<Ratio<T>>`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Self>`
  - `fn from_i128(n: i128) -> Option<Self>`
  - `fn from_u64(n: u64) -> Option<Self>`
  - `fn from_u128(n: u128) -> Option<Self>`
  - `fn from_f32(n: f32) -> Option<Self>`
  - `fn from_f64(n: f64) -> Option<Self>`



## num_rational::Rational

*Type Alias*: `Ratio<isize>`

Alias for a `Ratio` of machine-sized integers.



## num_rational::Rational32

*Type Alias*: `Ratio<i32>`

Alias for a `Ratio` of 32-bit-sized integers.



## num_rational::Rational64

*Type Alias*: `Ratio<i64>`

Alias for a `Ratio` of 64-bit-sized integers.



