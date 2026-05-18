**rand**

# Module: rand

## Contents

**Modules**

- [`distr`](#distr) - Generating random samples from probability distributions
- [`prelude`](#prelude) - Convenience re-export of common members
- [`rngs`](#rngs) - Random number generators and adapters
- [`seq`](#seq) - Sequence-related functionality

---

## Module: distr

Generating random samples from probability distributions

This module is the home of the [`Distribution`] trait and several of its
implementations. It is the workhorse behind some of the convenient
functionality of the [`RngExt`] trait, e.g. [`RngExt::random`] and of course
[`RngExt::sample`].

Abstractly, a [probability distribution] describes the probability of
occurrence of each value in its sample space.

More concretely, an implementation of `Distribution<T>` for type `X` is an
algorithm for choosing values from the sample space (a subset of `T`)
according to the distribution `X` represents, using an external source of
randomness (an RNG supplied to the `sample` function).

A type `X` may implement `Distribution<T>` for multiple types `T`.
Any type implementing [`Distribution`] is stateless (i.e. immutable),
but it may have internal parameters set at construction time (for example,
[`Uniform`] allows specification of its sample space as a range within `T`).


# The Standard Uniform distribution

The [`StandardUniform`] distribution is important to mention. This is the
distribution used by [`RngExt::random`] and represents the "default" way to
produce a random value for many different types, including most primitive
types, tuples, arrays, and a few derived types. See the documentation of
[`StandardUniform`] for more details.

Implementing [`Distribution<T>`] for [`StandardUniform`] for user types `T` makes it
possible to generate type `T` with [`RngExt::random`], and by extension also
with the [`random`] function.

## Other standard uniform distributions

[`Alphanumeric`] is a simple distribution to sample random letters and
numbers of the `char` type; in contrast [`StandardUniform`] may sample any valid
`char`.

There's also an [`Alphabetic`] distribution which acts similarly to [`Alphanumeric`] but
doesn't include digits.

For floats (`f32`, `f64`), [`StandardUniform`] samples from `[0, 1)`. Also
provided are [`Open01`] (samples from `(0, 1)`) and [`OpenClosed01`]
(samples from `(0, 1]`). No option is provided to sample from `[0, 1]`; it
is suggested to use one of the above half-open ranges since the failure to
sample a value which would have a low chance of being sampled anyway is
rarely an issue in practice.

# Parameterized Uniform distributions

The [`Uniform`] distribution provides uniform sampling over a specified
range on a subset of the types supported by the above distributions.

Implementations support single-value-sampling via
[`Rng::random_range(Range)`](RngExt::random_range).
Where a fixed (non-`const`) range will be sampled many times, it is likely
faster to pre-construct a [`Distribution`] object using
[`Uniform::new`], [`Uniform::new_inclusive`] or `From<Range>`.

# Non-uniform sampling

Sampling a simple true/false outcome with a given probability has a name:
the [`Bernoulli`] distribution (this is used by [`RngExt::random_bool`]).

For weighted sampling of discrete values see the [`weighted`] module.

This crate no longer includes other non-uniform distributions; instead
it is recommended that you use either [`rand_distr`] or [`statrs`].


[probability distribution]: https://en.wikipedia.org/wiki/Probability_distribution
[`rand_distr`]: https://crates.io/crates/rand_distr
[`statrs`]: https://crates.io/crates/statrs
[`random`]: crate::random
[`rand_distr`]: https://crates.io/crates/rand_distr
[`statrs`]: https://crates.io/crates/statrs



## Module: prelude

Convenience re-export of common members

Like the standard library's prelude, this module simplifies importing of
common items. Unlike the standard prelude, the contents of this module must
be imported manually:

```
use rand::prelude::*;
# let mut r: StdRng = rand::make_rng();
# let _: f32 = r.random();
```



## Module: rngs

Random number generators and adapters

## Generators

This crate provides a small selection of generators.
See also [Types of generators] and [Our RNGs] in the book.

##### Non-deterministic generators

-   [`SysRng`] is a stateless interface over the operating system's random number
    source. This is typically secure with some form of periodic re-seeding.
-   [`ThreadRng`], provided by [`crate::rng()`], is a handle to a
    thread-local generator with periodic seeding from [`SysRng`]. Because this
    is local, it is typically much faster than [`SysRng`]. It should be
    secure, but see documentation on [`ThreadRng`].

##### Standard generators

These use selected best-in-class algorithms. They are deterministic but not
portable: the algorithms may be changed in any release and may be
platform-dependent.

-   [`StdRng`] is a CSPRNG chosen for good performance and trust of security
    (based on reviews, maturity and usage). The current algorithm is
    [`ChaCha12Rng`], which is well established and rigorously analysed.
    [`StdRng`] is the deterministic generator used by [`ThreadRng`] but
    without the periodic reseeding or thread-local management.
-   [`SmallRng`] is a relatively simple, insecure generator designed to be
    fast, use little memory, and pass various statistical tests of
    randomness quality. The current algorithm is one of the Xoshiro
    generators below, depending on the target's pointer size.

##### Named portable generators

These are similar to the [standard generators](#standard-generators), but
with the additional [guarantees of reproducibility]:

-   [`Xoshiro256PlusPlus`] is a very fast 64-bit insecure generator using
    256 bits of state with good performance in statistical tests of quality
-   [`Xoshiro128PlusPlus`] is a very fast 32-bit insecure generator using
    128 bits of state with good performance in statistical tests of quality
-   [`ChaCha8Rng`], [`ChaCha12Rng`] and [`ChaCha20Rng`] are generators over
    the ChaCha stream cipher designed by Daniel J. Bernstein[^1].

### Additional generators

-   The [`rdrand`] crate provides an interface to the RDRAND and RDSEED
    instructions available in modern Intel and AMD CPUs.
-   The [`rand_jitter`] crate provides a user-space implementation of
    entropy harvesting from CPU timer jitter, but is very slow and has
    [security issues](https://github.com/rust-random/rand/issues/699).
-   The [`rand_pcg`] crate provides portable implementations of a subset
    of the [PCG] family of small, insecure generators
-   The [`rand_xoshiro`] crate provides portable implementations of the
    [xoshiro] family of small, insecure generators

For more, search [crates with the `rng` tag].

## Traits and functionality

All generators implement [`TryRng`]. Most implement [`Rng`] (i.e.
`TryRng<Error = Infallible>`) and thus also implement [`Rng`][crate::Rng].
See also the [Random Values] chapter in the book.

Secure RNGs may additionally implement the [`CryptoRng`] trait.

Use the [`rand_core`] crate when implementing your own RNGs.

[^1]: D. J. Bernstein, [*ChaCha, a variant of Salsa20*](https://cr.yp.to/chacha.html)

[guarantees of reproducibility]: https://rust-random.github.io/book/crate-reprod.html
[Types of generators]: https://rust-random.github.io/book/guide-gen.html
[Our RNGs]: https://rust-random.github.io/book/guide-rngs.html
[Random Values]: https://rust-random.github.io/book/guide-values.html
[`Rng`]: crate::RngExt
[`TryRng`]: crate::TryRng
[`Rng`]: crate::Rng
[`CryptoRng`]: crate::CryptoRng
[`SeedableRng`]: crate::SeedableRng
[`rdrand`]: https://crates.io/crates/rdrand
[`rand_jitter`]: https://crates.io/crates/rand_jitter
[`rand_pcg`]: https://crates.io/crates/rand_pcg
[`rand_xoshiro`]: https://crates.io/crates/rand_xoshiro
[crates with the `rng` tag]: https://crates.io/keywords/rng
[chacha]: https://cr.yp.to/chacha.html
[PCG]: https://www.pcg-random.org/
[xoshiro]: https://prng.di.unimi.it/



## Module: seq

Sequence-related functionality

This module provides:

*   [`IndexedRandom`] for sampling slices and other indexable lists
*   [`IndexedMutRandom`] for sampling slices and other mutably indexable lists
*   [`SliceRandom`] for mutating slices
*   [`IteratorRandom`] for sampling iterators
*   [`index::sample`] low-level API to choose multiple indices from
    `0..length`

Also see:

*   [`crate::distr::weighted::WeightedIndex`] distribution which provides
    weighted index sampling.

In order to make results reproducible across 32-64 bit architectures, all
`usize` indices are sampled as a `u32` where possible (also providing a
small performance boost in some cases).



