**rand > distr**

# Module: distr

## Contents

**Modules**

- [`slice`](#slice) - Distributions over slices
- [`uniform`](#uniform) - A distribution uniformly sampling numbers within a given range.

**Structs**

- [`StandardUniform`](#standarduniform) - The Standard Uniform distribution

---

## rand::distr::StandardUniform

*Struct*

The Standard Uniform distribution

This [`Distribution`] is the *standard* parameterization of [`Uniform`]. Bounds
are selected according to the output type.

Assuming the provided `Rng` is well-behaved, these implementations
generate values with the following ranges and distributions:

* Integers (`i8`, `i32`, `u64`, etc.) are uniformly distributed
  over the whole range of the type (thus each possible value may be sampled
  with equal probability).
* `char` is uniformly distributed over all Unicode scalar values, i.e. all
  code points in the range `0...0x10_FFFF`, except for the range
  `0xD800...0xDFFF` (the surrogate code points). This includes
  unassigned/reserved code points.
  For some uses, the [`Alphanumeric`] or [`Alphabetic`] distribution will be more
  appropriate.
* `bool` samples `false` or `true`, each with probability 0.5.
* Floating point types (`f32` and `f64`) are uniformly distributed in the
  half-open range `[0, 1)`. See also the [notes below](#floating-point-implementation).
* Wrapping integers ([`Wrapping<T>`]), besides the type identical to their
  normal integer variants.
* Non-zero integers ([`NonZeroU8`]), which are like their normal integer
  variants but cannot sample zero.

The `StandardUniform` distribution also supports generation of the following
compound types where all component types are supported:

* Tuples (up to 12 elements): each element is sampled sequentially and
  independently (thus, assuming a well-behaved RNG, there is no correlation
  between elements).
* Arrays `[T; n]` where `T` is supported. Each element is sampled
  sequentially and independently. Note that for small `T` this usually
  results in the RNG discarding random bits; see also [`RngExt::fill`] which
  offers a more efficient approach to filling an array of integer types
  with random data.
* SIMD types (requires [`simd_support`] feature) like x86's [`__m128i`]
  and `std::simd`'s [`u32x4`], [`f32x4`] and [`mask32x4`] types are
  effectively arrays of integer or floating-point types. Each lane is
  sampled independently, potentially with more efficient random-bit-usage
  (and a different resulting value) than would be achieved with sequential
  sampling (as with the array types above).

## Custom implementations

The [`StandardUniform`] distribution may be implemented for user types as follows:

```
# #![allow(dead_code)]
use rand::{Rng, RngExt};
use rand::distr::{Distribution, StandardUniform};

struct MyF32 {
    x: f32,
}

impl Distribution<MyF32> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyF32 {
        MyF32 { x: rng.random() }
    }
}
```

## Example usage
```
use rand::prelude::*;
use rand::distr::StandardUniform;

let val: f32 = rand::rng().sample(StandardUniform);
println!("f32 from [0, 1): {}", val);
```

# Floating point implementation
The floating point implementations for `StandardUniform` generate a random value in
the half-open interval `[0, 1)`, i.e. including 0 but not 1.

All values that can be generated are of the form `n * ε/2`. For `f32`
the 24 most significant random bits of a `u32` are used and for `f64` the
53 most significant bits of a `u64` are used. The conversion uses the
multiplicative method: `(rng.gen::<$uty>() >> N) as $ty * (ε/2)`.

See also: [`Open01`] which samples from `(0, 1)`, [`OpenClosed01`] which
samples from `(0, 1]` and `Rng::random_range(0..1)` which also samples from
`[0, 1)`. Note that `Open01` uses transmute-based methods which yield 1 bit
less precision but may perform faster on some architectures (on modern Intel
CPUs all methods have approximately equal performance).

[`Uniform`]: uniform::Uniform
[`Wrapping<T>`]: std::num::Wrapping
[`NonZeroU8`]: std::num::NonZeroU8
[`__m128i`]: https://doc.rust-lang.org/core/arch/x86/struct.__m128i.html
[`u32x4`]: std::simd::u32x4
[`f32x4`]: std::simd::f32x4
[`mask32x4`]: std::simd::mask32x4
[`simd_support`]: https://github.com/rust-random/rand#crate-features

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroI32`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> [T; N]`
- **Default**
  - `fn default() -> StandardUniform`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> i64`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E, F, G, H, I, J, K)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> i16`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> f32`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E, F, G, H, I)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> u128`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroU64`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E, F, G)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> u32`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> u8`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroI64`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> Wrapping<T>`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> StandardUniform`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroU128`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> char`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroI128`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> i128`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E, F, G, H, I, J, K, L)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> i32`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroI8`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E, F, G, H, I, J)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> i8`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E, F, G, H)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> u64`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D, E, F)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> u16`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> __m128i`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroU16`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B, C, D)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroU8`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> (A, B)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> f64`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R)`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroI16`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> __m256i`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> NonZeroU32`



## Module: slice

Distributions over slices



## Module: uniform

A distribution uniformly sampling numbers within a given range.

[`Uniform`] is the standard distribution to sample uniformly from a range;
e.g. `Uniform::new_inclusive(1, 6).unwrap()` can sample integers from 1 to 6, like a
standard die. [`RngExt::random_range`] is implemented over [`Uniform`].

# Example usage

```
use rand::RngExt;
use rand::distr::Uniform;

let mut rng = rand::rng();
let side = Uniform::new(-10.0, 10.0).unwrap();

// sample between 1 and 10 points
for _ in 0..rng.random_range(1..=10) {
    // sample a point from the square with sides -10 - 10 in two dimensions
    let (x, y) = (rng.sample(side), rng.sample(side));
    println!("Point: {}, {}", x, y);
}
```

# Extending `Uniform` to support a custom type

To extend [`Uniform`] to support your own types, write a back-end which
implements the [`UniformSampler`] trait, then implement the [`SampleUniform`]
helper trait to "register" your back-end. See the `MyF32` example below.

At a minimum, the back-end needs to store any parameters needed for sampling
(e.g. the target range) and implement `new`, `new_inclusive` and `sample`.
Those methods should include an assertion to check the range is valid (i.e.
`low < high`). The example below merely wraps another back-end.

The `new`, `new_inclusive`, `sample_single` and `sample_single_inclusive`
functions use arguments of
type `SampleBorrow<X>` to support passing in values by reference or
by value. In the implementation of these functions, you can choose to
simply use the reference returned by [`SampleBorrow::borrow`], or you can choose
to copy or clone the value, whatever is appropriate for your type.

```
use rand::prelude::*;
use rand::distr::uniform::{Uniform, SampleUniform,
        UniformSampler, UniformFloat, SampleBorrow, Error};

struct MyF32(f32);

#[derive(Clone, Copy, Debug)]
struct UniformMyF32(UniformFloat<f32>);

impl UniformSampler for UniformMyF32 {
    type X = MyF32;

    fn new<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
        where B1: SampleBorrow<Self::X> + Sized,
              B2: SampleBorrow<Self::X> + Sized
    {
        UniformFloat::<f32>::new(low.borrow().0, high.borrow().0).map(UniformMyF32)
    }
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Self, Error>
        where B1: SampleBorrow<Self::X> + Sized,
              B2: SampleBorrow<Self::X> + Sized
    {
        UniformFloat::<f32>::new_inclusive(low.borrow().0, high.borrow().0).map(UniformMyF32)
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        MyF32(self.0.sample(rng))
    }
}

impl SampleUniform for MyF32 {
    type Sampler = UniformMyF32;
}

let (low, high) = (MyF32(17.0f32), MyF32(22.0f32));
let uniform = Uniform::new(low, high).unwrap();
let x = uniform.sample(&mut rand::rng());
```

[`SampleUniform`]: crate::distr::uniform::SampleUniform
[`UniformSampler`]: crate::distr::uniform::UniformSampler
[`UniformInt`]: crate::distr::uniform::UniformInt
[`UniformFloat`]: crate::distr::uniform::UniformFloat
[`UniformDuration`]: crate::distr::uniform::UniformDuration
[`SampleBorrow::borrow`]: crate::distr::uniform::SampleBorrow::borrow



