**rand > distr > uniform > float**

# Module: distr::uniform::float

## Contents

**Structs**

- [`UniformFloat`](#uniformfloat) - The back-end implementing [`UniformSampler`] for floating-point types.

---

## rand::distr::uniform::float::UniformFloat

*Struct*

The back-end implementing [`UniformSampler`] for floating-point types.

Unless you are implementing [`UniformSampler`] for your own type, this type
should not be used directly, use [`Uniform`] instead.

# Implementation notes

`UniformFloat` implementations convert RNG output to a float in the range
`[1, 2)` via transmutation, map this to `[0, 1)`, then scale and translate
to the desired range. Values produced this way have what equals 23 bits of
random digits for an `f32` and 52 for an `f64`.

# Bias and range errors

Bias may be expected within the least-significant bit of the significand.
It is not guaranteed that exclusive limits of a range are respected; i.e.
when sampling the range `[a, b)` it is not guaranteed that `b` is never
sampled.

[`new`]: UniformSampler::new
[`new_inclusive`]: UniformSampler::new_inclusive
[`StandardUniform`]: crate::distr::StandardUniform
[`Uniform`]: super::Uniform

**Generic Parameters:**
- X

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UniformFloat<X>`
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X`
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **UniformSampler**
  - `fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>`
  - `fn sample<R>(self: &Self, rng: & mut R) -> <Self as >::X`
  - `fn sample_single<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
  - `fn sample_single_inclusive<R, B1, B2>(low_b: B1, high_b: B2, rng: & mut R) -> Result<<Self as >::X, Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &UniformFloat<X>) -> bool`



