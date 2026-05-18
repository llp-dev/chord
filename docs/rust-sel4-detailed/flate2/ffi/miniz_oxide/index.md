*[flate2](../../index.md) / [ffi](../index.md) / [miniz_oxide](index.md)*

---

# Module `miniz_oxide`

Implementation for `miniz_oxide` rust backend.

## Contents

- [Structs](#structs)
  - [`ErrorMessage`](#errormessage)
  - [`Inflate`](#inflate)
  - [`Deflate`](#deflate)
- [Functions](#functions)
  - [`format_from_bool`](#format-from-bool)
- [Constants](#constants)
  - [`MZ_NO_FLUSH`](#mz-no-flush)
  - [`MZ_PARTIAL_FLUSH`](#mz-partial-flush)
  - [`MZ_SYNC_FLUSH`](#mz-sync-flush)
  - [`MZ_FULL_FLUSH`](#mz-full-flush)
  - [`MZ_FINISH`](#mz-finish)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorMessage`](#errormessage) | struct |  |
| [`Inflate`](#inflate) | struct |  |
| [`Deflate`](#deflate) | struct |  |
| [`format_from_bool`](#format-from-bool) | fn |  |
| [`MZ_NO_FLUSH`](#mz-no-flush) | const |  |
| [`MZ_PARTIAL_FLUSH`](#mz-partial-flush) | const |  |
| [`MZ_SYNC_FLUSH`](#mz-sync-flush) | const |  |
| [`MZ_FULL_FLUSH`](#mz-full-flush) | const |  |
| [`MZ_FINISH`](#mz-finish) | const |  |

## Structs

### `ErrorMessage`

```rust
struct ErrorMessage;
```

#### Implementations

- <span id="errormessage-get"></span>`fn get(&self) -> Option<&str>`

#### Trait Implementations

##### `impl Clone for ErrorMessage`

- <span id="errormessage-clone"></span>`fn clone(&self) -> ErrorMessage` — [`ErrorMessage`](../index.md#errormessage)

##### `impl Debug for ErrorMessage`

- <span id="errormessage-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ErrorMessage`

- <span id="errormessage-default"></span>`fn default() -> ErrorMessage` — [`ErrorMessage`](../index.md#errormessage)

### `Inflate`

```rust
struct Inflate {
    inner: Box<::miniz_oxide::inflate::stream::InflateState>,
    total_in: u64,
    total_out: u64,
}
```

#### Trait Implementations

##### `impl Backend for Inflate`

- <span id="inflate-backend-total-in"></span>`fn total_in(&self) -> u64`

- <span id="inflate-backend-total-out"></span>`fn total_out(&self) -> u64`

##### `impl Debug for Inflate`

- <span id="inflate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl InflateBackend for Inflate`

- <span id="inflate-inflatebackend-make"></span>`fn make(zlib_header: bool, _window_bits: u8) -> Self`

- <span id="inflate-inflatebackend-decompress"></span>`fn decompress(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](../../mem/index.md#flushdecompress), [`Status`](../../mem/index.md#status), [`DecompressError`](../../mem/index.md#decompresserror)

- <span id="inflate-inflatebackend-reset"></span>`fn reset(&mut self, zlib_header: bool)`

### `Deflate`

```rust
struct Deflate {
    inner: Box<::miniz_oxide::deflate::core::CompressorOxide>,
    total_in: u64,
    total_out: u64,
}
```

#### Trait Implementations

##### `impl Backend for Deflate`

- <span id="deflate-backend-total-in"></span>`fn total_in(&self) -> u64`

- <span id="deflate-backend-total-out"></span>`fn total_out(&self) -> u64`

##### `impl Debug for Deflate`

- <span id="deflate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl DeflateBackend for Deflate`

- <span id="deflate-deflatebackend-make"></span>`fn make(level: Compression, zlib_header: bool, _window_bits: u8) -> Self` — [`Compression`](../../index.md#compression)

- <span id="deflate-deflatebackend-compress"></span>`fn compress(&mut self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](../../mem/index.md#flushcompress), [`Status`](../../mem/index.md#status), [`CompressError`](../../mem/index.md#compresserror)

- <span id="deflate-deflatebackend-reset"></span>`fn reset(&mut self)`

## Functions

### `format_from_bool`

```rust
fn format_from_bool(zlib_header: bool) -> DataFormat
```

## Constants

### `MZ_NO_FLUSH`
```rust
const MZ_NO_FLUSH: isize = 0isize;
```

### `MZ_PARTIAL_FLUSH`
```rust
const MZ_PARTIAL_FLUSH: isize = 1isize;
```

### `MZ_SYNC_FLUSH`
```rust
const MZ_SYNC_FLUSH: isize = 2isize;
```

### `MZ_FULL_FLUSH`
```rust
const MZ_FULL_FLUSH: isize = 3isize;
```

### `MZ_FINISH`
```rust
const MZ_FINISH: isize = 4isize;
```

