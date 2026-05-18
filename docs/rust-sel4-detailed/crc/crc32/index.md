*[crc](../index.md) / [crc32](index.md)*

---

# Module `crc32`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`init`](#init) | fn |  |
| [`finalize`](#finalize) | fn |  |
| [`update_table`](#update-table) | fn |  |

## Functions

### `init`

```rust
const fn init(algorithm: &crc_catalog::Algorithm<u32>, initial: u32) -> u32
```

### `finalize`

```rust
const fn finalize(algorithm: &crc_catalog::Algorithm<u32>, crc: u32) -> u32
```

### `update_table`

```rust
const fn update_table<const L: usize>(crc: u32, algorithm: &crc_catalog::Algorithm<u32>, table: &[[u32; 256]; L], bytes: &[u8]) -> u32
```

