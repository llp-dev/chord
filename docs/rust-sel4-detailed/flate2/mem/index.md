*[flate2](../index.md) / [mem](index.md)*

---

# Module `mem`

## Contents

- [Structs](#structs)
  - [`Compress`](#compress)
  - [`Decompress`](#decompress)
  - [`DecompressError`](#decompresserror)
  - [`CompressError`](#compresserror)
- [Enums](#enums)
  - [`FlushCompress`](#flushcompress)
  - [`FlushDecompress`](#flushdecompress)
  - [`DecompressErrorInner`](#decompresserrorinner)
  - [`Status`](#status)
- [Functions](#functions)
  - [`decompress_failed`](#decompress-failed)
  - [`decompress_need_dict`](#decompress-need-dict)
  - [`compress_failed`](#compress-failed)
  - [`write_to_spare_capacity_of_vec`](#write-to-spare-capacity-of-vec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Compress`](#compress) | struct | Raw in-memory compression stream for blocks of data. |
| [`Decompress`](#decompress) | struct | Raw in-memory decompression stream for blocks of data. |
| [`DecompressError`](#decompresserror) | struct | Error returned when a decompression object finds that the input stream of bytes was not a valid input stream of bytes. |
| [`CompressError`](#compresserror) | struct | Error returned when a compression object is used incorrectly or otherwise generates an error. |
| [`FlushCompress`](#flushcompress) | enum | Values which indicate the form of flushing to be used when compressing in-memory data. |
| [`FlushDecompress`](#flushdecompress) | enum | Values which indicate the form of flushing to be used when decompressing in-memory data. |
| [`DecompressErrorInner`](#decompresserrorinner) | enum | The inner state for an error when decompressing |
| [`Status`](#status) | enum | Possible status results of compressing some data or successfully decompressing a block of data. |
| [`decompress_failed`](#decompress-failed) | fn |  |
| [`decompress_need_dict`](#decompress-need-dict) | fn |  |
| [`compress_failed`](#compress-failed) | fn |  |
| [`write_to_spare_capacity_of_vec`](#write-to-spare-capacity-of-vec) | fn | Allows `writer` to write data into the spare capacity of the `output` vector. |

## Structs

### `Compress`

```rust
struct Compress {
    inner: crate::ffi::Deflate,
}
```

Raw in-memory compression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the `Read`/[`Write`]() API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.



#### Implementations

- <span id="compress-new"></span>`fn new(level: Compression, zlib_header: bool) -> Compress` — [`Compression`](../index.md#compression), [`Compress`](#compress)

  Creates a new object ready for compressing data that it's given.

  

  The `level` argument here indicates what level of compression is going

  to be performed, and the `zlib_header` argument indicates whether the

  output data should have a zlib header or not.

- <span id="compress-total-in"></span>`fn total_in(&self) -> u64`

  Returns the total number of input bytes which have been processed by

  this compression object.

- <span id="compress-total-out"></span>`fn total_out(&self) -> u64`

  Returns the total number of output bytes which have been produced by

  this compression object.

- <span id="compress-reset"></span>`fn reset(&mut self)`

  Quickly resets this compressor without having to reallocate anything.

  

  This is equivalent to dropping this object and then creating a new one.

- <span id="compress-compress"></span>`fn compress(&mut self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](#flushcompress), [`Status`](#status), [`CompressError`](#compresserror)

  Compresses the input data into the output, consuming only as much

  input as needed and writing as much output as possible.

  

  The flush option can be any of the available `FlushCompress` parameters.

  

  To learn how much data was consumed or how much output was produced, use

  the `total_in` and `total_out` functions before/after this is called.

- <span id="compress-compress-uninit"></span>`fn compress_uninit(&mut self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](#flushcompress), [`Status`](#status), [`CompressError`](#compresserror)

  Similar to `Self::compress` but accepts uninitialized buffer.

  

  If you want to avoid the overhead of zero initializing the

  buffer and you don't want to use a [`Vec`](../../addr2line/maybe_small/index.md), then please use

  this API.

- <span id="compress-compress-vec"></span>`fn compress_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](#flushcompress), [`Status`](#status), [`CompressError`](#compresserror)

  Compresses the input data into the extra space of the output, consuming

  only as much input as needed and writing as much output as possible.

  

  This function has the same semantics as `compress`, except that the

  length of `vec` is managed by this function. This will not reallocate

  the vector provided or attempt to grow it, so space for the output must

  be reserved in the output vector by the caller before calling this

  function.

#### Trait Implementations

##### `impl Debug for Compress`

- <span id="compress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Ops for crate::Compress`

- <span id="cratecompress-ops-type-error"></span>`type Error = CompressError`

- <span id="cratecompress-ops-type-flush"></span>`type Flush = FlushCompress`

- <span id="cratecompress-ops-total-in"></span>`fn total_in(&self) -> u64`

- <span id="cratecompress-ops-total-out"></span>`fn total_out(&self) -> u64`

- <span id="cratecompress-ops-run"></span>`fn run(&mut self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](#flushcompress), [`Status`](#status), [`CompressError`](#compresserror)

- <span id="cratecompress-ops-run-vec"></span>`fn run_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](#flushcompress), [`Status`](#status), [`CompressError`](#compresserror)

### `Decompress`

```rust
struct Decompress {
    inner: crate::ffi::Inflate,
}
```

Raw in-memory decompression stream for blocks of data.

This type is the building block for the I/O streams in the rest of this
crate. It requires more management than the `Read`/[`Write`]() API but is
maximally flexible in terms of accepting input from any source and being
able to produce output to any memory location.

It is recommended to use the I/O stream adaptors over this type as they're
easier to use.



#### Implementations

- <span id="decompress-new"></span>`fn new(zlib_header: bool) -> Decompress` — [`Decompress`](#decompress)

  Creates a new object ready for decompressing data that it's given.

  

  The `zlib_header` argument indicates whether the input data is expected

  to have a zlib header or not.

- <span id="decompress-total-in"></span>`fn total_in(&self) -> u64`

  Returns the total number of input bytes which have been processed by

  this decompression object.

- <span id="decompress-total-out"></span>`fn total_out(&self) -> u64`

  Returns the total number of output bytes which have been produced by

  this decompression object.

- <span id="decompress-decompress"></span>`fn decompress(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](#flushdecompress), [`Status`](#status), [`DecompressError`](#decompresserror)

  Decompresses the input data into the output, consuming only as much

  input as needed and writing as much output as possible.

  

  The flush option can be any of the available `FlushDecompress` parameters.

  

  If the first call passes `FlushDecompress::Finish` it is assumed that

  the input and output buffers are both sized large enough to decompress

  the entire stream in a single call.

  

  A flush value of `FlushDecompress::Finish` indicates that there are no

  more source bytes available beside what's already in the input buffer,

  and the output buffer is large enough to hold the rest of the

  decompressed data.

  

  To learn how much data was consumed or how much output was produced, use

  the `total_in` and `total_out` functions before/after this is called.

  

  # Errors

  

  If the input data to this instance of `Decompress` is not a valid

  zlib/deflate stream then this function may return an instance of

  `DecompressError` to indicate that the stream of input bytes is corrupted.

- <span id="decompress-decompress-uninit"></span>`fn decompress_uninit(&mut self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](#flushdecompress), [`Status`](#status), [`DecompressError`](#decompresserror)

  Similar to `Self::decompress` but accepts uninitialized buffer

  

  If you want to avoid the overhead of zero initializing the

  buffer and you don't want to use a [`Vec`](../../addr2line/maybe_small/index.md), then please use

  this API.

- <span id="decompress-decompress-vec"></span>`fn decompress_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](#flushdecompress), [`Status`](#status), [`DecompressError`](#decompresserror)

  Decompresses the input data into the extra space in the output vector

  specified by `output`.

  

  This function has the same semantics as `decompress`, except that the

  length of `vec` is managed by this function. This will not reallocate

  the vector provided or attempt to grow it, so space for the output must

  be reserved in the output vector by the caller before calling this

  function.

  

  # Errors

  

  If the input data to this instance of `Decompress` is not a valid

  zlib/deflate stream then this function may return an instance of

  `DecompressError` to indicate that the stream of input bytes is corrupted.

- <span id="decompress-reset"></span>`fn reset(&mut self, zlib_header: bool)`

  Performs the equivalent of replacing this decompression state with a

  freshly allocated copy.

  

  This function may not allocate memory, though, and attempts to reuse any

  previously existing resources.

  

  The argument provided here indicates whether the reset state will

  attempt to decode a zlib header first or not.

#### Trait Implementations

##### `impl Debug for Decompress`

- <span id="decompress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Ops for crate::Decompress`

- <span id="cratedecompress-ops-type-error"></span>`type Error = DecompressError`

- <span id="cratedecompress-ops-type-flush"></span>`type Flush = FlushDecompress`

- <span id="cratedecompress-ops-total-in"></span>`fn total_in(&self) -> u64`

- <span id="cratedecompress-ops-total-out"></span>`fn total_out(&self) -> u64`

- <span id="cratedecompress-ops-run"></span>`fn run(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](#flushdecompress), [`Status`](#status), [`DecompressError`](#decompresserror)

- <span id="cratedecompress-ops-run-vec"></span>`fn run_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](#flushdecompress), [`Status`](#status), [`DecompressError`](#decompresserror)

### `DecompressError`

```rust
struct DecompressError(DecompressErrorInner);
```

Error returned when a decompression object finds that the input stream of
bytes was not a valid input stream of bytes.

#### Implementations

- <span id="decompresserror-needs-dictionary"></span>`fn needs_dictionary(&self) -> Option<u32>`

  Indicates whether decompression failed due to requiring a dictionary.

  

  The resulting integer is the Adler-32 checksum of the dictionary

  required.

#### Trait Implementations

##### `impl Clone for DecompressError`

- <span id="decompresserror-clone"></span>`fn clone(&self) -> DecompressError` — [`DecompressError`](#decompresserror)

##### `impl Debug for DecompressError`

- <span id="decompresserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecompressError`

- <span id="decompresserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for DecompressError`

##### `impl ToString for DecompressError`

- <span id="decompresserror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `CompressError`

```rust
struct CompressError {
    msg: crate::ffi::ErrorMessage,
}
```

Error returned when a compression object is used incorrectly or otherwise
generates an error.

#### Implementations

- <span id="compresserror-message"></span>`fn message(&self) -> Option<&str>`

  Retrieve the implementation's message about why the operation failed, if one exists.

#### Trait Implementations

##### `impl Clone for CompressError`

- <span id="compresserror-clone"></span>`fn clone(&self) -> CompressError` — [`CompressError`](#compresserror)

##### `impl Debug for CompressError`

- <span id="compresserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CompressError`

- <span id="compresserror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for CompressError`

##### `impl ToString for CompressError`

- <span id="compresserror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `FlushCompress`

```rust
enum FlushCompress {
    None,
    Partial,
    Sync,
    Full,
    Finish,
}
```

Values which indicate the form of flushing to be used when compressing
in-memory data.

#### Variants

- **`None`**

  A typical parameter for passing to compression/decompression functions,
  this indicates that the underlying stream to decide how much data to
  accumulate before producing output in order to maximize compression.

- **`Partial`**

  All pending output is flushed to the output buffer, but the output is
  not aligned to a byte boundary.
  
  All input data so far will be available to the decompressor (as with
  `Flush::Sync`). This completes the current deflate block and follows it
  with an empty fixed codes block that is 10 bits long, and it assures
  that enough bytes are output in order for the decompressor to finish the
  block before the empty fixed code block.

- **`Sync`**

  All pending output is flushed to the output buffer and the output is
  aligned on a byte boundary so that the decompressor can get all input
  data available so far.
  
  Flushing may degrade compression for some compression algorithms and so
  it should only be used when necessary. This will complete the current
  deflate block and follow it with an empty stored block.

- **`Full`**

  All output is flushed as with `Flush::Sync` and the compression state is
  reset so decompression can restart from this point if previous
  compressed data has been damaged or if random access is desired.
  
  Using this option too often can seriously degrade compression.

- **`Finish`**

  Pending input is processed and pending output is flushed.
  
  The return value may indicate that the stream is not yet done and more
  data has yet to be processed.

#### Trait Implementations

##### `impl Clone for FlushCompress`

- <span id="flushcompress-clone"></span>`fn clone(&self) -> FlushCompress` — [`FlushCompress`](#flushcompress)

##### `impl Copy for FlushCompress`

##### `impl Debug for FlushCompress`

- <span id="flushcompress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlushCompress`

##### `impl Flush for crate::FlushCompress`

- <span id="crateflushcompress-flush-none"></span>`fn none() -> Self`

- <span id="crateflushcompress-flush-sync"></span>`fn sync() -> Self`

- <span id="crateflushcompress-flush-finish"></span>`fn finish() -> Self`

##### `impl PartialEq for FlushCompress`

- <span id="flushcompress-partialeq-eq"></span>`fn eq(&self, other: &FlushCompress) -> bool` — [`FlushCompress`](#flushcompress)

##### `impl StructuralPartialEq for FlushCompress`

### `FlushDecompress`

```rust
enum FlushDecompress {
    None,
    Sync,
    Finish,
}
```

Values which indicate the form of flushing to be used when
decompressing in-memory data.

#### Variants

- **`None`**

  A typical parameter for passing to compression/decompression functions,
  this indicates that the underlying stream to decide how much data to
  accumulate before producing output in order to maximize compression.

- **`Sync`**

  All pending output is flushed to the output buffer and the output is
  aligned on a byte boundary so that the decompressor can get all input
  data available so far.
  
  Flushing may degrade compression for some compression algorithms and so
  it should only be used when necessary. This will complete the current
  deflate block and follow it with an empty stored block.

- **`Finish`**

  Pending input is processed and pending output is flushed.
  
  The return value may indicate that the stream is not yet done and more
  data has yet to be processed.

#### Trait Implementations

##### `impl Clone for FlushDecompress`

- <span id="flushdecompress-clone"></span>`fn clone(&self) -> FlushDecompress` — [`FlushDecompress`](#flushdecompress)

##### `impl Copy for FlushDecompress`

##### `impl Debug for FlushDecompress`

- <span id="flushdecompress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlushDecompress`

##### `impl Flush for crate::FlushDecompress`

- <span id="crateflushdecompress-flush-none"></span>`fn none() -> Self`

- <span id="crateflushdecompress-flush-sync"></span>`fn sync() -> Self`

- <span id="crateflushdecompress-flush-finish"></span>`fn finish() -> Self`

##### `impl PartialEq for FlushDecompress`

- <span id="flushdecompress-partialeq-eq"></span>`fn eq(&self, other: &FlushDecompress) -> bool` — [`FlushDecompress`](#flushdecompress)

##### `impl StructuralPartialEq for FlushDecompress`

### `DecompressErrorInner`

```rust
enum DecompressErrorInner {
    General {
        msg: crate::ffi::ErrorMessage,
    },
    NeedsDictionary(u32),
}
```

The inner state for an error when decompressing

#### Trait Implementations

##### `impl Clone for DecompressErrorInner`

- <span id="decompresserrorinner-clone"></span>`fn clone(&self) -> DecompressErrorInner` — [`DecompressErrorInner`](#decompresserrorinner)

##### `impl Debug for DecompressErrorInner`

- <span id="decompresserrorinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Status`

```rust
enum Status {
    Ok,
    BufError,
    StreamEnd,
}
```

Possible status results of compressing some data or successfully
decompressing a block of data.

#### Variants

- **`Ok`**

  Indicates success.
  
  Means that more input may be needed but isn't available
  and/or there's more output to be written but the output buffer is full.

- **`BufError`**

  Indicates that forward progress is not possible due to input or output
  buffers being empty.
  
  For compression it means the input buffer needs some more data or the
  output buffer needs to be freed up before trying again.
  
  For decompression this means that more input is needed to continue or
  the output buffer isn't large enough to contain the result. The function
  can be called again after fixing both.

- **`StreamEnd`**

  Indicates that all input has been consumed and all output bytes have
  been written. Decompression/compression should not be called again.
  
  For decompression with zlib streams the adler-32 of the decompressed
  data has also been verified.

#### Trait Implementations

##### `impl Clone for Status`

- <span id="status-clone"></span>`fn clone(&self) -> Status` — [`Status`](#status)

##### `impl Copy for Status`

##### `impl Debug for Status`

- <span id="status-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Status`

##### `impl PartialEq for Status`

- <span id="status-partialeq-eq"></span>`fn eq(&self, other: &Status) -> bool` — [`Status`](#status)

##### `impl StructuralPartialEq for Status`

## Functions

### `decompress_failed`

```rust
fn decompress_failed<T>(msg: crate::ffi::ErrorMessage) -> Result<T, DecompressError>
```

### `decompress_need_dict`

```rust
fn decompress_need_dict<T>(adler: u32) -> Result<T, DecompressError>
```

### `compress_failed`

```rust
fn compress_failed<T>(msg: crate::ffi::ErrorMessage) -> Result<T, CompressError>
```

### `write_to_spare_capacity_of_vec`

```rust
unsafe fn write_to_spare_capacity_of_vec<T>(output: &mut Vec<u8>, writer: impl FnOnce(&mut [std::mem::MaybeUninit<u8>]) -> (usize, T)) -> T
```

Allows `writer` to write data into the spare capacity of the `output` vector.
This will not reallocate the vector provided or attempt to grow it, so space
for the `output` must be reserved by the caller before calling this
function.

`writer` needs to return the number of bytes written (and can also return
another arbitrary return value).

# Safety:

The length returned by the `writer` must be equal to actual number of bytes written
to the uninitialized slice passed in and initialized.

