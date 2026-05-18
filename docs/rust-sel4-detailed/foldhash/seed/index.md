*[foldhash](../index.md) / [seed](index.md)*

---

# Module `seed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`global`](#global) | mod |  |
| [`SharedSeed`](#sharedseed) | struct | A random seed intended to be shared by many different foldhash instances. |
| [`gen_per_hasher_seed`](#gen-per-hasher-seed) | fn |  |
| [`FIXED_GLOBAL_SEED`](#fixed-global-seed) | const | Used for FixedState, and RandomState if atomics for dynamic init are unavailable. |

## Modules

- [`global`](global/index.md)

## Structs

### `SharedSeed`

```rust
struct SharedSeed {
    seeds: [u64; 6],
}
```

A random seed intended to be shared by many different foldhash instances.

This seed is consumed by [`FoldHasher::with_seed`](crate::fast::FoldHasher::with_seed),
and [`SeedableRandomState::with_seed`](crate::fast::SeedableRandomState::with_seed).

#### Implementations

- <span id="sharedseed-global-random"></span>`fn global_random() -> &'static SharedSeed` — [`SharedSeed`](#sharedseed)

  Returns the globally shared randomly initialized [`SharedSeed`](#sharedseed) as used

  by [`RandomState`](crate::fast::RandomState).

- <span id="sharedseed-global-fixed"></span>`const fn global_fixed() -> &'static SharedSeed` — [`SharedSeed`](#sharedseed)

  Returns the globally shared fixed [`SharedSeed`](#sharedseed) as used

  by [`FixedState`](crate::fast::FixedState).

- <span id="sharedseed-from-u64"></span>`const fn from_u64(seed: u64) -> Self`

  Generates a new [`SharedSeed`](#sharedseed) from a single 64-bit seed.

  

  Note that this is somewhat expensive so it is suggested to re-use the

  [`SharedSeed`](#sharedseed) as much as possible, using the per-hasher seed to

  differentiate between hash instances.

#### Trait Implementations

##### `impl Clone for SharedSeed`

- <span id="sharedseed-clone"></span>`fn clone(&self) -> SharedSeed` — [`SharedSeed`](#sharedseed)

##### `impl Debug for SharedSeed`

- <span id="sharedseed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `gen_per_hasher_seed`

```rust
fn gen_per_hasher_seed() -> u64
```

## Constants

### `FIXED_GLOBAL_SEED`
```rust
const FIXED_GLOBAL_SEED: SharedSeed;
```

Used for FixedState, and RandomState if atomics for dynamic init are unavailable.

