*[ruzstd](../../index.md) / [bit_io](../index.md) / [bit_reader](index.md)*

---

# Module `bit_reader`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BitReader`](#bitreader) | struct | Wraps a slice and enables reading arbitrary amounts of bits from that slice. |
| [`GetBitsError`](#getbitserror) | enum |  |

## Structs

### `BitReader<'s>`

```rust
struct BitReader<'s> {
    idx: usize,
    source: &'s [u8],
}
```

Wraps a slice and enables reading arbitrary amounts of bits
from that slice.

#### Implementations

- <span id="bitreader-new"></span>`fn new(source: &'s [u8]) -> BitReader<'s>` — [`BitReader`](#bitreader)

- <span id="bitreader-bits-left"></span>`fn bits_left(&self) -> usize`

- <span id="bitreader-bits-read"></span>`fn bits_read(&self) -> usize`

- <span id="bitreader-return-bits"></span>`fn return_bits(&mut self, n: usize)`

- <span id="bitreader-get-bits"></span>`fn get_bits(&mut self, n: usize) -> Result<u64, GetBitsError>` — [`GetBitsError`](#getbitserror)

## Enums

### `GetBitsError`

```rust
enum GetBitsError {
    TooManyBits {
        num_requested_bits: usize,
        limit: u8,
    },
    NotEnoughRemainingBits {
        requested: usize,
        remaining: usize,
    },
}
```

#### Trait Implementations

##### `impl Debug for GetBitsError`

- <span id="getbitserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for GetBitsError`

- <span id="getbitserror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for GetBitsError`

##### `impl ToString for GetBitsError`

- <span id="getbitserror-tostring-to-string"></span>`fn to_string(&self) -> String`

