*[crc](../index.md) / [crc16](index.md)*

---

# Module `crc16`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`init`](#init) | fn |  |
| [`finalize`](#finalize) | fn |  |
| [`update_table`](#update-table) | fn |  |

## Functions

### `init`

```rust
const fn init(algorithm: &crc_catalog::Algorithm<u16>, initial: u16) -> u16
```

### `finalize`

```rust
const fn finalize(algorithm: &crc_catalog::Algorithm<u16>, crc: u16) -> u16
```

### `update_table`

```rust
const fn update_table<const L: usize>(crc: u16, algorithm: &crc_catalog::Algorithm<u16>, table: &[[u16; 256]; L], bytes: &[u8]) -> u16
```

