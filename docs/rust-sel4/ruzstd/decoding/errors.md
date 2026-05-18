**ruzstd > decoding > errors**

# Module: decoding::errors

## Contents

**Enums**

- [`BlockHeaderReadError`](#blockheaderreaderror)
- [`BlockSizeError`](#blocksizeerror)
- [`BlockTypeError`](#blocktypeerror)
- [`DecodeBlockContentError`](#decodeblockcontenterror)
- [`DecodeBufferError`](#decodebuffererror)
- [`DecodeSequenceError`](#decodesequenceerror)
- [`DecompressBlockError`](#decompressblockerror)
- [`DecompressLiteralsError`](#decompressliteralserror)
- [`DictionaryDecodeError`](#dictionarydecodeerror)
- [`ExecuteSequencesError`](#executesequenceserror)
- [`FSEDecoderError`](#fsedecodererror)
- [`FSETableError`](#fsetableerror)
- [`FrameDecoderError`](#framedecodererror)
- [`FrameDescriptorError`](#framedescriptorerror)
- [`FrameHeaderError`](#frameheadererror)
- [`HuffmanDecoderError`](#huffmandecodererror)
- [`HuffmanTableError`](#huffmantableerror)
- [`LiteralsSectionParseError`](#literalssectionparseerror)
- [`ReadFrameHeaderError`](#readframeheadererror)
- [`SequencesHeaderParseError`](#sequencesheaderparseerror)

---

## ruzstd::decoding::errors::BlockHeaderReadError

*Enum*

**Variants:**
- `ReadError(crate::io::Error)`
- `FoundReservedBlock`
- `BlockTypeError(BlockTypeError)`
- `BlockSizeError(BlockSizeError)`

**Trait Implementations:**

- **From**
  - `fn from(val: BlockTypeError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> ::core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: BlockSizeError) -> Self`
- **From**
  - `fn from(val: Error) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`



## ruzstd::decoding::errors::BlockSizeError

*Enum*

**Variants:**
- `BlockSizeTooLarge{ size: u32 }`

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ruzstd::decoding::errors::BlockTypeError

*Enum*

**Variants:**
- `InvalidBlocktypeNumber{ num: u8 }`

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ruzstd::decoding::errors::DecodeBlockContentError

*Enum*

**Variants:**
- `DecoderStateIsFailed`
- `ExpectedHeaderOfPreviousBlock`
- `ReadError{ step: crate::blocks::block::BlockType, source: crate::io::Error }`
- `DecompressBlockError(DecompressBlockError)`

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`
- **From**
  - `fn from(val: DecompressBlockError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ruzstd::decoding::errors::DecodeBufferError

*Enum*

**Variants:**
- `NotEnoughBytesInDictionary{ got: usize, need: usize }`
- `OffsetTooBig{ offset: usize, buf_len: usize }`

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ruzstd::decoding::errors::DecodeSequenceError

*Enum*

**Variants:**
- `GetBitsError(crate::bit_io::GetBitsError)`
- `FSEDecoderError(FSEDecoderError)`
- `FSETableError(FSETableError)`
- `ExtraPadding{ skipped_bits: i32 }`
- `UnsupportedOffset{ offset_code: u8 }`
- `ZeroOffset`
- `NotEnoughBytesForNumSequences`
- `ExtraBits{ bits_remaining: isize }`
- `MissingCompressionMode`
- `MissingByteForRleLlTable`
- `MissingByteForRleOfTable`
- `MissingByteForRleMlTable`

**Trait Implementations:**

- **From**
  - `fn from(val: FSEDecoderError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: FSETableError) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`



## ruzstd::decoding::errors::DecompressBlockError

*Enum*

**Variants:**
- `BlockContentReadError(crate::io::Error)`
- `MalformedSectionHeader{ expected_len: usize, remaining_bytes: usize }`
- `DecompressLiteralsError(DecompressLiteralsError)`
- `LiteralsSectionParseError(LiteralsSectionParseError)`
- `SequencesHeaderParseError(SequencesHeaderParseError)`
- `DecodeSequenceError(DecodeSequenceError)`
- `ExecuteSequencesError(ExecuteSequencesError)`

**Trait Implementations:**

- **From**
  - `fn from(val: DecodeSequenceError) -> Self`
- **From**
  - `fn from(val: LiteralsSectionParseError) -> Self`
- **From**
  - `fn from(val: Error) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`
- **From**
  - `fn from(val: ExecuteSequencesError) -> Self`
- **From**
  - `fn from(val: SequencesHeaderParseError) -> Self`
- **From**
  - `fn from(val: DecompressLiteralsError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ruzstd::decoding::errors::DecompressLiteralsError

*Enum*

**Variants:**
- `MissingCompressedSize`
- `MissingNumStreams`
- `GetBitsError(crate::bit_io::GetBitsError)`
- `HuffmanTableError(HuffmanTableError)`
- `HuffmanDecoderError(HuffmanDecoderError)`
- `UninitializedHuffmanTable`
- `MissingBytesForJumpHeader{ got: usize }`
- `MissingBytesForLiterals{ got: usize, needed: usize }`
- `ExtraPadding{ skipped_bits: i32 }`
- `BitstreamReadMismatch{ read_til: isize, expected: isize }`
- `DecodedLiteralCountMismatch{ decoded: usize, expected: usize }`

**Trait Implementations:**

- **From**
  - `fn from(val: HuffmanDecoderError) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`
- **From**
  - `fn from(val: HuffmanTableError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ruzstd::decoding::errors::DictionaryDecodeError

*Enum*

**Variants:**
- `BadMagicNum{ got: [u8; 4] }`
- `FSETableError(FSETableError)`
- `HuffmanTableError(HuffmanTableError)`

**Trait Implementations:**

- **From**
  - `fn from(val: HuffmanTableError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: FSETableError) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`



## ruzstd::decoding::errors::ExecuteSequencesError

*Enum*

**Variants:**
- `DecodebufferError(DecodeBufferError)`
- `NotEnoughBytesForSequence{ wanted: usize, have: usize }`
- `ZeroOffset`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: DecodeBufferError) -> Self`



## ruzstd::decoding::errors::FSEDecoderError

*Enum*

**Variants:**
- `GetBitsError(crate::bit_io::GetBitsError)`
- `TableIsUninitialized`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`



## ruzstd::decoding::errors::FSETableError

*Enum*

**Variants:**
- `AccLogIsZero`
- `AccLogTooBig{ got: u8, max: u8 }`
- `GetBitsError(crate::bit_io::GetBitsError)`
- `ProbabilityCounterMismatch{ got: u32, expected_sum: u32, symbol_probabilities: alloc::vec::Vec<i32> }`
- `TooManySymbols{ got: usize }`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`



## ruzstd::decoding::errors::FrameDecoderError

*Enum*

**Variants:**
- `ReadFrameHeaderError(ReadFrameHeaderError)`
- `FrameHeaderError(FrameHeaderError)`
- `WindowSizeTooBig{ requested: u64 }`
- `DictionaryDecodeError(DictionaryDecodeError)`
- `FailedToReadBlockHeader(BlockHeaderReadError)`
- `FailedToReadBlockBody(DecodeBlockContentError)`
- `FailedToReadChecksum(crate::io::Error)`
- `NotYetInitialized`
- `FailedToInitialize(FrameHeaderError)`
- `FailedToDrainDecodebuffer(crate::io::Error)`
- `FailedToSkipFrame`
- `TargetTooSmall`
- `DictNotProvided{ dict_id: u32 }`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> ::core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: BlockHeaderReadError) -> Self`
- **From**
  - `fn from(val: FrameHeaderError) -> Self`
- **From**
  - `fn from(val: ReadFrameHeaderError) -> Self`
- **From**
  - `fn from(val: DictionaryDecodeError) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn StdError>`



## ruzstd::decoding::errors::FrameDescriptorError

*Enum*

**Variants:**
- `InvalidFrameContentSizeFlag{ got: u8 }`

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## ruzstd::decoding::errors::FrameHeaderError

*Enum*

**Variants:**
- `WindowTooBig{ got: u64 }`
- `WindowTooSmall{ got: u64 }`
- `FrameDescriptorError(FrameDescriptorError)`
- `DictIdTooSmall{ got: usize, expected: usize }`
- `MismatchedFrameSize{ got: usize, expected: u8 }`
- `FrameSizeIsZero`
- `InvalidFrameSize{ got: u8 }`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(error: FrameDescriptorError) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn StdError>`



## ruzstd::decoding::errors::HuffmanDecoderError

*Enum*

**Variants:**
- `GetBitsError(crate::bit_io::GetBitsError)`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn StdError>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ruzstd::decoding::errors::HuffmanTableError

*Enum*

**Variants:**
- `GetBitsError(crate::bit_io::GetBitsError)`
- `FSEDecoderError(FSEDecoderError)`
- `FSETableError(FSETableError)`
- `SourceIsEmpty`
- `NotEnoughBytesForWeights{ got_bytes: usize, expected_bytes: u8 }`
- `ExtraPadding{ skipped_bits: i32 }`
- `TooManyWeights{ got: usize }`
- `MissingWeights`
- `LeftoverIsNotAPowerOf2{ got: u32 }`
- `NotEnoughBytesToDecompressWeights{ have: usize, need: usize }`
- `FSETableUsedTooManyBytes{ used: usize, available_bytes: u8 }`
- `NotEnoughBytesInSource{ got: usize, need: usize }`
- `WeightBiggerThanMaxNumBits{ got: u8 }`
- `MaxBitsTooHigh{ got: u8 }`

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn StdError>`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> ::core::fmt::Result`
- **From**
  - `fn from(val: FSETableError) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(val: FSEDecoderError) -> Self`



## ruzstd::decoding::errors::LiteralsSectionParseError

*Enum*

**Variants:**
- `IllegalLiteralSectionType{ got: u8 }`
- `GetBitsError(crate::bit_io::GetBitsError)`
- `NotEnoughBytes{ have: usize, need: u8 }`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn std::error::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ruzstd::decoding::errors::ReadFrameHeaderError

*Enum*

**Variants:**
- `MagicNumberReadError(crate::io::Error)`
- `BadMagicNumber(u32)`
- `FrameDescriptorReadError(crate::io::Error)`
- `InvalidFrameDescriptor(FrameDescriptorError)`
- `WindowDescriptorReadError(crate::io::Error)`
- `DictionaryIdReadError(crate::io::Error)`
- `FrameContentSizeReadError(crate::io::Error)`
- `SkipFrame{ magic_number: u32, length: u32 }`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(error: FrameDescriptorError) -> Self`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn StdError>`



## ruzstd::decoding::errors::SequencesHeaderParseError

*Enum*

**Variants:**
- `NotEnoughBytes{ need_at_least: u8, got: usize }`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



