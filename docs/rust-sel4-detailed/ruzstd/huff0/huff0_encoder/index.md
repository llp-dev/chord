*[ruzstd](../../index.md) / [huff0](../index.md) / [huff0_encoder](index.md)*

---

# Module `huff0_encoder`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HuffmanEncoder`](#huffmanencoder) | struct |  |
| [`HuffmanTable`](#huffmantable) | struct |  |
| [`highest_bit_set`](#highest-bit-set) | fn | Assert that the provided value is greater than zero, and returns index of the first set bit |
| [`distribute_weights`](#distribute-weights) | fn | Distributes weights that add up to a clean power of two |
| [`redistribute_weights`](#redistribute-weights) | fn | Sometimes distribute_weights generates weights that require too many bits to encode This redistributes the weights to have less variance by raising the lower weights while still maintaining the required attributes of the weight distribution |

## Structs

### `HuffmanEncoder<'output, 'table, V: AsMut<alloc::vec::Vec<u8>>>`

```rust
struct HuffmanEncoder<'output, 'table, V: AsMut<alloc::vec::Vec<u8>>> {
    table: &'table HuffmanTable,
    writer: &'output mut crate::bit_io::BitWriter<V>,
}
```

#### Implementations

- <span id="huffmanencoder-new"></span>`fn new<'o, 't>(table: &'t HuffmanTable, writer: &'o mut BitWriter<V>) -> HuffmanEncoder<'o, 't, V>` — [`HuffmanTable`](#huffmantable), [`BitWriter`](../../bit_io/bit_writer/index.md#bitwriter), [`HuffmanEncoder`](#huffmanencoder)

- <span id="huffmanencoder-encode"></span>`fn encode(&mut self, data: &[u8], with_table: bool)`

  Encodes the data using the provided table

  Writes

  * Table description

  * Encoded data

  * Padding bits to fill up last byte

- <span id="huffmanencoder-encode4x"></span>`fn encode4x(&mut self, data: &[u8], with_table: bool)`

  Encodes the data using the provided table in 4 concatenated streams

  Writes

  * Table description

  * Jumptable

  * Encoded data in 4 streams, each padded to fill the last byte

- <span id="huffmanencoder-encode-stream"></span>`fn encode_stream<VV: AsMut<Vec<u8>>>(table: &HuffmanTable, writer: &mut BitWriter<VV>, data: &[u8])` — [`HuffmanTable`](#huffmantable), [`BitWriter`](../../bit_io/bit_writer/index.md#bitwriter)

  Encode one stream and pad it to fill the last byte

- <span id="huffmanencoder-weights"></span>`fn weights(&self) -> Vec<u8>`

- <span id="huffmanencoder-write-table"></span>`fn write_table(&mut self)`

### `HuffmanTable`

```rust
struct HuffmanTable {
    codes: alloc::vec::Vec<(u32, u8)>,
}
```

#### Fields

- **`codes`**: `alloc::vec::Vec<(u32, u8)>`

  Index is the symbol, values are the bitstring in the lower bits of the u32 and the amount of bits in the u8

#### Implementations

- <span id="huffmantable-build-from-data"></span>`fn build_from_data(data: &[u8]) -> Self`

- <span id="huffmantable-build-from-counts"></span>`fn build_from_counts(counts: &[usize]) -> Self`

- <span id="huffmantable-build-from-weights"></span>`fn build_from_weights(weights: &[usize]) -> Self`

- <span id="huffmantable-can-encode"></span>`fn can_encode(&self, other: &Self) -> Option<usize>`

## Functions

### `highest_bit_set`

```rust
fn highest_bit_set(x: usize) -> usize
```

Assert that the provided value is greater than zero, and returns index of the first set bit

### `distribute_weights`

```rust
fn distribute_weights(amount: usize) -> alloc::vec::Vec<usize>
```

Distributes weights that add up to a clean power of two

### `redistribute_weights`

```rust
fn redistribute_weights(weights: &mut [usize], max_num_bits: usize)
```

Sometimes distribute_weights generates weights that require too many bits to encode
This redistributes the weights to have less variance by raising the lower weights while still maintaining the
required attributes of the weight distribution

