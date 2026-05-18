*[getrandom](../index.md) / [imp](index.md)*

---

# Module `imp`

Implementation for Linux / Android with `/dev/urandom` fallback

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`getrandom_inner`](#getrandom-inner) | fn |  |
| [`is_getrandom_available`](#is-getrandom-available) | fn |  |

## Functions

### `getrandom_inner`

```rust
fn getrandom_inner(dest: &mut [core::mem::MaybeUninit<u8>]) -> Result<(), crate::Error>
```

### `is_getrandom_available`

```rust
fn is_getrandom_available() -> bool
```

