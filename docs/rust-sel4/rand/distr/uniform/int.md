**rand > distr > uniform > int**

# Module: distr::uniform::int

## Contents

**Structs**

- [`UniformInt`](#uniformint) - The back-end implementing [`UniformSampler`] for integer types.
- [`UniformUsize`](#uniformusize) - The back-end implementing [`UniformSampler`] for `usize`.

---

## rand::distr::uniform::int::UniformInt

*Struct*

The back-end implementing [`UniformSampler`] for integer types.

Unless you are implementing [`UniformSampler`] for your own type, this type
should not be used directly, use [`Uniform`] instead.

# Implementation notes

For simplicity, we use the same generic struct `UniformInt<X>` for all
integer types `X`. This gives us only one field type, `X`; to store unsigned
values of this size, we take use the fact that these conversions are no-ops.

For a closed range, the number of possible numbers we should generate is
`range = (high - low + 1)`. To avoid bias, we must ensure that the size of
our sample space, `zone`, is a multiple of `range`; other values must be
rejected (by replacing with a new random sample).

As a special case, we use `range = 0` to represent the full range of the
result type (i.e. for `new_inclusive($ty::MIN, $ty::MAX)`).

The optimum `zone` is the largest product of `range` which fits in our
(unsigned) target type. We calculate this by calculating how many numbers we
must reject: `reject = (MAX + 1) % range = (MAX - range + 1) % range`. Any (large)
product of `range` will suffice, thus in `sample_single` we multiply by a
power of 2 via bit-shifting (faster but may cause more rejections).

The smallest integer PRNGs generate is `u32`. For 8- and 16-bit outputs we
use `u32` for our `zone` and samples (because it's not slower and because
it reduces the chance of having to reject a sample). In this case we cannot
store `zone` in the target type since it is too large, however we know
`ints_to_reject < range <= $uty::MAX`.

An alternative to using a modulus is widening multiply: After a widening
multiply by `range`, the result is in the high word. Then comparing the low
word against `zone` makes sure our distribution is uniform.

# Bias

Unless the `unbiased` feature flag is used, outputs may have a small bias.
In the worst case, bias affects 1 in `2^n` samples where n is
56 (`i8` and `u8`), 48 (`i16` and `u16`), 96 (`i32` and `u32`), 64 (`i64`
and `u64`), 128 (`i128` and `u128`).

[`Uniform`]: super::Uniform

**Generic Parameters:**
- X

**Traits:** Eq, Copy

**Trait Implementations:**

- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **PartialEq**
  - `fn eq(self: &Self, other: &UniformInt<X>) -> bool`
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X` - Sample from distribution, Lemire's method, unbiased
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>` - Sample single value, Canon's method, biased
- **Clone**
  - `fn clone(self: &Self) -> UniformInt<X>`



## rand::distr::uniform::int::UniformUsize

*Struct*

The back-end implementing [`UniformSampler`] for `usize`.

# Implementation notes

Sampling a `usize` value is usually used in relation to the length of an
array or other memory structure, thus it is reasonable to assume that the
vast majority of use-cases will have a maximum size under [`u32::MAX`].
In part to optimise for this use-case, but mostly to ensure that results
are portable across 32-bit and 64-bit architectures (as far as is possible),
this implementation will use 32-bit sampling when possible.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UniformUsize`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> usize`
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &UniformUsize) -> bool`



