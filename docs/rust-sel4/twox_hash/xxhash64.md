**twox_hash > xxhash64**

# Module: xxhash64

## Contents

**Structs**

- [`Accumulators`](#accumulators)
- [`Buffer`](#buffer)
- [`BufferData`](#bufferdata)
- [`Hasher`](#hasher) - Calculates the 64-bit hash.
- [`State`](#state) - Constructs [`Hasher`][] for multiple hasher instances.

**Functions**

- [`round`](#round)

**Constants**

- [`BYTES_IN_LANE`](#bytes_in_lane)
- [`PRIME64_1`](#prime64_1)
- [`PRIME64_2`](#prime64_2)
- [`PRIME64_3`](#prime64_3)
- [`PRIME64_4`](#prime64_4)
- [`PRIME64_5`](#prime64_5)

**Type Aliases**

- [`Bytes`](#bytes)
- [`Lane`](#lane)
- [`Lanes`](#lanes)

---

## twox_hash::xxhash64::Accumulators

*Struct*

**Tuple Struct**: `([u64; 4])`

**Methods:**

- `fn new(seed: u64) -> Self`
- `fn write(self: & mut Self, lanes: [u64; 4])`
- `fn write_many<'d>(self: & mut Self, data: &'d [u8]) -> &'d [u8]`
- `fn finish(self: &Self) -> u64`
- `fn merge_accumulator(acc: u64, acc_n: u64) -> u64`

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Accumulators) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Accumulators`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## twox_hash::xxhash64::BYTES_IN_LANE

*Constant*: `usize`



## twox_hash::xxhash64::Buffer

*Struct*

**Fields:**
- `offset: usize`
- `data: BufferData`

**Methods:**

- `fn new() -> Self`
- `fn extend<'d>(self: & mut Self, data: &'d [u8]) -> (Option<&[u64; 4]>, &'d [u8])`
- `fn set(self: & mut Self, data: &[u8])`
- `fn remaining(self: &Self) -> &[u8]`

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Buffer) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Buffer`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## twox_hash::xxhash64::BufferData

*Struct*

**Tuple Struct**: `([u64; 4])`

**Methods:**

- `fn new() -> Self`
- `fn bytes(self: &Self) -> &[u8; 32]`
- `fn bytes_mut(self: & mut Self) -> & mut [u8; 32]`

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &BufferData) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BufferData`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## twox_hash::xxhash64::Bytes

*Type Alias*: `[u8; 32]`



## twox_hash::xxhash64::Hasher

*Struct*

Calculates the 64-bit hash.

**Fields:**
- `seed: u64`
- `accumulators: Accumulators`
- `buffer: Buffer`
- `length: u64`

**Methods:**

- `fn oneshot(seed: u64, data: &[u8]) -> u64` - Hash all data at once. If you can use this function, you may
- `fn with_seed(seed: u64) -> Self` - Constructs the hasher with an initial seed.
- `fn seed(self: &Self) -> u64` - The seed this hasher was created with.
- `fn total_len(self: &Self) -> u64` - The total number of bytes hashed.
- `fn finish_with(seed: u64, len: u64, accumulators: &Accumulators, remaining: &[u8]) -> u64`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Hasher) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Hasher`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hasher**
  - `fn write(self: & mut Self, data: &[u8])`
  - `fn finish(self: &Self) -> u64`



## twox_hash::xxhash64::Lane

*Type Alias*: `u64`



## twox_hash::xxhash64::Lanes

*Type Alias*: `[u64; 4]`



## twox_hash::xxhash64::PRIME64_1

*Constant*: `u64`



## twox_hash::xxhash64::PRIME64_2

*Constant*: `u64`



## twox_hash::xxhash64::PRIME64_3

*Constant*: `u64`



## twox_hash::xxhash64::PRIME64_4

*Constant*: `u64`



## twox_hash::xxhash64::PRIME64_5

*Constant*: `u64`



## twox_hash::xxhash64::State

*Struct*

Constructs [`Hasher`][] for multiple hasher instances.

**Tuple Struct**: `(u64)`

**Methods:**

- `fn with_seed(seed: u64) -> Self` - Constructs the hasher with an initial seed.

**Trait Implementations:**

- **BuildHasher**
  - `fn build_hasher(self: &Self) -> <Self as >::Hasher`
- **Clone**
  - `fn clone(self: &Self) -> State`



## twox_hash::xxhash64::round

*Function*

```rust
fn round(acc: u64, lane: u64) -> u64
```



