*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [rich](index.md)*

---

# Module `rich`

PE rich header handling

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RichHeaderInfo`](#richheaderinfo) | struct | Parsed information about a Rich Header. |
| [`RichHeaderEntry`](#richheaderentry) | struct | A PE rich header entry after it has been unmasked. |
| [`memmem`](#memmem) | fn | Find the offset of the first occurrence of needle in the data. |

## Structs

### `RichHeaderInfo<'data>`

```rust
struct RichHeaderInfo<'data> {
    pub offset: usize,
    pub length: usize,
    pub xor_key: u32,
    masked_entries: &'data [pe::MaskedRichHeaderEntry],
}
```

Parsed information about a Rich Header.

#### Fields

- **`offset`**: `usize`

  The offset at which the rich header starts.

- **`length`**: `usize`

  The length (in bytes) of the rich header.
  
  This includes the payload, but also the 16-byte start sequence and the
  8-byte final "Rich" and XOR key.

- **`xor_key`**: `u32`

  The XOR key used to mask the rich header.
  
  Unless the file has been tampered with, it should be equal to a checksum
  of the file header.

#### Implementations

- <span id="richheaderinfo-parse"></span>`fn parse<R: ReadRef<'data>>(data: R, nt_header_offset: u64) -> Option<Self>`

  Try to locate a rich header and its entries in the current PE file.

- <span id="richheaderinfo-unmasked-entries"></span>`fn unmasked_entries(&self) -> impl Iterator<Item = RichHeaderEntry> + 'data` — [`RichHeaderEntry`](../index.md#richheaderentry)

  Returns an iterator over the unmasked entries.

#### Trait Implementations

##### `impl Clone for RichHeaderInfo<'data>`

- <span id="richheaderinfo-clone"></span>`fn clone(&self) -> RichHeaderInfo<'data>` — [`RichHeaderInfo`](../index.md#richheaderinfo)

##### `impl Copy for RichHeaderInfo<'data>`

##### `impl Debug for RichHeaderInfo<'data>`

- <span id="richheaderinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RichHeaderEntry`

```rust
struct RichHeaderEntry {
    pub comp_id: u32,
    pub count: u32,
}
```

A PE rich header entry after it has been unmasked.

See [`pe::MaskedRichHeaderEntry`](../../../pe/index.md).

#### Fields

- **`comp_id`**: `u32`

  ID of the component.

- **`count`**: `u32`

  Number of times this component has been used when building this PE.

#### Trait Implementations

##### `impl Clone for RichHeaderEntry`

- <span id="richheaderentry-clone"></span>`fn clone(&self) -> RichHeaderEntry` — [`RichHeaderEntry`](../index.md#richheaderentry)

##### `impl Copy for RichHeaderEntry`

##### `impl Debug for RichHeaderEntry`

- <span id="richheaderentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `memmem`

```rust
fn memmem(data: &[u8], needle: &[u8], align: usize) -> Option<usize>
```

Find the offset of the first occurrence of needle in the data.

The offset must have the given alignment.

