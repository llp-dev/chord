**rand > rngs > xoshiro128plusplus**

# Module: rngs::xoshiro128plusplus

## Contents

**Structs**

- [`Xoshiro128PlusPlus`](#xoshiro128plusplus) - A xoshiro128++ random number generator.

---

## rand::rngs::xoshiro128plusplus::Xoshiro128PlusPlus

*Struct*

A xoshiro128++ random number generator.

The xoshiro128++ algorithm is not suitable for cryptographic purposes, but
is very fast and has excellent statistical properties.

The algorithm used here is translated from [the `xoshiro128plusplus.c`
reference source code](http://xoshiro.di.unimi.it/xoshiro128plusplus.c) by
David Blackman and Sebastiano Vigna.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Xoshiro128PlusPlus) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryRng**
  - `fn try_next_u32(self: & mut Self) -> Result<u32, Infallible>`
  - `fn try_next_u64(self: & mut Self) -> Result<u64, Infallible>`
  - `fn try_fill_bytes(self: & mut Self, dst: & mut [u8]) -> Result<(), Infallible>`
- **SeedableRng**
  - `fn from_seed(seed: [u8; 16]) -> Xoshiro128PlusPlus` - Create a new `Xoshiro128PlusPlus`.  If `seed` is entirely 0, it will be
  - `fn seed_from_u64(state: u64) -> Self` - Create a new `Xoshiro128PlusPlus` from a `u64` seed.
- **Clone**
  - `fn clone(self: &Self) -> Xoshiro128PlusPlus`



