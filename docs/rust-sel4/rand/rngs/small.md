**rand > rngs > small**

# Module: rngs::small

## Contents

**Structs**

- [`SmallRng`](#smallrng) - A small-state, fast, non-crypto, non-portable PRNG

---

## rand::rngs::small::SmallRng

*Struct*

A small-state, fast, non-crypto, non-portable PRNG

This is the "standard small" RNG, a generator with the following properties:

- Non-[portable]: any future library version may replace the algorithm
  and results may be platform-dependent.
  (For a small portable generator, use the [rand_pcg] or [rand_xoshiro] crate.)
- Non-cryptographic: output is easy to predict (insecure)
- [Quality]: statistically good quality
- Fast: the RNG is fast for both bulk generation and single values, with
  consistent cost of method calls
- Fast initialization
- Small state: little memory usage (current state size is 16-32 bytes
  depending on platform)

The current algorithm is
`Xoshiro256PlusPlus` on 64-bit platforms and `Xoshiro128PlusPlus` on 32-bit
platforms. Both are also implemented by the [rand_xoshiro] crate.

## Seeding (construction)

This generator implements the [`SeedableRng`] trait. All methods are
suitable for seeding, but note that, even with a fixed seed, output is not
[portable]. Some suggestions:

1.  To automatically seed with a unique seed, use [`rand::make_rng()`]:
    ```
    use rand::rngs::SmallRng;
    let mut rng: SmallRng = rand::make_rng();
    # let _ = rand::Rng::next_u32(&mut rng);
    ```
2.  To use a deterministic integral seed, use `seed_from_u64`. This uses a
    hash function internally to yield a (typically) good seed from any
    input.
    ```
    # use rand::{SeedableRng, rngs::SmallRng};
    let rng = SmallRng::seed_from_u64(1);
    # let _: SmallRng = rng;
    ```
3.  To seed deterministically from text or other input, use [`rand_seeder`].

See also [Seeding RNGs] in the book.

## Generation

The generators implements [`Rng`] and thus also [`Rng`][crate::Rng].
See also the [Random Values] chapter in the book.

[portable]: https://rust-random.github.io/book/crate-reprod.html
[Seeding RNGs]: https://rust-random.github.io/book/guide-seeding.html
[Random Values]: https://rust-random.github.io/book/guide-values.html
[Quality]: https://rust-random.github.io/book/guide-rngs.html#quality
[`StdRng`]: crate::rngs::StdRng
[rand_pcg]: https://crates.io/crates/rand_pcg
[rand_xoshiro]: https://crates.io/crates/rand_xoshiro
[`rand_seeder`]: https://docs.rs/rand_seeder/latest/rand_seeder/
[`Rng`]: rand_core::Rng
[`rand::make_rng()`]: crate::make_rng

**Tuple Struct**: `()`

**Traits:** Eq

**Trait Implementations:**

- **SeedableRng**
  - `fn from_seed(seed: <Self as >::Seed) -> Self`
  - `fn seed_from_u64(state: u64) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SmallRng) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SmallRng`
- **TryRng**
  - `fn try_next_u32(self: & mut Self) -> Result<u32, Infallible>`
  - `fn try_next_u64(self: & mut Self) -> Result<u64, Infallible>`
  - `fn try_fill_bytes(self: & mut Self, dest: & mut [u8]) -> Result<(), Infallible>`



