**foldhash > fast**

# Module: fast

## Contents

**Structs**

- [`FixedState`](#fixedstate) - A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed.
- [`FoldHasher`](#foldhasher) - A [`Hasher`] instance implementing foldhash, optimized for speed.
- [`RandomState`](#randomstate) - A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly initialized.
- [`SeedableRandomState`](#seedablerandomstate) - A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly

---

## foldhash::fast::FixedState

*Struct*

A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed.

Not recommended unless you absolutely need determinism.

**Methods:**

- `fn with_seed(per_hasher_seed: u64) -> Self` - Creates a [`FixedState`] with the given per-hasher-seed.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FixedState`
- **BuildHasher**
  - `fn build_hasher(self: &Self) -> FoldHasher<'static>`
- **Default**
  - `fn default() -> Self`



## foldhash::fast::FoldHasher

*Struct*

A [`Hasher`] instance implementing foldhash, optimized for speed.

While you can create one directly with [`FoldHasher::with_seed`], you
most likely want to use [`RandomState`], [`SeedableRandomState`] or
[`FixedState`] to create [`FoldHasher`]s.

**Generic Parameters:**
- 'a

**Methods:**

- `fn with_seed(per_hasher_seed: u64, shared_seed: &'a SharedSeed) -> FoldHasher<'a>` - Initializes this [`FoldHasher`] with the given per-hasher seed and

**Trait Implementations:**

- **Hasher**
  - `fn write(self: & mut Self, bytes: &[u8])`
  - `fn write_u8(self: & mut Self, i: u8)`
  - `fn write_u16(self: & mut Self, i: u16)`
  - `fn write_u32(self: & mut Self, i: u32)`
  - `fn write_u64(self: & mut Self, i: u64)`
  - `fn write_u128(self: & mut Self, i: u128)`
  - `fn write_usize(self: & mut Self, i: usize)`
  - `fn finish(self: &Self) -> u64`
- **Clone**
  - `fn clone(self: &Self) -> FoldHasher<'a>`



## foldhash::fast::RandomState

*Struct*

A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly initialized.

**Trait Implementations:**

- **BuildHasher**
  - `fn build_hasher(self: &Self) -> FoldHasher<'static>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RandomState`



## foldhash::fast::SeedableRandomState

*Struct*

A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly
initialized by default, but can also be initialized with a specific seed.

This can be useful for e.g. testing, but the downside is that this type
has a size of 16 bytes rather than the 8 bytes [`RandomState`] is.

**Methods:**

- `fn random() -> Self` - Generates a random [`SeedableRandomState`], similar to [`RandomState`].
- `fn fixed() -> Self` - Generates a fixed [`SeedableRandomState`], similar to [`FixedState`].
- `fn with_seed(per_hasher_seed: u64, shared_seed: &'static SharedSeed) -> Self` - Generates a [`SeedableRandomState`] with the given per-hasher seed

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SeedableRandomState`
- **BuildHasher**
  - `fn build_hasher(self: &Self) -> FoldHasher<'static>`



