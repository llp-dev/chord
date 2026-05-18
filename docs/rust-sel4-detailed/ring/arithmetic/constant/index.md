*[ring](../../index.md) / [arithmetic](../index.md) / [constant](index.md)*

---

# Module `constant`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse_digit`](#parse-digit) | fn |  |
| [`limbs_from_hex`](#limbs-from-hex) | fn |  |

## Functions

### `parse_digit`

```rust
const fn parse_digit(d: u8) -> u8
```

### `limbs_from_hex`

```rust
const fn limbs_from_hex<const LIMBS: usize>(hex: &str) -> [u64; LIMBS]
```

