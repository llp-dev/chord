**rand > distr > float**

# Module: distr::float

## Contents

**Structs**

- [`Open01`](#open01) - A distribution to sample floating point numbers uniformly in the open
- [`OpenClosed01`](#openclosed01) - A distribution to sample floating point numbers uniformly in the half-open

---

## rand::distr::float::Open01

*Struct*

A distribution to sample floating point numbers uniformly in the open
interval `(0, 1)`, i.e. not including either endpoint.

All values that can be generated are of the form `n * ε + ε/2`. For `f32`
the 23 most significant random bits of an `u32` are used, for `f64` 52 from
an `u64`. The conversion uses a transmute-based method.

See also: [`StandardUniform`] which samples from `[0, 1)`, [`OpenClosed01`]
which samples from `(0, 1]` and [`Uniform`] which samples from arbitrary
ranges.

# Example
```
use rand::RngExt;
use rand::distr::Open01;

let val: f32 = rand::rng().sample(Open01);
println!("f32 from (0, 1): {}", val);
```

[`StandardUniform`]: crate::distr::StandardUniform
[`OpenClosed01`]: crate::distr::OpenClosed01
[`Uniform`]: crate::distr::uniform::Uniform

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Open01`
- **Clone**
  - `fn clone(self: &Self) -> Open01`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> f32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> f64`



## rand::distr::float::OpenClosed01

*Struct*

A distribution to sample floating point numbers uniformly in the half-open
interval `(0, 1]`, i.e. including 1 but not 0.

All values that can be generated are of the form `n * ε/2`. For `f32`
the 24 most significant random bits of a `u32` are used and for `f64` the
53 most significant bits of a `u64` are used. The conversion uses the
multiplicative method.

See also: [`StandardUniform`] which samples from `[0, 1)`, [`Open01`]
which samples from `(0, 1)` and [`Uniform`] which samples from arbitrary
ranges.

# Example
```
use rand::RngExt;
use rand::distr::OpenClosed01;

let val: f32 = rand::rng().sample(OpenClosed01);
println!("f32 from (0, 1): {}", val);
```

[`StandardUniform`]: crate::distr::StandardUniform
[`Open01`]: crate::distr::Open01
[`Uniform`]: crate::distr::uniform::Uniform

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> OpenClosed01`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> f64`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> f32`
- **Clone**
  - `fn clone(self: &Self) -> OpenClosed01`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



