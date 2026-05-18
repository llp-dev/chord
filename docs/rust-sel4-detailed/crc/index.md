# Crate `crc`

Rust implementation of CRC.

### Examples
Using a well-known algorithm:
```rust
const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
assert_eq!(X25.checksum(b"123456789"), 0x906e);
```

Using a custom algorithm:
```rust
const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000
};
let crc = crc::Crc::<u16>::new(&CUSTOM_ALG);
let mut digest = crc.digest();
digest.update(b"123456789");
assert_eq!(digest.finalize(), 0xaee7);
```

## Contents

- [Modules](#modules)
  - [`crc128`](#crc128)
  - [`crc16`](#crc16)
  - [`crc32`](#crc32)
  - [`crc64`](#crc64)
  - [`crc8`](#crc8)
  - [`table`](#table)
  - [`util`](#util)
  - [`private`](#private)
  - [`Algorithm`](#algorithm)
- [Structs](#structs)
  - [`Table`](#table)
  - [`Crc`](#crc)
  - [`Digest`](#digest)
- [Traits](#traits)
  - [`Implementation`](#implementation)
- [Type Aliases](#type-aliases)
  - [`NoTable`](#notable)
  - [`DefaultImpl`](#defaultimpl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`crc128`](#crc128) | mod |  |
| [`crc16`](#crc16) | mod |  |
| [`crc32`](#crc32) | mod |  |
| [`crc64`](#crc64) | mod |  |
| [`crc8`](#crc8) | mod |  |
| [`table`](#table) | mod |  |
| [`util`](#util) | mod |  |
| [`private`](#private) | mod |  |
| [`Algorithm`](#algorithm) | mod |  |
| [`Table`](#table) | struct | A table-based implementation of the CRC algorithm, with `L` lanes. |
| [`Crc`](#crc) | struct | Crc instance with a specific width, algorithm, and implementation. |
| [`Digest`](#digest) | struct |  |
| [`Implementation`](#implementation) | trait | A trait for CRC implementations. |
| [`NoTable`](#notable) | type | An implementation of the CRC algorithm with no lookup table. |
| [`DefaultImpl`](#defaultimpl) | type |  |

## Modules

- [`crc128`](crc128/index.md)
- [`crc16`](crc16/index.md)
- [`crc32`](crc32/index.md)
- [`crc64`](crc64/index.md)
- [`crc8`](crc8/index.md)
- [`table`](table/index.md)
- [`util`](util/index.md)
- [`private`](private/index.md)
- [`Algorithm`](Algorithm/index.md)

## Structs

### `Table<const L: usize>`

```rust
struct Table<const L: usize> {
}
```

A table-based implementation of the CRC algorithm, with `L` lanes.
The number of entries in the lookup table is `L * 256`.

#### Trait Implementations

##### `impl Clone for Table<L>`

- <span id="table-clone"></span>`fn clone(&self) -> Table<L>` — [`Table`](#table)

##### `impl Copy for Table<L>`

##### `impl Implementation for Table<L>`

- <span id="table-implementation-type-data"></span>`type Data = [[W; 256]; L]`

##### `impl Sealed for super::Table<0>`

### `Crc<W: Width, I: Implementation>`

```rust
struct Crc<W: Width, I: Implementation> {
    pub algorithm: &'static Algorithm<W>,
    data: <I as >::Data,
}
```

Crc instance with a specific width, algorithm, and implementation.

#### Implementations

- <span id="crc-new"></span>`const fn new(algorithm: &'static Algorithm<u128>) -> Self` — [`Algorithm`](#algorithm)

- <span id="crc-checksum"></span>`const fn checksum(&self, bytes: &[u8]) -> u128`

- <span id="crc-update"></span>`const fn update(&self, crc: u128, bytes: &[u8]) -> u128`

- <span id="crc-digest"></span>`const fn digest(&self) -> Digest<'_, u128, Table<L>>` — [`Digest`](#digest), [`Table`](#table)

- <span id="crc-digest-with-initial"></span>`const fn digest_with_initial(&self, initial: u128) -> Digest<'_, u128, Table<L>>` — [`Digest`](#digest), [`Table`](#table)

  Construct a `Digest` with a given initial value.

  

  This overrides the initial value specified by the algorithm.

  The effects of the algorithm's properties `refin` and `width`

  are applied to the custom initial value.

- <span id="crc-table"></span>`const fn table(&self) -> &<Table<L> as Implementation>::Data` — [`Table`](#table), [`Implementation`](#implementation)

#### Trait Implementations

##### `impl<W: clone::Clone + Width, I: clone::Clone + Implementation> Clone for Crc<W, I>`

- <span id="crc-clone"></span>`fn clone(&self) -> Crc<W, I>` — [`Crc`](#crc)

### `Digest<'a, W: Width, I: Implementation>`

```rust
struct Digest<'a, W: Width, I: Implementation> {
    crc: &'a Crc<W, I>,
    value: W,
}
```

#### Implementations

- <span id="digest-new"></span>`const fn new(crc: &'a Crc<u128, Table<L>>, value: u128) -> Self` — [`Crc`](#crc), [`Table`](#table)

- <span id="digest-update"></span>`const fn update(&mut self, bytes: &[u8])`

- <span id="digest-finalize"></span>`const fn finalize(self) -> u128`

#### Trait Implementations

##### `impl<W: clone::Clone + Width, I: clone::Clone + Implementation> Clone for Digest<'a, W, I>`

- <span id="digest-clone"></span>`fn clone(&self) -> Digest<'a, W, I>` — [`Digest`](#digest)

## Traits

### `Implementation`

```rust
trait Implementation { ... }
```

A trait for CRC implementations.

#### Associated Types

- `type Data`

#### Implementors

- [`Table`](#table)

## Type Aliases

### `NoTable`

```rust
type NoTable = Table<0>;
```

An implementation of the CRC algorithm with no lookup table.

### `DefaultImpl`

```rust
type DefaultImpl = Table<1>;
```

