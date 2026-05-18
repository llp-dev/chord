*[zmij](../index.md) / [traits](index.md)*

---

# Module `traits`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Float`](#float) | trait |  |
| [`UInt`](#uint) | trait |  |

## Traits

### `Float`

```rust
trait Float: Copy { ... }
```

#### Associated Constants

- `const MANTISSA_DIGITS: u32`

- `const MIN_10_EXP: i32`

- `const MAX_10_EXP: i32`

- `const MAX_DIGITS10: u32`

#### Implementors

- `f32`
- `f64`

### `UInt`

```rust
trait UInt: Copy + From<u8> + From<bool> + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + Shl<u8, Output = Self> + Shl<i32, Output = Self> + Shl<u32, Output = Self> + Shr<i32, Output = Self> + Shr<u32, Output = Self> + BitOrAssign + BitXorAssign + PartialOrd + Into<u64> + Display { ... }
```

#### Associated Types

- `type Signed: 1`

#### Required Methods

- `fn wrapping_sub(self, other: Self) -> Self`

- `fn truncate(big: u64) -> Self`

- `fn enlarge(small: u32) -> Self`

- `fn to_signed(self) -> <Self as >::Signed`

#### Implementors

- `u32`
- `u64`

