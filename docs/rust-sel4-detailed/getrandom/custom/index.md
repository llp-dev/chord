*[getrandom](../index.md) / [custom](index.md)*

---

# Module `custom`

An implementation which calls out to an externally defined function.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`getrandom_inner`](#getrandom-inner) | fn |  |

## Functions

### `getrandom_inner`

```rust
fn getrandom_inner(dest: &mut [core::mem::MaybeUninit<u8>]) -> Result<(), crate::Error>
```

