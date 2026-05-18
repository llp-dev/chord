**foldhash > seed**

# Module: seed

## Contents

**Structs**

- [`SharedSeed`](#sharedseed) - A random seed intended to be shared by many different foldhash instances.

---

## foldhash::seed::SharedSeed

*Struct*

A random seed intended to be shared by many different foldhash instances.

This seed is consumed by [`FoldHasher::with_seed`](crate::fast::FoldHasher::with_seed),
and [`SeedableRandomState::with_seed`](crate::fast::SeedableRandomState::with_seed).

**Methods:**

- `fn global_random() -> &'static SharedSeed` - Returns the globally shared randomly initialized [`SharedSeed`] as used
- `fn global_fixed() -> &'static SharedSeed` - Returns the globally shared fixed [`SharedSeed`] as used
- `fn from_u64(seed: u64) -> Self` - Generates a new [`SharedSeed`] from a single 64-bit seed.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SharedSeed`



