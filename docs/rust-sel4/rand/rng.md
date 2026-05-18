**rand > rng**

# Module: rng

## Contents

**Traits**

- [`Fill`](#fill) - Support filling a slice with random data
- [`RngExt`](#rngext) - User-level interface for RNGs

---

## rand::rng::Fill

*Trait*

Support filling a slice with random data

This trait allows slices of "plain data" types to be efficiently filled
with random data.

Implementations are expected to be portable across machines unless
clearly documented otherwise (see the
[Chapter on Portability](https://rust-random.github.io/book/portability.html)).
The implementations provided achieve this by byte-swapping on big-endian
machines.

**Methods:**

- `fill_slice`: Fill this with random data



## rand::rng::RngExt

*Trait*

User-level interface for RNGs

[`Rng`] is the `dyn`-safe implementation-level interface for Random
(Number) Generators. This trait, `Rng`, provides a user-level interface on
RNGs. It is implemented automatically for any `R: Rng`.

This trait must usually be brought into scope via `use rand::RngExt;` or
`use rand::prelude::*;`.

# Generic usage

The basic pattern is `fn foo<R: Rng + ?Sized>(rng: &mut R)`. Some
things are worth noting here:

- Since `RngExt: Rng` and every `RngExt` implements `Rng`, it makes no
  difference whether we use `R: Rng` or `R: RngExt` for `R: Sized`.
- Only `Rng` is dyn safe, supporting `&mut dyn Rng` and `R: Rng + ?Sized`.

An alternative pattern is possible: `fn foo<R: Rng>(rng: R)`. This has some
trade-offs. It allows the argument to be consumed directly without a `&mut`;
also it still works directly
on references (including type-erased references). Unfortunately within the
function `foo` it is not known whether `rng` is a reference type or not,
hence many uses of `rng` require an extra reference, either explicitly
(`distr.sample(&mut rng)`) or implicitly (`rng.random()`); one may hope the
optimiser can remove redundant references later.

Example:

```
use rand::{Rng, RngExt};

fn foo<R: Rng + ?Sized>(rng: &mut R) -> f32 {
    rng.random()
}

# let v = foo(&mut rand::rng());
```

**Methods:**

- `random`: Return a random value via the [`StandardUniform`] distribution.
- `random_iter`: Return an iterator over [`random`](Self::random) variates
- `random_range`: Generate a random value in the given range.
- `random_bool`: Return a bool with a probability `p` of being true.
- `random_ratio`: Return a bool with a probability of `numerator/denominator` of being
- `sample`: Sample a new value, using the given distribution.
- `sample_iter`: Create an iterator that generates values using the given distribution.
- `fill`: Fill any type implementing [`Fill`] with random data



