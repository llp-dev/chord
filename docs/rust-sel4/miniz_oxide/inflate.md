**miniz_oxide > inflate**

# Module: inflate

## Contents

**Modules**

- [`core`](#core) - Streaming decompression functionality.
- [`stream`](#stream) - Extra streaming decompression functionality.

**Structs**

- [`DecompressError`](#decompresserror) - Struct return when decompress_to_vec functions fail.

**Enums**

- [`TINFLStatus`](#tinflstatus) - Return status codes.

**Functions**

- [`decompress_slice_iter_to_slice`](#decompress_slice_iter_to_slice) - Decompress one or more source slices from an iterator into the output slice.
- [`decompress_to_vec`](#decompress_to_vec) - Decompress the deflate-encoded data in `input` to a vector.
- [`decompress_to_vec_with_limit`](#decompress_to_vec_with_limit) - Decompress the deflate-encoded data in `input` to a vector.
- [`decompress_to_vec_zlib`](#decompress_to_vec_zlib) - Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.
- [`decompress_to_vec_zlib_with_limit`](#decompress_to_vec_zlib_with_limit) - Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.

---

## miniz_oxide::inflate::DecompressError

*Struct*

Struct return when decompress_to_vec functions fail.

**Fields:**
- `status: TINFLStatus` - Decompressor status on failure. See [TINFLStatus] for details.
- `output: crate::alloc::vec::Vec<u8>` - The currently decompressed data if any.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## miniz_oxide::inflate::TINFLStatus

*Enum*

Return status codes.

**Variants:**
- `FailedCannotMakeProgress` - More input data was expected, but the caller indicated that there was no more data, so the
- `BadParam` - The output buffer is an invalid size; consider the `flags` parameter.
- `Adler32Mismatch` - The decompression went fine, but the adler32 checksum did not match the one
- `Failed` - Failed to decompress due to invalid data.
- `Done` - Finished decompression without issues.
- `NeedsMoreInput` - The decompressor needs more input data to continue decompressing.
- `HasMoreOutput` - There is still pending data that didn't fit in the output buffer.

**Methods:**

- `fn from_i32(value: i32) -> Option<TINFLStatus>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &TINFLStatus) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> TINFLStatus`



## Module: core

Streaming decompression functionality.



## miniz_oxide::inflate::decompress_slice_iter_to_slice

*Function*

Decompress one or more source slices from an iterator into the output slice.

* On success, returns the number of bytes that were written.
* On failure, returns the failure status code.

This will fail if the output buffer is not large enough, but in that case
the output buffer will still contain the partial decompression.

* `out` the output buffer.
* `it` the iterator of input slices.
* `zlib_header` if the first slice out of the iterator is expected to have a
  Zlib header. Otherwise the slices are assumed to be the deflate data only.
* `ignore_adler32` if the adler32 checksum should be calculated or not.

```rust
fn decompress_slice_iter_to_slice<'out, 'inp, impl Iterator<Item = &'inp [u8]>>(out: &'out  mut [u8], it: impl Trait, zlib_header: bool, ignore_adler32: bool) -> Result<usize, TINFLStatus>
```



## miniz_oxide::inflate::decompress_to_vec

*Function*

Decompress the deflate-encoded data in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

```rust
fn decompress_to_vec(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```



## miniz_oxide::inflate::decompress_to_vec_with_limit

*Function*

Decompress the deflate-encoded data in `input` to a vector.

The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status [`TINFLStatus::HasMoreOutput`] and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] on failure.

```rust
fn decompress_to_vec_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```



## miniz_oxide::inflate::decompress_to_vec_zlib

*Function*

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.

NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error.
It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or
ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure.

```rust
fn decompress_to_vec_zlib(input: &[u8]) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```



## miniz_oxide::inflate::decompress_to_vec_zlib_with_limit

*Function*

Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector.
The vector is grown to at most `max_size` bytes; if the data does not fit in that size,
the error [struct][DecompressError] will contain the status [`TINFLStatus::HasMoreOutput`] and the data that was decompressed on failure.

As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small.
It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead.

Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] on failure.

```rust
fn decompress_to_vec_zlib_with_limit(input: &[u8], max_size: usize) -> Result<crate::alloc::vec::Vec<u8>, DecompressError>
```



## Module: stream

Extra streaming decompression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.



