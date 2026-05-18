*[ring](../../../../index.md) / [ec](../../../index.md) / [suite_b](../../index.md) / [ops](../index.md) / [p256](index.md)*

---

# Module `p256`

## Contents

- [Functions](#functions)
  - [`p256_mul_mont`](#p256-mul-mont)
  - [`p256_sqr_mont`](#p256-sqr-mont)
  - [`p256_point_add`](#p256-point-add)
  - [`p256_point_mul`](#p256-point-mul)
  - [`p256_scalar_mul_mont`](#p256-scalar-mul-mont)
  - [`p256_scalar_sqr_rep_mont`](#p256-scalar-sqr-rep-mont)
  - [`p256_elem_inv_squared`](#p256-elem-inv-squared)
  - [`p256_point_mul_base_impl`](#p256-point-mul-base-impl)
  - [`twin_mul_nistz256`](#twin-mul-nistz256)
  - [`point_mul_base_vartime`](#point-mul-base-vartime)
  - [`p256_scalar_inv_to_mont`](#p256-scalar-inv-to-mont)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`p256_mul_mont`](#p256-mul-mont) | fn |  |
| [`p256_sqr_mont`](#p256-sqr-mont) | fn |  |
| [`p256_point_add`](#p256-point-add) | fn |  |
| [`p256_point_mul`](#p256-point-mul) | fn |  |
| [`p256_scalar_mul_mont`](#p256-scalar-mul-mont) | fn |  |
| [`p256_scalar_sqr_rep_mont`](#p256-scalar-sqr-rep-mont) | fn |  |
| [`p256_elem_inv_squared`](#p256-elem-inv-squared) | fn |  |
| [`p256_point_mul_base_impl`](#p256-point-mul-base-impl) | fn |  |
| [`twin_mul_nistz256`](#twin-mul-nistz256) | fn |  |
| [`point_mul_base_vartime`](#point-mul-base-vartime) | fn |  |
| [`p256_scalar_inv_to_mont`](#p256-scalar-inv-to-mont) | fn |  |

## Functions

### `p256_mul_mont`

```rust
unsafe fn p256_mul_mont(r: *mut u64, a: *const u64, b: *const u64)
```

### `p256_sqr_mont`

```rust
unsafe fn p256_sqr_mont(r: *mut u64, a: *const u64)
```

### `p256_point_add`

```rust
unsafe fn p256_point_add(r: *mut u64, a: *const u64, b: *const u64)
```

### `p256_point_mul`

```rust
unsafe fn p256_point_mul(r: *mut u64, p_scalar: *const u64, p_x: *const u64, p_y: *const u64)
```

### `p256_scalar_mul_mont`

```rust
unsafe fn p256_scalar_mul_mont(r: *mut u64, a: *const u64, b: *const u64)
```

### `p256_scalar_sqr_rep_mont`

```rust
unsafe fn p256_scalar_sqr_rep_mont(r: *mut u64, a: *const u64, rep: u64)
```

### `p256_elem_inv_squared`

```rust
fn p256_elem_inv_squared(a: &elem::Elem<Q, R>) -> elem::Elem<Q, R>
```

### `p256_point_mul_base_impl`

```rust
fn p256_point_mul_base_impl(g_scalar: &elem::Elem<N, Unencoded>) -> Point
```

### `twin_mul_nistz256`

```rust
fn twin_mul_nistz256(g_scalar: &elem::Elem<N, Unencoded>, p_scalar: &elem::Elem<N, Unencoded>, p_xy: &(elem::Elem<Q, R>, elem::Elem<Q, R>)) -> Point
```

### `point_mul_base_vartime`

```rust
fn point_mul_base_vartime(g_scalar: &elem::Elem<N, Unencoded>) -> Point
```

### `p256_scalar_inv_to_mont`

```rust
fn p256_scalar_inv_to_mont(a: elem::Elem<N, R>) -> elem::Elem<N, R>
```

