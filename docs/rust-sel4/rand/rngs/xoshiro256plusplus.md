**rand > rngs > xoshiro256plusplus**

# Module: rngs::xoshiro256plusplus

## Contents

**Structs**

- [`Xoshiro256PlusPlus`](#xoshiro256plusplus) - A xoshiro256++ random number generator.

---

## rand::rngs::xoshiro256plusplus::Xoshiro256PlusPlus

*Struct*

A xoshiro256++ random number generator.

The xoshiro256++ algorithm is not suitable for cryptographic purposes, but
is very fast and has excellent statistical properties.

The algorithm used here is translated from [the `xoshiro256plusplus.c`
reference source code](http://xoshiro.di.unimi.it/xoshiro256plusplus.c) by
David Blackman and Sebastiano Vigna.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Xoshiro256PlusPlus) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryRng**
  - `fn try_next_u32(self: & mut Self) -> Result<u32, Infallible>`
  - `fn try_next_u64(self: & mut Self) -> Result<u64, Infallible>`
  - `fn try_fill_bytes(self: & mut Self, dst: & mut [u8]) -> Result<(), Infallible>`
- **SeedableRng**
  - `fn from_seed(seed: [u8; 32]) -> Xoshiro256PlusPlus` - Create a new `Xoshiro256PlusPlus`.  If `seed` is entirely 0, it will be
  - `fn seed_from_u64(state: u64) -> Self` - Create a new `Xoshiro256PlusPlus` from a `u64` seed.
- **Clone**
  - `fn clone(self: &Self) -> Xoshiro256PlusPlus`



