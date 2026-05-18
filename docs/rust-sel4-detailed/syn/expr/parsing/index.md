*[syn](../../index.md) / [expr](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Structs](#structs)
  - [`AllowStruct`](#allowstruct)
- [Functions](#functions)
  - [`parse_with_earlier_boundary_rule`](#parse-with-earlier-boundary-rule)
  - [`parse_expr`](#parse-expr)
  - [`parse_binop_rhs`](#parse-binop-rhs)
  - [`peek_precedence`](#peek-precedence)
  - [`ambiguous_expr`](#ambiguous-expr)
  - [`expr_attrs`](#expr-attrs)
  - [`unary_expr`](#unary-expr)
  - [`trailer_expr`](#trailer-expr)
  - [`trailer_helper`](#trailer-helper)
  - [`atom_expr`](#atom-expr)
  - [`atom_labeled`](#atom-labeled)
  - [`expr_builtin`](#expr-builtin)
  - [`path_or_macro_or_struct`](#path-or-macro-or-struct)
  - [`rest_of_path_or_macro_or_struct`](#rest-of-path-or-macro-or-struct)
  - [`paren_or_tuple`](#paren-or-tuple)
  - [`array_or_repeat`](#array-or-repeat)
  - [`continue_parsing_early`](#continue-parsing-early)
  - [`expr_group`](#expr-group)
  - [`expr_let`](#expr-let)
  - [`expr_unary`](#expr-unary)
  - [`expr_become`](#expr-become)
  - [`expr_closure`](#expr-closure)
  - [`closure_arg`](#closure-arg)
  - [`expr_break`](#expr-break)
  - [`expr_struct_helper`](#expr-struct-helper)
  - [`expr_range`](#expr-range)
  - [`parse_range_end`](#parse-range-end)
  - [`multi_index`](#multi-index)
  - [`check_cast`](#check-cast)
- [Macros](#macros)
  - [`impl_by_parsing_expr!`](#impl-by-parsing-expr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AllowStruct`](#allowstruct) | struct |  |
| [`parse_with_earlier_boundary_rule`](#parse-with-earlier-boundary-rule) | fn |  |
| [`parse_expr`](#parse-expr) | fn |  |
| [`parse_binop_rhs`](#parse-binop-rhs) | fn |  |
| [`peek_precedence`](#peek-precedence) | fn |  |
| [`ambiguous_expr`](#ambiguous-expr) | fn |  |
| [`expr_attrs`](#expr-attrs) | fn |  |
| [`unary_expr`](#unary-expr) | fn |  |
| [`trailer_expr`](#trailer-expr) | fn |  |
| [`trailer_helper`](#trailer-helper) | fn |  |
| [`atom_expr`](#atom-expr) | fn |  |
| [`atom_labeled`](#atom-labeled) | fn |  |
| [`expr_builtin`](#expr-builtin) | fn |  |
| [`path_or_macro_or_struct`](#path-or-macro-or-struct) | fn |  |
| [`rest_of_path_or_macro_or_struct`](#rest-of-path-or-macro-or-struct) | fn |  |
| [`paren_or_tuple`](#paren-or-tuple) | fn |  |
| [`array_or_repeat`](#array-or-repeat) | fn |  |
| [`continue_parsing_early`](#continue-parsing-early) | fn |  |
| [`expr_group`](#expr-group) | fn |  |
| [`expr_let`](#expr-let) | fn |  |
| [`expr_unary`](#expr-unary) | fn |  |
| [`expr_become`](#expr-become) | fn |  |
| [`expr_closure`](#expr-closure) | fn |  |
| [`closure_arg`](#closure-arg) | fn |  |
| [`expr_break`](#expr-break) | fn |  |
| [`expr_struct_helper`](#expr-struct-helper) | fn |  |
| [`expr_range`](#expr-range) | fn |  |
| [`parse_range_end`](#parse-range-end) | fn |  |
| [`multi_index`](#multi-index) | fn |  |
| [`check_cast`](#check-cast) | fn |  |
| [`impl_by_parsing_expr!`](#impl-by-parsing-expr) | macro |  |

## Structs

### `AllowStruct`

```rust
struct AllowStruct(bool);
```

#### Trait Implementations

##### `impl Clone for AllowStruct`

- <span id="allowstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for AllowStruct`

## Functions

### `parse_with_earlier_boundary_rule`

```rust
fn parse_with_earlier_boundary_rule(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `parse_expr`

```rust
fn parse_expr(input: crate::parse::ParseStream<'_>, lhs: crate::expr::Expr, allow_struct: AllowStruct, base: crate::precedence::Precedence) -> crate::error::Result<crate::expr::Expr>
```

### `parse_binop_rhs`

```rust
fn parse_binop_rhs(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct, precedence: crate::precedence::Precedence) -> crate::error::Result<alloc::boxed::Box<crate::expr::Expr>>
```

### `peek_precedence`

```rust
fn peek_precedence(input: crate::parse::ParseStream<'_>) -> crate::precedence::Precedence
```

### `ambiguous_expr`

```rust
fn ambiguous_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `expr_attrs`

```rust
fn expr_attrs(input: crate::parse::ParseStream<'_>) -> crate::error::Result<alloc::vec::Vec<crate::attr::Attribute>>
```

### `unary_expr`

```rust
fn unary_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `trailer_expr`

```rust
fn trailer_expr(begin: crate::parse::ParseBuffer<'_>, attrs: alloc::vec::Vec<crate::attr::Attribute>, input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `trailer_helper`

```rust
fn trailer_helper(input: crate::parse::ParseStream<'_>, e: crate::expr::Expr) -> crate::error::Result<crate::expr::Expr>
```

### `atom_expr`

```rust
fn atom_expr(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `atom_labeled`

```rust
fn atom_labeled(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `expr_builtin`

```rust
fn expr_builtin(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `path_or_macro_or_struct`

```rust
fn path_or_macro_or_struct(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `rest_of_path_or_macro_or_struct`

```rust
fn rest_of_path_or_macro_or_struct(qself: Option<crate::path::QSelf>, path: crate::path::Path, input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `paren_or_tuple`

```rust
fn paren_or_tuple(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `array_or_repeat`

```rust
fn array_or_repeat(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `continue_parsing_early`

```rust
fn continue_parsing_early(expr: &crate::expr::Expr) -> bool
```

### `expr_group`

```rust
fn expr_group(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```

### `expr_let`

```rust
fn expr_let(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprLet>
```

### `expr_unary`

```rust
fn expr_unary(input: crate::parse::ParseStream<'_>, attrs: alloc::vec::Vec<crate::attr::Attribute>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprUnary>
```

### `expr_become`

```rust
fn expr_become(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `expr_closure`

```rust
fn expr_closure(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprClosure>
```

### `closure_arg`

```rust
fn closure_arg(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

### `expr_break`

```rust
fn expr_break(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprBreak>
```

### `expr_struct_helper`

```rust
fn expr_struct_helper(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::expr::ExprStruct>
```

### `expr_range`

```rust
fn expr_range(input: crate::parse::ParseStream<'_>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprRange>
```

### `parse_range_end`

```rust
fn parse_range_end(input: crate::parse::ParseStream<'_>, limits: &crate::expr::RangeLimits, allow_struct: AllowStruct) -> crate::error::Result<Option<alloc::boxed::Box<crate::expr::Expr>>>
```

### `multi_index`

```rust
fn multi_index(e: &mut crate::expr::Expr, dot_token: &mut token::Dot, float: crate::lit::LitFloat) -> crate::error::Result<bool>
```

### `check_cast`

```rust
fn check_cast(input: crate::parse::ParseStream<'_>) -> crate::error::Result<()>
```

## Macros

### `impl_by_parsing_expr!`

