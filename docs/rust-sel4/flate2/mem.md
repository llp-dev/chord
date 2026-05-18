**flate2 > mem**

# Module: mem

## Contents

**Structs**

- [`Compress`](#compress) - Raw in-memory compression stream for blocks of data.
- [`CompressError`](#compresserror) - Error returned when a compression object is used incorrectly or otherwise
- [`Decompress`](#decompress) - Raw in-memory decompression stream for blocks of data.
- [`DecompressError`](#decompresserror) - Error returned when a decompression object finds that the input stream of

**Enums**

- [`FlushCompress`](#flushcompress) - Values which indicate the form of flushing to be used when compressing
- [`FlushDecompress`](#flushdecompress) - Values which indicate the form of flushing to be used when
- [`Status`](#status) - Possible status results of compressing some data or successfully

---

## flate2::mem::Compress

*Struct*

Raw in-memory compression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the [`Read`]/[`Write`] API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

**Methods:**

- `fn new(level: Compression, zlib_header: bool) -> Compress` - Creates a new object ready for compressing data that it's given.
- `fn total_in(self: &Self) -> u64` - Returns the total number of input bytes which have been processed by
- `fn total_out(self: &Self) -> u64` - Returns the total number of output bytes which have been produced by
- `fn reset(self: & mut Self)` - Quickly resets this compressor without having to reallocate anything.
- `fn compress(self: & mut Self, input: &[u8], output: & mut [u8], flush: FlushCompress) -> Result<Status, CompressError>` - Compresses the input data into the output, consuming only as much
- `fn compress_uninit(self: & mut Self, input: &[u8], output: & mut [MaybeUninit<u8>], flush: FlushCompress) -> Result<Status, CompressError>` - Similar to [`Self::compress`] but accepts uninitialized buffer.
- `fn compress_vec(self: & mut Self, input: &[u8], output: & mut Vec<u8>, flush: FlushCompress) -> Result<Status, CompressError>` - Compresses the input data into the extra space of the output, consuming

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::mem::CompressError

*Struct*

Error returned when a compression object is used incorrectly or otherwise
generates an error.

**Methods:**

- `fn message(self: &Self) -> Option<&str>` - Retrieve the implementation's message about why the operation failed, if one exists.

**Traits:** Error

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CompressError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::mem::Decompress

*Struct*

Raw in-memory decompression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the [`Read`]/[`Write`] API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

**Methods:**

- `fn new(zlib_header: bool) -> Decompress` - Creates a new object ready for decompressing data that it's given.
- `fn total_in(self: &Self) -> u64` - Returns the total number of input bytes which have been processed by
- `fn total_out(self: &Self) -> u64` - Returns the total number of output bytes which have been produced by
- `fn decompress(self: & mut Self, input: &[u8], output: & mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>` - Decompresses the input data into the output, consuming only as much
- `fn decompress_uninit(self: & mut Self, input: &[u8], output: & mut [MaybeUninit<u8>], flush: FlushDecompress) -> Result<Status, DecompressError>` - Similar to [`Self::decompress`] but accepts uninitialized buffer
- `fn decompress_vec(self: & mut Self, input: &[u8], output: & mut Vec<u8>, flush: FlushDecompress) -> Result<Status, DecompressError>` - Decompresses the input data into the extra space in the output vector
- `fn reset(self: & mut Self, zlib_header: bool)` - Performs the equivalent of replacing this decompression state with a

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::mem::DecompressError

*Struct*

Error returned when a decompression object finds that the input stream of
bytes was not a valid input stream of bytes.

**Tuple Struct**: `()`

**Methods:**

- `fn message(self: &Self) -> Option<&str>` - Retrieve the implementation's message about why the operation failed, if one exists.
- `fn needs_dictionary(self: &Self) -> Option<u32>` - Indicates whether decompression failed due to requiring a dictionary.

**Traits:** Error

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DecompressError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## flate2::mem::FlushCompress

*Enum*

Values which indicate the form of flushing to be used when compressing
in-memory data.

**Variants:**
- `None` - A typical parameter for passing to compression/decompression functions,
- `Partial` - All pending output is flushed to the output buffer, but the output is
- `Sync` - All pending output is flushed to the output buffer and the output is
- `Full` - All output is flushed as with `Flush::Sync` and the compression state is
- `Finish` - Pending input is processed and pending output is flushed.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FlushCompress`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FlushCompress) -> bool`



## flate2::mem::FlushDecompress

*Enum*

Values which indicate the form of flushing to be used when
decompressing in-memory data.

**Variants:**
- `None` - A typical parameter for passing to compression/decompression functions,
- `Sync` - All pending output is flushed to the output buffer and the output is
- `Finish` - Pending input is processed and pending output is flushed.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FlushDecompress`
- **PartialEq**
  - `fn eq(self: &Self, other: &FlushDecompress) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## flate2::mem::Status

*Enum*

Possible status results of compressing some data or successfully
decompressing a block of data.

**Variants:**
- `Ok` - Indicates success.
- `BufError` - Indicates that forward progress is not possible due to input or output
- `StreamEnd` - Indicates that all input has been consumed and all output bytes have

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Status`
- **PartialEq**
  - `fn eq(self: &Self, other: &Status) -> bool`



