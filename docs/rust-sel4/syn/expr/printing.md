**syn > expr > printing**

# Module: expr::printing

## Contents

**Functions**

- [`inner_attrs_to_tokens`](#inner_attrs_to_tokens)
- [`outer_attrs_to_tokens`](#outer_attrs_to_tokens)
- [`print_expr`](#print_expr)
- [`print_expr_assign`](#print_expr_assign)
- [`print_expr_await`](#print_expr_await)
- [`print_expr_binary`](#print_expr_binary)
- [`print_expr_break`](#print_expr_break)
- [`print_expr_call`](#print_expr_call)
- [`print_expr_cast`](#print_expr_cast)
- [`print_expr_closure`](#print_expr_closure)
- [`print_expr_field`](#print_expr_field)
- [`print_expr_index`](#print_expr_index)
- [`print_expr_let`](#print_expr_let)
- [`print_expr_method_call`](#print_expr_method_call)
- [`print_expr_range`](#print_expr_range)
- [`print_expr_raw_addr`](#print_expr_raw_addr)
- [`print_expr_reference`](#print_expr_reference)
- [`print_expr_return`](#print_expr_return)
- [`print_expr_try`](#print_expr_try)
- [`print_expr_unary`](#print_expr_unary)
- [`print_expr_yield`](#print_expr_yield)
- [`print_subexpression`](#print_subexpression)

---

## syn::expr::printing::inner_attrs_to_tokens

*Function*

```rust
fn inner_attrs_to_tokens(attrs: &[crate::attr::Attribute], tokens: & mut proc_macro2::TokenStream)
```



## syn::expr::printing::outer_attrs_to_tokens

*Function*

```rust
fn outer_attrs_to_tokens(attrs: &[crate::attr::Attribute], tokens: & mut proc_macro2::TokenStream)
```



## syn::expr::printing::print_expr

*Function*

```rust
fn print_expr(expr: &crate::expr::Expr, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_assign

*Function*

```rust
fn print_expr_assign(e: &crate::expr::ExprAssign, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_await

*Function*

```rust
fn print_expr_await(e: &crate::expr::ExprAwait, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_binary

*Function*

```rust
fn print_expr_binary(e: &crate::expr::ExprBinary, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_break

*Function*

```rust
fn print_expr_break(e: &crate::expr::ExprBreak, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_call

*Function*

```rust
fn print_expr_call(e: &crate::expr::ExprCall, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_cast

*Function*

```rust
fn print_expr_cast(e: &crate::expr::ExprCast, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_closure

*Function*

```rust
fn print_expr_closure(e: &crate::expr::ExprClosure, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_field

*Function*

```rust
fn print_expr_field(e: &crate::expr::ExprField, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_index

*Function*

```rust
fn print_expr_index(e: &crate::expr::ExprIndex, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_let

*Function*

```rust
fn print_expr_let(e: &crate::expr::ExprLet, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_method_call

*Function*

```rust
fn print_expr_method_call(e: &crate::expr::ExprMethodCall, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_range

*Function*

```rust
fn print_expr_range(e: &crate::expr::ExprRange, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_raw_addr

*Function*

```rust
fn print_expr_raw_addr(e: &crate::expr::ExprRawAddr, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_reference

*Function*

```rust
fn print_expr_reference(e: &crate::expr::ExprReference, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_return

*Function*

```rust
fn print_expr_return(e: &crate::expr::ExprReturn, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_try

*Function*

```rust
fn print_expr_try(e: &crate::expr::ExprTry, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_unary

*Function*

```rust
fn print_expr_unary(e: &crate::expr::ExprUnary, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_expr_yield

*Function*

```rust
fn print_expr_yield(e: &crate::expr::ExprYield, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



## syn::expr::printing::print_subexpression

*Function*

```rust
fn print_subexpression(expr: &crate::expr::Expr, needs_group: bool, tokens: & mut proc_macro2::TokenStream, fixup: crate::fixup::FixupContext)
```



