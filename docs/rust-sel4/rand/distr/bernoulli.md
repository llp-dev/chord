**rand > distr > bernoulli**

# Module: distr::bernoulli

## Contents

**Structs**

- [`Bernoulli`](#bernoulli) - The [Bernoulli distribution](https://en.wikipedia.org/wiki/Bernoulli_distribution) `Bernoulli(p)`.

**Enums**

- [`BernoulliError`](#bernoullierror) - Error type returned from [`Bernoulli::new`].

---

## rand::distr::bernoulli::Bernoulli

*Struct*

The [Bernoulli distribution](https://en.wikipedia.org/wiki/Bernoulli_distribution) `Bernoulli(p)`.

This distribution describes a single boolean random variable, which is true
with probability `p` and false with probability `1 - p`.
It is a special case of the Binomial distribution with `n = 1`.

# Plot

The following plot shows the Bernoulli distribution with `p = 0.1`,
`p = 0.5`, and `p = 0.9`.

![Bernoulli distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/bernoulli.svg)

# Example

```rust
use rand::distr::{Bernoulli, Distribution};

let d = Bernoulli::new(0.3).unwrap();
let v = d.sample(&mut rand::rng());
println!("{} is from a Bernoulli distribution", v);
```

# Precision

This `Bernoulli` distribution uses 64 bits from the RNG (a `u64`),
so only probabilities that are multiples of 2<sup>-64</sup> can be
represented.

**Methods:**

- `fn new(p: f64) -> Result<Bernoulli, BernoulliError>` - Construct a new `Bernoulli` with the given probability of success `p`.
- `fn from_ratio(numerator: u32, denominator: u32) -> Result<Bernoulli, BernoulliError>` - Construct a new `Bernoulli` with the probability of success of
- `fn p(self: &Self) -> f64` - Returns the probability (`p`) of the distribution.

**Traits:** Copy

**Trait Implementations:**

- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &Bernoulli) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Bernoulli`



## rand::distr::bernoulli::BernoulliError

*Enum*

Error type returned from [`Bernoulli::new`].

**Variants:**
- `InvalidProbability` - `p < 0` or `p > 1`.

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> BernoulliError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BernoulliError) -> bool`



