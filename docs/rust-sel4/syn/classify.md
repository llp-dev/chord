**syn > classify**

# Module: classify

## Contents

**Functions**

- [`expr_leading_label`](#expr_leading_label) - Whether the expression's first token is the label of a loop/block.
- [`expr_trailing_brace`](#expr_trailing_brace) - Whether the expression's last token is `}`.
- [`requires_comma_to_be_match_arm`](#requires_comma_to_be_match_arm)
- [`requires_semi_to_be_stmt`](#requires_semi_to_be_stmt)
- [`trailing_unparameterized_path`](#trailing_unparameterized_path)

---

## syn::classify::expr_leading_label

*Function*

Whether the expression's first token is the label of a loop/block.

```rust
fn expr_leading_label(expr: &crate::expr::Expr) -> bool
```



## syn::classify::expr_trailing_brace

*Function*

Whether the expression's last token is `}`.

```rust
fn expr_trailing_brace(expr: &crate::expr::Expr) -> bool
```



## syn::classify::requires_comma_to_be_match_arm

*Function*

```rust
fn requires_comma_to_be_match_arm(expr: &crate::expr::Expr) -> bool
```



## syn::classify::requires_semi_to_be_stmt

*Function*

```rust
fn requires_semi_to_be_stmt(expr: &crate::expr::Expr) -> bool
```



## syn::classify::trailing_unparameterized_path

*Function*

```rust
fn trailing_unparameterized_path(ty: &crate::ty::Type) -> bool
```



