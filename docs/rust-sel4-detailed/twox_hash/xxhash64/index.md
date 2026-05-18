*[twox_hash](../index.md) / [xxhash64](index.md)*

---

# Module `xxhash64`

The implementation of XXH64.

## Contents

- [Structs](#structs)
  - [`BufferData`](#bufferdata)
  - [`Buffer`](#buffer)
  - [`Accumulators`](#accumulators)
  - [`Hasher`](#hasher)
  - [`State`](#state)
- [Functions](#functions)
  - [`round`](#round)
- [Type Aliases](#type-aliases)
  - [`Lane`](#lane)
  - [`Lanes`](#lanes)
  - [`Bytes`](#bytes)
- [Constants](#constants)
  - [`PRIME64_1`](#prime64-1)
  - [`PRIME64_2`](#prime64-2)
  - [`PRIME64_3`](#prime64-3)
  - [`PRIME64_4`](#prime64-4)
  - [`PRIME64_5`](#prime64-5)
  - [`BYTES_IN_LANE`](#bytes-in-lane)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BufferData`](#bufferdata) | struct |  |
| [`Buffer`](#buffer) | struct |  |
| [`Accumulators`](#accumulators) | struct |  |
| [`Hasher`](#hasher) | struct | Calculates the 64-bit hash. |
| [`State`](#state) | struct | Constructs [`Hasher`][] for multiple hasher instances. |
| [`round`](#round) | fn |  |
| [`Lane`](#lane) | type |  |
| [`Lanes`](#lanes) | type |  |
| [`Bytes`](#bytes) | type |  |
| [`PRIME64_1`](#prime64-1) | const |  |
| [`PRIME64_2`](#prime64-2) | const |  |
| [`PRIME64_3`](#prime64-3) | const |  |
| [`PRIME64_4`](#prime64-4) | const |  |
| [`PRIME64_5`](#prime64-5) | const |  |
| [`BYTES_IN_LANE`](#bytes-in-lane) | const |  |

## Structs

### `BufferData`

```rust
struct BufferData([u64; 4]);
```

#### Implementations

- <span id="bufferdata-new"></span>`const fn new() -> Self`

- <span id="bufferdata-bytes"></span>`const fn bytes(&self) -> &[u8; 32]`

- <span id="bufferdata-bytes-mut"></span>`fn bytes_mut(&mut self) -> &mut [u8; 32]`

#### Trait Implementations

##### `impl Clone for BufferData`

- <span id="bufferdata-clone"></span>`fn clone(&self) -> BufferData` — [`BufferData`](#bufferdata)

##### `impl Debug for BufferData`

- <span id="bufferdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for BufferData`

- <span id="bufferdata-partialeq-eq"></span>`fn eq(&self, other: &BufferData) -> bool` — [`BufferData`](#bufferdata)

##### `impl StructuralPartialEq for BufferData`

### `Buffer`

```rust
struct Buffer {
    offset: usize,
    data: BufferData,
}
```

#### Implementations

- <span id="buffer-new"></span>`const fn new() -> Self`

- <span id="buffer-extend"></span>`fn extend<'d>(&mut self, data: &'d [u8]) -> (Option<&[u64; 4]>, &'d [u8])`

- <span id="buffer-set"></span>`fn set(&mut self, data: &[u8])`

- <span id="buffer-remaining"></span>`fn remaining(&self) -> &[u8]`

#### Trait Implementations

##### `impl Clone for Buffer`

- <span id="buffer-clone"></span>`fn clone(&self) -> Buffer` — [`Buffer`](#buffer)

##### `impl Debug for Buffer`

- <span id="buffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Buffer`

- <span id="buffer-partialeq-eq"></span>`fn eq(&self, other: &Buffer) -> bool` — [`Buffer`](#buffer)

##### `impl StructuralPartialEq for Buffer`

### `Accumulators`

```rust
struct Accumulators([u64; 4]);
```

#### Implementations

- <span id="accumulators-new"></span>`const fn new(seed: u64) -> Self`

- <span id="accumulators-write"></span>`fn write(&mut self, lanes: [u64; 4])`

- <span id="accumulators-write-many"></span>`fn write_many<'d>(&mut self, data: &'d [u8]) -> &'d [u8]`

- <span id="accumulators-finish"></span>`const fn finish(&self) -> u64`

- <span id="accumulators-merge-accumulator"></span>`const fn merge_accumulator(acc: u64, acc_n: u64) -> u64`

#### Trait Implementations

##### `impl Clone for Accumulators`

- <span id="accumulators-clone"></span>`fn clone(&self) -> Accumulators` — [`Accumulators`](#accumulators)

##### `impl Debug for Accumulators`

- <span id="accumulators-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Accumulators`

- <span id="accumulators-partialeq-eq"></span>`fn eq(&self, other: &Accumulators) -> bool` — [`Accumulators`](#accumulators)

##### `impl StructuralPartialEq for Accumulators`

### `Hasher`

```rust
struct Hasher {
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

- <span id="hasher-finish-with"></span>`fn finish_with(seed: u64, len: u64, accumulators: &Accumulators, remaining: &[u8]) -> u64` — [`Accumulators`](#accumulators)

#### Trait Implementations

##### `impl Clone for Hasher`

- <span id="hasher-clone"></span>`fn clone(&self) -> Hasher` — [`Hasher`](#hasher)

##### `impl Debug for Hasher`

- <span id="hasher-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Hasher`

- <span id="hasher-default"></span>`fn default() -> Self`

##### `impl Hasher for Hasher`

- <span id="hasher-hasher-write"></span>`fn write(&mut self, data: &[u8])`

- <span id="hasher-hasher-finish"></span>`fn finish(&self) -> u64`

##### `impl PartialEq for Hasher`

- <span id="hasher-partialeq-eq"></span>`fn eq(&self, other: &Hasher) -> bool` — [`Hasher`](#hasher)

##### `impl StructuralPartialEq for Hasher`

### `State`

```rust
struct State(u64);
```

Constructs [`Hasher`][] for multiple hasher instances.

#### Implementations

- <span id="state-with-seed"></span>`fn with_seed(seed: u64) -> Self`

  Constructs the hasher with an initial seed.

#### Trait Implementations

##### `impl BuildHasher for State`

- <span id="state-buildhasher-type-hasher"></span>`type Hasher = Hasher`

- <span id="state-buildhasher-build-hasher"></span>`fn build_hasher(&self) -> <Self as >::Hasher`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

## Functions

### `round`

```rust
const fn round(acc: u64, lane: u64) -> u64
```

## Type Aliases

### `Lane`

```rust
type Lane = u64;
```

### `Lanes`

```rust
type Lanes = [u64; 4];
```

### `Bytes`

```rust
type Bytes = [u8; 32];
```

## Constants

### `PRIME64_1`
```rust
const PRIME64_1: u64 = 11_400_714_785_074_694_791u64;
```

### `PRIME64_2`
```rust
const PRIME64_2: u64 = 14_029_467_366_897_019_727u64;
```

### `PRIME64_3`
```rust
const PRIME64_3: u64 = 1_609_587_929_392_839_161u64;
```

### `PRIME64_4`
```rust
const PRIME64_4: u64 = 9_650_029_242_287_828_579u64;
```

### `PRIME64_5`
```rust
const PRIME64_5: u64 = 2_870_177_450_012_600_261u64;
```

### `BYTES_IN_LANE`
```rust
const BYTES_IN_LANE: usize = 32usize;
```

