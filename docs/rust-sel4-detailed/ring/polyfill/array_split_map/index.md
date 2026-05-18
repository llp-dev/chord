*[ring](../../index.md) / [polyfill](../index.md) / [array_split_map](index.md)*

---

# Module `array_split_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArraySplitMap`](#arraysplitmap) | trait |  |

## Traits

### `ArraySplitMap<I, O, const CN: usize, const ON: usize>`

```rust
trait ArraySplitMap<I, O, const CN: usize, const ON: usize> { ... }
```

#### Required Methods

- `fn array_split_map(self, f: impl Fn([I; CN]) -> O) -> [O; ON]`

#### Implementors

- `[I; 12]`
- `[I; 16]`
- `[I; 32]`

