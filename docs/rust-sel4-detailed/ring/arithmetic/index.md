*[ring](../index.md) / [arithmetic](index.md)*

---

# Module `arithmetic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`constant`](#constant) | mod |  |
| [`bigint`](#bigint) | mod | Multi-precision integers. |
| [`montgomery`](#montgomery) | mod |  |
| [`n0`](#n0) | mod |  |
| [`limbs_from_hex`](#limbs-from-hex) | fn |  |
| [`BIGINT_MODULUS_MAX_LIMBS`](#bigint-modulus-max-limbs) | const |  |

## Modules

- [`constant`](constant/index.md)
- [`bigint`](bigint/index.md) — Multi-precision integers.
- [`montgomery`](montgomery/index.md)
- [`n0`](n0/index.md)

## Functions

### `limbs_from_hex`

```rust
const fn limbs_from_hex<const LIMBS: usize>(hex: &str) -> [u64; LIMBS]
```

## Constants

### `BIGINT_MODULUS_MAX_LIMBS`
```rust
const BIGINT_MODULUS_MAX_LIMBS: usize = 128usize;
```

