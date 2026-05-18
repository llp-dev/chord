*[ruzstd](../../index.md) / [bit_io](../index.md) / [bit_reader_reverse](index.md)*

---

# Module `bit_reader_reverse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BitReaderReversed`](#bitreaderreversed) | struct | Zstandard encodes some types of data in a way that the data must be read back to front to decode it properly. |

## Structs

### `BitReaderReversed<'s>`

```rust
struct BitReaderReversed<'s> {
    index: usize,
    bits_consumed: u8,
    extra_bits: usize,
    source: &'s [u8],
    bit_container: u64,
}
```

Zstandard encodes some types of data in a way that the data must be read
back to front to decode it properly. `BitReaderReversed` provides a
convenient interface to do that.

#### Fields

- **`index`**: `usize`

  The index of the last read byte in the source.

- **`bits_consumed`**: `u8`

  How many bits have been consumed from `bit_container`.

- **`extra_bits`**: `usize`

  How many bits have been consumed past the end of the input. Will be zero until all the input
  has been read.

- **`source`**: `&'s [u8]`

  The source data to read from.

- **`bit_container`**: `u64`

  The reader doesn't read directly from the source, it reads bits from here, and the container
  is "refilled" as it's emptied.

#### Implementations

- <span id="bitreaderreversed-bits-remaining"></span>`fn bits_remaining(&self) -> isize`

  How many bits are left to read by the reader.

- <span id="bitreaderreversed-new"></span>`fn new(source: &'s [u8]) -> BitReaderReversed<'s>` — [`BitReaderReversed`](#bitreaderreversed)

- <span id="bitreaderreversed-refill"></span>`fn refill(&mut self)`

  We refill the container in full bytes, shifting the still unread portion to the left, and filling the lower bits with new data

- <span id="bitreaderreversed-get-bits"></span>`fn get_bits(&mut self, n: u8) -> u64`

  Read `n` number of bits from the source. Will read at most 56 bits.

  If there are no more bits to be read from the source zero bits will be returned instead.

- <span id="bitreaderreversed-peek-bits"></span>`fn peek_bits(&mut self, n: u8) -> u64`

  Get the next `n` bits from the source without consuming them.

  Caller is responsible for making sure that `n` many bits have been refilled.

- <span id="bitreaderreversed-peek-bits-triple"></span>`fn peek_bits_triple(&mut self, sum: u8, n1: u8, n2: u8, n3: u8) -> (u64, u64, u64)`

  Get the next `n1` `n2` and `n3` bits from the source without consuming them.

  Caller is responsible for making sure that `sum` many bits have been refilled.

- <span id="bitreaderreversed-consume"></span>`fn consume(&mut self, n: u8)`

  Consume `n` bits from the source.

- <span id="bitreaderreversed-get-bits-triple"></span>`fn get_bits_triple(&mut self, n1: u8, n2: u8, n3: u8) -> (u64, u64, u64)`

  Same as calling get_bits three times but slightly more performant

