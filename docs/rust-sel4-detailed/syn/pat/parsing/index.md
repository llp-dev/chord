*[syn](../../index.md) / [pat](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Enums](#enums)
  - [`PatRangeBound`](#patrangebound)
- [Functions](#functions)
  - [`multi_pat_impl`](#multi-pat-impl)
  - [`pat_path_or_macro_or_struct_or_range`](#pat-path-or-macro-or-struct-or-range)
  - [`pat_wild`](#pat-wild)
  - [`pat_box`](#pat-box)
  - [`pat_ident`](#pat-ident)
  - [`pat_tuple_struct`](#pat-tuple-struct)
  - [`pat_struct`](#pat-struct)
  - [`field_pat`](#field-pat)
  - [`pat_range`](#pat-range)
  - [`pat_range_half_open`](#pat-range-half-open)
  - [`pat_paren_or_tuple`](#pat-paren-or-tuple)
  - [`pat_reference`](#pat-reference)
  - [`pat_lit_or_range`](#pat-lit-or-range)
  - [`pat_range_bound`](#pat-range-bound)
  - [`pat_slice`](#pat-slice)
  - [`pat_const`](#pat-const)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PatRangeBound`](#patrangebound) | enum |  |
| [`multi_pat_impl`](#multi-pat-impl) | fn |  |
| [`pat_path_or_macro_or_struct_or_range`](#pat-path-or-macro-or-struct-or-range) | fn |  |
| [`pat_wild`](#pat-wild) | fn |  |
| [`pat_box`](#pat-box) | fn |  |
| [`pat_ident`](#pat-ident) | fn |  |
| [`pat_tuple_struct`](#pat-tuple-struct) | fn |  |
| [`pat_struct`](#pat-struct) | fn |  |
| [`field_pat`](#field-pat) | fn |  |
| [`pat_range`](#pat-range) | fn |  |
| [`pat_range_half_open`](#pat-range-half-open) | fn |  |
| [`pat_paren_or_tuple`](#pat-paren-or-tuple) | fn |  |
| [`pat_reference`](#pat-reference) | fn |  |
| [`pat_lit_or_range`](#pat-lit-or-range) | fn |  |
| [`pat_range_bound`](#pat-range-bound) | fn |  |
| [`pat_slice`](#pat-slice) | fn |  |
| [`pat_const`](#pat-const) | fn |  |

## Enums

### `PatRangeBound`

```rust
enum PatRangeBound {
    Const(crate::expr::ExprConst),
    Lit(crate::expr::ExprLit),
    Path(crate::expr::ExprPath),
}
```

#### Implementations

- <span id="patrangebound-into-expr"></span>`fn into_expr(self) -> Box<Expr>` — [`Expr`](../../expr/index.md#expr)

- <span id="patrangebound-into-pat"></span>`fn into_pat(self) -> Pat` — [`Pat`](../index.md#pat)

## Functions

### `multi_pat_impl`

```rust
fn multi_pat_impl(input: crate::parse::ParseStream<'_>, leading_vert: Option<token::Or>) -> crate::error::Result<crate::pat::Pat>
```

### `pat_path_or_macro_or_struct_or_range`

```rust
fn pat_path_or_macro_or_struct_or_range(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

### `pat_wild`

```rust
fn pat_wild(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatWild>
```

### `pat_box`

```rust
fn pat_box(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

### `pat_ident`

```rust
fn pat_ident(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatIdent>
```

### `pat_tuple_struct`

```rust
fn pat_tuple_struct(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::PatTupleStruct>
```

### `pat_struct`

```rust
fn pat_struct(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::PatStruct>
```

### `field_pat`

```rust
fn field_pat(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::FieldPat>
```

### `pat_range`

```rust
fn pat_range(input: crate::parse::ParseStream<'_>, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::Pat>
```

### `pat_range_half_open`

```rust
fn pat_range_half_open(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

### `pat_paren_or_tuple`

```rust
fn pat_paren_or_tuple(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

### `pat_reference`

```rust
fn pat_reference(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatReference>
```

### `pat_lit_or_range`

```rust
fn pat_lit_or_range(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::Pat>
```

### `pat_range_bound`

```rust
fn pat_range_bound(input: crate::parse::ParseStream<'_>) -> crate::error::Result<Option<PatRangeBound>>
```

### `pat_slice`

```rust
fn pat_slice(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::pat::PatSlice>
```

### `pat_const`

```rust
fn pat_const(input: crate::parse::ParseStream<'_>) -> crate::error::Result<proc_macro2::TokenStream>
```

