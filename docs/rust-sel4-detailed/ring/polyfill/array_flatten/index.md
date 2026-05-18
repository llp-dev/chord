*[ring](../../index.md) / [polyfill](../index.md) / [array_flatten](index.md)*

---

# Module `array_flatten`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArrayFlatten`](#arrayflatten) | trait |  |

## Traits

### `ArrayFlatten`

```rust
trait ArrayFlatten { ... }
```

#### Associated Types

- `type Output`

#### Required Methods

- `fn array_flatten(self) -> <Self as >::Output`

  Returns the flattened form of `a`

#### Implementors

- `[[T; 4]; 4]`
- `[[T; 8]; 2]`

