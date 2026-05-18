# Crate `twox_hash`

A Rust implementation of the [xxHash] algorithm.

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]






# Examples

These examples use [`XxHash64`][] but the same ideas can be
used for [`XxHash32`][], [`XxHash3_64`][], or [`XxHash3_128`][].

## Hashing arbitrary data

### When all the data is available at once

```rust
use twox_hash::XxHash64;

let seed = 1234;
let hash = XxHash64::oneshot(seed, b"some bytes");
assert_eq!(0xeab5_5659_a496_d78b, hash);
```

### When the data is streaming

```rust
use std::hash::Hasher as _;
use twox_hash::XxHash64;

let seed = 1234;
let mut hasher = XxHash64::with_seed(seed);
hasher.write(b"some");
hasher.write(b" ");
hasher.write(b"bytes");
let hash = hasher.finish();
assert_eq!(0xeab5_5659_a496_d78b, hash);
```

## In a [`HashMap`][]

### With a default seed

```rust
use std::{collections::HashMap, hash::BuildHasherDefault};
use twox_hash::XxHash64;

let mut hash = HashMap::<_, _, BuildHasherDefault<XxHash64>>::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
```

### With a random seed

```rust
use std::collections::HashMap;
use twox_hash::xxhash64;

let mut hash = HashMap::<_, _, xxhash64::RandomState>::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
```

### With a fixed seed

```rust
use std::collections::HashMap;
use twox_hash::xxhash64;

let mut hash = HashMap::with_hasher(xxhash64::State::with_seed(0xdead_cafe));
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
```

# Feature Flags

| name        | description                                                                                                                   |
|-------------|-------------------------------------------------------------------------------------------------------------------------------|
| xxhash32    | Include the [`XxHash32`][] algorithm                                                                                          |
| xxhash64    | Include the [`XxHash64`][] algorithm                                                                                          |
| xxhash3_64  | Include the [`XxHash3_64`][] algorithm                                                                                        |
| xxhash3_128 | Include the [`XxHash3_128`][] algorithm                                                                                       |
| random      | Create random instances of the hashers                                                                                        |
| serialize   | Serialize and deserialize hasher state with Serde                                                                             |
| std         | Use the Rust standard library. Enable this if you want SIMD support in [`XxHash3_64`][] or [`XxHash3_128`][]                  |
| alloc       | Use the Rust allocator library. Enable this if you want to create [`XxHash3_64`][] or [`XxHash3_128`][]  with dynamic secrets |

# Benchmarks

See benchmarks in the [comparison][] README.

# Contributing

1. Fork it (<https://github.com/shepmaster/twox-hash/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Add a failing test.
4. Add code to pass the test.
5. Commit your changes (`git commit -am 'Add some feature'`)
6. Ensure tests pass.
7. Push to the branch (`git push origin my-new-feature`)
8. Create a new Pull Request






## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`xxhash64`](#xxhash64) | mod | The implementation of XXH64. |
| [`XxHash64`](#xxhash64) | struct |  |
| [`IntoU32`](#intou32) | trait |  |
| [`IntoU64`](#intou64) | trait |  |
| [`IntoU128`](#intou128) | trait |  |

## Modules

- [`xxhash64`](xxhash64/index.md) — The implementation of XXH64.

## Structs

### `XxHash64`

```rust
struct XxHash64 {
    seed: u64,
    accumulators: Accumulators,
    buffer: Buffer,
    length: u64,
}
```

Calculates the 64-bit hash.

#### Implementations

- <span id="hasher-oneshot"></span>`fn oneshot(seed: u64, data: &[u8]) -> u64`

  Hash all data at once. If you can use this function, you may

  see noticable speed gains for certain types of input.

- <span id="hasher-with-seed"></span>`const fn with_seed(seed: u64) -> Self`

  Constructs the hasher with an initial seed.

- <span id="hasher-seed"></span>`const fn seed(&self) -> u64`

  The seed this hasher was created with.

- <span id="hasher-total-len"></span>`const fn total_len(&self) -> u64`

  The total number of bytes hashed.

- <span id="hasher-finish-with"></span>`fn finish_with(seed: u64, len: u64, accumulators: &Accumulators, remaining: &[u8]) -> u64` — [`Accumulators`](xxhash64/index.md#accumulators)

#### Trait Implementations

##### `impl Clone for Hasher`

- <span id="hasher-clone"></span>`fn clone(&self) -> Hasher` — [`Hasher`](xxhash64/index.md#hasher)

##### `impl Debug for Hasher`

- <span id="hasher-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Hasher`

- <span id="hasher-default"></span>`fn default() -> Self`

##### `impl Hasher for Hasher`

- <span id="hasher-hasher-write"></span>`fn write(&mut self, data: &[u8])`

- <span id="hasher-hasher-finish"></span>`fn finish(&self) -> u64`

##### `impl PartialEq for Hasher`

- <span id="hasher-partialeq-eq"></span>`fn eq(&self, other: &Hasher) -> bool` — [`Hasher`](xxhash64/index.md#hasher)

##### `impl StructuralPartialEq for Hasher`

## Traits

### `IntoU32`

```rust
trait IntoU32 { ... }
```

#### Required Methods

- `fn into_u32(self) -> u32`

#### Implementors

- `u8`

### `IntoU64`

```rust
trait IntoU64 { ... }
```

#### Required Methods

- `fn into_u64(self) -> u64`

#### Implementors

- `u32`
- `u8`
- `usize`

### `IntoU128`

```rust
trait IntoU128 { ... }
```

#### Required Methods

- `fn into_u128(self) -> u128`

#### Implementors

- `u64`

