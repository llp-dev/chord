# Crate `ruzstd`

A pure Rust implementation of the [Zstandard compression format](https://www.rfc-editor.org/rfc/rfc8878.pdf).

## Decompression
The [`decoding`](decoding/index.md) module contains the code for decompression.
Decompression can be achieved by using the [`decoding::StreamingDecoder`](decoding/streaming_decoder/index.md)
or the more low-level [`decoding::FrameDecoder`](decoding/frame_decoder/index.md)

## Compression
The [`encoding`](encoding/index.md) module contains the code for compression.
Compression can be achieved by using the [`encoding::compress`](encoding/index.md)/[`encoding::compress_to_vec`](encoding/index.md)
functions or [`encoding::FrameCompressor`](encoding/frame_compressor/index.md)

# Ruzstd (a pure rust zstd format implementation)

[![Released API docs](https://docs.rs/ruzstd/badge.svg)](https://docs.rs/ruzstd)
[![CI](https://github.com/killingspark/zstd-rs/workflows/CI/badge.svg)](https://github.com/killingspark/zstd-rs/actions?query=workflow%3ACI)


# What is this

A pure Rust implementation of the Zstandard compression format, as defined in [RFC8878](https://www.rfc-editor.org/rfc/rfc8878.pdf).

This crate contains a fully operational implementation of the decompression portion of the standard.
It also provides a compressor which is usable, but it does not yet reach the speed, ratio or configurability of the original zstd library.

This crate is currently actively maintained.

# Current Status

## Decompression
The `decoding` module provides a complete
implementation of a Zstandard decompressor.

In terms of speed, `ruzstd` is behind the original C implementation
which has a rust binding located [here](https://github.com/gyscos/zstd-rs).

Measuring with the 'time' utility the original zstd and my decoder both
decoding the same enwik9.zst file from a ramfs, my decoder is about 3.5
times slower. Enwik9 is highly compressible, for less compressible data
(like a ubuntu installation .iso) my decoder comes close to only being
1.4 times slower.

## Compression
On the compression side:
- Support for generating compressed blocks at any compression level
  - [x] Uncompressed
  - [x] Fastest (roughly level 1)
  - [ ] Default (roughly level 3)
  - [ ] Better (roughly level 7)
  - [ ] Best (roughly level 11)
- [x] Checksums
- [ ] Dictionaries

## Dictionary Generation
When the `dict_builder` feature is enabled, the `dictionary` module
provides the ability to create new dictionaries. 

On the `github-users` sample set, our implementation benchmarks within
0.2% of the official implementation (as of commit 
`09e52d07340acdb2e13817b066e8be6e424f7258`):
```no_build
uncompressed: 100.00% (7484607 bytes)
no dict: 34.99% of original size (2618872 bytes)
reference dict: 16.16% of no dict size (2195672 bytes smaller)
our dict: 16.28% of no dict size (2192400 bytes smaller)
```

The dictionary generator only provides support for creating "raw
content" dictionaries. Tagged dictionaries are currently unsupported.

See <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#dictionary-format>
for clarification.


# How can you use it?

## Compression

The easiest is to use the provided `compress`/`compress_to_vec` functions

```rust, no_run
use ruzstd::encoding::{compress, compress_to_vec, CompressionLevel};
let data: &[u8] = todo!();
// Either
let mut compressed = Vec::new();
compress(data, &mut compressed, CompressionLevel::Fastest);
// or
let compressed = compress_to_vec(data, CompressionLevel::Fastest);
 ```

 Or you can use the `FrameDecoder` manually to compress data. This allows you to process encoded data while it is being encoded instead of collecting into a big vector.

## Decompression

Additionally to the descriptions and the docs you can have a look at the zstd / zstd_streaming binaries. They showcase how this library can be used.

### Easy

The easiest is to wrap the io::Read into a StreamingDecoder which itself implements io::Read. It will decode blocks as necessary to fulfill the read requests

```rust, no_run
use ruzstd::decoding::StreamingDecoder;
use ruzstd::io::Read;

let mut source: &[u8] = todo!("Get a reader from a File or any other source");
let mut decoder = StreamingDecoder::new(&mut source).unwrap();

let mut result = Vec::new();
decoder.read_to_end(&mut result).unwrap();
```

This might be a problem if you are accepting user provided data. Frames can be REALLY big when decoded. If this is the case you should either check how big the frame
actually is or use the memory efficient approach described below.

### Memory efficient

If memory is a concern you can decode frames partially. There are two ways to do this:

#### Streaming decoder

Use the StreamingDecoder and use a while loop to fill your buffer (see src/bin/zstd_stream.rs for an example). This is the
recommended approach.

#### Use the lower level FrameDecoder

For an example see the src/bin/zstd.rs file. Basically you can decode the frame until either a
given block count has been decoded or the decodebuffer has reached a certain size. Then you can collect no longer needed bytes from the buffer and do something with them, discard them and resume decoding the frame in a loop until the frame has been decoded completely.

## Roadmap

1. More Performance optimizations
    1. sequence_decoding and reverse_bitreader::get_bits. Those account for about 50% of the whole time used in decoding
    2. Matching suffixes. This accounts for >60% of the whole time used in encoding
2. Implement encoder features
    1. More levels
    2. Dictionaries
    3. Checksums

## Testing

Tests take two forms.

1. Tests using well-formed files that have to decode correctly and are checked against their originals
1. Tests using malformed input that have been generated by the fuzzer. These don't have to decode (they are garbage) but they must not make the decoder panic

## Fuzzing

Fuzzing has been done on

1. Random input with no initial corpus
2. The \*.zst in /fuzz_decodecorpus


### You want to help fuzz?

Use `cargo +nightly fuzz run decode` or some other fuzz target to run the fuzzer. It is seeded with files created with decodecorpus.

If the fuzzer finds a crash it will be saved to the artifacts dir by the fuzzer. Run `cargo test artifacts` to run the artifacts tests.
This will tell you where the decoder panics exactly. If you are able to fix the issue please feel free to do a pull request. If not please still submit the offending input and I will see how to fix it myself.

# Contributing

Contributions will be published under the same MIT license as this project. Please make an entry in the Changelog.md file when you make a PR.

## Contents

- [Modules](#modules)
  - [`bit_io`](#bit-io)
  - [`common`](#common)
  - [`decoding`](#decoding)
  - [`encoding`](#encoding)
  - [`blocks`](#blocks)
  - [`fse`](#fse)
  - [`huff0`](#huff0)
  - [`io_std`](#io-std)
  - [`tests`](#tests)
  - [`io`](#io)
- [Constants](#constants)
  - [`VERBOSE`](#verbose)
- [Macros](#macros)
  - [`vprintln!`](#vprintln)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bit_io`](#bit-io) | mod | Encoding agnostic ways to read and write binary data |
| [`common`](#common) | mod | Values and interfaces shared between the encoding side and the decoding side. |
| [`decoding`](#decoding) | mod | Structures and utilities used for decoding zstd formatted data |
| [`encoding`](#encoding) | mod | Structures and utilities used for compressing/encoding data into the Zstd format. |
| [`blocks`](#blocks) | mod | In a Zstandard frame, there's a frame header, followed by one or more *blocks*. |
| [`fse`](#fse) | mod | FSE, short for Finite State Entropy, is an encoding technique that assigns shorter codes to symbols that appear more frequently in data, and longer codes to less frequent symbols. |
| [`huff0`](#huff0) | mod |  |
| [`io_std`](#io-std) | mod | Re-exports of std traits or local reimplementations if std is not available |
| [`tests`](#tests) | mod |  |
| [`io`](#io) | mod |  |
| [`VERBOSE`](#verbose) | const |  |
| [`vprintln!`](#vprintln) | macro |  |

## Modules

- [`bit_io`](bit_io/index.md) — Encoding agnostic ways to read and write binary data
- [`common`](common/index.md) — Values and interfaces shared between the encoding side
- [`decoding`](decoding/index.md) — Structures and utilities used for decoding zstd formatted data
- [`encoding`](encoding/index.md) — Structures and utilities used for compressing/encoding data into the Zstd format.
- [`blocks`](blocks/index.md) — In a Zstandard frame, there's a frame header, followed by one or more *blocks*.
- [`fse`](fse/index.md) — FSE, short for Finite State Entropy, is an encoding technique
- [`huff0`](huff0/index.md)
- [`io_std`](io_std/index.md) — Re-exports of std traits or local reimplementations if std is not available
- [`tests`](tests/index.md)
- [`io`](io/index.md) — Re-exports of std traits or local reimplementations if std is not available

## Constants

### `VERBOSE`
```rust
const VERBOSE: bool = false;
```

## Macros

### `vprintln!`

