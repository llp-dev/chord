**syn > pat > parsing**

# Module: pat::parsing

## Contents

**Enums**

- [`PatRangeBound`](#patrangebound)

**Functions**

- [`field_pat`](#field_pat)
- [`multi_pat_impl`](#multi_pat_impl)
- [`pat_box`](#pat_box)
- [`pat_const`](#pat_const)
- [`pat_ident`](#pat_ident)
- [`pat_lit_or_range`](#pat_lit_or_range)
- [`pat_paren_or_tuple`](#pat_paren_or_tuple)
- [`pat_path_or_macro_or_struct_or_range`](#pat_path_or_macro_or_struct_or_range)
- [`pat_range`](#pat_range)
- [`pat_range_bound`](#pat_range_bound)
- [`pat_range_half_open`](#pat_range_half_open)
- [`pat_reference`](#pat_reference)
- [`pat_slice`](#pat_slice)
- [`pat_struct`](#pat_struct)
- [`pat_tuple_struct`](#pat_tuple_struct)
- [`pat_wild`](#pat_wild)

---

## syn::pat::parsing::PatRangeBound

*Enum*

**Variants:**
- `Const(crate::expr::ExprConst)`
- `Lit(crate::expr::ExprLit)`
- `Path(crate::expr::ExprPath)`

**Methods:**

- `fn into_expr(self: Self) -> Box<Expr>`
- `fn into_pat(self: Self) -> Pat`



## syn::pat::parsing::field_pat

*Function*

```rust
fn field_pat(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::FieldPat>
```



## syn::pat::parsing::multi_pat_impl

*Function*

```rust
fn multi_pat_impl(input: crate::parse::ParseStream, leading_vert: Option<$crate::token::Or>) -> crate::error::Result<crate::pat::Pat>
```



## syn::pat::parsing::pat_box

*Function*

```rust
fn pat_box(begin: crate::parse::ParseBuffer, input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::Pat>
```



## syn::pat::parsing::pat_const

*Function*

```rust
fn pat_const(input: crate::parse::ParseStream) -> crate::error::Result<proc_macro2::TokenStream>
```



## syn::pat::parsing::pat_ident

*Function*

```rust
fn pat_ident(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::PatIdent>
```



## syn::pat::parsing::pat_lit_or_range

*Function*

```rust
fn pat_lit_or_range(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::Pat>
```



## syn::pat::parsing::pat_paren_or_tuple

*Function*

```rust
fn pat_paren_or_tuple(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::Pat>
```



## syn::pat::parsing::pat_path_or_macro_or_struct_or_range

*Function*

```rust
fn pat_path_or_macro_or_struct_or_range(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::Pat>
```



## syn::pat::parsing::pat_range

*Function*

```rust
fn pat_range(input: crate::parse::ParseStream, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::Pat>
```



## syn::pat::parsing::pat_range_bound

*Function*

```rust
fn pat_range_bound(input: crate::parse::ParseStream) -> crate::error::Result<Option<PatRangeBound>>
```



## syn::pat::parsing::pat_range_half_open

*Function*

```rust
fn pat_range_half_open(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::Pat>
```



## syn::pat::parsing::pat_reference

*Function*

```rust
fn pat_reference(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::PatReference>
```



## syn::pat::parsing::pat_slice

*Function*

```rust
fn pat_slice(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::PatSlice>
```



## syn::pat::parsing::pat_struct

*Function*

```rust
fn pat_struct(input: crate::parse::ParseStream, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::PatStruct>
```



## syn::pat::parsing::pat_tuple_struct

*Function*

```rust
fn pat_tuple_struct(input: crate::parse::ParseStream, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::pat::PatTupleStruct>
```



## syn::pat::parsing::pat_wild

*Function*

```rust
fn pat_wild(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::PatWild>
```



