# crc32fast

Fast, SIMD-accelerated CRC32 (IEEE) checksum computation.

## Usage

### Simple usage

For simple use-cases, you can call the [`hash()`] convenience function to
directly compute the CRC32 checksum for a given byte slice:

```rust
let checksum = crc32fast::hash(b"foo bar baz");
```

### Advanced usage

For use-cases that require more flexibility or performance, for example when
processing large amounts of data, you can create and manipulate a [`Hasher`]:

```rust
use crc32fast::Hasher;

let mut hasher = Hasher::new();
hasher.update(b"foo bar baz");
let checksum = hasher.finalize();
```

## Performance

This crate contains multiple CRC32 implementations:

- A fast baseline implementation which processes up to 16 bytes per iteration
- An optimized implementation for modern `x86` using `sse` and `pclmulqdq` instructions

Calling the [`Hasher::new`] constructor at runtime will perform a feature detection to select the most
optimal implementation for the current CPU feature set.

## Modules

### [`crc32fast`](crc32fast.md)

*1 function, 1 struct*

