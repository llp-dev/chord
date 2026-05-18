*[indexmap](../../index.md) / [inner](../index.md) / [extract](index.md)*

---

# Module `extract`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ExtractCore`](#extractcore) | struct |  |

## Structs

### `ExtractCore<'a, K, V>`

```rust
struct ExtractCore<'a, K, V> {
    map: &'a mut super::Core<K, V>,
    new_len: usize,
    current: usize,
    end: usize,
}
```

#### Implementations

- <span id="extractcore-extract-if"></span>`fn extract_if<F>(&mut self, pred: F) -> Option<Bucket<K, V>>` — [`Bucket`](../../index.md#bucket)

- <span id="extractcore-remaining"></span>`fn remaining(&self) -> usize`

#### Trait Implementations

##### `impl<K, V> Drop for ExtractCore<'_, K, V>`

- <span id="extractcore-drop"></span>`fn drop(&mut self)`

