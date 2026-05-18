*[ring](../../../index.md) / [aead](../../index.md) / [gcm](../index.md) / [gcm_nohw](index.md)*

---

# Module `gcm_nohw`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`gcm_mul64_nohw`](#gcm-mul64-nohw) | fn |  |
| [`init`](#init) | fn |  |
| [`gcm_polyval_nohw`](#gcm-polyval-nohw) | fn |  |
| [`gmult`](#gmult) | fn |  |
| [`ghash`](#ghash) | fn |  |
| [`with_swapped_xi`](#with-swapped-xi) | fn |  |

## Functions

### `gcm_mul64_nohw`

```rust
fn gcm_mul64_nohw(a: u64, b: u64) -> (u64, u64)
```

### `init`

```rust
fn init(xi: [u64; 2]) -> super::u128
```

### `gcm_polyval_nohw`

```rust
fn gcm_polyval_nohw(xi: &mut [u64; 2], h: super::u128)
```

### `gmult`

```rust
fn gmult(xi: &mut super::Xi, h: super::u128)
```

### `ghash`

```rust
fn ghash(xi: &mut super::Xi, h: super::u128, input: &[[u8; 16]])
```

### `with_swapped_xi`

```rust
fn with_swapped_xi(Xi: &mut super::Xi, f: impl FnOnce(&mut [u64; 2]))
```

