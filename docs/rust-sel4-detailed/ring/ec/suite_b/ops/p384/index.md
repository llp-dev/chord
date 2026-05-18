*[ring](../../../../index.md) / [ec](../../../index.md) / [suite_b](../../index.md) / [ops](../index.md) / [p384](index.md)*

---

# Module `p384`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`p384_elem_mul_mont`](#p384-elem-mul-mont) | fn |  |
| [`p384_point_add`](#p384-point-add) | fn |  |
| [`p384_point_mul`](#p384-point-mul) | fn |  |
| [`p384_scalar_mul_mont`](#p384-scalar-mul-mont) | fn |  |
| [`p384_elem_inv_squared`](#p384-elem-inv-squared) | fn |  |
| [`p384_point_mul_base_impl`](#p384-point-mul-base-impl) | fn |  |
| [`p384_scalar_inv_to_mont`](#p384-scalar-inv-to-mont) | fn |  |
| [`p384_elem_sqr_mont`](#p384-elem-sqr-mont) | fn |  |

## Functions

### `p384_elem_mul_mont`

```rust
unsafe fn p384_elem_mul_mont(r: *mut u64, a: *const u64, b: *const u64)
```

### `p384_point_add`

```rust
unsafe fn p384_point_add(r: *mut u64, a: *const u64, b: *const u64)
```

### `p384_point_mul`

```rust
unsafe fn p384_point_mul(r: *mut u64, p_scalar: *const u64, p_x: *const u64, p_y: *const u64)
```

### `p384_scalar_mul_mont`

```rust
unsafe fn p384_scalar_mul_mont(r: *mut u64, a: *const u64, b: *const u64)
```

### `p384_elem_inv_squared`

```rust
fn p384_elem_inv_squared(a: &elem::Elem<Q, R>) -> elem::Elem<Q, R>
```

### `p384_point_mul_base_impl`

```rust
fn p384_point_mul_base_impl(a: &elem::Elem<N, Unencoded>) -> Point
```

### `p384_scalar_inv_to_mont`

```rust
fn p384_scalar_inv_to_mont(a: elem::Elem<N, R>) -> elem::Elem<N, R>
```

### `p384_elem_sqr_mont`

```rust
unsafe fn p384_elem_sqr_mont(r: *mut u64, a: *const u64)
```

