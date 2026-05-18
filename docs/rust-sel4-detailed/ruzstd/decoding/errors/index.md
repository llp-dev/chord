*[ruzstd](../../index.md) / [decoding](../index.md) / [errors](index.md)*

---

# Module `errors`

Errors that might occur while decoding zstd formatted data

## Contents

- [Enums](#enums)
  - [`FrameDescriptorError`](#framedescriptorerror)
  - [`FrameHeaderError`](#frameheadererror)
  - [`ReadFrameHeaderError`](#readframeheadererror)
  - [`BlockHeaderReadError`](#blockheaderreaderror)
  - [`BlockTypeError`](#blocktypeerror)
  - [`BlockSizeError`](#blocksizeerror)
  - [`DecompressBlockError`](#decompressblockerror)
  - [`DecodeBlockContentError`](#decodeblockcontenterror)
  - [`DecodeBufferError`](#decodebuffererror)
  - [`DictionaryDecodeError`](#dictionarydecodeerror)
  - [`FrameDecoderError`](#framedecodererror)
  - [`DecompressLiteralsError`](#decompressliteralserror)
  - [`ExecuteSequencesError`](#executesequenceserror)
  - [`DecodeSequenceError`](#decodesequenceerror)
  - [`LiteralsSectionParseError`](#literalssectionparseerror)
  - [`SequencesHeaderParseError`](#sequencesheaderparseerror)
  - [`FSETableError`](#fsetableerror)
  - [`FSEDecoderError`](#fsedecodererror)
  - [`HuffmanTableError`](#huffmantableerror)
  - [`HuffmanDecoderError`](#huffmandecodererror)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FrameDescriptorError`](#framedescriptorerror) | enum |  |
| [`FrameHeaderError`](#frameheadererror) | enum |  |
| [`ReadFrameHeaderError`](#readframeheadererror) | enum |  |
| [`BlockHeaderReadError`](#blockheaderreaderror) | enum |  |
| [`BlockTypeError`](#blocktypeerror) | enum |  |
| [`BlockSizeError`](#blocksizeerror) | enum |  |
| [`DecompressBlockError`](#decompressblockerror) | enum |  |
| [`DecodeBlockContentError`](#decodeblockcontenterror) | enum |  |
| [`DecodeBufferError`](#decodebuffererror) | enum |  |
| [`DictionaryDecodeError`](#dictionarydecodeerror) | enum |  |
| [`FrameDecoderError`](#framedecodererror) | enum |  |
| [`DecompressLiteralsError`](#decompressliteralserror) | enum |  |
| [`ExecuteSequencesError`](#executesequenceserror) | enum |  |
| [`DecodeSequenceError`](#decodesequenceerror) | enum |  |
| [`LiteralsSectionParseError`](#literalssectionparseerror) | enum |  |
| [`SequencesHeaderParseError`](#sequencesheaderparseerror) | enum |  |
| [`FSETableError`](#fsetableerror) | enum |  |
| [`FSEDecoderError`](#fsedecodererror) | enum |  |
| [`HuffmanTableError`](#huffmantableerror) | enum |  |
| [`HuffmanDecoderError`](#huffmandecodererror) | enum |  |

## Enums

### `FrameDescriptorError`

```rust
enum FrameDescriptorError {
    InvalidFrameContentSizeFlag {
        got: u8,
    },
}
```

#### Trait Implementations

##### `impl Debug for FrameDescriptorError`

- <span id="framedescriptorerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FrameDescriptorError`

- <span id="framedescriptorerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for FrameDescriptorError`

##### `impl ToString for FrameDescriptorError`

- <span id="framedescriptorerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `FrameHeaderError`

```rust
enum FrameHeaderError {
    WindowTooBig {
        got: u64,
    },
    WindowTooSmall {
        got: u64,
    },
    FrameDescriptorError(FrameDescriptorError),
    DictIdTooSmall {
        got: usize,
        expected: usize,
    },
    MismatchedFrameSize {
        got: usize,
        expected: u8,
    },
    FrameSizeIsZero,
    InvalidFrameSize {
        got: u8,
    },
}
```

#### Trait Implementations

##### `impl Debug for FrameHeaderError`

- <span id="frameheadererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FrameHeaderError`

- <span id="frameheadererror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for FrameHeaderError`

- <span id="frameheadererror-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl ToString for FrameHeaderError`

- <span id="frameheadererror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ReadFrameHeaderError`

```rust
enum ReadFrameHeaderError {
    MagicNumberReadError(crate::io::Error),
    BadMagicNumber(u32),
    FrameDescriptorReadError(crate::io::Error),
    InvalidFrameDescriptor(FrameDescriptorError),
    WindowDescriptorReadError(crate::io::Error),
    DictionaryIdReadError(crate::io::Error),
    FrameContentSizeReadError(crate::io::Error),
    SkipFrame {
        magic_number: u32,
        length: u32,
    },
}
```

#### Trait Implementations

##### `impl Debug for ReadFrameHeaderError`

- <span id="readframeheadererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ReadFrameHeaderError`

- <span id="readframeheadererror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ReadFrameHeaderError`

- <span id="readframeheadererror-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl ToString for ReadFrameHeaderError`

- <span id="readframeheadererror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `BlockHeaderReadError`

```rust
enum BlockHeaderReadError {
    ReadError(crate::io::Error),
    FoundReservedBlock,
    BlockTypeError(BlockTypeError),
    BlockSizeError(BlockSizeError),
}
```

#### Trait Implementations

##### `impl Debug for BlockHeaderReadError`

- <span id="blockheaderreaderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BlockHeaderReadError`

- <span id="blockheaderreaderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Error for BlockHeaderReadError`

- <span id="blockheaderreaderror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for BlockHeaderReadError`

- <span id="blockheaderreaderror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `BlockTypeError`

```rust
enum BlockTypeError {
    InvalidBlocktypeNumber {
        num: u8,
    },
}
```

#### Trait Implementations

##### `impl Debug for BlockTypeError`

- <span id="blocktypeerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BlockTypeError`

- <span id="blocktypeerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BlockTypeError`

##### `impl ToString for BlockTypeError`

- <span id="blocktypeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `BlockSizeError`

```rust
enum BlockSizeError {
    BlockSizeTooLarge {
        size: u32,
    },
}
```

#### Trait Implementations

##### `impl Debug for BlockSizeError`

- <span id="blocksizeerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BlockSizeError`

- <span id="blocksizeerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BlockSizeError`

##### `impl ToString for BlockSizeError`

- <span id="blocksizeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DecompressBlockError`

```rust
enum DecompressBlockError {
    BlockContentReadError(crate::io::Error),
    MalformedSectionHeader {
        expected_len: usize,
        remaining_bytes: usize,
    },
    DecompressLiteralsError(DecompressLiteralsError),
    LiteralsSectionParseError(LiteralsSectionParseError),
    SequencesHeaderParseError(SequencesHeaderParseError),
    DecodeSequenceError(DecodeSequenceError),
    ExecuteSequencesError(ExecuteSequencesError),
}
```

#### Trait Implementations

##### `impl Debug for DecompressBlockError`

- <span id="decompressblockerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecompressBlockError`

- <span id="decompressblockerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DecompressBlockError`

- <span id="decompressblockerror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for DecompressBlockError`

- <span id="decompressblockerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DecodeBlockContentError`

```rust
enum DecodeBlockContentError {
    DecoderStateIsFailed,
    ExpectedHeaderOfPreviousBlock,
    ReadError {
        step: crate::blocks::block::BlockType,
        source: crate::io::Error,
    },
    DecompressBlockError(DecompressBlockError),
}
```

#### Trait Implementations

##### `impl Debug for DecodeBlockContentError`

- <span id="decodeblockcontenterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecodeBlockContentError`

- <span id="decodeblockcontenterror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DecodeBlockContentError`

- <span id="decodeblockcontenterror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for DecodeBlockContentError`

- <span id="decodeblockcontenterror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DecodeBufferError`

```rust
enum DecodeBufferError {
    NotEnoughBytesInDictionary {
        got: usize,
        need: usize,
    },
    OffsetTooBig {
        offset: usize,
        buf_len: usize,
    },
}
```

#### Trait Implementations

##### `impl Debug for DecodeBufferError`

- <span id="decodebuffererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecodeBufferError`

- <span id="decodebuffererror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DecodeBufferError`

##### `impl ToString for DecodeBufferError`

- <span id="decodebuffererror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DictionaryDecodeError`

```rust
enum DictionaryDecodeError {
    BadMagicNum {
        got: [u8; 4],
    },
    FSETableError(FSETableError),
    HuffmanTableError(HuffmanTableError),
}
```

#### Trait Implementations

##### `impl Debug for DictionaryDecodeError`

- <span id="dictionarydecodeerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DictionaryDecodeError`

- <span id="dictionarydecodeerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DictionaryDecodeError`

- <span id="dictionarydecodeerror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for DictionaryDecodeError`

- <span id="dictionarydecodeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `FrameDecoderError`

```rust
enum FrameDecoderError {
    ReadFrameHeaderError(ReadFrameHeaderError),
    FrameHeaderError(FrameHeaderError),
    WindowSizeTooBig {
        requested: u64,
    },
    DictionaryDecodeError(DictionaryDecodeError),
    FailedToReadBlockHeader(BlockHeaderReadError),
    FailedToReadBlockBody(DecodeBlockContentError),
    FailedToReadChecksum(crate::io::Error),
    NotYetInitialized,
    FailedToInitialize(FrameHeaderError),
    FailedToDrainDecodebuffer(crate::io::Error),
    FailedToSkipFrame,
    TargetTooSmall,
    DictNotProvided {
        dict_id: u32,
    },
}
```

#### Trait Implementations

##### `impl Debug for FrameDecoderError`

- <span id="framedecodererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FrameDecoderError`

- <span id="framedecodererror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Error for FrameDecoderError`

- <span id="framedecodererror-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl ToString for FrameDecoderError`

- <span id="framedecodererror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DecompressLiteralsError`

```rust
enum DecompressLiteralsError {
    MissingCompressedSize,
    MissingNumStreams,
    GetBitsError(crate::bit_io::GetBitsError),
    HuffmanTableError(HuffmanTableError),
    HuffmanDecoderError(HuffmanDecoderError),
    UninitializedHuffmanTable,
    MissingBytesForJumpHeader {
        got: usize,
    },
    MissingBytesForLiterals {
        got: usize,
        needed: usize,
    },
    ExtraPadding {
        skipped_bits: i32,
    },
    BitstreamReadMismatch {
        read_til: isize,
        expected: isize,
    },
    DecodedLiteralCountMismatch {
        decoded: usize,
        expected: usize,
    },
}
```

#### Trait Implementations

##### `impl Debug for DecompressLiteralsError`

- <span id="decompressliteralserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecompressLiteralsError`

- <span id="decompressliteralserror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DecompressLiteralsError`

- <span id="decompressliteralserror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for DecompressLiteralsError`

- <span id="decompressliteralserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ExecuteSequencesError`

```rust
enum ExecuteSequencesError {
    DecodebufferError(DecodeBufferError),
    NotEnoughBytesForSequence {
        wanted: usize,
        have: usize,
    },
    ZeroOffset,
}
```

#### Trait Implementations

##### `impl Debug for ExecuteSequencesError`

- <span id="executesequenceserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ExecuteSequencesError`

- <span id="executesequenceserror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for ExecuteSequencesError`

- <span id="executesequenceserror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for ExecuteSequencesError`

- <span id="executesequenceserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `DecodeSequenceError`

```rust
enum DecodeSequenceError {
    GetBitsError(crate::bit_io::GetBitsError),
    FSEDecoderError(FSEDecoderError),
    FSETableError(FSETableError),
    ExtraPadding {
        skipped_bits: i32,
    },
    UnsupportedOffset {
        offset_code: u8,
    },
    ZeroOffset,
    NotEnoughBytesForNumSequences,
    ExtraBits {
        bits_remaining: isize,
    },
    MissingCompressionMode,
    MissingByteForRleLlTable,
    MissingByteForRleOfTable,
    MissingByteForRleMlTable,
}
```

#### Trait Implementations

##### `impl Debug for DecodeSequenceError`

- <span id="decodesequenceerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecodeSequenceError`

- <span id="decodesequenceerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DecodeSequenceError`

- <span id="decodesequenceerror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for DecodeSequenceError`

- <span id="decodesequenceerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `LiteralsSectionParseError`

```rust
enum LiteralsSectionParseError {
    IllegalLiteralSectionType {
        got: u8,
    },
    GetBitsError(crate::bit_io::GetBitsError),
    NotEnoughBytes {
        have: usize,
        need: u8,
    },
}
```

#### Trait Implementations

##### `impl Debug for LiteralsSectionParseError`

- <span id="literalssectionparseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LiteralsSectionParseError`

- <span id="literalssectionparseerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for LiteralsSectionParseError`

- <span id="literalssectionparseerror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for LiteralsSectionParseError`

- <span id="literalssectionparseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `SequencesHeaderParseError`

```rust
enum SequencesHeaderParseError {
    NotEnoughBytes {
        need_at_least: u8,
        got: usize,
    },
}
```

#### Trait Implementations

##### `impl Debug for SequencesHeaderParseError`

- <span id="sequencesheaderparseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SequencesHeaderParseError`

- <span id="sequencesheaderparseerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for SequencesHeaderParseError`

##### `impl ToString for SequencesHeaderParseError`

- <span id="sequencesheaderparseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `FSETableError`

```rust
enum FSETableError {
    AccLogIsZero,
    AccLogTooBig {
        got: u8,
        max: u8,
    },
    GetBitsError(crate::bit_io::GetBitsError),
    ProbabilityCounterMismatch {
        got: u32,
        expected_sum: u32,
        symbol_probabilities: alloc::vec::Vec<i32>,
    },
    TooManySymbols {
        got: usize,
    },
}
```

#### Trait Implementations

##### `impl Debug for FSETableError`

- <span id="fsetableerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FSETableError`

- <span id="fsetableerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for FSETableError`

- <span id="fsetableerror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for FSETableError`

- <span id="fsetableerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `FSEDecoderError`

```rust
enum FSEDecoderError {
    GetBitsError(crate::bit_io::GetBitsError),
    TableIsUninitialized,
}
```

#### Trait Implementations

##### `impl Debug for FSEDecoderError`

- <span id="fsedecodererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FSEDecoderError`

- <span id="fsedecodererror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for FSEDecoderError`

- <span id="fsedecodererror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for FSEDecoderError`

- <span id="fsedecodererror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `HuffmanTableError`

```rust
enum HuffmanTableError {
    GetBitsError(crate::bit_io::GetBitsError),
    FSEDecoderError(FSEDecoderError),
    FSETableError(FSETableError),
    SourceIsEmpty,
    NotEnoughBytesForWeights {
        got_bytes: usize,
        expected_bytes: u8,
    },
    ExtraPadding {
        skipped_bits: i32,
    },
    TooManyWeights {
        got: usize,
    },
    MissingWeights,
    LeftoverIsNotAPowerOf2 {
        got: u32,
    },
    NotEnoughBytesToDecompressWeights {
        have: usize,
        need: usize,
    },
    FSETableUsedTooManyBytes {
        used: usize,
        available_bytes: u8,
    },
    NotEnoughBytesInSource {
        got: usize,
        need: usize,
    },
    WeightBiggerThanMaxNumBits {
        got: u8,
    },
    MaxBitsTooHigh {
        got: u8,
    },
}
```

#### Trait Implementations

##### `impl Debug for HuffmanTableError`

- <span id="huffmantableerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HuffmanTableError`

- <span id="huffmantableerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Error for HuffmanTableError`

- <span id="huffmantableerror-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl ToString for HuffmanTableError`

- <span id="huffmantableerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `HuffmanDecoderError`

```rust
enum HuffmanDecoderError {
    GetBitsError(crate::bit_io::GetBitsError),
}
```

#### Trait Implementations

##### `impl Debug for HuffmanDecoderError`

- <span id="huffmandecodererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HuffmanDecoderError`

- <span id="huffmandecodererror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for HuffmanDecoderError`

- <span id="huffmandecodererror-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl ToString for HuffmanDecoderError`

- <span id="huffmandecodererror-tostring-to-string"></span>`fn to_string(&self) -> String`

