# miniz_oxide

A pure rust replacement for the [miniz](https://github.com/richgel999/miniz)
DEFLATE/zlib encoder/decoder.
Used a rust back-end for the
[flate2](https://github.com/alexcrichton/flate2-rs) crate.

# Usage
## Simple compression/decompression:
``` rust

use miniz_oxide::inflate::decompress_to_vec;
use miniz_oxide::deflate::compress_to_vec;

fn roundtrip(data: &[u8]) {
    let compressed = compress_to_vec(data, 6);
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
#   let _ = decompressed;
}

# roundtrip(b"Test_data test data lalalal blabla");

## Modules

### [`miniz_oxide`](miniz_oxide.md)

*1 struct, 1 type alias, 2 modules, 4 enums*

### [`deflate`](deflate.md)

*1 enum, 2 functions, 2 modules*

### [`deflate::core`](deflate/core.md)

*1 module, 2 structs, 3 enums, 3 functions*

### [`deflate::core::deflate_flags`](deflate/core/deflate_flags.md)

*8 constants*

### [`deflate::stream`](deflate/stream.md)

*1 function*

### [`inflate`](inflate.md)

*1 enum, 1 struct, 2 modules, 5 functions*

### [`inflate::core`](inflate/core.md)

*1 constant, 1 function, 1 module, 1 struct*

### [`inflate::core::inflate_flags`](inflate/core/inflate_flags.md)

*5 constants*

### [`inflate::stream`](inflate/stream.md)

*1 function, 1 trait, 4 structs*

