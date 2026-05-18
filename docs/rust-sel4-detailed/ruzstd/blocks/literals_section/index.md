*[ruzstd](../../index.md) / [blocks](../index.md) / [literals_section](index.md)*

---

# Module `literals_section`

Utilities and representations for the first half of a block, the literals section.
It contains data that is then copied from by the sequences section.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LiteralsSection`](#literalssection) | struct | A compressed block consists of two sections, a literals section, and a sequences section. |
| [`LiteralsSectionType`](#literalssectiontype) | enum | The way which a literal section is encoded. |

## Structs

### `LiteralsSection`

```rust
struct LiteralsSection {
    pub regenerated_size: u32,
    pub compressed_size: Option<u32>,
    pub num_streams: Option<u8>,
    pub ls_type: LiteralsSectionType,
}
```

A compressed block consists of two sections, a literals section, and a sequences section.

This is the first of those two sections. A literal is just any arbitrary data, and it is copied by the sequences section

#### Fields

- **`regenerated_size`**: `u32`

  - If this block is of type [LiteralsSectionType::Raw], then the data is `regenerated_bytes`
    bytes long, and it contains the raw literals data to be used during the second section,
    the sequences section.
  - If this block is of type [LiteralsSectionType::RLE],
    then the literal consists of a single byte repeated `regenerated_size` times.
  - For types [LiteralsSectionType::Compressed] or [LiteralsSectionType::Treeless],
    then this is the size of the decompressed data.

- **`compressed_size`**: `Option<u32>`

  - For types [LiteralsSectionType::Raw] and [LiteralsSectionType::RLE], this value is not present.
  - For types [LiteralsSectionType::Compressed] and [LiteralsSectionType::Treeless], this value will
    be set to the size of the compressed data.

- **`num_streams`**: `Option<u8>`

  This value will be either 1 stream or 4 streams if the literal is of type
  [LiteralsSectionType::Compressed] or [LiteralsSectionType::Treeless], and it
  is not used for RLE or uncompressed literals.

- **`ls_type`**: `LiteralsSectionType`

  The type of the literal section.

#### Implementations

- <span id="literalssection-new"></span>`fn new() -> LiteralsSection` — [`LiteralsSection`](#literalssection)

  Create a new [LiteralsSection].

- <span id="literalssection-header-bytes-needed"></span>`fn header_bytes_needed(&self, first_byte: u8) -> Result<u8, LiteralsSectionParseError>` — [`LiteralsSectionParseError`](../../decoding/errors/index.md#literalssectionparseerror)

  Given the first byte of a header, determine the size of the whole header, from 1 to 5 bytes.

- <span id="literalssection-parse-from-header"></span>`fn parse_from_header(&mut self, raw: &[u8]) -> Result<u8, LiteralsSectionParseError>` — [`LiteralsSectionParseError`](../../decoding/errors/index.md#literalssectionparseerror)

  Parse the header into `self`, and returns the number of bytes read.

- <span id="literalssection-section-type"></span>`fn section_type(raw: u8) -> Result<LiteralsSectionType, LiteralsSectionParseError>` — [`LiteralsSectionType`](#literalssectiontype), [`LiteralsSectionParseError`](../../decoding/errors/index.md#literalssectionparseerror)

  Given the first two bits of a header, determine the type of a header.

#### Trait Implementations

##### `impl Default for LiteralsSection`

- <span id="literalssection-default"></span>`fn default() -> Self`

## Enums

### `LiteralsSectionType`

```rust
enum LiteralsSectionType {
    Raw,
    RLE,
    Compressed,
    Treeless,
}
```

The way which a literal section is encoded.

#### Variants

- **`Raw`**

  Literals are stored uncompressed.

- **`RLE`**

  Literals consist of a single byte value repeated [LiteralsSection::regenerated_size] times.

- **`Compressed`**

  This is a standard Huffman-compressed block, starting with a Huffman tree description.
  In this mode, there are at least *2* different literals represented in the Huffman tree
  description.

- **`Treeless`**

  This is a Huffman-compressed block,
  using the Huffman tree from the previous [LiteralsSectionType::Compressed] block
  in the sequence. If this mode is triggered without any previous Huffman-tables in the
  frame (or dictionary), it should be treated as data corruption.

#### Trait Implementations

##### `impl Display for crate::blocks::literals_section::LiteralsSectionType`

- <span id="crateblocksliterals-sectionliteralssectiontype-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

##### `impl ToString for LiteralsSectionType`

- <span id="literalssectiontype-tostring-to-string"></span>`fn to_string(&self) -> String`

