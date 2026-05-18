*[num_bigint](../index.md) / [macros](index.md)*

---

# Module `macros`

## Contents

- [Macros](#macros)
  - [`cfg_32!`](#cfg-32)
  - [`cfg_32_or_test!`](#cfg-32-or-test)
  - [`cfg_64!`](#cfg-64)
  - [`cfg_digit!`](#cfg-digit)
  - [`cfg_digit_expr!`](#cfg-digit-expr)
  - [`forward_val_val_binop!`](#forward-val-val-binop)
  - [`forward_val_val_binop_commutative!`](#forward-val-val-binop-commutative)
  - [`forward_ref_val_binop!`](#forward-ref-val-binop)
  - [`forward_ref_val_binop_commutative!`](#forward-ref-val-binop-commutative)
  - [`forward_val_ref_binop!`](#forward-val-ref-binop)
  - [`forward_ref_ref_binop!`](#forward-ref-ref-binop)
  - [`forward_ref_ref_binop_commutative!`](#forward-ref-ref-binop-commutative)
  - [`forward_val_assign!`](#forward-val-assign)
  - [`forward_val_assign_scalar!`](#forward-val-assign-scalar)
  - [`forward_scalar_val_val_binop_commutative!`](#forward-scalar-val-val-binop-commutative)
  - [`forward_scalar_val_val_binop_to_ref_val!`](#forward-scalar-val-val-binop-to-ref-val)
  - [`forward_scalar_ref_ref_binop_to_ref_val!`](#forward-scalar-ref-ref-binop-to-ref-val)
  - [`forward_scalar_val_ref_binop_to_ref_val!`](#forward-scalar-val-ref-binop-to-ref-val)
  - [`forward_scalar_val_ref_binop_to_val_val!`](#forward-scalar-val-ref-binop-to-val-val)
  - [`forward_scalar_ref_val_binop_to_val_val!`](#forward-scalar-ref-val-binop-to-val-val)
  - [`forward_scalar_ref_ref_binop_to_val_val!`](#forward-scalar-ref-ref-binop-to-val-val)
  - [`promote_scalars!`](#promote-scalars)
  - [`promote_scalars_assign!`](#promote-scalars-assign)
  - [`promote_unsigned_scalars!`](#promote-unsigned-scalars)
  - [`promote_unsigned_scalars_assign!`](#promote-unsigned-scalars-assign)
  - [`promote_signed_scalars!`](#promote-signed-scalars)
  - [`promote_signed_scalars_assign!`](#promote-signed-scalars-assign)
  - [`forward_all_binop_to_ref_ref!`](#forward-all-binop-to-ref-ref)
  - [`forward_all_binop_to_val_ref!`](#forward-all-binop-to-val-ref)
  - [`forward_all_binop_to_val_ref_commutative!`](#forward-all-binop-to-val-ref-commutative)
  - [`forward_all_scalar_binop_to_ref_val!`](#forward-all-scalar-binop-to-ref-val)
  - [`forward_all_scalar_binop_to_val_val!`](#forward-all-scalar-binop-to-val-val)
  - [`forward_all_scalar_binop_to_val_val_commutative!`](#forward-all-scalar-binop-to-val-val-commutative)
  - [`promote_all_scalars!`](#promote-all-scalars)
  - [`promote_all_scalars_assign!`](#promote-all-scalars-assign)
  - [`impl_sum_iter_type!`](#impl-sum-iter-type)
  - [`impl_product_iter_type!`](#impl-product-iter-type)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfg_32!`](#cfg-32) | macro |  |
| [`cfg_32_or_test!`](#cfg-32-or-test) | macro |  |
| [`cfg_64!`](#cfg-64) | macro |  |
| [`cfg_digit!`](#cfg-digit) | macro |  |
| [`cfg_digit_expr!`](#cfg-digit-expr) | macro |  |
| [`forward_val_val_binop!`](#forward-val-val-binop) | macro |  |
| [`forward_val_val_binop_commutative!`](#forward-val-val-binop-commutative) | macro |  |
| [`forward_ref_val_binop!`](#forward-ref-val-binop) | macro |  |
| [`forward_ref_val_binop_commutative!`](#forward-ref-val-binop-commutative) | macro |  |
| [`forward_val_ref_binop!`](#forward-val-ref-binop) | macro |  |
| [`forward_ref_ref_binop!`](#forward-ref-ref-binop) | macro |  |
| [`forward_ref_ref_binop_commutative!`](#forward-ref-ref-binop-commutative) | macro |  |
| [`forward_val_assign!`](#forward-val-assign) | macro |  |
| [`forward_val_assign_scalar!`](#forward-val-assign-scalar) | macro |  |
| [`forward_scalar_val_val_binop_commutative!`](#forward-scalar-val-val-binop-commutative) | macro | use this if val_val_binop is already implemented and the reversed order is required |
| [`forward_scalar_val_val_binop_to_ref_val!`](#forward-scalar-val-val-binop-to-ref-val) | macro |  |
| [`forward_scalar_ref_ref_binop_to_ref_val!`](#forward-scalar-ref-ref-binop-to-ref-val) | macro |  |
| [`forward_scalar_val_ref_binop_to_ref_val!`](#forward-scalar-val-ref-binop-to-ref-val) | macro |  |
| [`forward_scalar_val_ref_binop_to_val_val!`](#forward-scalar-val-ref-binop-to-val-val) | macro |  |
| [`forward_scalar_ref_val_binop_to_val_val!`](#forward-scalar-ref-val-binop-to-val-val) | macro |  |
| [`forward_scalar_ref_ref_binop_to_val_val!`](#forward-scalar-ref-ref-binop-to-val-val) | macro |  |
| [`promote_scalars!`](#promote-scalars) | macro |  |
| [`promote_scalars_assign!`](#promote-scalars-assign) | macro |  |
| [`promote_unsigned_scalars!`](#promote-unsigned-scalars) | macro |  |
| [`promote_unsigned_scalars_assign!`](#promote-unsigned-scalars-assign) | macro |  |
| [`promote_signed_scalars!`](#promote-signed-scalars) | macro |  |
| [`promote_signed_scalars_assign!`](#promote-signed-scalars-assign) | macro |  |
| [`forward_all_binop_to_ref_ref!`](#forward-all-binop-to-ref-ref) | macro |  |
| [`forward_all_binop_to_val_ref!`](#forward-all-binop-to-val-ref) | macro |  |
| [`forward_all_binop_to_val_ref_commutative!`](#forward-all-binop-to-val-ref-commutative) | macro |  |
| [`forward_all_scalar_binop_to_ref_val!`](#forward-all-scalar-binop-to-ref-val) | macro |  |
| [`forward_all_scalar_binop_to_val_val!`](#forward-all-scalar-binop-to-val-val) | macro |  |
| [`forward_all_scalar_binop_to_val_val_commutative!`](#forward-all-scalar-binop-to-val-val-commutative) | macro |  |
| [`promote_all_scalars!`](#promote-all-scalars) | macro |  |
| [`promote_all_scalars_assign!`](#promote-all-scalars-assign) | macro |  |
| [`impl_sum_iter_type!`](#impl-sum-iter-type) | macro |  |
| [`impl_product_iter_type!`](#impl-product-iter-type) | macro |  |

## Macros

### `cfg_32!`

### `cfg_32_or_test!`

### `cfg_64!`

### `cfg_digit!`

### `cfg_digit_expr!`

### `forward_val_val_binop!`

### `forward_val_val_binop_commutative!`

### `forward_ref_val_binop!`

### `forward_ref_val_binop_commutative!`

### `forward_val_ref_binop!`

### `forward_ref_ref_binop!`

### `forward_ref_ref_binop_commutative!`

### `forward_val_assign!`

### `forward_val_assign_scalar!`

### `forward_scalar_val_val_binop_commutative!`

use this if val_val_binop is already implemented and the reversed order is required

### `forward_scalar_val_val_binop_to_ref_val!`

### `forward_scalar_ref_ref_binop_to_ref_val!`

### `forward_scalar_val_ref_binop_to_ref_val!`

### `forward_scalar_val_ref_binop_to_val_val!`

### `forward_scalar_ref_val_binop_to_val_val!`

### `forward_scalar_ref_ref_binop_to_val_val!`

### `promote_scalars!`

### `promote_scalars_assign!`

### `promote_unsigned_scalars!`

### `promote_unsigned_scalars_assign!`

### `promote_signed_scalars!`

### `promote_signed_scalars_assign!`

### `forward_all_binop_to_ref_ref!`

### `forward_all_binop_to_val_ref!`

### `forward_all_binop_to_val_ref_commutative!`

### `forward_all_scalar_binop_to_ref_val!`

### `forward_all_scalar_binop_to_val_val!`

### `forward_all_scalar_binop_to_val_val_commutative!`

### `promote_all_scalars!`

### `promote_all_scalars_assign!`

### `impl_sum_iter_type!`

### `impl_product_iter_type!`

