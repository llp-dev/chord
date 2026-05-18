**rand > distr > uniform**

# Module: distr::uniform

## Contents

**Structs**

- [`Uniform`](#uniform) - Sample values uniformly between two bounds.

**Enums**

- [`Error`](#error) - Error type returned from [`Uniform::new`] and `new_inclusive`.

**Traits**

- [`SampleBorrow`](#sampleborrow) - Helper trait similar to [`Borrow`] but implemented
- [`SampleRange`](#samplerange) - Range that supports generating a single sample efficiently.
- [`SampleUniform`](#sampleuniform) - Helper trait for creating objects using the correct implementation of
- [`UniformSampler`](#uniformsampler) - Helper trait handling actual uniform sampling.

---

## rand::distr::uniform::Error

*Enum*

Error type returned from [`Uniform::new`] and `new_inclusive`.

**Variants:**
- `EmptyRange` - `low > high`, or equal in case of exclusive range.
- `NonFinite` - Input or range `high - low` is non-finite. Not relevant to integer types.

**Traits:** Error, Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rand::distr::uniform::SampleBorrow

*Trait*

Helper trait similar to [`Borrow`] but implemented
only for [`SampleUniform`] and references to [`SampleUniform`]
in order to resolve ambiguity issues.

[`Borrow`]: std::borrow::Borrow

**Methods:**

- `borrow`: Immutably borrows from an owned value. See [`Borrow::borrow`]



## rand::distr::uniform::SampleRange

*Trait*

Range that supports generating a single sample efficiently.

Any type implementing this trait can be used to specify the sampled range
for `Rng::random_range`.

**Methods:**

- `sample_single`: Generate a sample from the given range.
- `is_empty`: Check whether the range is empty.



## rand::distr::uniform::SampleUniform

*Trait*

Helper trait for creating objects using the correct implementation of
[`UniformSampler`] for the sampling type.

See the [module documentation] on how to implement [`Uniform`] range
sampling for a custom type.

[module documentation]: crate::distr::uniform

**Methods:**

- `Sampler`: The `UniformSampler` implementation supporting type `X`.



## rand::distr::uniform::Uniform

*Struct*

Sample values uniformly between two bounds.

# Construction

[`Uniform::new`] and [`Uniform::new_inclusive`] construct a uniform
distribution sampling from the given `low` and `high` limits. `Uniform` may
also be constructed via [`TryFrom`] as in `Uniform::try_from(1..=6).unwrap()`.

Constructors may do extra work up front to allow faster sampling of multiple
values. Where only a single sample is required it is suggested to use
[`Rng::random_range`] or one of the `sample_single` methods instead.

When sampling from a constant range, many calculations can happen at
compile-time and all methods should be fast; for floating-point ranges and
the full range of integer types, this should have comparable performance to
the [`StandardUniform`](super::StandardUniform) distribution.

# Provided implementations

- `char` ([`UniformChar`]): samples a range over the implementation for `u32`
- `f32`, `f64` ([`UniformFloat`]): samples approximately uniformly within a
  range; bias may be present in the least-significant bit of the significand
  and the limits of the input range may be sampled even when an open
  (exclusive) range is used
- Integer types ([`UniformInt`]) may show a small bias relative to the
  expected uniform distribution of output. In the worst case, bias affects
  1 in `2^n` samples where n is 56 (`i8` and `u8`), 48 (`i16` and `u16`), 96
  (`i32` and `u32`), 64 (`i64` and `u64`), 128 (`i128` and `u128`).
  The `unbiased` feature flag fixes this bias.
- `usize` ([`UniformUsize`]) is handled specially, using the `u32`
  implementation where possible to enable portable results across 32-bit and
  64-bit CPU architectures.
- `Duration` ([`UniformDuration`]): samples a range over the implementation
  for `u32` or `u64`
- SIMD types (requires [`simd_support`] feature) like x86's [`__m128i`]
  and `std::simd`'s [`u32x4`], [`f32x4`] and [`mask32x4`] types are
  effectively arrays of integer or floating-point types. Each lane is
  sampled independently from its own range, potentially with more efficient
  random-bit-usage than would be achieved with sequential sampling.

# Example

```
use rand::distr::{Distribution, Uniform};

let between = Uniform::try_from(10..10000).unwrap();
let mut rng = rand::rng();
let mut sum = 0;
for _ in 0..1000 {
    sum += between.sample(&mut rng);
}
println!("{}", sum);
```

For a single sample, [`Rng::random_range`] may be preferred:

```
use rand::RngExt;

let mut rng = rand::rng();
println!("{}", rng.random_range(0..10));
```

[`new`]: Uniform::new
[`new_inclusive`]: Uniform::new_inclusive
[`Rng::random_range`]: RngExt::random_range
[`__m128i`]: https://doc.rust-lang.org/core/arch/x86/struct.__m128i.html
[`u32x4`]: std::simd::u32x4
[`f32x4`]: std::simd::f32x4
[`mask32x4`]: std::simd::mask32x4
[`simd_support`]: https://github.com/rust-random/rand#crate-features

**Generic Parameters:**
- X

**Tuple Struct**: `()`

**Methods:**

- `fn new<B1, B2>(low: B1, high: B2) -> Result<Uniform<X>, Error>` - Create a new `Uniform` instance, which samples uniformly from the half
- `fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Uniform<X>, Error>` - Create a new `Uniform` instance, which samples uniformly from the closed

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Uniform<X>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Uniform<X>`
- **TryFrom**
  - `fn try_from(r: ::core::ops::RangeInclusive<X>) -> Result<Uniform<X>, Error>`
- **TryFrom**
  - `fn try_from(r: Range<X>) -> Result<Uniform<X>, Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> X`



## rand::distr::uniform::UniformSampler

*Trait*

Helper trait handling actual uniform sampling.

See the [module documentation] on how to implement [`Uniform`] range
sampling for a custom type.

Implementation of [`sample_single`] is optional, and is only useful when
the implementation can be faster than `Self::new(low, high).sample(rng)`.

[module documentation]: crate::distr::uniform
[`sample_single`]: UniformSampler::sample_single

**Methods:**

- `X`: The type sampled by this implementation.
- `new`: Construct self, with inclusive lower bound and exclusive upper bound `[low, high)`.
- `new_inclusive`: Construct self, with inclusive bounds `[low, high]`.
- `sample`: Sample a value.
- `sample_single`: Sample a single value uniformly from a range with inclusive lower bound
- `sample_single_inclusive`: Sample a single value uniformly from a range with inclusive lower bound



