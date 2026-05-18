*[crc](../index.md) / [crc8](index.md)*

---

# Module `crc8`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`init`](#init) | fn |  |
| [`finalize`](#finalize) | fn |  |
| [`update_table`](#update-table) | fn |  |

## Functions

### `init`

```rust
const fn init(algorithm: &crc_catalog::Algorithm<u8>, initial: u8) -> u8
```

### `finalize`

```rust
const fn finalize(algorithm: &crc_catalog::Algorithm<u8>, crc: u8) -> u8
```

### `update_table`

```rust
const fn update_table<const L: usize>(crc: u8, algorithm: &crc_catalog::Algorithm<u8>, table: &[[u8; 256]; L], bytes: &[u8]) -> u8
```

