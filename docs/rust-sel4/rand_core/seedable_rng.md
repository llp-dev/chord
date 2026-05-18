**rand_core > seedable_rng**

# Module: seedable_rng

## Contents

**Traits**

- [`SeedableRng`](#seedablerng) - A random number generator that can be explicitly seeded.

---

## rand_core::seedable_rng::SeedableRng

*Trait*

A random number generator that can be explicitly seeded.

This trait encapsulates the low-level functionality common to all
pseudo-random number generators (PRNGs, or algorithmic generators).

A generator implementing `SeedableRng` will usually be deterministic, but
beware that portability and reproducibility of results **is not implied**.
Refer to documentation of the generator, noting that generators named after
a specific algorithm are usually tested for reproducibility against a
reference vector, while `SmallRng` and `StdRng` specifically opt out of
reproducibility guarantees.

**Methods:**

- `Seed`: Seed type, which is restricted to types mutably-dereferenceable as `u8`
- `from_seed`: Create a new PRNG using the given seed.
- `seed_from_u64`: Create a new PRNG using a `u64` seed.
- `from_rng`: Create a new PRNG seeded from an infallible `Rng`.
- `try_from_rng`: Create a new PRNG seeded from a potentially fallible `Rng`.
- `fork`: Fork this PRNG
- `try_fork`: Fork this PRNG



