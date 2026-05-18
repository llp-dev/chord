*[byteorder](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sealed`](#sealed) | trait | Sealed stops crates other than byteorder from implementing any traits that use it. |

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

Sealed stops crates other than byteorder from implementing any traits
that use it.

#### Implementors

- [`BigEndian`](../index.md#bigendian)
- [`LittleEndian`](../index.md#littleendian)

