*[zmij](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sealed`](#sealed) | trait |  |

## Traits

### `Sealed`

```rust
trait Sealed: crate::traits::Float { ... }
```

#### Required Methods

- `fn is_nonfinite(self) -> bool`

- `fn format_nonfinite(self) -> &'static str`

- `fn write_to_zmij_buffer(self, buffer: *mut u8) -> *mut u8`

#### Implementors

- `f32`
- `f64`

