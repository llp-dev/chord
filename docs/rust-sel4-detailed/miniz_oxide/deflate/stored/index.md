*[miniz_oxide](../../index.md) / [deflate](../index.md) / [stored](index.md)*

---

# Module `stored`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`compress_stored`](#compress-stored) | fn | Compression function for stored blocks, split out from the main compression function. |

## Functions

### `compress_stored`

```rust
fn compress_stored(d: &mut crate::deflate::core::CompressorOxide, callback: &mut crate::deflate::core::CallbackOxide<'_>) -> bool
```

Compression function for stored blocks, split out from the main compression function.

