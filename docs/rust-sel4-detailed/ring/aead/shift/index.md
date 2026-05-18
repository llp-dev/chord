*[ring](../../index.md) / [aead](../index.md) / [shift](index.md)*

---

# Module `shift`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`shift_partial`](#shift-partial) | fn |  |

## Functions

### `shift_partial`

```rust
fn shift_partial<F>((in_prefix_len, in_out): (usize, &mut [u8]), transform: F)
where
    F: FnOnce(&[u8]) -> super::block::Block
```

