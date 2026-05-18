*[crc](../index.md) / [crc128](index.md)*

---

# Module `crc128`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`init`](#init) | fn |  |
| [`finalize`](#finalize) | fn |  |
| [`update_table`](#update-table) | fn |  |

## Functions

### `init`

```rust
const fn init(algorithm: &crc_catalog::Algorithm<u128>, initial: u128) -> u128
```

### `finalize`

```rust
const fn finalize(algorithm: &crc_catalog::Algorithm<u128>, crc: u128) -> u128
```

### `update_table`

```rust
const fn update_table<const L: usize>(crc: u128, algorithm: &crc_catalog::Algorithm<u128>, table: &[[u128; 256]; L], bytes: &[u8]) -> u128
```

