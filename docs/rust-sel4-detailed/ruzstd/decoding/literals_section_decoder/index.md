*[ruzstd](../../index.md) / [decoding](../index.md) / [literals_section_decoder](index.md)*

---

# Module `literals_section_decoder`

This module contains the decompress_literals function, used to take a
parsed literals header and a source and decompress it.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decode_literals`](#decode-literals) | fn | Decode and decompress the provided literals section into `target`, returning the number of bytes read. |
| [`decompress_literals`](#decompress-literals) | fn | Decompress the provided literals section and source into the provided `target`. |

## Functions

### `decode_literals`

```rust
fn decode_literals(section: &super::super::blocks::literals_section::LiteralsSection, scratch: &mut super::scratch::HuffmanScratch, source: &[u8], target: &mut alloc::vec::Vec<u8>) -> Result<u32, crate::decoding::errors::DecompressLiteralsError>
```

Decode and decompress the provided literals section into `target`, returning the number of bytes read.

### `decompress_literals`

```rust
fn decompress_literals(section: &super::super::blocks::literals_section::LiteralsSection, scratch: &mut super::scratch::HuffmanScratch, source: &[u8], target: &mut alloc::vec::Vec<u8>) -> Result<u32, crate::decoding::errors::DecompressLiteralsError>
```

Decompress the provided literals section and source into the provided `target`.
This function is used when the literals section is `Compressed` or `Treeless`

Returns the number of bytes read.

