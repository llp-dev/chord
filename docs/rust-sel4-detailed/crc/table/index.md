*[crc](../index.md) / [table](index.md)*

---

# Module `table`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`crc8_table`](#crc8-table) | fn |  |
| [`crc16_table`](#crc16-table) | fn |  |
| [`crc32_table`](#crc32-table) | fn |  |
| [`crc64_table`](#crc64-table) | fn |  |
| [`crc128_table`](#crc128-table) | fn |  |

## Functions

### `crc8_table`

```rust
const fn crc8_table<const L: usize>(width: u8, poly: u8, reflect: bool) -> [[u8; 256]; L]
```

### `crc16_table`

```rust
const fn crc16_table<const L: usize>(width: u8, poly: u16, reflect: bool) -> [[u16; 256]; L]
```

### `crc32_table`

```rust
const fn crc32_table<const L: usize>(width: u8, poly: u32, reflect: bool) -> [[u32; 256]; L]
```

### `crc64_table`

```rust
const fn crc64_table<const L: usize>(width: u8, poly: u64, reflect: bool) -> [[u64; 256]; L]
```

### `crc128_table`

```rust
const fn crc128_table<const L: usize>(width: u8, poly: u128, reflect: bool) -> [[u128; 256]; L]
```

