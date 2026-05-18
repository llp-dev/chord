*[miniz_oxide](../index.md) / [inflate](index.md)*

---

# Module `inflate`

This module contains functionality for decompression.

## Contents

- [Modules](#modules)
  - [`core`](#core)
  - [`output_buffer`](#output-buffer)
  - [`stream`](#stream)
- [Structs](#structs)
  - [`DecompressError`](#decompresserror)
- [Enums](#enums)
  - [`TINFLStatus`](#tinflstatus)
- [Functions](#functions)
  - [`decompress_error`](#decompress-error)
  - [`decompress_to_vec`](#decompress-to-vec)
  - [`decompress_to_vec_zlib`](#decompress-to-vec-zlib)
  - [`decompress_to_vec_with_limit`](#decompress-to-vec-with-limit)
  - [`decompress_to_vec_zlib_with_limit`](#decompress-to-vec-zlib-with-limit)
  - [`decompress_to_vec_inner`](#decompress-to-vec-inner)
  - [`decompress_slice_iter_to_slice`](#decompress-slice-iter-to-slice)
- [Constants](#constants)
  - [`TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS`](#tinfl-status-failed-cannot-make-progress)
  - [`TINFL_STATUS_BAD_PARAM`](#tinfl-status-bad-param)
  - [`TINFL_STATUS_ADLER32_MISMATCH`](#tinfl-status-adler32-mismatch)
  - [`TINFL_STATUS_FAILED`](#tinfl-status-failed)
  - [`TINFL_STATUS_DONE`](#tinfl-status-done)
  - [`TINFL_STATUS_NEEDS_MORE_INPUT`](#tinfl-status-needs-more-input)
  - [`TINFL_STATUS_HAS_MORE_OUTPUT`](#tinfl-status-has-more-output)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`core`](#core) | mod | Core decompression functionality. |
| [`output_buffer`](#output-buffer) | mod |  |
| [`stream`](#stream) | mod | Extra streaming decompression functionality. |
| [`DecompressError`](#decompresserror) | struct | Struct return when decompress_to_vec functions fail. |
| [`TINFLStatus`](#tinflstatus) | enum | Return status codes. |
| [`decompress_error`](#decompress-error) | fn |  |
| [`decompress_to_vec`](#decompress-to-vec) | fn | Decompress the deflate-encoded data in `input` to a vector. |
| [`decompress_to_vec_zlib`](#decompress-to-vec-zlib) | fn | Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector. |
| [`decompress_to_vec_with_limit`](#decompress-to-vec-with-limit) | fn | Decompress the deflate-encoded data in `input` to a vector. |
| [`decompress_to_vec_zlib_with_limit`](#decompress-to-vec-zlib-with-limit) | fn | Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector. |
| [`decompress_to_vec_inner`](#decompress-to-vec-inner) | fn | Backend of various to-[`Vec`] decompressions. |
| [`decompress_slice_iter_to_slice`](#decompress-slice-iter-to-slice) | fn | Decompress one or more source slices from an iterator into the output slice. |
| [`TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS`](#tinfl-status-failed-cannot-make-progress) | const |  |
| [`TINFL_STATUS_BAD_PARAM`](#tinfl-status-bad-param) | const |  |
| [`TINFL_STATUS_ADLER32_MISMATCH`](#tinfl-status-adler32-mismatch) | const |  |
| [`TINFL_STATUS_FAILED`](#tinfl-status-failed) | const |  |
| [`TINFL_STATUS_DONE`](#tinfl-status-done) | const |  |
| [`TINFL_STATUS_NEEDS_MORE_INPUT`](#tinfl-status-needs-more-input) | const |  |
| [`TINFL_STATUS_HAS_MORE_OUTPUT`](#tinfl-status-has-more-output) | const |  |

## Modules

- [`core`](core/index.md) — Core decompression functionality.
- [`output_buffer`](output_buffer/index.md)
- [`stream`](stream/index.md) — Extra streaming decompression functionality.

## Structs

### `DecompressError`

```rust
struct DecompressError {
    pub status: TINFLStatus,
    pub output: crate::alloc::vec::Vec<u8>,
}
```

Struct return when decompress_to_vec functions fail.

#### Fields

- **`status`**: `TINFLStatus`

  Decompressor status on failure. See [TINFLStatus] for details.

- **`output`**: `crate::alloc::vec::Vec<u8>`

  The currently decompressed data if any.

#### Trait Implementations

##### `impl Debug for DecompressError`

- <span id="decompresserror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecompressError`

- <span id="decompresserror-display-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl ToString for DecompressError`

- <span id="decompresserror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `TINFLStatus`

```rust
enum TINFLStatus {
    FailedCannotMakeProgress,
    BadParam,
    Adler32Mismatch,
    Failed,
    Done,
    NeedsMoreInput,
    HasMoreOutput,
}
```

Return status codes.

#### Variants

- **`FailedCannotMakeProgress`**

  More input data was expected, but the caller indicated that there was no more data, so the
  input stream is likely truncated.
  
  This can't happen if you have provided the
  `TINFL_FLAG_HAS_MORE_INPUT` flag to the
  decompression.  By setting that flag, you indicate more input exists but is not provided,
  and so reaching the end of the input data without finding the end of the compressed stream
  would instead return a `NeedsMoreInput` status.

- **`BadParam`**

  The output buffer is an invalid size; consider the `flags` parameter.

- **`Adler32Mismatch`**

  The decompression went fine, but the adler32 checksum did not match the one
  provided in the header.

- **`Failed`**

  Failed to decompress due to invalid data.

- **`Done`**

  Finished decompression without issues.
  
  This indicates the end of the compressed stream has been reached.

- **`NeedsMoreInput`**

  The decompressor needs more input data to continue decompressing.
  
  This occurs when there's no more consumable input, but the end of the stream hasn't been
  reached, and you have supplied the
  `TINFL_FLAG_HAS_MORE_INPUT` flag to the
  decompressor.  Had you not supplied that flag (which would mean you were asserting that you
  believed all the data was available) you would have gotten a
  `FailedCannotMakeProcess` instead.

- **`HasMoreOutput`**

  There is still pending data that didn't fit in the output buffer.

#### Implementations

- <span id="tinflstatus-from-i32"></span>`fn from_i32(value: i32) -> Option<TINFLStatus>` — [`TINFLStatus`](#tinflstatus)

#### Trait Implementations

##### `impl Clone for TINFLStatus`

- <span id="tinflstatus-clone"></span>`fn clone(&self) -> TINFLStatus` — [`TINFLStatus`](#tinflstatus)

##### `impl Copy for TINFLStatus`

##### `impl Debug for TINFLStatus`

- <span id="tinflstatus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TINFLStatus`

##### `impl Hash for TINFLStatus`

- <span id="tinflstatus-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TINFLStatus`

- <span id="tinflstatus-partialeq-eq"></span>`fn eq(&self, other: &TINFLStatus) -> bool` — [`TINFLStatus`](#tinflstatus)

##### `impl StructuralPartialEq for TINFLStatus`

## Functions

### `decompress_error`

```rust
fn decompress_error(status: TINFLStatus, output: crate::alloc::vec::Vec<u8>) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

### `decompress_to_vec`

```rust
fn decompress_to_vec(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../sel4/error/index.md) containing the [`Vec`](../../addr2line/maybe_small/index.md) of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

### `decompress_to_vec_zlib`

```rust
fn decompress_to_vec_zlib(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../sel4/error/index.md) containing the [`Vec`](../../addr2line/maybe_small/index.md) of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

### `decompress_to_vec_with_limit`

```rust
fn decompress_to_vec_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data in `input` to a vector.

The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status [`TINFLStatus::HasMoreOutput`](../index.md) and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../sel4/error/index.md) containing the [`Vec`](../../addr2line/maybe_small/index.md) of decompressed data on success, and a [struct][DecompressError] on failure.

### `decompress_to_vec_zlib_with_limit`

```rust
fn decompress_to_vec_zlib_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.
The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status [`TINFLStatus::HasMoreOutput`](../index.md) and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`](../../sel4/error/index.md) containing the [`Vec`](../../addr2line/maybe_small/index.md) of decompressed data on success, and a [struct][DecompressError] on failure.

### `decompress_to_vec_inner`

```rust
fn decompress_to_vec_inner(input: &[u8], flags: u32, max_output_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```

Backend of various to-[`Vec`](../../addr2line/maybe_small/index.md) decompressions.

Returns [`Vec`](../../addr2line/maybe_small/index.md) of decompressed data on success and the [error struct][DecompressError] with details on failure.

### `decompress_slice_iter_to_slice`

```rust
fn decompress_slice_iter_to_slice<'out, 'inp>(out: &'out mut [u8], it: impl Iterator<Item = &'inp [u8]>, zlib_header: bool, ignore_adler32: bool) -> Result<usize, TINFLStatus>
```

Decompress one or more source slices from an iterator into the output slice.

* On success, returns the number of bytes that were written.
* On failure, returns the failure status code.

This will fail if the output buffer is not large enough, but in that case
the output buffer will still contain the partial decompression.

* `out` the output buffer.
* `it` the iterator of input slices.
* `zlib_header` if the first slice out of the iterator is expected to have a
  Zlib header. Otherwise the slices are assumed to be the deflate data only.
* `ignore_adler32` if the adler32 checksum should be validated in case of
  of zlib data. (Set this to true if it should be ignored)

# Examples
```rust
use core::iter;
use core::result::Result;
use miniz_oxide::inflate::decompress_slice_iter_to_slice;

fn main() -> Result<(), ()> {
    const ENCODED: [u8; 20] = [
        120, 156, 243, 72, 205, 201, 201, 215, 81, 168, 202, 201, 76, 82, 4, 0, 27, 101, 4, 19,
    ];
    let mut output = [0u8; 20];
    // Using `once` to do the whole buffer in one go. One could also use e.g
    // `slice::chunks` to easily split up a buffer into parts instead.
    let result =
        decompress_slice_iter_to_slice(&mut output, iter::once(ENCODED.as_slice()), true, false);

    if let Ok(bytes) = result {
        if output[..bytes] == b"Hello, zlib!"[..] {
            return Ok(());
        }
    }
    Err(())
}
```

## Constants

### `TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS`
```rust
const TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS: i32 = -4i32;
```

### `TINFL_STATUS_BAD_PARAM`
```rust
const TINFL_STATUS_BAD_PARAM: i32 = -3i32;
```

### `TINFL_STATUS_ADLER32_MISMATCH`
```rust
const TINFL_STATUS_ADLER32_MISMATCH: i32 = -2i32;
```

### `TINFL_STATUS_FAILED`
```rust
const TINFL_STATUS_FAILED: i32 = -1i32;
```

### `TINFL_STATUS_DONE`
```rust
const TINFL_STATUS_DONE: i32 = 0i32;
```

### `TINFL_STATUS_NEEDS_MORE_INPUT`
```rust
const TINFL_STATUS_NEEDS_MORE_INPUT: i32 = 1i32;
```

### `TINFL_STATUS_HAS_MORE_OUTPUT`
```rust
const TINFL_STATUS_HAS_MORE_OUTPUT: i32 = 2i32;
```

