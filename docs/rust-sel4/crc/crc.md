**crc**

# Module: crc

## Contents

**Structs**

- [`Crc`](#crc) - Crc instance with a specific width, algorithm, and implementation.
- [`Digest`](#digest)
- [`Table`](#table) - A table-based implementation of the CRC algorithm, with `L` lanes.

**Traits**

- [`Implementation`](#implementation) - A trait for CRC implementations.

**Type Aliases**

- [`NoTable`](#notable) - An implementation of the CRC algorithm with no lookup table.

---

## crc::Crc

*Struct*

Crc instance with a specific width, algorithm, and implementation.

**Generic Parameters:**
- W
- I

**Fields:**
- `algorithm: &'static Algorithm<W>`

**Methods:**

- `fn new(algorithm: &'static Algorithm<u16>) -> Self`
- `fn checksum(self: &Self, bytes: &[u8]) -> u16`
- `fn digest(self: &Self) -> Digest<u16, Table<L>>`
- `fn digest_with_initial(self: &Self, initial: u16) -> Digest<u16, Table<L>>` - Construct a `Digest` with a given initial value.
- `fn table(self: &Self) -> &<Table<L> as Implementation>::Data`
- `fn new(algorithm: &'static Algorithm<u64>) -> Self`
- `fn checksum(self: &Self, bytes: &[u8]) -> u64`
- `fn digest(self: &Self) -> Digest<u64, Table<L>>`
- `fn digest_with_initial(self: &Self, initial: u64) -> Digest<u64, Table<L>>` - Construct a `Digest` with a given initial value.
- `fn table(self: &Self) -> &<Table<L> as Implementation>::Data`
- `fn new(algorithm: &'static Algorithm<u128>) -> Self`
- `fn checksum(self: &Self, bytes: &[u8]) -> u128`
- `fn digest(self: &Self) -> Digest<u128, Table<L>>`
- `fn digest_with_initial(self: &Self, initial: u128) -> Digest<u128, Table<L>>` - Construct a `Digest` with a given initial value.
- `fn table(self: &Self) -> &<Table<L> as Implementation>::Data`
- `fn new(algorithm: &'static Algorithm<u32>) -> Self`
- `fn checksum(self: &Self, bytes: &[u8]) -> u32`
- `fn digest(self: &Self) -> Digest<u32, Table<L>>`
- `fn digest_with_initial(self: &Self, initial: u32) -> Digest<u32, Table<L>>` - Construct a `Digest` with a given initial value.
- `fn table(self: &Self) -> &<Table<L> as Implementation>::Data`
- `fn new(algorithm: &'static Algorithm<u8>) -> Self`
- `fn checksum(self: &Self, bytes: &[u8]) -> u8`
- `fn digest(self: &Self) -> Digest<u8, Table<L>>`
- `fn digest_with_initial(self: &Self, initial: u8) -> Digest<u8, Table<L>>` - Construct a `Digest` with a given initial value.
- `fn table(self: &Self) -> &<Table<L> as Implementation>::Data`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Crc<W, I>`



## crc::Digest

*Struct*

**Generic Parameters:**
- 'a
- W
- I

**Methods:**

- `fn update(self: & mut Self, bytes: &[u8])`
- `fn finalize(self: Self) -> u128`
- `fn update(self: & mut Self, bytes: &[u8])`
- `fn finalize(self: Self) -> u8`
- `fn update(self: & mut Self, bytes: &[u8])`
- `fn finalize(self: Self) -> u64`
- `fn update(self: & mut Self, bytes: &[u8])`
- `fn finalize(self: Self) -> u32`
- `fn update(self: & mut Self, bytes: &[u8])`
- `fn finalize(self: Self) -> u16`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Digest<'a, W, I>`



## crc::Implementation

*Trait*

A trait for CRC implementations.

**Methods:**

- `Data`: Associated data necessary for the implementation (e.g. lookup tables).



## crc::NoTable

*Type Alias*: `Table<0>`

An implementation of the CRC algorithm with no lookup table.



## crc::Table

*Struct*

A table-based implementation of the CRC algorithm, with `L` lanes.
The number of entries in the lookup table is `L * 256`.

**Generic Parameters:**
- const L

**Traits:** Implementation, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<L>`



