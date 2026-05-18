*[ruzstd](../../index.md) / [bit_io](../index.md) / [bit_writer](index.md)*

---

# Module `bit_writer`

Use [BitWriter] to write an arbitrary amount of bits into a buffer.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BitWriter`](#bitwriter) | struct | An interface for writing an arbitrary number of bits into a buffer. |

## Structs

### `BitWriter<V: AsMut<alloc::vec::Vec<u8>>>`

```rust
struct BitWriter<V: AsMut<alloc::vec::Vec<u8>>> {
    output: V,
    partial: u64,
    bits_in_partial: usize,
    bit_idx: usize,
}
```

An interface for writing an arbitrary number of bits into a buffer. Write new bits into the buffer with `write_bits`, and
obtain the output using `dump`.

#### Fields

- **`output`**: `V`

  The buffer that's filled with bits

- **`partial`**: `u64`

  holds a partially filled byte which gets put in outpu when it's fill with a write_bits call

- **`bit_idx`**: `usize`

  The index pointing to the next unoccupied bit. Effectively just
  the number of bits that have been written into the buffer so far.

#### Implementations

- <span id="bitwriter-new"></span>`fn new() -> Self`

  Initialize a new writer.

#### Trait Implementations

##### `impl<V: fmt::Debug + AsMut<alloc::vec::Vec<u8>>> Debug for BitWriter<V>`

- <span id="bitwriter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

