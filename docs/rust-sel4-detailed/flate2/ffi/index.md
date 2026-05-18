*[flate2](../index.md) / [ffi](index.md)*

---

# Module `ffi`

This module contains backend-specific code.

## Contents

- [Modules](#modules)
  - [`miniz_oxide`](#miniz-oxide)
- [Structs](#structs)
  - [`ErrorMessage`](#errormessage)
  - [`Inflate`](#inflate)
  - [`Deflate`](#deflate)
- [Traits](#traits)
  - [`Backend`](#backend)
  - [`InflateBackend`](#inflatebackend)
  - [`DeflateBackend`](#deflatebackend)
- [Functions](#functions)
  - [`initialize_buffer`](#initialize-buffer)
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
| [`miniz_oxide`](#miniz-oxide) | mod | Implementation for `miniz_oxide` rust backend. |
| [`ErrorMessage`](#errormessage) | struct |  |
| [`Inflate`](#inflate) | struct |  |
| [`Deflate`](#deflate) | struct |  |
| [`Backend`](#backend) | trait | Traits specifying the interface of the backends. |
| [`InflateBackend`](#inflatebackend) | trait |  |
| [`DeflateBackend`](#deflatebackend) | trait |  |
| [`initialize_buffer`](#initialize-buffer) | fn |  |
| [`format_from_bool`](#format-from-bool) | fn |  |
| [`MZ_NO_FLUSH`](#mz-no-flush) | const |  |
| [`MZ_PARTIAL_FLUSH`](#mz-partial-flush) | const |  |
| [`MZ_SYNC_FLUSH`](#mz-sync-flush) | const |  |
| [`MZ_FULL_FLUSH`](#mz-full-flush) | const |  |
| [`MZ_FINISH`](#mz-finish) | const |  |

## Modules

- [`miniz_oxide`](miniz_oxide/index.md) — Implementation for `miniz_oxide` rust backend.

## Structs

### `ErrorMessage`

```rust
struct ErrorMessage;
```

#### Implementations

- <span id="errormessage-get"></span>`fn get(&self) -> Option<&str>`

#### Trait Implementations

##### `impl Clone for ErrorMessage`

- <span id="errormessage-clone"></span>`fn clone(&self) -> ErrorMessage` — [`ErrorMessage`](#errormessage)

##### `impl Debug for ErrorMessage`

- <span id="errormessage-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ErrorMessage`

- <span id="errormessage-default"></span>`fn default() -> ErrorMessage` — [`ErrorMessage`](#errormessage)

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

- <span id="inflate-inflatebackend-decompress"></span>`fn decompress(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>` — [`FlushDecompress`](../mem/index.md#flushdecompress), [`Status`](../mem/index.md#status), [`DecompressError`](../mem/index.md#decompresserror)

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

- <span id="deflate-deflatebackend-make"></span>`fn make(level: Compression, zlib_header: bool, _window_bits: u8) -> Self` — [`Compression`](../index.md#compression)

- <span id="deflate-deflatebackend-compress"></span>`fn compress(&mut self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>` — [`FlushCompress`](../mem/index.md#flushcompress), [`Status`](../mem/index.md#status), [`CompressError`](../mem/index.md#compresserror)

- <span id="deflate-deflatebackend-reset"></span>`fn reset(&mut self)`

## Traits

### `Backend`

```rust
trait Backend: Sync + Send { ... }
```

Traits specifying the interface of the backends.

Sync + Send are added as a condition to ensure they are available
for the frontend.

#### Required Methods

- `fn total_in(&self) -> u64`

- `fn total_out(&self) -> u64`

#### Implementors

- [`Deflate`](#deflate)
- [`Inflate`](#inflate)

### `InflateBackend`

```rust
trait InflateBackend: Backend { ... }
```

#### Required Methods

- `fn make(zlib_header: bool, window_bits: u8) -> Self`

- `fn decompress(&mut self, input: &[u8], output: &mut [u8], flush: FlushDecompress) -> Result<Status, DecompressError>`

- `fn reset(&mut self, zlib_header: bool)`

#### Provided Methods

- `fn decompress_uninit(&mut self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushDecompress) -> Result<Status, DecompressError>`

#### Implementors

- [`Inflate`](#inflate)

### `DeflateBackend`

```rust
trait DeflateBackend: Backend { ... }
```

#### Required Methods

- `fn make(level: Compression, zlib_header: bool, window_bits: u8) -> Self`

- `fn compress(&mut self, input: &[u8], output: &mut [u8], flush: FlushCompress) -> Result<Status, CompressError>`

- `fn reset(&mut self)`

#### Provided Methods

- `fn compress_uninit(&mut self, input: &[u8], output: &mut [MaybeUninit<u8>], flush: FlushCompress) -> Result<Status, CompressError>`

#### Implementors

- [`Deflate`](#deflate)

## Functions

### `initialize_buffer`

```rust
fn initialize_buffer(output: &mut [std::mem::MaybeUninit<u8>]) -> &mut [u8]
```

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

