**rand > distr > distribution**

# Module: distr::distribution

## Contents

**Structs**

- [`Iter`](#iter) - An iterator over a [`Distribution`]
- [`Map`](#map) - A [`Distribution`] which maps sampled values to type `S`

**Traits**

- [`Distribution`](#distribution) - Types (distributions) that can be used to create a random instance of `T`.

---

## rand::distr::distribution::Distribution

*Trait*

Types (distributions) that can be used to create a random instance of `T`.

It is possible to sample from a distribution through both the
`Distribution` and [`RngExt`] traits, via `distr.sample(&mut rng)` and
`rng.sample(distr)`. They also both offer the [`sample_iter`] method, which
produces an iterator that samples from the distribution.

All implementations are expected to be immutable; this has the significant
advantage of not needing to consider thread safety, and for most
distributions efficient state-less sampling algorithms are available.

Implementations are typically expected to be portable with reproducible
results when used with a PRNG with fixed seed; see the
[portability chapter](https://rust-random.github.io/book/portability.html)
of The Rust Rand Book. In some cases this does not apply, e.g. the `usize`
type requires different sampling on 32-bit and 64-bit machines.

[`sample_iter`]: Distribution::sample_iter

**Methods:**

- `sample`: Generate a random value of `T`, using `rng` as the source of randomness.
- `sample_iter`: Create an iterator that generates random values of `T`, using `rng` as
- `map`: Map sampled values to type `S`



## rand::distr::distribution::Iter

*Struct*

An iterator over a [`Distribution`]

This iterator yields random values of type `T` with distribution `D`
from a random generator of type `R`.

Construct this `struct` using [`Distribution::sample_iter`] or
[`RngExt::sample_iter`]. It is also used by [`RngExt::random_iter`] and
[`crate::random_iter`].

**Generic Parameters:**
- D
- R
- T

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<T>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rand::distr::distribution::Map

*Struct*

A [`Distribution`] which maps sampled values to type `S`

This `struct` is created by the [`Distribution::map`] method.
See its documentation for more.

**Generic Parameters:**
- D
- F
- T
- S

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Distribution**
  - `fn sample<R>(self: &Self, rng: & mut R) -> S`



