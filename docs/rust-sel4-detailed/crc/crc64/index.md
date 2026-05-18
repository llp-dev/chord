*[crc](../index.md) / [crc64](index.md)*

---

# Module `crc64`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`init`](#init) | fn |  |
| [`finalize`](#finalize) | fn |  |
| [`update_table`](#update-table) | fn |  |

## Functions

### `init`

```rust
const fn init(algorithm: &crc_catalog::Algorithm<u64>, initial: u64) -> u64
```

### `finalize`

```rust
const fn finalize(algorithm: &crc_catalog::Algorithm<u64>, crc: u64) -> u64
```

### `update_table`

```rust
const fn update_table<const L: usize>(crc: u64, algorithm: &crc_catalog::Algorithm<u64>, table: &[[u64; 256]; L], bytes: &[u8]) -> u64
```

