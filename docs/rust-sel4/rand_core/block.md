**rand_core > block**

# Module: block

## Contents

**Structs**

- [`BlockRng`](#blockrng) - RNG functionality for a block [`Generator`]

**Traits**

- [`Generator`](#generator) - A random (block) generator

---

## rand_core::block::BlockRng

*Struct*

RNG functionality for a block [`Generator`]

This type encompasses a [`Generator`] [`core`](Self::core) and a buffer.
It provides optimized implementations of methods required by an [`Rng`].

All values are consumed in-order of generation. No whole words (e.g. `u32`
or `u64`) are discarded, though where a word is partially used (e.g. for a
byte-fill whose length is not a multiple of the word size) the rest of the
word is discarded.

[`Rng`]: crate::Rng

**Generic Parameters:**
- G

**Fields:**
- `core: G` - The *core* part of the RNG, implementing the `generate` function.

**Methods:**

- `fn fill_bytes(self: & mut Self, dest: & mut [u8])` - Fill `dest`
- `fn next_u64_from_u32(self: & mut Self) -> u64` - Generate a `u64` from two `u32` words
- `fn reset_and_skip(self: & mut Self, n: usize)` - Re-generate buffer contents, skipping the first `n` words
- `fn word_offset(self: &Self) -> usize` - Get the number of words consumed since the start of the block
- `fn remaining_results(self: &Self) -> &[W]` - Access the unused part of the results buffer
- `fn next_word(self: & mut Self) -> W` - Generate the next word (e.g. `u32`)
- `fn new(core: G) -> BlockRng<G>` - Create a new `BlockRng` from an existing RNG implementing
- `fn reconstruct(core: G, remaining_results: &[W]) -> Option<Self>` - Reconstruct from a core and a remaining-results buffer.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlockRng<G>`
- **Drop**
  - `fn drop(self: & mut Self)`



## rand_core::block::Generator

*Trait*

A random (block) generator

**Methods:**

- `Output`: The output type.
- `generate`: Generate a new block of `output`.
- `drop`: Destruct the output buffer



