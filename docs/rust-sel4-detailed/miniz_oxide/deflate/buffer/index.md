*[miniz_oxide](../../index.md) / [deflate](../index.md) / [buffer](index.md)*

---

# Module `buffer`

Buffer wrappers implementing default so we can allocate the buffers with `Box::default()`
to avoid stack copies. Box::new() doesn't at the moment, and using a vec means we would lose
static length info.

## Contents

- [Structs](#structs)
  - [`HashBuffers`](#hashbuffers)
  - [`LocalBuf`](#localbuf)
- [Functions](#functions)
  - [`update_hash`](#update-hash)
- [Constants](#constants)
  - [`LZ_CODE_BUF_SIZE`](#lz-code-buf-size)
  - [`LZ_CODE_BUF_MASK`](#lz-code-buf-mask)
  - [`OUT_BUF_SIZE`](#out-buf-size)
  - [`LZ_DICT_FULL_SIZE`](#lz-dict-full-size)
  - [`LZ_HASH_BITS`](#lz-hash-bits)
  - [`LZ_HASH_SHIFT`](#lz-hash-shift)
  - [`LZ_HASH_SIZE`](#lz-hash-size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HashBuffers`](#hashbuffers) | struct |  |
| [`LocalBuf`](#localbuf) | struct |  |
| [`update_hash`](#update-hash) | fn |  |
| [`LZ_CODE_BUF_SIZE`](#lz-code-buf-size) | const | Size of the buffer of lz77 encoded data. |
| [`LZ_CODE_BUF_MASK`](#lz-code-buf-mask) | const |  |
| [`OUT_BUF_SIZE`](#out-buf-size) | const | Size of the output buffer. |
| [`LZ_DICT_FULL_SIZE`](#lz-dict-full-size) | const |  |
| [`LZ_HASH_BITS`](#lz-hash-bits) | const | Size of hash values in the hash chains. |
| [`LZ_HASH_SHIFT`](#lz-hash-shift) | const | How many bits to shift when updating the current hash value. |
| [`LZ_HASH_SIZE`](#lz-hash-size) | const | Size of the chained hash tables. |

## Structs

### `HashBuffers`

```rust
struct HashBuffers {
    pub dict: alloc::boxed::Box<[u8; 33026]>,
    pub next: alloc::boxed::Box<[u16; 32768]>,
    pub hash: alloc::boxed::Box<[u16; 32768]>,
}
```

#### Implementations

- <span id="hashbuffers-reset"></span>`fn reset(&mut self)`

#### Trait Implementations

##### `impl Clone for HashBuffers`

- <span id="hashbuffers-clone"></span>`fn clone(&self) -> HashBuffers` — [`HashBuffers`](#hashbuffers)

##### `impl Default for HashBuffers`

- <span id="hashbuffers-default"></span>`fn default() -> HashBuffers` — [`HashBuffers`](#hashbuffers)

### `LocalBuf`

```rust
struct LocalBuf {
    pub b: [u8; 85196],
}
```

#### Trait Implementations

##### `impl Clone for LocalBuf`

- <span id="localbuf-clone"></span>`fn clone(&self) -> LocalBuf` — [`LocalBuf`](#localbuf)

##### `impl Default for LocalBuf`

- <span id="localbuf-default"></span>`fn default() -> LocalBuf` — [`LocalBuf`](#localbuf)

## Functions

### `update_hash`

```rust
const fn update_hash(current_hash: u16, byte: u8) -> u16
```

## Constants

### `LZ_CODE_BUF_SIZE`
```rust
const LZ_CODE_BUF_SIZE: usize = 65_536usize;
```

Size of the buffer of lz77 encoded data.

### `LZ_CODE_BUF_MASK`
```rust
const LZ_CODE_BUF_MASK: usize = 65_535usize;
```

### `OUT_BUF_SIZE`
```rust
const OUT_BUF_SIZE: usize = 85_196usize;
```

Size of the output buffer.

### `LZ_DICT_FULL_SIZE`
```rust
const LZ_DICT_FULL_SIZE: usize = 33_026usize;
```

### `LZ_HASH_BITS`
```rust
const LZ_HASH_BITS: i32 = 15i32;
```

Size of hash values in the hash chains.

### `LZ_HASH_SHIFT`
```rust
const LZ_HASH_SHIFT: i32 = 5i32;
```

How many bits to shift when updating the current hash value.

### `LZ_HASH_SIZE`
```rust
const LZ_HASH_SIZE: usize = 32_768usize;
```

Size of the chained hash tables.

