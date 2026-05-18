*[ruzstd](../index.md) / [common](index.md)*

---

# Module `common`

Values and interfaces shared between the encoding side
and the decoding side.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MAGIC_NUM`](#magic-num) | const | This magic number is included at the start of a single Zstandard frame |
| [`MIN_WINDOW_SIZE`](#min-window-size) | const | Window size refers to the minimum amount of memory needed to decode any given frame. |
| [`MAX_WINDOW_SIZE`](#max-window-size) | const | Window size refers to the minimum amount of memory needed to decode any given frame. |
| [`MAX_BLOCK_SIZE`](#max-block-size) | const | While the spec limits block size to 128KB, the implementation uses 128kibibytes |

## Constants

### `MAGIC_NUM`
```rust
const MAGIC_NUM: u32 = 4_247_762_216u32;
```

This magic number is included at the start of a single Zstandard frame

### `MIN_WINDOW_SIZE`
```rust
const MIN_WINDOW_SIZE: u64 = 1_024u64;
```

Window size refers to the minimum amount of memory needed to decode any given frame.

The minimum window size is defined as 1 KB

### `MAX_WINDOW_SIZE`
```rust
const MAX_WINDOW_SIZE: u64 = 4_123_168_604_160u64;
```

Window size refers to the minimum amount of memory needed to decode any given frame.

The maximum window size allowed by the spec is 3.75TB

### `MAX_BLOCK_SIZE`
```rust
const MAX_BLOCK_SIZE: u32 = 131_072u32;
```

While the spec limits block size to 128KB, the implementation uses
128kibibytes

<https://github.com/facebook/zstd/blob/eca205fc7849a61ab287492931a04960ac58e031/doc/educational_decoder/zstd_decompress.c#L28-L29>

