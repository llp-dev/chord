*[num_traits](../../index.md) / [ops](../index.md) / [mul_add](index.md)*

---

# Module `mul_add`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MulAdd`](#muladd) | trait | Fused multiply-add. |
| [`MulAddAssign`](#muladdassign) | trait | The fused multiply-add assignment operation `*self = (*self * a) + b` |
| [`mul_add_impl!`](#mul-add-impl) | macro |  |
| [`mul_add_assign_impl!`](#mul-add-assign-impl) | macro |  |

## Traits

### `MulAdd<A, B>`

```rust
trait MulAdd<A, B> { ... }
```

Fused multiply-add. Computes `(self * a) + b` with only one rounding
error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` can be more performant than an unfused multiply-add if
the target architecture has a dedicated `fma` CPU instruction.

Note that `A` and `B` are `Self` by default, but this is not mandatory.

# Example

```rust
use std::f32;

let m = 10.0_f32;
let x = 4.0_f32;
let b = 60.0_f32;

// 100.0
let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();

assert!(abs_difference <= 100.0 * f32::EPSILON);
```

#### Associated Types

- `type Output`

#### Required Methods

- `fn mul_add(self, a: A, b: B) -> <Self as >::Output`

  Performs the fused multiply-add operation `(self * a) + b`

#### Implementors

- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `MulAddAssign<A, B>`

```rust
trait MulAddAssign<A, B> { ... }
```

The fused multiply-add assignment operation `*self = (*self * a) + b`

#### Required Methods

- `fn mul_add_assign(&mut self, a: A, b: B)`

  Performs the fused multiply-add assignment operation `*self = (*self * a) + b`

#### Implementors

- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Macros

### `mul_add_impl!`

### `mul_add_assign_impl!`

