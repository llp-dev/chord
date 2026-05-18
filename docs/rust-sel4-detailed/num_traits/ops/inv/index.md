*[num_traits](../../index.md) / [ops](../index.md) / [inv](index.md)*

---

# Module `inv`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Inv`](#inv) | trait | Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value. |

## Traits

### `Inv`

```rust
trait Inv { ... }
```

Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.

#### Associated Types

- `type Output`

#### Required Methods

- `fn inv(self) -> <Self as >::Output`

  Returns the multiplicative inverse of `self`.

#### Implementors

- `&'a f32`
- `&'a f64`
- `f32`
- `f64`

