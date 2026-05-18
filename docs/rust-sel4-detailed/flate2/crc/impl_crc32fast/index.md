*[flate2](../../index.md) / [crc](../index.md) / [impl_crc32fast](index.md)*

---

# Module `impl_crc32fast`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Crc`](#crc) | struct | The CRC calculated by a [`CrcReader`]. |

## Structs

### `Crc`

```rust
struct Crc {
    amt: u32,
    hasher: crc32fast::Hasher,
}
```

The CRC calculated by a [`CrcReader`](../index.md).


#### Implementations

- <span id="crc-new"></span>`fn new() -> Self`

  Create a new CRC.

- <span id="crc-sum"></span>`fn sum(&self) -> u32`

  Returns the current crc32 checksum.

- <span id="crc-amount"></span>`fn amount(&self) -> u32`

  The number of bytes that have been used to calculate the CRC.

  This value is only accurate if the amount is lower than 2<sup>32</sup>.

- <span id="crc-update"></span>`fn update(&mut self, data: &[u8])`

  Update the CRC with the bytes in `data`.

- <span id="crc-reset"></span>`fn reset(&mut self)`

  Reset the CRC.

- <span id="crc-combine"></span>`fn combine(&mut self, additional_crc: &Self)`

  Combine the CRC with the CRC for the subsequent block of bytes.

#### Trait Implementations

##### `impl Debug for Crc`

- <span id="crc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Crc`

- <span id="crc-default"></span>`fn default() -> Crc` — [`Crc`](#crc)

