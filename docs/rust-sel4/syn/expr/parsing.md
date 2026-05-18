**syn > expr > parsing**

# Module: expr::parsing

## Contents

**Macros**

- [`impl_by_parsing_expr`](#impl_by_parsing_expr)

**Structs**

- [`AllowStruct`](#allowstruct)

**Functions**

- [`ambiguous_expr`](#ambiguous_expr)
- [`array_or_repeat`](#array_or_repeat)
- [`atom_expr`](#atom_expr)
- [`atom_labeled`](#atom_labeled)
- [`check_cast`](#check_cast)
- [`closure_arg`](#closure_arg)
- [`continue_parsing_early`](#continue_parsing_early)
- [`expr_attrs`](#expr_attrs)
- [`expr_become`](#expr_become)
- [`expr_break`](#expr_break)
- [`expr_builtin`](#expr_builtin)
- [`expr_closure`](#expr_closure)
- [`expr_group`](#expr_group)
- [`expr_let`](#expr_let)
- [`expr_range`](#expr_range)
- [`expr_struct_helper`](#expr_struct_helper)
- [`expr_unary`](#expr_unary)
- [`multi_index`](#multi_index)
- [`paren_or_tuple`](#paren_or_tuple)
- [`parse_binop_rhs`](#parse_binop_rhs)
- [`parse_expr`](#parse_expr)
- [`parse_range_end`](#parse_range_end)
- [`parse_with_earlier_boundary_rule`](#parse_with_earlier_boundary_rule)
- [`path_or_macro_or_struct`](#path_or_macro_or_struct)
- [`peek_precedence`](#peek_precedence)
- [`rest_of_path_or_macro_or_struct`](#rest_of_path_or_macro_or_struct)
- [`trailer_expr`](#trailer_expr)
- [`trailer_helper`](#trailer_helper)
- [`unary_expr`](#unary_expr)

---

## syn::expr::parsing::AllowStruct

*Struct*

**Tuple Struct**: `(bool)`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## syn::expr::parsing::ambiguous_expr

*Function*

```rust
fn ambiguous_expr(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::array_or_repeat

*Function*

```rust
fn array_or_repeat(input: crate::parse::ParseStream) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::atom_expr

*Function*

```rust
fn atom_expr(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::atom_labeled

*Function*

```rust
fn atom_labeled(input: crate::parse::ParseStream) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::check_cast

*Function*

```rust
fn check_cast(input: crate::parse::ParseStream) -> crate::error::Result<()>
```



## syn::expr::parsing::closure_arg

*Function*

```rust
fn closure_arg(input: crate::parse::ParseStream) -> crate::error::Result<crate::pat::Pat>
```



## syn::expr::parsing::continue_parsing_early

*Function*

```rust
fn continue_parsing_early(expr: &crate::expr::Expr) -> bool
```



## syn::expr::parsing::expr_attrs

*Function*

```rust
fn expr_attrs(input: crate::parse::ParseStream) -> crate::error::Result<alloc::vec::Vec<crate::attr::Attribute>>
```



## syn::expr::parsing::expr_become

*Function*

```rust
fn expr_become(input: crate::parse::ParseStream) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::expr_break

*Function*

```rust
fn expr_break(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprBreak>
```



## syn::expr::parsing::expr_builtin

*Function*

```rust
fn expr_builtin(input: crate::parse::ParseStream) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::expr_closure

*Function*

```rust
fn expr_closure(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprClosure>
```



## syn::expr::parsing::expr_group

*Function*

```rust
fn expr_group(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::expr_let

*Function*

```rust
fn expr_let(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprLet>
```



## syn::expr::parsing::expr_range

*Function*

```rust
fn expr_range(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprRange>
```



## syn::expr::parsing::expr_struct_helper

*Function*

```rust
fn expr_struct_helper(input: crate::parse::ParseStream, qself: Option<crate::path::QSelf>, path: crate::path::Path) -> crate::error::Result<crate::expr::ExprStruct>
```



## syn::expr::parsing::expr_unary

*Function*

```rust
fn expr_unary(input: crate::parse::ParseStream, attrs: alloc::vec::Vec<crate::attr::Attribute>, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::ExprUnary>
```



## syn::expr::parsing::impl_by_parsing_expr

*Declarative Macro*

```rust
macro_rules! impl_by_parsing_expr {
    (
            $(
                $expr_type:ty, $variant:ident, $msg:expr,
            )*
        ) => { ... };
}
```



## syn::expr::parsing::multi_index

*Function*

```rust
fn multi_index(e: & mut crate::expr::Expr, dot_token: & mut $crate::token::Dot, float: crate::lit::LitFloat) -> crate::error::Result<bool>
```



## syn::expr::parsing::paren_or_tuple

*Function*

```rust
fn paren_or_tuple(input: crate::parse::ParseStream) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::parse_binop_rhs

*Function*

```rust
fn parse_binop_rhs(input: crate::parse::ParseStream, allow_struct: AllowStruct, precedence: crate::precedence::Precedence) -> crate::error::Result<alloc::boxed::Box<crate::expr::Expr>>
```



## syn::expr::parsing::parse_expr

*Function*

```rust
fn parse_expr(input: crate::parse::ParseStream, lhs: crate::expr::Expr, allow_struct: AllowStruct, base: crate::precedence::Precedence) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::parse_range_end

*Function*

```rust
fn parse_range_end(input: crate::parse::ParseStream, limits: &crate::expr::RangeLimits, allow_struct: AllowStruct) -> crate::error::Result<Option<alloc::boxed::Box<crate::expr::Expr>>>
```



## syn::expr::parsing::parse_with_earlier_boundary_rule

*Function*

```rust
fn parse_with_earlier_boundary_rule(input: crate::parse::ParseStream) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::path_or_macro_or_struct

*Function*

```rust
fn path_or_macro_or_struct(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::peek_precedence

*Function*

```rust
fn peek_precedence(input: crate::parse::ParseStream) -> crate::precedence::Precedence
```



## syn::expr::parsing::rest_of_path_or_macro_or_struct

*Function*

```rust
fn rest_of_path_or_macro_or_struct(qself: Option<crate::path::QSelf>, path: crate::path::Path, input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::trailer_expr

*Function*

```rust
fn trailer_expr(begin: crate::parse::ParseBuffer, attrs: alloc::vec::Vec<crate::attr::Attribute>, input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::trailer_helper

*Function*

```rust
fn trailer_helper(input: crate::parse::ParseStream, e: crate::expr::Expr) -> crate::error::Result<crate::expr::Expr>
```



## syn::expr::parsing::unary_expr

*Function*

```rust
fn unary_expr(input: crate::parse::ParseStream, allow_struct: AllowStruct) -> crate::error::Result<crate::expr::Expr>
```



